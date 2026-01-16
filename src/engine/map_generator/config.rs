//! Map generator configuration and types

use crate::state::tile_state::TileState;

/// Grid type alias for the map
pub type Grid = Vec<Vec<TileState>>;

// ============================================================================
// CONFIGURATION
// ============================================================================

#[derive(Debug, Clone)]
pub struct MapConfig {
    pub width: usize,
    pub height: usize,
    pub seed: Option<u64>,
    pub gold_richness: f32,      // 0.0 - 1.0, affects gold vein frequency
    pub gem_richness: f32,        // 0.0 - 1.0, affects gem frequency
    pub mana_richness: f32,       // 0.0 - 1.0, affects mana crystal frequency
    pub water_frequency: f32,     // 0.0 - 1.0, chance of water regions
    pub lava_frequency: f32,      // 0.0 - 1.0, chance of lava regions
    pub starting_area_size: usize, // Size of cleared starting area

    // Phase 1: Noise terrain parameters
    pub use_noise_terrain: bool,         // Enable noise-based terrain generation
    pub cave_density: f32,               // 0.0 = lots of caves, 1.0 = mostly solid
    pub cave_smoothing_iterations: usize, // Number of cellular automata passes

    // Phase 2: Natural features and portals
    pub enable_natural_features: bool,    // Add stone pillars, chambers, etc.
    pub num_stone_pillars: usize,         // Number of stone pillars to place
    pub num_collapsed_chambers: usize,    // Number of large open caverns
    pub num_hero_portals: usize,          // Number of hero spawn portals
    pub min_portal_distance: f32,         // Minimum distance from start for portals

    // Phase 3: Biome system
    pub enable_biomes: bool,              // Enable biome-based terrain variation
    pub num_biome_regions: usize,         // Number of distinct biome regions

    // Phase 4: Starting position
    pub starting_position: StartingPosition,
    
    // Phase 5: Hero Base
    pub hero_base_enabled: bool,
    pub hero_base_position: StartingPosition,

    // Difficulty & starting layout
    pub difficulty: Difficulty,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            width: 50,
            height: 50,
            seed: None,
            gold_richness: 0.3,
            gem_richness: 0.15,
            mana_richness: 0.2,
            water_frequency: 0.1,
            lava_frequency: 0.05,
            starting_area_size: 7,

            // Phase 1 defaults
            use_noise_terrain: false,  // DISABLED - use flat solid terrain instead
            cave_density: 0.3,
            cave_smoothing_iterations: 3,

            // Phase 2 defaults
            enable_natural_features: true,
            num_stone_pillars: 8,
            num_collapsed_chambers: 3,
            num_hero_portals: 2,
            min_portal_distance: 25.0,

            // Phase 3 defaults
            enable_biomes: false,  // DISABLED to reduce gem spam
            num_biome_regions: 4,

            // Phase 4 defaults
            starting_position: StartingPosition::Center,
            
            hero_base_enabled: true,
            hero_base_position: StartingPosition::Corner,

            // Difficulty default
            difficulty: Difficulty::Normal,
        }
    }
}

// ============================================================================
// BIOME TYPES
// ============================================================================

/// Distinct biome regions with unique properties
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Biome {
    Standard,     // Normal earth and rock
    Volcanic,     // Lava flows, fire hazards
    Crystalline,  // Rich in mana crystals
    Flooded,      // Underground rivers, water pools
    Ancient,      // Ruins and special floors
    Corrupted,    // Dark corruption, stronger monsters
}

// ============================================================================
// PHASE 4: STARTING POSITION
// ============================================================================

/// Strategies for placing the player's starting dungeon heart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StartingPosition {
    #[default]
    Center,       // Map center (classic)
    Corner,       // Random corner
    Edge,         // Random edge position
    Random,       // Random valid position
}

// ============================================================================
// DIFFICULTY & STARTING LAYOUT
// ============================================================================

/// Game difficulty affects starting resources and layout
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Difficulty {
    Easy,         // Large starting area, more rooms, extra resources
    #[default]
    Normal,       // Standard layout
    Hard,         // Small starting area, fewer rooms
    Nightmare,    // Minimal starting area, dangerous
}

/// Defines what rooms are included in starting area
#[derive(Debug, Clone)]
pub struct StartingLayout {
    pub cleared_area_size: usize,  // Size of cleared floor
    pub rooms: Vec<StartingRoom>,  // Rooms to create
}

/// A room to place in the starting area
#[derive(Debug, Clone)]
pub struct StartingRoom {
    pub room_type: String,
    pub size: usize,
    pub offset_x: i32,  // Offset from center
    pub offset_y: i32,
}

impl StartingLayout {
    /// Generate layout based on difficulty
    pub fn from_difficulty(difficulty: Difficulty) -> Self {
        match difficulty {
            Difficulty::Easy => Self {
                cleared_area_size: 3,
                rooms: vec![], // No prebuilt rooms - player builds their own
            },
            Difficulty::Normal => Self {
                cleared_area_size: 3,
                rooms: vec![], // No prebuilt rooms - player builds their own
            },
            Difficulty::Hard => Self {
                cleared_area_size: 3,
                rooms: vec![], // No prebuilt rooms
            },
            Difficulty::Nightmare => Self {
                cleared_area_size: 3,
                rooms: vec![], // Minimal space, no rooms
            },
        }
    }
}

