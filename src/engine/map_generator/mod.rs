//! Procedural map generation for dungeons
//!
//! Modular map generator with:
//! - Phase 1: Noise terrain, cellular automata, connectivity, veins
//! - Phase 2: Natural features, hero portals  
//! - Phase 3: Biome system
//! - Phase 4: Variable starting positions

mod biomes;
mod config;
mod connectivity;
mod features;
mod resources;
mod starting_area;
mod terrain;
mod utils;
mod hero_base_gen;

// Re-export public types
pub use config::{Grid, MapConfig, StartingPosition};

use crate::data::GameData;
use macroquad::rand;

// ============================================================================
// MAIN GENERATION FUNCTION
// ============================================================================

/// Main map generation function
pub fn generate_map(config: &MapConfig, game_data: &GameData) -> Grid {
    // Seed the RNG if a seed is provided
    if let Some(seed) = config.seed {
        rand::srand(seed);
    } else {
        // Use current time as seed for variety
        rand::srand(macroquad::miniquad::date::now() as u64);
    }

    // Step 1: Create base terrain
    let mut grid = if config.use_noise_terrain {
        terrain::create_noise_terrain(config)
    } else {
        terrain::create_base_terrain(config.width, config.height)
    };

    // Step 2: Smooth terrain with cellular automata
    if config.use_noise_terrain {
        terrain::smooth_caves_cellular_automata(&mut grid, config.cave_smoothing_iterations);
    }

    // Step 3: Ensure connectivity
    connectivity::ensure_connectivity(&mut grid);

    // Step 4: Calculate starting position early (needed for strategic placement)
    let start_pos = starting_area::calculate_starting_position(config);

    // Step 5: Add hazard regions FIRST (needed for risk/reward resource placement)
    resources::add_enhanced_hazards(&mut grid, config);

    // Step 6: Place hero portals early (mana crystals placed near them)
    if config.num_hero_portals > 0 {
        features::place_hero_portals(&mut grid, start_pos, config);
    }

    // Step 7: Add strategic mineral veins (risk/reward placement)
    resources::add_mineral_veins(&mut grid, config, start_pos, game_data);

    // Step 8: Apply biome features
    if config.enable_biomes {
        let biome_map = biomes::generate_biome_map(config.width, config.height, config.num_biome_regions);
        biomes::apply_biome_features(&mut grid, &biome_map);
    }

    // Step 9: Add natural features
    if config.enable_natural_features {
        features::add_natural_features(&mut grid, config);
        features::add_monster_lairs(&mut grid, start_pos, config);
    }

    // Step 10: Create starting area LAST (clears resources to make room)
    starting_area::create_starting_area(&mut grid, config, start_pos);

    // Step 11: Create Hero Base
    if config.hero_base_enabled {
        hero_base_gen::place_hero_base(&mut grid, start_pos, config);
    }

    grid
}

// ============================================================================
// PRESET MAP GENERATORS
// ============================================================================

/// Generate a map with specific features for testing
pub fn generate_test_map(width: usize, height: usize, game_data: &GameData) -> Grid {
    let config = MapConfig {
        width,
        height,
        seed: Some(12345),
        gold_richness: 0.4,
        gem_richness: 0.2,
        mana_richness: 0.2,
        water_frequency: 0.2,
        lava_frequency: 0.1,
        starting_area_size: 3,
        use_noise_terrain: false,  // Flat solid terrain
        enable_biomes: false,
        ..Default::default()
    };
    generate_map(&config, game_data)
}

/// Generate a rich map with lots of resources
pub fn generate_rich_map(width: usize, height: usize, game_data: &GameData) -> Grid {
    let config = MapConfig {
        width,
        height,
        seed: None,
        gold_richness: 0.6,
        gem_richness: 0.4,
        mana_richness: 0.4,
        water_frequency: 0.05,
        lava_frequency: 0.02,
        starting_area_size: 3,
        use_noise_terrain: false,  // Flat solid terrain
        enable_biomes: false,
        ..Default::default()
    };
    generate_map(&config, game_data)
}

/// Generate a challenging map with hazards
pub fn generate_hazardous_map(width: usize, height: usize, game_data: &GameData) -> Grid {
    let config = MapConfig {
        width,
        height,
        seed: None,
        gold_richness: 0.2,
        gem_richness: 0.1,
        mana_richness: 0.1,
        water_frequency: 0.0,
        lava_frequency: 0.3,
        starting_area_size: 3,
        use_noise_terrain: false,  // Flat solid terrain
        enable_biomes: false,
        num_hero_portals: 4,
        ..Default::default()
    };
    generate_map(&config, game_data)
}
