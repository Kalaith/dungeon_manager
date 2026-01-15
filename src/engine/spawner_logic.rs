use crate::data::GameData;
use crate::engine::tile_grid::Grid;
use crate::state::entities::{CreatureState, EntityManager};
use crate::state::tile_state::TilePos;

#[derive(Clone, Debug, PartialEq)] // Added PartialEq
pub struct MonsterSpawner {
    pub pos: TilePos,
    pub monster_id: String,
    pub timer: f32,
    pub spawn_interval: f32,
    pub spawn_count: usize,
    pub max_spawns: usize,
}

impl MonsterSpawner {
    pub fn new(pos: TilePos, monster_id: String, max_spawns: usize) -> Self {
        Self {
            pos,
            monster_id,
            timer: 0.0,
            spawn_interval: 60.0, // 1 minute
            spawn_count: 0,
            max_spawns,
        }
    }
}

pub struct SpawnerSystem;

impl SpawnerSystem {
    pub fn update(
        spawners: &mut Vec<MonsterSpawner>,
        grid: &Grid,
        entities: &mut EntityManager,
        game_data: &GameData,
        dt: f32,
    ) {
        // Collect indices of spawners that need to spawn
        let mut spawn_indices = Vec::new();

        for (index, spawner) in spawners.iter_mut().enumerate() {
            // Check if spawner tile still exists (hasn't been destroyed)
            // Assuming "monster_spawner" tile type
            let tile = &grid[spawner.pos.y as usize][spawner.pos.x as usize];
            if tile.tile_type != "monster_spawner" {
                // Spawner destroyed!
                // We'll filter these out later or handle removal
                continue;
            }

            if spawner.spawn_count >= spawner.max_spawns {
                continue;
            }

            spawner.timer += dt;
            if spawner.timer >= spawner.spawn_interval {
                spawner.timer = 0.0;
                spawn_indices.push(index);
            }
        }

        // Process spawns
        for index in spawn_indices {
            let spawner = &mut spawners[index];
            
            // Spawn the monster
            if let Some(monster_data) = game_data.monsters.get(&spawner.monster_id) {
                let creature = CreatureState::new(
                    spawner.monster_id.clone(),
                    1, // Level 1
                    monster_data.stats.health,
                    monster_data.stats.mana,
                );
                
                // Spawn on the spawner tile
                entities.spawn_creature(spawner.pos, creature); // Removed unused result
                spawner.spawn_count += 1;
                
                // eprintln!("Spawner at {:?} spawned {}", spawner.pos, spawner.monster_id);
            }
        }
        
        // Remove destroyed spawners
        spawners.retain(|s| {
            let tile = &grid[s.pos.y as usize][s.pos.x as usize];
            tile.tile_type == "monster_spawner"
        });
    }
}
