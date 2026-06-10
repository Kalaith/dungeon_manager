//! Starting area generation
//! Supports procedural layouts based on difficulty

use crate::state::tile_state::{Ownership, TilePos};
use macroquad_toolkit::rng;

use super::config::{Difficulty, Grid, MapConfig, StartingLayout, StartingPosition};

/// Rotate offset by 0, 90, 180, or 270 degrees
fn rotate_offset(x: i32, y: i32, rotation: u8) -> (i32, i32) {
    match rotation % 4 {
        0 => (x, y),   // 0° - no rotation
        1 => (-y, x),  // 90° clockwise
        2 => (-x, -y), // 180°
        _ => (y, -x),  // 270° clockwise
    }
}

// ============================================================================
// STARTING POSITION
// ============================================================================

/// Calculate starting position based on strategy
pub fn calculate_starting_position(config: &MapConfig) -> TilePos {
    match config.starting_position {
        StartingPosition::Center => {
            TilePos::new((config.width / 2) as i32, (config.height / 2) as i32)
        }
        StartingPosition::Corner => {
            let corners = [
                TilePos::new(12, 12),
                TilePos::new((config.width - 12) as i32, 12),
                TilePos::new(12, (config.height - 12) as i32),
                TilePos::new((config.width - 12) as i32, (config.height - 12) as i32),
            ];
            corners[rng::gen_range(0, 4)]
        }
        StartingPosition::Edge => {
            let edge = rng::gen_range(0u32, 4);
            match edge {
                0 => TilePos::new(rng::gen_range(12, config.width as i32 - 12), 12),
                1 => TilePos::new(
                    (config.width - 12) as i32,
                    rng::gen_range(12, config.height as i32 - 12),
                ),
                2 => TilePos::new(
                    rng::gen_range(12, config.width as i32 - 12),
                    (config.height - 12) as i32,
                ),
                _ => TilePos::new(12, rng::gen_range(12, config.height as i32 - 12)),
            }
        }
        StartingPosition::Random => TilePos::new(
            rng::gen_range(15, config.width as i32 - 15),
            rng::gen_range(15, config.height as i32 - 15),
        ),
    }
}

// ============================================================================
// STARTING AREA GENERATION
// ============================================================================

/// Create the player's starting area based on difficulty
pub fn create_starting_area(grid: &mut Grid, config: &MapConfig, start_pos: TilePos) {
    let height = grid.len();
    let width = grid[0].len();
    let center_x = start_pos.x as usize;
    let center_y = start_pos.y as usize;

    // Get layout from difficulty
    let layout = StartingLayout::from_difficulty(config.difficulty);

    // Random rotation for variety (0, 90, 180, or 270 degrees)
    let rotation = rng::gen_range(0u8, 4);

    // Clear area around center
    let size = layout.cleared_area_size;
    for dy in -(size as i32)..=(size as i32) {
        for dx in -(size as i32)..=(size as i32) {
            let x = (center_x as i32 + dx).max(1).min(width as i32 - 2) as usize;
            let y = (center_y as i32 + dy).max(1).min(height as i32 - 2) as usize;

            grid[y][x].tile_type = "claimed_floor".to_string();
            grid[y][x].ownership = Ownership::Player;
            grid[y][x].resources_remaining = None;
        }
    }

    // Place dungeon heart at center
    grid[center_y][center_x].tile_type = "dungeon_heart".to_string();
    grid[center_y][center_x].ownership = Ownership::Player;

    // Create rooms from layout with rotation applied
    for room in &layout.rooms {
        // Apply rotation to offsets
        let (rotated_x, rotated_y) = rotate_offset(room.offset_x, room.offset_y, rotation);
        let room_x = (center_x as i32 + rotated_x).max(3).min(width as i32 - 4) as usize;
        let room_y = (center_y as i32 + rotated_y).max(3).min(height as i32 - 4) as usize;
        create_room(grid, room_x, room_y, room.size, &room.room_type);
    }

    // Add difficulty-specific bonuses
    apply_difficulty_bonuses(grid, config.difficulty, center_x, center_y);
}

/// Create a room at the specified position
fn create_room(grid: &mut Grid, cx: usize, cy: usize, size: usize, room_type: &str) {
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

/// Apply difficulty-specific bonuses (e.g., extra gold for Easy)
fn apply_difficulty_bonuses(grid: &mut Grid, difficulty: Difficulty, cx: usize, cy: usize) {
    match difficulty {
        Difficulty::Easy => {
            // Place some starting gold tiles nearby
            let gold_positions = [
                (cx.saturating_sub(3), cy.saturating_sub(3)),
                (cx + 3, cy.saturating_sub(3)),
                (cx.saturating_sub(3), cy + 3),
                (cx + 3, cy + 3),
            ];
            for (gx, gy) in gold_positions {
                if gx > 0 && gx < grid[0].len() - 1 && gy > 0 && gy < grid.len() - 1 {
                    if grid[gy][gx].tile_type == "claimed_floor" {
                        grid[gy][gx].tile_type = "gold_vein".to_string();
                        grid[gy][gx].resources_remaining = Some(100);
                    }
                }
            }
        }
        Difficulty::Nightmare => {
            // Nightmare: lava surrounds the starting area!
            let size = 5i32;
            for dy in -(size + 2)..=(size + 2) {
                for dx in -(size + 2)..=(size + 2) {
                    // Only on the edge ring
                    if dx.abs() == size + 2 || dy.abs() == size + 2 {
                        let x = (cx as i32 + dx).max(1).min(grid[0].len() as i32 - 2) as usize;
                        let y = (cy as i32 + dy).max(1).min(grid.len() as i32 - 2) as usize;
                        if grid[y][x].ownership != Ownership::Player {
                            grid[y][x].tile_type = "lava".to_string();
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
