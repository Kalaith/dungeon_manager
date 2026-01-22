//! Tile generation for dungeon floors, walls, and room types
//!
//! Generates all ground tiles, room floors, and trap tiles.

use image::{Rgba, RgbaImage};
use super::core::*;

// ============================================================================
// NATURAL TILES
// ============================================================================

pub fn create_solid_rock() -> RgbaImage {
    // Create a volumetric carved stone block wall
    let mut img = create_carved_block(
        Rgba([55, 55, 60, 255]),    // Base stone color
        16,                          // Block width
        12,                          // Block height
        Rgba([35, 35, 40, 255]),    // Dark mortar
        1,                           // Mortar width
    );
    
    // Add rock texture to both faces
    add_fbm_noise(&mut img, 12.0, 4, 15, 42);
    
    // Add subtle crack patterns for weathering
    add_crack_pattern(&mut img, 0.3, Rgba([30, 30, 35, 180]), 123);
    
    // Add color variation for natural look
    add_color_variation(&mut img, 6, 4, 77);
    
    img
}

pub fn create_earth() -> RgbaImage {
    let mut img = create_tile_base(Rgba([95, 65, 35, 255]));
    
    // Rich soil texture with FBM
    add_fbm_noise(&mut img, 10.0, 5, 25, 101);
    
    // Add darker patches for depth
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let noise = value_noise(x as f32 / 8.0, y as f32 / 8.0);
            if noise > 0.3 {
                let p = img.get_pixel(x, y);
                let darken = ((noise - 0.3) * 40.0) as i32;
                img.put_pixel(x, y, Rgba([
                    (p[0] as i32 - darken).clamp(0, 255) as u8,
                    (p[1] as i32 - darken).clamp(0, 255) as u8,
                    (p[2] as i32 - darken / 2).clamp(0, 255) as u8,
                    255
                ]));
            }
        }
    }
    
    // Add small rocks/pebbles
    add_crack_pattern(&mut img, 0.15, Rgba([70, 50, 25, 150]), 202);
    
    // Color variation for organic feel
    add_color_variation(&mut img, 12, 8, 55);
    
    // Subtle edge darkening
    add_edge_bevel(&mut img, 3, 20, 10);
    
    img
}

pub fn create_claimed_floor() -> RgbaImage {
    let mut img = create_tile_base(Rgba([50, 48, 55, 255]));
    
    // Add stone texture base
    add_fbm_noise(&mut img, 16.0, 3, 12, 333);
    
    // Draw carved stone grid pattern
    for y in (0..TILE_HEIGHT).step_by(16) {
        for x in 0..TILE_WIDTH {
            // Mortar lines (darker)
            img.put_pixel(x, y, Rgba([32, 30, 38, 255]));
            if y > 0 {
                img.put_pixel(x, y - 1, Rgba([38, 36, 42, 255]));
            }
        }
    }
    for x in (0..TILE_WIDTH).step_by(16) {
        for y in 0..TILE_HEIGHT {
            img.put_pixel(x, y, Rgba([32, 30, 38, 255]));
            if x > 0 {
                img.put_pixel(x - 1, y, Rgba([38, 36, 42, 255]));
            }
        }
    }
    
    // Add subtle wear patterns
    add_crack_pattern(&mut img, 0.2, Rgba([40, 38, 45, 100]), 444);
    
    // 3D beveled edges
    add_3d_border(&mut img, 2, Rgba([80, 78, 85, 50]), Rgba([20, 18, 25, 70]));
    
    img
}

pub fn create_reinforced_wall() -> RgbaImage {
    // Create a volumetric wall with metal reinforcement
    let mut img = create_volumetric_wall(
        Rgba([75, 75, 80, 255]),    // Stone base
        Some(Rgba([90, 90, 95, 255])),   // Lighter top
        Some(Rgba([60, 60, 65, 255])),   // Darker front
    );
    
    // Add stone texture
    add_fbm_noise(&mut img, 10.0, 3, 12, 555);
    
    let top_height = (WALL_HEIGHT as f32 * 0.5) as u32;
    
    // Draw metal reinforcement bands on front face
    for y in (top_height + 4..TILE_HEIGHT).step_by(12) {
        for x in 0..TILE_WIDTH {
            // Main metal band
            img.put_pixel(x, y, Rgba([130, 130, 140, 255]));
            if y + 1 < TILE_HEIGHT {
                img.put_pixel(x, y + 1, Rgba([110, 110, 120, 255]));
            }
        }
        // Rivets
        for x in (6..TILE_WIDTH).step_by(12) {
            img.put_pixel(x, y, Rgba([160, 160, 170, 255]));
        }
    }
    
    // Add metal bands on top face too
    for y in (2..top_height).step_by(6) {
        for x in 0..TILE_WIDTH {
            let existing = *img.get_pixel(x, y);
            img.put_pixel(x, y, lighten_color(existing, 25));
        }
    }
    
    // Add subtle highlight for depth
    add_3d_border(&mut img, 1, Rgba([120, 120, 130, 40]), Rgba([30, 30, 35, 60]));
    
    img
}

// ============================================================================
// RESOURCE TILES
// ============================================================================

pub fn create_gold_vein() -> RgbaImage {
    let mut img = create_tile_base(Rgba([80, 70, 50, 255]));
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if (x + y * 2) % 13 < 3 {
                img.put_pixel(x, y, Rgba([255, 215, 0, 255]));
            }
        }
    }
    img
}

pub fn create_gem_seam() -> RgbaImage {
    let mut img = create_tile_base(Rgba([70, 70, 80, 255]));
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let val = (x * 7 + y * 11) % 19;
            if val < 2 {
                img.put_pixel(x, y, Rgba([100, 100, 255, 255]));
            } else if val < 4 {
                img.put_pixel(x, y, Rgba([200, 100, 200, 255]));
            }
        }
    }
    img
}

pub fn create_gold_pile() -> RgbaImage {
    let mut img = create_tile_base(Rgba([180, 140, 20, 255])); // Darker gold base
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let hash = ((x as u32).wrapping_mul(2654435761) ^ (y as u32).wrapping_mul(2246822519)) as i32;
            if hash % 5 < 2 {
                 img.put_pixel(x, y, Rgba([255, 215, 0, 255])); // Shiny gold coins
            } else if hash % 7 < 1 {
                 img.put_pixel(x, y, Rgba([255, 255, 150, 255])); // Highlights
            }
        }
    }
    // Make it look like a pile (fade edges/corners?) - Texture wrapping on cube might make this weird if edges are dark.
    // For a simple texture, uniform-ish is better.
    add_noise(&mut img, 10);
    img
}

pub fn create_mana_crystal() -> RgbaImage {
    let mut img = create_tile_base(Rgba([40, 40, 60, 255]));
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;

    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let dx = (x as i32 - center_x as i32).abs();
            let dy = (y as i32 - center_y as i32) as f32;

            if dx < 4 && dy < 0.0 && dy > -12.0 {
                img.put_pixel(x, y, Rgba([100, 200, 255, 255]));
            }

            if (dx > 4 && dx < 8) && (dy > -5.0 && dy < 2.0) {
                img.put_pixel(x, y, Rgba([50, 150, 255, 255]));
            }

            if dx < 10 && dy.abs() < 8.0 && (x + y as u32) % 5 == 0 {
                let pixel = img.get_pixel(x, y);
                if pixel[0] < 100 {
                    img.put_pixel(x, y, Rgba([60, 60, 90, 255]));
                }
            }
        }
    }

    img.put_pixel(center_x, center_y - 8, Rgba([255, 255, 255, 255]));
    img.put_pixel(center_x - 5, center_y, Rgba([200, 200, 255, 255]));
    img.put_pixel(center_x + 5, center_y, Rgba([200, 200, 255, 255]));

    img
}

// ============================================================================
// HAZARD TILES
// ============================================================================

pub fn create_lava() -> RgbaImage {
    let mut img = create_tile_base(Rgba([200, 50, 0, 255]));
    
    // Create flowing lava pattern with turbulence
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let turb = turbulence(x as f32 / 10.0, y as f32 / 10.0, 4);
            
            // Hot spots (brighter)
            if turb > 0.5 {
                let intensity = (turb - 0.5) * 2.0;
                let r = (200.0 + intensity * 55.0) as u8;
                let g = (50.0 + intensity * 150.0) as u8;
                let b = (intensity * 50.0) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
            
            // Super hot core (yellow-white)
            if turb > 0.75 {
                let intensity = (turb - 0.75) * 4.0;
                img.put_pixel(x, y, Rgba([
                    255,
                    (200.0 + intensity * 55.0).min(255.0) as u8,
                    (50.0 + intensity * 100.0).min(200.0) as u8,
                    255
                ]));
            }
        }
    }
    
    // Add subtle movement lines
    add_fbm_noise(&mut img, 6.0, 2, 15, 666);
    
    // Glowing effect - brighter in center
    add_gradient_overlay(&mut img, GradientDirection::Radial, 0, 40, Rgba([255, 200, 100, 40]));
    
    img
}

pub fn create_water() -> RgbaImage {
    let mut img = create_tile_base(Rgba([25, 50, 100, 255]));
    
    // Create ripple pattern with turbulence
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let turb = turbulence(x as f32 / 12.0, y as f32 / 12.0, 3);
            
            // Lighter ripple areas
            if turb > 0.4 {
                let intensity = (turb - 0.4) / 0.6;
                let r = (25.0 + intensity * 30.0) as u8;
                let g = (50.0 + intensity * 40.0) as u8;
                let b = (100.0 + intensity * 55.0) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
    }
    
    // Add subtle FBM for organic look
    add_fbm_noise(&mut img, 8.0, 3, 10, 888);
    
    // Specular highlights for wet surface
    add_specular_highlights(&mut img, 0.4, 0.3, 999);
    
    // Slight edge darkening
    add_edge_bevel(&mut img, 2, 15, 8);
    
    img
}

pub fn create_bridge() -> RgbaImage {
    let mut img = create_tile_base(Rgba([100, 90, 80, 255]));
    for x in (0..TILE_WIDTH).step_by(16) {
        for y in 0..TILE_HEIGHT {
            img.put_pixel(x, y, Rgba([80, 70, 60, 255]));
            if x > 0 {
                img.put_pixel(x - 1, y, Rgba([80, 70, 60, 255]));
            }
        }
    }
    for x in (8..TILE_WIDTH).step_by(16) {
        img.put_pixel(x, 4, Rgba([50, 45, 40, 255]));
        img.put_pixel(x, TILE_HEIGHT - 4, Rgba([50, 45, 40, 255]));
    }
    img
}

pub fn create_corrupted_floor() -> RgbaImage {
    let mut img = create_tile_base(Rgba([60, 30, 80, 255]));
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if (x * 3 + y * 5) % 13 < 4 {
                img.put_pixel(x, y, Rgba([80, 40, 100, 255]));
            }
        }
    }
    img
}

pub fn create_ancient_rune_floor() -> RgbaImage {
    let mut img = create_tile_base(Rgba([50, 50, 60, 255]));
    for y in (8..TILE_HEIGHT).step_by(16) {
        for x in (8..TILE_WIDTH).step_by(16) {
            draw_rect(&mut img, x, y, 6, 2, Rgba([100, 150, 255, 255]));
            draw_rect(&mut img, x + 2, y - 2, 2, 6, Rgba([100, 150, 255, 255]));
        }
    }
    img
}

// ============================================================================
// DECORATIVE FLOOR TILES
// ============================================================================

pub fn create_wooden_floor() -> RgbaImage {
    let mut img = create_tile_base(Rgba([120, 80, 40, 255]));
    for x in (0..TILE_WIDTH).step_by(8) {
        for y in 0..TILE_HEIGHT {
            img.put_pixel(x, y, Rgba([100, 60, 30, 255]));
        }
    }
    img
}

pub fn create_carpet() -> RgbaImage {
    let mut img = create_tile_base(Rgba([150, 30, 30, 255]));
    draw_rect(&mut img, 0, 0, TILE_WIDTH, 4, Rgba([200, 180, 50, 255]));
    draw_rect(&mut img, 0, TILE_HEIGHT - 4, TILE_WIDTH, 4, Rgba([200, 180, 50, 255]));
    draw_rect(&mut img, 0, 0, 4, TILE_HEIGHT, Rgba([200, 180, 50, 255]));
    draw_rect(&mut img, TILE_WIDTH - 4, 0, 4, TILE_HEIGHT, Rgba([200, 180, 50, 255]));
    img
}

pub fn create_bone_floor() -> RgbaImage {
    let mut img = create_tile_base(Rgba([200, 190, 180, 255]));
    for y in (0..TILE_HEIGHT).step_by(16) {
        for x in (0..TILE_WIDTH).step_by(16) {
            draw_circle(&mut img, x + 8, y + 8, 4, Rgba([180, 170, 160, 255]));
            draw_circle(&mut img, x + 6, y + 6, 2, Rgba([50, 50, 50, 255]));
            draw_circle(&mut img, x + 10, y + 6, 2, Rgba([50, 50, 50, 255]));
        }
    }
    img
}

/// Mosaic tile floor - NEW
pub fn create_mosaic_floor() -> RgbaImage {
    let mut img = create_tile_base(Rgba([180, 170, 150, 255]));
    let colors = [
        Rgba([200, 50, 50, 255]),   // Red
        Rgba([50, 150, 200, 255]),  // Blue
        Rgba([200, 180, 50, 255]),  // Gold
        Rgba([50, 150, 50, 255]),   // Green
    ];

    for y in (0..TILE_HEIGHT).step_by(8) {
        for x in (0..TILE_WIDTH).step_by(8) {
            let color_idx = ((x / 8 + y / 8) % 4) as usize;
            draw_rect(&mut img, x + 1, y + 1, 6, 6, colors[color_idx]);
        }
    }
    add_noise(&mut img, 5);
    img
}

/// Marble floor - NEW
pub fn create_marble_floor() -> RgbaImage {
    let mut img = create_tile_base(Rgba([240, 240, 245, 255]));
    // Add marble veins
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let hash = ((x as u32).wrapping_mul(2654435761) ^ (y as u32).wrapping_mul(2246822519)) as i32;
            if hash % 50 < 3 {
                img.put_pixel(x, y, Rgba([180, 180, 190, 255]));
            }
        }
    }
    add_noise(&mut img, 3);
    img
}

/// Grass floor - NEW
pub fn create_grass() -> RgbaImage {
    let mut img = create_tile_base(Rgba([60, 120, 40, 255]));
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let hash = ((x as u32).wrapping_mul(2654435761) ^ (y as u32).wrapping_mul(2246822519)) as i32;
            if hash % 7 < 2 {
                img.put_pixel(x, y, Rgba([80, 140, 50, 255]));
            } else if hash % 11 < 1 {
                img.put_pixel(x, y, Rgba([50, 100, 35, 255]));
            }
        }
    }
    add_noise(&mut img, 8);
    img
}

/// Sand floor - NEW
pub fn create_sand() -> RgbaImage {
    let mut img = create_tile_base(Rgba([220, 200, 150, 255]));
    add_noise(&mut img, 15);
    img
}

/// Snow floor - NEW
pub fn create_snow() -> RgbaImage {
    let mut img = create_tile_base(Rgba([245, 250, 255, 255]));
    // Add subtle blue shadows
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let hash = ((x as u32).wrapping_mul(2654435761) ^ (y as u32).wrapping_mul(2246822519)) as i32;
            if hash % 20 < 2 {
                img.put_pixel(x, y, Rgba([220, 230, 250, 255]));
            }
        }
    }
    add_noise(&mut img, 3);
    img
}

// ============================================================================
// ROOM TILES
// ============================================================================

pub fn create_dungeon_heart() -> RgbaImage {
    let mut img = create_tile_base(Rgba([139, 0, 0, 255]));
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let dx = (x as i32 - center_x as i32).abs();
            let dy = (y as i32 - center_y as i32).abs();
            if dx + dy < 16 {
                img.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            }
        }
    }
    img
}

pub fn create_lair() -> RgbaImage {
    let mut img = create_tile_base(Rgba([70, 50, 40, 255]));
    for i in 0..3 {
        let cx = 16 + i * 20;
        let cy = 16 + (i % 2) * 20;
        draw_circle(&mut img, cx, cy, 10, Rgba([90, 70, 50, 255]));
    }
    add_noise(&mut img, 10);
    img
}

pub fn create_hatchery() -> RgbaImage {
    let mut img = create_tile_base(Rgba([85, 70, 50, 255]));
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if (x * 7 + y * 3) % 11 < 3 {
                img.put_pixel(x, y, Rgba([200, 180, 80, 255]));
            }
        }
    }
    img
}

pub fn create_treasury() -> RgbaImage {
    let mut img = create_tile_base(Rgba([218, 165, 32, 255]));
    for y in (0..TILE_HEIGHT).step_by(8) {
        for x in (0..TILE_WIDTH).step_by(8) {
            img.put_pixel(x, y, Rgba([255, 215, 0, 255]));
            img.put_pixel(x + 1, y, Rgba([255, 255, 100, 255]));
        }
    }
    img
}

pub fn create_workshop() -> RgbaImage {
    let mut img = create_tile_base(Rgba([60, 60, 65, 255]));
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if y > TILE_HEIGHT - 20 && x > TILE_WIDTH - 20 {
                img.put_pixel(x, y, Rgba([255, 100, 0, 255]));
            }
        }
    }
    draw_rect(&mut img, 16, 16, 20, 10, Rgba([30, 30, 30, 255]));
    img
}

pub fn create_training_room() -> RgbaImage {
    let mut img = create_tile_base(Rgba([80, 60, 50, 255]));
    draw_rect(&mut img, 10, 10, 4, 40, Rgba([100, 80, 70, 255]));
    draw_rect(&mut img, 30, 10, 4, 40, Rgba([100, 80, 70, 255]));
    draw_rect(&mut img, 50, 10, 4, 40, Rgba([100, 80, 70, 255]));
    img
}

pub fn create_library() -> RgbaImage {
    let mut img = create_tile_base(Rgba([70, 50, 80, 255]));
    for x in (0..TILE_WIDTH).step_by(16) {
        draw_rect(&mut img, x + 4, 4, 8, 56, Rgba([139, 69, 19, 255]));
    }
    img
}

pub fn create_prison() -> RgbaImage {
    let mut img = create_tile_base(Rgba([50, 50, 55, 255]));
    for x in (8..TILE_WIDTH).step_by(16) {
        for y in 0..TILE_HEIGHT {
            img.put_pixel(x, y, Rgba([30, 30, 35, 255]));
        }
    }
    img
}

pub fn create_guard_post() -> RgbaImage {
    let mut img = create_tile_base(Rgba([80, 70, 60, 255]));
    let cx = TILE_WIDTH / 2;
    let cy = TILE_HEIGHT / 2;
    draw_rect(&mut img, cx - 10, cy - 10, 20, 20, Rgba([100, 90, 80, 255]));
    draw_rect(&mut img, cx - 5, cy - 5, 10, 10, Rgba([150, 140, 130, 255]));
    img
}

pub fn create_ritual_circle() -> RgbaImage {
    let mut img = create_tile_base(Rgba([40, 20, 50, 255]));
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let dx = (x as i32 - center_x as i32).abs();
            let dy = (y as i32 - center_y as i32).abs();
            if dx * dx + dy * dy > 100 && dx * dx + dy * dy < 150 {
                img.put_pixel(x, y, Rgba([200, 50, 50, 255]));
            }
        }
    }
    img
}

pub fn create_monster_spawner() -> RgbaImage {
    let mut img = create_tile_base(Rgba([80, 20, 100, 255]));
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            if (dx * dx + dy * dy) < 400 {
                if (dx * dx + dy * dy) % 20 < 10 {
                    img.put_pixel(x, y, Rgba([150, 50, 200, 255]));
                }
            }
        }
    }
    img
}

pub fn create_graveyard() -> RgbaImage {
    let mut img = create_tile_base(Rgba([40, 40, 45, 255]));
    draw_rect(&mut img, 16, 16, 8, 12, Rgba([90, 90, 95, 255]));
    draw_rect(&mut img, 40, 24, 8, 12, Rgba([90, 90, 95, 255]));
    draw_rect(&mut img, 20, 40, 8, 12, Rgba([90, 90, 95, 255]));
    add_noise(&mut img, 15);
    img
}

pub fn create_kennel() -> RgbaImage {
    let mut img = create_tile_base(Rgba([60, 40, 30, 255]));
    draw_rect(&mut img, 10, 10, 20, 20, Rgba([100, 80, 50, 255]));
    draw_circle(&mut img, 40, 40, 5, Rgba([200, 200, 200, 255]));
    img
}

pub fn create_dungeon_barracks() -> RgbaImage {
    let mut img = create_tile_base(Rgba([80, 80, 85, 255]));
    draw_rect(&mut img, 5, 5, 15, 25, Rgba([100, 100, 110, 255]));
    draw_rect(&mut img, 5, 35, 15, 25, Rgba([100, 100, 110, 255]));
    draw_rect(&mut img, 44, 5, 15, 25, Rgba([100, 100, 110, 255]));
    draw_rect(&mut img, 44, 35, 15, 25, Rgba([100, 100, 110, 255]));
    img
}

/// Torture chamber floor - NEW
pub fn create_torture_chamber() -> RgbaImage {
    let mut img = create_tile_base(Rgba([50, 40, 40, 255]));
    // Blood stains
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let hash = ((x as u32).wrapping_mul(2654435761) ^ (y as u32).wrapping_mul(2246822519)) as i32;
            if hash % 30 < 2 {
                img.put_pixel(x, y, Rgba([100, 20, 20, 255]));
            }
        }
    }
    // Chains/shackles pattern
    draw_circle(&mut img, 16, 16, 4, Rgba([80, 80, 90, 255]));
    draw_circle(&mut img, 48, 48, 4, Rgba([80, 80, 90, 255]));
    add_noise(&mut img, 8);
    img
}

/// Casino/gambling room - NEW
pub fn create_casino() -> RgbaImage {
    let mut img = create_tile_base(Rgba([30, 80, 30, 255])); // Green felt
    // Card/dice pattern
    draw_rect(&mut img, 20, 20, 10, 14, Rgba([255, 255, 255, 255]));
    draw_rect(&mut img, 34, 30, 10, 14, Rgba([255, 255, 255, 255]));
    // Red diamonds on cards
    draw_circle(&mut img, 25, 27, 2, Rgba([200, 20, 20, 255]));
    draw_circle(&mut img, 39, 37, 2, Rgba([20, 20, 20, 255]));
    add_noise(&mut img, 5);
    img
}

/// Temple - dark shrine where creatures pray for mana
pub fn create_temple() -> RgbaImage {
    let mut img = create_tile_base(Rgba([35, 25, 45, 255])); // Dark purple-gray stone
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;

    // Ornate floor pattern - concentric circles suggesting mystical power
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            let dist_sq = dx * dx + dy * dy;

            // Outer ring
            if dist_sq > 400 && dist_sq < 500 {
                img.put_pixel(x, y, Rgba([80, 50, 100, 255]));
            }
            // Middle ring
            if dist_sq > 200 && dist_sq < 250 {
                img.put_pixel(x, y, Rgba([100, 60, 130, 255]));
            }
            // Inner sanctum
            if dist_sq < 80 {
                img.put_pixel(x, y, Rgba([60, 40, 80, 255]));
            }
        }
    }

    // Central altar symbol (stylized eye/diamond)
    draw_rect(&mut img, center_x - 3, center_y - 6, 6, 12, Rgba([150, 100, 180, 255]));
    draw_rect(&mut img, center_x - 6, center_y - 3, 12, 6, Rgba([150, 100, 180, 255]));
    img.put_pixel(center_x, center_y, Rgba([255, 200, 255, 255])); // Glowing center

    // Corner candle positions
    draw_circle(&mut img, 8, 8, 3, Rgba([200, 150, 50, 255]));
    draw_circle(&mut img, TILE_WIDTH - 8, 8, 3, Rgba([200, 150, 50, 255]));
    draw_circle(&mut img, 8, TILE_HEIGHT - 8, 3, Rgba([200, 150, 50, 255]));
    draw_circle(&mut img, TILE_WIDTH - 8, TILE_HEIGHT - 8, 3, Rgba([200, 150, 50, 255]));

    add_noise(&mut img, 5);
    img
}

// ============================================================================
// TRAP TILES
// ============================================================================

pub fn create_door() -> RgbaImage {
    let mut img = create_tile_base(Rgba([100, 60, 30, 255]));
    for x in (0..TILE_WIDTH).step_by(8) {
        for y in 0..TILE_HEIGHT {
            img.put_pixel(x, y, Rgba([90, 50, 25, 255]));
        }
    }
    draw_rect(&mut img, 0, 0, TILE_WIDTH, 4, Rgba([60, 40, 20, 255]));
    draw_rect(&mut img, 0, TILE_HEIGHT - 4, TILE_WIDTH, 4, Rgba([60, 40, 20, 255]));
    draw_rect(&mut img, 0, 0, 4, TILE_HEIGHT, Rgba([60, 40, 20, 255]));
    draw_rect(&mut img, TILE_WIDTH - 4, 0, 4, TILE_HEIGHT, Rgba([60, 40, 20, 255]));
    draw_circle(&mut img, TILE_WIDTH - 10, TILE_HEIGHT / 2, 4, Rgba([200, 200, 200, 255]));
    img
}

pub fn create_spike_trap() -> RgbaImage {
    let mut img = create_solid_rock();
    for y in (10..TILE_HEIGHT - 10).step_by(10) {
        for x in (10..TILE_WIDTH - 10).step_by(10) {
            draw_circle(&mut img, x, y, 2, Rgba([30, 30, 30, 255]));
            img.put_pixel(x, y, Rgba([200, 200, 200, 255]));
        }
    }
    img
}

pub fn create_boulder_trap() -> RgbaImage {
    let mut img = create_solid_rock();
    let cx = TILE_WIDTH / 2;
    let cy = TILE_HEIGHT / 2;
    draw_circle(&mut img, cx, cy, 20, Rgba([80, 80, 85, 255]));
    draw_circle(&mut img, cx - 5, cy - 5, 10, Rgba([100, 100, 105, 255]));
    img
}

pub fn create_alarm_trap() -> RgbaImage {
    let mut img = create_solid_rock();
    let cx = TILE_WIDTH / 2;
    let cy = TILE_HEIGHT / 2;
    draw_rect(&mut img, cx - 10, cy - 10, 20, 20, Rgba([150, 100, 50, 255]));
    draw_circle(&mut img, cx, cy, 8, Rgba([200, 50, 50, 255]));
    draw_rect(&mut img, 0, cy, TILE_WIDTH, 2, Rgba([100, 100, 100, 255]));
    img
}

/// Gas trap tile - NEW
pub fn create_gas_trap() -> RgbaImage {
    let mut img = create_solid_rock();
    let cx = TILE_WIDTH / 2;
    let cy = TILE_HEIGHT / 2;
    // Grate pattern
    for y in (cy - 10..cy + 10).step_by(4) {
        draw_rect(&mut img, cx - 10, y, 20, 2, Rgba([60, 60, 65, 255]));
    }
    // Green gas wisps
    draw_circle(&mut img, cx - 5, cy - 5, 3, Rgba([100, 200, 50, 180]));
    draw_circle(&mut img, cx + 3, cy + 2, 2, Rgba([120, 220, 70, 150]));
    img
}

/// Lightning trap tile - NEW
pub fn create_lightning_trap() -> RgbaImage {
    let mut img = create_solid_rock();
    let cx = TILE_WIDTH / 2;
    let cy = TILE_HEIGHT / 2;
    // Metal conductors
    draw_rect(&mut img, cx - 15, cy - 15, 5, 5, Rgba([200, 200, 220, 255]));
    draw_rect(&mut img, cx + 10, cy - 15, 5, 5, Rgba([200, 200, 220, 255]));
    draw_rect(&mut img, cx - 15, cy + 10, 5, 5, Rgba([200, 200, 220, 255]));
    draw_rect(&mut img, cx + 10, cy + 10, 5, 5, Rgba([200, 200, 220, 255]));
    // Electric arc in center
    draw_circle(&mut img, cx, cy, 6, Rgba([150, 200, 255, 255]));
    img.put_pixel(cx, cy, Rgba([255, 255, 255, 255]));
    img
}

/// Fire trap tile - NEW
pub fn create_fire_trap() -> RgbaImage {
    let mut img = create_solid_rock();
    let cx = TILE_WIDTH / 2;
    let cy = TILE_HEIGHT / 2;
    // Fire nozzles
    for angle in 0..4 {
        let offset_x = (angle % 2) as i32 * 20 - 10;
        let offset_y = (angle / 2) as i32 * 20 - 10;
        draw_circle(&mut img, (cx as i32 + offset_x) as u32, (cy as i32 + offset_y) as u32, 4, Rgba([100, 50, 30, 255]));
        draw_circle(&mut img, (cx as i32 + offset_x) as u32, (cy as i32 + offset_y) as u32, 2, Rgba([255, 100, 0, 255]));
    }
    img
}

// ============================================================================
// SAVE FUNCTION
// ============================================================================

pub fn save_tile(name: &str, img: RgbaImage) {
    let path = format!("assets/tiles/{}.png", name);
    img.save(&path).unwrap();
    println!("Generated: {}", path);
}
