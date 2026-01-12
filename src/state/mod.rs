// State module - game state management
pub mod entities;
pub mod game_state;
pub mod player_state;
pub mod tile_state;

pub use entities::{
    CreatureState, Entity, EntityId, EntityManager, EntityType, HeroGoal, HeroState, StatusEffect,
    Task,
};
pub use game_state::MapType;
pub use player_state::PlayerState;
pub use tile_state::{FogState, Ownership, TilePos, TileState};
