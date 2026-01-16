use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use crate::engine::hero_ai; // Assuming we might need types later, but for now simple strings/structs

#[derive(Debug, Deserialize, Clone)]
pub struct HeroBuildingData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub hp: i32,
    pub width: i32,
    pub height: i32,
    pub spawn_triggers: Vec<SpawnTrigger>,
    pub destruction_effect: DestructionEffect,
    pub visual: BuildingVisual,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpawnTrigger {
    pub hero_id: String,
    pub spawn_rate_seconds: f32,
    pub max_active: usize,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DestructionEffect {
    #[serde(rename = "win_game")]
    WinGame,
    #[serde(rename = "reduce_spawn_rate")]
    ReduceSpawnRate { percent: i32 },
    #[serde(rename = "reduce_hero_speed")]
    ReduceHeroSpeed { percent: i32 },
    #[serde(rename = "reduce_hero_stats")]
    ReduceHeroStats { attack: i32, defense: i32 },
    #[serde(rename = "open_path")]
    OpenPath,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BuildingVisual {
    pub tile: String,
}

#[derive(Debug, Deserialize)]
struct HeroBuildingsWrapper {
    settings: Option<serde_json::Value>, // Optional settings if we add them later
}

pub fn load_hero_buildings() -> Result<HashMap<String, HeroBuildingData>, Box<dyn Error>> {
    let file_content = fs::read_to_string("assets/data/hero_buildings.json")?;
    
    // The JSON is an array of objects
    let buildings_list: Vec<HeroBuildingData> = serde_json::from_str(&file_content)?;
    
    let mut buildings_map = HashMap::new();
    for building in buildings_list {
        buildings_map.insert(building.id.clone(), building);
    }
    
    Ok(buildings_map)
}
