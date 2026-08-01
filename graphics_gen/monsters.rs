//! Monster sprite generation
//!
//! Generates all dungeon creature sprites using 3D shading.

use super::core::*;
use image::RgbaImage;

mod basic;
mod beast;
mod demon;
mod tank;
mod undead;

pub use basic::*;
pub use beast::*;
pub use demon::*;
pub use tank::*;
pub use undead::*;

// ============================================================================
// SAVE FUNCTION
// ============================================================================

pub fn save_sprite(category: &str, name: &str, img: RgbaImage) {
    let path = format!("assets/sprites/{}/{}.png", category, name);
    img.save(&path).unwrap();
    println!("Generated: {}", path);
}
