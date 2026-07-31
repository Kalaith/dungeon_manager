use std::collections::HashSet;

use macroquad_toolkit::rng;

use crate::data::GameData;
use crate::engine::creature_ai;
use crate::engine::room_validator::Room;
use crate::engine::tile_types;
use crate::state::dungeon::Dungeon;
use crate::state::entities::{EntityManager, Task};
use crate::state::room_manager::RoomManager;
use crate::state::tile_state::TilePos;

/// Pick a walkable tile for wandering, drifting toward rooms creatures like.
///
/// Still random — the candidates are drawn exactly as before — but of the ones
/// that come up, the most pleasant wins. Without this an amenity room is
/// decorative: nothing in the AI seeks a room out unless it satisfies a
/// *need*, so a room whose whole mechanic is "creatures are happier here"
/// would never be visited on purpose. The same pull moves idle creatures out
/// of the torture chamber.
pub(super) fn pick_wander_position(
    dungeon: &Dungeon,
    current_pos: TilePos,
    room_manager: &RoomManager,
    game_data: &GameData,
) -> Option<TilePos> {
    let wander_radius = game_data.config.creature_ai.wander_radius;
    let wander_attempts = game_data.config.creature_ai.wander_attempts;

    let mut best: Option<(TilePos, f32)> = None;

    for _ in 0..wander_attempts {
        let dx = rng::gen_range(-wander_radius, wander_radius + 1);
        let dy = rng::gen_range(-wander_radius, wander_radius + 1);
        let candidate = TilePos::new(current_pos.x + dx, current_pos.y + dy);

        let Some(tile) = dungeon.get_tile(candidate) else {
            continue;
        };
        if !tile_types::is_walkable(&tile.tile_type, game_data) {
            continue;
        }

        let happiness = creature_ai::needs::room_happiness_at(candidate, room_manager, game_data);
        // Strictly greater, so equally-pleasant candidates keep the first —
        // and since draws are random, corridors stay as random as they were.
        if best.is_none_or(|(_, best_happiness)| happiness > best_happiness) {
            best = Some((candidate, happiness));
        }
    }

    best.map(|(pos, _)| pos)
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
