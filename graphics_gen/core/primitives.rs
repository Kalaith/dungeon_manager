//! Basic 2D helper drawing primitives (shadow, tile base, noise, outline, rect, circle, line)

use super::super::core::*;
use image::{Rgba, RgbaImage};

// ============================================================================
// HELPER DRAWING FUNCTIONS
// ============================================================================

/// Draw a simple ground shadow (ellipse)
pub fn draw_shadow(img: &mut RgbaImage, cx: f32, cy: f32, rx: f32, ry: f32) {
    let size = img.width() as i32;
    for dy in -(ry as i32)..=(ry as i32) {
        for dx in -(rx as i32)..=(rx as i32) {
            let nx = dx as f32 / rx;
            let ny = dy as f32 / ry;
            if nx * nx + ny * ny <= 1.0 {
                let px = (cx + dx as f32) as i32;
                let py = (cy + dy as f32) as i32;
                if px >= 0 && py >= 0 && px < size && py < size {
                    let pixel = img.get_pixel(px as u32, py as u32);
                    if pixel[3] == 0 {
                        img.put_pixel(px as u32, py as u32, Rgba([20, 20, 25, 100]));
                    }
                }
            }
        }
    }
}

/// Create a tile base with solid color
pub fn create_tile_base(color: Rgba<u8>) -> RgbaImage {
    let mut img = RgbaImage::new(TILE_WIDTH, TILE_HEIGHT);
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            img.put_pixel(x, y, color);
        }
    }
    img
}

/// Add noise to an image for texture
pub fn add_noise(img: &mut RgbaImage, amount: i32) {
    for (x, y, p) in img.enumerate_pixels_mut() {
        if p[3] > 0 {
            let hash = (x.wrapping_mul(2654435761) ^ y.wrapping_mul(2246822519)) as i32;
            let noise = (hash % (amount * 2 + 1)) - amount;
            for i in 0..3 {
                let val = p[i] as i32 + noise;
                p[i] = val.clamp(0, 255) as u8;
            }
        }
    }
}

/// Add outline to non-transparent pixels
pub fn add_outline(img: &mut RgbaImage, outline_color: Rgba<u8>) {
    let width = img.width();
    let height = img.height();
    let mut outline_pixels = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            if pixel[3] > 0 {
                let mut is_edge = false;
                for dy in -1..=1i32 {
                    for dx in -1..=1i32 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0
                            && nx < width as i32
                            && ny >= 0
                            && ny < height as i32
                            && img.get_pixel(nx as u32, ny as u32)[3] == 0
                        {
                            is_edge = true;
                            break;
                        }
                    }
                    if is_edge {
                        break;
                    }
                }
                if is_edge {
                    outline_pixels.push((x, y));
                }
            }
        }
    }

    for (x, y) in outline_pixels {
        img.put_pixel(x, y, outline_color);
    }
}

/// Draw a filled rectangle
pub fn draw_rect(img: &mut RgbaImage, x: u32, y: u32, w: u32, h: u32, color: Rgba<u8>) {
    for dy in 0..h {
        for dx in 0..w {
            let px = x + dx;
            let py = y + dy;
            if px < img.width() && py < img.height() {
                img.put_pixel(px, py, color);
            }
        }
    }
}

/// Draw a filled circle (2D)
pub fn draw_circle(img: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
    for y in 0..img.height() {
        for x in 0..img.width() {
            let dx = (x as i32 - cx as i32).abs();
            let dy = (y as i32 - cy as i32).abs();
            if (dx * dx + dy * dy) < (radius * radius) as i32 {
                img.put_pixel(x, y, color);
            }
        }
    }
}

/// Draw a line between two points
pub fn draw_line(img: &mut RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgba<u8>) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        if x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32 {
            img.put_pixel(x as u32, y as u32, color);
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
