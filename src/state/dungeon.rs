use crate::data::GameData;
use crate::engine::map_generator;
use crate::engine::tile_grid::{self, Grid};
use crate::engine::tile_types::types as tt;
use crate::state::game_state::MapType;
use crate::state::tile_state::{TilePos, TileState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dungeon {
    pub grid: Grid,
    pub width: usize,
    pub height: usize,
}

impl Dungeon {
    pub fn new(width: usize, height: usize, game_data: &GameData, map_type: MapType) -> Self {
        // Randomize starting position each game
        let starting_positions = [
            map_generator::StartingPosition::Center,
            map_generator::StartingPosition::Corner,
            map_generator::StartingPosition::Edge,
        ];
        let random_idx = macroquad::rand::gen_range(0, starting_positions.len());
        let random_start = starting_positions[random_idx];

        let grid = match map_type {
            MapType::Standard => {
                let config = map_generator::MapConfig {
                    width,
                    height,
                    starting_position: random_start,
                    ..Default::default()
                };
                map_generator::generate_map(&config, game_data)
            }
            MapType::Rich => map_generator::generate_rich_map(width, height, game_data),
            MapType::Hazardous => map_generator::generate_hazardous_map(width, height, game_data),
            MapType::Test => map_generator::generate_test_map(width, height, game_data),
            MapType::File(_) => {
                // Fallback if called directly (GameState handles File separately via loader)
                let config = map_generator::MapConfig {
                    width,
                    height,
                    starting_position: random_start,
                    ..Default::default()
                };
                map_generator::generate_map(&config, game_data)
            }
        };

        Self {
            grid,
            width,
            height,
        }
    }

    pub fn get_tile(&self, pos: TilePos) -> Option<&TileState> {
        tile_grid::get_tile(&self.grid, pos)
    }

    pub fn get_tile_mut(&mut self, pos: TilePos) -> Option<&mut TileState> {
        tile_grid::get_tile_mut(&mut self.grid, pos)
    }

    /// Update fog of war using the tile_grid system
    pub fn update_fog_of_war(
        &mut self,
        claimed_tiles: &std::collections::HashSet<TilePos>,
        creature_positions: &[TilePos],
        game_data: &GameData,
    ) {
        tile_grid::update_fog_of_war(
            &mut self.grid,
            claimed_tiles,
            creature_positions,
            8, // Sight radius
            game_data,
        );
    }
    pub fn find_dungeon_heart(&self) -> Option<TilePos> {
        for row in &self.grid {
            for tile in row {
                if tile.tile_type == tt::DUNGEON_HEART {
                    return Some(tile.pos);
                }
            }
        }
        None
    }
}
