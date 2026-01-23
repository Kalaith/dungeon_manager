//! Creature AI - Needs-based decision making
//! Stateless service for creature behavior

use crate::data::monsters::MonsterData;
use crate::data::GameData;
use crate::engine::combat;
use crate::engine::pathfinding::{find_path, Heuristic, PathfindingGrid, Pos};
use crate::engine::tile_types;
use crate::state::dungeon::Dungeon;
use crate::state::entities::{CreatureState, EntityId, EntityManager, Task};
use crate::state::room_manager::RoomManager;
use crate::state::tile_state::TilePos;

/// Main entry point: Update all creature AI and movement
/// This replaces the old GameState::update_creature_ai_and_movement method
pub fn update_creatures(
    dungeon: &Dungeon,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    game_data: &GameData,
    dt: f32,
    attack_marker: Option<TilePos>,
    defend_marker: Option<TilePos>,
    get_task_target: impl Fn(&Task, &RoomManager, &EntityManager) -> Option<TilePos>,
) {
    let creature_ids: Vec<EntityId> = entities.creatures()
        .filter(|(_, c)| c.creature_id != "imp") // Imps have their own AI (digging)
        .map(|(id, _)| id)
        .collect();

    for creature_id in creature_ids {
        update_single_creature(
            creature_id,
            dungeon,
            entities,
            room_manager,
            game_data,
            dt,
            attack_marker,
            defend_marker,
            &get_task_target,
        );
    }
}

/// Update AI and movement for a single creature
fn update_single_creature(
    creature_id: EntityId,
    dungeon: &Dungeon,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    game_data: &GameData,
    dt: f32,
    attack_marker: Option<TilePos>,
    defend_marker: Option<TilePos>,
    get_task_target: &impl Fn(&Task, &RoomManager, &EntityManager) -> Option<TilePos>,
) {
    // Get current position and state
    let (current_pos, needs_new_task, current_task, creature_type) = {
        let entity = match entities.get(creature_id) {
            Some(e) => e,
            None => return,
        };

        let creature = match entity.as_creature() {
            Some(c) => c,
            None => return,
        };

        let needs_new_task = creature.current_path.is_none() && creature.current_task.is_none();

        (entity.pos, needs_new_task, creature.current_task.clone(), creature.creature_id.clone())
    };

    // Check if this is a wild/neutral creature
    let is_wild = game_data.monsters.get(&creature_type)
        .map(|m| m.faction != "dungeon")
        .unwrap_or(false);

    // Update needs and mood (only for dungeon creatures, not wild ones)
    if !is_wild {
        if let Some(monster_data) = game_data.monsters.get(&creature_type) {
            if let Some(entity) = entities.get_mut(creature_id) {
                if let Some(creature) = entity.as_creature_mut() {
                    update_needs(creature, dt, monster_data);
                    update_mood(creature, monster_data, game_data);
                }
            }
        }
    }

    // COMBAT INTERRUPT: If not already attacking, check for nearby enemies
    // This ensures creatures always respond to threats regardless of current task
    let current_task = check_combat_interrupt(
        creature_id,
        current_task,
        entities,
        dungeon,
        game_data,
    );

    // Special handling for Attack tasks - combat engagement logic
    if let Some(Task::Attack(target_id)) = &current_task {
        let combat_result = handle_combat_movement(
            creature_id,
            *target_id,
            current_pos,
            dungeon,
            entities,
            game_data,
            dt,
        );

        match combat_result {
            CombatMovementResult::StayAndFight => {
                // At preferred range, don't move - combat resolution handles damage
                return;
            }
            CombatMovementResult::ChaseTarget => {
                // Need to move closer - check if current path leads to target
                // If path is stale (wrong destination), clear it to recalculate
                let target_pos = entities.get(*target_id).map(|e| e.pos);
                if let Some(target_pos) = target_pos {
                    let path_is_stale = {
                        if let Some(entity) = entities.get(creature_id) {
                            if let Some(creature) = entity.as_creature() {
                                if let Some(path) = &creature.current_path {
                                    // Check if path destination is NOT the target's current position
                                    path.last().map(|&dest| dest != target_pos).unwrap_or(true)
                                } else {
                                    false // No path, will be calculated below
                                }
                            } else { false }
                        } else { false }
                    };

                    if path_is_stale {
                        if let Some(entity) = entities.get_mut(creature_id) {
                            if let Some(creature) = entity.as_creature_mut() {
                                creature.current_path = None;
                            }
                        }
                    }
                }
                // Fall through to movement and pathfinding below
            }
            CombatMovementResult::Kite(kite_pos) => {
                // Ranged unit needs to back off - set path to kite position
                if let Some(entity) = entities.get_mut(creature_id) {
                    if let Some(creature) = entity.as_creature_mut() {
                        creature.current_path = Some(vec![kite_pos]);
                    }
                }
                crate::engine::movement::process_entity_movement(entities, creature_id, dt);
                return;
            }
            CombatMovementResult::TargetLost => {
                // Target died or escaped, clear task
                if let Some(entity) = entities.get_mut(creature_id) {
                    if let Some(creature) = entity.as_creature_mut() {
                        creature.current_task = None;
                        creature.current_path = None;
                    }
                }
                return;
            }
        }
    }

    // Decide new task if needed
    if needs_new_task {
        if is_wild {
            // Wild creatures just wander randomly - they don't use dungeon rooms
            if let Some(entity) = entities.get_mut(creature_id) {
                if let Some(creature) = entity.as_creature_mut() {
                    creature.current_task = Some(Task::Idle); // Idle triggers wander
                }
            }
        } else {
            decide_and_assign_task(creature_id, current_pos, entities, dungeon, room_manager, game_data, attack_marker, defend_marker);
        }
    }

    // Pathfind FIRST, then move - this ensures no 1-frame delay when chasing
    let needs_path = {
        let entity = match entities.get(creature_id) {
            Some(e) => e,
            None => return,
        };
        let creature = match entity.as_creature() {
            Some(c) => c,
            None => return,
        };
        creature.current_path.is_none() && creature.current_task.is_some()
    };

    if needs_path {
        let task_target = {
            let entity = entities.get(creature_id).unwrap();
            let creature = entity.as_creature().unwrap();
            creature.current_task.as_ref().and_then(|task| get_task_target(task, room_manager, entities))
        };

        pathfind_to_target(
            creature_id,
            current_pos,
            task_target,
            dungeon,
            entities,
            game_data,
        );
    }

    // Handle movement along path (AFTER pathfinding so we can move immediately)
    crate::engine::movement::process_entity_movement(entities, creature_id, dt);
}

/// Check if creature should interrupt current task to engage in combat
/// Returns the (potentially updated) current task
fn check_combat_interrupt(
    creature_id: EntityId,
    current_task: Option<Task>,
    entities: &mut EntityManager,
    dungeon: &Dungeon,
    game_data: &GameData,
) -> Option<Task> {
    // Already attacking? Don't interrupt
    if matches!(current_task, Some(Task::Attack(_))) {
        return current_task;
    }

    // Check if creature is deserting (fleeing)
    let is_deserting = {
        let entity = match entities.get(creature_id) {
            Some(e) => e,
            None => return current_task,
        };
        let creature = match entity.as_creature() {
            Some(c) => c,
            None => return current_task,
        };
        creature.is_deserting
    };

    if is_deserting {
        return current_task; // Don't interrupt fleeing
    }

    // Look for nearby enemies
    let entity = match entities.get(creature_id) {
        Some(e) => e,
        None => return current_task,
    };

    let targets = combat::find_combat_targets(entity, entities.entities(), dungeon, game_data);

    if let Some(&target_id) = targets.first() {
        // Found an enemy! Switch to attack mode
        if let Some(entity) = entities.get_mut(creature_id) {
            if let Some(creature) = entity.as_creature_mut() {
                creature.current_task = Some(Task::Attack(target_id));
                creature.current_path = None; // Clear old path, will recalculate
            }
        }
        return Some(Task::Attack(target_id));
    }

    current_task
}

/// Result of combat movement decision
enum CombatMovementResult {
    /// At preferred range, stay and fight
    StayAndFight,
    /// Need to move closer to target
    ChaseTarget,
    /// Need to kite (move to specified position)
    Kite(TilePos),
    /// Target is dead or out of range, clear task
    TargetLost,
}

/// Handle combat movement - decide whether to stay, chase, or kite
fn handle_combat_movement(
    creature_id: EntityId,
    target_id: EntityId,
    current_pos: TilePos,
    dungeon: &Dungeon,
    entities: &mut EntityManager,
    game_data: &GameData,
    _dt: f32,
) -> CombatMovementResult {
    // Get target position and check if target is still valid
    let target_pos = match entities.get(target_id) {
        Some(target) if target.is_alive() => target.pos,
        _ => return CombatMovementResult::TargetLost,
    };

    // Get attacker's attack type and detection range
    let (attack_type, detection_range) = {
        let entity = match entities.get(creature_id) {
            Some(e) => e,
            None => return CombatMovementResult::TargetLost,
        };
        (
            combat::get_entity_attack_type(entity, game_data),
            combat::get_detection_range(entity, game_data),
        )
    };

    let distance = combat::manhattan_distance(current_pos, target_pos);

    // Check if target is too far away (escaped beyond sight range)
    if distance > detection_range {
        return CombatMovementResult::TargetLost;
    }

    // Check if in attack range
    let in_range = combat::in_combat_range(current_pos, target_pos, &attack_type, &dungeon.grid, game_data);

    match attack_type.as_str() {
        "melee" => {
            if in_range && distance <= 1 {
                // Adjacent to target - stay and fight
                // Clear path so we don't keep moving
                if let Some(entity) = entities.get_mut(creature_id) {
                    if let Some(creature) = entity.as_creature_mut() {
                        creature.current_path = None;
                    }
                }
                CombatMovementResult::StayAndFight
            } else {
                // Need to get closer
                CombatMovementResult::ChaseTarget
            }
        }
        "ranged" | "magic" => {
            if distance <= 1 {
                // Too close! Try to kite
                if let Some(kite_pos) = find_kite_position(current_pos, target_pos, dungeon, game_data) {
                    CombatMovementResult::Kite(kite_pos)
                } else {
                    // Can't kite, just fight from here
                    if let Some(entity) = entities.get_mut(creature_id) {
                        if let Some(creature) = entity.as_creature_mut() {
                            creature.current_path = None;
                        }
                    }
                    CombatMovementResult::StayAndFight
                }
            } else if in_range {
                // Good range - stay and attack
                if let Some(entity) = entities.get_mut(creature_id) {
                    if let Some(creature) = entity.as_creature_mut() {
                        creature.current_path = None;
                    }
                }
                CombatMovementResult::StayAndFight
            } else {
                // Out of range, need to get closer
                CombatMovementResult::ChaseTarget
            }
        }
        _ => {
            // Default to melee behavior
            if in_range {
                if let Some(entity) = entities.get_mut(creature_id) {
                    if let Some(creature) = entity.as_creature_mut() {
                        creature.current_path = None;
                    }
                }
                CombatMovementResult::StayAndFight
            } else {
                CombatMovementResult::ChaseTarget
            }
        }
    }
}

/// Find a position to kite to (move away from melee attacker)
fn find_kite_position(
    current_pos: TilePos,
    threat_pos: TilePos,
    dungeon: &Dungeon,
    game_data: &GameData,
) -> Option<TilePos> {
    // Calculate direction away from threat
    let dx = current_pos.x - threat_pos.x;
    let dy = current_pos.y - threat_pos.y;

    // Try to move in the opposite direction
    let directions = if dx.abs() > dy.abs() {
        // Primarily horizontal movement
        if dx > 0 {
            vec![(1, 0), (1, 1), (1, -1), (0, 1), (0, -1)]
        } else {
            vec![(-1, 0), (-1, 1), (-1, -1), (0, 1), (0, -1)]
        }
    } else {
        // Primarily vertical movement
        if dy > 0 {
            vec![(0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0)]
        } else {
            vec![(0, -1), (1, -1), (-1, -1), (1, 0), (-1, 0)]
        }
    };

    for (move_dx, move_dy) in directions {
        let candidate = TilePos::new(current_pos.x + move_dx, current_pos.y + move_dy);

        // Check if walkable
        if let Some(tile) = dungeon.get_tile(candidate) {
            if tile_types::is_walkable(&tile.tile_type, game_data) {
                // Make sure we're actually moving away
                let new_dist = combat::manhattan_distance(candidate, threat_pos);
                let old_dist = combat::manhattan_distance(current_pos, threat_pos);
                if new_dist > old_dist {
                    return Some(candidate);
                }
            }
        }
    }

    // If we can't move away, try any walkable adjacent tile
    for (move_dx, move_dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let candidate = TilePos::new(current_pos.x + move_dx, current_pos.y + move_dy);
        if candidate == threat_pos {
            continue; // Don't move toward threat
        }
        if let Some(tile) = dungeon.get_tile(candidate) {
            if tile_types::is_walkable(&tile.tile_type, game_data) {
                return Some(candidate);
            }
        }
    }

    None
}



// process_creature_movement removed in favor of shared movement::process_entity_movement

/// Decide and assign a new task to a creature
fn decide_and_assign_task(
    creature_id: EntityId,
    current_pos: TilePos,
    entities: &mut EntityManager,
    dungeon: &Dungeon,
    room_manager: &RoomManager,
    game_data: &GameData,
    attack_marker: Option<TilePos>,
    defend_marker: Option<TilePos>,
) {
    // Get creature data for decision
    let new_task = {
        let entity = match entities.get(creature_id) {
            Some(e) => e,
            None => return,
        };
        let creature = match entity.as_creature() {
            Some(c) => c,
            None => return,
        };

        // Create a temporary GameState-like access for decide_task
        // For now, we'll pass room_manager directly since that's what decide_task needs
        // Create a temporary GameState-like access for decide_task
        // For now, we'll pass room_manager directly since that's what decide_task needs
        decide_task_from_rooms(creature, entity, entities, dungeon, current_pos, room_manager, game_data, attack_marker, defend_marker)
    };

    // Apply the new task
    if let Some(entity) = entities.get_mut(creature_id) {
        if let Some(creature) = entity.as_creature_mut() {
            if new_task.is_some() {
                // eprintln!("[AI] Creature {} decided new task: {:?}", creature.creature_id, new_task);
            }
            creature.current_task = new_task;
        }
    }
}

/// Simplified decide_task that works with RoomManager directly
fn decide_task_from_rooms(
    creature: &CreatureState,
    entity: &crate::state::entities::Entity,
    entities: &EntityManager,
    dungeon: &Dungeon,
    creature_pos: TilePos,
    room_manager: &RoomManager,
    game_data: &GameData,
    attack_marker: Option<TilePos>,
    defend_marker: Option<TilePos>,
) -> Option<Task> {
    let monster_data = match game_data.monsters.get(&creature.creature_id) {
        Some(data) => data,
        None => return Some(Task::Idle),
    };

    // Priority 1: Check Global Markers (Override needs unless critical)
    // Attack Marker
    let marker_dist_threshold = game_data.config.creature_ai.marker_distance_threshold;
    if let Some(marker) = attack_marker {
        // Calculate distance via Manhattan for quick check
        let dist = (creature_pos.x - marker.x).abs() + (creature_pos.y - marker.y).abs();

        // If not at marker, move there
        if dist > marker_dist_threshold {
             return Some(Task::MoveTo(marker));
        }
    }

    // Defend Marker
    if let Some(marker) = defend_marker {
        let dist = (creature_pos.x - marker.x).abs() + (creature_pos.y - marker.y).abs();
        if dist > marker_dist_threshold {
             return Some(Task::MoveTo(marker));
        }
    }

    // Priority 2: Combat - Look for enemies
    // Only attack if not fleeing
    if !creature.is_deserting {
        let targets = crate::engine::combat::find_combat_targets(entity, entities.entities(), dungeon, game_data);
        if let Some(target_id) = targets.first() {
            // Found a target!
            return Some(Task::Attack(*target_id));
        }
    }

    // If fleeing or deserting, flee
    if creature.is_deserting {
        return Some(Task::Flee);
    }

    // If carrying gold, prioritize depositing
    if creature.gold_carried > game_data.config.creature_ai.gold_carrying_threshold {
        use crate::engine::room_validator;
        if let Some((room_id, _)) = room_validator::find_nearest_room(&room_manager.rooms, "treasury", creature_pos, 0.0) {
            return Some(Task::DepositGold(room_id));
        }
    }

    // Check most urgent need
    if let Some(task) = crate::engine::creature_task_logic::try_satisfy_critical_need(creature, creature_pos, room_manager, monster_data, game_data) {
        return Some(task);
    }

    // Evaluate all possible tasks
    let mut candidate_tasks = Vec::new();

    for (room_type, desire) in &monster_data.ai.room_desires {
        use crate::engine::room_validator;
        if let Some((room_id, _)) = room_validator::find_nearest_room(&room_manager.rooms, room_type, creature_pos, 0.0) {
            // Find valid slot
            if let Some(room) = room_manager.rooms.iter().find(|r| r.id == room_id) {
                if let Some(slot_pos) = find_available_work_slot(room, entities) {
                    let task = Task::Work(room_id, slot_pos);
                    let desirability = crate::engine::creature_task_logic::calculate_task_desirability(&task, creature, monster_data, game_data) * desire;
                    candidate_tasks.push((task, desirability));
                }
            }
        }
    }

    // Add training if available
    if creature.mood > game_data.config.creature_ai.training_mood_threshold && creature.level < game_data.config.combat.max_creature_level {
        use crate::engine::room_validator;
        if let Some((room_id, _)) = room_validator::find_nearest_room(&room_manager.rooms, "training_room", creature_pos, 0.0) {
            let task = Task::Train(room_id);
            let desirability = crate::engine::creature_task_logic::calculate_task_desirability(&task, creature, monster_data, game_data);
            candidate_tasks.push((task, desirability));
        }
    }

    // Add research
    use crate::engine::room_validator;
    if let Some((room_id, _)) = room_validator::find_nearest_room(&room_manager.rooms, "library", creature_pos, 0.0) {
        let task = Task::Research(room_id);
        let desirability = crate::engine::creature_task_logic::calculate_task_desirability(&task, creature, monster_data, game_data) * game_data.config.creature_ai.research_desirability;
        candidate_tasks.push((task, desirability));
    }

    // Select best task
    if let Some((task, _)) = candidate_tasks
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    {
        Some(task)
    } else {
        Some(Task::Idle)
    }
}

/// Pathfind creature to target position
fn pathfind_to_target(
    creature_id: EntityId,
    current_pos: TilePos,
    mut target_pos: Option<TilePos>,
    dungeon: &Dungeon,
    entities: &mut EntityManager,
    game_data: &GameData,
) {
    // Special case for Idle: pick a random wander position
    if target_pos.is_none() {
        let is_idle = {
            let entity = match entities.get(creature_id) {
                Some(e) => e,
                None => return,
            };
            let creature = match entity.as_creature() {
                Some(c) => c,
                None => return,
            };
            matches!(creature.current_task, Some(Task::Idle))
        };

        if is_idle {
            target_pos = pick_wander_position(dungeon, current_pos, game_data);
        }
    }

    let target = match target_pos {
        Some(t) => t,
        None => return,
    };

    if current_pos == target {
        return; // Already at target
    }

    // Create pathfinding grid
    let mut pf_grid = PathfindingGrid::new(dungeon.width, dungeon.height);

    // Mark walkable tiles
    for y in 0..dungeon.height {
        for x in 0..dungeon.width {
            let tile_pos = TilePos::new(x as i32, y as i32);
            if let Some(tile) = dungeon.get_tile(tile_pos) {
                let walkable = tile_types::is_walkable(&tile.tile_type, game_data);
                pf_grid.set_walkable(Pos::new(x as i32, y as i32), walkable);
            }
        }
    }

    // Find path
    let start = Pos::new(current_pos.x, current_pos.y);
    let goal = Pos::new(target.x, target.y);

    let mut path_result = find_path(start, goal, &pf_grid, Heuristic::Manhattan, false);

    // Fallback: If path to goal failed and goal is unwalkable (e.g. wall/building), 
    // try to path to an adjacent walkable tile.
    if path_result.is_none() && !pf_grid.is_walkable(goal) {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut neighbors = Vec::new();
        
        for (dx, dy) in directions.iter() {
            let n = Pos::new(target.x + dx, target.y + dy);
            // Check bounds
            if n.x >= 0 && n.x < dungeon.width as i32 && n.y >= 0 && n.y < dungeon.height as i32 {
                if pf_grid.is_walkable(n) {
                    neighbors.push(n);
                }
            }
        }

        // Sort by distance to start to pick "closest" approach
        neighbors.sort_by_key(|n| (n.x - start.x).abs() + (n.y - start.y).abs());

        for n in neighbors {
            if let Some(p) = find_path(start, n, &pf_grid, Heuristic::Manhattan, false) {
                path_result = Some(p);
                break;
            }
        }
    }

    if let Some(path) = path_result {
        if let Some(entity) = entities.get_mut(creature_id) {
            if let Some(creature) = entity.as_creature_mut() {
                creature.current_path = Some(
                    path.waypoints
                        .iter()
                        .map(|p| TilePos::new(p.x, p.y))
                        .collect(),
                );
            }
        }
    }
}

/// Try to satisfy a critical need by finding an appropriate room


/// Pick a random walkable tile for wandering
fn pick_wander_position(dungeon: &Dungeon, current_pos: TilePos, game_data: &GameData) -> Option<TilePos> {
    let wander_radius = game_data.config.creature_ai.wander_radius;
    let wander_attempts = game_data.config.creature_ai.wander_attempts;
    let mut attempts = 0;

    while attempts < wander_attempts {
        let dx = macroquad::rand::gen_range(-wander_radius, wander_radius + 1);
        let dy = macroquad::rand::gen_range(-wander_radius, wander_radius + 1);
        let candidate = TilePos::new(current_pos.x + dx, current_pos.y + dy);

        if let Some(tile) = dungeon.get_tile(candidate) {
            if tile_types::is_walkable(&tile.tile_type, game_data) {
                return Some(candidate);
            }
        }
        attempts += 1;
    }
    None
}

/// Find an available work slot in a room
/// Checks all entities to see if they are targeting any of the room's slots
fn find_available_work_slot(
    room: &crate::engine::room_validator::Room,
    entities: &EntityManager,
) -> Option<TilePos> {
    // Collect occupied slots
    let mut occupied_slots = std::collections::HashSet::new();
    
    // Check all creatures for Work tasks targeting this room
    for (_, creature) in entities.creatures() {
        if let Some(Task::Work(target_room_id, target_pos)) = &creature.current_task {
            if *target_room_id == room.id {
                occupied_slots.insert(*target_pos);
            }
        }
    }
    
    // Return first slot that isn't occupied
    // We prefer slots closer to center? Or random? Or just first available.
    // room.work_slots are likely sorted or deterministic.
    for slot in &room.work_slots {
        if !occupied_slots.contains(slot) {
            return Some(*slot);
        }
    }
    
    None
}

/// Update all needs for a creature based on time passed
pub fn update_needs(creature: &mut CreatureState, dt: f32, monster_data: &MonsterData) {
    let decay_per_second = dt / 60.0; // Convert to per-second from per-minute

    // Decay each need based on monster data
    for (need_name, need_data) in &monster_data.needs {
        let current = creature.get_need(need_name);
        let decay = need_data.decay_per_minute * decay_per_second;
        creature.set_need(need_name.clone(), current - decay);
    }
}

/// Calculate creature mood based on needs satisfaction
pub fn calculate_mood(creature: &CreatureState, monster_data: &MonsterData, game_data: &GameData) -> f32 {
    let base_mood = monster_data.ai.base_mood as f32;
    let mood_penalties = &game_data.config.creature_ai.mood_penalties;

    // Start from base mood
    let mut mood = base_mood;

    // Average all needs (0-100 each)
    let need_count = creature.needs.len() as f32;
    if need_count > 0.0 {
        let total_satisfaction: f32 = creature.needs.values().sum();
        let average_satisfaction = total_satisfaction / need_count;

        // Needs contribute ±30 points to mood
        // 100% satisfied = +30, 0% satisfied = -30
        let need_modifier = (average_satisfaction - 50.0) * game_data.config.creature_ai.task_desirability.need_modifier;
        mood += need_modifier;
    }

    // Health affects mood
    let health_percent = (creature.health / creature.max_health) * 100.0;
    if health_percent < mood_penalties.low_health_threshold {
        mood -= mood_penalties.low_health_penalty;
    }

    // Being angry reduces mood
    if creature.is_angry {
        mood -= mood_penalties.angry_penalty;
    }

    // Clamp to 0-100
    mood.clamp(0.0, 100.0)
}

/// Update creature mood
pub fn update_mood(creature: &mut CreatureState, monster_data: &MonsterData, game_data: &GameData) {
    creature.mood = calculate_mood(creature, monster_data, game_data);

    // Update angry state
    let anger_threshold = monster_data.ai.anger_threshold as f32;
    creature.is_angry = creature.mood < anger_threshold;

    // Update deserting state
    let desertion_threshold = monster_data.ai.desertion_threshold as f32;
    creature.is_deserting = creature.mood < desertion_threshold;
}

/// Check if creature should desert the dungeon


/// Find the best room of a specific type for a creature


/// Calculate desirability of a task for a creature


/// Satisfy a need when creature is in appropriate room
pub fn satisfy_need(
    creature: &mut CreatureState,
    need_name: &str,
    rate: f32,
    dt: f32,
) {
    let current = creature.get_need(need_name);
    let increase = rate * dt;
    creature.set_need(need_name.to_string(), current + increase);
}

/// Handle slapping a creature (discipline)
pub fn apply_slap(
    creature: &mut CreatureState,
    monster_data: &MonsterData,
    game_time: f32,
) {
    // Prevent slap spamming (minimum 5 second cooldown)
    if game_time - creature.last_slapped < 5.0 {
        return;
    }

    creature.last_slapped = game_time;

    // Apply mood change from discipline response
    if let Some(&mood_change) = monster_data.ai.discipline_response.get("slap") {
        creature.mood = (creature.mood + mood_change).clamp(0.0, 100.0);
    }

    // Slapping interrupts current task
    creature.current_task = None;
    creature.task_time = 0.0;
}

/// Calculate work efficiency based on creature stats and mood
pub fn calculate_work_efficiency(creature: &CreatureState, _monster_data: &MonsterData) -> f32 {
    let base_efficiency = 1.0;

    // Mood affects efficiency (50% to 150%)
    let mood_multiplier = 0.5 + (creature.mood / 100.0);

    // Health affects efficiency when low
    let health_percent = creature.health / creature.max_health;
    let health_multiplier = if health_percent < 0.5 {
        0.5 + health_percent
    } else {
        1.0
    };

    base_efficiency * mood_multiplier * health_multiplier
}


