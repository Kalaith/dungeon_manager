//! Task execution system
//! Handles creature task completion logic

use crate::data::GameData;
use crate::engine::creature_ai;
use crate::state::entities::{EntityId, EntityManager, Task};
use crate::state::player_state::PlayerState;
use crate::state::room_manager::RoomManager;
use crate::state::tile_state::{TilePos, TileState};

/// Result of task execution that may require state updates
pub struct TaskResult {
    pub gold_change: i32,
    pub food_change: i32,
    pub materials_change: i32,
    pub research_change: f32,
    pub claimed_tile: Option<TilePos>,
    pub task_complete: bool,
}

impl Default for TaskResult {
    fn default() -> Self {
        Self {
            gold_change: 0,
            food_change: 0,
            materials_change: 0,
            research_change: 0.0,
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
    get_tile: impl Fn(TilePos) -> Option<TileState>,
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
            execute_sleep(creature_id, *room_id, entities, room_manager, dt);
        }
        Task::Eat(room_id) => {
            result.food_change = execute_eat(creature_id, *room_id, entities, room_manager, player, dt);
        }
        Task::DepositGold(room_id) => {
            result.gold_change = execute_deposit_gold(creature_id, *room_id, entities, room_manager, dt);
            if result.gold_change > 0 {
                result.task_complete = true;
            }
        }
        Task::Train(room_id) => {
            execute_train(creature_id, *room_id, entities, room_manager, dt);
        }
        Task::Dig(_) => {
            // Do nothing here.
            // Digging is handled by imp_ai system which manages the task timer.
            // If we complete it here, the imp will stop digging instantly every frame.
        }
        Task::Work(room_id) => {
            result.materials_change = execute_work(creature_id, *room_id, entities, room_manager, game_data, dt);
        }
        Task::Research(room_id) => {
             result.research_change = execute_research(creature_id, *room_id, entities, room_manager, game_data, dt);
        }
        Task::CollectWages(room_id) => {
            result.gold_change = execute_collect_wages(creature_id, *room_id, entities, room_manager, player, game_data, dt);
            if result.gold_change < 0 {
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
fn execute_sleep(creature_id: EntityId, room_id: usize, entities: &mut EntityManager, room_manager: &RoomManager, dt: f32) {
    let room = match room_manager.rooms.iter().find(|r| r.id == room_id) {
        Some(r) if r.room_type == "lair" => r,
        _ => return,
    };
    let creature = match entities.get_mut(creature_id).and_then(|e| e.as_creature_mut()) {
        Some(c) => c,
        None => return,
    };
    creature_ai::satisfy_need(creature, "sleep", 10.0, dt);
}

/// Handle Eat task - returns food consumed (negative)
fn execute_eat(creature_id: EntityId, room_id: usize, entities: &mut EntityManager, room_manager: &RoomManager, player: &PlayerState, dt: f32) -> i32 {
    let room = match room_manager.rooms.iter().find(|r| r.id == room_id) {
        Some(r) if r.room_type == "hatchery" => r,
        _ => return 0,
    };

    if player.food <= 0 {
        return 0;
    }

    let food_consumed = (dt * 5.0).min(player.food as f32) as i32;
    if let Some(creature) = entities.get_mut(creature_id).and_then(|e| e.as_creature_mut()) {
        creature_ai::satisfy_need(creature, "food", food_consumed as f32 * 2.0, dt);
    }

    -food_consumed
}

/// Handle DepositGold task - returns gold deposited
fn execute_deposit_gold(creature_id: EntityId, room_id: usize, entities: &mut EntityManager, room_manager: &RoomManager, dt: f32) -> i32 {
    let is_treasury = room_manager.rooms.iter().any(|r| r.id == room_id && r.room_type == "treasury");
    if !is_treasury {
        return 0;
    }

    let creature = match entities.get_mut(creature_id).and_then(|e| e.as_creature_mut()) {
        Some(c) => c,
        None => return 0,
    };

    let gold = creature.gold_carried;
    creature.gold_carried = 0;
    creature_ai::satisfy_need(creature, "gold", 5.0, dt);
    gold
}

/// Handle Train task
fn execute_train(creature_id: EntityId, room_id: usize, entities: &mut EntityManager, room_manager: &RoomManager, dt: f32) {
    let is_training_room = room_manager.rooms.iter().any(|r| r.id == room_id && r.room_type == "training_room");
    if !is_training_room {
        return;
    }

    let creature = match entities.get_mut(creature_id).and_then(|e| e.as_creature_mut()) {
        Some(c) => c,
        None => return,
    };

    creature.training_timer += dt;
    if creature.training_timer < 1.0 {
        return;
    }

    creature.training_timer = 0.0;
    creature.experience += 10.0;

    if creature.level >= 5 {
        creature.experience = creature.max_experience;
        return;
    }

    if creature.experience >= creature.max_experience {
        creature.level += 1;
        creature.experience = 0.0;
        creature.max_experience *= 1.5;
        creature.max_health *= 1.2;
        creature.health = creature.max_health;
        eprintln!("Creature {} leveled up to {}", creature.creature_id, creature.level);
    }
}

/// Handle Work task - returns materials produced
fn execute_work(creature_id: EntityId, room_id: usize, entities: &mut EntityManager, room_manager: &RoomManager, game_data: &GameData, dt: f32) -> i32 {
    let room = match room_manager.rooms.iter().find(|r| r.id == room_id && r.room_type == "workshop") {
        Some(r) => r,
        None => return 0,
    };

    let creature = match entities.get_mut(creature_id).and_then(|e| e.as_creature_mut()) {
        Some(c) => c,
        None => return 0,
    };

    let efficiency = game_data.monsters.get(&creature.creature_id)
        .map(|m| creature_ai::calculate_work_efficiency(creature, m))
        .unwrap_or(1.0);

    creature.work_timer += dt * room.efficiency * efficiency;

    if creature.work_timer >= 5.0 {
        creature.work_timer = 0.0;
        eprintln!("Creature {} produced generic material!", creature.creature_id);
        return 1;
    }

    0
}

/// Handle CollectWages task - returns gold consumed from player (negative)
fn execute_collect_wages(creature_id: EntityId, room_id: usize, entities: &mut EntityManager, room_manager: &RoomManager, player: &PlayerState, game_data: &GameData, dt: f32) -> i32 {
    let is_treasury = room_manager.rooms.iter().any(|r| r.id == room_id && r.room_type == "treasury");
    if !is_treasury || player.gold <= 0 {
        return 0;
    }

    let creature = match entities.get_mut(creature_id).and_then(|e| e.as_creature_mut()) {
        Some(c) => c,
        None => return 0,
    };

    if game_data.monsters.get(&creature.creature_id).is_none() {
        return 0;
    }

    creature_ai::satisfy_need(creature, "gold", 5.0, dt);
    -1
}

/// Handle Research task - returns research points generated
fn execute_research(creature_id: EntityId, room_id: usize, entities: &mut EntityManager, room_manager: &RoomManager, game_data: &GameData, dt: f32) -> f32 {
    let is_library = room_manager.rooms.iter().any(|r| r.id == room_id && r.room_type == "library");
    if !is_library {
        return 0.0;
    }

    let creature = match entities.get_mut(creature_id).and_then(|e| e.as_creature_mut()) {
        Some(c) => c,
        None => return 0.0,
    };

    let efficiency = game_data.monsters.get(&creature.creature_id)
        .map(|m| creature_ai::calculate_work_efficiency(creature, m))
        .unwrap_or(1.0);

    2.0 * dt * efficiency
}
