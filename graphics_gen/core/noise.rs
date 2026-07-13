//! Advanced procedural noise & texture functions

use super::super::core::*;
use super::walls::blend_colors;
use image::{Rgba, RgbaImage};

// ============================================================================
// ADVANCED PROCEDURAL NOISE & TEXTURE FUNCTIONS
// ============================================================================

/// Hash function for procedural noise (returns 0.0 to 1.0)
fn hash2d(x: i32, y: i32) -> f32 {
    let n = x
        .wrapping_mul(374761393)
        .wrapping_add(y.wrapping_mul(668265263));
    let n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    ((n ^ (n >> 16)) as u32) as f32 / u32::MAX as f32
}

/// Smooth interpolation (smoothstep)
fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Value noise at a given position (returns -1.0 to 1.0)
pub fn value_noise(x: f32, y: f32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let xf = x - xi as f32;
    let yf = y - yi as f32;

    let tx = smoothstep(xf);
    let ty = smoothstep(yf);

    let c00 = hash2d(xi, yi);
    let c10 = hash2d(xi + 1, yi);
    let c01 = hash2d(xi, yi + 1);
    let c11 = hash2d(xi + 1, yi + 1);

    let a = c00 + (c10 - c00) * tx;
    let b = c01 + (c11 - c01) * tx;

    (a + (b - a) * ty) * 2.0 - 1.0
}

/// Fractal Brownian Motion - multi-octave noise for natural textures
/// Returns values roughly in -1.0 to 1.0 range
pub fn fbm_noise(x: f32, y: f32, octaves: u32, lacunarity: f32, persistence: f32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        value += amplitude * value_noise(x * frequency, y * frequency);
        max_value += amplitude;
        amplitude *= persistence;
        frequency *= lacunarity;
    }

    value / max_value
}

/// Generate turbulence (absolute value of fbm) for more dramatic textures
pub fn turbulence(x: f32, y: f32, octaves: u32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        value += amplitude * value_noise(x * frequency, y * frequency).abs();
        max_value += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    value / max_value
}

/// Add multi-octave noise to an image for richer natural textures
pub fn add_fbm_noise(img: &mut RgbaImage, scale: f32, octaves: u32, intensity: i32, seed: u32) {
    let offset = seed as f32 * 100.0;
    for (x, y, p) in img.enumerate_pixels_mut() {
        if p[3] > 0 {
            let nx = (x as f32 + offset) / scale;
            let ny = (y as f32 + offset) / scale;
            let noise = fbm_noise(nx, ny, octaves, 2.0, 0.5);
            let adjustment = (noise * intensity as f32) as i32;
            for i in 0..3 {
                let val = p[i] as i32 + adjustment;
                p[i] = val.clamp(0, 255) as u8;
            }
        }
    }
}

/// Add color variation - shifts hue/saturation slightly for more organic look
pub fn add_color_variation(img: &mut RgbaImage, hue_range: i32, sat_range: i32, seed: u32) {
    let offset = seed as f32 * 50.0;
    for (x, y, p) in img.enumerate_pixels_mut() {
        if p[3] > 0 {
            let noise = value_noise((x as f32 + offset) / 16.0, (y as f32 + offset) / 16.0);
            let hue_shift = (noise * hue_range as f32) as i32;
            let sat_shift = (noise * sat_range as f32) as i32;

            // Simple RGB shift to approximate hue/sat change
            p[0] = (p[0] as i32 + hue_shift).clamp(0, 255) as u8;
            p[1] = (p[1] as i32 + sat_shift).clamp(0, 255) as u8;
            p[2] = (p[2] as i32 - hue_shift / 2).clamp(0, 255) as u8;
        }
    }
}

/// Add edge darkening/bevel effect to tile borders
pub fn add_edge_bevel(
    img: &mut RgbaImage,
    bevel_size: u32,
    darken_amount: i32,
    lighten_amount: i32,
) {
    let width = img.width();
    let height = img.height();

    for y in 0..height {
        for x in 0..width {
            let p = img.get_pixel(x, y);
            if p[3] == 0 {
                continue;
            }

            // Distance from edges
            let dist_left = x;
            let dist_right = width - 1 - x;
            let dist_top = y;
            let dist_bottom = height - 1 - y;
            let min_dist = dist_left.min(dist_right).min(dist_top).min(dist_bottom);

            if min_dist < bevel_size {
                let factor = min_dist as f32 / bevel_size as f32;

                // Top/left edges get lighter (highlight)
                // Bottom/right edges get darker (shadow)
                let is_top_left = x < bevel_size || y < bevel_size;
                let adjustment = if is_top_left {
                    ((1.0 - factor) * lighten_amount as f32) as i32
                } else {
                    -((1.0 - factor) * darken_amount as f32) as i32
                };

                let mut new_pixel = *p;
                for i in 0..3 {
                    new_pixel[i] = (p[i] as i32 + adjustment).clamp(0, 255) as u8;
                }
                img.put_pixel(x, y, new_pixel);
            }
        }
    }
}

/// Add stone/brick crack patterns
pub fn add_crack_pattern(img: &mut RgbaImage, density: f32, crack_color: Rgba<u8>, seed: u32) {
    let width = img.width();
    let height = img.height();
    let offset = seed as f32 * 100.0;

    for y in 0..height {
        for x in 0..width {
            let p = img.get_pixel(x, y);
            if p[3] == 0 {
                continue;
            }

            // Use turbulence for crack pattern
            let nx = (x as f32 + offset) / 8.0;
            let ny = (y as f32 + offset) / 8.0;
            let turb = turbulence(nx, ny, 4);

            // Create thin lines where turbulence crosses threshold
            if turb > 0.6 && turb < 0.65 && hash2d(x as i32, y as i32) < density {
                img.put_pixel(x, y, blend_colors(*p, crack_color));
            }
        }
    }
}

/// Create a gradient overlay (useful for lighting effects)
pub fn add_gradient_overlay(
    img: &mut RgbaImage,
    direction: GradientDirection,
    start_alpha: u8,
    end_alpha: u8,
    color: Rgba<u8>,
) {
    let width = img.width();
    let height = img.height();

    for y in 0..height {
        for x in 0..width {
            let p = img.get_pixel(x, y);
            if p[3] == 0 {
                continue;
            }

            let t = match direction {
                GradientDirection::TopToBottom => y as f32 / height as f32,
                GradientDirection::LeftToRight => x as f32 / width as f32,
                GradientDirection::TopLeftToBottomRight => {
                    (x as f32 + y as f32) / (width + height) as f32
                }
                GradientDirection::Radial => {
                    let cx = width as f32 / 2.0;
                    let cy = height as f32 / 2.0;
                    let dx = x as f32 - cx;
                    let dy = y as f32 - cy;
                    (dx * dx + dy * dy).sqrt() / (cx.max(cy))
                }
            };

            let alpha =
                (start_alpha as f32 + (end_alpha as f32 - start_alpha as f32) * t.min(1.0)) as u8;
            let overlay = Rgba([color[0], color[1], color[2], alpha]);
            img.put_pixel(x, y, blend_colors(*p, overlay));
        }
    }
}

/// Direction for gradient overlays
#[derive(Clone, Copy)]
pub enum GradientDirection {
    TopToBottom,
    LeftToRight,
    TopLeftToBottomRight,
    Radial,
}

/// Draw a 3D beveled border around the tile for depth
pub fn add_3d_border(
    img: &mut RgbaImage,
    border_width: u32,
    light_color: Rgba<u8>,
    shadow_color: Rgba<u8>,
) {
    let width = img.width();
    let height = img.height();

    // Top and left edges (lighter - facing light)
    for i in 0..border_width {
        let intensity = 1.0 - (i as f32 / border_width as f32);
        let alpha = (intensity * light_color[3] as f32) as u8;
        let overlay = Rgba([light_color[0], light_color[1], light_color[2], alpha]);

        // Top edge
        for x in i..(width - i) {
            let p = img.get_pixel(x, i);
            img.put_pixel(x, i, blend_colors(*p, overlay));
        }
        // Left edge
        for y in i..(height - i) {
            let p = img.get_pixel(i, y);
            img.put_pixel(i, y, blend_colors(*p, overlay));
        }
    }

    // Bottom and right edges (darker - in shadow)
    for i in 0..border_width {
        let intensity = 1.0 - (i as f32 / border_width as f32);
        let alpha = (intensity * shadow_color[3] as f32) as u8;
        let overlay = Rgba([shadow_color[0], shadow_color[1], shadow_color[2], alpha]);

        // Bottom edge
        for x in i..(width - i) {
            let p = img.get_pixel(x, height - 1 - i);
            img.put_pixel(x, height - 1 - i, blend_colors(*p, overlay));
        }
        // Right edge
        for y in i..(height - i) {
            let p = img.get_pixel(width - 1 - i, y);
            img.put_pixel(width - 1 - i, y, blend_colors(*p, overlay));
        }
    }
}

/// Add specular highlights for wet/shiny surfaces
pub fn add_specular_highlights(img: &mut RgbaImage, intensity: f32, spread: f32, seed: u32) {
    let width = img.width();
    let height = img.height();
    let offset = seed as f32 * 100.0;

    for y in 0..height {
        for x in 0..width {
            let p = img.get_pixel(x, y);
            if p[3] == 0 {
                continue;
            }

            let nx = (x as f32 + offset) / (spread * TILE_SIZE as f32);
            let ny = (y as f32 + offset) / (spread * TILE_SIZE as f32);
            let noise = value_noise(nx, ny);

            if noise > 0.7 {
                let highlight = ((noise - 0.7) / 0.3 * intensity * 255.0) as u8;
                let overlay = Rgba([255, 255, 255, highlight]);
                img.put_pixel(x, y, blend_colors(*p, overlay));
            }
        }
    }
}
