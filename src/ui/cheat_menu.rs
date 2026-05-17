use crate::state::entities::EntityCategory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySpawnSelection {
    pub category: EntityCategory,
    pub entity_id: String,
    pub level: u32,
}

impl Default for EntitySpawnSelection {
    fn default() -> Self {
        Self {
            category: EntityCategory::Monster,
            entity_id: "goblin".to_string(), // Default fallback
            level: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheatTab {
    Spawn,
    // Add Resources, Tech, etc. later
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheatMenuState {
    pub is_open: bool,
    pub selected_tab: CheatTab,
    pub spawn_selection: EntitySpawnSelection,
}

impl Default for CheatMenuState {
    fn default() -> Self {
        Self {
            is_open: false,
            selected_tab: CheatTab::Spawn,
            spawn_selection: EntitySpawnSelection::default(),
        }
    }
}
