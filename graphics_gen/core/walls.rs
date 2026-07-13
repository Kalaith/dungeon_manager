//! Volumetric wall / isometric tile rendering and color blend helpers

use super::super::core::*;
use image::{Rgba, RgbaImage};

// ============================================================================
// VOLUMETRIC WALL / ISOMETRIC TILE RENDERING
// ============================================================================

/// Wall height constant (pixels above floor level)
pub const WALL_HEIGHT: u32 = 24;

/// Create a volumetric wall tile with visible top and front faces
/// This creates the 3D isometric look where walls appear to have depth
pub fn create_volumetric_wall(
    base_color: Rgba<u8>,
    top_color: Option<Rgba<u8>>,
    front_color: Option<Rgba<u8>>,
) -> RgbaImage {
    let mut img = RgbaImage::new(TILE_WIDTH, TILE_HEIGHT);

    let top = top_color.unwrap_or(lighten_color(base_color, 40));
    let front = front_color.unwrap_or(darken_color(base_color, 30));

    // Calculate the visible top height based on isometric projection
    let top_visible_height = (WALL_HEIGHT as f32 * TILT) as u32;

    // Draw the top face (visible due to isometric view)
    for y in 0..top_visible_height.min(TILE_HEIGHT) {
        for x in 0..TILE_WIDTH {
            // Add subtle gradient for 3D feel
            let gradient = y as f32 / top_visible_height as f32;
            let r = (top[0] as f32 * (1.0 - gradient * 0.15)) as u8;
            let g = (top[1] as f32 * (1.0 - gradient * 0.15)) as u8;
            let b = (top[2] as f32 * (1.0 - gradient * 0.15)) as u8;
            img.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }

    // Draw the front face (the main wall surface)
    for y in top_visible_height..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            // Add vertical gradient for depth
            let gradient =
                (y - top_visible_height) as f32 / (TILE_HEIGHT - top_visible_height) as f32;
            let r = (front[0] as f32 * (1.0 - gradient * 0.2)) as u8;
            let g = (front[1] as f32 * (1.0 - gradient * 0.2)) as u8;
            let b = (front[2] as f32 * (1.0 - gradient * 0.2)) as u8;
            img.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }

    // Draw a highlight line at the edge between top and front
    if top_visible_height > 0 && top_visible_height < TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let highlight = lighten_color(top, 30);
            img.put_pixel(x, top_visible_height - 1, highlight);
        }
    }

    img
}

/// Create a carved stone block for dungeon walls
pub fn create_carved_block(
    base_color: Rgba<u8>,
    block_width: u32,
    block_height: u32,
    mortar_color: Rgba<u8>,
    mortar_width: u32,
) -> RgbaImage {
    let mut img = create_volumetric_wall(base_color, None, None);

    let top_visible_height = (WALL_HEIGHT as f32 * TILT) as u32;

    // Add block pattern to the front face
    for y in top_visible_height..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let local_y = y - top_visible_height;

            // Offset every other row for brick pattern
            let row = local_y / block_height;
            let offset = if row % 2 == 1 { block_width / 2 } else { 0 };
            let local_x = (x + offset) % block_width;

            // Draw mortar lines
            if local_x < mortar_width || local_y % block_height < mortar_width {
                img.put_pixel(x, y, mortar_color);
            }
        }
    }

    // Add mortar lines to top face too
    for y in 0..top_visible_height.min(TILE_HEIGHT) {
        for x in 0..TILE_WIDTH {
            let row = y / (block_height / 2).max(1);
            let offset = if row % 2 == 1 { block_width / 2 } else { 0 };
            let local_x = (x + offset) % block_width;

            if local_x < mortar_width || y % (block_height / 2).max(1) < mortar_width {
                let top_mortar = darken_color(mortar_color, 10);
                img.put_pixel(x, y, top_mortar);
            }
        }
    }

    img
}

/// Draw a raised platform/dais for special rooms
pub fn draw_raised_platform(
    img: &mut RgbaImage,
    margin: u32,
    platform_height: u32,
    platform_color: Rgba<u8>,
) {
    let width = img.width();
    let height = img.height();
    let top_offset = (platform_height as f32 * TILT) as u32;

    // Draw platform top
    let top_color = lighten_color(platform_color, 25);
    for y in margin..height.saturating_sub(margin + top_offset) {
        for x in margin..width.saturating_sub(margin) {
            img.put_pixel(x, y, top_color);
        }
    }

    // Draw front edge
    let edge_color = darken_color(platform_color, 15);
    for y in height.saturating_sub(margin + top_offset)..height.saturating_sub(margin) {
        for x in margin..width.saturating_sub(margin) {
            img.put_pixel(x, y, edge_color);
        }
    }

    // Add highlight on front edge top
    for x in margin..width.saturating_sub(margin) {
        let y = height.saturating_sub(margin + top_offset);
        if y < height {
            img.put_pixel(x, y, lighten_color(edge_color, 20));
        }
    }
}

/// Add carved details/insets to a wall
pub fn add_carved_inset(img: &mut RgbaImage, x: u32, y: u32, width: u32, height: u32, depth: i32) {
    let inset_color = if depth > 0 {
        // Recessed - darker
        Rgba([0, 0, 0, (depth.min(50) * 3) as u8])
    } else {
        // Raised - lighter
        Rgba([255, 255, 255, ((-depth).min(30) * 2) as u8])
    };

    for dy in 0..height {
        for dx in 0..width {
            let px = x + dx;
            let py = y + dy;
            if px < img.width() && py < img.height() {
                let current = *img.get_pixel(px, py);
                img.put_pixel(px, py, blend_colors(current, inset_color));
            }
        }
    }

    // Add edge highlights/shadows for 3D effect
    if depth > 0 {
        // Top and left edges are shadowed (light blocked)
        for dx in 0..width {
            if x + dx < img.width() && y > 0 && y < img.height() {
                let current = *img.get_pixel(x + dx, y);
                img.put_pixel(x + dx, y, blend_colors(current, Rgba([0, 0, 0, 40])));
            }
        }
    } else {
        // Top and left edges are highlighted (catching light)
        for dx in 0..width {
            if x + dx < img.width() && y < img.height() {
                let current = *img.get_pixel(x + dx, y);
                img.put_pixel(x + dx, y, blend_colors(current, Rgba([255, 255, 255, 30])));
            }
        }
    }
}

/// Lighten a color by a given amount
pub fn lighten_color(color: Rgba<u8>, amount: i32) -> Rgba<u8> {
    Rgba([
        (color[0] as i32 + amount).clamp(0, 255) as u8,
        (color[1] as i32 + amount).clamp(0, 255) as u8,
        (color[2] as i32 + amount).clamp(0, 255) as u8,
        color[3],
    ])
}

/// Darken a color by a given amount
pub fn darken_color(color: Rgba<u8>, amount: i32) -> Rgba<u8> {
    Rgba([
        (color[0] as i32 - amount).clamp(0, 255) as u8,
        (color[1] as i32 - amount).clamp(0, 255) as u8,
        (color[2] as i32 - amount).clamp(0, 255) as u8,
        color[3],
    ])
}

/// Blend two colors with alpha
pub fn blend_colors(base: Rgba<u8>, overlay: Rgba<u8>) -> Rgba<u8> {
    let alpha = overlay[3] as f32 / 255.0;
    let inv_alpha = 1.0 - alpha;
    Rgba([
        (base[0] as f32 * inv_alpha + overlay[0] as f32 * alpha) as u8,
        (base[1] as f32 * inv_alpha + overlay[1] as f32 * alpha) as u8,
        (base[2] as f32 * inv_alpha + overlay[2] as f32 * alpha) as u8,
        255,
    ])
}
