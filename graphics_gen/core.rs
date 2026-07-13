//! Core 3D rendering infrastructure for procedural graphics generation
//!
//! This module provides:
//! - Material system with various presets (matte, metallic, glowing, etc.)
//! - Depth buffer for proper 3D overlap
//! - 3D primitive drawing functions (sphere, ellipsoid, cylinder, cone, torus, box)
//! - Shading with Blinn-Phong lighting model

mod material;
mod noise;
mod primitives;
mod walls;

pub use material::*;
pub use noise::*;
pub use primitives::*;
pub use walls::*;

// ============================================================================
// CONSTANTS
// ============================================================================

pub const TILE_SIZE: u32 = 64;
pub const TILE_WIDTH: u32 = 64;
pub const TILE_HEIGHT: u32 = 64;
pub const SPRITE_SIZE: u32 = 64;
pub const PROJECTILE_SIZE: u32 = 32;

// Projection constants
pub const TILT: f32 = 0.5; // Z-axis foreshortening factor (0.5 means 1 unit up = 0.5 units up-screen)

// ============================================================================
// 3D PRIMITIVE DRAWING
// ============================================================================

// Re-export primitives moved to separate module
pub use crate::graphics_gen::primitives::*;
