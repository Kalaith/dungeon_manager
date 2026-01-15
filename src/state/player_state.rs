//! Player state - resources, research, and unlocks
//! Tracks the player's economy, technology tree, and game progress

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Player's current resources and state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    // Economic resources
    pub gold: i32,
    pub mana: i32,
    pub food: i32,
    pub max_gold: i32,
    pub max_mana: i32,
    pub max_food: i32,
    pub materials: i32,
    pub max_materials: i32,

    // Research and unlocks
    pub unlocked_rooms: HashSet<String>,
    pub unlocked_creatures: HashSet<String>,
    pub unlocked_spells: HashSet<String>,
    pub research_points: i32,
    pub active_research: Option<String>,
    pub research_progress: f32,

    // Game state
    pub dungeon_heart_health: f32,
    pub max_creatures: usize,
    pub current_creature_count: usize,
    pub claimed_tile_count: usize,

    // Spell cooldowns (spell_id -> remaining time in seconds)
    pub spell_cooldowns: HashMap<String, f32>,

    // Statistics
    pub kills: HashMap<String, u32>,
    pub deaths: HashMap<String, u32>,
    pub gold_mined: u32,
    pub spells_cast: HashMap<String, u32>,
    pub game_time: f32,
}

impl PlayerState {
    /// Create a new player state with starting values, auto-unlocking all defined rooms
    pub fn new(game_data: &crate::data::GameData) -> Self {
        let mut unlocked_rooms = HashSet::new();
        // Auto-unlock all rooms defined in data
        for room_id in game_data.rooms.keys() {
            unlocked_rooms.insert(room_id.clone());
        }

        let mut unlocked_creatures = HashSet::new();
        unlocked_creatures.insert("imp".to_string());

        let mut unlocked_spells = HashSet::new();
        unlocked_spells.insert("heal".to_string());
        unlocked_spells.insert("summon_imps".to_string());
        unlocked_spells.insert("lightning_strike".to_string());
        unlocked_spells.insert("make_earth".to_string());

        Self {
            gold: crate::config::STARTING_GOLD,
            mana: crate::config::STARTING_MANA,
            food: crate::config::STARTING_FOOD,
            max_gold: crate::config::INITIAL_MAX_GOLD,
            max_mana: crate::config::INITIAL_MAX_MANA,

            max_food: crate::config::INITIAL_MAX_FOOD,
            materials: crate::config::STARTING_MATERIALS,
            max_materials: crate::config::INITIAL_MAX_MATERIALS,

            unlocked_rooms,
            unlocked_creatures,
            unlocked_spells,
            research_points: 0,
            active_research: None,
            research_progress: 0.0,

            dungeon_heart_health: 100.0,
            max_creatures: 20,
            current_creature_count: 0,
            claimed_tile_count: 0,

            spell_cooldowns: HashMap::new(),

            kills: HashMap::new(),
            deaths: HashMap::new(),
            gold_mined: 0,
            spells_cast: HashMap::new(),
            game_time: 0.0,
        }
    }

    /// Check if player can afford the given cost
    pub fn can_afford(&self, gold: i32, mana: i32) -> bool {
        self.gold >= gold && self.mana >= mana
    }

    /// Spend resources, returns true if successful
    pub fn spend(&mut self, gold: i32, mana: i32) -> bool {
        if self.can_afford(gold, mana) {
            self.gold -= gold;
            self.mana -= mana;
            true
        } else {
            false
        }
    }

    /// Add resources, respecting max limits
    pub fn add_resources(&mut self, gold: i32, mana: i32, food: i32, materials: i32) {
        self.gold = (self.gold + gold).min(self.max_gold);
        self.mana = (self.mana + mana).min(self.max_mana);
        self.food = (self.food + food).min(self.max_food);
        self.materials = (self.materials + materials).min(self.max_materials);
    }

    /// Check if a room type is unlocked
    pub fn is_room_unlocked(&self, room_id: &str) -> bool {
        self.unlocked_rooms.contains(room_id)
    }

    /// Check if a creature type is unlocked
    pub fn is_creature_unlocked(&self, creature_id: &str) -> bool {
        self.unlocked_creatures.contains(creature_id)
    }

    /// Check if a spell is unlocked
    pub fn is_spell_unlocked(&self, spell_id: &str) -> bool {
        self.unlocked_spells.contains(spell_id)
    }

    /// Unlock a room type
    pub fn unlock_room(&mut self, room_id: String) {
        self.unlocked_rooms.insert(room_id);
    }

    /// Unlock a creature type
    pub fn unlock_creature(&mut self, creature_id: String) {
        self.unlocked_creatures.insert(creature_id);
    }

    /// Unlock a spell
    pub fn unlock_spell(&mut self, spell_id: String) {
        self.unlocked_spells.insert(spell_id);
    }

    /// Start researching a technology
    pub fn start_research(&mut self, research_id: String) {
        self.active_research = Some(research_id);
        self.research_progress = 0.0;
    }

    /// Update research progress, returns Some(research_id) if research completes
    pub fn update_research(&mut self, research_rate: f32, dt: f32) -> Option<String> {
        if let Some(ref research_id) = self.active_research {
            self.research_progress += research_rate * dt;

            // Check if research complete (assuming 100.0 is completion threshold)
            if self.research_progress >= 100.0 {
                let completed = research_id.clone();
                self.active_research = None;
                self.research_progress = 0.0;
                return Some(completed);
            }
        }
        None
    }

    /// Record a kill
    pub fn record_kill(&mut self, enemy_type: String) {
        *self.kills.entry(enemy_type).or_insert(0) += 1;
    }

    /// Record a death
    pub fn record_death(&mut self, creature_type: String) {
        *self.deaths.entry(creature_type).or_insert(0) += 1;
    }

    /// Record a spell cast
    pub fn record_spell_cast(&mut self, spell_id: String) {
        *self.spells_cast.entry(spell_id).or_insert(0) += 1;
    }

    /// Check if player has lost (dungeon heart destroyed)
    pub fn is_defeated(&self) -> bool {
        self.dungeon_heart_health <= 0.0
    }

    /// Check if player is at creature limit
    pub fn is_at_creature_limit(&self) -> bool {
        self.current_creature_count >= self.max_creatures
    }

    /// Update game time
    pub fn update_time(&mut self, dt: f32) {
        self.game_time += dt;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spend_resources() {
        // manually construct player state to avoid needing GameData for simple unit tests
        let mut player = PlayerState {
            gold: 200, mana: 1000, food: 100, max_gold: 10000, max_mana: 5000, max_food: 1000,
            materials: 100, max_materials: 500,
            unlocked_rooms: HashSet::new(), unlocked_creatures: HashSet::new(), unlocked_spells: HashSet::new(),
            research_points: 0, active_research: None, research_progress: 0.0,
            dungeon_heart_health: 100.0, max_creatures: 20, current_creature_count: 0,
            claimed_tile_count: 0, spell_cooldowns: HashMap::new(),
            kills: HashMap::new(), deaths: HashMap::new(), gold_mined: 0, spells_cast: HashMap::new(), game_time: 0.0,
        };
        
        // Assert initial state matches what test expects
        assert!(player.can_afford(100, 50));
        assert!(player.spend(100, 50));
        assert_eq!(player.gold, 100);
        assert_eq!(player.mana, 950);
    }

    #[test]
    fn test_cannot_overspend() {
        let mut player = PlayerState {
            gold: 200, mana: 1000, food: 100, max_gold: 10000, max_mana: 5000, max_food: 1000,
            materials: 100, max_materials: 500,
            unlocked_rooms: HashSet::new(), unlocked_creatures: HashSet::new(), unlocked_spells: HashSet::new(),
            research_points: 0, active_research: None, research_progress: 0.0,
            dungeon_heart_health: 100.0, max_creatures: 20, current_creature_count: 0,
            claimed_tile_count: 0, spell_cooldowns: HashMap::new(),
            kills: HashMap::new(), deaths: HashMap::new(), gold_mined: 0, spells_cast: HashMap::new(), game_time: 0.0,
        };
        
        assert!(!player.can_afford(10000, 0));
        assert!(!player.spend(10000, 0));
        assert_eq!(player.gold, 200); // Unchanged
    }

    #[test]
    fn test_resource_caps() {
        let mut player = PlayerState {
            gold: 200, mana: 1000, food: 100, max_gold: 10000, max_mana: 5000, max_food: 1000,
            materials: 100, max_materials: 500,
            unlocked_rooms: HashSet::new(), unlocked_creatures: HashSet::new(), unlocked_spells: HashSet::new(),
            research_points: 0, active_research: None, research_progress: 0.0,
            dungeon_heart_health: 100.0, max_creatures: 20, current_creature_count: 0,
            claimed_tile_count: 0, spell_cooldowns: HashMap::new(),
            kills: HashMap::new(), deaths: HashMap::new(), gold_mined: 0, spells_cast: HashMap::new(), game_time: 0.0,
        };
        
        player.add_resources(0, 20000, 1000, 1000);
        assert_eq!(player.mana, player.max_mana);
        assert_eq!(player.food, player.max_food);
        assert_eq!(player.materials, player.max_materials);
    }
}
