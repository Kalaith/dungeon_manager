//! UI Action system
//! Decouples UI events from game state mutations

/// Represents a player action from the UI
#[derive(Debug, Clone)]
pub enum UiAction {
    // UI actions
    StartResearch(String),
    SaveGame,
    LoadGame,
    TogglePause,

    // Cheats
    CheatAddGold(i32),
    CheatToggleFog,
    CheatInstantDig(crate::state::tile_state::TilePos),
    CheatToggleImmortalHeart,
}

/// Queue for collecting UI actions during a frame
#[derive(Default)]
pub struct ActionQueue {
    actions: Vec<UiAction>,
}

impl ActionQueue {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Add an action to the queue
    pub fn push(&mut self, action: UiAction) {
        self.actions.push(action);
    }

    /// Take all actions from the queue
    pub fn drain(&mut self) -> Vec<UiAction> {
        std::mem::take(&mut self.actions)
    }
}
