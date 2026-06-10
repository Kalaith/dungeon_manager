use std::collections::HashSet;

use macroquad_toolkit::rng;

use crate::data::GameData;
use crate::engine::room_validator::Room;
use crate::engine::tile_types;
use crate::state::dungeon::Dungeon;
use crate::state::entities::{EntityManager, Task};
use crate::state::tile_state::TilePos;

/// Pick a random walkable tile for wandering.
pub(super) fn pick_wander_position(
    dungeon: &Dungeon,
    current_pos: TilePos,
    game_data: &GameData,
) -> Option<TilePos> {
    let wander_radius = game_data.config.creature_ai.wander_radius;
    let wander_attempts = game_data.config.creature_ai.wander_attempts;
    let mut attempts = 0;

    while attempts < wander_attempts {
        let dx = rng::gen_range(-wander_radius, wander_radius + 1);
        let dy = rng::gen_range(-wander_radius, wander_radius + 1);
        let candidate = TilePos::new(current_pos.x + dx, current_pos.y + dy);

        if let Some(tile) = dungeon.get_tile(candidate) {
            if tile_types::is_walkable(&tile.tile_type, game_data) {
                return Some(candidate);
            }
        }
        attempts += 1;
    }
    None
}

/// Find an available work slot in a room.
pub(super) fn find_available_work_slot(room: &Room, entities: &EntityManager) -> Option<TilePos> {
    let mut occupied_slots = HashSet::new();

    for (_, creature) in entities.creatures() {
        if let Some(Task::Work(target_room_id, target_pos)) = &creature.current_task {
            if *target_room_id == room.id {
                occupied_slots.insert(*target_pos);
            }
        }
    }

    room.work_slots
        .iter()
        .find(|slot| !occupied_slots.contains(slot))
        .copied()
}
