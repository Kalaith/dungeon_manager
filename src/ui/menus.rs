//! Menu rendering module
//!
//! Handles drawing main menu, pause menu, and game over screens.

use crate::data::GameData;
use crate::state::game_state::GameState;
use crate::state::settings::GameSettings;
use crate::state::MapType;
use crate::ui::core::colors;
use crate::ui::menu_layout;
use crate::ui::resources::GraphicsCache;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{button_rect_tone, draw_ui_text, measure_ui_text, ButtonTone};

/// Draw a menu button for visuals only; clicks are handled by the input
/// layer against the same `menu_layout` rects.
fn draw_menu_button(rect: Rect, text: &str, enabled: bool, tone: ButtonTone) {
    let _ = button_rect_tone(rect, text, enabled, tone);
}

fn draw_title_block(title: &str, subtitle: &str, baseline_y: f32) {
    let title_dims = measure_ui_text(title, None, 64, 1.0);
    draw_ui_text(
        title,
        screen_width() / 2.0 - title_dims.width / 2.0,
        baseline_y,
        64.0,
        colors::ACCENT_GOLD,
    );
    if !subtitle.is_empty() {
        let sub_dims = measure_ui_text(subtitle, None, 20, 1.0);
        draw_ui_text(
            subtitle,
            screen_width() / 2.0 - sub_dims.width / 2.0,
            baseline_y + 32.0,
            20.0,
            colors::TEXT_DIM,
        );
    }
}

/// Draw the main menu screen
pub fn draw_main_menu(graphics_cache: Option<&GraphicsCache>, _selected_map_type: &MapType) {
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
        Color::new(0.0, 0.0, 0.0, 0.55),
    );

    draw_title_block(
        "Deep Dominion",
        "Dig deep. Rule the dark.",
        screen_height() / 2.0 - 160.0,
    );

    let layout = menu_layout::main_menu();
    let save_exists = crate::state::save_system::save_exists("slot_1");

    draw_menu_button(layout.start, "START GAME", true, ButtonTone::Primary);
    draw_menu_button(layout.load, "LOAD GAME", save_exists, ButtonTone::Positive);
    draw_menu_button(layout.settings, "SETTINGS", true, ButtonTone::Secondary);
    if let Some(exit) = layout.exit {
        draw_menu_button(exit, "EXIT GAME", true, ButtonTone::Danger);
    }

    // Version string
    let version = concat!("v", env!("CARGO_PKG_VERSION"));
    draw_ui_text(
        version,
        12.0,
        screen_height() - 14.0,
        16.0,
        colors::TEXT_DIM,
    );
}

/// Draw the settings menu screen
pub fn draw_settings_menu(settings: &GameSettings) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.55),
    );

    draw_title_block("Settings", "", screen_height() / 2.0 - 140.0);

    let layout = menu_layout::settings_menu();
    let fullscreen_label = if settings.fullscreen {
        "FULLSCREEN: ON"
    } else {
        "FULLSCREEN: OFF"
    };
    draw_menu_button(
        layout.fullscreen,
        fullscreen_label,
        true,
        ButtonTone::Secondary,
    );

    let scale_label = format!("UI SCALE: {:.0}%", settings.ui_text_scale * 100.0);
    draw_menu_button(layout.ui_scale, &scale_label, true, ButtonTone::Secondary);

    draw_menu_button(layout.back, "BACK", true, ButtonTone::Muted);

    let hint = "Press ESC to return";
    let hint_dims = measure_ui_text(hint, None, 16, 1.0);
    draw_ui_text(
        hint,
        screen_width() / 2.0 - hint_dims.width / 2.0,
        layout.back.y + layout.back.h + 40.0,
        16.0,
        colors::TEXT_DIM,
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

    let layout = menu_layout::pause_menu();

    draw_title_block("PAUSED", "", layout.resume.y - 60.0);

    let save_exists = crate::state::save_system::save_exists("slot_1");
    draw_menu_button(layout.resume, "RESUME", true, ButtonTone::Positive);
    draw_menu_button(layout.save, "SAVE GAME", true, ButtonTone::Primary);
    draw_menu_button(layout.load, "LOAD GAME", save_exists, ButtonTone::Secondary);
    draw_menu_button(layout.main_menu, "MAIN MENU", true, ButtonTone::Warning);
    if let Some(exit) = layout.exit {
        draw_menu_button(exit, "EXIT", true, ButtonTone::Danger);
    }
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

        draw_menu_button(
            menu_layout::game_over_next_mission(),
            "NEXT MISSION",
            true,
            ButtonTone::Positive,
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
