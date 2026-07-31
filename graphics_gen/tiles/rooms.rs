use super::super::core::*;
use image::{Rgba, RgbaImage};

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
            if (dx * dx + dy * dy) < 400 && (dx * dx + dy * dy) % 20 < 10 {
                img.put_pixel(x, y, Rgba([150, 50, 200, 255]));
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
            let hash = (x.wrapping_mul(2654435761) ^ y.wrapping_mul(2246822519)) as i32;
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
    draw_rect(
        &mut img,
        center_x - 3,
        center_y - 6,
        6,
        12,
        Rgba([150, 100, 180, 255]),
    );
    draw_rect(
        &mut img,
        center_x - 6,
        center_y - 3,
        12,
        6,
        Rgba([150, 100, 180, 255]),
    );
    img.put_pixel(center_x, center_y, Rgba([255, 200, 255, 255])); // Glowing center

    // Corner candle positions
    draw_circle(&mut img, 8, 8, 3, Rgba([200, 150, 50, 255]));
    draw_circle(&mut img, TILE_WIDTH - 8, 8, 3, Rgba([200, 150, 50, 255]));
    draw_circle(&mut img, 8, TILE_HEIGHT - 8, 3, Rgba([200, 150, 50, 255]));
    draw_circle(
        &mut img,
        TILE_WIDTH - 8,
        TILE_HEIGHT - 8,
        3,
        Rgba([200, 150, 50, 255]),
    );

    add_noise(&mut img, 5);
    img
}

/// Scavenger room - a lure sigil that draws enemy minions in to defect
pub fn create_scavenger() -> RgbaImage {
    let mut img = create_tile_base(Rgba([38, 46, 44, 255])); // Damp green-grey stone
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;

    // Spokes of inward-marching dashes: the "pull" of the lure. Each arm starts
    // near the tile edge and closes on the focus crystal, brightening as it
    // goes, so the eye is dragged to the centre the way the room drags
    // creatures.
    for arm in 0..8 {
        let angle = arm as f32 * std::f32::consts::TAU / 8.0;
        let (sin_a, cos_a) = angle.sin_cos();
        for step in 6..30 {
            // Dashes, not a solid ray — a dotted pull line reads better at 64px.
            if step % 5 >= 3 {
                continue;
            }
            let radius = step as f32;
            let px = center_x as f32 + cos_a * radius;
            let py = center_y as f32 + sin_a * radius;
            if px < 0.0 || py < 0.0 || px >= TILE_WIDTH as f32 || py >= TILE_HEIGHT as f32 {
                continue;
            }
            // Closer to the centre -> brighter teal.
            let closeness = 1.0 - (radius / 30.0);
            let g = (110.0 + 110.0 * closeness) as u8;
            let b = (100.0 + 90.0 * closeness) as u8;
            img.put_pixel(px as u32, py as u32, Rgba([40, g, b, 255]));
        }
    }

    // Containment ring the sigil is bound inside
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            let dist_sq = dx * dx + dy * dy;
            if (600..700).contains(&dist_sq) {
                img.put_pixel(x, y, Rgba([70, 150, 135, 255]));
            }
        }
    }

    // Focus crystal at the centre (the room's `object_spawn`), lit from within
    draw_circle(&mut img, center_x, center_y, 7, Rgba([30, 70, 65, 255]));
    draw_circle(&mut img, center_x, center_y, 5, Rgba([90, 200, 180, 255]));
    draw_circle(&mut img, center_x, center_y, 2, Rgba([210, 255, 245, 255]));

    add_noise(&mut img, 8);
    img
}
