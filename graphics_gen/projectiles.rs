//! Projectile sprite generators
//!
//! This module contains all projectile-related sprite generation functions.

use image::{Rgba, RgbaImage};

pub const PROJECTILE_SIZE: u32 = 32;

/// Generate all projectile sprites
pub fn generate_projectile_sprites() {
    std::fs::create_dir_all("assets/sprites/projectiles").unwrap();

    save_projectile("projectile_melee", create_melee_slash());
    save_projectile("projectile_arrow", create_arrow_projectile());
    save_projectile("projectile_magic", create_magic_orb());
    save_projectile("projectile_fireball", create_fireball());
    save_projectile("projectile_ice_shard", create_ice_shard());
    save_projectile("projectile_poison", create_poison_bolt());
    save_projectile("projectile_lightning", create_lightning_bolt());
    save_projectile("projectile_shadow", create_shadow_bolt());
    save_projectile("projectile_holy", create_holy_bolt());
    save_projectile("projectile_rock", create_rock_projectile());
}

fn save_projectile(name: &str, img: RgbaImage) {
    let path = format!("assets/sprites/projectiles/{}.png", name);
    img.save(&path).unwrap();
    println!("Generated: {}", path);
}

/// Melee slash - short orange/red horizontal bar with motion blur
pub fn create_melee_slash() -> RgbaImage {
    let mut img = RgbaImage::new(PROJECTILE_SIZE, PROJECTILE_SIZE);
    let cx = PROJECTILE_SIZE as f32 / 2.0;
    let cy = PROJECTILE_SIZE as f32 / 2.0;

    // Motion blur trail (fainter)
    for i in 0..4 {
        let alpha = 100 - i * 20;
        let trail_color = Rgba([255, 100 + i * 30, 0, alpha]);
        for y in (cy as i32 - 2)..(cy as i32 + 2) {
            for x in (cx as i32 - 12 + i as i32)..(cx as i32 + 12 - i as i32) {
                if x >= 0 && x < PROJECTILE_SIZE as i32 && y >= 0 && y < PROJECTILE_SIZE as i32 {
                    let current = img.get_pixel(x as u32, y as u32);
                    if current[3] == 0 {
                        img.put_pixel(x as u32, y as u32, trail_color);
                    }
                }
            }
        }
    }

    // Main slash (bright core)
    let core_color = Rgba([255, 220, 100, 255]);
    for y in (cy as i32 - 1)..(cy as i32 + 2) {
        for x in (cx as i32 - 10)..(cx as i32 + 10) {
            if x >= 0 && x < PROJECTILE_SIZE as i32 && y >= 0 && y < PROJECTILE_SIZE as i32 {
                img.put_pixel(x as u32, y as u32, core_color);
            }
        }
    }

    // White highlight
    for x in (cx as i32 - 6)..(cx as i32 + 6) {
        if x >= 0 && x < PROJECTILE_SIZE as i32 {
            img.put_pixel(x as u32, cy as u32, Rgba([255, 255, 255, 255]));
        }
    }

    img
}

/// Arrow projectile - brown shaft with metal arrowhead
pub fn create_arrow_projectile() -> RgbaImage {
    let mut img = RgbaImage::new(PROJECTILE_SIZE, PROJECTILE_SIZE);
    let cx = PROJECTILE_SIZE as f32 / 2.0;
    let cy = PROJECTILE_SIZE as f32 / 2.0;

    let wood_color = Rgba([139, 90, 43, 255]);
    let metal_color = Rgba([180, 180, 190, 255]);
    let fletching_color = Rgba([100, 80, 60, 255]);

    // Shaft (horizontal arrow pointing right)
    for x in (cx as i32 - 10)..(cx as i32 + 6) {
        if x >= 0 && x < PROJECTILE_SIZE as i32 {
            img.put_pixel(x as u32, cy as u32, wood_color);
            img.put_pixel(x as u32, (cy - 1.0) as u32, wood_color);
        }
    }

    // Arrowhead (triangle pointing right)
    for i in 0..6 {
        let x = cx as i32 + 6 + i;
        let half_height = (6 - i) / 2;
        for dy in -half_height..=half_height {
            let y = cy as i32 + dy;
            if x >= 0 && x < PROJECTILE_SIZE as i32 && y >= 0 && y < PROJECTILE_SIZE as i32 {
                img.put_pixel(x as u32, y as u32, metal_color);
            }
        }
    }

    // Fletching (feathers at the back)
    for i in 0..4 {
        let x = cx as i32 - 10 - i;
        if x >= 0 && x < PROJECTILE_SIZE as i32 {
            img.put_pixel(x as u32, (cy - 2.0) as u32, fletching_color);
            img.put_pixel(x as u32, (cy + 2.0) as u32, fletching_color);
        }
    }

    img
}

/// Magic orb - glowing purple/blue sphere with aura
pub fn create_magic_orb() -> RgbaImage {
    let mut img = RgbaImage::new(PROJECTILE_SIZE, PROJECTILE_SIZE);
    let cx = PROJECTILE_SIZE as f32 / 2.0;
    let cy = PROJECTILE_SIZE as f32 / 2.0;

    // Outer glow (soft purple aura)
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 12.0 && dist > 6.0 {
                let alpha = ((12.0 - dist) / 6.0 * 100.0) as u8;
                img.put_pixel(x, y, Rgba([150, 100, 255, alpha]));
            }
        }
    }

    // Core orb
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 6.0 {
                // Gradient from bright center to purple edge
                let brightness = 1.0 - (dist / 6.0);
                let r = (100.0 + brightness * 155.0) as u8;
                let g = (50.0 + brightness * 150.0) as u8;
                let b = (200.0 + brightness * 55.0) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
    }

    // White highlight sparkle
    img.put_pixel(
        (cx - 2.0) as u32,
        (cy - 2.0) as u32,
        Rgba([255, 255, 255, 255]),
    );
    img.put_pixel(
        (cx - 1.0) as u32,
        (cy - 2.0) as u32,
        Rgba([255, 255, 255, 200]),
    );
    img.put_pixel(
        (cx - 2.0) as u32,
        (cy - 1.0) as u32,
        Rgba([255, 255, 255, 200]),
    );

    img
}

/// Fireball - blazing orange/red sphere with flame trail
pub fn create_fireball() -> RgbaImage {
    let mut img = RgbaImage::new(PROJECTILE_SIZE, PROJECTILE_SIZE);
    let cx = PROJECTILE_SIZE as f32 / 2.0;
    let cy = PROJECTILE_SIZE as f32 / 2.0;

    // Flame trail behind the fireball
    for i in 0..8 {
        let trail_x = cx - 4.0 - i as f32 * 1.5;
        let trail_radius = 4.0 - i as f32 * 0.4;
        let alpha = (180 - i * 20) as u8;

        for y in 0..PROJECTILE_SIZE {
            for x in 0..PROJECTILE_SIZE {
                let dx = x as f32 - trail_x;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < trail_radius && img.get_pixel(x, y)[3] == 0 {
                    // Orange-yellow flame color
                    let r = 255;
                    let g = (150 + i * 10).min(255) as u8;
                    let b = (50 - i * 5).max(0) as u8;
                    img.put_pixel(x, y, Rgba([r, g, b, alpha]));
                }
            }
        }
    }

    // Outer glow (orange aura)
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 10.0 && dist > 5.0 && img.get_pixel(x, y)[3] < 100 {
                let alpha = ((10.0 - dist) / 5.0 * 120.0) as u8;
                img.put_pixel(x, y, Rgba([255, 100, 0, alpha]));
            }
        }
    }

    // Core fireball
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 5.0 {
                let brightness = 1.0 - (dist / 5.0);
                let r = 255;
                let g = (180.0 + brightness * 75.0) as u8;
                let b = (100.0 * brightness) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
    }

    // White-hot center
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 2.0 {
                img.put_pixel(x, y, Rgba([255, 255, 200, 255]));
            }
        }
    }

    img
}

/// Ice shard - crystalline blue projectile
pub fn create_ice_shard() -> RgbaImage {
    let mut img = RgbaImage::new(PROJECTILE_SIZE, PROJECTILE_SIZE);
    let cx = PROJECTILE_SIZE as f32 / 2.0;
    let cy = PROJECTILE_SIZE as f32 / 2.0;

    // Ice frost aura
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 14.0 && dist > 8.0 {
                let alpha = ((14.0 - dist) / 6.0 * 60.0) as u8;
                img.put_pixel(x, y, Rgba([200, 230, 255, alpha]));
            }
        }
    }

    // Main crystal shard (elongated diamond shape)
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;

            // Diamond shape: |dx| + |dy| < radius, stretched horizontally
            let diamond_dist = dx.abs() / 2.5 + dy.abs();

            if diamond_dist < 5.0 {
                let brightness = 1.0 - (diamond_dist / 5.0);
                let r = (150.0 + brightness * 80.0) as u8;
                let g = (200.0 + brightness * 55.0) as u8;
                let b = 255;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
    }

    // Sharp point at front
    for i in 0..6 {
        let x = (cx + 6.0 + i as f32) as u32;
        if x < PROJECTILE_SIZE {
            img.put_pixel(x, cy as u32, Rgba([220, 240, 255, 255]));
        }
    }

    // Crystal highlight
    img.put_pixel(
        (cx - 2.0) as u32,
        (cy - 1.0) as u32,
        Rgba([255, 255, 255, 255]),
    );
    img.put_pixel(
        (cx - 1.0) as u32,
        (cy - 2.0) as u32,
        Rgba([255, 255, 255, 200]),
    );

    img
}

/// Poison bolt - sickly green projectile with dripping effect
pub fn create_poison_bolt() -> RgbaImage {
    let mut img = RgbaImage::new(PROJECTILE_SIZE, PROJECTILE_SIZE);
    let cx = PROJECTILE_SIZE as f32 / 2.0;
    let cy = PROJECTILE_SIZE as f32 / 2.0;

    // Dripping trail effect
    for i in 0..5 {
        let drop_x = cx - 6.0 - i as f32 * 2.0;
        let drop_y = cy + (i as f32 * 0.5);
        let drop_size = 2.0 - i as f32 * 0.3;
        let alpha = (180 - i * 30) as u8;

        for y in 0..PROJECTILE_SIZE {
            for x in 0..PROJECTILE_SIZE {
                let dx = x as f32 - drop_x;
                let dy = y as f32 - drop_y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < drop_size && img.get_pixel(x, y)[3] == 0 {
                    img.put_pixel(x, y, Rgba([80, 180, 40, alpha]));
                }
            }
        }
    }

    // Toxic aura
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 10.0 && dist > 5.0 && img.get_pixel(x, y)[3] < 50 {
                let alpha = ((10.0 - dist) / 5.0 * 80.0) as u8;
                img.put_pixel(x, y, Rgba([100, 200, 50, alpha]));
            }
        }
    }

    // Main poison blob
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 5.0 {
                let brightness = 1.0 - (dist / 5.0);
                let r = (60.0 + brightness * 60.0) as u8;
                let g = (150.0 + brightness * 80.0) as u8;
                let b = (30.0 + brightness * 40.0) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
    }

    // Bright highlight (toxic gleam)
    img.put_pixel(
        (cx - 1.0) as u32,
        (cy - 2.0) as u32,
        Rgba([180, 255, 100, 255]),
    );

    img
}

/// Lightning bolt - jagged electric yellow bolt
pub fn create_lightning_bolt() -> RgbaImage {
    let mut img = RgbaImage::new(PROJECTILE_SIZE, PROJECTILE_SIZE);
    let cx = PROJECTILE_SIZE as f32 / 2.0;
    let cy = PROJECTILE_SIZE as f32 / 2.0;

    // Electric aura/glow
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 14.0 && dist > 6.0 {
                let alpha = ((14.0 - dist) / 8.0 * 80.0) as u8;
                img.put_pixel(x, y, Rgba([200, 200, 255, alpha]));
            }
        }
    }

    // Main lightning bolt shape (jagged line)
    let bolt_points = [
        (-12.0, 0.0),
        (-6.0, -3.0),
        (-2.0, 2.0),
        (4.0, -2.0),
        (8.0, 1.0),
        (12.0, 0.0),
    ];

    // Draw thick bolt segments
    for i in 0..bolt_points.len() - 1 {
        let (x1, y1) = bolt_points[i];
        let (x2, y2) = bolt_points[i + 1];

        // Draw line between points
        let steps = 20;
        for s in 0..=steps {
            let t = s as f32 / steps as f32;
            let px = cx + x1 + (x2 - x1) * t;
            let py = cy + y1 + (y2 - y1) * t;

            // Draw thick bolt
            for dy in -2i32..=2 {
                for dx in -1i32..=1 {
                    let fx = (px + dx as f32) as u32;
                    let fy = (py + dy as f32) as u32;
                    if fx < PROJECTILE_SIZE && fy < PROJECTILE_SIZE {
                        let dist_from_center = (dy.abs() as f32 + dx.abs() as f32 * 0.5).sqrt();
                        if dist_from_center < 2.0 {
                            let brightness = 1.0 - dist_from_center / 2.0;
                            let r = 255;
                            let g = (220.0 + brightness * 35.0) as u8;
                            let b = (100.0 + brightness * 155.0) as u8;
                            img.put_pixel(fx, fy, Rgba([r, g, b, 255]));
                        }
                    }
                }
            }
        }
    }

    // Bright white core
    for i in 0..bolt_points.len() - 1 {
        let (x1, y1) = bolt_points[i];
        let (x2, y2) = bolt_points[i + 1];

        let steps = 15;
        for s in 0..=steps {
            let t = s as f32 / steps as f32;
            let px = (cx + x1 + (x2 - x1) * t) as u32;
            let py = (cy + y1 + (y2 - y1) * t) as u32;

            if px < PROJECTILE_SIZE && py < PROJECTILE_SIZE {
                img.put_pixel(px, py, Rgba([255, 255, 255, 255]));
            }
        }
    }

    img
}

/// Shadow bolt - dark purple/black projectile with wisps
pub fn create_shadow_bolt() -> RgbaImage {
    let mut img = RgbaImage::new(PROJECTILE_SIZE, PROJECTILE_SIZE);
    let cx = PROJECTILE_SIZE as f32 / 2.0;
    let cy = PROJECTILE_SIZE as f32 / 2.0;

    // Shadow wisps trailing behind
    for i in 0..6 {
        let wisp_x = cx - 5.0 - i as f32 * 2.0;
        let wisp_offset = (i as f32 * 1.5).sin() * 3.0;
        let wisp_size = 3.0 - i as f32 * 0.4;
        let alpha = (150 - i * 20) as u8;

        for y in 0..PROJECTILE_SIZE {
            for x in 0..PROJECTILE_SIZE {
                let dx = x as f32 - wisp_x;
                let dy = y as f32 - (cy + wisp_offset);
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < wisp_size && img.get_pixel(x, y)[3] == 0 {
                    img.put_pixel(x, y, Rgba([60, 30, 80, alpha]));
                }
            }
        }
    }

    // Dark aura
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 12.0 && dist > 6.0 && img.get_pixel(x, y)[3] < 100 {
                let alpha = ((12.0 - dist) / 6.0 * 100.0) as u8;
                img.put_pixel(x, y, Rgba([80, 40, 100, alpha]));
            }
        }
    }

    // Core shadow orb
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 6.0 {
                let darkness = dist / 6.0;
                let r = (40.0 + darkness * 40.0) as u8;
                let g = (20.0 + darkness * 20.0) as u8;
                let b = (60.0 + darkness * 40.0) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
    }

    // Purple highlight (dark gleam)
    img.put_pixel(
        (cx - 2.0) as u32,
        (cy - 2.0) as u32,
        Rgba([150, 100, 180, 200]),
    );
    img.put_pixel(
        (cx - 1.0) as u32,
        (cy - 1.0) as u32,
        Rgba([120, 80, 150, 150]),
    );

    img
}

/// Holy bolt - radiant golden/white projectile
pub fn create_holy_bolt() -> RgbaImage {
    let mut img = RgbaImage::new(PROJECTILE_SIZE, PROJECTILE_SIZE);
    let cx = PROJECTILE_SIZE as f32 / 2.0;
    let cy = PROJECTILE_SIZE as f32 / 2.0;

    // Divine rays emanating outward
    for ray in 0..8 {
        let angle = ray as f32 * std::f32::consts::PI / 4.0;
        let ray_length = 10.0;

        for i in 0..15 {
            let dist = 4.0 + i as f32 * 0.5;
            if dist > ray_length {
                break;
            }

            let rx = cx + angle.cos() * dist;
            let ry = cy + angle.sin() * dist;
            let alpha = (200.0 - i as f32 * 12.0).max(0.0) as u8;

            if rx >= 0.0 && rx < PROJECTILE_SIZE as f32 && ry >= 0.0 && ry < PROJECTILE_SIZE as f32
            {
                let current = img.get_pixel(rx as u32, ry as u32);
                if current[3] < alpha {
                    img.put_pixel(rx as u32, ry as u32, Rgba([255, 240, 180, alpha]));
                }
            }
        }
    }

    // Golden outer glow
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 10.0 && dist > 5.0 {
                let alpha = ((10.0 - dist) / 5.0 * 150.0) as u8;
                let current = img.get_pixel(x, y);
                if current[3] < alpha {
                    img.put_pixel(x, y, Rgba([255, 220, 100, alpha]));
                }
            }
        }
    }

    // Core holy orb
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 5.0 {
                let brightness = 1.0 - (dist / 5.0);
                let r = 255;
                let g = (230.0 + brightness * 25.0) as u8;
                let b = (180.0 + brightness * 75.0) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
    }

    // Pure white center
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 2.0 {
                img.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            }
        }
    }

    img
}

/// Rock projectile - simple brown/gray rock for basic ranged attacks
pub fn create_rock_projectile() -> RgbaImage {
    let mut img = RgbaImage::new(PROJECTILE_SIZE, PROJECTILE_SIZE);
    let cx = PROJECTILE_SIZE as f32 / 2.0;
    let cy = PROJECTILE_SIZE as f32 / 2.0;

    // Main rock body (irregular circle)
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;

            // Add irregularity to the circle
            let angle = dy.atan2(dx);
            let irregular = 1.0 + 0.2 * (angle * 3.0).sin() + 0.1 * (angle * 7.0).cos();
            let dist = (dx * dx + dy * dy).sqrt() / irregular;

            if dist < 6.0 {
                // Rock coloring with some variation
                let shade = 0.7 + 0.3 * (1.0 - dist / 6.0);
                let noise = (x as f32 * 0.5 + y as f32 * 0.7).sin() * 0.1 + 0.9;

                let base_gray = 120.0;
                let r = (base_gray * shade * noise) as u8;
                let g = ((base_gray - 10.0) * shade * noise) as u8;
                let b = ((base_gray - 20.0) * shade * noise) as u8;

                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
    }

    // Highlight on top-left
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - (cx - 2.0);
            let dy = y as f32 - (cy - 2.0);
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 2.5 {
                let current = img.get_pixel(x, y);
                if current[3] > 0 {
                    let r = (current[0] as u16 + 40).min(255) as u8;
                    let g = (current[1] as u16 + 40).min(255) as u8;
                    let b = (current[2] as u16 + 40).min(255) as u8;
                    img.put_pixel(x, y, Rgba([r, g, b, 255]));
                }
            }
        }
    }

    // Dark edge on bottom-right
    for y in 0..PROJECTILE_SIZE {
        for x in 0..PROJECTILE_SIZE {
            let dx = x as f32 - (cx + 2.0);
            let dy = y as f32 - (cy + 2.0);
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 3.0 && dist > 1.0 {
                let current = img.get_pixel(x, y);
                if current[3] > 0 {
                    let r = (current[0] as i16 - 30).max(0) as u8;
                    let g = (current[1] as i16 - 30).max(0) as u8;
                    let b = (current[2] as i16 - 30).max(0) as u8;
                    img.put_pixel(x, y, Rgba([r, g, b, 255]));
                }
            }
        }
    }

    img
}
