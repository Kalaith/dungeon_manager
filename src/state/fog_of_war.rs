use crate::data::GameData;
use crate::engine::tile_grid;
use crate::state::game_state::GameState;
use crate::state::tile_state::{Ownership, TilePos};
use std::collections::HashSet;

impl GameState {
    pub(in crate::state) fn update_fog_of_war_system(&mut self, game_data: &GameData) {
        let mut claimed_tiles = HashSet::new();
        let (width, height) = tile_grid::get_grid_dimensions(&self.dungeon.grid);

        for y in 0..height {
            for x in 0..width {
                let pos = TilePos::new(x as i32, y as i32);
                if let Some(tile) = tile_grid::get_tile(&self.dungeon.grid, pos) {
                    if tile.ownership == Ownership::Player {
                        claimed_tiles.insert(pos);
                    }
                }
            }
        }

        let creature_positions: Vec<TilePos> = self
            .entities
            .creatures()
            .filter(|(_, creature)| {
                if creature.creature_id == "imp" {
                    return false;
                }
                game_data
                    .monsters
                    .get(&creature.creature_id)
                    .map(|monster_data| monster_data.faction == "dungeon")
                    .unwrap_or(false)
            })
            .filter_map(|(id, _)| self.entities.get(id))
            .map(|entity| entity.pos)
            .collect();

        self.dungeon
            .update_fog_of_war(&claimed_tiles, &creature_positions, game_data);
    }
}
