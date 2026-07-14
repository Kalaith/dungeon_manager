use super::{HeroGoal, StatusEffect};
use crate::state::tile_state::TilePos;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Runtime state for a hero
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroState {
    /// Reference to hero data ID (e.g., "knight", "archer")
    pub hero_id: String,

    /// Visual variation seed for unique appearance
    pub visual_seed: u64,

    /// Current level
    pub level: u32,

    /// Current health
    pub health: f32,

    /// Maximum health
    pub max_health: f32,

    /// Current mana
    pub mana: f32,

    /// Maximum mana
    pub max_mana: f32,

    /// Current goal being pursued
    pub current_goal: HeroGoal,

    /// Target position for current goal
    pub target_pos: Option<TilePos>,

    /// Target room for current goal
    pub target_room_id: Option<usize>,

    /// Gold stolen so far
    pub gold_stolen: i32,

    /// Creatures killed so far
    pub kills: u32,

    /// Whether hero is fleeing
    pub is_fleeing: bool,

    /// Active status effects
    pub status_effects: Vec<StatusEffect>,

    /// Current path being followed
    pub current_path: Option<Vec<TilePos>>,

    /// Movement speed (tiles per second)
    pub movement_speed: f32,

    /// Time accumulator for movement
    pub move_timer: f32,

    /// Spawn position (for retreating/resting)
    pub spawn_pos: TilePos,

    // Digging state
    pub is_digging: bool,
    pub dig_timer: f32,
    pub max_dig_time: f32, // Time to dig one wall
    pub can_dig: bool,     // Tunneled capability

    // Prison/capture state
    /// Whether this hero is captured and being converted
    pub is_captured: bool,
    /// Conversion progress (0.0 to 1.0, at 1.0 hero becomes a creature)
    pub conversion_progress: f32,

    // Wave attack state
    /// Whether this hero stays at base as a defender (true) or joins attack waves (false)
    pub is_defender: bool,
    /// Which wave number this hero is assigned to (0 = not yet assigned)
    pub wave_assigned: u32,

    /// Whether this hero has been converted to the dungeon faction
    pub is_converted: bool,

    /// Remaining cooldown per ability id (see `data::heroes::HeroAbilityData`, `engine::hero_abilities`)
    #[serde(default)]
    pub ability_cooldowns: HashMap<String, f32>,
}

impl HeroState {
    /// Create a new hero state
    pub fn new(
        hero_id: String,
        level: u32,
        max_health: f32,
        max_mana: f32,
        spawn_pos: TilePos,
        dig_time: f32,
        visual_seed: u64,
    ) -> Self {
        let can_dig = true; // Allow all heroes to try and breach walls if path is blocked

        Self {
            hero_id,
            visual_seed,
            level,
            health: max_health,
            max_health,
            mana: max_mana,
            max_mana,
            current_goal: HeroGoal::RestAtSpawn(spawn_pos), // Start by resting/grouping up
            target_pos: Some(spawn_pos),                    // Start with target at spawn
            target_room_id: None,
            gold_stolen: 0,
            kills: 0,
            is_fleeing: false,
            status_effects: Vec::new(),
            current_path: None,
            movement_speed: 1.5, // 1.5 tiles per second default
            move_timer: 0.0,
            spawn_pos,
            is_digging: false,
            dig_timer: 0.0,
            max_dig_time: dig_time,
            can_dig,
            is_captured: false,
            conversion_progress: 0.0,
            is_defender: false, // Assigned later by spawner
            wave_assigned: 0,   // Assigned when wave launches
            is_converted: false,
            ability_cooldowns: HashMap::new(),
        }
    }

    /// Damage hero by amount
    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
    }

    /// Check if hero should retreat based on health
    pub fn should_retreat(&self, retreat_threshold: f32) -> bool {
        (self.health / self.max_health) < retreat_threshold
    }
}
