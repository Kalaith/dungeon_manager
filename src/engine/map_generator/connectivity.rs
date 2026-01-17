//! Connectivity validation
//! Ensures all open regions are connected via flood fill and tunnel carving

use crate::state::tile_state::TilePos;
use macroquad::rand;
use std::collections::VecDeque;

use super::config::Grid;
use super::terrain::is_solid_tile;
use super::utils::distance_f32;

// ============================================================================
// CONNECTIVITY VALIDATION
// ============================================================================

/// Ensure all open regions are connected via tunnels
pub fn ensure_connectivity(grid: &mut Grid) {
    let regions = find_disconnected_regions(grid);

    if regions.is_empty() || regions.len() == 1 {
        return;
    }

    // Connect all regions to the largest one
    let largest_region = &regions[0];
    for i in 1..regions.len() {
        connect_regions(grid, largest_region, &regions[i]);
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

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

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
fn connect_regions(grid: &mut Grid, region_a: &[TilePos], region_b: &[TilePos]) {
    let (start, end) = find_closest_points(region_a, region_b);
    carve_tunnel(grid, start, end);
}

/// Find the two closest points between two regions
fn find_closest_points(region_a: &[TilePos], region_b: &[TilePos]) -> (TilePos, TilePos) {
    let mut min_dist = f32::INFINITY;
    let mut best_pair = (region_a[0], region_b[0]);

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
fn carve_tunnel(grid: &mut Grid, start: TilePos, end: TilePos) {
    let width = 1;
    let horizontal_first = rand::gen_range(0u32, 2) == 0;

    if horizontal_first {
        carve_horizontal_line(grid, start.x, end.x, start.y, width);
        carve_vertical_line(grid, end.x, start.y, end.y, width);
    } else {
        carve_vertical_line(grid, start.x, start.y, end.y, width);
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
