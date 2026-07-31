//! Window icon generation.
//!
//! Emitted at the three sizes `miniquad::conf::Icon` wants (16, 32, 64) from a
//! single description, so the small one is a genuine redraw rather than a
//! downscale — at 16px a resampled dungeon heart turns to mud.

use image::{Rgba, RgbaImage};

/// The dungeon heart: the one object the whole game is about protecting.
///
/// Deliberately only three elements — dark surround, stone rim, burning core —
/// because anything more is unreadable in a 16px taskbar slot.
pub fn create_window_icon(size: u32) -> RgbaImage {
    let mut img = RgbaImage::new(size, size);
    let center = (size as f32 - 1.0) / 2.0;
    // Everything below is expressed as a fraction of the icon so the three
    // sizes are the same picture rather than three drawings that drifted.
    let radius = size as f32 / 2.0;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let dist = (dx * dx + dy * dy).sqrt() / radius;

            let pixel = if dist > 0.98 {
                // Transparent corners: a round icon reads better against any
                // taskbar colour than a square one.
                Rgba([0, 0, 0, 0])
            } else if dist > 0.80 {
                Rgba([26, 22, 30, 255]) // Outer dark
            } else if dist > 0.60 {
                Rgba([64, 58, 72, 255]) // Stone rim
            } else {
                // Burning core, hottest at the centre. Squared falloff keeps
                // the bright part small enough to survive downsizing.
                let heat = (1.0 - dist / 0.60).powi(2);
                Rgba([
                    (150.0 + 105.0 * heat) as u8,
                    (16.0 + 180.0 * heat) as u8,
                    (24.0 + 90.0 * heat) as u8,
                    255,
                ])
            };

            img.put_pixel(x, y, pixel);
        }
    }

    // A single highlight on the rim, top-left, so the icon has a light source.
    let highlight = (size as f32 * 0.28) as u32;
    if highlight > 0 {
        img.put_pixel(highlight, highlight, Rgba([120, 112, 128, 255]));
    }

    img
}

pub fn save_icon(size: u32, img: RgbaImage) {
    let path = format!("assets/ui/icon_{}.png", size);
    img.save(&path).unwrap();
    println!("Generated: {}", path);
}
