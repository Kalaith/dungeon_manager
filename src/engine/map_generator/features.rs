//! Natural features and hero portals

use crate::state::tile_state::{Ownership, TilePos};
use macroquad_toolkit::rng;
use std::collections::VecDeque;

use super::config::{Grid, MapConfig};
use super::terrain::is_solid_tile;
use super::utils::distance_f32;

// ============================================================================
// NATURAL FEATURES
// ============================================================================

/// Add natural geological features to the map
pub fn add_natural_features(grid: &mut Grid, config: &MapConfig) {
    add_stone_pillars(grid, config.num_stone_pillars);
    add_collapsed_chambers(grid, config.num_collapsed_chambers);
}

fn add_stone_pillars(grid: &mut Grid, count: usize) {
    let height = grid.len();
    let width = grid[0].len();

    for _ in 0..count {
        let x = rng::gen_range(8, width - 8);
        let y = rng::gen_range(8, height - 8);
        let radius = rng::gen_range(1, 3);

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

fn add_collapsed_chambers(grid: &mut Grid, count: usize) {
    let height = grid.len();
    let width = grid[0].len();

    for _ in 0..count {
        let cx = rng::gen_range(12, width - 12);
        let cy = rng::gen_range(12, height - 12);
        let size = rng::gen_range(5, 10);
        create_collapsed_chamber(grid, cx, cy, size);
    }
}

fn create_collapsed_chamber(grid: &mut Grid, cx: usize, cy: usize, size: usize) {
    let height = grid.len();
    let width = grid[0].len();

    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    queue.push_back((cx, cy));
    visited[cy][cx] = true;

    let mut tiles_placed = 0;
    let max_tiles = size * size;

    while let Some((x, y)) = queue.pop_front() {
        if tiles_placed >= max_tiles {
            break;
        }

        if grid[y][x].tile_type == "solid_rock" {
            grid[y][x].tile_type = "earth".to_string();
            tiles_placed += 1;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;

            if nx > 5
                && nx < width - 5
                && ny > 5
                && ny < height - 5
                && !visited[ny][nx]
                && rng::gen_range(0.0f32, 1.0) < 0.75
            {
                visited[ny][nx] = true;
                queue.push_back((nx, ny));
            }
        }
    }
}

// ============================================================================
// MONSTER LAIR PLACEMENT
// ============================================================================

/// Add underground monster lairs with spawners
pub fn add_monster_lairs(grid: &mut Grid, start_pos: TilePos, _config: &MapConfig) {
    let height = grid.len();
    let width = grid[0].len();
    let target_lairs = rng::gen_range(3u32, 6); // Aim for 3-5 lairs
    let mut min_distance = 25.0; // Initial strict distance

    let mut lairs_placed = 0;
    let mut attempts = 0;

    // First pass: Try to place target number with strict rules
    while lairs_placed < target_lairs && attempts < 100 {
        attempts += 1;
        if try_place_lair(grid, width, height, start_pos, min_distance) {
            lairs_placed += 1;
        }
    }

    // Second pass: Ensure at least 2 lairs with relaxed rules if needed
    let mut fallback_attempts = 0;
    while lairs_placed < 2 && fallback_attempts < 200 {
        fallback_attempts += 1;
        min_distance *= 0.8; // Reduce distance requirement
        if min_distance < 10.0 {
            min_distance = 10.0;
        } // Hard limit

        if try_place_lair(grid, width, height, start_pos, min_distance) {
            lairs_placed += 1;
        }
    }
}

fn try_place_lair(
    grid: &mut Grid,
    width: usize,
    height: usize,
    start_pos: TilePos,
    min_dist: f32,
) -> bool {
    let cx = rng::gen_range(10, width - 10);
    let cy = rng::gen_range(10, height - 10);
    let pos = TilePos::new(cx as i32, cy as i32);

    if distance_f32(pos, start_pos) < min_dist {
        return false;
    }

    // Check if area is solid (we want to carve out of rock)
    if is_open_area(grid, pos, 3) {
        return false;
    }

    // Carve the lair
    create_monster_lair(grid, cx, cy);
    true
}

fn create_monster_lair(grid: &mut Grid, cx: usize, cy: usize) {
    let height = grid.len();
    let width = grid[0].len();
    let radius = rng::gen_range(2i32, 4);

    // Carve room
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                let x = (cx as i32 + dx).max(1).min(width as i32 - 2) as usize;
                let y = (cy as i32 + dy).max(1).min(height as i32 - 2) as usize;

                // Use corrupted floor for monster lairs
                grid[y][x].tile_type = "corrupted_floor".to_string();
                grid[y][x].ownership = Ownership::Enemy;
                grid[y][x].resources_remaining = None;
            }
        }
    }

    // Place spawner in center
    grid[cy][cx].tile_type = "monster_spawner".to_string();
    grid[cy][cx].ownership = Ownership::Enemy;

    // Add some random gold/treasure around
    for _ in 0..3 {
        let dx = rng::gen_range(-radius, radius + 1);
        let dy = rng::gen_range(-radius, radius + 1);
        let x = (cx as i32 + dx).max(1).min(width as i32 - 2) as usize;
        let y = (cy as i32 + dy).max(1).min(height as i32 - 2) as usize;

        if grid[y][x].tile_type == "corrupted_floor"
            && (dx != 0 || dy != 0)
            && rng::gen_range(0.0f32, 1.0) < 0.3
        {
            grid[y][x].tile_type = "gold_vein".to_string();
            grid[y][x].resources_remaining = Some(rng::gen_range(100u32, 300));
        }
    }
}

pub fn place_hero_portals(grid: &mut Grid, start_pos: TilePos, config: &MapConfig) {
    let height = grid.len();
    let width = grid[0].len();
    let mut portal_positions: Vec<TilePos> = Vec::new();

    let candidates = find_portal_candidates(grid, start_pos, config.min_portal_distance);
    if candidates.is_empty() {
        return;
    }

    for _ in 0..config.num_hero_portals {
        if let Some(pos) = select_best_portal_location(&candidates, &portal_positions) {
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
            if distance_f32(pos, start_pos) < min_distance {
                continue;
            }
            if !is_open_area(grid, pos, 2) {
                continue;
            }
            if is_solid_tile(&grid[y][x].tile_type) {
                continue;
            }
            candidates.push(pos);
        }
    }

    candidates
}

fn select_best_portal_location(
    candidates: &[TilePos],
    existing_portals: &[TilePos],
) -> Option<TilePos> {
    if candidates.is_empty() {
        return None;
    }
    if existing_portals.is_empty() {
        return Some(candidates[rng::gen_range(0, candidates.len())]);
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
        if far_enough {
            best_candidates.push(*candidate);
        }
    }

    if best_candidates.is_empty() {
        Some(candidates[rng::gen_range(0, candidates.len())])
    } else {
        Some(best_candidates[rng::gen_range(0, best_candidates.len())])
    }
}
