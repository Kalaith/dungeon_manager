//! Monster trait definitions — data-driven behavioral/stat modifiers.
//!
//! Traits are tag strings on `MonsterData.traits` (e.g. "cowardly", "greedy"). Each tag is
//! looked up here for a set of generic modifiers that the engine applies uniformly (see
//! `engine::creature_ai::needs`, `engine::creature_task_logic`, `engine::combat`). Adding a new
//! trait — or changing what an existing one does — is a `traits.json` edit (or a content pack's
//! own trait file, see `data::content_pack`); the engine never branches on a trait's name.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TraitData {
    pub id: String,
    #[serde(default)]
    pub description: String,
    /// Added directly to the creature's calculated mood (see `needs::calculate_mood`).
    #[serde(default)]
    pub mood_modifier: f32,
    /// Added to `ai.anger_threshold` before the `mood < threshold` comparison — a positive value
    /// means the creature angers at a higher mood, i.e. sooner.
    #[serde(default)]
    pub anger_threshold_modifier: f32,
    /// Same as `anger_threshold_modifier` but for `ai.desertion_threshold`.
    #[serde(default)]
    pub desertion_threshold_modifier: f32,
    /// Multiplies a matching need's `decay_per_minute` (need name -> multiplier).
    #[serde(default)]
    pub need_decay_multipliers: HashMap<String, f32>,
    /// Multiplies a matching task type's desirability weight (task type -> multiplier).
    #[serde(default)]
    pub task_preference_multipliers: HashMap<String, f32>,
    /// Multiplies combat attack stat.
    #[serde(default = "one")]
    pub attack_multiplier: f32,
    /// Multiplies combat defense stat.
    #[serde(default = "one")]
    pub defense_multiplier: f32,
    /// Multiplies the magnitude of discipline responses (slap/torture/reward mood swings).
    #[serde(default = "one")]
    pub discipline_response_multiplier: f32,
}

fn one() -> f32 {
    1.0
}

pub fn load_traits() -> Result<HashMap<String, TraitData>, Box<dyn Error>> {
    let json_content = {
        #[cfg(target_arch = "wasm32")]
        {
            include_str!("../../assets/data/traits.json").to_string()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::fs::read_to_string("assets/data/traits.json")
                .unwrap_or_else(|_| include_str!("../../assets/data/traits.json").to_string())
        }
    };

    let traits_vec: Vec<TraitData> = serde_json::from_str(&json_content)?;

    let mut traits_map = HashMap::new();
    for trait_data in traits_vec {
        traits_map.insert(trait_data.id.clone(), trait_data);
    }

    Ok(traits_map)
}
