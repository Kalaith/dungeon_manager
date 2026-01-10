//! Main game state container
//! Holds the dungeon grid, entities, rooms, and player state

use crate::data::GameData;
use crate::engine::room_validator::Room;
use crate::engine::tile_grid::{self, Grid};
use crate::state::entities::EntityManager;
use crate::state::player_state::PlayerState;
use crate::state::tile_state::TilePos;

pub struct GameState {
    pub grid: Grid,
    pub width: usize,
    pub height: usize,
    pub time_elapsed: f32,
    pub tick_accumulator: f32,
    pub camera_x: f32,
    pub camera_y: f32,
    pub zoom: f32,
    pub rooms: Vec<Room>,
    pub entities: EntityManager,
    pub player: PlayerState,
    pub next_room_id: usize,
}

impl GameState {
    pub fn new(width: usize, height: usize, game_data: &GameData) -> Self {
        let grid = tile_grid::create_grid(width, height, game_data);

        Self {
            grid,
            width,
            height,
            time_elapsed: 0.0,
            tick_accumulator: 0.0,
            camera_x: 400.0,
            camera_y: 200.0,
            zoom: 1.0,
            rooms: Vec::new(),
            entities: EntityManager::new(),
            player: PlayerState::new(),
            next_room_id: 0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time_elapsed += dt;
        self.tick_accumulator += dt;

        // Fixed timestep simulation (10 ticks per second)
        const TICK_RATE: f32 = 0.1;
        while self.tick_accumulator >= TICK_RATE {
            self.tick();
            self.tick_accumulator -= TICK_RATE;
        }
    }

    fn tick(&mut self) {
        // Simulation tick - will be filled in later phases
    }

    pub fn get_tile(&self, pos: TilePos) -> Option<&crate::state::tile_state::TileState> {
        tile_grid::get_tile(&self.grid, pos)
    }

    pub fn get_tile_mut(&mut self, pos: TilePos) -> Option<&mut crate::state::tile_state::TileState> {
        tile_grid::get_tile_mut(&mut self.grid, pos)
    }
}
