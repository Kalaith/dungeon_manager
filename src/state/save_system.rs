use crate::state::game_state::GameState;
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use crate::state::wasm_storage;

const GAME_NAME: &str = "dungeon_manager";

/// Wrapper for saving game state with metadata (uses reference to avoid Clone)
#[derive(Serialize)]
struct GameSaveWrapper<'a> {
    game_state: &'a GameState,
    save_date: String,
    version: String,
}

/// Wrapper for loading game state (owns the data)
#[derive(Deserialize)]
struct GameLoadWrapper {
    game_state: GameState,
    #[allow(dead_code)]
    save_date: String,
    #[allow(dead_code)]
    version: String,
}

/// Save game to persistent storage
/// On native: saves to .json file
/// On WebGL: uses localStorage via JS interop
pub fn save_game(game_state: &GameState, slot_name: &str) -> Result<(), String> {
    let wrapper = GameSaveWrapper {
        game_state,
        save_date: "Unknown Date".to_string(),
        version: "0.1.0".to_string(),
    };

    let key = format!("save_{}", slot_name);

    #[cfg(target_arch = "wasm32")]
    {
        macroquad_toolkit::persistence::save_json_key(GAME_NAME, &key, &wrapper)?;
        eprintln!("Game saved to localStorage key: {}", key);
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let filename = format!("{}.json", key);
        macroquad_toolkit::persistence::save_json_atomic(&filename, &wrapper)
            .map_err(|e| format!("File write error: {}", e))?;
        eprintln!("Game saved to file: {}", filename);
        Ok(())
    }
}

/// Load game from persistent storage
pub fn load_game(slot_name: &str) -> Result<GameState, String> {
    let key = format!("save_{}", slot_name);

    #[cfg(target_arch = "wasm32")]
    {
        let wrapper =
            macroquad_toolkit::persistence::load_json_key::<GameLoadWrapper>(GAME_NAME, &key)
                .or_else(|_| load_legacy_browser_save(&key))
                .map_err(|e| format!("No save found for slot {slot_name}: {e}"))?;
        eprintln!("Game loaded from {}", slot_name);
        Ok(wrapper.game_state)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let filename = format!("{}.json", key);
        let wrapper: GameLoadWrapper = macroquad_toolkit::persistence::load_json(&filename)
            .map_err(|e| format!("File read error: {}", e))?;
        eprintln!("Game loaded from {}", slot_name);
        Ok(wrapper.game_state)
    }
}

/// Check if a save exists for the given slot
pub fn save_exists(slot_name: &str) -> bool {
    let key = format!("save_{}", slot_name);

    #[cfg(target_arch = "wasm32")]
    {
        macroquad_toolkit::persistence::json_key_exists(GAME_NAME, &key)
            || wasm_storage::storage_exists(&key)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let filename = format!("{}.json", key);
        macroquad_toolkit::persistence::file_exists(filename)
    }
}

#[cfg(target_arch = "wasm32")]
fn load_legacy_browser_save(key: &str) -> Result<GameLoadWrapper, String> {
    let serialized = wasm_storage::storage_get(key)
        .ok_or_else(|| format!("No legacy browser save found for key '{key}'."))?;
    let wrapper = serde_json::from_str(&serialized)
        .map_err(|e| format!("Deserialization error for legacy save {key}: {e}"))?;
    let _ = macroquad_toolkit::persistence::save_string_key(GAME_NAME, key, &serialized);
    Ok(wrapper)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::GameData;

    #[test]
    fn campaign_progress_survives_save_serialization() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_campaign_start(&game_data, "deep_dominion");
        let progress = state
            .campaign_progress
            .as_mut()
            .expect("campaign progress should exist");
        progress
            .completed_missions
            .insert("dark_beginnings".to_string());

        let wrapper = GameSaveWrapper {
            game_state: &state,
            save_date: "test".to_string(),
            version: "0.1.0".to_string(),
        };
        let json = serde_json::to_string(&wrapper).expect("save should serialize");
        let loaded: GameLoadWrapper = serde_json::from_str(&json).expect("save should deserialize");

        let loaded_progress = loaded
            .game_state
            .campaign_progress
            .expect("campaign progress should load");
        assert_eq!(loaded_progress.campaign_id, "deep_dominion");
        assert!(loaded_progress
            .completed_missions
            .contains("dark_beginnings"));
    }
}
