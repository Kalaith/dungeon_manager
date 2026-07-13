use super::super::core::*;
use image::{Rgba, RgbaImage};

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
                img.put_pixel(
                    x,
                    y,
                    Rgba([
                        255,
                        (200.0 + intensity * 55.0).min(255.0) as u8,
                        (50.0 + intensity * 100.0).min(200.0) as u8,
                        255,
                    ]),
                );
            }
        }
    }

    // Add subtle movement lines
    add_fbm_noise(&mut img, 6.0, 2, 15, 666);

    // Glowing effect - brighter in center
    add_gradient_overlay(
        &mut img,
        GradientDirection::Radial,
        0,
        40,
        Rgba([255, 200, 100, 40]),
    );

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
