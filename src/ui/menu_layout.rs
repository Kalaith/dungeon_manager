//! Shared menu button geometry
//!
//! Both the renderer (`ui::menus`) and the input handler (`engine::input`)
//! consume these rects, so drawing and hit-testing can never drift apart.

use macroquad::prelude::*;

pub const BUTTON_WIDTH: f32 = 240.0;
pub const BUTTON_HEIGHT: f32 = 52.0;
pub const BUTTON_SPACING: f32 = 18.0;

fn stacked(start_y: f32, index: usize) -> Rect {
    Rect::new(
        screen_width() / 2.0 - BUTTON_WIDTH / 2.0,
        start_y + index as f32 * (BUTTON_HEIGHT + BUTTON_SPACING),
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
    )
}

/// Whether the running platform can exit the process (hidden on WASM).
pub fn can_exit() -> bool {
    cfg!(not(target_arch = "wasm32"))
}

pub struct MainMenuLayout {
    pub start: Rect,
    pub load: Rect,
    pub settings: Rect,
    pub exit: Option<Rect>,
}

pub fn main_menu() -> MainMenuLayout {
    let start_y = screen_height() / 2.0 - 80.0;
    MainMenuLayout {
        start: stacked(start_y, 0),
        load: stacked(start_y, 1),
        settings: stacked(start_y, 2),
        exit: can_exit().then(|| stacked(start_y, 3)),
    }
}

pub struct PauseMenuLayout {
    pub resume: Rect,
    pub save: Rect,
    pub load: Rect,
    pub main_menu: Rect,
    pub exit: Option<Rect>,
}

pub fn pause_menu() -> PauseMenuLayout {
    let start_y = screen_height() / 2.0 - 100.0;
    PauseMenuLayout {
        resume: stacked(start_y, 0),
        save: stacked(start_y, 1),
        load: stacked(start_y, 2),
        main_menu: stacked(start_y, 3),
        exit: can_exit().then(|| stacked(start_y, 4)),
    }
}

/// "Next Mission" button on the victory screen.
pub fn game_over_next_mission() -> Rect {
    Rect::new(
        screen_width() / 2.0 - BUTTON_WIDTH / 2.0,
        screen_height() / 2.0 + 100.0,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
    )
}

pub struct SettingsMenuLayout {
    pub fullscreen: Rect,
    pub ui_scale: Rect,
    pub back: Rect,
}

pub fn settings_menu() -> SettingsMenuLayout {
    let start_y = screen_height() / 2.0 - 60.0;
    SettingsMenuLayout {
        fullscreen: stacked(start_y, 0),
        ui_scale: stacked(start_y, 1),
        back: stacked(start_y, 2),
    }
}
