//! Tile grid operations and isometric projection
//! Stateless functions for grid creation, coordinate conversion, and neighbor detection.

use crate::state::tile_state::{TilePos, TileState, FogState};
use crate::data::GameData;
use crate::engine::tile_types::{self, types as tt};
use std::collections::HashSet;
use macroquad::camera::Camera;

pub type Grid = Vec<Vec<TileState>>;

/// Create a new grid with the specified dimensions
pub fn create_grid(width: usize, height: usize, game_data: &GameData) -> Grid {
    let mut grid = Vec::with_capacity(height);

    for y in 0..height {
        let mut row = Vec::with_capacity(width);
        for x in 0..width {
            let pos = TilePos::new(x as i32, y as i32);

            // Default to earth for most tiles, solid rock for edges
            let tile_type = if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                "solid_rock".to_string()
            } else {
                // 10% chance of gold vein
                if rand::random::<f32>() < 0.10 {
                    "gold_vein".to_string()
                } else {
                    "earth".to_string()
                }
            };

            let mut tile = TileState::new(tile_type.clone(), pos);

            // Set up resources for resource tiles
            if let Some(tile_data) = game_data.tiles.get(&tile_type) {
                if let Some(resources) = &tile_data.resources {
                    if resources.amount > 0 {
                        tile = tile.with_resources(resources.amount as u32);
                    }
                }
            }

            // Gold veins have gold resources
            if tile_type == "gold_vein" {
                tile = tile.with_resources(100); // 100 gold per vein
            }

            row.push(tile);
        }
        grid.push(row);
    }

    grid
}

/// Convert world coordinates to isometric screen coordinates
pub fn world_to_iso(x: f32, y: f32, tile_width: f32, tile_height: f32) -> (f32, f32) {
    let iso_x = (x - y) * tile_width / 2.0;
    let iso_y = (x + y) * tile_height / 2.0;
    (iso_x, iso_y)
}

/// Convert isometric screen coordinates to world coordinates
pub fn iso_to_world(iso_x: f32, iso_y: f32, tile_width: f32, tile_height: f32) -> (f32, f32) {
    let x = (iso_x / (tile_width / 2.0) + iso_y / (tile_height / 2.0)) / 2.0;
    let y = (iso_y / (tile_height / 2.0) - iso_x / (tile_width / 2.0)) / 2.0;
    (x, y)
}

/// Convert screen position to tile position (considering camera offset)
/// Convert screen position to tile position using 3D raycasting
pub fn screen_to_tile(
    mouse_x: f32,
    mouse_y: f32,
    camera: &macroquad::camera::Camera3D,
    _tile_width: f32,  // Unused in 3D physics
    _tile_height: f32, // Unused in 3D physics
) -> TilePos {
    // 1. Convert mouse coordinates to world space ray
    // We can use the camera's matrices to unproject the mouse position
    
    // Normalized device coordinates (-1 to 1)
    let ndc_x = (mouse_x / macroquad::window::screen_width()) * 2.0 - 1.0;
    let ndc_y = 1.0 - (mouse_y / macroquad::window::screen_height()) * 2.0; // Flip Y for 3D

    // View-Projection Matrix Inverse
    let proj = camera.matrix();
    let view = macroquad::math::Mat4::look_at_rh(
        camera.position,
        camera.target,
        camera.up,
    );
    let inv_vp = (proj * view).inverse();

    // Ray start (Near plane) and end (Far plane)
    let ray_start = inv_vp.transform_point3(macroquad::math::vec3(ndc_x, ndc_y, -1.0));
    let ray_end = inv_vp.transform_point3(macroquad::math::vec3(ndc_x, ndc_y, 1.0));

    let ray_dir = (ray_end - ray_start).normalize();

    // 2. Intersect ray with ground plane (y = 0 in 3D world, but we might want our tiles to be at y=0)
    // IMPORTANT: In our 3D setup:
    // x = grid x
    // z = grid y
    // y = height (0 = floor)
    
    // Ray: P = P0 + t * D
    // Plane: y = 0
    // 0 = P0.y + t * D.y
    // t = -P0.y / D.y

    if ray_dir.y.abs() < 0.0001 {
        // Ray is parallel to plane, return origin or something safe
        return TilePos::new(0, 0);
    }

    let t = -ray_start.y / ray_dir.y;

    if t < 0.0 {
        // Intersection is behind camera
        return TilePos::new(0, 0);
    }

    let intersect_point = ray_start + ray_dir * t;

    // 3. Convert intersection point to integer tile coordinates
    // Assuming 1 unit = 1 tile in our 3D world for simplicity
    // If we used a scale factor, we'd divide by it here
    
    TilePos::new(
        intersect_point.x.round() as i32,
        intersect_point.z.round() as i32,
    )
}

/// Get a tile from the grid at the specified position
pub fn get_tile(grid: &Grid, pos: TilePos) -> Option<&TileState> {
    if pos.y >= 0 && (pos.y as usize) < grid.len() {
        let row = &grid[pos.y as usize];
        if pos.x >= 0 && (pos.x as usize) < row.len() {
            return Some(&row[pos.x as usize]);
        }
    }
    None
}

/// Get a mutable reference to a tile from the grid
pub fn get_tile_mut(grid: &mut Grid, pos: TilePos) -> Option<&mut TileState> {
    if pos.y >= 0 && (pos.y as usize) < grid.len() {
        let row = &mut grid[pos.y as usize];
        if pos.x >= 0 && (pos.x as usize) < row.len() {
            return Some(&mut row[pos.x as usize]);
        }
    }
    None
}

/// Get all 8 neighbors of a tile (including diagonals)
pub fn get_neighbors(grid: &Grid, pos: TilePos) -> Vec<TilePos> {
    let mut neighbors = Vec::new();

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let neighbor_pos = TilePos::new(pos.x + dx, pos.y + dy);
            if get_tile(grid, neighbor_pos).is_some() {
                neighbors.push(neighbor_pos);
            }
        }
    }

    neighbors
}

/// Get the 4 cardinal neighbors (no diagonals)
pub fn get_cardinal_neighbors(grid: &Grid, pos: TilePos) -> Vec<TilePos> {
    let offsets = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut neighbors = Vec::new();

    for (dx, dy) in offsets.iter() {
        let neighbor_pos = TilePos::new(pos.x + dx, pos.y + dy);
        if get_tile(grid, neighbor_pos).is_some() {
            neighbors.push(neighbor_pos);
        }
    }

    neighbors
}

/// Calculate fog state for a tile based on player vision
pub fn calculate_fog_state(
    tile: &TileState,
    player_vision: &HashSet<TilePos>,
) -> FogState {
    if player_vision.contains(&tile.pos) {
        FogState::Visible
    } else if tile.fog_state == FogState::Visible {
        FogState::Revealed
    } else {
        tile.fog_state
    }
}

/// Check if there's a clear line of sight between two positions (not blocked by solid tiles)
fn has_line_of_sight(grid: &Grid, from: TilePos, to: TilePos) -> bool {
    // Use Bresenham's line algorithm to check tiles between from and to
    let mut x0 = from.x;
    let mut y0 = from.y;
    let x1 = to.x;
    let y1 = to.y;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        // Check if current tile blocks vision (skip the start position)
        if (x0, y0) != (from.x, from.y) {
            let pos = TilePos::new(x0, y0);
            if let Some(tile) = get_tile(grid, pos) {
                // Solid tiles block vision
                if tile_types::blocks_vision(&tile.tile_type) {
                    return false;
                }
            }
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }

    true
}

/// Update fog of war based on claimed tiles and creature positions
pub fn update_fog_of_war(
    grid: &mut Grid,
    claimed_tiles: &HashSet<TilePos>,
    creature_positions: &[TilePos],
    sight_radius: i32,
) {
    let mut visible_tiles = HashSet::new();

    // Add all claimed tiles to visible
    for pos in claimed_tiles {
        visible_tiles.insert(*pos);
    }

    // Create a read-only snapshot of tile types for line-of-sight checks
    let tile_types: Vec<Vec<String>> = grid
        .iter()
        .map(|row| row.iter().map(|t| t.tile_type.clone()).collect())
        .collect();

    // Add tiles around creatures (with line-of-sight check)
    for creature_pos in creature_positions {
        for dy in -sight_radius..=sight_radius {
            for dx in -sight_radius..=sight_radius {
                let distance = (dx * dx + dy * dy) as f32;
                if distance <= (sight_radius * sight_radius) as f32 {
                    let tile_pos = TilePos::new(creature_pos.x + dx, creature_pos.y + dy);
                    
                    // Check line of sight using the snapshot
                    if has_line_of_sight_snapshot(&tile_types, *creature_pos, tile_pos) {
                        visible_tiles.insert(tile_pos);
                    }
                }
            }
        }
    }

    // Update fog states
    for row in grid.iter_mut() {
        for tile in row.iter_mut() {
            tile.fog_state = calculate_fog_state(tile, &visible_tiles);
        }
    }
}

/// Check line of sight using a snapshot of tile types (to avoid borrow issues)
fn has_line_of_sight_snapshot(tile_types: &[Vec<String>], from: TilePos, to: TilePos) -> bool {
    let mut x0 = from.x;
    let mut y0 = from.y;
    let x1 = to.x;
    let y1 = to.y;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        // Check if current tile blocks vision (skip the start position)
        if (x0, y0) != (from.x, from.y) {
            if y0 >= 0 && (y0 as usize) < tile_types.len() {
                let row = &tile_types[y0 as usize];
                if x0 >= 0 && (x0 as usize) < row.len() {
                    let tile_type = &row[x0 as usize];
                    // Solid tiles block vision
                    if tile_types::blocks_vision(tile_type) {
                        return false;
                    }
                }
            }
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }

    true
}

/// Get the dimensions of the grid
pub fn get_grid_dimensions(grid: &Grid) -> (usize, usize) {
    if grid.is_empty() {
        (0, 0)
    } else {
        (grid[0].len(), grid.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_to_iso_roundtrip() {
        let tile_width = 64.0;
        let tile_height = 32.0;

        let (iso_x, iso_y) = world_to_iso(5.0, 3.0, tile_width, tile_height);
        let (world_x, world_y) = iso_to_world(iso_x, iso_y, tile_width, tile_height);

        assert!((world_x - 5.0).abs() < 0.001);
        assert!((world_y - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_neighbor_detection() {
        use crate::data::GameData;
        use std::collections::HashMap;

        let game_data = GameData {
            tiles: HashMap::new(),
            rooms: HashMap::new(),
            monsters: HashMap::new(),
            heroes: HashMap::new(),
            spells: HashMap::new(),
        };

        let grid = create_grid(5, 5, &game_data);
        let center = TilePos::new(2, 2);

        let neighbors = get_neighbors(&grid, center);
        assert_eq!(neighbors.len(), 8);

        let cardinal = get_cardinal_neighbors(&grid, center);
        assert_eq!(cardinal.len(), 4);
    }

    #[test]
    fn test_tile_pos_distance() {
        let pos1 = TilePos::new(0, 0);
        let pos2 = TilePos::new(3, 4);

        assert_eq!(pos1.manhattan_distance(&pos2), 7);
        assert!((pos1.distance_to(&pos2) - 5.0).abs() < 0.001);
    }
}
