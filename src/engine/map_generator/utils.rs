//! Utility functions for map generation

use crate::state::tile_state::TilePos;

/// Calculate distance between two tile positions
pub fn distance_f32(a: TilePos, b: TilePos) -> f32 {
    let dx = (a.x - b.x) as f32;
    let dy = (a.y - b.y) as f32;
    (dx * dx + dy * dy).sqrt()
}
