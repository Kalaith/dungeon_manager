use crate::data::GameData;
use crate::state::dungeon::Dungeon;
use crate::state::entities::{EntityId, EntityManager};
use crate::state::tile_state::TilePos;

/// Process hero digging logic with flattened control flow
/// Returns (carved_wall_pos, should_move)
pub fn process_hero_digging(
    entities: &mut EntityManager,
    dungeon: &Dungeon,
    game_data: &GameData,
    hero_id: EntityId,
    dt: f32,
) -> (Option<TilePos>, bool) {
    // First, gather the info we need without holding mutable borrows
    let (next_pos, can_dig) = {
        let entity = match entities.get(hero_id) {
            Some(e) => e,
            None => return (None, true),
        };
        let hero_state = match entity.as_hero() {
            Some(h) => h,
            None => return (None, true),
        };
        let path = match &hero_state.current_path {
            Some(p) => p,
            None => return (None, true),
        };
        let next_pos = match path.first() {
            Some(&p) => p,
            None => return (None, true),
        };
        (next_pos, hero_state.can_dig)
    };

    // Check tile properties without holding entity borrow
    let is_wall = tile_blocks_movement(dungeon, next_pos, game_data);
    let is_hero_wall = dungeon
        .get_tile(next_pos)
        .map(|t| t.tile_type == "hero_wall")
        .unwrap_or(false);

    let is_diggable = dungeon
        .get_tile(next_pos)
        .and_then(|t| game_data.tiles.get(&t.tile_type))
        .map(|td| td.diggable)
        .unwrap_or(false);

    // Now apply changes with mutable borrow
    let entity = match entities.get_mut(hero_id) {
        Some(e) => e,
        None => return (None, true),
    };
    let hero_state = match entity.as_hero_mut() {
        Some(h) => h,
        None => return (None, true),
    };

    if !is_wall {
        hero_state.is_digging = false;
        hero_state.dig_timer = 0.0;
        return (None, true);
    }

    if !can_dig {
        hero_state.current_path = None;
        return (None, true);
    }

    // PREVENT DIGGING IF TILE IS NOT DIGGABLE (e.g. Dungeon Heart, Bedrock)
    if !is_diggable || is_hero_wall {
        hero_state.current_path = None;
        hero_state.is_digging = false;
        return (None, true);
    }

    hero_state.is_digging = true;
    hero_state.dig_timer += dt;

    if hero_state.dig_timer >= hero_state.max_dig_time {
        hero_state.dig_timer = 0.0;
        hero_state.is_digging = false;
        return (Some(next_pos), false);
    }

    (None, false)
}

/// Check if a tile blocks movement
fn tile_blocks_movement(dungeon: &Dungeon, pos: TilePos, game_data: &GameData) -> bool {
    let tile = match dungeon.get_tile(pos) {
        Some(t) => t,
        None => return false,
    };
    game_data
        .tiles
        .get(&tile.tile_type)
        .map(|td| td.blocks_movement)
        .unwrap_or(false)
}
