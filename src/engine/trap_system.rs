//! Trap construction system
//! Handles funding and building of traps/doors

use crate::data::GameData;
use crate::state::dungeon::Dungeon;
use crate::state::player_state::PlayerState;
use crate::state::tile_state::TilePos;
use std::collections::HashSet;

/// Get the material cost for a trap type from game data
pub fn get_trap_cost(trap_type: &str, game_data: &GameData) -> i32 {
    game_data.traps
        .get(trap_type)
        .map(|data| data.cost)
        .unwrap_or(50) // Default fallback
}

/// Get the build time for a trap type in seconds from game data
pub fn get_trap_build_time(trap_type: &str, game_data: &GameData) -> f32 {
    game_data.traps
        .get(trap_type)
        .map(|data| data.build_time)
        .unwrap_or(5.0) // Default fallback
}

/// Process trap construction progress
/// Returns positions of completed traps
pub fn process_trap_construction(
    dungeon: &mut Dungeon,
    player: &mut PlayerState,
    pending_trap_builds: &mut HashSet<TilePos>,
    game_data: &GameData,
    dt: f32,
) -> Vec<TilePos> {
    let mut completed_traps = Vec::new();

    // 1. Check pending traps to fund
    let pending: Vec<TilePos> = pending_trap_builds.iter().cloned().collect();
    for pos in &pending {
        if let Some(tile) = dungeon.get_tile_mut(*pos) {
            if let Some(ref mut trap) = tile.trap {
                if !trap.funded && !trap.constructed {
                    let cost = get_trap_cost(&trap.trap_type, game_data);

                    if player.materials >= cost {
                        player.materials -= cost;
                        trap.funded = true;
                        eprintln!("Funded trap at {:?}", pos);
                    }
                }
            }
        }
    }

    // 2. Progress funded traps
    for pos in pending {
        let mut finished = false;
        if let Some(tile) = dungeon.get_tile_mut(pos) {
            if let Some(ref mut trap) = tile.trap {
                if trap.funded && !trap.constructed {
                    let build_time = get_trap_build_time(&trap.trap_type, game_data);

                    trap.construction_progress += dt;
                    if trap.construction_progress >= build_time {
                        trap.constructed = true;
                        trap.active = true;
                        finished = true;
                        eprintln!("Trap construction complete at {:?}", pos);
                    }
                }
            }
        }

        if finished {
            completed_traps.push(pos);
        }
    }

    for pos in &completed_traps {
        pending_trap_builds.remove(pos);
    }

    completed_traps
}

