use crate::data::GameData;
use crate::engine::tile_types::{self, types as tt};
use crate::state::entities::EntityId;
use crate::state::game_state::GameState;
use crate::state::{InteractionMode, Ownership, TilePos};
use macroquad::prelude::*;

/// Handle toggling or marking a tile for digging via single click
pub fn handle_dig(state: &mut GameState, game_data: &GameData, tile_pos: TilePos) {
    if let Some(tile) = state.get_tile_mut(tile_pos) {
        if tile_types::is_diggable(&tile.tile_type, game_data)
            && tile.ownership == Ownership::Unclaimed
        {
            tile.marked_for_dig = !tile.marked_for_dig;
        }
    }
}

/// Mark/unmark multiple tiles for digging
pub fn handle_dig_multi(state: &mut GameState, game_data: &GameData, tiles: &[TilePos]) {
    if tiles.is_empty() {
        return;
    }

    // Determine intent based on the first tile
    let should_mark = if let Some(first_tile) = state.get_tile(tiles[0]) {
        !first_tile.marked_for_dig
    } else {
        true
    };

    for &tile_pos in tiles {
        if let Some(tile) = state.get_tile_mut(tile_pos) {
            if tile.ownership == Ownership::Unclaimed {
                if should_mark {
                    if tile_types::is_diggable(&tile.tile_type, game_data) {
                        tile.marked_for_dig = true;
                    }
                } else {
                    tile.marked_for_dig = false;
                }
            }
        }
    }
}

pub fn handle_build_room(
    state: &mut GameState,
    game_data: &GameData,
    room_type: &str,
    tile_pos: TilePos,
) {
    if room_type == tt::BRIDGE {
        handle_build_bridge(state, game_data, tile_pos);
        return;
    }

    let cost = game_data
        .rooms
        .get(room_type)
        .map(|data| data.build.cost_per_tile)
        .unwrap_or_else(|| panic!("Room type '{}' missing in rooms.json", room_type));

    let can_build = state.player.gold >= cost;
    let is_valid_tile = if let Some(tile) = state.get_tile(tile_pos) {
        tile.ownership == Ownership::Player
            && tile.room_id.is_none()
            && tile_types::can_build_room(&tile.tile_type, game_data)
    } else {
        false
    };

    if !state.player.is_room_unlocked(room_type) {
        eprintln!("Cannot build {}: Not yet unlocked!", room_type);
        return;
    }

    if can_build && is_valid_tile {
        state.player.gold -= cost;
        if let Some(tile) = state.get_tile_mut(tile_pos) {
            tile.tile_type = room_type.to_string();
        }
        state.detect_and_update_rooms(game_data);
    }
}

pub fn handle_build_room_multi(
    state: &mut GameState,
    game_data: &GameData,
    room_type: &str,
    tiles: &[TilePos],
) {
    if room_type == tt::BRIDGE {
        for &tile_pos in tiles {
            handle_build_bridge(state, game_data, tile_pos);
        }
        return;
    }

    let cost_per_tile = game_data
        .rooms
        .get(room_type)
        .map(|data| data.build.cost_per_tile)
        .unwrap_or_else(|| panic!("Room type '{}' missing in rooms.json", room_type));

    if !state.player.is_room_unlocked(room_type) {
        eprintln!("Cannot build {}: Not yet unlocked!", room_type);
        return;
    }

    // Count valid tiles first
    let valid_tiles: Vec<TilePos> = tiles
        .iter()
        .filter(|&&pos| {
            if let Some(tile) = state.get_tile(pos) {
                tile.ownership == Ownership::Player
                    && tile.room_id.is_none()
                    && tile_types::can_build_room(&tile.tile_type, game_data)
            } else {
                false
            }
        })
        .copied()
        .collect();

    // Calculate total cost and check if we can afford it
    let total_cost = cost_per_tile * valid_tiles.len() as i32;
    if state.player.gold < total_cost {
        eprintln!(
            "Cannot build {}: Not enough gold! Need {}, have {}",
            room_type, total_cost, state.player.gold
        );
        return;
    }

    // Apply the build to all valid tiles
    if !valid_tiles.is_empty() {
        state.player.gold -= total_cost;
        for tile_pos in valid_tiles {
            if let Some(tile) = state.get_tile_mut(tile_pos) {
                tile.tile_type = room_type.to_string();
            }
        }
        state.detect_and_update_rooms(game_data);
    }
}

pub fn handle_build_trap(
    state: &mut GameState,
    game_data: &GameData,
    trap_type: &str,
    tile_pos: TilePos,
) {
    if !state.player.is_trap_unlocked(trap_type) || !game_data.traps.contains_key(trap_type) {
        eprintln!("Cannot build trap: {} is not available.", trap_type);
        return;
    }

    // Check for Workshop requirement
    let has_workshop = state
        .room_manager
        .rooms
        .iter()
        .any(|r| r.active && r.room_type == "workshop");
    if !has_workshop {
        eprintln!("Cannot build trap: No functioning Workshop!");
        return;
    }

    let can_place = state.get_tile(tile_pos).is_some_and(|tile| {
        tile.ownership == Ownership::Player
            && tile_types::can_build_room(&tile.tile_type, game_data)
            && tile.trap.is_none()
    });
    if !can_place {
        return;
    }

    if !state.player.consume_trap_inventory(trap_type, 1) {
        eprintln!("Cannot build trap: No {} crates in inventory.", trap_type);
        return;
    }

    if let Some(tile) = state.get_tile_mut(tile_pos) {
        tile.trap = Some(crate::state::tile_state::TrapState {
            trap_type: trap_type.to_string(),
            constructed: false,
            construction_progress: 0.0,
            active: false,
            funded: true,
            cooldown: 0.0,
            triggered: false,
        });

        state.pending_trap_builds.insert(tile_pos);
        eprintln!(
            "Trap '{}' placement started at {:?}. Waiting for construction.",
            trap_type, tile_pos
        );
    }
}

pub fn handle_build_trap_multi(
    state: &mut GameState,
    game_data: &GameData,
    trap_type: &str,
    tiles: &[TilePos],
) {
    if !state.player.is_trap_unlocked(trap_type) || !game_data.traps.contains_key(trap_type) {
        eprintln!("Cannot build trap: {} is not available.", trap_type);
        return;
    }

    // Check for Workshop requirement
    let has_workshop = state
        .room_manager
        .rooms
        .iter()
        .any(|r| r.active && r.room_type == "workshop");
    if !has_workshop {
        eprintln!("Cannot build trap: No functioning Workshop!");
        return;
    }

    let valid_tiles: Vec<TilePos> = tiles
        .iter()
        .copied()
        .filter(|&tile_pos| {
            state.get_tile(tile_pos).is_some_and(|tile| {
                tile.ownership == Ownership::Player
                    && tile_types::can_build_room(&tile.tile_type, game_data)
                    && tile.trap.is_none()
            })
        })
        .take(state.player.trap_inventory_count(trap_type) as usize)
        .collect();

    for tile_pos in valid_tiles {
        if !state.player.consume_trap_inventory(trap_type, 1) {
            break;
        }
        if let Some(tile) = state.get_tile_mut(tile_pos) {
            tile.trap = Some(crate::state::tile_state::TrapState {
                trap_type: trap_type.to_string(),
                constructed: false,
                construction_progress: 0.0,
                active: false,
                funded: true,
                cooldown: 0.0,
                triggered: false,
            });

            state.pending_trap_builds.insert(tile_pos);
        }
    }
}

fn handle_build_bridge(state: &mut GameState, game_data: &GameData, tile_pos: TilePos) {
    let cost = game_data
        .tiles
        .get(tt::BRIDGE)
        .and_then(|tile| tile.cost)
        .unwrap_or(50);

    if state.player.gold < cost || !can_build_bridge(state, tile_pos) {
        return;
    }

    state.player.gold -= cost;
    if let Some(tile) = state.get_tile_mut(tile_pos) {
        tile.tile_type = tt::BRIDGE.to_string();
        tile.claim();
    }
}

fn can_build_bridge(state: &GameState, tile_pos: TilePos) -> bool {
    let Some(tile) = state.get_tile(tile_pos) else {
        return false;
    };
    if !matches!(tile.tile_type.as_str(), "water" | "lava") {
        return false;
    }

    [
        TilePos::new(tile_pos.x + 1, tile_pos.y),
        TilePos::new(tile_pos.x - 1, tile_pos.y),
        TilePos::new(tile_pos.x, tile_pos.y + 1),
        TilePos::new(tile_pos.x, tile_pos.y - 1),
    ]
    .into_iter()
    .any(|neighbor| {
        state
            .get_tile(neighbor)
            .map(|tile| {
                tile.ownership == Ownership::Player
                    && !matches!(tile.tile_type.as_str(), "water" | "lava")
            })
            .unwrap_or(false)
    })
}

pub fn handle_place_spawner(state: &mut GameState, game_data: &GameData, tile_pos: TilePos) {
    let spawner_cost = game_data
        .tiles
        .get("monster_spawner")
        .and_then(|t| t.cost)
        .unwrap_or(50);
    if state.player.gold < spawner_cost {
        return;
    }

    let tile = match state.get_tile(tile_pos) {
        Some(t) => t,
        None => return,
    };

    if tile.ownership != Ownership::Player
        || !tile_types::can_build_room(&tile.tile_type, game_data)
    {
        return;
    }

    state.player.gold -= spawner_cost;
    if let Some(tile_mut) = state.get_tile_mut(tile_pos) {
        tile_mut.tile_type = tt::MONSTER_SPAWNER.to_string();
    }
}

pub fn handle_place_spawner_multi(state: &mut GameState, game_data: &GameData, tiles: &[TilePos]) {
    let cost = game_data
        .tiles
        .get("monster_spawner")
        .and_then(|t| t.cost)
        .unwrap_or(50);

    // Count valid tiles
    let valid_tiles: Vec<TilePos> = tiles
        .iter()
        .filter(|&&pos| {
            if let Some(tile) = state.get_tile(pos) {
                tile.ownership == Ownership::Player
                    && tile_types::can_build_room(&tile.tile_type, game_data)
            } else {
                false
            }
        })
        .copied()
        .collect();

    // Calculate total cost
    let total_cost = cost * valid_tiles.len() as i32;
    if state.player.gold < total_cost {
        eprintln!(
            "Cannot place spawners: Not enough gold! Need {}, have {}",
            total_cost, state.player.gold
        );
        return;
    }

    // Apply
    if !valid_tiles.is_empty() {
        state.player.gold -= total_cost;
        for tile_pos in valid_tiles {
            if let Some(tile) = state.get_tile_mut(tile_pos) {
                tile.tile_type = tt::MONSTER_SPAWNER.to_string();
            }
        }
    }
}

pub fn handle_pickup(
    state: &mut GameState,
    held_entity: &mut Option<EntityId>,
    interaction_mode: &mut InteractionMode,
    tile_pos: TilePos,
) {
    // Fog check: Cannot interact with hidden tiles
    if let Some(tile) = state.get_tile(tile_pos) {
        if tile.fog_state == crate::state::tile_state::FogState::Hidden {
            return;
        }
    }

    // Find pickable entity
    let mut pickable_id = None;

    if let Some(entity) = state.entities.at_position(tile_pos).next() {
        let can_pickup = match &entity.entity_type {
            crate::state::entities::EntityType::Creature(_) => true,
            crate::state::entities::EntityType::Hero(h) => h.is_captured,
            crate::state::entities::EntityType::Structure(_) => false,
            crate::state::entities::EntityType::ResourcePile(_) => false,
        };

        if can_pickup {
            pickable_id = Some(entity.id);
        }
    }

    if let Some(id) = pickable_id {
        *held_entity = Some(id);
        eprintln!("Picked up entity: {}", id);
        *interaction_mode = InteractionMode::Drop;
    }
}

pub fn handle_drop(
    state: &mut GameState,
    held_entity: &mut Option<EntityId>,
    interaction_mode: &mut InteractionMode,
    tile_pos: TilePos,
    game_data: &GameData,
) {
    let entity_id = match *held_entity {
        Some(id) => id,
        None => return,
    };

    let tile = match state.get_tile(tile_pos) {
        Some(t) => t,
        None => return,
    };

    let is_walkable =
        tile.ownership == Ownership::Player && tile_types::is_tile_walkable(tile, game_data);
    if !is_walkable {
        return;
    }

    let should_try_sacrifice = state
        .entities
        .get(entity_id)
        .map(|entity| {
            entity.owner == crate::state::OwnerId::Player && entity.as_creature().is_some()
        })
        .unwrap_or(false);
    if should_try_sacrifice
        && crate::engine::special_rooms::sacrifice_creature_at(
            state, game_data, entity_id, tile_pos,
        )
    {
        *held_entity = None;
        *interaction_mode = InteractionMode::Pickup;
        return;
    }

    let entity = match state.entities.get_mut(entity_id) {
        Some(e) => e,
        None => return,
    };

    entity.pos = tile_pos;
    if let Some(creature) = entity.as_creature_mut() {
        creature.current_path = None;
        creature.current_task = None;
        creature.task_time = 0.0;
        creature.move_timer = 0.0;
    }

    eprintln!("Dropped entity {} at {:?}", entity_id, tile_pos);
    *held_entity = None;
    *interaction_mode = InteractionMode::Pickup;
}

pub fn handle_sell(state: &mut GameState, game_data: &GameData, tile_pos: TilePos) {
    let mut action = None;
    if let Some(tile) = state.get_tile(tile_pos) {
        if tile.marked_for_dig {
            action = Some("unmark");
        } else if state.player.is_room_unlocked(&tile.tile_type)
            && tile.ownership == Ownership::Player
        {
            action = Some("sell");
        }
    }

    match action {
        Some("unmark") => {
            if let Some(tile) = state.get_tile_mut(tile_pos) {
                tile.marked_for_dig = false;
            }
        }
        Some("sell") => {
            // Calculate refund percentage
            let refund_pct = game_data.config.economy.room_sell_refund_percentage;

            // Default refund logic if we can't determine the original cost
            // This is a bit tricky as we don't store what room was here
            // But tile_type usually stores the room ID string which maps to room config
            let mut refund_amount = 0;
            if let Some(tile) = state.get_tile(tile_pos) {
                if tile.room_id.is_some() {
                    // Try to resolve room type from tile type
                    if let Some(room_data) = game_data.rooms.get(&tile.tile_type) {
                        let cost = room_data.build.cost_per_tile;
                        let raw_refund = (cost as f32 * refund_pct).ceil() as i32;
                        // Round up to nearest 5
                        refund_amount = ((raw_refund + 4) / 5) * 5;
                    } else {
                        // Fallback if room data not found (shouldn't happen for valid rooms)
                        refund_amount = 5;
                    }
                }
            }

            state.player.gold += refund_amount;
            if let Some(tile) = state.get_tile_mut(tile_pos) {
                tile.tile_type = tt::CLAIMED_FLOOR.to_string();
                tile.room_id = None;
            }
            state.detect_and_update_rooms(game_data);
        }
        _ => {}
    }
}

/// Select an entity or room at the given position, returns true if something was selected
pub fn select_entity_or_room(
    state: &GameState,
    selected_entity: &mut Option<EntityId>,
    selected_room: &mut Option<usize>,
    tile_pos: TilePos,
) -> bool {
    // Fog check: Cannot interact with hidden tiles
    let tile = match state.get_tile(tile_pos) {
        Some(t) => t,
        None => {
            *selected_room = None;
            *selected_entity = None;
            return false;
        }
    };

    if tile.fog_state == crate::state::tile_state::FogState::Hidden {
        *selected_entity = None;
        *selected_room = None;
        return false;
    }

    if let Some(entity) = state.entities.at_position(tile_pos).next() {
        *selected_entity = Some(entity.id);
        *selected_room = None;
        return true;
    }

    *selected_entity = None;

    if let Some(room_id) = tile.room_id {
        *selected_room = Some(room_id);
        return true;
    }

    *selected_room = None;
    false
}

pub fn handle_inspect(
    state: &GameState,
    selected_entity: &mut Option<EntityId>,
    selected_room: &mut Option<usize>,
    tile_pos: TilePos,
) {
    // Reuse the same selection logic
    select_entity_or_room(state, selected_entity, selected_room, tile_pos);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::room_validator::Room;

    fn active_room(id: usize, room_type: &str, tiles: &[TilePos]) -> Room {
        let mut room = Room::new(
            id,
            room_type.to_string(),
            tiles.iter().copied().collect(),
            Vec::new(),
        );
        room.active = true;
        room
    }

    fn set_tile(state: &mut GameState, pos: TilePos, tile_type: &str, ownership: Ownership) {
        let tile = state.get_tile_mut(pos).expect("test tile should exist");
        tile.tile_type = tile_type.to_string();
        match ownership {
            Ownership::Player => tile.claim(),
            Ownership::Enemy => tile.set_owner(crate::state::OwnerId::RivalKeeper(1)),
            Ownership::Unclaimed => {
                tile.ownership = Ownership::Unclaimed;
                tile.owner = crate::state::OwnerId::Neutral;
            }
        }
    }

    #[test]
    fn bridge_building_claims_water_or_lava_crossing() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_for_scenario(&game_data, "dark_beginnings");
        let floor = TilePos::new(2, 2);
        let water = TilePos::new(3, 2);
        set_tile(&mut state, floor, tt::CLAIMED_FLOOR, Ownership::Player);
        set_tile(&mut state, water, "water", Ownership::Unclaimed);
        let before_gold = state.player.gold;

        handle_build_room(&mut state, &game_data, tt::BRIDGE, water);

        let tile = state.get_tile(water).expect("bridge tile should exist");
        assert_eq!(tile.tile_type, tt::BRIDGE);
        assert_eq!(tile.ownership, Ownership::Player);
        assert!(state.player.gold < before_gold);
    }

    #[test]
    fn trap_placement_consumes_manufactured_inventory() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_for_scenario(&game_data, "dark_beginnings");
        let trap_pos = TilePos::new(2, 2);
        set_tile(&mut state, trap_pos, tt::CLAIMED_FLOOR, Ownership::Player);
        state
            .room_manager
            .rooms
            .push(active_room(777, "workshop", &[TilePos::new(3, 3)]));
        state.player.unlock_trap("spike_trap".to_string());
        state.player.add_trap_inventory("spike_trap".to_string(), 1);

        handle_build_trap(&mut state, &game_data, "spike_trap", trap_pos);

        let tile = state.get_tile(trap_pos).expect("trap tile should exist");
        let trap = tile.trap.as_ref().expect("trap should be placed");
        assert_eq!(trap.trap_type, "spike_trap");
        assert!(trap.funded);
        assert_eq!(state.player.trap_inventory_count("spike_trap"), 0);
        assert!(state.pending_trap_builds.contains(&trap_pos));
    }
}
