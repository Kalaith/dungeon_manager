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
    // Defenders always stay at spawn - don't change their goal
    if hero_state.is_defender {
        // Ensure defender is in RestAtSpawn mode
        if !matches!(hero_state.current_goal, HeroGoal::RestAtSpawn(_)) {
            hero_state.current_goal = HeroGoal::RestAtSpawn(hero_state.spawn_pos);
            hero_state.target_pos = None;
            hero_state.target_room_id = None;
            hero_state.current_path = None;
        }
    } else {
        // For wave attackers with DestroyHeart goal, don't re-evaluate - they have their mission
        // Only re-evaluate idle heroes (RestAtSpawn with no wave) or heroes that completed their goal
        let should_reevaluate = match &hero_state.current_goal {
            HeroGoal::RestAtSpawn(_) => false, // Stay resting until wave launches
            HeroGoal::DestroyHeart => false,   // Keep attacking - wave assigned this goal
            _ => hero_state.current_path.is_none(), // Other goals: re-evaluate if lost
        };
        
        if should_reevaluate {
            let new_goal = decide_hero_goal(hero_state, game_state, game_data);
            if new_goal != hero_state.current_goal {
                hero_state.current_goal = new_goal;
                hero_state.target_pos = None;
                hero_state.target_room_id = None;
                hero_state.current_path = None;
            }
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
                     // Use abs() to ensure positive modulo result
                     let dx = macroquad::rand::gen_range(-radius, radius + 1);
                     let dy = macroquad::rand::gen_range(-radius, radius + 1);
                     let target_x = spawn_pos.x + dx;
                     let target_y = spawn_pos.y + dy;
                     let target_pos = TilePos::new(target_x, target_y);
                     
                     // Skip if this is the current position (we need to move somewhere DIFFERENT)
                     if target_pos == hero_entity.pos {
                         attempts += 1;
                         continue;
                     }
                     
                     // Check if valid and walkable
                     if let Some(tile) = game_state.dungeon.get_tile(target_pos) {
                         // Check tile data if available, otherwise check if it's a hero building
                         let is_walkable = if let Some(tile_data) = game_data.tiles.get(&tile.tile_type) {
                             !tile_data.blocks_movement
                         } else if tile.tile_type == "hero_wall" || tile.tile_type == "hero_gate" {
                             // Walls and gates block movement
                             false
                         } else if game_data.hero_buildings.get(&tile.tile_type).is_some() {
                             // Other hero building tiles (barracks, etc.) are walkable for heroes
                             true
                         } else {
                             // Default valid if no data (assume floor)
                             true
                         };
                         
                         if is_walkable {

                             found_target = Some(target_pos);
                             break;
                         }
                     } else {

                     }
                     attempts += 1;
                 }
                 
                 // Only update target if we found a valid DIFFERENT position
                 // If we couldn't find one, leave target_pos unchanged and try again next tick
                 if let Some(target) = found_target {
                     hero_state.target_pos = Some(target);
                     // Reset path to force recalculation
                     hero_state.current_path = None;
                 } else {
                 }
             }
        }
        _ => {
            // For goals that target rooms, try to find a target room
            if hero_state.target_room_id.is_none() && hero_state.target_pos.is_none() {
                hero_state.target_room_id = find_target_room(hero_entity.pos, &hero_state.hero_id, &hero_state.current_goal, game_state, game_data);
                
                // Special fallback for DestroyHeart: if no room found, find dungeon heart tile directly
                if hero_state.target_room_id.is_none() && matches!(hero_state.current_goal, HeroGoal::DestroyHeart) {
                    eprintln!("[DEBUG] Hero {} attempting fallback find_dungeon_heart_position...", hero_state.hero_id);
                    if let Some(heart_pos) = game_state.find_dungeon_heart_position() {
                        hero_state.target_pos = Some(heart_pos);
                        eprintln!("[DEBUG] Hero {} targeting dungeon heart directly at {:?}", hero_state.hero_id, heart_pos);
                    } else {
                        eprintln!("[DEBUG] Hero {} FAILED to find dungeon heart tile!", hero_state.hero_id);
                    }
                }
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
                    let idx = macroquad::rand::gen_range(0, room.tiles.len());
                    if let Some(&pos) = room.tiles.iter().nth(idx) {
                        hero_state.target_pos = Some(pos);
                        eprintln!("[DEBUG] Hero {} target resolved to room {} at {:?}", hero_state.hero_id, room_id, pos);
                    }
                 }
             }
        }
    }

    // Check if we arrived at target (and clear it so we can pick a new one)
    if let Some(target) = hero_state.target_pos {
        if target == hero_entity.pos {
            hero_state.target_pos = None;
            hero_state.target_room_id = None; 
            // Also clear path just in case
            hero_state.current_path = None;
        }
    }

    // Pathfinding
    if let Some(target) = hero_state.target_pos {
        // If we have no path or target changed (simplified: just check if no path)
        if hero_state.current_path.is_none() && target != hero_entity.pos {
              if matches!(hero_state.current_goal, HeroGoal::DestroyHeart) {
                  eprintln!("[DEBUG] Hero {} calculating path from {:?} to {:?} (Goal: DestroyHeart)", hero_state.hero_id, hero_entity.pos, target);
              }

             
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
                         // Special overrides for structural tiles that need to be walkable for pathfinding
                         if tile.tile_type == "dungeon_heart" || tile.tile_type == "hero_gate" {
                             walkable = true;
                         } else if let Some(td) = game_data.tiles.get(&tile.tile_type) {
                             if !td.blocks_movement {
                                 walkable = true;
                             } else if hero_state.can_dig && td.diggable {
                                 // Heroes can dig, but NOT their own walls
                                 if tile.tile_type == "hero_wall" {
                                     walkable = false;
                                 } else {
                                     // Tunneler can move through diggable walls with high cost
                                     walkable = true;
                                     cost = 10.0;
                                 }
                             }
                         } else if tile.tile_type == "hero_wall" {
                             // Walls block movement (fallback)
                             walkable = false;
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
                 false   // Allow diagonals
             );
             
            // Convert back
             if let Some(p) = path_result {
                 let waypoints: Vec<TilePos> = p.waypoints.iter()
                     .map(|pos| TilePos::new(pos.x, pos.y))
                     .collect();
                 hero_state.current_path = Some(waypoints);
            } else if hero_state.can_dig && matches!(hero_state.current_goal, HeroGoal::DestroyHeart) {
                 // Fallback: If pathfinding fails (stuck?), pick a random nearby valid tile to wander to.
                 // This breaks the "stuck in corner" loop by changing the immediate goal.
                 
                 eprintln!("[DEBUG] Hero {} pathfinding to Heart failed! Switching to temporary Wander.", hero_state.hero_id);
                 
                 let radius = 3;
                 let mut attempts = 0;
                 let mut wander_target = None;
                 
                 while attempts < 10 {
                     let dx = macroquad::rand::gen_range(-radius, radius + 1);
                     let dy = macroquad::rand::gen_range(-radius, radius + 1);
                     let nx = hero_entity.pos.x + dx;
                     let ny = hero_entity.pos.y + dy;
                     let n_pos = TilePos::new(nx, ny);
                     
                     if n_pos != hero_entity.pos {
                         // Check validity
                         if let Some(tile) = game_state.dungeon.get_tile(n_pos) {
                             let is_walkable = tile.tile_type != "hero_wall"; // Simple check
                             if is_walkable {
                                 wander_target = Some(n_pos);
                                 break;
                             }
                         }
                     }
                     attempts += 1;
                 }
                 
                 if let Some(wt) = wander_target {
                     // Set new temporary target. 
                     // Next frame, pathfinding will run for this new target (which is likely reachable).
                     hero_state.target_pos = Some(wt);
                     hero_state.current_path = None; // clear path to force calc
                 } else {
                     eprintln!("[DEBUG] Hero {} stuck and could not find wander target!", hero_state.hero_id);
                     // Just wait?
                 }
             }
        }
    }

    // Set fleeing flag based on threat
    let threat_level = evaluate_threat(hero_entity.pos, game_state);
    hero_state.is_fleeing = matches!(threat_level, ThreatLevel::Overwhelming) ||
                           (matches!(threat_level, ThreatLevel::High) && hero_state.should_retreat(0.6));
}