use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: f32, // Research points needed
    pub prerequisites: Vec<String>,
    pub unlocks: UnlockData,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockData {
    pub rooms: Vec<String>,
    pub spells: Vec<String>,
    pub creatures: Vec<String>,
}

pub fn load_technologies() -> Result<HashMap<String, TechData>, Box<dyn Error>> {
    let json_content = include_str!("../../assets/data/technologies.json");
    let techs_vec: Vec<TechData> = serde_json::from_str(json_content)?;

    let mut techs_map = HashMap::new();
    for tech in techs_vec {
        techs_map.insert(tech.id.clone(), tech);
    }

    Ok(techs_map)
}
