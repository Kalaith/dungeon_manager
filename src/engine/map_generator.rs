//! Procedural map generation for dungeons
//! Creates varied, playable maps with interesting terrain features

use crate::data::GameData;
use crate::state::tile_state::{Ownership, TilePos, TileState};
use rand::{Rng, SeedableRng};

pub type Grid = Vec<Vec<TileState>>;

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
        }
    }
}

/// Main map generation function
pub fn generate_map(config: &MapConfig, _game_data: &GameData) -> Grid {
    let mut rng = if let Some(seed) = config.seed {
        rand::rngs::StdRng::seed_from_u64(seed)
    } else {
        rand::rngs::StdRng::from_entropy()
    };

    // Step 1: Create base terrain (solid rock borders, earth interior)
    let mut grid = create_base_terrain(config.width, config.height);

    // Step 2: Add resource veins (gold, gems)
    add_resource_veins(&mut grid, config, &mut rng);

    // Step 3: Add hazard regions (water, lava)
    add_hazard_regions(&mut grid, config, &mut rng);

    // Step 4: Create player starting area
    create_starting_area(&mut grid, config);

    grid
}

/// Create base terrain with solid rock borders and earth interior
fn create_base_terrain(width: usize, height: usize) -> Grid {
    let mut grid = Vec::new();

    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
            let tile_type = if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                "solid_rock".to_string()
            } else {
                "earth".to_string()
            };

            let tile = TileState::new(tile_type, TilePos::new(x as i32, y as i32));
            row.push(tile);
        }
        grid.push(row);
    }

    grid
}

/// Add resource veins (gold and gems) in clustered patterns
fn add_resource_veins(grid: &mut Grid, config: &MapConfig, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();

    // Generate gold vein clusters
    let num_gold_clusters = (config.gold_richness * 15.0) as usize + 5;
    for _ in 0..num_gold_clusters {
        let center_x = rng.gen_range(5..width - 5);
        let center_y = rng.gen_range(5..height - 5);
        let cluster_size = rng.gen_range(3..8);

        create_vein_cluster(grid, center_x, center_y, cluster_size, "gold_vein", rng);
    }

    // Generate gem seam clusters
    let num_gem_clusters = (config.gem_richness * 10.0) as usize + 3;
    for _ in 0..num_gem_clusters {
        let center_x = rng.gen_range(5..width - 5);
        let center_y = rng.gen_range(5..height - 5);
        let cluster_size = rng.gen_range(2..5);

        create_vein_cluster(grid, center_x, center_y, cluster_size, "gem_seam", rng);
    }

    // Generate mana crystal clusters
    let num_mana_clusters = (config.mana_richness * 12.0) as usize + 2;
    for _ in 0..num_mana_clusters {
        let center_x = rng.gen_range(5..width - 5);
        let center_y = rng.gen_range(5..height - 5);
        let cluster_size = rng.gen_range(2..4);

        create_vein_cluster(grid, center_x, center_y, cluster_size, "mana_crystal", rng);
    }
}

/// Create a cluster of resource tiles
fn create_vein_cluster(
    grid: &mut Grid,
    center_x: usize,
    center_y: usize,
    size: usize,
    tile_type: &str,
    rng: &mut impl Rng,
) {
    let height = grid.len();
    let width = grid[0].len();

    for _ in 0..size {
        let dx = rng.gen_range(-3..=3);
        let dy = rng.gen_range(-3..=3);
        let x = (center_x as i32 + dx).max(1).min(width as i32 - 2) as usize;
        let y = (center_y as i32 + dy).max(1).min(height as i32 - 2) as usize;

        // Only place on earth tiles
        if grid[y][x].tile_type == "earth" {
            grid[y][x].tile_type = tile_type.to_string();

            // Set resource amount based on type
            let resources = match tile_type {
                "gold_vein" => rng.gen_range(80..150),
                "gem_seam" => rng.gen_range(100..200),
                "mana_crystal" => rng.gen_range(200..300),
                _ => 100,
            };
            grid[y][x].resources_remaining = Some(resources);
        }
    }
}

/// Add hazard regions (water pools, lava pools)
fn add_hazard_regions(grid: &mut Grid, config: &MapConfig, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();

    // Add water pools
    if rng.gen::<f32>() < config.water_frequency {
        let num_water_pools = rng.gen_range(1..4);
        for _ in 0..num_water_pools {
            let center_x = rng.gen_range(10..width - 10);
            let center_y = rng.gen_range(10..height - 10);
            let pool_size = rng.gen_range(4..10);

            create_hazard_pool(grid, center_x, center_y, pool_size, "water", rng);
        }
    }

    // Add lava pools
    if rng.gen::<f32>() < config.lava_frequency {
        let num_lava_pools = rng.gen_range(1..3);
        for _ in 0..num_lava_pools {
            let center_x = rng.gen_range(10..width - 10);
            let center_y = rng.gen_range(10..height - 10);
            let pool_size = rng.gen_range(3..7);

            create_hazard_pool(grid, center_x, center_y, pool_size, "lava", rng);
        }
    }
}

/// Create a pool of hazardous terrain
fn create_hazard_pool(
    grid: &mut Grid,
    center_x: usize,
    center_y: usize,
    size: usize,
    tile_type: &str,
    rng: &mut impl Rng,
) {
    let height = grid.len();
    let width = grid[0].len();

    for _ in 0..(size * size / 2) {
        let size_i32 = size as i32;
        let dx = rng.gen_range(-size_i32..=size_i32);
        let dy = rng.gen_range(-size_i32..=size_i32);
        let x = (center_x as i32 + dx).max(1).min(width as i32 - 2) as usize;
        let y = (center_y as i32 + dy).max(1).min(height as i32 - 2) as usize;

        // Only place on earth tiles
        if grid[y][x].tile_type == "earth" {
            grid[y][x].tile_type = tile_type.to_string();
            grid[y][x].resources_remaining = None;
        }
    }
}

/// Create the player's starting area with dungeon heart and some rooms
fn create_starting_area(grid: &mut Grid, config: &MapConfig) {
    let height = grid.len();
    let width = grid[0].len();

    // Place starting area in center
    let center_x = width / 2;
    let center_y = height / 2;
    let size = config.starting_area_size;

    // Clear area around center
    for dy in -(size as i32)..=(size as i32) {
        for dx in -(size as i32)..=(size as i32) {
            let x = (center_x as i32 + dx).max(1).min(width as i32 - 2) as usize;
            let y = (center_y as i32 + dy).max(1).min(height as i32 - 2) as usize;

            // Create cleared floor
            grid[y][x].tile_type = "claimed_floor".to_string();
            grid[y][x].ownership = Ownership::Player;
            grid[y][x].resources_remaining = None;
        }
    }

    // Place dungeon heart in center
    grid[center_y][center_x].tile_type = "dungeon_heart".to_string();
    grid[center_y][center_x].ownership = Ownership::Player;

    // Create a small lair to the left
    for dy in -2..=2 {
        for dx in -2..=2 {
            let x = (center_x as i32 - 6 + dx).max(1).min(width as i32 - 2) as usize;
            let y = (center_y as i32 + dy).max(1).min(height as i32 - 2) as usize;

            grid[y][x].tile_type = "lair".to_string();
            grid[y][x].ownership = Ownership::Player;
            grid[y][x].resources_remaining = None;
        }
    }

    // Create a small hatchery to the right
    for dy in -2..=2 {
        for dx in -2..=2 {
            let x = (center_x as i32 + 6 + dx).max(1).min(width as i32 - 2) as usize;
            let y = (center_y as i32 + dy).max(1).min(height as i32 - 2) as usize;

            grid[y][x].tile_type = "hatchery".to_string();
            grid[y][x].ownership = Ownership::Player;
            grid[y][x].resources_remaining = None;
        }
    }

    // Create a small treasury above
    for dy in -2..=2 {
        for dx in -2..=2 {
            let x = (center_x as i32 + dx).max(1).min(width as i32 - 2) as usize;
            let y = (center_y as i32 - 6 + dy).max(1).min(height as i32 - 2) as usize;

            grid[y][x].tile_type = "treasury".to_string();
            grid[y][x].ownership = Ownership::Player;
            grid[y][x].resources_remaining = None;
        }
    }
}

/// Generate a map with specific features for testing
pub fn generate_test_map(width: usize, height: usize, game_data: &GameData) -> Grid {
    let config = MapConfig {
        width,
        height,
        seed: Some(12345), // Fixed seed for consistent testing
        gold_richness: 0.4,
        gem_richness: 0.2,
        mana_richness: 0.2,
        water_frequency: 0.2,
        lava_frequency: 0.1,
        starting_area_size: 8,
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
        starting_area_size: 9,
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
        starting_area_size: 6,
    };

    generate_map(&config, game_data)
}
