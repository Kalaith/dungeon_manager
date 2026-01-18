use crate::state::game_state::GameState;
use serde::{Serialize, Deserialize};

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

/// Save game to persistent storage (works on both native and WebGL/WASM)
/// On native: saves to local.data file
/// On WebGL: uses browser localStorage
pub fn save_game(game_state: &GameState, slot_name: &str) -> Result<(), String> {
    let wrapper = GameSaveWrapper {
        game_state,
        save_date: "Unknown Date".to_string(),
        version: "0.1.0".to_string(),
    };

    let serialized = serde_json::to_string(&wrapper)
        .map_err(|e| format!("Serialization error: {}", e))?;
    
    let key = format!("save_{}", slot_name);
    
    let mut storage = quad_storage::STORAGE.lock()
        .map_err(|e| format!("Storage lock error: {}", e))?;
    
    storage.set(&key, &serialized);
    
    eprintln!("Game saved to {} (quad-storage)", slot_name);
    Ok(())
}

/// Load game from persistent storage (works on both native and WebGL/WASM)
pub fn load_game(slot_name: &str) -> Result<GameState, String> {
    let key = format!("save_{}", slot_name);
    
    let storage = quad_storage::STORAGE.lock()
        .map_err(|e| format!("Storage lock error: {}", e))?;
    
    let content = storage.get(&key)
        .ok_or_else(|| format!("No save found for slot: {}", slot_name))?;
    
    let wrapper: GameLoadWrapper = serde_json::from_str(&content)
        .map_err(|e| format!("Deserialization error: {}", e))?;
    
    eprintln!("Game loaded from {} (quad-storage)", slot_name);
    Ok(wrapper.game_state)
}

/// Check if a save exists for the given slot
pub fn save_exists(slot_name: &str) -> bool {
    let key = format!("save_{}", slot_name);
    
    if let Ok(storage) = quad_storage::STORAGE.lock() {
        storage.get(&key).is_some()
    } else {
        false
    }
}

/// Get list of available save slots
pub fn get_save_files() -> Vec<String> {
    // quad-storage doesn't provide iteration, so we check known slots
    let known_slots = ["slot_1", "slot_2", "slot_3", "autosave"];
    let mut saves = Vec::new();
    
    for slot in known_slots {
        if save_exists(slot) {
            saves.push(slot.to_string());
        }
    }
    
    saves
}
