//! Technical configuration constants
//!
//! Note: Game balance constants have been moved to data-driven JSON files:
//! - Player starting resources and capacities: assets/data/game_config.json
//! - Trap costs: assets/data/traps.json
//! - Tile costs (spawner): assets/data/tiles.json
//! - Monster data (imp limits, summon costs): assets/data/monsters.json
//! - Room costs: assets/data/rooms.json
//! - Hero wave timing: assets/data/game_config.json
//! - Creature AI thresholds: assets/data/game_config.json
//! - Fog of war settings: assets/data/game_config.json

// ============================================================================
// ENGINE CONSTANTS (Technical - not game balance)
// ============================================================================

/// Fixed timestep for game simulation (10 ticks per second)
pub const TICK_RATE: f32 = 0.1;

/// Visual movement interpolation speed (rendering smoothness)
pub const MOVEMENT_LERP_SPEED: f32 = 10.0;
