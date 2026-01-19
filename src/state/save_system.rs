use crate::state::game_state::GameState;
use serde::{Serialize, Deserialize};

#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[cfg(target_arch = "wasm32")]
use quad_storage::STORAGE;

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
/// On WebGL: uses quad-storage (localStorage)
pub fn save_game(game_state: &GameState, slot_name: &str) -> Result<(), String> {
    let wrapper = GameSaveWrapper {
        game_state,
        save_date: "Unknown Date".to_string(), 
        version: "0.1.0".to_string(),
    };

    let serialized = serde_json::to_string(&wrapper)
        .map_err(|e| format!("Serialization error: {}", e))?;
    
    let key = format!("save_{}", slot_name);
    
    #[cfg(target_arch = "wasm32")]
    {
        if let Ok(mut storage) = STORAGE.lock() {
            storage.set(&key, &serialized);
            eprintln!("Game saved to quad-storage key: {}", key);
            Ok(())
        } else {
            Err("Failed to lock quad-storage".to_string())
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let filename = format!("{}.json", key);
        fs::write(&filename, serialized)
            .map_err(|e| format!("File write error: {}", e))?;
        eprintln!("Game saved to file: {}", filename);
        Ok(())
    }
}

/// Load game from persistent storage
pub fn load_game(slot_name: &str) -> Result<GameState, String> {
    let key = format!("save_{}", slot_name);
    
    let content = {
        #[cfg(target_arch = "wasm32")]
        {
            if let Ok(storage) = STORAGE.lock() {
                storage.get(&key)
                    .ok_or_else(|| format!("No save found for slot: {}", slot_name))?
            } else {
                return Err("Failed to lock quad-storage".to_string());
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let filename = format!("{}.json", key);
            fs::read_to_string(&filename)
                 .map_err(|e| format!("File read error: {}", e))?
        }
    };
    
    let wrapper: GameLoadWrapper = serde_json::from_str(&content)
        .map_err(|e| format!("Deserialization error: {}", e))?;
    
    eprintln!("Game loaded from {}", slot_name);
    Ok(wrapper.game_state)
}

/// Check if a save exists for the given slot
pub fn save_exists(slot_name: &str) -> bool {
    let key = format!("save_{}", slot_name);
    
    #[cfg(target_arch = "wasm32")]
    {
        if let Ok(storage) = STORAGE.lock() {
            storage.get(&key).is_some()
        } else {
            false
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let filename = format!("{}.json", key);
        Path::new(&filename).exists()
    }
}

/// Get list of available save slots
pub fn get_save_files() -> Vec<String> {
    let known_slots = ["slot_1", "slot_2", "slot_3", "autosave"];
    let mut saves = Vec::new();
    
    for slot in known_slots {
        if save_exists(slot) {
            saves.push(slot.to_string());
        }
    }
    
    saves
}

