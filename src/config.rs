//! Global configuration constants

/// Cost to place a monster spawner
pub const SPAWNER_COST: i32 = 50;



/// Cost to build a door
pub const DOOR_COST: i32 = 5;

/// Cost to build a spike trap
pub const SPIKE_TRAP_COST: i32 = 10;

/// Refund amount for selling a room tile
pub const ROOM_SELL_REFUND: i32 = 5;

/// Gold value of a single gem seam mine action
pub const GEM_MINE_VALUE: i32 = 25;

// Player Starting Stats
pub const STARTING_GOLD: i32 = 20000;
pub const STARTING_MANA: i32 = 10000;
pub const STARTING_FOOD: i32 = 100;
pub const STARTING_MATERIALS: i32 = 100;

pub const INITIAL_MAX_GOLD: i32 = 20000; // Capacity determined by treasury/dungeon heart rooms
pub const INITIAL_MAX_MANA: i32 = 0;
pub const INITIAL_MAX_FOOD: i32 = 500;
pub const INITIAL_MAX_MATERIALS: i32 = 100;

/// Whether Fog of War is enabled (visibility tracking)
pub const FOG_OF_WAR_ENABLED: bool = true;

// ============================================================================
// GAME TIMING CONSTANTS
// ============================================================================

/// Fixed timestep for game simulation (10 ticks per second)
pub const TICK_RATE: f32 = 0.1;

/// Pay day interval in seconds (5 minutes)
pub const PAY_DAY_INTERVAL: f32 = 300.0;

/// Initial delay before first hero spawns (seconds)
pub const INITIAL_HERO_SPAWN_DELAY: f32 = 30.0;

/// Initial delay before first creature spawns (seconds)
pub const INITIAL_CREATURE_SPAWN_DELAY: f32 = 10.0;

/// Minimum time between creature spawns (seconds)
pub const CREATURE_SPAWN_MIN_INTERVAL: f32 = 15.0;

/// Maximum time between creature spawns (seconds)
pub const CREATURE_SPAWN_MAX_INTERVAL: f32 = 30.0;

/// Visual movement interpolation speed
pub const MOVEMENT_LERP_SPEED: f32 = 10.0;

// ============================================================================
// CREATURE LIMITS
// ============================================================================

/// Maximum number of imps allowed
pub const MAX_IMPS: usize = 10;

/// Base mana cost for summoning an imp
pub const IMP_SUMMON_BASE_COST: i32 = 10;

/// Additional mana cost per existing imp
pub const IMP_SUMMON_COST_PER_IMP: i32 = 5;

// ============================================================================
// FOG OF WAR
// ============================================================================

/// Sight radius for creatures providing vision
pub const FOG_OF_WAR_SIGHT_RADIUS: i32 = 5;

// ============================================================================
// CREATURE AI THRESHOLDS
// ============================================================================

/// Need value below which a need is considered critical
pub const NEED_CRITICAL_THRESHOLD: f32 = 30.0;

/// Need value below which creature should desert
pub const NEED_DESERT_THRESHOLD: f32 = 10.0;

/// Mood threshold below which creatures need attention
pub const MOOD_ATTENTION_THRESHOLD: f32 = 40.0;

/// Need value below which it's considered critically low for attention
pub const NEED_ATTENTION_THRESHOLD: f32 = 20.0;

// ============================================================================
// HERO WAVE ATTACK CONSTANTS
// ============================================================================

/// Initial delay before first hero wave attacks (5 minutes)
pub const HERO_WAVE_INITIAL_DELAY: f32 = 2000.0;

/// Time between hero attack waves (3 minutes)
pub const HERO_WAVE_INTERVAL: f32 = 180.0;

/// Fraction of heroes that stay as defenders at base (0.5 = 50%)
pub const HERO_DEFENDER_RATIO: f32 = 0.5;

/// Time in seconds for a hero to dig through one wall tile
pub const HERO_DIG_TIME: f32 = 0.5; // Made faster (was 2.0) to help them break through

/// Max health for the dungeon heart
pub const DUNGEON_HEART_MAX_HEALTH: f32 = 1000.0;

