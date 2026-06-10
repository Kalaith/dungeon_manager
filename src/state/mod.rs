pub mod camera_state;
mod combat_tick;
pub mod drag_selection;
pub mod dungeon;
pub mod entities;
pub mod game_state;
mod heart_attack;
pub mod hero_base;
pub mod interaction;
pub mod map_loader;
pub mod notifications;
pub mod player_state;
pub mod projectiles;
pub mod room_manager;
mod rooms;
pub mod save_system;
mod spawning;
pub mod tile_state;

#[cfg(target_arch = "wasm32")]
pub mod wasm_storage;

pub use drag_selection::DragSelection;
pub use game_state::MapType;
pub use interaction::{GamePhase, InteractionMode};
pub use tile_state::{Ownership, TilePos};
