use crate::data::GameData;
use crate::state::dungeon::Dungeon;
use crate::state::entities::{EntityManager, CreatureState};
use crate::state::tile_state::{TilePos, TileState};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct MapFile {
    pub name: String,
    pub description: String,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<String>,
    pub legend: HashMap<char, String>,
    pub entities: Vec<MapEntity>,
}

#[derive(Debug, Deserialize)]
pub struct MapEntity {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub owner: String,
}

pub fn load_map(
    path: &str, 
    game_data: &GameData, 
    entities: &mut EntityManager
) -> Result<Dungeon, Box<dyn Error>> {
    let json_content = std::fs::read_to_string(path)?;
    let map_data: MapFile = serde_json::from_str(&json_content)?;

    // Validate dimensions
    if map_data.tiles.len() != map_data.height {
        return Err(format!("Map height mismatch. Expected {}, got {}", map_data.height, map_data.tiles.len()).into());
    }

    let mut grid = Vec::with_capacity(map_data.height);

    for (y, row_str) in map_data.tiles.iter().enumerate() {
        if row_str.len() != map_data.width {
            return Err(format!("Map width mismatch on row {}. Expected {}, got {}", y, map_data.width, row_str.len()).into());
        }

        let mut row = Vec::with_capacity(map_data.width);
        for (x, char) in row_str.chars().enumerate() {
            let tile_type = map_data.legend.get(&char)
                .ok_or_else(|| format!("Unknown tile character '{}' at ({}, {})", char, x, y))?;

            let pos = TilePos::new(x as i32, y as i32);
            let mut tile = TileState::new(tile_type.clone(), pos);

            // Set resources if applicable
            if let Some(tile_data) = game_data.tiles.get(tile_type) {
                if let Some(resources) = &tile_data.resources {
                     if resources.amount > 0 {
                        tile = tile.with_resources(resources.amount as u32);
                     }
                }
            }
            // Logic for specific tile types (e.g. gold veins default amount override if needed)
             if tile_type == "gold_vein" {
                tile = tile.with_resources(100);
            }

            row.push(tile);
        }
        grid.push(row);
    }
    
    // Create Dungeon instance
    let mut dungeon = Dungeon {
        grid,
        width: map_data.width,
        height: map_data.height,
    };

    // Spawn entities
    for entity_def in map_data.entities {
        let pos = TilePos::new(entity_def.x, entity_def.y);
        
        // Check if it's a monster/creature
        if let Some(monster_data) = game_data.monsters.get(&entity_def.id) {
            let visual_seed = macroquad::rand::gen_range(0u64, u64::MAX);
            let creature_state = CreatureState::new(
                entity_def.id.clone(),
                1, // level
                monster_data.stats.health,
                monster_data.stats.mana,
                visual_seed,
            );
            entities.spawn_creature(pos, creature_state);
        }
        // Could handle other entity types here (traps, items, etc.)
        
        // If it's the dungeon heart, claim the area around it
        if let Some(tile) = dungeon.get_tile(pos) {
             if tile.tile_type == "dungeon_heart" {
                 // Claim 3x3 area around heart
                 for dy in -1..=1 {
                     for dx in -1..=1 {
                         let claim_pos = TilePos::new(pos.x + dx, pos.y + dy);
                         if let Some(t) = dungeon.get_tile_mut(claim_pos) {
                             t.claim();
                         }
                     }
                 }
             }
        }
    }

    Ok(dungeon)
}
