use crate::state::entities::EntityManager;
use crate::state::tile_state::TilePos;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroBuilding {
    pub id: String,            // Unique instance ID
    pub building_type: String, // Reference to HeroBuildingData.id
    pub pos: TilePos,          // Top-left position
    pub spawn_timers: Vec<SpawnTimer>,
    pub entity_id: Option<crate::state::entities::EntityId>,
}

impl HeroBuilding {
    /// Check if this building is destroyed by reading health from entity system
    pub fn is_destroyed(&self, entities: &EntityManager) -> bool {
        if let Some(entity_id) = self.entity_id {
            if let Some(entity) = entities.get(entity_id) {
                return !entity.is_alive();
            }
        }
        // No entity assigned - consider destroyed
        true
    }

    /// Get current health from entity system
    pub fn get_health(&self, entities: &EntityManager) -> (f32, f32) {
        if let Some(entity_id) = self.entity_id {
            if let Some(entity) = entities.get(entity_id) {
                if let crate::state::entities::EntityType::Structure(s) = &entity.entity_type {
                    return (s.health, s.max_health);
                }
            }
        }
        (0.0, 0.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnTimer {
    pub hero_id: String,
    pub time_until_spawn: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroBase {
    pub buildings: Vec<HeroBuilding>,
    pub position: TilePos, // General location (e.g., center of base)
    pub enabled: bool,

    // Wave attack state
    /// Time until the next attack wave launches
    pub time_until_next_wave: f32,
    /// Current wave number (increments after each wave is defeated)
    pub current_wave_number: u32,
    /// Whether the current wave is actively attacking
    pub wave_in_progress: bool,
    /// Count of attackers in current wave that are still alive
    pub active_attackers: u32,

    // Accumulated consequences of razing hero buildings. `hero_buildings.json`
    // has authored a `destruction_effect` for every building since it shipped,
    // and nothing applied any of them. These persist: burning the armory has
    // to keep mattering after the rubble cools.
    /// Percentage slowdown on hero spawn timers, from destroyed spawn
    /// buildings. Clamped below 100 so razing everything cannot make the
    /// remaining garrison immortal by dividing by zero.
    #[serde(default)]
    pub spawn_rate_penalty_percent: f32,
    /// Percentage cut to hero movement speed, from destroyed stables.
    #[serde(default)]
    pub hero_speed_penalty_percent: f32,
    /// Flat attack and defence lost by every hero, from destroyed armouries
    /// and forges.
    #[serde(default)]
    pub hero_attack_penalty: f32,
    #[serde(default)]
    pub hero_defense_penalty: f32,
    /// Set when a building whose destruction effect is `win_game` falls. The
    /// hero base counts as defeated from that moment, so the existing
    /// `DestroyAllHeroBuildings` objective resolves without new plumbing —
    /// razing the town hall wins the map outright.
    #[serde(default)]
    pub decisive_building_destroyed: bool,
}

impl HeroBase {
    pub fn new(game_data: &crate::data::GameData) -> Self {
        Self {
            buildings: Vec::new(),
            position: TilePos { x: 0, y: 0 },
            enabled: false,
            time_until_next_wave: game_data.config.hero_waves.initial_delay,
            current_wave_number: 0,
            wave_in_progress: false,
            active_attackers: 0,
            spawn_rate_penalty_percent: 0.0,
            hero_speed_penalty_percent: 0.0,
            hero_attack_penalty: 0.0,
            hero_defense_penalty: 0.0,
            decisive_building_destroyed: false,
        }
    }

    /// Check if all buildings are destroyed using entity health
    pub fn is_defeated(&self, entities: &EntityManager) -> bool {
        // Victory condition: All buildings destroyed
        if !self.enabled {
            return false;
        }

        if self.decisive_building_destroyed {
            return true;
        }

        if self.buildings.is_empty() {
            return true;
        }

        self.buildings.iter().all(|b| b.is_destroyed(entities))
    }

    /// Multiplier on hero spawn timers. Above 1.0 means slower.
    pub fn spawn_interval_multiplier(&self) -> f32 {
        100.0 / (100.0 - self.spawn_rate_penalty_percent).max(10.0)
    }

    /// Multiplier on hero movement speed. Below 1.0 means slower.
    pub fn hero_speed_multiplier(&self) -> f32 {
        (1.0 - self.hero_speed_penalty_percent / 100.0).max(0.1)
    }

    /// Record a building's authored destruction effect.
    pub fn apply_destruction_effect(
        &mut self,
        effect: &crate::data::hero_buildings::DestructionEffect,
    ) {
        use crate::data::hero_buildings::DestructionEffect as Effect;
        match effect {
            Effect::WinGame => self.decisive_building_destroyed = true,
            Effect::ReduceSpawnRate { percent } => {
                self.spawn_rate_penalty_percent =
                    (self.spawn_rate_penalty_percent + *percent as f32).min(90.0);
            }
            Effect::ReduceHeroSpeed { percent } => {
                self.hero_speed_penalty_percent =
                    (self.hero_speed_penalty_percent + *percent as f32).min(90.0);
            }
            Effect::ReduceHeroStats { attack, defense } => {
                self.hero_attack_penalty += *attack as f32;
                self.hero_defense_penalty += *defense as f32;
            }
            // Walls and gates: the destruction itself already turns the tile
            // into open floor, which *is* the path opening.
            Effect::OpenPath => {}
        }
    }
}
