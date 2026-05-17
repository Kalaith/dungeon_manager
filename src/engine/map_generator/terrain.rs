//! Terrain generation functions
//! Includes noise generation, base terrain, and cellular automata smoothing

use crate::state::tile_state::{TilePos, TileState};
use macroquad::rand;

use super::config::{Grid, MapConfig};

// ============================================================================
// NOISE GENERATION
// ============================================================================

/// Simple Perlin-like noise implementation
pub struct SimpleNoise {
    seed: u64,
}

impl SimpleNoise {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Generate noise value at (x, y) in range [-1.0, 1.0]
    pub fn noise2d(&self, x: f64, y: f64) -> f64 {
        let n = (x * 12.9898 + y * 78.233 + self.seed as f64).sin() * 43758.5453;
        (n - n.floor()) * 2.0 - 1.0
    }

    /// Multi-octave noise for more natural variation
    pub fn fractal_noise(&self, x: f64, y: f64, octaves: usize) -> f64 {
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

// ============================================================================
// TERRAIN GENERATION
// ============================================================================

/// Create terrain using Perlin noise for natural cave systems
pub fn create_noise_terrain(config: &MapConfig) -> Grid {
    let seed = config
        .seed
        .unwrap_or_else(|| rand::gen_range(0u64, u64::MAX));
    let noise = SimpleNoise::new(seed);
    let mut grid = Vec::new();

    let scale = 0.08;

    for y in 0..config.height {
        let mut row = Vec::new();
        for x in 0..config.width {
            let noise_val = noise.fractal_noise(x as f64 * scale, y as f64 * scale, 3);

            let tile_type = if x == 0 || y == 0 || x == config.width - 1 || y == config.height - 1 {
                "solid_rock".to_string()
            } else {
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
pub fn create_base_terrain(width: usize, height: usize) -> Grid {
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

/// Check if a tile type is considered solid (blocks movement until mined)
pub fn is_solid_tile(tile_type: &str) -> bool {
    matches!(
        tile_type,
        "solid_rock" | "earth" | "gold_vein" | "gem_seam" | "mana_crystal" | "reinforced_wall"
    )
}

// ============================================================================
// CELLULAR AUTOMATA SMOOTHING
// ============================================================================

/// Smooth caves using cellular automata to create more natural shapes
pub fn smooth_caves_cellular_automata(grid: &mut Grid, iterations: usize) {
    let height = grid.len();
    let width = grid[0].len();

    for _ in 0..iterations {
        let mut next_grid = grid.clone();

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let solid_neighbors = count_solid_neighbors(grid, x, y);

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
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if is_solid_tile(&grid[ny][nx].tile_type) {
                count += 1;
            }
        }
    }
    count
}
