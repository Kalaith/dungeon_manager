use crate::state::tile_state::TilePos;
use crate::data::hero_buildings::{HeroBuildingData, SpawnTrigger};
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
}

impl HeroBase {
    pub fn new() -> Self {
        Self {
            buildings: Vec::new(),
            position: TilePos { x: 0, y: 0 },
            enabled: false,
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
        // If Town Hall is destroyed (not present or HP <= 0), base is defeated
        // Ideally we check if it exists in the list; if it's destroyed intended behavior is to remove it or mark dead
        // For now, let's assume if it's gone or 0 HP, we win.
        if let Some(hall) = self.get_town_hall() {
            hall.current_hp <= 0
        } else {
            // No town hall found - implies it hasn't generated yet OR was destroyed and removed.
            // If enabled is true, and no town hall, then it's defeated.
             self.enabled 
        }
    }
}
