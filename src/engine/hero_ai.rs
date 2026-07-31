//! Hero AI system - goal-driven hero behavior
//!
//! Heroes operate on mission priorities and adapt to dungeon threats.
//! This module handles goal selection, threat evaluation, and target finding.

use crate::data::heroes::HeroData;
use crate::data::GameData;
use crate::engine::room_validator::Room;
use crate::state::entities::{Entity, HeroGoal, HeroState};
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

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

/// How much punishment a hero absorbs before their nerve goes, as a fraction
/// of max health.
///
/// `threat_response.retreat_below_health` is the authored baseline and was the
/// only part of the hero personality model the engine read. `bravery` and
/// `fear_resistance` were parsed and ignored, so a peasant militiaman
/// (bravery 30, fear resistance 0.2) broke on exactly the same terms as a
/// champion. They now pull the threshold down together, and
/// `will_fight_to_death` removes it: a hero so authored dies where they stand.
pub fn effective_retreat_threshold(hero_data: &crate::data::HeroData) -> f32 {
    if hero_data.behavior.will_fight_to_death {
        return 0.0;
    }

    // Bravery is authored 0-100, fear resistance 0-1; weigh them equally.
    let nerve = (hero_data.stats.bravery / 100.0).clamp(0.0, 1.0) * 0.5
        + hero_data.behavior.fear_resistance.clamp(0.0, 1.0) * 0.5;

    // Capped at a 60% reduction so even the steadiest hero still has a
    // breaking point — otherwise `will_fight_to_death` would mean nothing.
    hero_data.ai.threat_response.retreat_below_health * (1.0 - nerve * 0.6)
}

/// Decide the current goal for a hero based on their state and dungeon situation
pub fn decide_hero_goal(
    hero_pos: TilePos,
    hero_state: &HeroState,
    game_state: &GameState,
    game_data: &GameData,
) -> HeroGoal {
    let hero_data = match game_data.heroes.get(&hero_state.hero_id) {
        Some(data) => data,
        None => return HeroGoal::DestroyHeart, // fallback
    };

    // Check if hero should retreat
    if hero_state.should_retreat(effective_retreat_threshold(hero_data)) {
        return HeroGoal::Retreat;
    }

    let threat_level = evaluate_threat(hero_pos, game_state);

    // Even overwhelming odds do not move someone who will not break.
    if threat_level == ThreatLevel::Overwhelming && !hero_data.behavior.will_fight_to_death {
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
                choose_secondary_goal(hero_state, hero_data)
            }
        }
        "steal_gold" => {
            let target_gold = calculate_gold_target(hero_data);
            if hero_state.gold_stolen < target_gold {
                HeroGoal::StealGold(target_gold)
            } else {
                choose_secondary_goal(hero_state, hero_data)
            }
        }
        "kill_creatures" => {
            let target_kills = calculate_kill_target(hero_data);
            if hero_state.kills < target_kills {
                HeroGoal::KillCreatures(target_kills)
            } else {
                choose_secondary_goal(hero_state, hero_data)
            }
        }
        "explore" => HeroGoal::Explore,
        _ => HeroGoal::DestroyHeart, // fallback
    }
}

/// Choose a secondary goal when primary is complete
fn choose_secondary_goal(hero_state: &HeroState, hero_data: &HeroData) -> HeroGoal {
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
                if creature.level >= 3
                    || matches!(creature.creature_id.as_str(), "demon_spawn" | "troll")
                {
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
            find_room_with_creatures(hero_pos, &room_types, game_state)
        }
        HeroGoal::SabotageRoom(room_id) => Some(*room_id),
        HeroGoal::Explore => {
            // Find unexplored areas (rooms with fog)
            find_unexplored_room(game_state)
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
            let priority = hero_data
                .ai
                .room_priorities
                .get(&room.room_type)
                .copied()
                .unwrap_or(1.0);
            let distance_factor = calculate_room_distance_factor(hero_pos, room);

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
    room_types: &[&str],
    game_state: &GameState,
) -> Option<usize> {
    let mut best_room = None;
    let mut best_score = 0.0;

    for room in &game_state.room_manager.rooms {
        if room_types.contains(&room.room_type.as_str()) {
            // Score based on room size and distance
            let size_score = room.tiles.len() as f32;
            let distance_factor = calculate_room_distance_factor(hero_pos, room);
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
fn find_unexplored_room(game_state: &GameState) -> Option<usize> {
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
fn calculate_room_distance_factor(hero_pos: TilePos, room: &Room) -> f32 {
    // Find closest tile in room to hero
    let mut min_distance = f32::INFINITY;

    for &tile_pos in &room.tiles {
        let distance = calculate_distance(hero_pos, tile_pos);
        if distance < min_distance {
            min_distance = distance;
        }
    }

    // Convert to factor (closer = smaller factor = higher priority)
    (min_distance / 10.0).clamp(0.5, 3.0)
}

/// Calculate Manhattan distance between two positions
fn calculate_distance(a: TilePos, b: TilePos) -> f32 {
    let dx = (a.x - b.x).abs() as f32;
    let dy = (a.y - b.y).abs() as f32;
    dx + dy // Manhattan distance
}

/// Check if dungeon heart is still alive
fn is_dungeon_heart_alive(game_state: &GameState) -> bool {
    game_state
        .room_manager
        .rooms
        .iter()
        .any(|room| room.room_type == "dungeon_heart")
}

/// Find a valid wander target position for a hero
fn find_wander_target(
    spawn_pos: TilePos,
    current_pos: TilePos,
    game_state: &GameState,
    game_data: &GameData,
) -> Option<TilePos> {
    let radius = 5;
    for _ in 0..10 {
        let dx = macroquad_toolkit::rng::gen_range(-radius, radius + 1);
        let dy = macroquad_toolkit::rng::gen_range(-radius, radius + 1);
        let target_pos = TilePos::new(spawn_pos.x + dx, spawn_pos.y + dy);

        if target_pos == current_pos {
            continue;
        }

        if is_tile_walkable_for_hero(target_pos, game_state, game_data) {
            return Some(target_pos);
        }
    }
    None
}

/// Check if a tile is walkable for heroes
fn is_tile_walkable_for_hero(pos: TilePos, game_state: &GameState, game_data: &GameData) -> bool {
    let tile = match game_state.dungeon.get_tile(pos) {
        Some(t) => t,
        None => return false,
    };

    if let Some(tile_data) = game_data.tiles.get(&tile.tile_type) {
        return !tile_data.blocks_movement;
    }

    if tile.tile_type == "hero_wall" || tile.tile_type == "hero_gate" {
        return false;
    }

    if game_data.hero_buildings.contains_key(&tile.tile_type) {
        return true;
    }

    true
}

// ... existing code ...

/// Update hero AI state based on current goal and situation
pub fn update_hero_ai(
    hero_entity: &Entity,
    hero_state: &mut HeroState,
    game_state: &GameState,
    game_data: &GameData,
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
            let new_goal = decide_hero_goal(hero_entity.pos, hero_state, game_state, game_data);
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
            let needs_new_target =
                hero_state.target_pos.is_none() || hero_state.target_pos == Some(hero_entity.pos);
            if needs_new_target {
                if let Some(target) =
                    find_wander_target(*spawn_pos, hero_entity.pos, game_state, game_data)
                {
                    hero_state.target_pos = Some(target);
                    hero_state.current_path = None;
                }
            }
        }
        HeroGoal::StealGold(_) => {
            // Priority: Visible gold piles -> Treasury room
            let mut found_pile = false;

            // 1. Look for nearby gold piles (visual range)
            if hero_state.target_pos.is_none() {
                if let Some(pile_pos) = find_nearby_gold_pile(hero_entity.pos, game_state) {
                    hero_state.target_pos = Some(pile_pos);
                    hero_state.target_room_id = None; // Ignore room if we see gold
                    found_pile = true;
                }
            }

            // 2. If no pile visible, go to treasury
            if !found_pile && hero_state.target_room_id.is_none() && hero_state.target_pos.is_none()
            {
                hero_state.target_room_id = find_target_room(
                    hero_entity.pos,
                    &hero_state.hero_id,
                    &hero_state.current_goal,
                    game_state,
                    game_data,
                );
            }
        }
        _ => {
            // For goals that target rooms, try to find a target room
            if hero_state.target_room_id.is_none() && hero_state.target_pos.is_none() {
                hero_state.target_room_id = find_target_room(
                    hero_entity.pos,
                    &hero_state.hero_id,
                    &hero_state.current_goal,
                    game_state,
                    game_data,
                );

                // Special fallback for DestroyHeart: if no room found, find dungeon heart tile directly
                if hero_state.target_room_id.is_none()
                    && matches!(hero_state.current_goal, HeroGoal::DestroyHeart)
                {
                    if let Some(heart_pos) = game_state.find_dungeon_heart_position() {
                        hero_state.target_pos = Some(heart_pos);
                    }
                }
            }
        }
    }

    // Resolve Room ID to Target Pos
    if let Some(room_id) = hero_state.target_room_id {
        if hero_state.target_pos.is_none() {
            // Find room directly
            if let Some(room) = game_state
                .room_manager
                .rooms
                .iter()
                .find(|r| r.id == room_id)
            {
                // Pick random tile in room or center
                if !room.tiles.is_empty() {
                    let idx = macroquad_toolkit::rng::gen_range(0, room.tiles.len());
                    if let Some(&pos) = room.tiles.iter().nth(idx) {
                        hero_state.target_pos = Some(pos);
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
    let target = match hero_state.target_pos {
        Some(t) if hero_state.current_path.is_none() && t != hero_entity.pos => t,
        _ => {
            let threat_level = evaluate_threat(hero_entity.pos, game_state);
            hero_state.is_fleeing = matches!(threat_level, ThreatLevel::Overwhelming)
                || (matches!(threat_level, ThreatLevel::High) && hero_state.should_retreat(0.6));
            return;
        }
    };

    let pf_grid = build_hero_pathfinding_grid(game_state, game_data, hero_state.can_dig);
    let pf_start = crate::engine::pathfinding::Pos::new(hero_entity.pos.x, hero_entity.pos.y);
    let pf_end = crate::engine::pathfinding::Pos::new(target.x, target.y);

    let path_result = crate::engine::pathfinding::find_path(
        pf_start,
        pf_end,
        &pf_grid,
        crate::engine::pathfinding::Heuristic::Manhattan,
        false,
    );

    if let Some(p) = path_result {
        let waypoints: Vec<TilePos> = p
            .waypoints
            .iter()
            .map(|pos| TilePos::new(pos.x, pos.y))
            .collect();
        hero_state.current_path = Some(waypoints);
    } else if hero_state.can_dig && matches!(hero_state.current_goal, HeroGoal::DestroyHeart) {
        if let Some(wt) = find_emergency_wander_target(hero_entity.pos, game_state) {
            hero_state.target_pos = Some(wt);
            hero_state.current_path = None;
        }
    }

    let threat_level = evaluate_threat(hero_entity.pos, game_state);
    hero_state.is_fleeing = matches!(threat_level, ThreatLevel::Overwhelming)
        || (matches!(threat_level, ThreatLevel::High) && hero_state.should_retreat(0.6));
}

/// Build a pathfinding grid for hero navigation
fn build_hero_pathfinding_grid(
    game_state: &GameState,
    game_data: &GameData,
    can_dig: bool,
) -> crate::engine::pathfinding::PathfindingGrid {
    let (w, h) = crate::engine::tile_grid::get_grid_dimensions(&game_state.dungeon.grid);
    let mut pf_grid = crate::engine::pathfinding::PathfindingGrid::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let pos = TilePos::new(x as i32, y as i32);
            let (walkable, cost) = get_tile_pathfinding_info(pos, game_state, game_data, can_dig);
            let pf_pos = crate::engine::pathfinding::Pos::new(x as i32, y as i32);
            pf_grid.set_walkable(pf_pos, walkable);
            pf_grid.set_cost(pf_pos, cost);
        }
    }

    pf_grid
}

/// Get pathfinding walkability and cost for a tile
fn get_tile_pathfinding_info(
    pos: TilePos,
    game_state: &GameState,
    game_data: &GameData,
    can_dig: bool,
) -> (bool, f32) {
    let tile = match game_state.dungeon.get_tile(pos) {
        Some(t) => t,
        None => return (false, 1.0),
    };

    if tile.tile_type == "dungeon_heart" || tile.tile_type == "hero_gate" {
        return (true, 1.0);
    }

    if !crate::engine::tile_types::is_tile_walkable(tile, game_data)
        && !can_dig_through_tile(tile, game_data, can_dig)
    {
        return (false, 1.0);
    }

    if let Some(td) = game_data.tiles.get(&tile.tile_type) {
        if !td.blocks_movement {
            return (true, 1.0);
        }
        if can_dig && td.diggable && tile.tile_type != "hero_wall" {
            return (true, 10.0);
        }
        return (false, 1.0);
    }

    if tile.tile_type == "hero_wall" {
        return (false, 1.0);
    }

    (true, 1.0)
}

fn can_dig_through_tile(
    tile: &crate::state::tile_state::TileState,
    game_data: &GameData,
    can_dig: bool,
) -> bool {
    can_dig
        && game_data
            .tiles
            .get(&tile.tile_type)
            .map(|td| td.diggable && tile.tile_type != "hero_wall")
            .unwrap_or(false)
}

/// Find an emergency wander target when pathfinding fails
fn find_emergency_wander_target(current_pos: TilePos, game_state: &GameState) -> Option<TilePos> {
    let radius = 3;
    for _ in 0..10 {
        let dx = macroquad_toolkit::rng::gen_range(-radius, radius + 1);
        let dy = macroquad_toolkit::rng::gen_range(-radius, radius + 1);
        let n_pos = TilePos::new(current_pos.x + dx, current_pos.y + dy);

        if n_pos == current_pos {
            continue;
        }

        if let Some(tile) = game_state.dungeon.get_tile(n_pos) {
            if tile.tile_type != "hero_wall" {
                return Some(n_pos);
            }
        }
    }
    None
}

/// Find nearest visible gold pile
fn find_nearby_gold_pile(hero_pos: TilePos, game_state: &GameState) -> Option<TilePos> {
    let mut best_pos = None;
    let mut min_dist = f32::MAX;
    let vision_radius = 8.0; // Heroes can see gold from this far

    for entity in game_state.entities.all() {
        if let crate::state::entities::EntityType::ResourcePile(pile) = &entity.entity_type {
            if pile.resource_type == "gold" {
                let dist = calculate_distance(hero_pos, entity.pos);
                if dist <= vision_radius && dist < min_dist {
                    min_dist = dist;
                    best_pos = Some(entity.pos);
                }
            }
        }
    }

    best_pos
}
