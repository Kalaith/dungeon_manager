use super::super::core::*;
use image::{Rgba, RgbaImage};

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
    draw_rect(
        &mut img,
        0,
        TILE_HEIGHT - 4,
        TILE_WIDTH,
        4,
        Rgba([200, 180, 50, 255]),
    );
    draw_rect(&mut img, 0, 0, 4, TILE_HEIGHT, Rgba([200, 180, 50, 255]));
    draw_rect(
        &mut img,
        TILE_WIDTH - 4,
        0,
        4,
        TILE_HEIGHT,
        Rgba([200, 180, 50, 255]),
    );
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
        Rgba([200, 50, 50, 255]),  // Red
        Rgba([50, 150, 200, 255]), // Blue
        Rgba([200, 180, 50, 255]), // Gold
        Rgba([50, 150, 50, 255]),  // Green
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
            let hash = (x.wrapping_mul(2654435761) ^ y.wrapping_mul(2246822519)) as i32;
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
            let hash = (x.wrapping_mul(2654435761) ^ y.wrapping_mul(2246822519)) as i32;
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
            let hash = (x.wrapping_mul(2654435761) ^ y.wrapping_mul(2246822519)) as i32;
            if hash % 20 < 2 {
                img.put_pixel(x, y, Rgba([220, 230, 250, 255]));
            }
        }
    }
    add_noise(&mut img, 3);
    img
}
