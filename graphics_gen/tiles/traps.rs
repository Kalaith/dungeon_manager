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

/// Braced door - the plain door reinforced with iron bands and rivets
pub fn create_braced_door() -> RgbaImage {
    let mut img = create_door();

    // Two horizontal iron braces across the planks
    for band_y in [TILE_HEIGHT / 4, TILE_HEIGHT * 3 / 4] {
        draw_rect(
            &mut img,
            4,
            band_y - 3,
            TILE_WIDTH - 8,
            6,
            Rgba([70, 72, 78, 255]),
        );
        // Top edge catches the light, bottom edge falls into shadow
        draw_rect(
            &mut img,
            4,
            band_y - 3,
            TILE_WIDTH - 8,
            1,
            Rgba([120, 124, 132, 255]),
        );
        draw_rect(
            &mut img,
            4,
            band_y + 2,
            TILE_WIDTH - 8,
            1,
            Rgba([40, 42, 46, 255]),
        );
        // Rivets
        for rivet_x in (8..TILE_WIDTH - 8).step_by(12) {
            img.put_pixel(rivet_x, band_y, Rgba([160, 164, 172, 255]));
        }
    }

    img
}

/// Magic door - a warded door; the planks are barely visible behind the sigil
pub fn create_magic_door() -> RgbaImage {
    let mut img = create_door();
    let cx = TILE_WIDTH / 2;
    let cy = TILE_HEIGHT / 2;

    // Wash the wood violet so the ward reads as *covering* the door
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let px = img.get_pixel(x, y).0;
            img.put_pixel(
                x,
                y,
                Rgba([px[0] / 2 + 30, px[1] / 2 + 15, px[2] / 2 + 55, 255]),
            );
        }
    }

    // Ward: a ring with a bound rune inside
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let dx = x as i32 - cx as i32;
            let dy = y as i32 - cy as i32;
            let dist_sq = dx * dx + dy * dy;
            if (280..340).contains(&dist_sq) {
                img.put_pixel(x, y, Rgba([170, 110, 240, 255]));
            }
        }
    }
    draw_line(
        &mut img,
        cx as i32 - 8,
        cy as i32 - 8,
        cx as i32 + 8,
        cy as i32 + 8,
        Rgba([210, 170, 255, 255]),
    );
    draw_line(
        &mut img,
        cx as i32 + 8,
        cy as i32 - 8,
        cx as i32 - 8,
        cy as i32 + 8,
        Rgba([210, 170, 255, 255]),
    );
    draw_circle(&mut img, cx, cy, 3, Rgba([245, 225, 255, 255]));

    img
}

/// Blowgun trap - dart tubes set into the floor around a firing slot
pub fn create_blowgun_trap() -> RgbaImage {
    let mut img = create_solid_rock();
    let cx = TILE_WIDTH / 2;
    let cy = TILE_HEIGHT / 2;

    // The firing slot: a narrow dark recess
    draw_rect(&mut img, cx - 14, cy - 4, 28, 8, Rgba([25, 25, 28, 255]));
    draw_rect(&mut img, cx - 14, cy - 4, 28, 1, Rgba([95, 95, 100, 255]));

    // Three brass tube mouths inside the slot
    for offset in [-9i32, 0, 9] {
        let tube_x = (cx as i32 + offset) as u32;
        draw_circle(&mut img, tube_x, cy, 3, Rgba([120, 95, 45, 255]));
        draw_circle(&mut img, tube_x, cy, 1, Rgba([15, 15, 15, 255]));
    }

    // Spent darts scattered on the stone
    draw_line(&mut img, 12, 50, 20, 46, Rgba([180, 180, 160, 255]));
    draw_line(&mut img, 44, 14, 52, 18, Rgba([180, 180, 160, 255]));

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
