use crate::data::GameData;
use crate::state::game_state::GameState;
use crate::state::entities::{EntityId, HeroState};
use crate::state::tile_state::TilePos;

pub fn update_hero_spawning(state: &mut GameState, game_data: &GameData, dt: f32) {
    if !state.hero_base.enabled {
        return;
    }

    let mut heroes_to_spawn = Vec::new();

    // Iterate over buildings to check timers
    for building in &mut state.hero_base.buildings {
        // Check if building is still alive (physical entity exists)
        let is_alive = if let Some(eid) = building.entity_id {
            state.entities.get(eid).is_some()
        } else {
            // Should have an entity ID if detection worked. 
            // If not, maybe it was destroyed before we could track it? 
            false 
        };

        if !is_alive {
            continue; // Destroyed buildings don't spawn
        }

        for timer in &mut building.spawn_timers {
            timer.time_until_spawn -= dt;

            if timer.time_until_spawn <= 0.0 {
                // Determine spawn rate
                let base_rate = if let Some(bd) = game_data.hero_buildings.get(&building.building_type) {
                    bd.spawn_triggers.iter().find(|t| t.hero_id == timer.hero_id)
                        .map(|t| t.spawn_rate_seconds)
                        .unwrap_or(60.0)
                } else {
                    60.0
                };

                timer.time_until_spawn = base_rate;
                
                // Queue for spawning
                heroes_to_spawn.push((timer.hero_id.clone(), building.pos));
            }
        }
    }

    // Spawn the heroes
    for (hero_id, pos) in heroes_to_spawn {
        spawn_hero_at(state, &hero_id, pos, game_data);
    }
}

fn spawn_hero_at(state: &mut GameState, hero_id: &str, building_pos: TilePos, game_data: &GameData) {
    if let Some(hero_data) = game_data.heroes.get(hero_id) {
        let spawn_pos = find_spawn_pos(state, building_pos, game_data);
        
        let hero_state = HeroState::new(
            hero_id.to_string(),
            1, // Level 1
            hero_data.stats.health,
            hero_data.stats.mana,
            spawn_pos,
        );
        state.entities.spawn_hero(spawn_pos, hero_state);
        eprintln!("Spawned {} at {:?}", hero_id, spawn_pos);
    }
}

fn find_spawn_pos(state: &GameState, center: TilePos, game_data: &GameData) -> TilePos {
    // Try to find a walkable tile near the center
    let (w, h) = crate::engine::tile_grid::get_grid_dimensions(&state.dungeon.grid);
    
    // Check increasing radius
    for r in 1..=4 {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx*dx + dy*dy <= r*r { 
                    let x = center.x + dx;
                    let y = center.y + dy;
                    if x >= 0 && y >= 0 && x < w as i32 && y < h as i32 {
                        let pos = TilePos::new(x, y);
                        // Check if walkable
                        if let Some(tile) = state.dungeon.get_tile(pos) {
                            let blocks = if let Some(td) = game_data.tiles.get(&tile.tile_type) {
                                td.blocks_movement
                            } else {
                                false
                            };

                            if !blocks {
                                if state.entities.at_position(pos).count() == 0 {
                                    return pos;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Fallback
    center
}
