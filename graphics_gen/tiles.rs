//! Tile generation for dungeon floors, walls, and room types
//!
//! Generates all ground tiles, room floors, and trap tiles.

use image::RgbaImage;

mod flooring;
mod hazards;
mod rooms;
mod terrain;
mod traps;

pub use flooring::*;
pub use hazards::*;
pub use rooms::*;
pub use terrain::*;
pub use traps::*;

// ============================================================================
// SAVE FUNCTION
// ============================================================================

pub fn save_tile(name: &str, img: RgbaImage) {
    let path = format!("assets/tiles/{}.png", name);
    img.save(&path).unwrap();
    println!("Generated: {}", path);
}
