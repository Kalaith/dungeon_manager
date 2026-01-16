//! Phase 1 Map Generator Improvements - Example Implementation
//!
//! This file demonstrates concrete implementations of the first phase improvements:
//! - Noise-based terrain generation
//! - Cellular automata cave smoothing
//! - Connectivity validation
//! - Realistic mineral vein generation
//!
//! To integrate: Copy relevant functions into src/engine/map_generator.rs

use crate::data::GameData;
use crate::state::tile_state::{Ownership, TilePos, TileState};
use rand::{Rng, SeedableRng};
use std::collections::{VecDeque, HashSet};

pub type Grid = Vec<Vec<TileState>>;

// ============================================================================
// NOISE-BASED TERRAIN GENERATION
// ============================================================================

/// Simple Perlin-like noise implementation
/// For production, use the `noise` crate for better quality
struct SimpleNoise {
    seed: u64,
}

impl SimpleNoise {
    fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Generate noise value at (x, y) in range [-1.0, 1.0]
    fn noise2d(&self, x: f64, y: f64) -> f64 {
        // Simple hash-based noise (replace with proper Perlin for production)
        let n = (x * 12.9898 + y * 78.233 + self.seed as f64).sin() * 43758.5453;
        (n - n.floor()) * 2.0 - 1.0
    }

    /// Multi-octave noise (more natural variation)
    fn fractal_noise(&self, x: f64, y: f64, octaves: usize) -> f64 {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            value += self.noise2d(x * frequency, y * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }

        value / max_value
    }
}

/// Enhanced map configuration with noise parameters
#[derive(Debug, Clone)]
pub struct EnhancedMapConfig {
    pub width: usize,
    pub height: usize,
    pub seed: Option<u64>,

    // Resource parameters
    pub gold_richness: f32,
    pub gem_richness: f32,
    pub mana_richness: f32,

    // Terrain generation
    pub use_noise_terrain: bool,
    pub cave_density: f32,          // 0.0 = lots of caves, 1.0 = mostly solid
    pub cave_smoothing_iterations: usize,

    // Hazards
    pub water_frequency: f32,
    pub lava_frequency: f32,

    // Starting area
    pub starting_area_size: usize,
}

impl Default for EnhancedMapConfig {
    fn default() -> Self {
        Self {
            width: 50,
            height: 50,
            seed: None,
            gold_richness: 0.3,
            gem_richness: 0.15,
            mana_richness: 0.2,
            use_noise_terrain: true,
            cave_density: 0.4,
            cave_smoothing_iterations: 3,
            water_frequency: 0.1,
            lava_frequency: 0.05,
            starting_area_size: 7,
        }
    }
}

/// Main enhanced map generation function
pub fn generate_enhanced_map(config: &EnhancedMapConfig, _game_data: &GameData) -> Grid {
    let mut rng = if let Some(seed) = config.seed {
        rand::rngs::StdRng::seed_from_u64(seed)
    } else {
        rand::rngs::StdRng::from_entropy()
    };

    // Step 1: Generate base terrain using noise
    let mut grid = if config.use_noise_terrain {
        create_noise_terrain(config, &mut rng)
    } else {
        create_base_terrain(config.width, config.height)
    };

    // Step 2: Smooth terrain with cellular automata
    if config.use_noise_terrain {
        smooth_caves_cellular_automata(&mut grid, config.cave_smoothing_iterations);
    }

    // Step 3: Ensure connectivity
    ensure_connectivity(&mut grid, &mut rng);

    // Step 4: Add realistic mineral veins
    add_mineral_veins(&mut grid, config, &mut rng);

    // Step 5: Add hazard regions
    add_enhanced_hazards(&mut grid, config, &mut rng);

    // Step 6: Create starting area
    create_starting_area(&mut grid, config);

    grid
}

/// Create terrain using Perlin noise for natural cave systems
fn create_noise_terrain(config: &EnhancedMapConfig, rng: &mut impl Rng) -> Grid {
    let seed = config.seed.unwrap_or(rng.gen());
    let noise = SimpleNoise::new(seed);
    let mut grid = Vec::new();

    let scale = 0.08; // Controls cave size (lower = larger features)

    for y in 0..config.height {
        let mut row = Vec::new();
        for x in 0..config.width {
            // Generate noise value
            let noise_val = noise.fractal_noise(x as f64 * scale, y as f64 * scale, 3);

            // Border is always solid rock
            let tile_type = if x == 0 || y == 0 || x == config.width - 1 || y == config.height - 1 {
                "solid_rock".to_string()
            } else {
                // Threshold determines cave density
                if noise_val > config.cave_density as f64 {
                    "solid_rock".to_string()
                } else {
                    "earth".to_string()
                }
            };

            let tile = TileState::new(tile_type, TilePos::new(x as i32, y as i32));
            row.push(tile);
        }
        grid.push(row);
    }

    grid
}

/// Traditional flat terrain generation (fallback)
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

// ============================================================================
// CELLULAR AUTOMATA SMOOTHING
// ============================================================================

/// Smooth caves using cellular automata to create more natural shapes
fn smooth_caves_cellular_automata(grid: &mut Grid, iterations: usize) {
    let height = grid.len();
    let width = grid[0].len();

    for _ in 0..iterations {
        let mut next_grid = grid.clone();

        for y in 1..height-1 {
            for x in 1..width-1 {
                let solid_neighbors = count_solid_neighbors(grid, x, y);

                // Cellular automata rules:
                // - If 5+ solid neighbors, become solid (fills small gaps)
                // - If <4 solid neighbors, become open (removes isolated rocks)
                // - Border tiles stay solid
                if solid_neighbors >= 5 {
                    next_grid[y][x].tile_type = "solid_rock".to_string();
                } else if solid_neighbors < 4 {
                    next_grid[y][x].tile_type = "earth".to_string();
                }
            }
        }

        *grid = next_grid;
    }
}

/// Count solid rock neighbors (8-way)
fn count_solid_neighbors(grid: &Grid, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if is_solid_tile(&grid[ny][nx].tile_type) {
                count += 1;
            }
        }
    }
    count
}

fn is_solid_tile(tile_type: &str) -> bool {
    matches!(tile_type, "solid_rock" | "gold_vein" | "gem_seam" | "mana_crystal" | "reinforced_wall")
}

// ============================================================================
// CONNECTIVITY VALIDATION
// ============================================================================

/// Ensure all open regions are connected via tunnels
fn ensure_connectivity(grid: &mut Grid, rng: &mut impl Rng) {
    let regions = find_disconnected_regions(grid);

    if regions.is_empty() {
        eprintln!("Warning: No open regions found!");
        return;
    }

    if regions.len() == 1 {
        // Already connected
        return;
    }

    println!("Found {} disconnected regions, connecting...", regions.len());

    // Connect all regions to the largest one
    let largest_region = &regions[0];
    for i in 1..regions.len() {
        connect_regions(grid, largest_region, &regions[i], rng);
    }
}

/// Find all disconnected open regions using flood fill
fn find_disconnected_regions(grid: &Grid) -> Vec<Vec<TilePos>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = vec![vec![false; width]; height];
    let mut regions = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if !visited[y][x] && !is_solid_tile(&grid[y][x].tile_type) {
                let region = flood_fill_region(grid, x, y, &mut visited);
                if !region.is_empty() {
                    regions.push(region);
                }
            }
        }
    }

    // Sort by size, largest first
    regions.sort_by_key(|r| std::cmp::Reverse(r.len()));
    regions
}

/// Flood fill to find all tiles in a connected region
fn flood_fill_region(grid: &Grid, start_x: usize, start_y: usize, visited: &mut Vec<Vec<bool>>) -> Vec<TilePos> {
    let height = grid.len();
    let width = grid[0].len();
    let mut region = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_x, start_y));
    visited[start_y][start_x] = true;

    while let Some((x, y)) = queue.pop_front() {
        region.push(TilePos::new(x as i32, y as i32));

        // Check 4-way neighbors
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = (x as i32 + dx);
            let ny = (y as i32 + dy);

            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                if !visited[ny][nx] && !is_solid_tile(&grid[ny][nx].tile_type) {
                    visited[ny][nx] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    region
}

/// Connect two regions by carving a tunnel
fn connect_regions(grid: &mut Grid, region_a: &[TilePos], region_b: &[TilePos], rng: &mut impl Rng) {
    // Find closest points between regions
    let (start, end) = find_closest_points(region_a, region_b);

    // Carve L-shaped tunnel (straight + turn)
    carve_tunnel(grid, start, end, rng);
}

/// Find the two closest points between two regions
fn find_closest_points(region_a: &[TilePos], region_b: &[TilePos]) -> (TilePos, TilePos) {
    let mut min_dist = f32::INFINITY;
    let mut best_pair = (region_a[0], region_b[0]);

    // Sample subset of points for performance
    let step = (region_a.len() / 20).max(1);
    for a_pos in region_a.iter().step_by(step) {
        for b_pos in region_b.iter().step_by(step) {
            let dist = distance_f32(*a_pos, *b_pos);
            if dist < min_dist {
                min_dist = dist;
                best_pair = (*a_pos, *b_pos);
            }
        }
    }

    best_pair
}

/// Carve an L-shaped tunnel between two points
fn carve_tunnel(grid: &mut Grid, start: TilePos, end: TilePos, rng: &mut impl Rng) {
    let width = 1; // Tunnel width

    // Choose whether to go horizontal-then-vertical or vertical-then-horizontal
    let horizontal_first = rng.gen_bool(0.5);

    if horizontal_first {
        // Horizontal segment
        carve_horizontal_line(grid, start.x, end.x, start.y, width);
        // Vertical segment
        carve_vertical_line(grid, end.x, start.y, end.y, width);
    } else {
        // Vertical segment
        carve_vertical_line(grid, start.x, start.y, end.y, width);
        // Horizontal segment
        carve_horizontal_line(grid, start.x, end.x, end.y, width);
    }
}

fn carve_horizontal_line(grid: &mut Grid, x1: i32, x2: i32, y: i32, width: i32) {
    let (start_x, end_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

    for x in start_x..=end_x {
        for dy in -width..=width {
            let ny = y + dy;
            if ny > 0 && ny < grid.len() as i32 - 1 && x > 0 && x < grid[0].len() as i32 - 1 {
                let tile = &mut grid[ny as usize][x as usize];
                if is_solid_tile(&tile.tile_type) {
                    tile.tile_type = "earth".to_string();
                }
            }
        }
    }
}

fn carve_vertical_line(grid: &mut Grid, x: i32, y1: i32, y2: i32, width: i32) {
    let (start_y, end_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    for y in start_y..=end_y {
        for dx in -width..=width {
            let nx = x + dx;
            if nx > 0 && nx < grid[0].len() as i32 - 1 && y > 0 && y < grid.len() as i32 - 1 {
                let tile = &mut grid[y as usize][nx as usize];
                if is_solid_tile(&tile.tile_type) {
                    tile.tile_type = "earth".to_string();
                }
            }
        }
    }
}

// ============================================================================
// REALISTIC MINERAL VEINS
// ============================================================================

/// Add mineral veins using drunk walk for realistic patterns
fn add_mineral_veins(grid: &mut Grid, config: &EnhancedMapConfig, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();

    // Gold veins - larger, more common
    let num_gold_veins = (config.gold_richness * 12.0) as usize + 3;
    for _ in 0..num_gold_veins {
        let start_x = rng.gen_range(5..width - 5);
        let start_y = rng.gen_range(5..height - 5);
        generate_mineral_vein(
            grid,
            TilePos::new(start_x as i32, start_y as i32),
            "gold_vein",
            rng.gen_range(15..30),
            2,
            rng,
        );
    }

    // Gem seams - smaller, rarer
    let num_gem_veins = (config.gem_richness * 8.0) as usize + 2;
    for _ in 0..num_gem_veins {
        let start_x = rng.gen_range(5..width - 5);
        let start_y = rng.gen_range(5..height - 5);
        generate_mineral_vein(
            grid,
            TilePos::new(start_x as i32, start_y as i32),
            "gem_seam",
            rng.gen_range(10..20),
            1,
            rng,
        );
    }

    // Mana crystals - clustered, medium size
    let num_mana_veins = (config.mana_richness * 10.0) as usize + 2;
    for _ in 0..num_mana_veins {
        let start_x = rng.gen_range(5..width - 5);
        let start_y = rng.gen_range(5..height - 5);
        generate_mineral_vein(
            grid,
            TilePos::new(start_x as i32, start_y as i32),
            "mana_crystal",
            rng.gen_range(8..15),
            1,
            rng,
        );
    }
}

/// Generate a single mineral vein using drunk walk algorithm
fn generate_mineral_vein(
    grid: &mut Grid,
    start: TilePos,
    tile_type: &str,
    length: usize,
    thickness: usize,
    rng: &mut impl Rng,
) {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut current = start;
    let mut direction = (rng.gen_range(-1..=1), rng.gen_range(-1..=1));

    if direction == (0, 0) {
        direction = (1, 0); // Ensure we have a direction
    }

    for _ in 0..length {
        // Place vein segment
        place_vein_segment(grid, current, tile_type, thickness, rng);

        // Drunk walk: 70% continue same direction, 30% change direction
        if rng.gen::<f32>() < 0.7 {
            current.x += direction.0;
            current.y += direction.1;
        } else {
            direction = (rng.gen_range(-1..=1), rng.gen_range(-1..=1));
            if direction == (0, 0) {
                direction = (1, 0);
            }
            current.x += direction.0;
            current.y += direction.1;
        }

        // Clamp to valid bounds (not on border)
        current.x = current.x.max(1).min(width - 2);
        current.y = current.y.max(1).min(height - 2);
    }
}

/// Place a circular segment of mineral vein
fn place_vein_segment(grid: &mut Grid, center: TilePos, tile_type: &str, thickness: usize, rng: &mut impl Rng) {
    let t = thickness as i32;
    for dy in -t..=t {
        for dx in -t..=t {
            if dx*dx + dy*dy <= (t * t) {
                let x = (center.x + dx).max(1).min(grid[0].len() as i32 - 2) as usize;
                let y = (center.y + dy).max(1).min(grid.len() as i32 - 2) as usize;

                // Only place on solid rock
                if grid[y][x].tile_type == "solid_rock" {
                    grid[y][x].tile_type = tile_type.to_string();

                    // Set resource amount
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
    }
}

// ============================================================================
// ENHANCED HAZARDS
// ============================================================================

/// Add hazard regions with more organic shapes
fn add_enhanced_hazards(grid: &mut Grid, config: &EnhancedMapConfig, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();

    // Add water regions
    if rng.gen::<f32>() < config.water_frequency {
        let num_water = rng.gen_range(1..3);
        for _ in 0..num_water {
            let center_x = rng.gen_range(10..width - 10);
            let center_y = rng.gen_range(10..height - 10);
            let size = rng.gen_range(5..12);
            create_organic_hazard_pool(grid, center_x, center_y, size, "water", rng);
        }
    }

    // Add lava regions
    if rng.gen::<f32>() < config.lava_frequency {
        let num_lava = rng.gen_range(1..3);
        for _ in 0..num_lava {
            let center_x = rng.gen_range(10..width - 10);
            let center_y = rng.gen_range(10..height - 10);
            let size = rng.gen_range(4..9);
            create_organic_hazard_pool(grid, center_x, center_y, size, "lava", rng);
        }
    }
}

/// Create hazard pool with organic shape using flood fill
fn create_organic_hazard_pool(
    grid: &mut Grid,
    cx: usize,
    cy: usize,
    target_size: usize,
    tile_type: &str,
    rng: &mut impl Rng,
) {
    let height = grid.len();
    let width = grid[0].len();

    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    queue.push_back((cx, cy));
    visited[cy][cx] = true;

    let mut tiles_placed = 0;
    let max_tiles = target_size * target_size;

    while let Some((x, y)) = queue.pop_front() {
        if tiles_placed >= max_tiles {
            break;
        }

        // Only place on earth
        if grid[y][x].tile_type == "earth" {
            grid[y][x].tile_type = tile_type.to_string();
            grid[y][x].resources_remaining = None;
            tiles_placed += 1;
        }

        // Spread to neighbors with decay probability
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;

            if nx > 5 && nx < width - 5 && ny > 5 && ny < height - 5 {
                if !visited[ny][nx] && rng.gen::<f32>() < 0.65 {
                    visited[ny][nx] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }
}

// ============================================================================
// STARTING AREA (UNCHANGED)
// ============================================================================

fn create_starting_area(grid: &mut Grid, config: &EnhancedMapConfig) {
    let height = grid.len();
    let width = grid[0].len();
    let center_x = width / 2;
    let center_y = height / 2;
    let size = config.starting_area_size;

    // Clear area around center
    for dy in -(size as i32)..=(size as i32) {
        for dx in -(size as i32)..=(size as i32) {
            let x = (center_x as i32 + dx).max(1).min(width as i32 - 2) as usize;
            let y = (center_y as i32 + dy).max(1).min(height as i32 - 2) as usize;

            grid[y][x].tile_type = "claimed_floor".to_string();
            grid[y][x].ownership = Ownership::Player;
            grid[y][x].resources_remaining = None;
        }
    }

    // Place dungeon heart
    grid[center_y][center_x].tile_type = "dungeon_heart".to_string();
    grid[center_y][center_x].ownership = Ownership::Player;

    // Starting rooms (same as before)
    create_starting_room(grid, center_x - 6, center_y, 2, "lair");
    create_starting_room(grid, center_x + 6, center_y, 2, "hatchery");
    create_starting_room(grid, center_x, center_y - 6, 2, "treasury");
}

fn create_starting_room(grid: &mut Grid, cx: usize, cy: usize, size: usize, room_type: &str) {
    let width = grid[0].len();
    let height = grid.len();

    for dy in -(size as i32)..=(size as i32) {
        for dx in -(size as i32)..=(size as i32) {
            let x = (cx as i32 + dx).max(1).min(width as i32 - 2) as usize;
            let y = (cy as i32 + dy).max(1).min(height as i32 - 2) as usize;

            grid[y][x].tile_type = room_type.to_string();
            grid[y][x].ownership = Ownership::Player;
            grid[y][x].resources_remaining = None;
        }
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn distance_f32(a: TilePos, b: TilePos) -> f32 {
    let dx = (a.x - b.x) as f32;
    let dy = (a.y - b.y) as f32;
    (dx * dx + dy * dy).sqrt()
}

// ============================================================================
// EXAMPLE USAGE
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_map_generation() {
        let config = EnhancedMapConfig {
            width: 60,
            height: 60,
            seed: Some(12345),
            use_noise_terrain: true,
            cave_density: 0.35,
            cave_smoothing_iterations: 4,
            ..Default::default()
        };

        // Note: This won't compile without actual GameData
        // let grid = generate_enhanced_map(&config, &game_data);
        // assert_eq!(grid.len(), 60);
        // assert_eq!(grid[0].len(), 60);
    }

    #[test]
    fn test_noise_generation() {
        let noise = SimpleNoise::new(42);
        let val = noise.fractal_noise(10.0, 10.0, 3);
        assert!(val >= -1.0 && val <= 1.0);
    }
}
