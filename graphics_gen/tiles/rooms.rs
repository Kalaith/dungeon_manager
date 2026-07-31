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

/// Vault - dense, armoured gold storage
///
/// Deliberately unlike [`create_treasury`], which is a bright scatter of loose
/// coin: the vault is dark plate steel with gold stacked in disciplined rows,
/// so a glance tells the two storage rooms apart on the map.
pub fn create_vault() -> RgbaImage {
    let mut img = create_tile_base(Rgba([58, 60, 68, 255])); // Gunmetal plate

    // Riveted floor plates, quartered
    for edge in [0u32, TILE_WIDTH / 2] {
        draw_rect(&mut img, edge, 0, 2, TILE_HEIGHT, Rgba([40, 42, 48, 255]));
        draw_rect(&mut img, 0, edge, TILE_WIDTH, 2, Rgba([40, 42, 48, 255]));
    }
    for rivet_y in (6..TILE_HEIGHT).step_by(16) {
        for rivet_x in (6..TILE_WIDTH).step_by(16) {
            img.put_pixel(rivet_x, rivet_y, Rgba([104, 108, 118, 255]));
        }
    }

    // Two stacks of ingots, offset so the tile does not read as a grid when
    // repeated across a room.
    for (origin_x, origin_y) in [(10u32, 12u32), (36, 38)] {
        for row in 0..3u32 {
            // Each row is inset, so the stack reads as a pyramid from above.
            let inset = row * 3;
            let bar_y = origin_y + row * 6;
            draw_rect(
                &mut img,
                origin_x + inset,
                bar_y,
                18 - inset * 2,
                4,
                Rgba([176, 132, 24, 255]),
            );
            // Lit top edge and shadowed underside give the bar its thickness.
            draw_rect(
                &mut img,
                origin_x + inset,
                bar_y,
                18 - inset * 2,
                1,
                Rgba([255, 214, 92, 255]),
            );
            draw_rect(
                &mut img,
                origin_x + inset,
                bar_y + 3,
                18 - inset * 2,
                1,
                Rgba([116, 84, 12, 255]),
            );
        }
    }

    add_noise(&mut img, 6);
    img
}

/// Mana Well - a stone-rimmed shaft of stored mana
pub fn create_mana_well() -> RgbaImage {
    let mut img = create_tile_base(Rgba([34, 36, 52, 255])); // Cold shadowed stone
    add_noise(&mut img, 10);

    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;

    // Rim blocks, then the shaft: successively deeper and bluer toward the
    // middle so the tile reads as a hole rather than a painted disc.
    for (radius, color) in [
        (28u32, Rgba([72, 74, 92, 255])),
        (25, Rgba([46, 48, 66, 255])),
        (22, Rgba([22, 30, 58, 255])),
        (17, Rgba([28, 62, 130, 255])),
        (12, Rgba([48, 118, 200, 255])),
        (7, Rgba([120, 200, 245, 255])),
        (3, Rgba([225, 248, 255, 255])),
    ] {
        draw_circle(&mut img, center_x, center_y, radius, color);
    }

    // Rune notches cut into the rim at the compass points
    for (dx, dy) in [(0i32, -26i32), (0, 26), (-26, 0), (26, 0)] {
        draw_rect(
            &mut img,
            (center_x as i32 + dx - 2) as u32,
            (center_y as i32 + dy - 2) as u32,
            4,
            4,
            Rgba([140, 190, 255, 255]),
        );
    }

    // Surface glimmer, off-centre so the pool does not look like a target
    draw_circle(
        &mut img,
        center_x - 5,
        center_y - 6,
        2,
        Rgba([200, 235, 255, 255]),
    );
    draw_circle(
        &mut img,
        center_x + 6,
        center_y + 4,
        1,
        Rgba([190, 228, 255, 255]),
    );

    img
}

/// Leisure Den - the amenity room the `happiness_modifier` hook makes possible
///
/// Warm and cluttered where the rest of the dungeon is cold and ordered: the
/// one place on the map that exists purely to be pleasant to stand in.
pub fn create_leisure_den() -> RgbaImage {
    let mut img = create_tile_base(Rgba([74, 44, 38, 255])); // Worn timber

    // Plank grain, running one way so the floor has a direction
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if (x * 5 + y * 13) % 17 < 2 {
                img.put_pixel(x, y, Rgba([88, 54, 44, 255]));
            }
        }
    }
    for plank_y in (0..TILE_HEIGHT).step_by(16) {
        draw_rect(&mut img, 0, plank_y, TILE_WIDTH, 1, Rgba([52, 30, 26, 255]));
    }

    // A rug, off-centre and overlapping the planks
    draw_rect(&mut img, 14, 18, 34, 28, Rgba([132, 40, 46, 255]));
    draw_rect(&mut img, 17, 21, 28, 22, Rgba([158, 58, 58, 255]));
    draw_rect(&mut img, 21, 25, 20, 14, Rgba([120, 34, 40, 255]));
    // Fringe
    for fringe_x in (14..48).step_by(4) {
        img.put_pixel(fringe_x, 17, Rgba([196, 156, 92, 255]));
        img.put_pixel(fringe_x, 46, Rgba([196, 156, 92, 255]));
    }

    // Cushions tossed at two corners of the rug
    for (cushion_x, cushion_y) in [(20u32, 24u32), (40, 40)] {
        draw_circle(&mut img, cushion_x, cushion_y, 5, Rgba([64, 52, 118, 255]));
        draw_circle(
            &mut img,
            cushion_x - 1,
            cushion_y - 1,
            3,
            Rgba([90, 76, 156, 255]),
        );
    }

    // Candle stub in the corner — the light source that makes the room warm
    draw_circle(&mut img, 53, 11, 4, Rgba([206, 188, 148, 255]));
    draw_circle(&mut img, 53, 11, 2, Rgba([255, 214, 120, 255]));
    img.put_pixel(53, 11, Rgba([255, 250, 220, 255]));

    add_noise(&mut img, 7);
    img
}

/// Arcane Archive - the research room's elite tier
///
/// Reads against [`create_library`]'s plain brown shelving: obsidian and
/// verdigris rather than wood, with a bound tome on a lectern instead of a
/// rank of spines. Same job, visibly more serious about it.
pub fn create_arcane_archive() -> RgbaImage {
    let mut img = create_tile_base(Rgba([30, 28, 44, 255])); // Obsidian
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;

    // Inlaid brass channels running to the centre, like a circuit feeding the
    // lectern — the room's power has somewhere to go.
    for offset in [-20i32, 20] {
        draw_rect(
            &mut img,
            (center_x as i32 + offset - 1) as u32,
            6,
            2,
            TILE_HEIGHT - 12,
            Rgba([92, 78, 42, 255]),
        );
    }
    draw_rect(&mut img, 20, center_y - 1, 24, 2, Rgba([92, 78, 42, 255]));

    // Shelf blocks in the corners, cut back so the centre stays clear
    for (shelf_x, shelf_y) in [(4u32, 4u32), (48, 4), (4, 48), (48, 48)] {
        draw_rect(&mut img, shelf_x, shelf_y, 12, 12, Rgba([46, 42, 62, 255]));
        draw_rect(&mut img, shelf_x, shelf_y, 12, 1, Rgba([70, 64, 92, 255]));
        // Spines, in the verdigris the library's brown never had
        for spine in 0..4u32 {
            draw_rect(
                &mut img,
                shelf_x + 1 + spine * 3,
                shelf_y + 3,
                2,
                8,
                Rgba([56, 128, 112, 255]),
            );
        }
    }

    // Lectern and the open tome on it
    draw_rect(
        &mut img,
        center_x - 9,
        center_y - 7,
        18,
        14,
        Rgba([58, 50, 40, 255]),
    );
    draw_rect(
        &mut img,
        center_x - 8,
        center_y - 6,
        7,
        12,
        Rgba([214, 206, 182, 255]),
    );
    draw_rect(
        &mut img,
        center_x + 1,
        center_y - 6,
        7,
        12,
        Rgba([196, 188, 164, 255]),
    );
    draw_rect(
        &mut img,
        center_x - 1,
        center_y - 6,
        2,
        12,
        Rgba([84, 72, 56, 255]),
    );

    // The glow coming off the open page
    draw_circle(
        &mut img,
        center_x,
        center_y - 9,
        3,
        Rgba([120, 224, 200, 255]),
    );
    img.put_pixel(center_x, center_y - 9, Rgba([232, 255, 248, 255]));

    add_noise(&mut img, 6);
    img
}

/// Combat Pit - the training family's brutal tier
///
/// Where [`create_training_room`] is upright practice posts, this is a sunken
/// sand floor with a blood-dark ring: creatures do not drill here, they fight
/// each other.
pub fn create_combat_pit() -> RgbaImage {
    let mut img = create_tile_base(Rgba([116, 96, 62, 255])); // Packed sand
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;

    // Raked sand, swept in arcs by whatever was dragged across it
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if (x * 3 + y * 7) % 13 < 3 {
                img.put_pixel(x, y, Rgba([132, 110, 74, 255]));
            }
        }
    }

    // The pit rim: stone kerb, then a drop into shadow
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            let dist_sq = dx * dx + dy * dy;
            if (676..=900).contains(&dist_sq) {
                img.put_pixel(x, y, Rgba([84, 78, 68, 255]));
            } else if (576..676).contains(&dist_sq) {
                img.put_pixel(x, y, Rgba([54, 44, 34, 255]));
            }
        }
    }

    // Fighting ring stained into the sand
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            let dist_sq = dx * dx + dy * dy;
            if (256..324).contains(&dist_sq) {
                img.put_pixel(x, y, Rgba([104, 34, 28, 255]));
            }
        }
    }

    // Spilled blood, spattered rather than pooled
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let hash = (x.wrapping_mul(2246822519) ^ y.wrapping_mul(3266489917)) as i32;
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            if dx * dx + dy * dy < 400 && hash % 23 < 2 {
                img.put_pixel(x, y, Rgba([88, 26, 22, 255]));
            }
        }
    }

    // A pair of notched weapons left in the sand
    draw_line(&mut img, 22, 38, 32, 26, Rgba([176, 176, 184, 255]));
    draw_rect(&mut img, 20, 38, 4, 3, Rgba([72, 52, 34, 255]));
    draw_line(&mut img, 42, 40, 34, 28, Rgba([160, 160, 170, 255]));
    draw_rect(&mut img, 41, 39, 4, 3, Rgba([72, 52, 34, 255]));

    add_noise(&mut img, 9);
    img
}

/// Soul Furnace - bodies rendered down into mana
///
/// Reads against [`create_temple`]'s violet devotional rings: this is industry,
/// not worship. Iron grate, ash, and a firebox burning the wrong colour.
pub fn create_soul_furnace() -> RgbaImage {
    let mut img = create_tile_base(Rgba([42, 38, 40, 255])); // Soot-blackened stone
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;

    // Drifted ash, heavier at the corners where nobody sweeps
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let edge = (x.min(TILE_WIDTH - 1 - x)).min(y.min(TILE_HEIGHT - 1 - y));
            let hash = (x.wrapping_mul(2654435761) ^ y.wrapping_mul(3266489917)) as i32;
            if edge < 14 && hash % 9 < 2 {
                img.put_pixel(x, y, Rgba([86, 82, 84, 255]));
            }
        }
    }

    // Iron plate surround, riveted
    draw_rect(
        &mut img,
        8,
        8,
        TILE_WIDTH - 16,
        TILE_HEIGHT - 16,
        Rgba([64, 60, 62, 255]),
    );
    draw_rect(
        &mut img,
        8,
        8,
        TILE_WIDTH - 16,
        1,
        Rgba([104, 100, 102, 255]),
    );
    for rivet in (12..TILE_WIDTH - 10).step_by(10) {
        img.put_pixel(rivet, 10, Rgba([132, 128, 130, 255]));
        img.put_pixel(rivet, TILE_HEIGHT - 11, Rgba([96, 92, 94, 255]));
    }

    // The firebox: bars over a heat gradient, so the glow reads as *contained*
    draw_rect(&mut img, 18, 18, 28, 28, Rgba([22, 18, 20, 255]));
    for y in 19..45u32 {
        for x in 19..45u32 {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            let falloff = 1.0 - ((dx * dx + dy * dy) as f32).sqrt() / 20.0;
            if falloff <= 0.0 {
                continue;
            }
            // Raised to a power so the core stays tight and the surround stays
            // dark — a linear ramp greys out into the iron and reads as fog.
            let heat = falloff.powf(1.8);
            // Sickly green at the core over a violet bed: soul-fire, not the
            // orange the workshop forge already uses.
            img.put_pixel(
                x,
                y,
                Rgba([
                    (14.0 + 210.0 * heat) as u8,
                    (10.0 + 245.0 * heat) as u8,
                    (26.0 + 180.0 * heat) as u8,
                    255,
                ]),
            );
        }
    }

    // Grate bars across the opening
    for bar_x in (20..46).step_by(6) {
        draw_rect(&mut img, bar_x, 18, 2, 28, Rgba([34, 30, 32, 255]));
    }

    // Flue mouths top and bottom, venting what is left of them
    draw_rect(&mut img, center_x - 4, 2, 8, 5, Rgba([28, 26, 28, 255]));
    draw_rect(
        &mut img,
        center_x - 4,
        TILE_HEIGHT - 7,
        8,
        5,
        Rgba([28, 26, 28, 255]),
    );

    add_noise(&mut img, 7);
    img
}
