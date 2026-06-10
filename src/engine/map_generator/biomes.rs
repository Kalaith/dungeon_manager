//! Biome system - Voronoi-based biome regions

use crate::state::tile_state::Ownership;
use macroquad_toolkit::rng;

use super::config::{Biome, Grid};

// ============================================================================
// BIOME GENERATION
// ============================================================================

/// Generate a biome map using Voronoi-style regions
pub fn generate_biome_map(width: usize, height: usize, num_regions: usize) -> Vec<Vec<Biome>> {
    let center_x = width / 2;
    let center_y = height / 2;

    let mut biome_centers: Vec<(usize, usize, Biome)> = Vec::new();
    biome_centers.push((center_x, center_y, Biome::Standard));

    for _ in 0..num_regions {
        let x = rng::gen_range(5, width - 5);
        let y = rng::gen_range(5, height - 5);

        let dx = (x as i32 - center_x as i32).abs();
        let dy = (y as i32 - center_y as i32).abs();
        if dx < 15 && dy < 15 {
            continue;
        }

        biome_centers.push((x, y, random_biome()));
    }

    let mut biome_map = vec![vec![Biome::Standard; width]; height];

    for y in 0..height {
        for x in 0..width {
            let mut nearest_biome = Biome::Standard;
            let mut min_dist = f32::INFINITY;

            for (cx, cy, biome) in &biome_centers {
                let dist =
                    ((x as f32 - *cx as f32).powi(2) + (y as f32 - *cy as f32).powi(2)).sqrt();
                if dist < min_dist {
                    min_dist = dist;
                    nearest_biome = *biome;
                }
            }

            biome_map[y][x] = nearest_biome;
        }
    }

    biome_map
}

fn random_biome() -> Biome {
    match rng::gen_range(0u32, 5) {
        0 => Biome::Volcanic,
        1 => Biome::Crystalline,
        2 => Biome::Flooded,
        3 => Biome::Ancient,
        _ => Biome::Corrupted,
    }
}

/// Apply biome-specific features to the terrain
pub fn apply_biome_features(grid: &mut Grid, biome_map: &[Vec<Biome>]) {
    let height = grid.len();
    let width = grid[0].len();

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let biome = biome_map[y][x];
            let tile = &mut grid[y][x];

            if tile.ownership == Ownership::Player {
                continue;
            }

            match biome {
                Biome::Standard => {}
                Biome::Volcanic => {
                    if tile.tile_type == "earth" && rng::gen_range(0.0f32, 1.0) < 0.15 {
                        tile.tile_type = "lava".to_string();
                    }
                }
                Biome::Crystalline => {
                    if tile.tile_type == "solid_rock" && rng::gen_range(0.0f32, 1.0) < 0.12 {
                        tile.tile_type = "mana_crystal".to_string();
                        tile.resources_remaining = Some(rng::gen_range(200u32, 400));
                    }
                }
                Biome::Flooded => {
                    if tile.tile_type == "earth" && rng::gen_range(0.0f32, 1.0) < 0.20 {
                        tile.tile_type = "water".to_string();
                    }
                }
                Biome::Ancient => {
                    if tile.tile_type == "earth" && rng::gen_range(0.0f32, 1.0) < 0.10 {
                        tile.tile_type = "gem_seam".to_string();
                        tile.resources_remaining = Some(rng::gen_range(150u32, 300));
                    }
                }
                Biome::Corrupted => {
                    if tile.tile_type == "gold_vein" && rng::gen_range(0.0f32, 1.0) < 0.3 {
                        if let Some(res) = tile.resources_remaining.as_mut() {
                            *res = (*res / 2).max(20);
                        }
                    }
                }
            }
        }
    }
}
