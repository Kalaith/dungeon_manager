//! Persistent game settings (fullscreen, UI scale)
//!
//! Stored as `settings.json` next to the save slots on native builds and in
//! localStorage on WASM, mirroring `save_system`.

use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
const GAME_NAME: &str = "dungeon_manager";
const SETTINGS_KEY: &str = "settings";

/// UI text scale steps cycled by the settings menu.
pub const UI_SCALE_STEPS: &[f32] = &[0.8, 1.0, 1.25];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    #[serde(default)]
    pub fullscreen: bool,
    #[serde(default = "default_ui_text_scale")]
    pub ui_text_scale: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            fullscreen: false,
            ui_text_scale: default_ui_text_scale(),
        }
    }
}

fn default_ui_text_scale() -> f32 {
    1.0
}

impl GameSettings {
    pub fn load() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            macroquad_toolkit::persistence::load_json_key(GAME_NAME, SETTINGS_KEY)
                .unwrap_or_default()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            macroquad_toolkit::persistence::load_json(format!("{}.json", SETTINGS_KEY))
                .unwrap_or_default()
        }
    }

    pub fn save(&self) {
        let result = {
            #[cfg(target_arch = "wasm32")]
            {
                macroquad_toolkit::persistence::save_json_key(GAME_NAME, SETTINGS_KEY, self)
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                macroquad_toolkit::persistence::save_json_atomic(
                    format!("{}.json", SETTINGS_KEY),
                    self,
                )
            }
        };
        if let Err(e) = result {
            eprintln!("Failed to save settings: {}", e);
        }
    }

    /// Push the current settings into the window and UI systems.
    pub fn apply(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        macroquad::window::set_fullscreen(self.fullscreen);
        macroquad_toolkit::ui::set_ui_text_scale(self.ui_text_scale);
    }

    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
        self.apply();
        self.save();
    }

    pub fn cycle_ui_text_scale(&mut self) {
        let current = UI_SCALE_STEPS
            .iter()
            .position(|scale| (scale - self.ui_text_scale).abs() < 0.01)
            .unwrap_or(1);
        self.ui_text_scale = UI_SCALE_STEPS[(current + 1) % UI_SCALE_STEPS.len()];
        self.apply();
        self.save();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_roundtrip_serialization() {
        let settings = GameSettings {
            fullscreen: true,
            ui_text_scale: 1.25,
        };
        let json = serde_json::to_string(&settings).expect("settings should serialize");
        let loaded: GameSettings = serde_json::from_str(&json).expect("settings should parse");
        assert!(loaded.fullscreen);
        assert_eq!(loaded.ui_text_scale, 1.25);

        // Missing fields fall back to defaults for forward compatibility
        let empty: GameSettings = serde_json::from_str("{}").expect("empty settings should parse");
        assert!(!empty.fullscreen);
        assert_eq!(empty.ui_text_scale, 1.0);
    }

    #[test]
    fn ui_scale_cycles_through_steps() {
        let mut settings = GameSettings::default();
        // Note: cycle applies + saves; in tests this writes settings.json to cwd,
        // so exercise only the pure step selection here.
        let current = UI_SCALE_STEPS
            .iter()
            .position(|scale| (scale - settings.ui_text_scale).abs() < 0.01)
            .unwrap_or(1);
        settings.ui_text_scale = UI_SCALE_STEPS[(current + 1) % UI_SCALE_STEPS.len()];
        assert_eq!(settings.ui_text_scale, 1.25);
    }
}
