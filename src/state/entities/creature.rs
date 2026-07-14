use super::{StatusEffect, Task};
use crate::state::tile_state::TilePos;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Runtime state for a creature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureState {
    /// Reference to creature data ID (e.g., "imp", "goblin")
    pub creature_id: String,

    /// Visual variation seed for unique appearance
    pub visual_seed: u64,

    /// Current level (1-based)
    pub level: u32,

    /// Current health (0.0 = dead)
    pub health: f32,

    /// Maximum health at this level
    pub max_health: f32,

    /// Current mana
    pub mana: f32,

    /// Maximum mana
    pub max_mana: f32,

    /// Current experience points
    pub experience: f32,

    /// Experience required for next level
    pub max_experience: f32,

    /// Timer for training XP ticks
    pub training_timer: f32,

    /// Needs tracking (0-100, higher = more satisfied)
    pub needs: HashMap<String, f32>,

    /// Overall mood (0-100, higher = happier)
    pub mood: f32,

    /// Current task being performed
    pub current_task: Option<Task>,

    /// Time spent on current task
    pub task_time: f32,

    /// Gold carried by this creature
    pub gold_carried: i32,

    /// Whether creature is angry (affects behavior)
    pub is_angry: bool,

    /// Whether creature is considering desertion
    pub is_deserting: bool,

    /// Last time creature was slapped (for cooldown)
    pub last_slapped: f32,

    /// Active status effects
    pub status_effects: Vec<StatusEffect>,

    /// Current path being followed
    pub current_path: Option<Vec<TilePos>>,

    /// Movement speed (tiles per second)
    pub movement_speed: f32,

    /// Time accumulator for movement
    pub move_timer: f32,

    /// Time accumulator for work production
    pub work_timer: f32,
}

impl CreatureState {
    /// Create a new creature state from data
    pub fn new(
        creature_id: String,
        level: u32,
        max_health: f32,
        max_mana: f32,
        visual_seed: u64,
    ) -> Self {
        Self {
            creature_id,
            visual_seed,
            level,
            health: max_health,
            max_health,
            mana: max_mana,
            max_mana,
            experience: 0.0,
            max_experience: 100.0 * (level as f32), // 100 XP per level base
            training_timer: 0.0,
            needs: {
                let mut m = HashMap::new();
                m.insert("sleep".to_string(), 100.0);
                m.insert("food".to_string(), 100.0);
                m.insert("gold".to_string(), 100.0);
                m
            },
            mood: 70.0, // Start at decent mood
            current_task: None,
            task_time: 0.0,
            gold_carried: 0,
            is_angry: false,
            is_deserting: false,
            last_slapped: 0.0,
            status_effects: Vec::new(),
            current_path: None,
            movement_speed: 2.0, // 2 tiles per second default
            move_timer: 0.0,
            work_timer: 0.0,
        }
    }

    /// Get the value of a specific need (0-100)
    pub fn get_need(&self, need_name: &str) -> f32 {
        self.needs.get(need_name).copied().unwrap_or(50.0)
    }

    /// Set a specific need value (clamped to 0-100)
    pub fn set_need(&mut self, need_name: String, value: f32) {
        let clamped = value.clamp(0.0, 100.0);
        self.needs.insert(need_name, clamped);
    }

    /// Get the most urgent need (lowest value)
    pub fn get_most_urgent_need(&self) -> Option<(String, f32)> {
        self.needs
            .iter()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(name, value)| (name.clone(), *value))
    }

    /// Damage creature by amount
    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
    }
}
