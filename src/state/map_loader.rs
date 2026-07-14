use crate::data::GameData;
use crate::engine::tile_types::types as tt;
use crate::state::dungeon::Dungeon;
use crate::state::entities::{CreatureState, EntityManager, HeroState, StructureState};
use crate::state::faction::{owner_from_label, OwnerId};
use crate::state::rival_keeper::{
    default_attack_cooldown, default_attack_cooldown_growth, default_dig_batch,
    default_first_attack_delay, default_min_garrison, default_raid_size, default_room_size,
    RivalKeeperAiState, RivalKeeperRuntime,
};
use crate::state::tile_state::{TilePos, TileState};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct MapFile {
    pub name: String,
    pub description: String,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<String>,
    pub legend: HashMap<char, String>,
    #[serde(default)]
    pub tile_owners: Vec<TileOwnerRegion>,
    #[serde(default)]
    pub entities: Vec<MapEntity>,
    #[serde(default)]
    pub rival_keepers: Vec<MapRivalKeeper>,
}

#[derive(Debug, Deserialize)]
pub struct MapEntity {
    pub id: String,
    pub x: i32,
    pub y: i32,
    #[serde(default = "default_owner_label")]
    pub owner: String,
    #[serde(default = "default_level")]
    pub level: u32,
}

#[derive(Debug, Deserialize)]
pub struct TileOwnerRegion {
    pub owner: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize)]
pub struct MapRivalKeeper {
    pub owner: OwnerId,
    pub ai_profile: String,
    #[serde(default)]
    pub desired_rooms: Vec<String>,
    #[serde(default)]
    pub preferred_creatures: Vec<String>,
    #[serde(default = "default_min_garrison")]
    pub min_garrison: usize,
    #[serde(default = "default_raid_size")]
    pub raid_size: usize,
    #[serde(default = "default_dig_batch")]
    pub dig_batch: usize,
    #[serde(default = "default_room_size")]
    pub room_size: usize,
    #[serde(default = "default_attack_cooldown")]
    pub attack_cooldown: f32,
    #[serde(default = "default_first_attack_delay")]
    pub first_attack_delay: f32,
    #[serde(default = "default_attack_cooldown_growth")]
    pub attack_cooldown_growth: f32,
}

pub fn load_map(
    path: &str,
    game_data: &GameData,
    entities: &mut EntityManager,
) -> Result<Dungeon, Box<dyn Error>> {
    let json_content = std::fs::read_to_string(path)?;
    let map_data: MapFile = serde_json::from_str(&json_content)?;

    // Validate dimensions
    if map_data.tiles.len() != map_data.height {
        return Err(format!(
            "Map height mismatch. Expected {}, got {}",
            map_data.height,
            map_data.tiles.len()
        )
        .into());
    }

    let mut grid = Vec::with_capacity(map_data.height);

    for (y, row_str) in map_data.tiles.iter().enumerate() {
        if row_str.len() != map_data.width {
            return Err(format!(
                "Map width mismatch on row {}. Expected {}, got {}",
                y,
                map_data.width,
                row_str.len()
            )
            .into());
        }

        let mut row = Vec::with_capacity(map_data.width);
        for (x, char) in row_str.chars().enumerate() {
            let tile_type = map_data
                .legend
                .get(&char)
                .ok_or_else(|| format!("Unknown tile character '{}' at ({}, {})", char, x, y))?;

            let pos = TilePos::new(x as i32, y as i32);
            let mut tile = TileState::new(tile_type.clone(), pos);

            // Set resources if applicable
            if let Some(tile_data) = game_data.tiles.get(tile_type) {
                if let Some(resources) = &tile_data.resources {
                    if resources.amount > 0 {
                        tile = tile.with_resources(resources.amount as u32);
                    }
                }
            }
            // Logic for specific tile types (e.g. gold veins default amount override if needed)
            if tile_type == "gold_vein" {
                tile = tile.with_resources(100);
            }

            row.push(tile);
        }
        grid.push(row);
    }

    // Create Dungeon instance
    let mut dungeon = Dungeon {
        grid,
        width: map_data.width,
        height: map_data.height,
    };

    apply_tile_owner_regions(&mut dungeon, &map_data.tile_owners);
    claim_map_heart_areas(&mut dungeon);

    // Spawn entities
    for entity_def in map_data.entities {
        let pos = TilePos::new(entity_def.x, entity_def.y);
        let owner = owner_from_label(&entity_def.owner);

        // Check if it's a monster/creature
        if let Some(monster_data) = game_data.monsters.get(&entity_def.id) {
            let visual_seed = macroquad_toolkit::rng::random_u64();
            let creature_state = CreatureState::new(
                entity_def.id.clone(),
                entity_def.level,
                monster_data.stats.health,
                monster_data.stats.mana,
                visual_seed,
            );
            entities.spawn_creature_for_owner(pos, creature_state, owner.clone());
        } else if let Some(hero_data) = game_data.heroes.get(&entity_def.id) {
            let visual_seed = macroquad_toolkit::rng::random_u64();
            let hero_state = HeroState::new(
                entity_def.id.clone(),
                entity_def.level,
                hero_data.stats.health,
                hero_data.stats.mana,
                pos,
                hero_data.stats.dig_time,
                visual_seed,
            );
            entities.spawn_hero_for_owner(pos, hero_state, owner.clone());
        } else if let Some(building_data) = game_data.hero_buildings.get(&entity_def.id) {
            let structure_state =
                StructureState::new(entity_def.id.clone(), building_data.hp as f32);
            entities.spawn_structure_for_owner(pos, structure_state, owner.clone());
        }
        // Could handle other entity types here (traps, items, etc.)

        // If an entity is placed on the dungeon heart, make sure its owner wins.
        if let Some(tile) = dungeon.get_tile(pos) {
            if tile.tile_type == "dungeon_heart" && owner.is_dungeon_keeper() {
                claim_heart_area(&mut dungeon, pos, owner.clone());
            }
        }
    }

    Ok(dungeon)
}

pub fn load_rival_keeper_runtime(path: &str) -> Result<RivalKeeperRuntime, Box<dyn Error>> {
    let json_content = std::fs::read_to_string(path)?;
    let map_data: MapFile = serde_json::from_str(&json_content)?;
    Ok(rival_keeper_runtime_from_map(&map_data))
}

pub fn rival_keeper_runtime_from_map(map_data: &MapFile) -> RivalKeeperRuntime {
    RivalKeeperRuntime {
        keepers: map_data
            .rival_keepers
            .iter()
            .map(|keeper| RivalKeeperAiState {
                owner: keeper.owner.clone(),
                ai_profile: keeper.ai_profile.clone(),
                preferred_creatures: keeper.preferred_creatures.clone(),
                desired_rooms: keeper.desired_rooms.clone(),
                next_decision_time: 1.0,
                min_garrison: keeper.min_garrison,
                raid_size: keeper.raid_size,
                dig_batch: keeper.dig_batch,
                room_size: keeper.room_size,
                attack_cooldown: keeper.attack_cooldown,
                first_attack_delay: keeper.first_attack_delay,
                attack_cooldown_growth: keeper.attack_cooldown_growth.max(1.0),
                next_attack_time: keeper.first_attack_delay,
            })
            .collect(),
    }
}

fn claim_map_heart_areas(dungeon: &mut Dungeon) {
    let heart_positions: Vec<TilePos> = dungeon
        .grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|tile| tile.tile_type == "dungeon_heart")
        .map(|tile| tile.pos)
        .collect();

    for pos in heart_positions {
        let owner = dungeon
            .get_tile(pos)
            .map(|tile| {
                if tile.owner == OwnerId::Neutral {
                    OwnerId::Player
                } else {
                    tile.owner.clone()
                }
            })
            .unwrap_or(OwnerId::Player);
        if owner.is_dungeon_keeper() {
            claim_heart_area(dungeon, pos, owner);
        }
    }
}

fn claim_heart_area(dungeon: &mut Dungeon, pos: TilePos, owner: OwnerId) {
    for dy in -1..=1 {
        for dx in -1..=1 {
            let claim_pos = TilePos::new(pos.x + dx, pos.y + dy);
            if let Some(tile) = dungeon.get_tile_mut(claim_pos) {
                if tile.tile_type != tt::DUNGEON_HEART && is_start_area_convertible(&tile.tile_type)
                {
                    tile.tile_type = tt::CLAIMED_FLOOR.to_string();
                    tile.resources_remaining = None;
                    tile.room_id = None;
                    tile.marked_for_dig = false;
                }
                if owner == OwnerId::Player {
                    tile.claim();
                } else {
                    tile.set_owner(owner.clone());
                }
            }
        }
    }
}

fn is_start_area_convertible(tile_type: &str) -> bool {
    !matches!(tile_type, "bedrock" | "solid_rock" | "hero_wall")
}

fn apply_tile_owner_regions(dungeon: &mut Dungeon, regions: &[TileOwnerRegion]) {
    for region in regions {
        let owner = owner_from_label(&region.owner);
        for y in region.y..region.y + region.height {
            for x in region.x..region.x + region.width {
                if let Some(tile) = dungeon.get_tile_mut(TilePos::new(x, y)) {
                    tile.set_owner(owner.clone());
                }
            }
        }
    }
}

fn default_owner_label() -> String {
    "neutral".to_string()
}

fn default_level() -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::input_handlers;
    use crate::state::tile_state::Ownership;

    #[test]
    fn file_map_claims_walkable_floor_around_player_heart() {
        let game_data = GameData::load().expect("game data should load");
        let mut entities = EntityManager::new();
        let dungeon = load_map("assets/maps/level_1.json", &game_data, &mut entities)
            .expect("level map should load");
        let heart_pos = dungeon.find_dungeon_heart().expect("heart should exist");

        for dy in -1..=1 {
            for dx in -1..=1 {
                let pos = TilePos::new(heart_pos.x + dx, heart_pos.y + dy);
                let tile = dungeon.get_tile(pos).expect("heart area tile exists");
                assert_eq!(tile.ownership, Ownership::Player);
                if pos == heart_pos {
                    assert_eq!(tile.tile_type, tt::DUNGEON_HEART);
                } else {
                    assert_eq!(tile.tile_type, tt::CLAIMED_FLOOR);
                }
            }
        }
    }

    #[test]
    fn blood_and_iron_map_loads_with_both_hearts_and_hero_base() {
        let game_data = GameData::load().expect("game data should load");
        let mut entities = EntityManager::new();
        let dungeon = load_map("assets/maps/blood_and_iron.json", &game_data, &mut entities)
            .expect("blood_and_iron map should load");

        assert_eq!(dungeon.width, 32);
        assert_eq!(dungeon.height, 32);
        // Player heart claimed for the player.
        let player_heart = dungeon.find_dungeon_heart().expect("player heart exists");
        assert_eq!(
            dungeon.get_tile(player_heart).unwrap().ownership,
            Ownership::Player
        );
        // The walled hero outpost placed its buildings as tiles (town_hall /
        // barracks / church), the same representation dark_beginnings uses; the
        // full GameState lifts these into `hero_base` for the destroy objective.
        let hero_building_tiles = dungeon
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|tile| matches!(tile.tile_type.as_str(), "town_hall" | "barracks" | "church"))
            .count();
        assert_eq!(
            hero_building_tiles, 3,
            "expected town_hall + barracks + church tiles"
        );
        // Rival keeper opens with a garrison.
        let rival_creatures = entities
            .all()
            .filter(|e| e.owner == OwnerId::RivalKeeper(1) && e.as_creature().is_some())
            .count();
        assert_eq!(rival_creatures, 3);
    }

    #[test]
    fn file_map_loads_rival_keeper_base_and_ai() {
        let game_data = GameData::load().expect("game data should load");
        let runtime = load_rival_keeper_runtime("assets/maps/level_1.json")
            .expect("rival keeper runtime should load");
        let keeper = runtime
            .keepers
            .iter()
            .find(|keeper| keeper.owner == OwnerId::RivalKeeper(1))
            .expect("keeper1 runtime should exist");
        assert_eq!(keeper.ai_profile, "builder_attacker");
        assert_eq!(keeper.dig_batch, 5);
        assert_eq!(keeper.room_size, 2);
        assert_eq!(keeper.attack_cooldown, 90.0);
        assert_eq!(keeper.first_attack_delay, 240.0);
        assert_eq!(keeper.attack_cooldown_growth, 1.5);
        // The first raid is gated by the grace period, not the repeat cooldown
        assert_eq!(keeper.next_attack_time, keeper.first_attack_delay);

        let mut entities = EntityManager::new();
        let dungeon = load_map("assets/maps/level_1.json", &game_data, &mut entities)
            .expect("level map should load");
        let heart_pos = TilePos::new(6, 23);
        let heart_tile = dungeon
            .get_tile(heart_pos)
            .expect("rival heart tile should exist");
        assert_eq!(heart_tile.tile_type, tt::DUNGEON_HEART);
        assert_eq!(heart_tile.owner, OwnerId::RivalKeeper(1));
        assert_eq!(heart_tile.ownership, Ownership::Enemy);

        let floor_tile = dungeon
            .get_tile(TilePos::new(5, 23))
            .expect("rival floor tile should exist");
        assert_eq!(floor_tile.tile_type, tt::CLAIMED_FLOOR);
        assert_eq!(floor_tile.owner, OwnerId::RivalKeeper(1));
        assert_eq!(floor_tile.ownership, Ownership::Enemy);

        let rival_creatures = entities
            .all()
            .filter(|entity| entity.owner == OwnerId::RivalKeeper(1))
            .filter(|entity| entity.as_creature().is_some())
            .count();
        assert_eq!(rival_creatures, 2);
    }

    #[test]
    fn blood_and_iron_boots_as_a_playable_mission() {
        // Full GameState construction is the real entry point missions run
        // through — proves the authored scenario+map load, the hero outpost is
        // live for the destroy objective, and the player heart is intact.
        let game_data = GameData::load().expect("game data should load");
        let state =
            crate::state::game_state::GameState::new_for_scenario(&game_data, "blood_and_iron");

        assert!(state.find_dungeon_heart_position().is_some());
        assert!(state.dungeon_heart_health > 0.0);
        assert!(
            state.hero_base.enabled,
            "hero outpost should be active for destroy_all_hero_buildings"
        );
        // The three core structures register in the base (the walled ring's
        // hero_wall/hero_gate tiles also count as buildings, as in level_1).
        for core in ["town_hall", "barracks", "church"] {
            assert!(
                state
                    .hero_base
                    .buildings
                    .iter()
                    .any(|b| b.building_type == core),
                "hero base should include a {core}"
            );
        }
        assert!(!state.hero_base.is_defeated(&state.entities));
    }

    #[test]
    fn the_long_dark_boots_as_an_economy_mission() {
        // M3 has no hero base (heroes only raid) — its win is gather-gold +
        // survive. Prove it boots, the player heart is live, and the map is
        // resource-rich enough to be an economy race (many gold/gem seams).
        let game_data = GameData::load().expect("game data should load");
        let state =
            crate::state::game_state::GameState::new_for_scenario(&game_data, "the_long_dark");

        assert!(state.find_dungeon_heart_position().is_some());
        assert!(state.dungeon_heart_health > 0.0);

        let mut entities = EntityManager::new();
        let dungeon = load_map("assets/maps/the_long_dark.json", &game_data, &mut entities)
            .expect("the_long_dark map should load");
        let resource_tiles = dungeon
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|t| matches!(t.tile_type.as_str(), "gold_vein" | "gem_seam"))
            .count();
        assert!(
            resource_tiles >= 20,
            "economy map should be resource-rich, found {resource_tiles} seams"
        );
    }

    #[test]
    fn no_prisoners_boots_with_prison_tools_and_hero_outpost() {
        // M4 (Act I finale) introduces the prison→skeleton / torture pipeline
        // and asks the player to raze a fortified outpost. Prove it boots, the
        // capture rooms are available, and the outpost is live to destroy.
        let game_data = GameData::load().expect("game data should load");
        let scenario = game_data
            .scenarios
            .get("no_prisoners")
            .expect("no_prisoners scenario should be loaded");
        for room in ["prison", "torture_chamber"] {
            assert!(
                scenario.availability.rooms.contains_key(room),
                "capture mission should offer the {room}"
            );
        }

        let state =
            crate::state::game_state::GameState::new_for_scenario(&game_data, "no_prisoners");
        assert!(state.dungeon_heart_health > 0.0);
        assert!(state.hero_base.enabled, "the outpost must be live to raze");
        for core in ["town_hall", "barracks", "church", "armory"] {
            assert!(
                state
                    .hero_base
                    .buildings
                    .iter()
                    .any(|b| b.building_type == core),
                "hero outpost should include a {core}"
            );
        }
    }

    #[test]
    fn whispers_in_the_circle_boots_with_ritual_tools() {
        // M5 (Act II opener) introduces the ritual circle + mana economy +
        // offensive spellcasting. Prove it boots, the ritual circle and warlock
        // are available, and the mage-tower outpost is live to raze.
        let game_data = GameData::load().expect("game data should load");
        let scenario = game_data
            .scenarios
            .get("whispers_in_the_circle")
            .expect("whispers_in_the_circle scenario should be loaded");
        assert!(scenario.availability.rooms.contains_key("ritual_circle"));
        assert!(scenario.availability.creatures.contains_key("warlock"));

        let state = crate::state::game_state::GameState::new_for_scenario(
            &game_data,
            "whispers_in_the_circle",
        );
        assert!(state.dungeon_heart_health > 0.0);
        assert!(
            state.hero_base.enabled,
            "the mage-tower outpost must be live"
        );
        assert!(
            state
                .hero_base
                .buildings
                .iter()
                .any(|b| b.building_type == "mage_tower"),
            "the outpost should feature a mage_tower"
        );
    }

    #[test]
    fn the_kennels_boots_with_beast_roster_and_larger_army_cap() {
        // M6 scales the army: kennel/barracks/scavenger + hellhound/spider/lizard
        // and a bigger creature cap. Prove it boots with those tools available.
        let game_data = GameData::load().expect("game data should load");
        let scenario = game_data
            .scenarios
            .get("the_kennels")
            .expect("the_kennels scenario should be loaded");
        for room in ["kennel", "barracks", "scavenger"] {
            assert!(
                scenario.availability.rooms.contains_key(room),
                "army mission should offer the {room}"
            );
        }
        for beast in ["hellhound", "spider", "lizard"] {
            assert!(
                scenario.availability.creatures.contains_key(beast),
                "army mission should offer the {beast}"
            );
        }
        // Meaningfully larger roster cap than the Act I missions (15–18).
        let player = scenario
            .players
            .iter()
            .find(|p| p.id == "keeper0")
            .expect("player slot exists");
        assert!(player.max_creatures.unwrap_or(0) >= 22);

        let state =
            crate::state::game_state::GameState::new_for_scenario(&game_data, "the_kennels");
        assert!(state.dungeon_heart_health > 0.0);
        assert!(state.hero_base.enabled, "the outpost must be live to raze");
    }

    #[test]
    fn m7_branch_missions_boot_with_their_signature_tools() {
        // The Act II branch: M7a (graveyard→vampire) and M7b (temple→demons).
        // Each is a full, bootable mission with its path's signature room.
        let game_data = GameData::load().expect("game data should load");

        let graveyard = game_data
            .scenarios
            .get("restless_dead")
            .expect("restless_dead scenario should be loaded");
        assert!(graveyard.availability.rooms.contains_key("graveyard"));

        let temple = game_data
            .scenarios
            .get("pacts_and_sacrifice")
            .expect("pacts_and_sacrifice scenario should be loaded");
        assert!(temple.availability.rooms.contains_key("temple"));
        assert!(temple.availability.creatures.contains_key("demon_spawn"));

        for id in ["restless_dead", "pacts_and_sacrifice"] {
            let state = crate::state::game_state::GameState::new_for_scenario(&game_data, id);
            assert!(
                state.dungeon_heart_health > 0.0,
                "{id} heart should be live"
            );
            assert!(
                state.hero_base.enabled,
                "{id} outpost should be live to raze"
            );
        }
    }

    #[test]
    fn default_scenario_starts_with_markable_dig_targets_near_heart() {
        let game_data = GameData::load().expect("game data should load");
        let mut state =
            crate::state::game_state::GameState::new_for_scenario(&game_data, "dark_beginnings");
        let heart_pos = state
            .find_dungeon_heart_position()
            .expect("heart should exist");
        let dig_pos = TilePos::new(heart_pos.x + 2, heart_pos.y);

        let tile = state.get_tile(dig_pos).expect("dig target should exist");
        assert_eq!(tile.tile_type, "earth");
        assert_eq!(tile.ownership, Ownership::Unclaimed);

        input_handlers::handle_dig(&mut state, &game_data, dig_pos);

        let tile = state
            .get_tile(dig_pos)
            .expect("dig target should still exist");
        assert!(tile.marked_for_dig);
    }
}
