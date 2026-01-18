//! Creature AI - Needs-based decision making
//! Stateless service for creature behavior

use crate::data::monsters::MonsterData;
use crate::data::GameData;
use crate::engine::pathfinding::{find_path, Heuristic, PathfindingGrid, Pos};
use crate::engine::tile_types;
use crate::state::dungeon::Dungeon;
use crate::state::entities::{CreatureState, EntityId, EntityManager, Task};
use crate::state::game_state::GameState;
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
    get_task_target: impl Fn(&Task, &RoomManager) -> Option<TilePos>,
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
    get_task_target: &impl Fn(&Task, &RoomManager) -> Option<TilePos>,
) {
    // Get current position and state
    let (current_pos, needs_new_task, needs_path, task_target, creature_type) = {
        let entity = match entities.get(creature_id) {
            Some(e) => e,
            None => return,
        };

        let creature = match entity.as_creature() {
            Some(c) => c,
            None => return,
        };

        let needs_new_task = creature.current_path.is_none() && creature.current_task.is_none();
        let needs_path = creature.current_path.is_none() && creature.current_task.is_some();
        let task_target = creature.current_task.as_ref().and_then(|task| get_task_target(task, room_manager));

        (entity.pos, needs_new_task, needs_path, task_target, creature.creature_id.clone())
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

    // Handle movement along path
    crate::engine::movement::process_entity_movement(entities, creature_id, dt);

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
            decide_and_assign_task(creature_id, current_pos, entities, room_manager, game_data, attack_marker, defend_marker);
        }
    }

    // Pathfind to task if needed
    if needs_path {
        pathfind_to_target(
            creature_id,
            current_pos,
            task_target,
            dungeon,
            entities,
            game_data,
        );
    }
}



// process_creature_movement removed in favor of shared movement::process_entity_movement

/// Decide and assign a new task to a creature
fn decide_and_assign_task(
    creature_id: EntityId,
    current_pos: TilePos,
    entities: &mut EntityManager,
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
        decide_task_from_rooms(creature, current_pos, room_manager, game_data, attack_marker, defend_marker)
    };

    // Apply the new task
    if let Some(entity) = entities.get_mut(creature_id) {
        if let Some(creature) = entity.as_creature_mut() {
            if new_task.is_some() {
                eprintln!("[AI] Creature {} decided new task: {:?}", creature.creature_id, new_task);
            }
            creature.current_task = new_task;
        }
    }
}

/// Simplified decide_task that works with RoomManager directly
fn decide_task_from_rooms(
    creature: &CreatureState,
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
        // If at marker, we stay/patrol/attack automatically via combat system + idle wander
        // But to ensure they don't leave, we can keep returning MoveTo if they wander too far?
        // Or just let them Idle (wander) nearby.
        // For now: MoveTo ensures they gather.
    }

    // Defend Marker
    if let Some(marker) = defend_marker {
        let dist = (creature_pos.x - marker.x).abs() + (creature_pos.y - marker.y).abs();
        if dist > marker_dist_threshold {
             return Some(Task::MoveTo(marker));
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
    if let Some(task) = try_satisfy_critical_need(creature, creature_pos, room_manager, monster_data, game_data) {
        return Some(task);
    }

    // Evaluate all possible tasks
    let mut candidate_tasks = Vec::new();

    for (room_type, desire) in &monster_data.ai.room_desires {
        use crate::engine::room_validator;
        if let Some((room_id, _)) = room_validator::find_nearest_room(&room_manager.rooms, room_type, creature_pos, 0.0) {
            let task = Task::Work(room_id);
            let desirability = calculate_task_desirability(&task, creature, monster_data, game_data) * desire;
            candidate_tasks.push((task, desirability));
        }
    }

    // Add training if available
    if creature.mood > game_data.config.creature_ai.training_mood_threshold && creature.level < game_data.config.combat.max_creature_level {
        use crate::engine::room_validator;
        if let Some((room_id, _)) = room_validator::find_nearest_room(&room_manager.rooms, "training_room", creature_pos, 0.0) {
            let task = Task::Train(room_id);
            let desirability = calculate_task_desirability(&task, creature, monster_data, game_data);
            candidate_tasks.push((task, desirability));
        }
    }

    // Add research
    use crate::engine::room_validator;
    if let Some((room_id, _)) = room_validator::find_nearest_room(&room_manager.rooms, "library", creature_pos, 0.0) {
        let task = Task::Research(room_id);
        let desirability = calculate_task_desirability(&task, creature, monster_data, game_data) * game_data.config.creature_ai.research_desirability;
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

    if let Some(path) = find_path(start, goal, &pf_grid, Heuristic::Manhattan, false) {
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
fn try_satisfy_critical_need(creature: &CreatureState, creature_pos: TilePos, room_manager: &RoomManager, monster_data: &MonsterData, game_data: &GameData) -> Option<Task> {
    let (need_name, need_value) = creature.get_most_urgent_need()?;

    if need_value >= game_data.config.creature_ai.need_critical_threshold {
        return None;
    }

    let need_data = monster_data.needs.get(&need_name)?;

    for room_type in &need_data.satisfied_by {
        use crate::engine::room_validator;
        let (room_id, _) = room_validator::find_nearest_room(&room_manager.rooms, room_type, creature_pos, 0.0)?;

        return match need_name.as_str() {
            "sleep" => Some(Task::Sleep(room_id)),
            "food" => Some(Task::Eat(room_id)),
            "gold" => Some(Task::CollectWages(room_id)),
            _ => None,
        };
    }

    None
}

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
pub fn should_desert(creature: &CreatureState, monster_data: &MonsterData) -> bool {
    let desertion_threshold = monster_data.ai.desertion_threshold as f32;
    creature.mood < desertion_threshold
}

/// Find the best room of a specific type for a creature


/// Calculate desirability of a task for a creature
pub fn calculate_task_desirability(
    task: &Task,
    creature: &CreatureState,
    monster_data: &MonsterData,
    game_data: &GameData,
) -> f32 {
    let task_type = task.task_type();
    let task_config = &game_data.config.creature_ai.task_desirability;
    let mut desirability = task_config.base;

    // Apply task preference from monster data
    if let Some(&preference) = monster_data.ai.task_preferences.get(task_type) {
        desirability *= preference;
    }

    // Boost desirability based on related needs
    match task {
        Task::Sleep(_) => {
            let sleep_need = 100.0 - creature.get_need("sleep");
            desirability *= 1.0 + (sleep_need / 100.0);
        }
        Task::Eat(_) => {
            let food_need = 100.0 - creature.get_need("food");
            desirability *= 1.0 + (food_need / 100.0);
        }
        Task::DepositGold(_) => {
            if creature.gold_carried > 0 {
                let gold_need = 100.0 - creature.get_need("gold");
                desirability *= task_config.gold_deposit + (gold_need / 100.0);
            } else {
                desirability *= task_config.skip_deposit; // Don't deposit if no gold
            }
        }
        Task::Train(_) => {
            // Training is more desirable when satisfied (not urgent need)
            let avg_satisfaction = if !creature.needs.is_empty() {
                creature.needs.values().sum::<f32>() / creature.needs.len() as f32
            } else {
                50.0
            };
            if avg_satisfaction > task_config.satisfaction_threshold {
                desirability *= task_config.training_high_satisfaction;
            } else {
                desirability *= task_config.training_low_satisfaction;
            }
        }
        Task::CollectWages(_) => {
            let gold_need = 100.0 - creature.get_need("gold");
            desirability *= task_config.wage_collection + (gold_need / 100.0);
        }
        _ => {}
    }

    desirability
}

/// Decide what task a creature should do next
pub fn decide_task(
    creature: &CreatureState,
    creature_pos: TilePos,
    game_state: &GameState,
    game_data: &GameData,
) -> Option<Task> {
    // Get monster data
    let monster_data = match game_data.monsters.get(&creature.creature_id) {
        Some(data) => data,
        None => {
            eprintln!("[AI] No monster data for {}", creature.creature_id);
            return Some(Task::Idle);
        }
    };

    // Debug: Log available rooms (Reduced)
    // eprintln!("[AI] Available rooms for {}: {:?}",
    //    creature.creature_id,
    //    game_state.room_manager.rooms.iter()
    //        .map(|r| format!("{}(id={}, active={}, quality={:.1})", r.room_type, r.id, r.active, r.quality))
    //        .collect::<Vec<_>>()
    // );

    // If fleeing or deserting, flee
    if creature.is_deserting {
        eprintln!("[AI] Creature {} is deserting!", creature.creature_id);
        return Some(Task::Flee);
    }

    // If carrying gold, prioritize depositing
    if creature.gold_carried > game_data.config.creature_ai.gold_carrying_threshold {
        use crate::engine::room_validator;
        if let Some((room_id, _)) = room_validator::find_nearest_room(&game_state.room_manager.rooms, "treasury", creature_pos, 0.0) {
            return Some(Task::DepositGold(room_id));
        }
    }

    // Check most urgent need
    if let Some((need_name, need_value)) = creature.get_most_urgent_need() {
        // eprintln!("[AI] Most urgent need for {}: {} = {:.1}%", creature.creature_id, need_name, need_value);

        // If need is critical, prioritize satisfying it
        if need_value < game_data.config.creature_ai.need_critical_threshold {
            eprintln!("[AI] Critical need {} detected!", need_name);
            // Find room that satisfies this need
            if let Some(need_data) = monster_data.needs.get(&need_name) {
                // eprintln!("[AI] Need {} satisfied by rooms: {:?}", need_name, need_data.satisfied_by);
                for room_type in &need_data.satisfied_by {
                    use crate::engine::room_validator;
                    if let Some((room_id, _)) =
                        room_validator::find_nearest_room(&game_state.room_manager.rooms, room_type, creature_pos, 0.0)
                    {
                        // eprintln!("[AI] Found {} room (id={})", room_type, room_id);
                        // Determine task type based on need
                        match need_name.as_str() {
                            "sleep" => return Some(Task::Sleep(room_id)),
                            "food" => return Some(Task::Eat(room_id)),
                            "gold" => return Some(Task::CollectWages(room_id)),
                            _ => {}
                        }
                    } else {
                        eprintln!("[AI] No {} room found!", room_type);
                    }
                }
            }
        }
    }

    // Evaluate all possible tasks
    let mut candidate_tasks = Vec::new();

    // Add work tasks for preferred rooms
    // eprintln!("[AI] Evaluating room desires: {:?}", monster_data.ai.room_desires);
    for (room_type, desire) in &monster_data.ai.room_desires {
        use crate::engine::room_validator;
        if let Some((room_id, _)) = room_validator::find_nearest_room(&game_state.room_manager.rooms, room_type, creature_pos, 0.0) {
            let task = Task::Work(room_id);
            let mut desirability = calculate_task_desirability(&task, creature, monster_data, game_data) * desire;

            // Adjust based on room factors
            if let Some(room) = game_state.room_manager.rooms.iter().find(|r| r.id == room_id) {
                 if let Some(room_data) = game_data.rooms.get(room_type) {
                      let room_factor = room_validator::calculate_task_desirability(room, room_data, creature_pos, 1.0);
                      desirability *= room_factor;
                 }
            }

            // eprintln!("[AI] Candidate: Work({}) - desirability={:.2}", room_type, desirability);
            candidate_tasks.push((task, desirability));
        } else {
            // eprintln!("[AI] No {} room found for work task", room_type);
        }
    }

    // Add training if available and mood is good
    if creature.mood > game_data.config.creature_ai.training_mood_threshold && creature.level < game_data.config.combat.max_creature_level {
        use crate::engine::room_validator;
        if let Some((room_id, _)) = room_validator::find_nearest_room(&game_state.room_manager.rooms, "training_room", creature_pos, 0.0)
        {
            let task = Task::Train(room_id);
            let desirability = calculate_task_desirability(&task, creature, monster_data, game_data);
            // eprintln!("[AI] Candidate: Train - desirability={:.2}", desirability);
            candidate_tasks.push((task, desirability));
        }
    }

    // Add research if creature can research
    use crate::engine::room_validator;
    if let Some((room_id, _)) = room_validator::find_nearest_room(&game_state.room_manager.rooms, "library", creature_pos, 0.0) {
        let task = Task::Research(room_id);
        let desirability = calculate_task_desirability(&task, creature, monster_data, game_data) * game_data.config.creature_ai.research_desirability;
        // eprintln!("[AI] Candidate: Research - desirability={:.2}", desirability);
        candidate_tasks.push((task, desirability));
    }

    // eprintln!("[AI] Total candidate tasks: {}", candidate_tasks.len());

    // Select best task
    if let Some((task, desirability)) = candidate_tasks
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    {
        eprintln!("[AI] Selected task: {:?} with desirability {:.2}", task, desirability);
        Some(task)
    } else {
        // No good tasks, just idle/wander
        eprintln!("[AI] No candidate tasks, idling");
        Some(Task::Idle)
    }
}

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

/// Check if creature can perform a specific task
pub fn can_perform_task(
    creature: &CreatureState,
    task: &Task,
    game_state: &GameState,
) -> bool {
    match task {
        Task::Idle | Task::Flee => true,

        Task::Dig(_) => {
            // Would check if creature is an imp or can dig
            creature.creature_id == "imp"
        }

        Task::Sleep(room_id)
        | Task::Eat(room_id)
        | Task::Work(room_id)
        | Task::Train(room_id)
        | Task::Research(room_id)
        | Task::DepositGold(room_id)
        | Task::CollectWages(room_id) => {
            // Check if room exists and is active
            game_state
                .room_manager.rooms
                .iter()
                .any(|r| r.id == *room_id && r.active)
        }

        Task::MoveTo(_) => true,

        Task::Attack(_) => {
            // Check if creature can fight
            creature.health > 0.0
        }
    }
}

/// Get all creatures that need attention (low mood, deserting, etc.)
pub fn get_creatures_needing_attention(
    game_state: &GameState,
    game_data: &GameData,
) -> Vec<(usize, String)> {
    let mut result = Vec::new();

    for (entity_id, creature) in game_state
        .entities
        .all()
        .filter_map(|e| e.as_creature().map(|c| (e.id, c)))
    {
        if let Some(monster_data) = game_data.monsters.get(&creature.creature_id) {
            if creature.is_deserting {
                result.push((entity_id, "deserting".to_string()));
            } else if creature.is_angry {
                result.push((entity_id, "angry".to_string()));
            } else if creature.mood < game_data.config.creature_ai.mood_attention_threshold {
                result.push((entity_id, "unhappy".to_string()));
            }

            // Check critical needs
            if let Some((need_name, need_value)) = creature.get_most_urgent_need() {
                if need_value < game_data.config.creature_ai.need_attention_threshold {
                    result.push((entity_id, format!("critical_{}", need_name)));
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::monsters::{MonsterAIData, NeedData};
    use std::collections::HashMap;

    fn create_test_monster_data() -> MonsterData {
        let mut needs = HashMap::new();
        let sleep_need = NeedData {
            decay_per_minute: 1.0,
            satisfied_by: vec!["lair".to_string()],
            stash_amount: None,
        };
        needs.insert("sleep".to_string(), sleep_need);

        let ai = MonsterAIData {
            base_mood: 70.0,
            anger_threshold: 30.0,
            desertion_threshold: 20.0,
            task_preferences: HashMap::new(),
            room_desires: HashMap::new(),
            discipline_response: HashMap::new(),
        };

        // Load actual monster data from JSON instead of constructing manually
        // For now, create minimal test data
        serde_json::from_str(r#"{
            "id": "test_creature",
            "name": "Test",
            "description": "Test creature",
            "faction": "dungeon",
            "role": "worker",
            "stats": {
                "health": 100,
                "mana": 0,
                "attack": 5,
                "defense": 2,
                "speed": 1.0,
                "carry_capacity": 10,
                "sight_radius": 5
            },
            "needs": {
                "sleep": {
                    "decay_per_minute": 1.0,
                    "satisfied_by": ["lair"]
                }
            },
            "traits": [],
            "ai": {
                "base_mood": 70,
                "anger_threshold": 30,
                "desertion_threshold": 20,
                "task_preferences": {},
                "room_desires": {},
                "discipline_response": {}
            },
            "combat": {
                "attack_type": "melee",
                "damage_range": [3, 6],
                "attack_speed": 1.0,
                "armor_type": "none",
                "resistances": {},
                "abilities": []
            },
            "progression": {
                "xp_to_level": [0, 100],
                "stat_growth_per_level": {},
                "max_level": 2,
                "mutations": []
            },
            "economy": {
                "wage_per_minute": 1,
                "steals_if_unpaid": false,
                "drops_gold_on_death": [5, 10]
            },
            "spawn": {
                "source": "portal",
                "min_dungeon_reputation": 0,
                "preferred_rooms": [],
                "spawn_weight": 1.0,
                "max_population": 10
            },
            "visual": {
                "sprite": "test.png",
                "scale": 1.0,
                "animations": ["idle"],
                "voice_set": "test"
            }
        }"#).unwrap()
    }

    #[test]
    fn test_calculate_mood() {
        let monster_data = create_test_monster_data();
        let mut creature = CreatureState::new("test_creature".to_string(), 1, 100.0, 0.0);

        // High need satisfaction = high mood
        creature.set_need("sleep".to_string(), 80.0);
        let mood = calculate_mood(&creature, &monster_data);
        assert!(mood > 70.0);

        // Low need satisfaction = low mood
        creature.set_need("sleep".to_string(), 20.0);
        let mood = calculate_mood(&creature, &monster_data);
        assert!(mood < 70.0);
    }

    #[test]
    fn test_should_desert() {
        let monster_data = create_test_monster_data();
        let mut creature = CreatureState::new("test_creature".to_string(), 1, 100.0, 0.0);

        creature.mood = 50.0;
        assert!(!should_desert(&creature, &monster_data));

        creature.mood = 15.0;
        assert!(should_desert(&creature, &monster_data));
    }

    #[test]
    fn test_task_desirability() {
        let monster_data = create_test_monster_data();
        let mut creature = CreatureState::new("test_creature".to_string(), 1, 100.0, 0.0);

        // Low sleep need = low desirability for sleep task
        creature.set_need("sleep".to_string(), 90.0);
        let task = Task::Sleep(0);
        let desirability = calculate_task_desirability(&task, &creature, &monster_data);
        assert!(desirability < 1.5);

        // High sleep need = high desirability for sleep task
        creature.set_need("sleep".to_string(), 10.0);
        let desirability = calculate_task_desirability(&task, &creature, &monster_data);
        assert!(desirability > 1.5);
    }
}
