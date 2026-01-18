use crate::state::tile_state::TilePos;
use crate::data::hero_buildings::HeroBuildingData;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroBuilding {
    pub id: String, // Unique instance ID
    pub building_type: String, // Reference to HeroBuildingData.id
    pub pos: TilePos, // Top-left position
    pub current_hp: i32,
    pub spawn_timers: Vec<SpawnTimer>,
    pub entity_id: Option<crate::state::entities::EntityId>,
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

    pub fn add_building(&mut self, building_type: &str, pos: TilePos, data: &HeroBuildingData) {
        let mut spawn_timers = Vec::new();
        for trigger in &data.spawn_triggers {
            spawn_timers.push(SpawnTimer {
                hero_id: trigger.hero_id.clone(),
                time_until_spawn: trigger.spawn_rate_seconds,
            });
        }

        self.buildings.push(HeroBuilding {
            id: format!("{}_{}_{}", building_type, pos.x, pos.y),
            building_type: building_type.to_string(),
            pos,
            current_hp: data.hp,
            spawn_timers,
            entity_id: None,
        });
    }

    pub fn get_town_hall(&self) -> Option<&HeroBuilding> {
        self.buildings.iter().find(|b| b.building_type == "town_hall")
    }

    pub fn is_defeated(&self) -> bool {
        // Victory condition: All buildings destroyed
        if !self.enabled { return false; }
        
        if self.buildings.is_empty() {
            return true;
        }

        self.buildings.iter().all(|b| b.current_hp <= 0)
    }
}
