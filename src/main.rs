// Allow some warnings that are acceptable during development
#![allow(dead_code)]  // Some code is kept for future features
#![allow(unused_variables)] // Some variables are kept for future features
#![allow(unused_imports)] // Some imports are kept for future features

use macroquad::prelude::*;

mod data;
mod engine;
mod state;
mod ui;
mod config;
mod draw_utils;
mod sprite_variation;

#[cfg(test)]
mod combat_tests;

use data::GameData;
use state::MapType;
use state::{GamePhase, InteractionMode, DragSelection};
use ui::renderer::GameRenderer;
use ui::actions::ActionQueue;
use engine::input::InputHandler;
use engine::action_processor;





pub struct Game {
    phase: GamePhase,
    game_data: Option<GameData>,
    renderer: GameRenderer,
    interaction_mode: InteractionMode,
    hovered_tile: Option<state::tile_state::TilePos>,
    selected_map_type: MapType,
    selected_entity: Option<state::entities::EntityId>,
    selected_room: Option<usize>,
    held_entity: Option<state::entities::EntityId>,
    spell_shop_open: bool,
    action_queue: ActionQueue,
    selected_spell: Option<String>,
    drag_selection: DragSelection,
}

impl Game {
    fn new() -> Self {
        Self {
            phase: GamePhase::Loading,
            game_data: None,
            renderer: GameRenderer::new(),
            interaction_mode: InteractionMode::None,
            hovered_tile: None,
            selected_map_type: MapType::Standard, // Default to Standard Map
            selected_entity: None,
            selected_room: None,
            held_entity: None,
            spell_shop_open: false,
            action_queue: ActionQueue::new(),
            selected_spell: None,
            drag_selection: DragSelection::new(),
        }
    }

    async fn load_resources(&mut self) {
        // Load game data
        if self.game_data.is_none() {
            match GameData::load() {
                Ok(data) => {
                    eprintln!("Successfully loaded game data!");
                    self.game_data = Some(data);
                }
                Err(e) => {
                    eprintln!("Failed to load game data: {}", e);
                    return;
                }
            }
        }

        // Load graphics
        if self.renderer.graphics_cache.is_none() {
            eprintln!("Loading graphics...");
            self.renderer.load_resources(self.game_data.as_ref()).await;
            if self.renderer.graphics_cache.is_some() {
                 self.phase = GamePhase::MainMenu;
            }
        }
    }

    fn update(&mut self, dt: f32) {
        InputHandler::update(
            dt,
            &mut self.phase,
            &mut self.game_data, // Changed to mut to allow applying cheats
            &mut self.interaction_mode,
            &mut self.selected_map_type,
            &mut self.hovered_tile,
            &mut self.held_entity,
            &mut self.selected_entity,
            &mut self.selected_room,
            &mut self.renderer.sidebar,
            &mut self.action_queue,
            &mut self.drag_selection,
        );

        // Process queued actions
        if let GamePhase::Playing(ref mut state) = self.phase {
            if let Some(ref game_data) = self.game_data {
                // Update helpers (traps, etc)
                crate::engine::trap_system::process_trap_construction(
                    &mut state.dungeon,
                    &mut state.player,
                    &mut state.pending_trap_builds,
                    game_data,
                    dt
                );

                action_processor::process_actions(
                    &mut self.action_queue,
                    state,
                    game_data,
                    &mut self.interaction_mode,
                    &mut self.held_entity,
                    &mut self.selected_entity,
                    &mut self.selected_room,
                    &mut self.selected_spell,
                );
            }
        }
    }

    fn draw(&mut self) {
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0)); // Ensure background is cleared every frame

        match self.phase {
            GamePhase::Playing(ref mut state) => {
                if let Some(ref data) = self.game_data {
                    self.renderer.draw_game(
                        state,
                        &self.interaction_mode,
                        self.hovered_tile,
                        self.held_entity,
                        data,
                        &self.drag_selection
                    );
                }
                self.renderer.draw_gui(
                    state, 
                    &mut self.interaction_mode, 
                    self.hovered_tile, 
                    self.held_entity, 
                    self.selected_entity, 
                    self.selected_room, 
                    &self.game_data, // Helper takes Option
                    &self.drag_selection
                );
            },
            _ => {
                self.renderer.draw(
                    &self.phase,
                    None,
                    &mut self.interaction_mode,
                    &self.selected_map_type,
                    self.hovered_tile,
                    self.held_entity,
                    self.selected_entity,
                    self.selected_room,
                    &self.game_data,
                    &self.drag_selection,
                );
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Deep Dominion".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    let mut loading_started = false;

    loop {
        // Handle loading phase
        if matches!(game.phase, GamePhase::Loading) && !loading_started {
            loading_started = true;
            game.load_resources().await;
        }

        let dt = get_frame_time().min(0.1);
        game.update(dt);
        game.draw();

        next_frame().await;
    }
}
