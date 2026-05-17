use crate::data::monsters::MonsterData;
use crate::data::GameData;
use crate::state::entities::{CreatureState, Task};
use crate::state::room_manager::RoomManager;
use crate::state::tile_state::TilePos;

/// Try to satisfy a critical need by finding an appropriate room
pub fn try_satisfy_critical_need(
    creature: &CreatureState,
    creature_pos: TilePos,
    room_manager: &RoomManager,
    monster_data: &MonsterData,
    game_data: &GameData,
) -> Option<Task> {
    let (need_name, need_value) = creature.get_most_urgent_need()?;

    if need_value >= game_data.config.creature_ai.need_critical_threshold {
        return None;
    }

    let need_data = monster_data.needs.get(&need_name)?;

    for room_type in &need_data.satisfied_by {
        use crate::engine::room_validator;
        let (room_id, _) =
            room_validator::find_nearest_room(&room_manager.rooms, room_type, creature_pos, 0.0)?;

        return match need_name.as_str() {
            "sleep" => Some(Task::Sleep(room_id)),
            "food" => Some(Task::Eat(room_id)),
            "gold" => Some(Task::CollectWages(room_id)),
            _ => None,
        };
    }

    None
}

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

/// Check if creature should desert the dungeon
pub fn should_desert(creature: &CreatureState, monster_data: &MonsterData) -> bool {
    let desertion_threshold = monster_data.ai.desertion_threshold as f32;
    creature.mood < desertion_threshold
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

        let _ai = MonsterAIData {
            base_mood: 70.0,
            anger_threshold: 30.0,
            desertion_threshold: 20.0,
            task_preferences: HashMap::new(),
            room_desires: HashMap::new(),
            discipline_response: HashMap::new(),
        };

        // Minimal test data skeleton
        serde_json::from_str(r#"{
            "id": "test_creature",
            "name": "Test",
            "description": "Test creature",
            "faction": "dungeon",
            "role": "worker",
            "stats": { "health": 100, "mana": 0, "attack": 5, "defense": 2, "speed": 1.0, "carry_capacity": 10, "sight_radius": 5 },
            "needs": { "sleep": { "decay_per_minute": 1.0, "satisfied_by": ["lair"] } },
            "traits": [],
            "ai": { "base_mood": 70, "anger_threshold": 30, "desertion_threshold": 20, "task_preferences": {}, "room_desires": {}, "discipline_response": {} },
            "combat": { "attack_type": "melee", "damage_range": [3, 6], "attack_speed": 1.0, "armor_type": "none", "resistances": {}, "abilities": [] },
            "progression": { "xp_to_level": [0, 100], "stat_growth_per_level": {}, "max_level": 2, "mutations": [] },
            "economy": { "wage_per_minute": 1, "steals_if_unpaid": false, "drops_gold_on_death": [5, 10] },
            "spawn": { "source": "portal", "min_dungeon_reputation": 0, "preferred_rooms": [], "spawn_weight": 1.0, "max_population": 10 },
            "visual": { "sprite": "test.png", "scale": 1.0, "animations": ["idle"], "voice_set": "test" }
        }"#).unwrap()
    }

    #[test]
    fn test_should_desert() {
        let monster_data = create_test_monster_data();
        let mut creature = CreatureState::new("test_creature".to_string(), 1, 100.0, 0.0, 0);

        creature.mood = 50.0;
        assert!(!should_desert(&creature, &monster_data));

        creature.mood = 15.0;
        assert!(should_desert(&creature, &monster_data));
    }

    #[test]
    fn test_task_desirability() {
        let monster_data = create_test_monster_data();
        let game_data = GameData::default();
        let mut creature = CreatureState::new("test_creature".to_string(), 1, 100.0, 0.0, 0);
        let task = Task::Sleep(0);

        // High satisfaction means a sleep task is not urgent.
        creature.set_need("sleep".to_string(), 90.0);
        let desirability = calculate_task_desirability(&task, &creature, &monster_data, &game_data);
        assert!(desirability < 1.5);

        // Low satisfaction makes sleep more desirable.
        creature.set_need("sleep".to_string(), 10.0);
        let desirability = calculate_task_desirability(&task, &creature, &monster_data, &game_data);
        assert!(desirability > 1.5);
    }
}
