//! Natural features and hero portals

use crate::state::tile_state::{Ownership, TilePos};
use rand::Rng;
use std::collections::VecDeque;

use super::config::{Grid, MapConfig};
use super::terrain::is_solid_tile;
use super::utils::distance_f32;

// ============================================================================
// NATURAL FEATURES
// ============================================================================

/// Add natural geological features to the map
pub fn add_natural_features(grid: &mut Grid, config: &MapConfig, rng: &mut impl Rng) {
    add_stone_pillars(grid, config.num_stone_pillars, rng);
    add_collapsed_chambers(grid, config.num_collapsed_chambers, rng);
}

fn add_stone_pillars(grid: &mut Grid, count: usize, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();

    for _ in 0..count {
        let x = rng.gen_range(8..width - 8);
        let y = rng.gen_range(8..height - 8);
        let radius = rng.gen_range(1..3);

        if is_open_area(grid, TilePos::new(x as i32, y as i32), radius * 2 + 1) {
            create_pillar(grid, x, y, radius);
        }
    }
}

/// Check if an area is mostly open
pub fn is_open_area(grid: &Grid, center: TilePos, radius: usize) -> bool {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let r = radius as i32;
    let mut open_count = 0;
    let mut total_count = 0;

    for dy in -r..=r {
        for dx in -r..=r {
            let nx = center.x + dx;
            let ny = center.y + dy;
            if nx > 0 && nx < width - 1 && ny > 0 && ny < height - 1 {
                total_count += 1;
                if !is_solid_tile(&grid[ny as usize][nx as usize].tile_type) {
                    open_count += 1;
                }
            }
        }
    }

    total_count > 0 && (open_count as f32 / total_count as f32) > 0.7
}

fn create_pillar(grid: &mut Grid, cx: usize, cy: usize, radius: usize) {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let r = radius as i32;

    for dy in -r..=r {
        for dx in -r..=r {
            if dx * dx + dy * dy <= r * r {
                let x = (cx as i32 + dx).max(1).min(width - 2) as usize;
                let y = (cy as i32 + dy).max(1).min(height - 2) as usize;

                if grid[y][x].ownership != Ownership::Player {
                    grid[y][x].tile_type = "solid_rock".to_string();
                }
            }
        }
    }
}

fn add_collapsed_chambers(grid: &mut Grid, count: usize, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();

    for _ in 0..count {
        let cx = rng.gen_range(12..width - 12);
        let cy = rng.gen_range(12..height - 12);
        let size = rng.gen_range(5..10);
        create_collapsed_chamber(grid, cx, cy, size, rng);
    }
}

fn create_collapsed_chamber(grid: &mut Grid, cx: usize, cy: usize, size: usize, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();

    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    queue.push_back((cx, cy));
    visited[cy][cx] = true;

    let mut tiles_placed = 0;
    let max_tiles = size * size;

    while let Some((x, y)) = queue.pop_front() {
        if tiles_placed >= max_tiles { break; }

        if grid[y][x].tile_type == "solid_rock" {
            grid[y][x].tile_type = "earth".to_string();
            tiles_placed += 1;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;

            if nx > 5 && nx < width - 5 && ny > 5 && ny < height - 5 {
                if !visited[ny][nx] && rng.gen::<f32>() < 0.75 {
                    visited[ny][nx] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }
}

// ============================================================================
// HERO PORTAL PLACEMENT
// ============================================================================

/// Place hero spawn portals strategically around the map
pub fn place_hero_portals(grid: &mut Grid, start_pos: TilePos, config: &MapConfig, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();
    let mut portal_positions: Vec<TilePos> = Vec::new();

    let candidates = find_portal_candidates(grid, start_pos, config.min_portal_distance);
    if candidates.is_empty() { return; }

    for _ in 0..config.num_hero_portals {
        if let Some(pos) = select_best_portal_location(&candidates, &portal_positions, rng) {
            let px = pos.x as usize;
            let py = pos.y as usize;
            if px > 0 && px < width - 1 && py > 0 && py < height - 1 {
                grid[py][px].tile_type = "hero_portal".to_string();
                portal_positions.push(pos);
            }
        }
    }
}

fn find_portal_candidates(grid: &Grid, start_pos: TilePos, min_distance: f32) -> Vec<TilePos> {
    let height = grid.len();
    let width = grid[0].len();
    let mut candidates = Vec::new();

    for y in 5..height - 5 {
        for x in 5..width - 5 {
            let pos = TilePos::new(x as i32, y as i32);
            if distance_f32(pos, start_pos) < min_distance { continue; }
            if !is_open_area(grid, pos, 2) { continue; }
            if is_solid_tile(&grid[y][x].tile_type) { continue; }
            candidates.push(pos);
        }
    }

    candidates
}

fn select_best_portal_location(candidates: &[TilePos], existing_portals: &[TilePos], rng: &mut impl Rng) -> Option<TilePos> {
    if candidates.is_empty() { return None; }
    if existing_portals.is_empty() {
        return Some(candidates[rng.gen_range(0..candidates.len())]);
    }

    let min_portal_spacing = 15.0;
    let mut best_candidates: Vec<TilePos> = Vec::new();

    for candidate in candidates {
        let mut far_enough = true;
        for existing in existing_portals {
            if distance_f32(*candidate, *existing) < min_portal_spacing {
                far_enough = false;
                break;
            }
        }
        if far_enough { best_candidates.push(*candidate); }
    }

    if best_candidates.is_empty() {
        Some(candidates[rng.gen_range(0..candidates.len())])
    } else {
        Some(best_candidates[rng.gen_range(0..best_candidates.len())])
    }
}
