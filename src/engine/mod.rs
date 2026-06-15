// Engine module - stateless game logic services
pub mod combat;
pub mod creature_ai;
mod creature_targets;
pub mod hero_ai;
pub mod hero_parties;
pub mod hero_spawner;
pub mod pathfinding;
pub mod rival_keeper_ai;
pub mod room_validator;
pub mod tile_grid;

pub mod action_processor;
pub mod imp_ai;
pub mod map_generator;
pub mod spell_effects;
pub mod task_system;
pub mod tile_types;
pub mod trap_system;

pub mod creature_needs;
pub mod creature_task_logic;
pub mod hero_digging;
pub mod movement;
pub mod objectives;
pub mod prison_system;
pub mod scenario_events;
pub mod spawner;
pub mod spawner_logic;
pub mod special_rooms;

pub mod input;
pub mod input_handlers;
