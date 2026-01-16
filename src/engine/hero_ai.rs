//! Hero AI system - goal-driven hero behavior
//!
//! Heroes operate on mission priorities and adapt to dungeon threats.
//! This module handles goal selection, threat evaluation, and target finding.

use crate::data::heroes::HeroData;
use crate::data::GameData;
use crate::state::entities::{Entity, HeroGoal, HeroState};
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;
use crate::engine::room_validator::Room;
use std::collections::HashMap;

/// Threat level assessment for heroes
#[derive(Debug, Clone, PartialEq)]
pub enum ThreatLevel {
    /// No threats nearby
    Safe,
    /// Some creatures nearby but manageable
    Moderate,
    /// Many creatures or strong opposition
    High,
    /// Overwhelming opposition, should retreat
    Overwhelming,
}

/// Decide the current goal for a hero based on their state and dungeon situation
pub fn decide_hero_goal(
    hero_state: &HeroState,
    game_state: &GameState,
    game_data: &GameData,
) -> HeroGoal {
    let hero_data = match game_data.heroes.get(&hero_state.hero_id) {
        Some(data) => data,
        None => return HeroGoal::DestroyHeart, // fallback
    };

    // Check if hero should retreat
    if hero_state.should_retreat(hero_data.ai.threat_response.retreat_below_health) {
        return HeroGoal::Retreat;
    }

    // Evaluate current situation - need hero position, but we don't have it here
    // For now, assume moderate threat if we can't evaluate
    let threat_level = ThreatLevel::Moderate; // Placeholder

    // If under high threat, focus on survival
    if threat_level == ThreatLevel::Overwhelming {
        return HeroGoal::Retreat;
    }

    // Primary goal logic based on hero type
    match hero_data.ai.primary_goal.as_str() {
        "destroy_heart" => {
            // Check if heart is still alive
            if is_dungeon_heart_alive(game_state) {
                HeroGoal::DestroyHeart
            } else {
                // Heart destroyed, switch to secondary goals
                choose_secondary_goal(hero_state, hero_data, game_state)
            }
        }
        "steal_gold" => {
            let target_gold = calculate_gold_target(hero_data);
            if hero_state.gold_stolen < target_gold {
                HeroGoal::StealGold(target_gold)
            } else {
                choose_secondary_goal(hero_state, hero_data, game_state)
            }
        }
        "kill_creatures" => {
            let target_kills = calculate_kill_target(hero_data);
            if hero_state.kills < target_kills {
                HeroGoal::KillCreatures(target_kills)
            } else {
                choose_secondary_goal(hero_state, hero_data, game_state)
            }
        }
        "explore" => HeroGoal::Explore,
        _ => HeroGoal::DestroyHeart, // fallback
    }
}

/// Choose a secondary goal when primary is complete
fn choose_secondary_goal(
    hero_state: &HeroState,
    hero_data: &HeroData,
    game_state: &GameState,
) -> HeroGoal {
    // Pick the highest priority secondary goal that's still viable
    for goal_name in &hero_data.ai.secondary_goals {
        match goal_name.as_str() {
            "steal_gold" => {
                let target = calculate_gold_target(hero_data);
                if hero_state.gold_stolen < target {
                    return HeroGoal::StealGold(target);
                }
            }
            "kill_creatures" => {
                let target = calculate_kill_target(hero_data);
                if hero_state.kills < target {
                    return HeroGoal::KillCreatures(target);
                }
            }
            "explore" => return HeroGoal::Explore,
            _ => continue,
        }
    }

    // Default to exploration if no secondary goals available
    HeroGoal::Explore
}

/// Calculate gold stealing target based on hero tier
fn calculate_gold_target(hero_data: &HeroData) -> i32 {
    match hero_data.tier {
        1 => 100,
        2 => 250,
        3 => 500,
        4 => 1000,
        5 => 2000,
        _ => 500,
    }
}

/// Calculate creature killing target based on hero tier
fn calculate_kill_target(hero_data: &HeroData) -> u32 {
    match hero_data.tier {
        1 => 3,
        2 => 8,
        3 => 15,
        4 => 25,
        5 => 50,
        _ => 10,
    }
}

/// Evaluate threat level around the hero
pub fn evaluate_threat(hero_pos: TilePos, game_state: &GameState) -> ThreatLevel {
    let mut nearby_creatures = 0;
    let mut nearby_strong_creatures = 0;

    // Count creatures within threat radius (roughly 5-8 tiles)
    let threat_radius = 6;

    for entity in game_state.entities.all() {
        if let Some(creature) = entity.as_creature() {
            let distance = calculate_distance(hero_pos, entity.pos);
            if distance <= threat_radius as f32 {
                nearby_creatures += 1;

                // Consider stronger creatures (higher level or specific types)
                if creature.level >= 3 || matches!(creature.creature_id.as_str(), "demon_spawn" | "troll") {
                    nearby_strong_creatures += 1;
                }
            }
        }
    }

    match (nearby_creatures, nearby_strong_creatures) {
        (0, _) => ThreatLevel::Safe,
        (1..=3, 0) => ThreatLevel::Moderate,
        (4..=6, _) | (_, 1..=2) => ThreatLevel::High,
        _ => ThreatLevel::Overwhelming,
    }
}

/// Find the best target room for the current hero goal
pub fn find_target_room(
    hero_pos: TilePos,
    hero_id: &str,
    goal: &HeroGoal,
    game_state: &GameState,
    game_data: &GameData,
) -> Option<usize> {
    let hero_data = game_data.heroes.get(hero_id)?;

    match goal {
        HeroGoal::DestroyHeart => {
            // Find dungeon heart room
            find_room_by_type(game_state, "dungeon_heart")
        }
        HeroGoal::StealGold(_) => {
            // Find treasury with highest priority
            find_best_room_by_priority(hero_pos, hero_id, "treasury", game_state, game_data)
        }
        HeroGoal::KillCreatures(_) => {
            // Find rooms with creatures (lairs, training halls)
            let room_types = ["lair", "training_hall"];
            find_room_with_creatures(hero_pos, hero_id, &room_types, game_state, game_data)
        }
        HeroGoal::SabotageRoom(room_id) => Some(*room_id),
        HeroGoal::Explore => {
            // Find unexplored areas (rooms with fog)
            find_unexplored_room(hero_pos, hero_id, game_state, game_data)
        }
        HeroGoal::RestAtSpawn(_) => None, // Not a room target
        HeroGoal::Retreat => {
            // Find path to entrance (simplified: any room near edge)
            find_entrance_room(game_state)
        }
    }
}

/// Find a room by its type
fn find_room_by_type(game_state: &GameState, room_type: &str) -> Option<usize> {
    for room in &game_state.room_manager.rooms {
        if room.room_type == room_type {
            return Some(room.id);
        }
    }
    None
}

/// Find the best room by hero's priority preferences
fn find_best_room_by_priority(
    hero_pos: TilePos,
    hero_id: &str,
    room_type: &str,
    game_state: &GameState,
    game_data: &GameData,
) -> Option<usize> {
    let hero_data = game_data.heroes.get(hero_id)?;

    let mut best_room = None;
    let mut best_priority = 0.0;

    for room in &game_state.room_manager.rooms {
        if room.room_type == room_type {
            let priority = hero_data.ai.room_priorities.get(&room.room_type).copied().unwrap_or(1.0);
            let distance_factor = calculate_room_distance_factor(hero_pos, room, game_state);

            let total_priority = priority / distance_factor; // Closer rooms get higher priority

            if total_priority > best_priority {
                best_priority = total_priority;
                best_room = Some(room.id);
            }
        }
    }

    best_room
}

/// Find a room that likely contains creatures
fn find_room_with_creatures(
    hero_pos: TilePos,
    hero_id: &str,
    room_types: &[&str],
    game_state: &GameState,
    game_data: &GameData,
) -> Option<usize> {
    let mut best_room = None;
    let mut best_score = 0.0;

    for room in &game_state.room_manager.rooms {
        if room_types.contains(&room.room_type.as_str()) {
            // Score based on room size and distance
            let size_score = room.tiles.len() as f32;
            let distance_factor = calculate_room_distance_factor(hero_pos, room, game_state);
            let score = size_score / distance_factor;

            if score > best_score {
                best_score = score;
                best_room = Some(room.id);
            }
        }
    }

    best_room
}

/// Find an unexplored room (with fog of war)
fn find_unexplored_room(
    hero_pos: TilePos,
    hero_id: &str,
    game_state: &GameState,
    game_data: &GameData,
) -> Option<usize> {
    // Look for rooms that have fog-covered tiles
    for room in &game_state.room_manager.rooms {
        for &tile_pos in &room.tiles {
            if let Some(tile) = game_state.get_tile(tile_pos) {
                if matches!(tile.fog_state, crate::state::tile_state::FogState::Hidden) {
                    return Some(room.id);
                }
            }
        }
    }

    // Fallback to any room
    game_state.room_manager.rooms.first().map(|r| r.id)
}

/// Find a room near the dungeon entrance
fn find_entrance_room(game_state: &GameState) -> Option<usize> {
    // Simplified: find room closest to (0,0) or edge of map
    let mut best_room = None;
    let mut best_distance = f32::INFINITY;

    for room in &game_state.room_manager.rooms {
        // Calculate distance from room center to origin
        let center_x = room.tiles.iter().map(|p| p.x as f32).sum::<f32>() / room.tiles.len() as f32;
        let center_y = room.tiles.iter().map(|p| p.y as f32).sum::<f32>() / room.tiles.len() as f32;
        let distance = (center_x.powi(2) + center_y.powi(2)).sqrt();

        if distance < best_distance {
            best_distance = distance;
            best_room = Some(room.id);
        }
    }

    best_room
}

/// Calculate distance factor for room priority (closer = higher priority)
fn calculate_room_distance_factor(hero_pos: TilePos, room: &Room, game_state: &GameState) -> f32 {
    // Find closest tile in room to hero
    let mut min_distance = f32::INFINITY;

    for &tile_pos in &room.tiles {
        let distance = calculate_distance(hero_pos, tile_pos);
        if distance < min_distance {
            min_distance = distance;
        }
    }

    // Convert to factor (closer = smaller factor = higher priority)
    (min_distance / 10.0).max(0.5).min(3.0)
}

/// Calculate Manhattan distance between two positions
fn calculate_distance(a: TilePos, b: TilePos) -> f32 {
    let dx = (a.x - b.x).abs() as f32;
    let dy = (a.y - b.y).abs() as f32;
    dx + dy // Manhattan distance
}

/// Check if dungeon heart is still alive
fn is_dungeon_heart_alive(game_state: &GameState) -> bool {
    // Check if heart room exists and has heart object
    for room in &game_state.room_manager.rooms {
        if room.room_type == "dungeon_heart" {
            // In a full implementation, check for heart entity or room health
            return true;
        }
    }
    false
}

// ... existing code ...

/// Update hero AI state based on current goal and situation
pub fn update_hero_ai(
    hero_entity: &Entity,
    hero_state: &mut HeroState,
    game_state: &GameState,
    game_data: &GameData,
    dt: f32,
) {
    // Re-evaluate goal periodically or when situation changes
    // Only re-evaluate if idle or goal complete
    if matches!(hero_state.current_goal, HeroGoal::RestAtSpawn(_)) || hero_state.current_path.is_none() {
         let new_goal = decide_hero_goal(hero_state, game_state, game_data);
         if new_goal != hero_state.current_goal {
            hero_state.current_goal = new_goal;
            hero_state.target_pos = None;
            hero_state.target_room_id = None;
            hero_state.current_path = None;
        }
    }

    // Update target if needed
    match &hero_state.current_goal {
        HeroGoal::RestAtSpawn(spawn_pos) => {
             // If we have no target, or we reached the target, pick a new random spot near spawn
             let needs_new_target = hero_state.target_pos.is_none() || 
                                   (hero_state.target_pos == Some(hero_entity.pos));
                                   
             if needs_new_target {
                 // Wander radius
                 let radius = 5;
                 let mut attempts = 0;
                 let mut found_target = None;
                 
                 while attempts < 10 {
                     let dx = (rand::random::<i32>() % (radius * 2 + 1)) - radius;
                     let dy = (rand::random::<i32>() % (radius * 2 + 1)) - radius;
                     let target_x = spawn_pos.x + dx;
                     let target_y = spawn_pos.y + dy;
                     let target_pos = TilePos::new(target_x, target_y);
                     
                     // Check if valid and walkable
                     if let Some(tile) = game_state.dungeon.get_tile(target_pos) {
                         if let Some(tile_data) = game_data.tiles.get(&tile.tile_type) {
                             if !tile_data.blocks_movement {
                                 found_target = Some(target_pos);
                                 break;
                             }
                         } else {
                            // Default valid if no data (assume floor)
                            found_target = Some(target_pos);
                            break;
                         }
                     }
                     attempts += 1;
                 }
                 
                 // If we found a valid wander target, go there. Otherwise stay at spawn.
                 if let Some(target) = found_target {
                     hero_state.target_pos = Some(target);
                 } else {
                     hero_state.target_pos = Some(*spawn_pos);
                 }
                 
                 // Reset path to force recalculation
                 hero_state.current_path = None;
             }
        }
        _ => {
            if hero_state.target_room_id.is_none() {
                hero_state.target_room_id = find_target_room(hero_entity.pos, &hero_state.hero_id, &hero_state.current_goal, game_state, game_data);
            }
        }
    }
    
    // Resolve Room ID to Target Pos
    if let Some(room_id) = hero_state.target_room_id {
        if hero_state.target_pos.is_none() {
             // Find room directly
             if let Some(room) = game_state.room_manager.rooms.iter().find(|r| r.id == room_id) {
                 // Pick random tile in room or center
                 if !room.tiles.is_empty() {
                    let idx = rand::random::<usize>() % room.tiles.len();
                    if let Some(&pos) = room.tiles.iter().nth(idx) {
                        hero_state.target_pos = Some(pos);
                    }
                 }
             }
        }
    }

    // Pathfinding
    if let Some(target) = hero_state.target_pos {
        // If we have no path or target changed (simplified: just check if no path)
        if hero_state.current_path.is_none() && target != hero_entity.pos {
             // Build pathfinding grid
             // note: this is expensive to do every frame for every hero, should be cached in GameState
             let (w, h) = crate::engine::tile_grid::get_grid_dimensions(&game_state.dungeon.grid);
             let mut pf_grid = crate::engine::pathfinding::PathfindingGrid::new(w, h);
             
             for y in 0..h {
                 for x in 0..w {
                     let pos = TilePos::new(x as i32, y as i32);
                     let mut walkable = false;
                     let mut cost = 1.0;

                     if let Some(tile) = game_state.dungeon.get_tile(pos) {
                         if let Some(td) = game_data.tiles.get(&tile.tile_type) {
                             if !td.blocks_movement {
                                 walkable = true;
                             } else if hero_state.can_dig && td.diggable {
                                 // Tunneler can move through diggable walls with high cost
                                 walkable = true;
                                 cost = 10.0;
                             }
                         } else { 
                            walkable = true; // Default floor
                         }
                     }
                     
                     let pf_pos = crate::engine::pathfinding::Pos::new(x as i32, y as i32);
                     pf_grid.set_walkable(pf_pos, walkable);
                     pf_grid.set_cost(pf_pos, cost);
                 }
             }

             // Calculate path
             let pf_start = crate::engine::pathfinding::Pos::new(hero_entity.pos.x, hero_entity.pos.y);
             let pf_end = crate::engine::pathfinding::Pos::new(target.x, target.y);
             
             let path_result = crate::engine::pathfinding::find_path(
                 pf_start,
                 pf_end,
                 &pf_grid,
                 crate::engine::pathfinding::Heuristic::Manhattan,
                 true   // Allow diagonals
             );
             
             // Convert back
             if let Some(p) = path_result {
                 let waypoints: Vec<TilePos> = p.waypoints.iter()
                     .map(|pos| TilePos::new(pos.x, pos.y))
                     .collect();
                 hero_state.current_path = Some(waypoints);
             }
        }
    }

    // Set fleeing flag based on threat
    let threat_level = evaluate_threat(hero_entity.pos, game_state);
    hero_state.is_fleeing = matches!(threat_level, ThreatLevel::Overwhelming) ||
                           (matches!(threat_level, ThreatLevel::High) && hero_state.should_retreat(0.6));
}