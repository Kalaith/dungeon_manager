//! Tutorial progression state
//!
//! Step logic lives in `engine::tutorial_system`; this holds only the
//! serializable progress so it survives save/load.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TutorialState {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub step_index: usize,
    #[serde(default)]
    pub complete: bool,
    #[serde(default)]
    pub intro_dismissed: bool,
}

impl TutorialState {
    pub fn for_new_scenario() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }
}
