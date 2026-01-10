use macroquad::prelude::*;

mod data;
mod engine;
mod state;
mod ui;

use data::GameData;
use state::game_state::GameState;

pub enum GamePhase {
    Loading,
    MainMenu,
    Playing(GameState),
}

pub struct Game {
    phase: GamePhase,
    game_data: Option<GameData>,
}

impl Game {
    fn new() -> Self {
        Self {
            phase: GamePhase::Loading,
            game_data: None,
        }
    }

    fn update(&mut self, dt: f32) {
        match &mut self.phase {
            GamePhase::Loading => {
                if self.game_data.is_none() {
                    match GameData::load() {
                        Ok(data) => {
                            eprintln!("Successfully loaded game data!");
                            eprintln!("  - {} tiles", data.tiles.len());
                            eprintln!("  - {} rooms", data.rooms.len());
                            eprintln!("  - {} monsters", data.monsters.len());
                            eprintln!("  - {} heroes", data.heroes.len());
                            eprintln!("  - {} spells", data.spells.len());
                            self.game_data = Some(data);
                            self.phase = GamePhase::MainMenu;
                        }
                        Err(e) => {
                            eprintln!("Failed to load game data: {}", e);
                        }
                    }
                }
            }
            GamePhase::MainMenu => {
                if is_key_pressed(KeyCode::Space) {
                    if let Some(ref game_data) = self.game_data {
                        let game_state = GameState::new(50, 50, game_data);
                        self.phase = GamePhase::Playing(game_state);
                    }
                }
            }
            GamePhase::Playing(state) => {
                state.update(dt);

                // Camera controls (WASD)
                let camera_speed = 300.0 * dt;
                if is_key_down(KeyCode::W) {
                    state.camera_y += camera_speed;
                }
                if is_key_down(KeyCode::S) {
                    state.camera_y -= camera_speed;
                }
                if is_key_down(KeyCode::A) {
                    state.camera_x += camera_speed;
                }
                if is_key_down(KeyCode::D) {
                    state.camera_x -= camera_speed;
                }
            }
        }
    }

    fn draw(&self) {
        clear_background(ui::core::colors::BACKGROUND);

        match &self.phase {
            GamePhase::Loading => {
                draw_text(
                    "Loading Deep Dominion...",
                    screen_width() / 2.0 - 150.0,
                    screen_height() / 2.0,
                    32.0,
                    WHITE,
                );
            }
            GamePhase::MainMenu => {
                draw_text(
                    "Deep Dominion",
                    screen_width() / 2.0 - 120.0,
                    screen_height() / 2.0 - 50.0,
                    48.0,
                    WHITE,
                );
                draw_text(
                    "Press SPACE to start",
                    screen_width() / 2.0 - 120.0,
                    screen_height() / 2.0 + 50.0,
                    24.0,
                    GRAY,
                );
            }
            GamePhase::Playing(state) => {
                self.draw_game(state);
            }
        }
    }

    fn draw_game(&self, state: &GameState) {
        use engine::tile_grid;
        use state::tile_state::FogState;

        // Draw grid
        for row in &state.grid {
            for tile in row {
                let (iso_x, iso_y) = tile_grid::world_to_iso(
                    tile.pos.x as f32,
                    tile.pos.y as f32,
                    ui::core::TILE_WIDTH,
                    ui::core::TILE_HEIGHT,
                );

                let screen_x = iso_x + state.camera_x;
                let screen_y = iso_y + state.camera_y;

                // Skip tiles outside screen
                if screen_x < -100.0 || screen_x > screen_width() + 100.0
                    || screen_y < -100.0 || screen_y > screen_height() + 100.0
                {
                    continue;
                }

                let mut color = ui::core::get_tile_color(&tile.tile_type);

                // Apply fog of war
                match tile.fog_state {
                    FogState::Hidden => color = ui::core::colors::FOG_HIDDEN,
                    FogState::Revealed => {
                        color.r *= 0.5;
                        color.g *= 0.5;
                        color.b *= 0.5;
                    }
                    FogState::Visible => {}
                }

                ui::core::draw_iso_tile(
                    screen_x,
                    screen_y,
                    ui::core::TILE_WIDTH,
                    ui::core::TILE_HEIGHT,
                    color,
                );
            }
        }

        // Draw HUD
        draw_rectangle(0.0, 0.0, screen_width(), ui::core::HUD_HEIGHT, ui::core::colors::PANEL);
        draw_text(
            &format!("Deep Dominion - Grid: {}x{}", state.width, state.height),
            10.0,
            30.0,
            20.0,
            ui::core::colors::TEXT,
        );
        draw_text(
            "WASD: Move Camera | ESC: Menu",
            10.0,
            50.0,
            16.0,
            ui::core::colors::TEXT_DIM,
        );
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

    loop {
        let dt = get_frame_time();
        game.update(dt);
        game.draw();

        next_frame().await;
    }
}
