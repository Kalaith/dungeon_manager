//! Menu rendering module
//!
//! Handles drawing main menu, pause menu, and game over screens.

use crate::data::GameData;
use crate::state::game_state::GameState;
use crate::state::MapType;
use crate::ui::resources::GraphicsCache;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

/// Draw the main menu screen
pub fn draw_main_menu(graphics_cache: Option<&GraphicsCache>, _selected_map_type: &MapType) {
    let mouse_pos = mouse_position();

    // Draw background if available
    if let Some(cache) = graphics_cache {
        if let Some(bg_tex) = cache.ui_textures.get("main_menu_bg") {
            draw_texture_ex(
                bg_tex,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
        }
    }

    // Dark overlay for readability (less intense than solid color)
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.4),
    );

    // Draw title
    draw_ui_text(
        "Deep Dominion",
        screen_width() / 2.0 - 150.0,
        screen_height() / 2.0 - 180.0,
        48.0,
        WHITE,
    );

    // Buttons
    let button_width = 200.0;
    let button_height = 50.0;
    let spacing = 20.0;
    let start_y = screen_height() / 2.0 - 50.0;
    let center_x = screen_width() / 2.0 - button_width / 2.0;

    // 1. Start Game
    let start_y_pos = start_y;
    let is_start_hovered = mouse_pos.0 >= center_x
        && mouse_pos.0 <= center_x + button_width
        && mouse_pos.1 >= start_y_pos
        && mouse_pos.1 <= start_y_pos + button_height;

    let start_color = if is_start_hovered {
        Color::new(0.4, 0.6, 0.9, 1.0)
    } else {
        Color::new(0.3, 0.5, 0.8, 1.0)
    };
    draw_rectangle(
        center_x,
        start_y_pos,
        button_width,
        button_height,
        start_color,
    );
    draw_rectangle_lines(
        center_x,
        start_y_pos,
        button_width,
        button_height,
        3.0,
        WHITE,
    );
    draw_ui_text(
        "START GAME",
        center_x + 35.0,
        start_y_pos + 32.0,
        24.0,
        WHITE,
    );

    // 2. Load Game
    let load_y_pos = start_y_pos + button_height + spacing;
    let save_exists = crate::state::save_system::save_exists("slot_1");
    let is_load_hovered = mouse_pos.0 >= center_x
        && mouse_pos.0 <= center_x + button_width
        && mouse_pos.1 >= load_y_pos
        && mouse_pos.1 <= load_y_pos + button_height;

    let load_color = if save_exists {
        if is_load_hovered {
            Color::new(0.4, 0.8, 0.4, 1.0)
        } else {
            Color::new(0.3, 0.7, 0.3, 1.0)
        }
    } else {
        Color::new(0.3, 0.3, 0.3, 0.5)
    };
    draw_rectangle(
        center_x,
        load_y_pos,
        button_width,
        button_height,
        load_color,
    );
    draw_rectangle_lines(
        center_x,
        load_y_pos,
        button_width,
        button_height,
        3.0,
        if save_exists { WHITE } else { GRAY },
    );
    draw_ui_text(
        "LOAD GAME",
        center_x + 35.0,
        load_y_pos + 32.0,
        24.0,
        if save_exists { WHITE } else { GRAY },
    );
}

/// Draw the pause menu overlay
pub fn draw_pause_menu() {
    // Semi-transparent overlay
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.7),
    );

    let screen_center_x = screen_width() / 2.0;
    let screen_center_y = screen_height() / 2.0;
    let button_width = 200.0;
    let button_height = 50.0;
    let spacing = 20.0;
    let start_y = screen_center_y - 100.0; // Adjusted to fit more buttons

    // Title
    let title = "PAUSED";
    let title_dims = measure_ui_text(title, None, 60, 1.0);
    draw_ui_text(
        title,
        screen_center_x - title_dims.width / 2.0,
        start_y - 80.0,
        60.0,
        WHITE,
    );

    // Resume Button
    let resume_y = start_y;
    draw_rectangle(
        screen_center_x - button_width / 2.0,
        resume_y,
        button_width,
        button_height,
        Color::new(0.2, 0.6, 0.2, 1.0),
    );
    let resume_text = "RESUME";
    let resume_dims = measure_ui_text(resume_text, None, 30, 1.0);
    draw_ui_text(
        resume_text,
        screen_center_x - resume_dims.width / 2.0,
        resume_y + 35.0,
        30.0,
        WHITE,
    );

    // Save Game Button
    let save_y = start_y + button_height + spacing;
    draw_rectangle(
        screen_center_x - button_width / 2.0,
        save_y,
        button_width,
        button_height,
        Color::new(0.3, 0.5, 0.7, 1.0),
    );
    let save_text = "SAVE GAME";
    let save_dims = measure_ui_text(save_text, None, 30, 1.0);
    draw_ui_text(
        save_text,
        screen_center_x - save_dims.width / 2.0,
        save_y + 35.0,
        30.0,
        WHITE,
    );

    // Load Game Button
    let load_y = start_y + (button_height + spacing) * 2.0;
    let save_exists = crate::state::save_system::save_exists("slot_1");
    let load_color = if save_exists {
        Color::new(0.3, 0.7, 0.4, 1.0)
    } else {
        Color::new(0.3, 0.3, 0.3, 1.0) // Grayed out if no save
    };
    draw_rectangle(
        screen_center_x - button_width / 2.0,
        load_y,
        button_width,
        button_height,
        load_color,
    );
    let load_text = "LOAD GAME";
    let load_dims = measure_ui_text(load_text, None, 30, 1.0);
    draw_ui_text(
        load_text,
        screen_center_x - load_dims.width / 2.0,
        load_y + 35.0,
        30.0,
        if save_exists { WHITE } else { GRAY },
    );

    // Main Menu Button
    let menu_y = start_y + (button_height + spacing) * 3.0;
    draw_rectangle(
        screen_center_x - button_width / 2.0,
        menu_y,
        button_width,
        button_height,
        Color::new(0.6, 0.4, 0.2, 1.0),
    );
    let menu_text = "MAIN MENU";
    let menu_dims = measure_ui_text(menu_text, None, 30, 1.0);
    draw_ui_text(
        menu_text,
        screen_center_x - menu_dims.width / 2.0,
        menu_y + 35.0,
        30.0,
        WHITE,
    );

    // Exit Button
    let exit_y = start_y + (button_height + spacing) * 4.0;
    draw_rectangle(
        screen_center_x - button_width / 2.0,
        exit_y,
        button_width,
        button_height,
        Color::new(0.8, 0.2, 0.2, 1.0),
    );
    let exit_text = "EXIT";
    let exit_dims = measure_ui_text(exit_text, None, 30, 1.0);
    draw_ui_text(
        exit_text,
        screen_center_x - exit_dims.width / 2.0,
        exit_y + 35.0,
        30.0,
        WHITE,
    );
}

/// Draw the game over screen (victory or defeat)
pub fn draw_game_over_screen(state: &GameState, game_data: Option<&GameData>) {
    // Semi-transparent overlay
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.8),
    );

    let screen_center_x = screen_width() / 2.0;
    let screen_center_y = screen_height() / 2.0;

    let title = if state.victory { "VICTORY" } else { "DEFEAT" };
    let color = if state.victory {
        crate::ui::core::colors::POSITIVE
    } else {
        crate::ui::core::colors::NEGATIVE
    };

    // Title
    let title_dims = measure_ui_text(title, None, 80, 1.0);
    draw_ui_text(
        title,
        screen_center_x - title_dims.width / 2.0,
        screen_center_y - 100.0,
        80.0,
        color,
    );

    // Subtitle
    let has_next_mission = state.victory
        && game_data
            .map(|data| state.has_pending_campaign_mission(data))
            .unwrap_or(false);
    let subtitle = if state.victory {
        if has_next_mission {
            "Mission complete."
        } else {
            "The campaign mission has been completed."
        }
    } else {
        "Your Dungeon Heart has fallen!"
    };
    let sub_dims = measure_ui_text(subtitle, None, 40, 1.0);
    draw_ui_text(
        subtitle,
        screen_center_x - sub_dims.width / 2.0,
        screen_center_y - 20.0,
        40.0,
        WHITE,
    );

    if has_next_mission {
        if let Some(data) = game_data {
            if let Some(mission) = state.active_campaign_mission(data) {
                let mission_text = format!("Next: {}", mission.name);
                let mission_dims = measure_ui_text(&mission_text, None, 28, 1.0);
                draw_ui_text(
                    &mission_text,
                    screen_center_x - mission_dims.width / 2.0,
                    screen_center_y + 30.0,
                    28.0,
                    WHITE,
                );

                if let Some(briefing) = state.active_campaign_briefing(data) {
                    let briefing = fit_text_to_width(briefing, 700.0, 20.0);
                    let briefing_dims = measure_ui_text(&briefing, None, 20, 1.0);
                    draw_ui_text(
                        &briefing,
                        screen_center_x - briefing_dims.width / 2.0,
                        screen_center_y + 64.0,
                        20.0,
                        LIGHTGRAY,
                    );
                }
            }
        }

        let button_width = 240.0;
        let button_height = 50.0;
        let button_x = screen_center_x - button_width / 2.0;
        let button_y = screen_center_y + 100.0;
        draw_rectangle(
            button_x,
            button_y,
            button_width,
            button_height,
            Color::new(0.25, 0.55, 0.35, 1.0),
        );
        draw_rectangle_lines(button_x, button_y, button_width, button_height, 2.0, WHITE);
        let next_text = "NEXT MISSION";
        let next_dims = measure_ui_text(next_text, None, 24, 1.0);
        draw_ui_text(
            next_text,
            screen_center_x - next_dims.width / 2.0,
            button_y + 32.0,
            24.0,
            WHITE,
        );
    }

    // Instructions
    let instr = if has_next_mission {
        "Press Enter for next mission or ESC for menu"
    } else {
        "Press ESC to Exit"
    };
    let instr_dims = measure_ui_text(instr, None, 30, 1.0);
    draw_ui_text(
        instr,
        screen_center_x - instr_dims.width / 2.0,
        if has_next_mission {
            screen_center_y + 180.0
        } else {
            screen_center_y + 60.0
        },
        30.0,
        GRAY,
    );
}

fn fit_text_to_width(text: &str, max_width: f32, font_size: f32) -> String {
    if measure_ui_text(text, None, font_size as u16, 1.0).width <= max_width {
        return text.to_string();
    }

    let mut fitted = String::new();
    for ch in text.chars() {
        let candidate = format!("{}{}...", fitted, ch);
        if measure_ui_text(&candidate, None, font_size as u16, 1.0).width > max_width {
            break;
        }
        fitted.push(ch);
    }

    format!("{}...", fitted.trim_end())
}
