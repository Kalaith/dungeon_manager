//! Hero sprite generation
//!
//! Generates all hero character sprites using 3D shading.

use image::RgbaImage;

mod tier1;
mod tier2;
mod tier3;
mod tier4;

pub use tier1::*;
pub use tier2::*;
pub use tier3::*;
pub use tier4::*;

// ============================================================================
// SAVE FUNCTION
// ============================================================================

pub fn save_sprite(category: &str, name: &str, img: RgbaImage) {
    let path = format!("assets/sprites/{}/{}.png", category, name);
    img.save(&path).unwrap();
    println!("Generated: {}", path);
}
