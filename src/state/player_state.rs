//! Player state - resources, research, and unlocks
//! Tracks the player's economy, technology tree, and game progress

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Accumulator for fractional resource gains
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceAccumulator {
    pub gold: f32,
    pub mana: f32,
    pub food: f32,
    pub materials: f32,
}

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

    // Fractional resource accumulator
    #[serde(default)]
    pub accumulators: ResourceAccumulator,

    // Research and unlocks
    pub unlocked_rooms: HashSet<String>,
    pub unlocked_creatures: HashSet<String>,
    pub unlocked_spells: HashSet<String>,
    #[serde(default)]
    pub unlocked_traps: HashSet<String>,
    #[serde(default)]
    pub trap_inventory: HashMap<String, u32>,
    #[serde(default)]
    pub trap_manufacturing_progress: HashMap<String, f32>,
    pub completed_technologies: HashSet<String>,
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

    #[serde(default)]
    pub graveyard_corpses: u32,
    #[serde(default)]
    pub scavenger_conversion_progress: HashMap<usize, f32>,
}

impl PlayerState {
    /// Create a new player state with starting values, auto-unlocking all defined rooms
    pub fn new(game_data: &crate::data::GameData) -> Self {
        let mut unlocked_rooms = HashSet::new();
        // Base rooms available at start
        unlocked_rooms.insert("lair".to_string());
        unlocked_rooms.insert("hatchery".to_string());
        unlocked_rooms.insert("treasury".to_string());
        unlocked_rooms.insert("library".to_string());
        unlocked_rooms.insert("dungeon_heart".to_string()); // Always needed

        let mut unlocked_creatures = HashSet::new();
        unlocked_creatures.insert("imp".to_string());

        let mut unlocked_spells = HashSet::new();
        unlocked_spells.insert("summon_imps".to_string());
        unlocked_spells.insert("lightning_strike".to_string());
        unlocked_spells.insert("heal".to_string());
        unlocked_spells.insert("speed_boost".to_string());
        // unlocked_spells.insert("create_food".to_string()); // Removed as it wasn't in json

        let mut unlocked_traps = HashSet::new();
        unlocked_traps.insert("door".to_string());

        Self {
            gold: game_data.config.player_starting_resources.gold,
            mana: game_data.config.player_starting_resources.mana,
            food: game_data.config.player_starting_resources.food,
            max_gold: game_data.config.player_initial_capacity.max_gold,
            max_mana: game_data.config.player_initial_capacity.max_mana,

            max_food: game_data.config.player_initial_capacity.max_food,
            materials: game_data.config.player_starting_resources.materials,
            max_materials: game_data.config.player_initial_capacity.max_materials,

            accumulators: ResourceAccumulator::default(),

            unlocked_rooms,
            unlocked_creatures,
            unlocked_spells,
            unlocked_traps,
            trap_inventory: HashMap::new(),
            trap_manufacturing_progress: HashMap::new(),
            completed_technologies: HashSet::new(),
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
            graveyard_corpses: 0,
            scavenger_conversion_progress: HashMap::new(),
        }
    }

    /// Add resources, respecting max limits
    pub fn add_resources(&mut self, gold: i32, mana: i32, food: i32, materials: i32) {
        self.gold = (self.gold + gold).min(self.max_gold);
        self.mana = (self.mana + mana).min(self.max_mana);
        self.food = (self.food + food).min(self.max_food);
        self.materials = (self.materials + materials).min(self.max_materials);
    }

    /// Check whether the player has enough gold and mana for a purchase.
    pub fn can_afford(&self, gold: i32, mana: i32) -> bool {
        self.gold >= gold && self.mana >= mana
    }

    /// Spend gold and mana if available.
    pub fn spend(&mut self, gold: i32, mana: i32) -> bool {
        if !self.can_afford(gold, mana) {
            return false;
        }

        self.gold -= gold;
        self.mana -= mana;
        true
    }

    /// Add resources with precision, accumulating fractional amounts
    pub fn add_resources_precise(&mut self, gold: f32, mana: f32, food: f32, materials: f32) {
        // Accumulate fractional partials
        self.accumulators.gold += gold;
        self.accumulators.mana += mana;
        self.accumulators.food += food;
        self.accumulators.materials += materials;

        // Extract integer parts
        let gold_int = self.accumulators.gold.trunc() as i32;
        let mana_int = self.accumulators.mana.trunc() as i32;
        let food_int = self.accumulators.food.trunc() as i32;
        let materials_int = self.accumulators.materials.trunc() as i32;

        // Reduce accumulators by the extracted integer parts
        self.accumulators.gold -= gold_int as f32;
        self.accumulators.mana -= mana_int as f32;
        self.accumulators.food -= food_int as f32;
        self.accumulators.materials -= materials_int as f32;

        // Add to main resources (can be negative consumption)
        self.add_resources(gold_int, mana_int, food_int, materials_int);
    }

    /// Check if a room type is unlocked
    pub fn is_room_unlocked(&self, room_id: &str) -> bool {
        self.unlocked_rooms.contains(room_id)
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

    /// Unlock a trap or door
    pub fn unlock_trap(&mut self, trap_id: String) {
        self.unlocked_traps.insert(trap_id);
    }

    pub fn is_trap_unlocked(&self, trap_id: &str) -> bool {
        self.unlocked_traps.contains(trap_id)
    }

    pub fn trap_inventory_count(&self, trap_id: &str) -> u32 {
        self.trap_inventory.get(trap_id).copied().unwrap_or(0)
    }

    pub fn add_trap_inventory(&mut self, trap_id: String, amount: u32) {
        *self.trap_inventory.entry(trap_id).or_insert(0) += amount;
    }

    pub fn consume_trap_inventory(&mut self, trap_id: &str, amount: u32) -> bool {
        let available = self.trap_inventory_count(trap_id);
        if available < amount {
            return false;
        }

        if available == amount {
            self.trap_inventory.remove(trap_id);
        } else if let Some(count) = self.trap_inventory.get_mut(trap_id) {
            *count -= amount;
        }
        true
    }

    // Check if a technology is unlocked (completed)
    // For now we don't store "unlocked_techs" explicitly, but we could infer it if needed.
    // However, it's better to store completed techs to check prerequisites.
    // We'll add a `completed_research` set to PlayerState in a separate step or just assume unlocks handle it.
    // Actually, let's just use the unlocks (rooms/spells) as proof of tech completion?
    // No, tech tree UI needs to know if "Training Tech" is done.
    // I need to add `completed_technologies` field to PlayerState struct first.
    // But since I can't easily change struct fields without a full replace, I will use `unlocked_rooms` to infer for now?
    // No, I should add the field. It's cleaner. But `PlayerState` is serialized.
    // Let's modify the struct definition first.

    /// Start researching a technology
    pub fn start_research(&mut self, research_id: String) {
        self.active_research = Some(research_id);
        self.research_progress = 0.0;
    }

    /// Update research progress
    pub fn update_research(
        &mut self,
        research_rate: f32,
        tech_cost: f32,
        dt: f32,
    ) -> Option<String> {
        if let Some(ref research_id) = self.active_research {
            self.research_progress += research_rate * dt;

            if self.research_progress >= tech_cost {
                let completed = research_id.clone();
                self.active_research = None;
                self.research_progress = 0.0;
                return Some(completed);
            }
        }
        None
    }

    /// Check if a technology is completed
    pub fn is_tech_completed(&self, tech_id: &str) -> bool {
        self.completed_technologies.contains(tech_id)
    }

    /// Complete a research and unlock rewards
    pub fn complete_research(&mut self, tech: &crate::data::TechData) {
        self.completed_technologies.insert(tech.id.clone());

        // Unlock rooms
        for room in &tech.unlocks.rooms {
            self.unlock_room(room.clone());
        }

        // Unlock spells
        for spell in &tech.unlocks.spells {
            self.unlock_spell(spell.clone());
        }

        // Unlock creatures
        for creature in &tech.unlocks.creatures {
            self.unlock_creature(creature.clone());
        }

        for trap in &tech.unlocks.traps {
            self.unlock_trap(trap.clone());
        }
    }

    /// Record a spell cast
    pub fn record_spell_cast(&mut self, spell_id: String) {
        *self.spells_cast.entry(spell_id).or_insert(0) += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spend_resources() {
        // manually construct player state to avoid needing GameData for simple unit tests
        let mut player = PlayerState {
            gold: 200,
            mana: 1000,
            food: 100,
            max_gold: 10000,
            max_mana: 5000,
            max_food: 1000,
            materials: 100,
            max_materials: 500,
            accumulators: ResourceAccumulator::default(),
            unlocked_rooms: HashSet::new(),
            unlocked_creatures: HashSet::new(),
            unlocked_spells: HashSet::new(),
            unlocked_traps: HashSet::new(),
            trap_inventory: HashMap::new(),
            trap_manufacturing_progress: HashMap::new(),
            completed_technologies: HashSet::new(),
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
            graveyard_corpses: 0,
            scavenger_conversion_progress: HashMap::new(),
        };

        // Assert initial state matches what test expects
        assert!(player.can_afford(100, 50));
        assert!(player.spend(100, 50));
        assert_eq!(player.gold, 100);
        assert_eq!(player.mana, 950);
    }

    #[test]
    fn test_resource_accumulation() {
        let mut player = PlayerState {
            gold: 0,
            mana: 0,
            food: 0,
            max_gold: 100,
            max_mana: 100,
            max_food: 100,
            materials: 0,
            max_materials: 100,
            accumulators: ResourceAccumulator::default(),
            unlocked_rooms: HashSet::new(),
            unlocked_creatures: HashSet::new(),
            unlocked_spells: HashSet::new(),
            unlocked_traps: HashSet::new(),
            trap_inventory: HashMap::new(),
            trap_manufacturing_progress: HashMap::new(),
            completed_technologies: HashSet::new(),
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
            graveyard_corpses: 0,
            scavenger_conversion_progress: HashMap::new(),
        };

        // Add small fractional amount 10 times
        for _ in 0..10 {
            player.add_resources_precise(0.1, 0.0, 0.0, 0.0);
        }

        // Should have 1 gold
        assert_eq!(player.gold, 1);
        // Accumulator should be effectively 0 (or close to it due to float precision)
        assert!(player.accumulators.gold < 0.001);

        // Add 0.3 three times
        for _ in 0..3 {
            player.add_resources_precise(0.0, 0.35, 0.0, 0.0);
        }
        // 0.35 * 3 = 1.05
        assert_eq!(player.mana, 1);
        assert!(player.accumulators.mana > 0.04); // Remaining 0.05
    }

    #[test]
    fn test_cannot_overspend() {
        let mut player = PlayerState {
            gold: 200,
            mana: 1000,
            food: 100,
            max_gold: 10000,
            max_mana: 5000,
            max_food: 1000,
            materials: 100,
            max_materials: 500,
            accumulators: ResourceAccumulator::default(),
            unlocked_rooms: HashSet::new(),
            unlocked_creatures: HashSet::new(),
            unlocked_spells: HashSet::new(),
            unlocked_traps: HashSet::new(),
            trap_inventory: HashMap::new(),
            trap_manufacturing_progress: HashMap::new(),
            completed_technologies: HashSet::new(),
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
            graveyard_corpses: 0,
            scavenger_conversion_progress: HashMap::new(),
        };

        assert!(!player.can_afford(10000, 0));
        assert!(!player.spend(10000, 0));
        assert_eq!(player.gold, 200); // Unchanged
    }

    #[test]
    fn test_resource_caps() {
        let mut player = PlayerState {
            gold: 200,
            mana: 1000,
            food: 100,
            max_gold: 10000,
            max_mana: 5000,
            max_food: 1000,
            materials: 100,
            max_materials: 500,
            accumulators: ResourceAccumulator::default(),
            unlocked_rooms: HashSet::new(),
            unlocked_creatures: HashSet::new(),
            unlocked_spells: HashSet::new(),
            unlocked_traps: HashSet::new(),
            trap_inventory: HashMap::new(),
            trap_manufacturing_progress: HashMap::new(),
            completed_technologies: HashSet::new(),
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
            graveyard_corpses: 0,
            scavenger_conversion_progress: HashMap::new(),
        };

        player.add_resources(0, 20000, 1000, 1000);
        assert_eq!(player.mana, player.max_mana);
        assert_eq!(player.food, player.max_food);
        assert_eq!(player.materials, player.max_materials);
    }
}
