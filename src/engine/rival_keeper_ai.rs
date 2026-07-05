use crate::data::GameData;
use crate::engine::tile_types::{self, types as tt};
use crate::state::entities::{CreatureState, EntityId, Task};
use crate::state::game_state::GameState;
use crate::state::tile_state::{Ownership, TilePos};
use crate::state::OwnerId;

const DECISION_INTERVAL: f32 = 10.0;
const DEFENSE_RADIUS: i32 = 7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RivalKeeperPlan {
    pub owner: OwnerId,
    pub ai_profile: String,
    pub owned_creatures: usize,
    pub desired_rooms: Vec<String>,
    pub preferred_creatures: Vec<String>,
    pub wants_reinforcements: bool,
    pub wants_attack: bool,
    pub wants_digging: bool,
    pub wants_room: Option<String>,
    pub wants_defense: bool,
}

pub fn plan_rival_keepers(state: &GameState) -> Vec<RivalKeeperPlan> {
    state
        .rival_keepers
        .keepers
        .iter()
        .map(|keeper| {
            let owned_creatures = state
                .entities
                .all()
                .filter(|entity| entity.owner == keeper.owner && entity.as_creature().is_some())
                .count();
            let missing_room = missing_desired_room(state, &keeper.owner, &keeper.desired_rooms);
            RivalKeeperPlan {
                owner: keeper.owner.clone(),
                ai_profile: keeper.ai_profile.clone(),
                owned_creatures,
                desired_rooms: keeper.desired_rooms.clone(),
                preferred_creatures: keeper.preferred_creatures.clone(),
                wants_reinforcements: owned_creatures < keeper.min_garrison,
                wants_attack: owned_creatures >= keeper.min_garrison
                    && keeper.next_attack_time <= 0.0,
                wants_digging: missing_room.is_some(),
                wants_room: missing_room,
                wants_defense: nearest_threat_position(state, &keeper.owner).is_some(),
            }
        })
        .collect()
}

pub fn update_rival_keeper_ai(state: &mut GameState, game_data: &GameData, dt: f32) {
    let mut decisions = Vec::new();

    for (index, keeper) in state.rival_keepers.keepers.iter_mut().enumerate() {
        keeper.next_decision_time -= dt;
        keeper.next_attack_time = (keeper.next_attack_time - dt).max(0.0);
        if keeper.next_decision_time <= 0.0 {
            keeper.next_decision_time = DECISION_INTERVAL;
            decisions.push(index);
        }
    }

    for index in decisions {
        execute_keeper_decision(state, game_data, index);
    }
}

fn execute_keeper_decision(state: &mut GameState, game_data: &GameData, keeper_index: usize) {
    let Some(keeper) = state.rival_keepers.keepers.get(keeper_index).cloned() else {
        return;
    };

    let owned_creatures = state
        .entities
        .all()
        .filter(|entity| entity.owner == keeper.owner && entity.as_creature().is_some())
        .count();

    if assign_defenders_to_threat(state, &keeper) {
        return;
    }

    if expand_or_build_desired_room(state, game_data, &keeper) {
        return;
    }

    if owned_creatures < keeper.min_garrison {
        spawn_reinforcement(state, game_data, &keeper);
        return;
    }

    if keeper.next_attack_time > 0.0 {
        dig_expansion(state, game_data, &keeper.owner, keeper.dig_batch);
        return;
    }

    if launch_heart_raid(state, &keeper.owner, keeper.raid_size) {
        if let Some(runtime) = state.rival_keepers.keepers.get_mut(keeper_index) {
            // Each raid widens the gap to the next one so the player gets
            // breathing room to rebuild between waves
            runtime.attack_cooldown =
                (runtime.attack_cooldown * runtime.attack_cooldown_growth.max(1.0)).min(600.0);
            runtime.next_attack_time = runtime.attack_cooldown;
        }
    }
}

fn spawn_reinforcement(
    state: &mut GameState,
    game_data: &GameData,
    keeper: &crate::state::rival_keeper::RivalKeeperAiState,
) {
    let Some(creature_id) = keeper
        .preferred_creatures
        .iter()
        .find(|id| game_data.monsters.contains_key(*id))
        .cloned()
    else {
        return;
    };
    let Some(spawn_pos) = find_owned_spawn_tile(state, &keeper.owner) else {
        return;
    };
    let Some(monster_data) = game_data.monsters.get(&creature_id) else {
        return;
    };

    let visual_seed = macroquad_toolkit::rng::random_u64();
    let creature_state = CreatureState::new(
        creature_id,
        1,
        monster_data.stats.health,
        monster_data.stats.mana,
        visual_seed,
    );
    state
        .entities
        .spawn_creature_for_owner(spawn_pos, creature_state, keeper.owner.clone());
}

fn expand_or_build_desired_room(
    state: &mut GameState,
    game_data: &GameData,
    keeper: &crate::state::rival_keeper::RivalKeeperAiState,
) -> bool {
    let Some(room_id) = missing_desired_room(state, &keeper.owner, &keeper.desired_rooms) else {
        return false;
    };

    if place_room(state, game_data, &keeper.owner, &room_id, keeper.room_size) {
        return true;
    }

    dig_expansion(state, game_data, &keeper.owner, keeper.dig_batch)
}

fn missing_desired_room(
    state: &GameState,
    owner: &OwnerId,
    desired_rooms: &[String],
) -> Option<String> {
    desired_rooms
        .iter()
        .find(|room_id| !owner_has_room(state, owner, room_id))
        .cloned()
}

fn owner_has_room(state: &GameState, owner: &OwnerId, room_id: &str) -> bool {
    state.dungeon.grid.iter().flatten().any(|tile| {
        &tile.owner == owner && tile.tile_type == room_id && tile.ownership == Ownership::Enemy
    })
}

fn place_room(
    state: &mut GameState,
    game_data: &GameData,
    owner: &OwnerId,
    room_id: &str,
    desired_side: usize,
) -> bool {
    let Some(room_data) = game_data.rooms.get(room_id) else {
        return false;
    };
    let min_tiles = room_data.build.min_tiles.max(1) as usize;
    let side = desired_side.max(square_side_for_tiles(min_tiles));
    let Some(tiles) = find_room_site(state, owner, side) else {
        return false;
    };

    for pos in tiles {
        if let Some(tile) = state.dungeon.get_tile_mut(pos) {
            tile.tile_type = room_id.to_string();
            tile.resources_remaining = None;
            tile.marked_for_dig = false;
            tile.room_id = None;
            tile.set_owner(owner.clone());
        }
    }
    true
}

fn square_side_for_tiles(tile_count: usize) -> usize {
    (tile_count as f32).sqrt().ceil() as usize
}

fn find_room_site(state: &GameState, owner: &OwnerId, side: usize) -> Option<Vec<TilePos>> {
    let width = state.dungeon.width as i32;
    let height = state.dungeon.height as i32;

    for y in 0..=(height - side as i32) {
        for x in 0..=(width - side as i32) {
            let mut positions = Vec::with_capacity(side * side);
            let mut valid = true;
            for dy in 0..side as i32 {
                for dx in 0..side as i32 {
                    let pos = TilePos::new(x + dx, y + dy);
                    let Some(tile) = state.dungeon.get_tile(pos) else {
                        valid = false;
                        break;
                    };
                    if &tile.owner != owner || tile.tile_type != tt::CLAIMED_FLOOR {
                        valid = false;
                        break;
                    }
                    positions.push(pos);
                }
                if !valid {
                    break;
                }
            }
            if valid {
                return Some(positions);
            }
        }
    }

    None
}

fn dig_expansion(
    state: &mut GameState,
    game_data: &GameData,
    owner: &OwnerId,
    dig_batch: usize,
) -> bool {
    let mut candidates = expansion_candidates(state, game_data, owner);
    let Some(player_heart) = state.find_dungeon_heart_position() else {
        return false;
    };
    candidates.sort_by_key(|pos| (pos.manhattan_distance(&player_heart), pos.y, pos.x));

    let mut dug = false;
    for pos in candidates.into_iter().take(dig_batch.max(1)) {
        if let Some(tile) = state.dungeon.get_tile_mut(pos) {
            tile.tile_type = tt::CLAIMED_FLOOR.to_string();
            tile.resources_remaining = None;
            tile.marked_for_dig = false;
            tile.room_id = None;
            tile.set_owner(owner.clone());
            dug = true;
        }
    }

    dug
}

fn expansion_candidates(state: &GameState, game_data: &GameData, owner: &OwnerId) -> Vec<TilePos> {
    let mut candidates = Vec::new();

    for row in &state.dungeon.grid {
        for tile in row {
            if &tile.owner != owner || !tile_types::is_walkable(&tile.tile_type, game_data) {
                continue;
            }

            for neighbor in cardinal_neighbors(tile.pos) {
                let Some(candidate) = state.dungeon.get_tile(neighbor) else {
                    continue;
                };
                let can_dig = tile_types::is_diggable(&candidate.tile_type, game_data)
                    && matches!(candidate.owner, OwnerId::Neutral)
                    && candidate.ownership == Ownership::Unclaimed;
                if can_dig && !candidates.contains(&neighbor) {
                    candidates.push(neighbor);
                }
            }
        }
    }

    candidates
}

fn cardinal_neighbors(pos: TilePos) -> [TilePos; 4] {
    [
        TilePos::new(pos.x + 1, pos.y),
        TilePos::new(pos.x - 1, pos.y),
        TilePos::new(pos.x, pos.y + 1),
        TilePos::new(pos.x, pos.y - 1),
    ]
}

fn assign_defenders_to_threat(
    state: &mut GameState,
    keeper: &crate::state::rival_keeper::RivalKeeperAiState,
) -> bool {
    let Some(threat_pos) = nearest_threat_position(state, &keeper.owner) else {
        return false;
    };

    let defenders: Vec<EntityId> = state
        .entities
        .all()
        .filter(|entity| entity.owner == keeper.owner && entity.as_creature().is_some())
        .map(|entity| entity.id)
        .take(keeper.raid_size.max(1))
        .collect();

    for defender_id in &defenders {
        if let Some(entity) = state.entities.get_mut(*defender_id) {
            if let Some(creature) = entity.as_creature_mut() {
                creature.current_task = Some(Task::MoveTo(threat_pos));
                creature.current_path = None;
            }
        }
    }

    !defenders.is_empty()
}

fn nearest_threat_position(state: &GameState, owner: &OwnerId) -> Option<TilePos> {
    state
        .entities
        .all()
        .filter(|entity| entity.owner.is_hostile_to(owner))
        .filter(|entity| is_near_owned_tile(state, owner, entity.pos, DEFENSE_RADIUS))
        .min_by_key(|entity| nearest_owned_distance(state, owner, entity.pos).unwrap_or(i32::MAX))
        .map(|entity| entity.pos)
}

fn is_near_owned_tile(state: &GameState, owner: &OwnerId, pos: TilePos, radius: i32) -> bool {
    nearest_owned_distance(state, owner, pos)
        .map(|distance| distance <= radius)
        .unwrap_or(false)
}

fn nearest_owned_distance(state: &GameState, owner: &OwnerId, pos: TilePos) -> Option<i32> {
    state
        .dungeon
        .grid
        .iter()
        .flatten()
        .filter(|tile| &tile.owner == owner)
        .map(|tile| tile.pos.manhattan_distance(&pos))
        .min()
}

fn launch_heart_raid(state: &mut GameState, owner: &OwnerId, raid_size: usize) -> bool {
    let Some(heart_pos) = state.find_dungeon_heart_position() else {
        return false;
    };

    let raiders: Vec<EntityId> = state
        .entities
        .all()
        .filter(|entity| entity.owner == *owner && entity.as_creature().is_some())
        .filter(|entity| {
            entity
                .as_creature()
                .and_then(|creature| creature.current_task.as_ref())
                != Some(&Task::MoveTo(heart_pos))
        })
        .map(|entity| entity.id)
        .take(raid_size.max(1))
        .collect();

    let launched = !raiders.is_empty();
    for entity_id in raiders {
        if let Some(entity) = state.entities.get_mut(entity_id) {
            if let Some(creature) = entity.as_creature_mut() {
                creature.current_task = Some(Task::MoveTo(heart_pos));
                creature.current_path = None;
            }
        }
    }

    launched
}

fn find_owned_spawn_tile(state: &GameState, owner: &OwnerId) -> Option<TilePos> {
    for row in &state.dungeon.grid {
        for tile in row {
            if &tile.owner == owner {
                return Some(tile.pos);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clear_rival_keeper_area(state: &mut GameState, owner: &OwnerId) {
        for row in &mut state.dungeon.grid {
            for tile in row {
                if &tile.owner == owner {
                    tile.set_owner(OwnerId::Neutral);
                    if tile.tile_type == tt::DUNGEON_HEART {
                        tile.tile_type = "earth".to_string();
                    }
                }
            }
        }
    }

    fn remove_rival_keeper_creatures(state: &mut GameState, owner: &OwnerId) {
        let ids: Vec<EntityId> = state
            .entities
            .all()
            .filter(|entity| &entity.owner == owner)
            .map(|entity| entity.id)
            .collect();
        for id in ids {
            state.entities.remove(id);
        }
    }

    fn set_tile(state: &mut GameState, pos: TilePos, tile_type: &str, owner: OwnerId) {
        let tile = state
            .dungeon
            .get_tile_mut(pos)
            .expect("test tile should exist");
        tile.tile_type = tile_type.to_string();
        tile.resources_remaining = None;
        tile.room_id = None;
        tile.marked_for_dig = false;
        tile.set_owner(owner);
    }

    #[test]
    fn rival_keeper_plan_requests_reinforcements() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_for_scenario(&game_data, "dark_beginnings");
        let owner = OwnerId::RivalKeeper(1);
        remove_rival_keeper_creatures(&mut state, &owner);
        state.rival_keepers.keepers[0].next_decision_time = 0.0;
        state.rival_keepers.keepers[0].desired_rooms.clear();
        state.dungeon.grid[1][1].set_owner(owner.clone());

        let plans = plan_rival_keepers(&state);
        assert!(plans[0].wants_reinforcements);

        let before_count = state
            .entities
            .all()
            .filter(|entity| entity.owner == owner)
            .count();
        update_rival_keeper_ai(&mut state, &game_data, 1.0);
        let rival_count = state
            .entities
            .all()
            .filter(|entity| entity.owner == owner)
            .count();
        assert!(rival_count > before_count);
    }

    #[test]
    fn rival_keeper_launches_attack_when_garrison_is_ready() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_for_scenario(&game_data, "dark_beginnings");
        let owner = OwnerId::RivalKeeper(1);
        remove_rival_keeper_creatures(&mut state, &owner);
        state.rival_keepers.keepers[0].next_decision_time = 0.0;
        state.rival_keepers.keepers[0].next_attack_time = 0.0;
        state.rival_keepers.keepers[0].desired_rooms.clear();
        state.dungeon.grid[1][1].set_owner(owner.clone());
        let min_garrison = state.rival_keepers.keepers[0].min_garrison;
        let raid_size = state.rival_keepers.keepers[0].raid_size;
        let monster_data = game_data.monsters.get("goblin").unwrap();
        for offset in 0..min_garrison {
            let creature = CreatureState::new(
                "goblin".to_string(),
                1,
                monster_data.stats.health,
                monster_data.stats.mana,
                offset as u64,
            );
            state.entities.spawn_creature_for_owner(
                TilePos::new(1 + offset as i32, 1),
                creature,
                owner.clone(),
            );
        }
        let heart_pos = state.find_dungeon_heart_position().unwrap();

        let plans = plan_rival_keepers(&state);
        assert!(plans[0].wants_attack);
        update_rival_keeper_ai(&mut state, &game_data, 1.0);

        let attackers = state
            .entities
            .all()
            .filter(|entity| entity.owner == owner)
            .filter_map(|entity| entity.as_creature())
            .filter(|creature| creature.current_task == Some(Task::MoveTo(heart_pos)))
            .count();
        assert_eq!(attackers, raid_size);
        assert!(state.rival_keepers.keepers[0].next_attack_time > 0.0);
    }

    #[test]
    fn rival_keeper_first_attack_waits_for_grace_period_then_gaps_grow() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_for_scenario(&game_data, "dark_beginnings");
        let owner = OwnerId::RivalKeeper(1);

        // The opening raid is gated by the grace period, not the repeat cooldown
        let keeper = &state.rival_keepers.keepers[0];
        assert_eq!(keeper.next_attack_time, keeper.first_attack_delay);
        assert!(keeper.first_attack_delay > keeper.attack_cooldown);

        // Force a raid and check the cooldown escalates for the next one
        remove_rival_keeper_creatures(&mut state, &owner);
        state.rival_keepers.keepers[0].next_decision_time = 0.0;
        state.rival_keepers.keepers[0].next_attack_time = 0.0;
        state.rival_keepers.keepers[0].desired_rooms.clear();
        state.dungeon.grid[1][1].set_owner(owner.clone());
        let min_garrison = state.rival_keepers.keepers[0].min_garrison;
        let cooldown_before = state.rival_keepers.keepers[0].attack_cooldown;
        let growth = state.rival_keepers.keepers[0].attack_cooldown_growth;
        assert!(growth > 1.0);
        let monster_data = game_data.monsters.get("goblin").unwrap();
        for offset in 0..min_garrison {
            let creature = CreatureState::new(
                "goblin".to_string(),
                1,
                monster_data.stats.health,
                monster_data.stats.mana,
                offset as u64,
            );
            state.entities.spawn_creature_for_owner(
                TilePos::new(1 + offset as i32, 1),
                creature,
                owner.clone(),
            );
        }

        update_rival_keeper_ai(&mut state, &game_data, 1.0);

        let keeper = &state.rival_keepers.keepers[0];
        assert_eq!(keeper.attack_cooldown, cooldown_before * growth);
        assert_eq!(keeper.next_attack_time, keeper.attack_cooldown);
    }

    #[test]
    fn rival_keeper_digs_owned_expansion() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_for_scenario(&game_data, "dark_beginnings");
        let owner = OwnerId::RivalKeeper(1);
        clear_rival_keeper_area(&mut state, &owner);

        let owned = TilePos::new(2, 2);
        let target = TilePos::new(3, 2);
        set_tile(&mut state, owned, tt::CLAIMED_FLOOR, owner.clone());
        set_tile(&mut state, target, "earth", OwnerId::Neutral);
        set_tile(&mut state, TilePos::new(1, 2), "bedrock", OwnerId::Neutral);
        set_tile(&mut state, TilePos::new(2, 1), "bedrock", OwnerId::Neutral);
        set_tile(&mut state, TilePos::new(2, 3), "bedrock", OwnerId::Neutral);

        assert!(dig_expansion(&mut state, &game_data, &owner, 1));

        let tile = state.dungeon.get_tile(target).expect("target should exist");
        assert_eq!(tile.tile_type, tt::CLAIMED_FLOOR);
        assert_eq!(tile.owner, owner);
    }

    #[test]
    fn rival_keeper_places_desired_room_on_owned_floor() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_for_scenario(&game_data, "dark_beginnings");
        let owner = OwnerId::RivalKeeper(1);
        clear_rival_keeper_area(&mut state, &owner);

        let room_tiles = [
            TilePos::new(2, 2),
            TilePos::new(3, 2),
            TilePos::new(2, 3),
            TilePos::new(3, 3),
        ];
        for pos in room_tiles {
            set_tile(&mut state, pos, tt::CLAIMED_FLOOR, owner.clone());
        }

        assert!(place_room(&mut state, &game_data, &owner, "lair", 2));

        for pos in room_tiles {
            let tile = state.dungeon.get_tile(pos).expect("room tile should exist");
            assert_eq!(tile.tile_type, "lair");
            assert_eq!(tile.owner, owner);
            assert_eq!(tile.ownership, Ownership::Enemy);
        }
    }

    #[test]
    fn rival_keeper_defends_owned_area_against_nearby_threat() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_for_scenario(&game_data, "dark_beginnings");
        let owner = OwnerId::RivalKeeper(1);
        remove_rival_keeper_creatures(&mut state, &owner);
        clear_rival_keeper_area(&mut state, &owner);
        set_tile(
            &mut state,
            TilePos::new(2, 2),
            tt::CLAIMED_FLOOR,
            owner.clone(),
        );

        let monster_data = game_data.monsters.get("goblin").unwrap();
        let defender = CreatureState::new(
            "goblin".to_string(),
            1,
            monster_data.stats.health,
            monster_data.stats.mana,
            1,
        );
        state
            .entities
            .spawn_creature_for_owner(TilePos::new(2, 2), defender, owner.clone());
        let threat = CreatureState::new(
            "goblin".to_string(),
            1,
            monster_data.stats.health,
            monster_data.stats.mana,
            2,
        );
        let threat_pos = TilePos::new(3, 2);
        state
            .entities
            .spawn_creature_for_owner(threat_pos, threat, OwnerId::Player);

        let keeper = state.rival_keepers.keepers[0].clone();
        assert!(assign_defenders_to_threat(&mut state, &keeper));

        let defenders = state
            .entities
            .all()
            .filter(|entity| entity.owner == owner)
            .filter_map(|entity| entity.as_creature())
            .filter(|creature| creature.current_task == Some(Task::MoveTo(threat_pos)))
            .count();
        assert_eq!(defenders, 1);
    }
}
