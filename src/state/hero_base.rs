use crate::state::tile_state::TilePos;
use crate::state::entities::EntityManager;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroBuilding {
    pub id: String, // Unique instance ID
    pub building_type: String, // Reference to HeroBuildingData.id
    pub pos: TilePos, // Top-left position
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
        }
    }

    /// Check if all buildings are destroyed using entity health
    pub fn is_defeated(&self, entities: &EntityManager) -> bool {
        // Victory condition: All buildings destroyed
        if !self.enabled { return false; }
        
        if self.buildings.is_empty() {
            return true;
        }

        self.buildings.iter().all(|b| b.is_destroyed(entities))
    }
}
