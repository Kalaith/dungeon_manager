//! Imp AI - Digging and wandering behavior
//! Stateless service for imp-specific behavior

use crate::data::GameData;
use crate::engine::pathfinding::{find_path, Heuristic, PathfindingGrid, Pos};
use crate::engine::tile_types::{self, types as tt};
use crate::state::dungeon::Dungeon;
use crate::state::entities::{EntityId, EntityManager};
use crate::state::player_state::PlayerState;
use crate::state::tile_state::{Ownership, TilePos};
use std::collections::HashSet;

/// Update all imp digging and movement behavior
pub fn update_imp_digging(
    dungeon: &mut Dungeon,
    entities: &mut EntityManager,
    player: &mut PlayerState,
    game_data: &GameData,
    dt: f32,
) {

    // Get all imp IDs
    let mut imp_ids: Vec<EntityId> = entities
        .creatures()
        .filter(|(_, creature)| creature.creature_id == "imp")
        .map(|(id, _)| id)
        .collect();

    // Sort by ID for deterministic behavior to prevent task flapping
    imp_ids.sort();

    // Track tiles being targeted by other imps to prevent multiple imps on one tile
    let mut targeted_tiles = collect_targeted_tiles(entities, &imp_ids);

    for imp_id in imp_ids {
        // Get imp position
        let imp_pos = match entities.get(imp_id) {
            Some(e) => e.pos,
            None => continue,
        };

        // Check if imp has a path
        let should_check_dig = {
            let entity = entities.get(imp_id).unwrap();
            let creature = entity.as_creature().unwrap();
            creature.current_path.is_none()
        };

        // Remove THIS imp's targets from the exclusion set so it can 'find' its own job
        if let Some(entity) = entities.get(imp_id) {
             if let Some(creature) = entity.as_creature() {
                  if let Some(crate::state::entities::Task::Dig(pos)) = creature.current_task {
                       targeted_tiles.remove(&pos);
                  }
                  if let Some(path) = &creature.current_path {
                       if let Some(last_pos) = path.last() {
                            targeted_tiles.remove(last_pos);
                       }
                  }
             }
             targeted_tiles.remove(&imp_pos);
        }

        // If imp has no path, find nearest marked tile
        if should_check_dig {
            process_idle_imp(
                dungeon,
                entities,
                player,
                imp_id,
                imp_pos,
                &targeted_tiles,
                game_data,
                dt,
            );
        }

        // Re-add potentially excluded target for next iterations
        update_targeted_tiles(entities, imp_id, imp_pos, &mut targeted_tiles);

        // Handle movement along path
        crate::engine::movement::process_entity_movement(entities, imp_id, dt);
    }
}

/// Collect all tiles currently targeted by imps
fn collect_targeted_tiles(entities: &EntityManager, imp_ids: &[EntityId]) -> HashSet<TilePos> {
    let mut targeted_tiles = HashSet::new();
    for &other_id in imp_ids {
        if let Some(entity) = entities.get(other_id) {
            if let Some(creature) = entity.as_creature() {
                // Check current task for Dig targets
                if let Some(crate::state::entities::Task::Dig(pos)) = creature.current_task {
                    targeted_tiles.insert(pos);
                }

                if let Some(path) = &creature.current_path {
                    if let Some(last_pos) = path.last() {
                        targeted_tiles.insert(*last_pos);
                    }
                }
                if creature.current_path.is_none() && creature.current_task.is_none() {
                    targeted_tiles.insert(entity.pos);
                }
            }
        }
    }
    targeted_tiles
}

/// Update the targeted tiles set for next iterations
fn update_targeted_tiles(
    entities: &EntityManager,
    imp_id: EntityId,
    imp_pos: TilePos,
    targeted_tiles: &mut HashSet<TilePos>,
) {
    targeted_tiles.insert(imp_pos);
    if let Some(entity) = entities.get(imp_id) {
        if let Some(creature) = entity.as_creature() {
            if let Some(path) = &creature.current_path {
                if let Some(last_pos) = path.last() {
                    targeted_tiles.insert(*last_pos);
                }
            }
        }
    }
}

/// Process an idle imp - find work or wander
fn process_idle_imp(
    dungeon: &mut Dungeon,
    entities: &mut EntityManager,
    player: &mut PlayerState,
    imp_id: EntityId,
    imp_pos: TilePos,
    targeted_tiles: &HashSet<TilePos>,
    game_data: &GameData,
    dt: f32,
) {
    // Find nearest marked tile NOT targeted by others
    let nearest_marked = find_nearest_marked_tile(dungeon, imp_pos, targeted_tiles, player);

    if let Some(marked_pos) = nearest_marked {
        // Check if we are adjacent (or on top) of the marked tile
        if imp_pos.manhattan_distance(&marked_pos) <= 1 {
            // Imp is in position - dig it
            process_digging(dungeon, entities, player, imp_id, marked_pos, dt);
        } else {
            // Reset task time when moving, but ONLY if we are starting a move to a new target
            // If we are adjacent, we shouldn't be moving anyway?
            // Actually, if we are NOT at distance <= 1, we must move.
            if let Some(entity) = entities.get_mut(imp_id) {
                if let Some(creature) = entity.as_creature_mut() {
                    creature.task_time = 0.0;
                    creature.current_task = None; // clear task if we have to move
                }
            }
            
            // Find a valid standing spot next to the marked tile
            let neighbors = crate::engine::tile_grid::get_cardinal_neighbors(&dungeon.grid, marked_pos);
            let mut best_target = None;
            let mut min_dist = f32::MAX;

            for target in neighbors {
                if let Some(tile) = dungeon.get_tile(target) {
                    if tile_types::is_walkable(&tile.tile_type, game_data) {
                        let d = imp_pos.distance_to(&target);
                        if d < min_dist {
                            min_dist = d;
                            best_target = Some(target);
                        }
                    }
                }
            }

            if let Some(target) = best_target {
                pathfind_to_target(dungeon, entities, imp_id, imp_pos, target, false);
            } else {
                // Cannot reach the tile (surrounded by walls?)
                wander_randomly(dungeon, entities, imp_id, imp_pos, game_data);
            }
        }
    } else {
        // No marked tiles - imp should wander
        wander_randomly(dungeon, entities, imp_id, imp_pos, game_data);
    }
}

/// Find the nearest marked tile not targeted by other imps
fn find_nearest_marked_tile(
    dungeon: &Dungeon,
    imp_pos: TilePos,
    targeted_tiles: &HashSet<TilePos>,
    player: &PlayerState,
) -> Option<TilePos> {
    let mut nearest_marked = None;
    let mut min_dist = f32::MAX;

    for y in 0..dungeon.height {
        for x in 0..dungeon.width {
            let pos = TilePos::new(x as i32, y as i32);
            if pos != imp_pos && targeted_tiles.contains(&pos) {
                continue;
            }
            if let Some(tile) = dungeon.get_tile(pos) {
                if tile.marked_for_dig {
                    let dist_sq = Pos::new(pos.x, pos.y).euclidean_distance_squared(&Pos::new(imp_pos.x, imp_pos.y));
                    let mut dist = dist_sq;

                    // Penalize gem seams (infinite gold) so they are lower priority
                    if tile.tile_type == tt::GEM_SEAM {
                        if player.gold >= player.max_gold {
                            continue; // Don't mine infinite gold if full
                        }
                        dist += 10000.0;
                    }

                    if dist < min_dist {
                        min_dist = dist;
                        nearest_marked = Some(pos);
                    }
                }
            }
        }
    }
    nearest_marked
}

/// Process digging at a marked tile
fn process_digging(
    dungeon: &mut Dungeon,
    entities: &mut EntityManager,
    player: &mut PlayerState,
    imp_id: EntityId,
    marked_pos: TilePos,
    dt: f32,
) {
    // Check if dig delay is complete
    let mut task_complete = false;
    if let Some(entity) = entities.get_mut(imp_id) {
        if let Some(creature) = entity.as_creature_mut() {
            // Set task if not set (ensures we "claim" the tile)
            if creature.current_task.is_none() {
                creature.current_task = Some(crate::state::entities::Task::Dig(marked_pos));
            }
            
            creature.task_time += dt;
            if creature.task_time >= 2.0 {
                creature.task_time = 0.0;
                creature.current_task = None; // Task done
                task_complete = true;
            }
        }
    }

    if task_complete {
        complete_dig(dungeon, player, marked_pos);
    }
}

/// Complete digging a tile and award resources
fn complete_dig(dungeon: &mut Dungeon, player: &mut PlayerState, marked_pos: TilePos) {
    if let Some(tile) = dungeon.get_tile_mut(marked_pos) {
        if !tile.marked_for_dig {
            return;
        }

        // Check if tile has resources
        let (gold_gained, mana_gained, is_gem_seam) = match tile.tile_type.as_str() {
            x if x == tt::GOLD_VEIN => {
                let gold = tile.resources_remaining.map_or(50, |r| 50.min(r as i32));
                (gold, 0, false)
            }
            x if x == tt::GEM_SEAM => (25, 0, true),
            x if x == tt::MANA_CRYSTAL => {
                let mana = tile.resources_remaining.map_or(20, |r| 20.min(r as i32));
                (0, mana, false)
            }
            _ => (0, 0, false),
        };

        if is_gem_seam {
            // Gem seams stay marked - imps will keep mining them continuously
            // Do NOT unmark the tile, do NOT convert to floor
            player.add_resources(gold_gained, 0, 0, 0);
            eprintln!(
                "Imp mined gem seam at {:?}, gained {} gold",
                marked_pos, gold_gained
            );
            return; // Exit early to preserve tile state
        }
        
        // Safety check: if gem seam and gold full, do nothing (should be handled by target selection but good for safety)
        if is_gem_seam && player.gold >= player.max_gold {
             return;
        }

        // For non-infinite resources:
        // Convert to claimed floor
        tile.tile_type = tt::CLAIMED_FLOOR.to_string();
        tile.ownership = Ownership::Player;
        tile.marked_for_dig = false;
        tile.resources_remaining = None;
        player.claimed_tile_count += 1;

        // Give resources to player
        if gold_gained > 0 {
            player.add_resources(gold_gained, 0, 0, 0);
            eprintln!(
                "Imp dug gold vein at {:?}, gained {} gold",
                marked_pos, gold_gained
            );
        } else if mana_gained > 0 {
            player.mana = (player.mana + mana_gained as i32).min(player.max_mana);
            eprintln!(
                "Imp mined mana crystal at {:?}, gained {} mana",
                marked_pos, mana_gained
            );
        }
    }
}

/// Pathfind to a target position
fn pathfind_to_target(
    dungeon: &Dungeon,
    entities: &mut EntityManager,
    imp_id: EntityId,
    imp_pos: TilePos,
    target_pos: TilePos,
    include_marked: bool,
) {
    let mut pf_grid = PathfindingGrid::new(dungeon.width, dungeon.height);

    // Mark walkable tiles
    for y in 0..dungeon.height {
        for x in 0..dungeon.width {
            let tile_pos = TilePos::new(x as i32, y as i32);
            if let Some(tile) = dungeon.get_tile(tile_pos) {
                let walkable = if include_marked {
                    tile.ownership == Ownership::Player || tile.marked_for_dig
                } else {
                    tile.ownership == Ownership::Player
                };
                let pf_pos = Pos::new(x as i32, y as i32);
                pf_grid.set_walkable(pf_pos, walkable);
            }
        }
    }

    // Find path
    let start = Pos::new(imp_pos.x, imp_pos.y);
    let goal = Pos::new(target_pos.x, target_pos.y);

    if let Some(path) = find_path(start, goal, &pf_grid, Heuristic::Manhattan, false) {
        let waypoints: Vec<TilePos> = path
            .waypoints
            .into_iter()
            .map(|p| TilePos::new(p.x, p.y))
            .collect();

        if let Some(entity) = entities.get_mut(imp_id) {
            if let Some(creature) = entity.as_creature_mut() {
                creature.current_path = Some(waypoints);
            }
        }
    }
}

/// Pick a random walkable tile and wander to it
fn wander_randomly(
    dungeon: &Dungeon,
    entities: &mut EntityManager,
    imp_id: EntityId,
    imp_pos: TilePos,
    game_data: &GameData,
) {
    let wander_radius = 5;
    let mut attempts = 0;
    let mut wander_pos = None;

    while attempts < 10 && wander_pos.is_none() {
        let dx = rand::random::<i32>() % (wander_radius * 2 + 1) - wander_radius;
        let dy = rand::random::<i32>() % (wander_radius * 2 + 1) - wander_radius;
        let candidate = TilePos::new(imp_pos.x + dx, imp_pos.y + dy);

        if let Some(tile) = dungeon.get_tile(candidate) {
            if tile_types::is_walkable(&tile.tile_type, game_data) {
                wander_pos = Some(candidate);
            }
        }
        attempts += 1;
    }

    if let Some(target_pos) = wander_pos {
        if imp_pos != target_pos {
            pathfind_to_target(dungeon, entities, imp_id, imp_pos, target_pos, false);
        }
    }
}


// process_imp_movement removed in favor of shared movement::process_entity_movement
