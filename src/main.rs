#![allow(dead_code, clippy::large_enum_variant, clippy::too_many_arguments)]

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod config;
mod data;
mod draw_utils;
mod engine;
mod sprite_variation;
mod state;
mod ui;

#[cfg(test)]
mod combat_tests;
#[cfg(test)]
mod hero_abilities_tests;
#[cfg(test)]
mod traits_tests;

use data::GameData;
use engine::action_processor;
use engine::input::InputHandler;
use state::game_state::GameState;
use state::MapType;
use state::{DragSelection, GamePhase, InteractionMode};
use ui::actions::ActionQueue;
use ui::renderer::GameRenderer;

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
    action_queue: ActionQueue,
    selected_spell: Option<String>,
    drag_selection: DragSelection,
    settings: state::settings::GameSettings,
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
            action_queue: ActionQueue::new(),
            selected_spell: None,
            drag_selection: DragSelection::new(),
            settings: state::settings::GameSettings::load(),
        }
    }

    async fn load_resources(&mut self) {
        // Load game data
        if self.game_data.is_none() {
            match GameData::load_with_default_mod_order() {
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
            &mut self.settings,
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
                    dt,
                );

                action_processor::process_actions(
                    &mut self.action_queue,
                    state,
                    &mut self.interaction_mode,
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
                        data,
                        &self.drag_selection,
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
                    &self.drag_selection,
                );
            }
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
                    &self.settings,
                );
            }
        }
    }

    /// Seed a specific scene for the screenshot harness. Call after
    /// `load_resources` so `game_data` is populated.
    fn begin_capture_scene(&mut self, scene: &str) {
        match scene {
            "mainmenu" => {
                self.phase = GamePhase::MainMenu;
            }
            "missionselect" => {
                self.phase = match self.game_data.as_ref() {
                    Some(data) if data.campaigns.contains_key("deep_dominion") => {
                        let progress = crate::data::campaign::CampaignProgress::new(
                            data.campaigns.get("deep_dominion").unwrap(),
                        );
                        GamePhase::MissionSelect(progress)
                    }
                    _ => GamePhase::MainMenu,
                };
            }
            _ => {
                // Default ("gameplay"): jump straight into a playable dungeon
                // so the capture photographs the main game view.
                if let Some(ref data) = self.game_data {
                    let game_state = if data.campaigns.contains_key("deep_dominion") {
                        GameState::new_campaign_start(data, "deep_dominion")
                    } else {
                        GameState::new_with_map_type(
                            data.config.map_size.width,
                            data.config.map_size.height,
                            data,
                            self.selected_map_type.clone(),
                        )
                    };
                    self.phase = GamePhase::Playing(game_state);
                } else {
                    self.phase = GamePhase::MainMenu;
                }
            }
        }
    }
}

fn window_conf() -> Conf {
    capture::capture_window_conf("DUNGEON_MANAGER", "Deep Dominion", 1280, 720)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    game.settings.apply();

    // Screenshot harness: when DUNGEON_MANAGER_CAPTURE_PATH is set, load
    // resources synchronously, seed a scene, simulate deterministic frames,
    // write a PNG, and exit.
    if let Some(config) = capture::CaptureConfig::from_env("DUNGEON_MANAGER") {
        game.load_resources().await;
        game.begin_capture_scene(&config.scene);
        capture::run_capture(&config, |dt| {
            game.update(dt);
            game.draw();
        })
        .await;
        return;
    }

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
