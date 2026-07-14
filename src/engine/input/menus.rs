//! Input handling for the main menu, settings, mission select, and skirmish
//! setup screens. Layout rects are shared with the renderer via
//! `crate::ui::menu_layout`.

use crate::data::GameData;
use crate::state::game_state::GameState;
use crate::state::{GamePhase, MapType};
use macroquad::prelude::*;

pub(super) fn handle_main_menu(
    selected_map_type: &mut MapType,
    phase: &mut GamePhase,
    game_data: &mut Option<GameData>,
) {
    let layout = crate::ui::menu_layout::main_menu();
    let mouse = mouse_position();
    let mouse = vec2(mouse.0, mouse.1);
    let clicked = |rect: macroquad::math::Rect| {
        is_mouse_button_pressed(MouseButton::Left) && rect.contains(mouse)
    };

    // Start Game (Space also starts)
    if is_key_pressed(KeyCode::Space) || clicked(layout.start) {
        // Force Standard Map type
        *selected_map_type = MapType::Standard;

        if let Some(ref data) = game_data {
            if let Some(campaign) = data.campaigns.get("deep_dominion") {
                // Open the mission-select screen so the player can pick which
                // unlocked mission to play (notably the M7 branch).
                let progress = crate::data::campaign::CampaignProgress::new(campaign);
                *phase = GamePhase::MissionSelect(progress);
            } else {
                let game_state = GameState::new_with_map_type(
                    data.config.map_size.width,
                    data.config.map_size.height,
                    data,
                    selected_map_type.clone(),
                );
                *phase = GamePhase::Playing(game_state);
            }
        }
        return;
    }

    // Load Game
    if clicked(layout.load) && crate::state::save_system::save_exists("slot_1") {
        match crate::state::save_system::load_game("slot_1") {
            Ok(loaded_state) => {
                println!("Game loaded successfully!");
                *phase = GamePhase::Playing(loaded_state);
            }
            Err(e) => {
                eprintln!("Failed to load game: {}", e);
            }
        }
        return;
    }

    // Skirmish setup (procedural sandbox)
    if clicked(layout.skirmish) {
        *phase = GamePhase::SkirmishSetup(crate::state::skirmish::SkirmishConfig::default());
        return;
    }

    // Settings
    if clicked(layout.settings) {
        *phase = GamePhase::Settings;
        return;
    }

    // Exit Game (native only)
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(exit) = layout.exit {
        if clicked(exit) {
            std::process::exit(0);
        }
    }
}

pub(super) fn handle_mission_select(
    phase: &mut GamePhase,
    game_data: &Option<GameData>,
    settings: &crate::state::settings::GameSettings,
) {
    let GamePhase::MissionSelect(progress) = phase else {
        return;
    };
    // Decouple from `phase` so we can reassign it below.
    let progress = progress.clone();
    let Some(data) = game_data.as_ref() else {
        return;
    };
    let Some(campaign) = data.campaigns.get(&progress.campaign_id) else {
        *phase = GamePhase::MainMenu;
        return;
    };

    let mouse = mouse_position();
    let mouse = vec2(mouse.0, mouse.1);
    let clicked = |rect: macroquad::math::Rect| {
        is_mouse_button_pressed(MouseButton::Left) && rect.contains(mouse)
    };

    if is_key_pressed(KeyCode::Escape) || clicked(crate::ui::menu_layout::mission_select_back()) {
        *phase = GamePhase::MainMenu;
        return;
    }

    let entries = progress.mission_menu(campaign);
    let rows = crate::ui::menu_layout::mission_select_rows(entries.len());
    for (entry, rect) in entries.iter().zip(rows.iter()) {
        if matches!(entry.status, crate::data::campaign::MissionStatus::Locked) {
            continue;
        }
        if clicked(*rect) {
            let mut chosen = progress.clone();
            chosen.select_mission(campaign, &entry.id);
            *phase = match GameState::new_for_campaign_progress(data, chosen) {
                Some(mut state) => {
                    state.difficulty = settings.difficulty;
                    GamePhase::Playing(state)
                }
                None => GamePhase::MainMenu,
            };
            return;
        }
    }
}

pub(super) fn handle_skirmish_setup(
    phase: &mut GamePhase,
    game_data: &Option<GameData>,
    settings: &crate::state::settings::GameSettings,
) {
    let GamePhase::SkirmishSetup(config) = phase else {
        return;
    };
    let layout = crate::ui::menu_layout::skirmish_setup();
    let mouse = mouse_position();
    let mouse = vec2(mouse.0, mouse.1);
    let clicked = |rect: macroquad::math::Rect| {
        is_mouse_button_pressed(MouseButton::Left) && rect.contains(mouse)
    };

    if is_key_pressed(KeyCode::Escape) || clicked(layout.back) {
        *phase = GamePhase::MainMenu;
        return;
    }
    if clicked(layout.map_type) {
        config.cycle_map_type();
        return;
    }
    if clicked(layout.size) {
        config.cycle_size();
        return;
    }
    if clicked(layout.start) || is_key_pressed(KeyCode::Enter) {
        if let Some(data) = game_data.as_ref() {
            let (w, h) = config.dimensions();
            let mut state = GameState::new_with_map_type(w, h, data, config.map_type());
            state.difficulty = settings.difficulty;
            *phase = GamePhase::Playing(state);
        }
    }
}

pub(super) fn handle_settings(
    phase: &mut GamePhase,
    settings: &mut crate::state::settings::GameSettings,
) {
    let layout = crate::ui::menu_layout::settings_menu();
    let mouse = mouse_position();
    let mouse = vec2(mouse.0, mouse.1);
    let clicked = |rect: macroquad::math::Rect| {
        is_mouse_button_pressed(MouseButton::Left) && rect.contains(mouse)
    };

    if clicked(layout.fullscreen) {
        settings.toggle_fullscreen();
    }

    if clicked(layout.ui_scale) {
        settings.cycle_ui_text_scale();
    }

    if clicked(layout.difficulty) {
        settings.cycle_difficulty();
    }

    if clicked(layout.back) || is_key_pressed(KeyCode::Escape) {
        *phase = GamePhase::MainMenu;
    }
}
