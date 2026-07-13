//! Monster sprite generation
//!
//! Generates all dungeon creature sprites using 3D shading.

use super::core::*;
use image::RgbaImage;

mod basic;
mod undead;
mod demon;
mod beast;

pub use basic::*;
pub use undead::*;
pub use demon::*;
pub use beast::*;

// ============================================================================
// SAVE FUNCTION
// ============================================================================

pub fn save_sprite(category: &str, name: &str, img: RgbaImage) {
    let path = format!("assets/sprites/{}/{}.png", category, name);
    img.save(&path).unwrap();
    println!("Generated: {}", path);
}
