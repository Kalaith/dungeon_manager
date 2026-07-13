use super::super::core::*;
use super::terrain::create_solid_rock;
use image::{Rgba, RgbaImage};

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
    draw_rect(
        &mut img,
        0,
        TILE_HEIGHT - 4,
        TILE_WIDTH,
        4,
        Rgba([60, 40, 20, 255]),
    );
    draw_rect(&mut img, 0, 0, 4, TILE_HEIGHT, Rgba([60, 40, 20, 255]));
    draw_rect(
        &mut img,
        TILE_WIDTH - 4,
        0,
        4,
        TILE_HEIGHT,
        Rgba([60, 40, 20, 255]),
    );
    draw_circle(
        &mut img,
        TILE_WIDTH - 10,
        TILE_HEIGHT / 2,
        4,
        Rgba([200, 200, 200, 255]),
    );
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
    draw_rect(
        &mut img,
        cx - 10,
        cy - 10,
        20,
        20,
        Rgba([150, 100, 50, 255]),
    );
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
        let offset_x = (angle % 2) * 20 - 10;
        let offset_y = (angle / 2) * 20 - 10;
        draw_circle(
            &mut img,
            (cx as i32 + offset_x) as u32,
            (cy as i32 + offset_y) as u32,
            4,
            Rgba([100, 50, 30, 255]),
        );
        draw_circle(
            &mut img,
            (cx as i32 + offset_x) as u32,
            (cy as i32 + offset_y) as u32,
            2,
            Rgba([255, 100, 0, 255]),
        );
    }
    img
}
