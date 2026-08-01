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
    pub skirmish: Rect,
    pub load: Rect,
    pub settings: Rect,
    pub exit: Option<Rect>,
}

pub fn main_menu() -> MainMenuLayout {
    let start_y = screen_height() / 2.0 - 110.0;
    MainMenuLayout {
        start: stacked(start_y, 0),
        skirmish: stacked(start_y, 1),
        load: stacked(start_y, 2),
        settings: stacked(start_y, 3),
        exit: can_exit().then(|| stacked(start_y, 4)),
    }
}

/// Skirmish setup screen: two option rows (map type, size) that cycle on
/// click, plus Start and Back. Consumed by both the renderer and input layer.
pub struct SkirmishSetupLayout {
    pub map_type: Rect,
    pub size: Rect,
    pub start: Rect,
    pub back: Rect,
}

pub fn skirmish_setup() -> SkirmishSetupLayout {
    let start_y = screen_height() / 2.0 - 90.0;
    SkirmishSetupLayout {
        map_type: stacked(start_y, 0),
        size: stacked(start_y, 1),
        start: stacked(start_y, 3),
        back: stacked(start_y, 4),
    }
}

/// Mission-select geometry: a vertical list of full-width mission rows plus a
/// Back button. `mission_select_rows(n)` returns one rect per mission in
/// authored order; both the renderer and the input handler consume it so a
/// click always maps to the row the player sees.
pub const MISSION_ROW_WIDTH: f32 = 560.0;
pub const MISSION_ROW_SPACING: f32 = 5.0;
/// Top of the mission list (below the title block).
const MISSION_LIST_TOP: f32 = 132.0;
/// Bottom of the list area (above the briefing line + Back button).
const MISSION_LIST_BOTTOM_MARGIN: f32 = 140.0;

/// One row per mission, sized to fit `count` rows in the available vertical
/// band so a 13-mission campaign never overflows the screen.
pub fn mission_select_rows(count: usize) -> Vec<Rect> {
    let band = (screen_height() - MISSION_LIST_TOP - MISSION_LIST_BOTTOM_MARGIN).max(120.0);
    let slot = (band / count.max(1) as f32).min(40.0);
    let row_h = (slot - MISSION_ROW_SPACING).max(20.0);
    (0..count)
        .map(|i| {
            Rect::new(
                screen_width() / 2.0 - MISSION_ROW_WIDTH / 2.0,
                MISSION_LIST_TOP + i as f32 * slot,
                MISSION_ROW_WIDTH,
                row_h,
            )
        })
        .collect()
}

pub fn mission_select_back() -> Rect {
    Rect::new(
        screen_width() / 2.0 - BUTTON_WIDTH / 2.0,
        screen_height() - 80.0,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
    )
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

/// Save-slot browser: one wide row per slot plus a Back button. Sized like the
/// mission list because it is the same shape of choice — pick one of a short
/// list of things that each need a line of description.
pub const SLOT_ROW_WIDTH: f32 = 560.0;
pub const SLOT_ROW_HEIGHT: f32 = 74.0;
pub const SLOT_ROW_SPACING: f32 = 14.0;

pub fn slot_rows(count: usize) -> Vec<Rect> {
    let total =
        count as f32 * SLOT_ROW_HEIGHT + (count.saturating_sub(1)) as f32 * SLOT_ROW_SPACING;
    let top = (screen_height() / 2.0 - total / 2.0).max(120.0);
    (0..count)
        .map(|i| {
            Rect::new(
                screen_width() / 2.0 - SLOT_ROW_WIDTH / 2.0,
                top + i as f32 * (SLOT_ROW_HEIGHT + SLOT_ROW_SPACING),
                SLOT_ROW_WIDTH,
                SLOT_ROW_HEIGHT,
            )
        })
        .collect()
}

pub fn slot_browser_back() -> Rect {
    Rect::new(
        screen_width() / 2.0 - BUTTON_WIDTH / 2.0,
        screen_height() - 80.0,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
    )
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
    pub difficulty: Rect,
    pub back: Rect,
}

pub fn settings_menu() -> SettingsMenuLayout {
    let start_y = screen_height() / 2.0 - 90.0;
    SettingsMenuLayout {
        fullscreen: stacked(start_y, 0),
        ui_scale: stacked(start_y, 1),
        difficulty: stacked(start_y, 2),
        back: stacked(start_y, 3),
    }
}
