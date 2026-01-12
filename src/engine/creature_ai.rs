//! Creature AI - Needs-based decision making
//! Stateless service for creature behavior

use crate::data::monsters::MonsterData;
use crate::data::rooms::RoomData;
use crate::data::GameData;
use crate::engine::room_validator::Room;
use crate::state::entities::{CreatureState, Task};
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;
use std::collections::HashMap;

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
pub fn calculate_mood(creature: &CreatureState, monster_data: &MonsterData) -> f32 {
    let base_mood = monster_data.ai.base_mood as f32;

    // Start from base mood
    let mut mood = base_mood;

    // Average all needs (0-100 each)
    let need_count = creature.needs.len() as f32;
    if need_count > 0.0 {
        let total_satisfaction: f32 = creature.needs.values().sum();
        let average_satisfaction = total_satisfaction / need_count;

        // Needs contribute ±30 points to mood
        // 100% satisfied = +30, 0% satisfied = -30
        let need_modifier = (average_satisfaction - 50.0) * 0.6;
        mood += need_modifier;
    }

    // Health affects mood
    let health_percent = (creature.health / creature.max_health) * 100.0;
    if health_percent < 30.0 {
        mood -= 20.0; // Low health makes creatures unhappy
    }

    // Being angry reduces mood
    if creature.is_angry {
        mood -= 15.0;
    }

    // Clamp to 0-100
    mood.clamp(0.0, 100.0)
}

/// Update creature mood
pub fn update_mood(creature: &mut CreatureState, monster_data: &MonsterData) {
    creature.mood = calculate_mood(creature, monster_data);

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
pub fn find_best_room(
    room_type: &str,
    creature_pos: TilePos,
    rooms: &[Room],
    min_quality: f32,
) -> Option<usize> {
    let mut best: Option<(usize, f32)> = None;

    for room in rooms {
        // Check if room matches type and quality requirements
        if room.room_type != room_type || room.quality < min_quality || !room.active {
            continue;
        }

        // Calculate distance to room center
        let center = room.get_center();
        let dx = (center.x - creature_pos.x) as f32;
        let dy = (center.y - creature_pos.y) as f32;
        let distance = (dx * dx + dy * dy).sqrt();

        // Update best if this is closer
        match best {
            None => best = Some((room.id, distance)),
            Some((_, best_dist)) if distance < best_dist => {
                best = Some((room.id, distance));
            }
            _ => {}
        }
    }

    best.map(|(id, _)| id)
}

/// Calculate desirability of a task for a creature
pub fn calculate_task_desirability(
    task: &Task,
    creature: &CreatureState,
    monster_data: &MonsterData,
) -> f32 {
    let task_type = task.task_type();
    let mut desirability = 1.0;

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
                desirability *= 1.5 + (gold_need / 100.0);
            } else {
                desirability *= 0.1; // Don't deposit if no gold
            }
        }
        Task::Train(_) => {
            // Training is more desirable when satisfied (not urgent need)
            let avg_satisfaction = if !creature.needs.is_empty() {
                creature.needs.values().sum::<f32>() / creature.needs.len() as f32
            } else {
                50.0
            };
            if avg_satisfaction > 60.0 {
                desirability *= 1.5;
            } else {
                desirability *= 0.5;
            }
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

    // Debug: Log available rooms
    eprintln!("[AI] Available rooms for {}: {:?}",
        creature.creature_id,
        game_state.rooms.iter()
            .map(|r| format!("{}(id={}, active={}, quality={:.1})", r.room_type, r.id, r.active, r.quality))
            .collect::<Vec<_>>()
    );

    // If fleeing or deserting, flee
    if creature.is_deserting {
        eprintln!("[AI] Creature {} is deserting!", creature.creature_id);
        return Some(Task::Flee);
    }

    // If carrying gold, prioritize depositing
    if creature.gold_carried > 20 {
        if let Some(room_id) = find_best_room("treasury", creature_pos, &game_state.rooms, 0.0) {
            return Some(Task::DepositGold(room_id));
        }
    }

    // Check most urgent need
    if let Some((need_name, need_value)) = creature.get_most_urgent_need() {
        eprintln!("[AI] Most urgent need for {}: {} = {:.1}%", creature.creature_id, need_name, need_value);

        // If need is critical (below 30), prioritize satisfying it
        if need_value < 30.0 {
            eprintln!("[AI] Critical need {} detected!", need_name);
            // Find room that satisfies this need
            if let Some(need_data) = monster_data.needs.get(&need_name) {
                eprintln!("[AI] Need {} satisfied by rooms: {:?}", need_name, need_data.satisfied_by);
                for room_type in &need_data.satisfied_by {
                    if let Some(room_id) =
                        find_best_room(room_type, creature_pos, &game_state.rooms, 0.0)
                    {
                        eprintln!("[AI] Found {} room (id={})", room_type, room_id);
                        // Determine task type based on need
                        match need_name.as_str() {
                            "sleep" => return Some(Task::Sleep(room_id)),
                            "food" => return Some(Task::Eat(room_id)),
                            "gold" => return Some(Task::DepositGold(room_id)),
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
    eprintln!("[AI] Evaluating room desires: {:?}", monster_data.ai.room_desires);
    for (room_type, desire) in &monster_data.ai.room_desires {
        if let Some(room_id) = find_best_room(room_type, creature_pos, &game_state.rooms, 0.0) {
            let task = Task::Work(room_id);
            let desirability = calculate_task_desirability(&task, creature, monster_data) * desire;
            eprintln!("[AI] Candidate: Work({}) - desirability={:.2}", room_type, desirability);
            candidate_tasks.push((task, desirability));
        } else {
            eprintln!("[AI] No {} room found for work task", room_type);
        }
    }

    // Add training if available and mood is good
    if creature.mood > 50.0 && creature.level < 5 {
        if let Some(room_id) = find_best_room("training_room", creature_pos, &game_state.rooms, 0.0)
        {
            let task = Task::Train(room_id);
            let desirability = calculate_task_desirability(&task, creature, monster_data);
            eprintln!("[AI] Candidate: Train - desirability={:.2}", desirability);
            candidate_tasks.push((task, desirability));
        }
    }

    // Add research if creature can research
    if let Some(room_id) = find_best_room("library", creature_pos, &game_state.rooms, 0.0) {
        let task = Task::Research(room_id);
        let desirability = calculate_task_desirability(&task, creature, monster_data) * 0.8;
        eprintln!("[AI] Candidate: Research - desirability={:.2}", desirability);
        candidate_tasks.push((task, desirability));
    }

    eprintln!("[AI] Total candidate tasks: {}", candidate_tasks.len());

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
        | Task::DepositGold(room_id) => {
            // Check if room exists and is active
            game_state
                .rooms
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
            } else if creature.mood < 40.0 {
                result.push((entity_id, "unhappy".to_string()));
            }

            // Check critical needs
            if let Some((need_name, need_value)) = creature.get_most_urgent_need() {
                if need_value < 20.0 {
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
