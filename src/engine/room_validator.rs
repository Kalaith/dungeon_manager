//! Room validation and quality calculation
//! Stateless service for detecting rooms, validating shapes, and calculating quality

use crate::data::rooms::RoomData;
use crate::engine::tile_grid::{get_tile, Grid};
use crate::engine::tile_types;
use crate::state::tile_state::{Ownership, TilePos};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};

/// A detected room instance in the game world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: usize,
    pub room_type: String,
    pub tiles: HashSet<TilePos>,
    pub quality: f32,
    pub efficiency: f32,
    pub active: bool,
}

impl Room {
    /// Create a new room instance
    pub fn new(id: usize, room_type: String, tiles: HashSet<TilePos>) -> Self {
        Self {
            id,
            room_type,
            tiles,
            quality: 0.0,
            efficiency: 1.0,
            active: false,
        }
    }

    /// Get the center position of the room (average of all tile positions)
    pub fn get_center(&self) -> TilePos {
        if self.tiles.is_empty() {
            return TilePos::new(0, 0);
        }

        let sum_x: i32 = self.tiles.iter().map(|pos| pos.x).sum();
        let sum_y: i32 = self.tiles.iter().map(|pos| pos.y).sum();
        let count = self.tiles.len() as i32;

        TilePos::new(sum_x / count, sum_y / count)
    }

    /// Get the number of tiles in this room
    pub fn size(&self) -> usize {
        self.tiles.len()
    }
}

/// Metrics describing the shape of a room
#[derive(Debug, Clone)]
pub struct ShapeMetrics {
    /// Width to height ratio (or inverse if height is larger)
    pub aspect_ratio: f32,
    /// How circular/square the room is (0.0 = line, 1.0 = perfect circle)
    pub compactness: f32,
    /// Whether the room has disconnected regions
    pub is_fragmented: bool,
    /// Whether the room is a thin corridor (aspect ratio > 4:1)
    pub is_thin_corridor: bool,
}

/// Detect all rooms of a specific type in the grid
/// Uses flood-fill to find contiguous regions of claimed tiles
pub fn detect_rooms(grid: &Grid, room_type: &str) -> Vec<HashSet<TilePos>> {
    let mut visited = HashSet::new();
    let mut rooms = Vec::new();

    // Scan grid for tiles of this room type
    for row in grid {
        for tile in row {
            // Skip if already visited or not matching room type
            if visited.contains(&tile.pos) {
                continue;
            }

            // Only detect rooms on tiles that match the room type
            if tile.tile_type == room_type {
                if tile.ownership == Ownership::Player {
                    let room_tiles = flood_fill(grid, tile.pos, room_type, &mut visited);
                    if !room_tiles.is_empty() {
                        rooms.push(room_tiles);
                    }
                }
            }
        }
    }

    rooms
}

/// Calculate the efficiency of a room based on its enclosure by walls/doors
/// Returns a value between 0.0 and 1.0
pub fn calculate_efficiency(room: &Room, grid: &Grid) -> f32 {
    let mut total_perimeter_segments = 0.0;
    let mut secured_segments = 0.0;
    
    // Check neighbors for each tile in the room
    for tile_pos in &room.tiles {
        let neighbors = [
            TilePos::new(tile_pos.x + 1, tile_pos.y),
            TilePos::new(tile_pos.x - 1, tile_pos.y),
            TilePos::new(tile_pos.x, tile_pos.y + 1),
            TilePos::new(tile_pos.x, tile_pos.y - 1),
        ];

        for neighbor in neighbors {
            // If neighbor is NOT in the room, it's a perimeter segment
            if !room.tiles.contains(&neighbor) {
                total_perimeter_segments += 1.0;
                
                // Check if this perimeter segment is secured (Wall or Door)
                // We need to check bounds first
                if neighbor.x >= 0 && neighbor.y >= 0 && 
                   (neighbor.y as usize) < grid.len() && 
                   (neighbor.x as usize) < grid[0].len() {
                    let tile = &grid[neighbor.y as usize][neighbor.x as usize];
                    // Walls (rock/earth marked as wall?) and Doors count as secured
                    // "wall", "reinforced_wall", "door", "rock" (unmined) are valid
                    if tile_types::is_secure(&tile.tile_type) {
                        secured_segments += 1.0;
                    }
                } else {
                    // Map edge counts as secured (indestructible bedrock usually)
                    secured_segments += 1.0;
                }
            }
        }
    }

    if total_perimeter_segments == 0.0 {
        return 1.0; // Should not happen for valid rooms
    }

    // Calculate ratio
    secured_segments / total_perimeter_segments
}

/// Flood fill algorithm to find all contiguous tiles from a starting position
fn flood_fill(grid: &Grid, start: TilePos, target_type: &str, visited: &mut HashSet<TilePos>) -> HashSet<TilePos> {
    let mut result = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        // Skip if already visited
        if visited.contains(&pos) {
            continue;
        }

        // Check if this tile exists and is valid
        if let Some(tile) = get_tile(grid, pos) {
            // Only include player-owned claimed tiles of the same type
            if tile.ownership != Ownership::Player || tile.tile_type != target_type {
                continue;
            }

            // Mark as visited and add to result
            visited.insert(pos);
            result.insert(pos);

            // Add neighbors to queue (4-way connectivity for rooms)
            let neighbors = [
                TilePos::new(pos.x + 1, pos.y),
                TilePos::new(pos.x - 1, pos.y),
                TilePos::new(pos.x, pos.y + 1),
                TilePos::new(pos.x, pos.y - 1),
            ];

            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    result
}

/// Validate that a set of tiles meets the requirements for a room type
pub fn validate_room_shape(
    tiles: &HashSet<TilePos>,
    room_data: &RoomData,
) -> Result<(), String> {
    let size = tiles.len() as u32;

    // Check minimum size
    if size < room_data.build.min_tiles {
        return Err(format!(
            "Room too small: {} tiles (minimum: {})",
            size, room_data.build.min_tiles
        ));
    }

    // Check maximum size
    if size > room_data.build.max_tiles {
        return Err(format!(
            "Room too large: {} tiles (maximum: {})",
            size, room_data.build.max_tiles
        ));
    }

    // Check if tiles are contiguous (no fragmentation)
    if !is_contiguous(tiles) {
        return Err("Room tiles must be contiguous".to_string());
    }

    Ok(())
}

/// Check if a set of tiles forms a contiguous region
fn is_contiguous(tiles: &HashSet<TilePos>) -> bool {
    if tiles.is_empty() {
        return false;
    }

    // Start flood fill from first tile
    let start = *tiles.iter().next().unwrap();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        // Check 4-way neighbors
        let neighbors = [
            TilePos::new(pos.x + 1, pos.y),
            TilePos::new(pos.x - 1, pos.y),
            TilePos::new(pos.x, pos.y + 1),
            TilePos::new(pos.x, pos.y - 1),
        ];

        for neighbor in neighbors {
            if tiles.contains(&neighbor) && !visited.contains(&neighbor) {
                queue.push_back(neighbor);
            }
        }
    }

    // All tiles should be reachable from the start
    visited.len() == tiles.len()
}

/// Calculate shape metrics for a set of tiles
pub fn calculate_shape_metrics(tiles: &HashSet<TilePos>) -> ShapeMetrics {
    if tiles.is_empty() {
        return ShapeMetrics {
            aspect_ratio: 1.0,
            compactness: 0.0,
            is_fragmented: true,
            is_thin_corridor: false,
        };
    }

    // Calculate bounding box
    let min_x = tiles.iter().map(|p| p.x).min().unwrap();
    let max_x = tiles.iter().map(|p| p.x).max().unwrap();
    let min_y = tiles.iter().map(|p| p.y).min().unwrap();
    let max_y = tiles.iter().map(|p| p.y).max().unwrap();

    let width = (max_x - min_x + 1) as f32;
    let height = (max_y - min_y + 1) as f32;

    // Calculate aspect ratio (always >= 1.0)
    let aspect_ratio = if width > height {
        width / height
    } else {
        height / width
    };

    // Calculate compactness (ratio of actual tiles to bounding box)
    let bounding_box_area = width * height;
    let actual_area = tiles.len() as f32;
    let compactness = actual_area / bounding_box_area;

    // Check for thin corridor (aspect ratio > 4:1)
    let is_thin_corridor = aspect_ratio > 4.0;

    // Check for fragmentation
    let is_fragmented = !is_contiguous(tiles);

    ShapeMetrics {
        aspect_ratio,
        compactness,
        is_fragmented,
        is_thin_corridor,
    }
}

/// Calculate the quality of a room based on size and shape
pub fn calculate_room_quality(room: &Room, room_data: &RoomData) -> f32 {
    let size = room.tiles.len() as u32;
    let mut quality = 100.0; // Start at 100% base quality

    // Apply size threshold multipliers
    let mut size_multiplier = 1.0;
    for threshold in &room_data.scaling.size_thresholds {
        if size >= threshold.tiles {
            size_multiplier = threshold.multiplier;
        } else {
            break; // Thresholds should be sorted, stop at first unmet
        }
    }
    quality *= size_multiplier;

    // Calculate shape metrics
    let metrics = calculate_shape_metrics(&room.tiles);

    // Apply shape penalties
    if metrics.is_thin_corridor {
        if let Some(&penalty) = room_data.scaling.shape_penalties.get("thin_corridor") {
            quality *= penalty;
        }
    }

    if metrics.is_fragmented {
        if let Some(&penalty) = room_data.scaling.shape_penalties.get("fragmented") {
            quality *= penalty;
        }
    }

    // Apply compactness penalty for very sparse rooms
    if metrics.compactness < 0.5 {
        if let Some(&penalty) = room_data.scaling.shape_penalties.get("sparse") {
            quality *= penalty;
        } else {
            // Default penalty for sparse rooms
            quality *= 0.9;
        }
    }

    // Apply poor aspect ratio penalty
    if metrics.aspect_ratio > 3.0 && !metrics.is_thin_corridor {
        if let Some(&penalty) = room_data.scaling.shape_penalties.get("poor_aspect_ratio") {
            quality *= penalty;
        } else {
            // Default penalty
            quality *= 0.95;
        }
    }

    quality
}

/// Find the best room of a given type near a position
/// Returns (room_index, distance) or None if no suitable room found
pub fn find_nearest_room(
    rooms: &[Room],
    room_type: &str,
    pos: TilePos,
    min_quality: f32,
) -> Option<(usize, f32)> {
    let mut best: Option<(usize, f32)> = None;

    for (idx, room) in rooms.iter().enumerate() {
        // Check if room matches type and quality requirements
        if room.room_type != room_type || room.quality < min_quality || !room.active {
            continue;
        }

        // Calculate distance to room center
        let center = room.get_center();
        let dx = (center.x - pos.x) as f32;
        let dy = (center.y - pos.y) as f32;
        let distance = (dx * dx + dy * dy).sqrt();

        // Update best if this is closer
        // Update best if this is closer
        match best {
            None => best = Some((room.id, distance)),
            Some((_, best_dist)) if distance < best_dist => {
                best = Some((room.id, distance));
            }
            _ => {}
        }
    }

    best
}

/// Calculate task desirability for a room based on distance and quality
pub fn calculate_task_desirability(
    room: &Room,
    room_data: &RoomData,
    creature_pos: TilePos,
    base_desirability: f32,
) -> f32 {
    let center = room.get_center();
    let dx = (center.x - creature_pos.x) as f32;
    let dy = (center.y - creature_pos.y) as f32;
    let distance = (dx * dx + dy * dy).sqrt();

    // Desirability decreases with distance
    let distance_factor = 1.0 / (1.0 + distance * 0.1);

    // Quality multiplier (normalize to 0.5-1.5 range)
    let quality_factor = 0.5 + (room.quality / 200.0).min(1.0);

    base_desirability * distance_factor * quality_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_contiguous() {
        // Test a contiguous 2x2 square
        let mut tiles = HashSet::new();
        tiles.insert(TilePos::new(0, 0));
        tiles.insert(TilePos::new(1, 0));
        tiles.insert(TilePos::new(0, 1));
        tiles.insert(TilePos::new(1, 1));
        assert!(is_contiguous(&tiles));

        // Test a non-contiguous set
        let mut tiles = HashSet::new();
        tiles.insert(TilePos::new(0, 0));
        tiles.insert(TilePos::new(5, 5));
        assert!(!is_contiguous(&tiles));
    }

    #[test]
    fn test_shape_metrics() {
        // Test a 4x1 corridor (thin corridor)
        let mut tiles = HashSet::new();
        for x in 0..5 {
            tiles.insert(TilePos::new(x, 0));
        }
        let metrics = calculate_shape_metrics(&tiles);
        assert!(metrics.is_thin_corridor);
        assert!(metrics.aspect_ratio > 4.0);

        // Test a 3x3 square (good shape)
        let mut tiles = HashSet::new();
        for x in 0..3 {
            for y in 0..3 {
                tiles.insert(TilePos::new(x, y));
            }
        }
        let metrics = calculate_shape_metrics(&tiles);
        assert!(!metrics.is_thin_corridor);
        assert_eq!(metrics.compactness, 1.0); // Perfect square
    }

    #[test]
    fn test_room_center() {
        let mut tiles = HashSet::new();
        tiles.insert(TilePos::new(0, 0));
        tiles.insert(TilePos::new(2, 0));
        tiles.insert(TilePos::new(1, 1));

        let room = Room::new(0, "test".to_string(), tiles);
        let center = room.get_center();

        // Average: x = (0+2+1)/3 = 1, y = (0+0+1)/3 = 0
        assert_eq!(center.x, 1);
        assert_eq!(center.y, 0);
    }
}
