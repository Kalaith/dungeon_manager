//! Task execution system
//! Handles creature task completion logic

use crate::data::GameData;
use crate::engine::creature_ai;
use crate::engine::tile_types::types as tt;
use crate::engine::room_validator::Room;
use crate::state::entities::{CreatureState, EntityId, EntityManager, Task};
use crate::state::player_state::PlayerState;
use crate::state::room_manager::RoomManager;
use crate::state::tile_state::{Ownership, TilePos, TileState};

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
fn execute_sleep(
    creature_id: EntityId,
    room_id: usize,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    dt: f32,
) {
    if let Some(room) = room_manager.rooms.iter().find(|r| r.id == room_id) {
        if room.room_type == "lair" {
            if let Some(creature) = entities.get_mut(creature_id)
                .and_then(|e| e.as_creature_mut())
            {
                creature_ai::satisfy_need(creature, "sleep", 10.0, dt);
            }
        }
    }
}

/// Handle Eat task - returns food consumed (negative)
fn execute_eat(
    creature_id: EntityId,
    room_id: usize,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    player: &PlayerState,
    dt: f32,
) -> i32 {
    if let Some(room) = room_manager.rooms.iter().find(|r| r.id == room_id) {
        if room.room_type == "hatchery" && player.food > 0 {
            let food_consumed = (dt * 5.0).min(player.food as f32) as i32;
            
            if let Some(creature) = entities.get_mut(creature_id)
                .and_then(|e| e.as_creature_mut())
            {
                creature_ai::satisfy_need(creature, "food", food_consumed as f32 * 2.0, dt);
            }
            
            return -food_consumed;
        }
    }
    0
}

/// Handle DepositGold task - returns gold deposited
fn execute_deposit_gold(
    creature_id: EntityId,
    room_id: usize,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    dt: f32,
) -> i32 {
    if let Some(room) = room_manager.rooms.iter().find(|r| r.id == room_id) {
        if room.room_type == "treasury" {
            if let Some(creature) = entities.get_mut(creature_id)
                .and_then(|e| e.as_creature_mut())
            {
                let gold = creature.gold_carried;
                creature.gold_carried = 0;
                // Minor satisfaction for doing good work
                // But NOT main gold satisfaction (that's wages)
                creature_ai::satisfy_need(creature, "gold", 5.0, dt);
                return gold;
            }
        }
    }
    0
}

/// Handle Train task
fn execute_train(
    creature_id: EntityId,
    room_id: usize,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    dt: f32,
) {
    if let Some(room) = room_manager.rooms.iter().find(|r| r.id == room_id) {
        if room.room_type == "training_room" {
            if let Some(creature) = entities.get_mut(creature_id)
                .and_then(|e| e.as_creature_mut())
            {
                creature.training_timer += dt;
                
                if creature.training_timer >= 1.0 {
                    creature.training_timer = 0.0;
                    creature.experience += 10.0;
                    
                    // Check level up (Max level 5)
                    if creature.experience >= creature.max_experience && creature.level < 5 {
                        creature.level += 1;
                        creature.experience = 0.0;
                        creature.max_experience *= 1.5;
                        creature.max_health *= 1.2;
                        creature.health = creature.max_health;
                        eprintln!("Creature {} leveled up to {}", creature.creature_id, creature.level);
                    } else if creature.level >= 5 {
                        creature.experience = creature.max_experience;
                    }
                }
            }
        }
    }
}

/// Handle Work task - returns materials produced
fn execute_work(
    creature_id: EntityId,
    room_id: usize,
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    game_data: &GameData,
    dt: f32,
) -> i32 {
    if let Some(room) = room_manager.rooms.iter().find(|r| r.id == room_id) {
        if room.room_type == "workshop" {
            if let Some(creature) = entities.get_mut(creature_id)
                .and_then(|e| e.as_creature_mut())
            {
                // Work progress with efficiency
                let efficiency = if let Some(monster_data) = game_data.monsters.get(&creature.creature_id) {
                    creature_ai::calculate_work_efficiency(creature, monster_data)
                } else {
                    1.0
                };
                
                creature.work_timer += dt * room.efficiency * efficiency;
                
                if creature.work_timer >= 5.0 {
                    creature.work_timer = 0.0;
                    eprintln!("Creature {} produced generic material!", creature.creature_id);
                    return 1;
                }
            }
        }
    }
    0
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
) -> i32 {
    if let Some(room) = room_manager.rooms.iter().find(|r| r.id == room_id) {
        if room.room_type == "treasury" {
            // Determine wage rate from monster data
            if let Some(creature) = entities.get_mut(creature_id)
                .and_then(|e| e.as_creature_mut()) 
            {
                if let Some(monster_data) = game_data.monsters.get(&creature.creature_id) {
                    let wage_per_minute = monster_data.economy.wage_per_minute.max(1) as f32;
                    let wage_per_second = wage_per_minute / 60.0;
                    
                    // Logic:
                    // Creature collects gold at a fast rate to satisfy need
                    // Player pays that gold
                    
                    // If player has gold
                    if player.gold > 0 {
                        // Take gold roughly 10x faster than wage accumulation so they don't spend forever collecting
                        // But precise amount matters. 
                        // Simplified: Pay 1 gold per tick (0.1s) until need is full?
                        // Or calculate how much 'gold need' is missing and charge player proportional cost?
                        // "Gold Need" 0..100 roughly maps to "Unpaid Wages"?
                        // If decay is 1.0/min, and wage is 1g/min.
                        // Then 1 unit of need = 1 unit of gold?
                        // Let's assume 1 unit of need = 1 gold for simplicity for now.
                        
                        let gold_taken = 1; // 1 gold per tick = 10 gold per second
                        
                        creature_ai::satisfy_need(creature, "gold", 5.0, dt); // Restore need quickly
                        
                        return -gold_taken;
                    } else {
                        // Player is broke!
                        // Creature gets angry?
                        // For now just stand there
                    }
                }
            }
        }
    }
    0
}
