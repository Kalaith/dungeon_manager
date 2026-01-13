use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub icon: String,
    pub build: BuildData,
    pub requirements: RequirementsData,
    pub effects: EffectsData,
    pub scaling: ScalingData,
    pub ai: AIData,
    pub visual: RoomVisualData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildData {
    pub cost_per_tile: i32,
    pub mana_cost: i32,
    pub min_tiles: u32,
    pub max_tiles: u32,
    pub dig_required: bool,
    pub requires_claimed: bool,
    pub can_overlap: bool,
    pub allowed_terrain: Vec<String>,
    pub construction_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementsData {
    pub research: Vec<String>,
    pub global_rooms_required: Vec<String>,
    pub max_instances: u32,
    pub forbidden_if: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectsData {
    #[serde(default)]
    pub mana_generation_per_second: f32,
    #[serde(default)]
    pub happiness_modifier: i32,
    #[serde(default)]
    pub sleep_recovery_rate: f32,
    #[serde(default)]
    pub food_generation_per_second: f32,
    #[serde(default)]
    pub gold_storage: i32,
    #[serde(default)]
    pub xp_per_minute: f32,
    #[serde(default)]
    pub research_per_minute: f32,
    #[serde(default)]
    pub torture_power: f32,
    #[serde(default)]
    pub creature_defense_modifier: f32,
    #[serde(default)]
    pub spawn_rate_modifier: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingData {
    pub per_tile_multiplier: f32,
    pub size_thresholds: Vec<SizeThreshold>,
    pub shape_penalties: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeThreshold {
    pub tiles: u32,
    pub multiplier: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIData {
    pub task_type: String,
    pub desirability: f32,
    pub max_creatures: u32,
    pub preferred_creatures: Vec<String>,
    pub forbidden_creatures: Vec<String>,
    pub entry_conditions: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomVisualData {
    pub floor_sprite: String,
    pub wall_sprite: String,
    pub object_spawn: Vec<ObjectSpawn>,
    pub light: LightEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectSpawn {
    pub object: String,
    pub density: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightEffect {
    pub color: [u8; 3],
    pub intensity: f32,
    pub flicker: bool,
}

pub fn load_rooms() -> Result<HashMap<String, RoomData>, Box<dyn Error>> {
    let json_content = include_str!("../../assets/data/rooms.json");
    let rooms_vec: Vec<RoomData> = serde_json::from_str(json_content)?;

    let mut rooms_map = HashMap::new();
    for room in rooms_vec {
        rooms_map.insert(room.id.clone(), room);
    }

    Ok(rooms_map)
}
