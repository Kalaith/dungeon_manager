use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    #[serde(default)]
    pub rooms: Vec<String>,
    #[serde(default)]
    pub spells: Vec<String>,
    #[serde(default)]
    pub creatures: Vec<String>,
    #[serde(default)]
    pub traps: Vec<String>,
}

pub fn load_technologies() -> Result<HashMap<String, TechData>, Box<dyn Error>> {
    let json_content = {
        #[cfg(target_arch = "wasm32")]
        {
            include_str!("../../assets/data/technologies.json").to_string()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::fs::read_to_string("assets/data/technologies.json")?
        }
    };

    let techs_vec: Vec<TechData> = serde_json::from_str(&json_content)?;

    let mut techs_map = HashMap::new();
    for tech in techs_vec {
        techs_map.insert(tech.id.clone(), tech);
    }

    Ok(techs_map)
}
