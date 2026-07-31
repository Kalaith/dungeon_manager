use super::super::core::*;
use image::{Rgba, RgbaImage};

// ============================================================================
// NATURAL TILES
// ============================================================================

pub fn create_solid_rock() -> RgbaImage {
    // Create a volumetric carved stone block wall
    let mut img = create_carved_block(
        Rgba([55, 55, 60, 255]), // Base stone color
        16,                      // Block width
        12,                      // Block height
        Rgba([35, 35, 40, 255]), // Dark mortar
        1,                       // Mortar width
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
                img.put_pixel(
                    x,
                    y,
                    Rgba([
                        (p[0] as i32 - darken).clamp(0, 255) as u8,
                        (p[1] as i32 - darken).clamp(0, 255) as u8,
                        (p[2] as i32 - darken / 2).clamp(0, 255) as u8,
                        255,
                    ]),
                );
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
        Rgba([75, 75, 80, 255]),       // Stone base
        Some(Rgba([90, 90, 95, 255])), // Lighter top
        Some(Rgba([60, 60, 65, 255])), // Darker front
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
    add_3d_border(
        &mut img,
        1,
        Rgba([120, 120, 130, 40]),
        Rgba([30, 30, 35, 60]),
    );

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
            let hash = (x.wrapping_mul(2654435761) ^ y.wrapping_mul(2246822519)) as i32;
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

            if dx < 10 && dy.abs() < 8.0 && (x + y) % 5 == 0 {
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

/// Hero entrance - the surface breach heroes pour in through
///
/// The map generator drops these on the far side of the map and the danger
/// heuristic treats them as the scariest thing on it, so the tile has to read
/// as a threat at a glance: a bright cold gate mouth punched through the rock.
pub fn create_hero_entrance() -> RgbaImage {
    let mut img = create_tile_base(Rgba([48, 46, 52, 255]));
    add_fbm_noise(&mut img, 10.0, 4, 18, 909);

    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;

    // Stone arch framing the breach, cut back in three receding steps so the
    // opening reads as a tunnel rather than a painted disc.
    // Kept dark on purpose: the glow below is the only bright thing on the
    // tile, so the arch has to stay well under it or the two merge into one
    // flat disc at 64px.
    for (radius, color) in [
        (26u32, Rgba([96, 92, 100, 255])),
        (22, Rgba([58, 55, 63, 255])),
        (18, Rgba([26, 24, 30, 255])),
    ] {
        draw_circle(&mut img, center_x, center_y, radius, color);
    }

    // The light beyond: a warm glow falling off sharply toward the arch, so
    // the mouth of the tunnel keeps a dark rim against the bright centre.
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            let dist_sq = (dx * dx + dy * dy) as f32;
            if dist_sq >= 256.0 {
                continue;
            }
            // Squared falloff concentrates the light in the middle instead of
            // smearing it evenly across the opening.
            let falloff = (1.0 - dist_sq.sqrt() / 16.0).powi(2);
            img.put_pixel(
                x,
                y,
                Rgba([
                    (40.0 + 215.0 * falloff) as u8,
                    (32.0 + 208.0 * falloff) as u8,
                    (26.0 + 175.0 * falloff) as u8,
                    255,
                ]),
            );
        }
    }

    // Banner pegs at the arch shoulders — someone claimed this doorway
    draw_rect(
        &mut img,
        center_x - 22,
        center_y - 26,
        3,
        9,
        Rgba([170, 40, 40, 255]),
    );
    draw_rect(
        &mut img,
        center_x + 19,
        center_y - 26,
        3,
        9,
        Rgba([170, 40, 40, 255]),
    );

    img
}
