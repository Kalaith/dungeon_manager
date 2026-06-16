//! Task execution system
//! Handles creature task completion logic

use crate::data::GameData;
use crate::engine::creature_ai;
use crate::state::entities::{EntityId, EntityManager, Task};
use crate::state::player_state::PlayerState;
use crate::state::room_manager::RoomManager;
use crate::state::tile_state::TilePos;

/// Result of task execution that may require state updates
pub struct TaskResult {
    pub gold_change: f32,
    pub food_change: f32,
    pub materials_change: f32,
    pub research_change: f32,
    pub manufactured_trap: Option<String>,
    pub claimed_tile: Option<TilePos>,
    pub task_complete: bool,
}

impl Default for TaskResult {
    fn default() -> Self {
        Self {
            gold_change: 0.0,
            food_change: 0.0,
            materials_change: 0.0,
            research_change: 0.0,
            manufactured_trap: None,
            claimed_tile: None,
            task_complete: false,
        }
    }
}

/// Execute a creature's current task
/// Returns changes that need to be applied to game state
pub fn execute_task(
    creature_id: EntityId,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    player: &PlayerState,
    game_data: &GameData,
    dt: f32,
) -> TaskResult {
    let mut result = TaskResult::default();

    // Get the current task
    let task = {
        let entity = match entities.get(creature_id) {
            Some(e) => e,
            None => return result,
        };
        let creature = match entity.as_creature() {
            Some(c) => c,
            None => return result,
        };
        creature.current_task.clone()
    };

    let Some(task) = task else { return result };

    match &task {
        Task::Sleep(room_id) => {
            execute_sleep(creature_id, *room_id, entities, room_manager, game_data, dt);
        }
        Task::Eat(room_id) => {
            result.food_change = execute_eat(
                creature_id,
                *room_id,
                entities,
                room_manager,
                player,
                game_data,
                dt,
            );
        }
        Task::DepositGold(room_id) => {
            result.gold_change =
                execute_deposit_gold(creature_id, *room_id, entities, room_manager, game_data, dt);
            if result.gold_change > 0.0 {
                result.task_complete = true;
            }
        }
        Task::Train(room_id) => {
            execute_train(creature_id, *room_id, entities, room_manager, game_data, dt);
        }
        Task::Dig(_) => {
            // Do nothing here.
            // Digging is handled by imp_ai system which manages the task timer.
            // If we complete it here, the imp will stop digging instantly every frame.
        }
        Task::Work(room_id, _) => {
            let work = execute_work(
                creature_id,
                *room_id,
                entities,
                room_manager,
                player,
                game_data,
                dt,
            );
            result.materials_change = work.materials_change;
            result.manufactured_trap = work.manufactured_trap;
        }
        Task::Research(room_id) => {
            result.research_change =
                execute_research(creature_id, *room_id, entities, room_manager, game_data, dt);
        }
        Task::CollectWages(room_id) => {
            result.gold_change = execute_collect_wages(
                creature_id,
                *room_id,
                entities,
                room_manager,
                player,
                game_data,
                dt,
            );
            if result.gold_change < 0.0 {
                // If we successfully collected gold (negative change), task is done
                // But simplified: satisfaction happens over time or instant?
                // Let's make it periodic satisfaction
                // Actually, wages are paid incrementally or lump sum?
                // For now, simple rate of wage collection
            }
        }
        _ => {}
    }

    // Mark task complete if needed
    if result.task_complete {
        if let Some(entity) = entities.get_mut(creature_id) {
            if let Some(creature) = entity.as_creature_mut() {
                creature.current_task = None;
            }
        }
    }

    result
}

/// Handle Sleep task
fn execute_sleep(
    creature_id: EntityId,
    room_id: usize,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    game_data: &GameData,
    dt: f32,
) {
    // Verify room is a lair
    if !room_manager
        .rooms
        .iter()
        .any(|r| r.id == room_id && r.room_type == "lair")
    {
        return;
    }
    let creature = match entities
        .get_mut(creature_id)
        .and_then(|e| e.as_creature_mut())
    {
        Some(c) => c,
        None => return,
    };
    let sleep_rate = game_data.config.task_execution.sleep_satisfaction_rate;
    creature_ai::satisfy_need(creature, "sleep", sleep_rate, dt);
}

/// Handle Eat task - returns food consumed (negative)
fn execute_eat(
    creature_id: EntityId,
    room_id: usize,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    player: &PlayerState,
    game_data: &GameData,
    dt: f32,
) -> f32 {
    // Verify room is a hatchery
    if !room_manager
        .rooms
        .iter()
        .any(|r| r.id == room_id && r.room_type == "hatchery")
    {
        return 0.0;
    }

    if player.food <= 0 {
        return 0.0;
    }

    let food_rate = game_data.config.task_execution.food_consumption_rate;
    let food_multiplier = game_data.config.task_execution.food_satisfaction_multiplier;
    let food_consumed = (dt * food_rate).min(player.food as f32); // Keep precise float
    if let Some(creature) = entities
        .get_mut(creature_id)
        .and_then(|e| e.as_creature_mut())
    {
        creature_ai::satisfy_need(creature, "food", food_consumed * food_multiplier, dt);
    }

    -food_consumed
}

/// Handle DepositGold task - returns gold deposited
fn execute_deposit_gold(
    creature_id: EntityId,
    room_id: usize,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    game_data: &GameData,
    dt: f32,
) -> f32 {
    let is_treasury = room_manager
        .rooms
        .iter()
        .any(|r| r.id == room_id && r.room_type == "treasury");
    if !is_treasury {
        return 0.0;
    }

    let creature = match entities
        .get_mut(creature_id)
        .and_then(|e| e.as_creature_mut())
    {
        Some(c) => c,
        None => return 0.0,
    };

    let gold = creature.gold_carried;
    creature.gold_carried = 0;
    let gold_satisfaction_rate = game_data
        .config
        .task_execution
        .gold_deposit_satisfaction_rate;
    creature_ai::satisfy_need(creature, "gold", gold_satisfaction_rate, dt);
    gold as f32
}

/// Handle Train task
fn execute_train(
    creature_id: EntityId,
    room_id: usize,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    game_data: &GameData,
    dt: f32,
) {
    let is_training_room = room_manager
        .rooms
        .iter()
        .any(|r| r.id == room_id && r.room_type == "training_room");
    if !is_training_room {
        return;
    }

    let creature = match entities
        .get_mut(creature_id)
        .and_then(|e| e.as_creature_mut())
    {
        Some(c) => c,
        None => return,
    };

    let task_config = &game_data.config.task_execution;
    let combat_config = &game_data.config.combat;

    creature.training_timer += dt;
    if creature.training_timer < task_config.training_timer_threshold {
        return;
    }

    creature.training_timer = 0.0;
    creature.experience += task_config.xp_per_training;

    if creature.level >= combat_config.max_creature_level {
        creature.experience = creature.max_experience;
        return;
    }

    if creature.experience >= creature.max_experience {
        creature.level += 1;
        creature.experience = 0.0;
        creature.max_experience *= task_config.level_up_exp_multiplier;
        creature.max_health *= task_config.level_up_health_multiplier;
        creature.health = creature.max_health;
        eprintln!(
            "Creature {} leveled up to {}",
            creature.creature_id, creature.level
        );
    }
}

/// Handle Work task - returns materials produced
struct WorkResult {
    materials_change: f32,
    manufactured_trap: Option<String>,
}

fn execute_work(
    creature_id: EntityId,
    room_id: usize,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    player: &PlayerState,
    game_data: &GameData,
    dt: f32,
) -> WorkResult {
    let room = match room_manager.rooms.iter().find(|r| {
        r.id == room_id && (r.room_type == "workshop" || r.room_type == "torture_chamber")
    }) {
        Some(r) => r,
        None => {
            return WorkResult {
                materials_change: 0.0,
                manufactured_trap: None,
            }
        }
    };

    let creature = match entities
        .get_mut(creature_id)
        .and_then(|e| e.as_creature_mut())
    {
        Some(c) => c,
        None => {
            return WorkResult {
                materials_change: 0.0,
                manufactured_trap: None,
            }
        }
    };

    let efficiency = game_data
        .monsters
        .get(&creature.creature_id)
        .map(|m| creature_ai::calculate_work_efficiency(creature, m))
        .unwrap_or(1.0);

    creature.work_timer += dt * room.efficiency * efficiency;

    let work_threshold = game_data.config.task_execution.work_timer_threshold;
    if creature.work_timer >= work_threshold {
        creature.work_timer = 0.0;
        if room.room_type == "workshop" {
            let manufactured_trap = select_manufactured_trap(player, game_data);
            if let Some(trap_id) = &manufactured_trap {
                eprintln!(
                    "Creature {} manufactured {} crate.",
                    creature.creature_id, trap_id
                );
            }
            return WorkResult {
                materials_change: 0.0,
                manufactured_trap,
            };
        }
    }

    WorkResult {
        materials_change: 0.0,
        manufactured_trap: None,
    }
}

fn select_manufactured_trap(player: &PlayerState, game_data: &GameData) -> Option<String> {
    let mut candidates: Vec<&crate::data::traps::TrapData> = player
        .unlocked_traps
        .iter()
        .filter_map(|id| game_data.traps.get(id))
        .collect();

    candidates.sort_by(|a, b| {
        player
            .trap_inventory_count(&a.id)
            .cmp(&player.trap_inventory_count(&b.id))
            .then_with(|| a.cost.cmp(&b.cost))
            .then_with(|| a.name.cmp(&b.name))
    });

    candidates.first().map(|trap| trap.id.clone())
}

/// Handle CollectWages task - returns gold consumed from player (negative)
fn execute_collect_wages(
    creature_id: EntityId,
    room_id: usize,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    player: &PlayerState,
    game_data: &GameData,
    dt: f32,
) -> f32 {
    let is_treasury = room_manager
        .rooms
        .iter()
        .any(|r| r.id == room_id && r.room_type == "treasury");
    if !is_treasury || player.gold <= 0 {
        return 0.0;
    }

    let creature = match entities
        .get_mut(creature_id)
        .and_then(|e| e.as_creature_mut())
    {
        Some(c) => c,
        None => return 0.0,
    };

    if !game_data.monsters.contains_key(&creature.creature_id) {
        return 0.0;
    }

    let wage_rate = game_data.config.task_execution.wage_satisfaction_rate;
    // Precise consumption
    let gold_consumed = (wage_rate * dt).min(player.gold as f32);

    creature_ai::satisfy_need(creature, "gold", gold_consumed, dt);
    -gold_consumed
}

/// Handle Research task - returns research points generated
fn execute_research(
    creature_id: EntityId,
    room_id: usize,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    game_data: &GameData,
    dt: f32,
) -> f32 {
    let is_library = room_manager
        .rooms
        .iter()
        .any(|r| r.id == room_id && r.room_type == "library");
    if !is_library {
        return 0.0;
    }

    let creature = match entities
        .get_mut(creature_id)
        .and_then(|e| e.as_creature_mut())
    {
        Some(c) => c,
        None => return 0.0,
    };

    let efficiency = game_data
        .monsters
        .get(&creature.creature_id)
        .map(|m| creature_ai::calculate_work_efficiency(creature, m))
        .unwrap_or(1.0);

    let research_rate = game_data.config.task_execution.research_production_rate;
    research_rate * dt * efficiency
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::room_validator::Room;
    use crate::state::entities::{CreatureState, EntityManager};
    use crate::state::player_state::PlayerState;
    use std::collections::HashSet;

    #[test]
    fn workshop_work_task_manufactures_lowest_stock_unlocked_trap() {
        let game_data = GameData::load().expect("game data should load");
        let mut entities = EntityManager::new();
        let monster_data = game_data.monsters.get("goblin").unwrap();
        let mut creature = CreatureState::new(
            "goblin".to_string(),
            1,
            monster_data.stats.health,
            monster_data.stats.mana,
            1,
        );
        creature.current_task = Some(Task::Work(42, TilePos::new(2, 2)));
        let creature_id = entities.spawn_creature(TilePos::new(2, 2), creature);

        let mut room_manager = RoomManager::new();
        let mut room = Room::new(
            42,
            "workshop".to_string(),
            [TilePos::new(2, 2)].into_iter().collect::<HashSet<_>>(),
            Vec::new(),
        );
        room.active = true;
        room_manager.rooms.push(room);

        let mut player = PlayerState::new(&game_data);
        player.unlock_trap("spike_trap".to_string());
        player.add_trap_inventory("door".to_string(), 2);

        let result = execute_task(
            creature_id,
            &mut entities,
            &room_manager,
            &player,
            &game_data,
            game_data.config.task_execution.work_timer_threshold * 2.0,
        );

        assert_eq!(result.manufactured_trap, Some("spike_trap".to_string()));
        assert_eq!(result.materials_change, 0.0);
    }
}
