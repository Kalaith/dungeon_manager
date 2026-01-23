//! Core 3D rendering infrastructure for procedural graphics generation
//!
//! This module provides:
//! - Material system with various presets (matte, metallic, glowing, etc.)
//! - Depth buffer for proper 3D overlap
//! - 3D primitive drawing functions (sphere, ellipsoid, cylinder, cone, torus, box)
//! - Shading with Blinn-Phong lighting model

use image::{Rgba, RgbaImage};

// ============================================================================
// CONSTANTS
// ============================================================================

pub const TILE_SIZE: u32 = 64;
pub const TILE_WIDTH: u32 = 64;
pub const TILE_HEIGHT: u32 = 64;
pub const SPRITE_SIZE: u32 = 64;
pub const PROJECTILE_SIZE: u32 = 32;

// Projection constants
pub const TILT: f32 = 0.5; // Z-axis foreshortening factor (0.5 means 1 unit up = 0.5 units up-screen)

// Light direction (normalized) - from top-left-front
const LIGHT_X: f32 = -0.4;
const LIGHT_Y: f32 = -0.5;
const LIGHT_Z: f32 = 0.75;

// ============================================================================
// MATERIAL SYSTEM
// ============================================================================

/// Material properties for 3D shading
#[derive(Clone, Copy)]
pub struct Material {
    pub base_color: [u8; 3],
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    /// Create a custom material with full control
    pub fn new(r: u8, g: u8, b: u8, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Material {
            base_color: [r, g, b],
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    /// Matte material - no specular highlights, good for cloth, skin, etc.
    pub fn matte(r: u8, g: u8, b: u8) -> Self {
        Material {
            base_color: [r, g, b],
            ambient: 0.3,
            diffuse: 0.7,
            specular: 0.0,
            shininess: 1.0,
        }
    }

    /// Metallic material - strong specular highlights
    pub fn metallic(r: u8, g: u8, b: u8) -> Self {
        Material {
            base_color: [r, g, b],
            ambient: 0.2,
            diffuse: 0.5,
            specular: 0.8,
            shininess: 32.0,
        }
    }

    /// Glowing/emissive material - high ambient, appears to emit light
    pub fn glowing(r: u8, g: u8, b: u8) -> Self {
        Material {
            base_color: [r, g, b],
            ambient: 0.9,
            diffuse: 0.1,
            specular: 0.3,
            shininess: 8.0,
        }
    }

    /// Bone material - off-white with subtle sheen
    pub fn bone() -> Self {
        Material {
            base_color: [240, 235, 220],
            ambient: 0.4,
            diffuse: 0.6,
            specular: 0.2,
            shininess: 4.0,
        }
    }

    /// Leather material - matte with very slight sheen
    pub fn leather(r: u8, g: u8, b: u8) -> Self {
        Material {
            base_color: [r, g, b],
            ambient: 0.25,
            diffuse: 0.7,
            specular: 0.1,
            shininess: 2.0,
        }
    }

    /// Stone material - rough with minimal highlights
    pub fn stone(r: u8, g: u8, b: u8) -> Self {
        Material {
            base_color: [r, g, b],
            ambient: 0.35,
            diffuse: 0.65,
            specular: 0.05,
            shininess: 2.0,
        }
    }

    /// Glass/crystal material - transparent look with high specular
    pub fn crystal(r: u8, g: u8, b: u8) -> Self {
        Material {
            base_color: [r, g, b],
            ambient: 0.4,
            diffuse: 0.3,
            specular: 0.9,
            shininess: 64.0,
        }
    }

    /// Wood material - warm tones with subtle grain appearance
    pub fn wood(r: u8, g: u8, b: u8) -> Self {
        Material {
            base_color: [r, g, b],
            ambient: 0.3,
            diffuse: 0.65,
            specular: 0.15,
            shininess: 4.0,
        }
    }

    /// Flesh/organic material - subsurface scattering approximation
    pub fn flesh(r: u8, g: u8, b: u8) -> Self {
        Material {
            base_color: [r, g, b],
            ambient: 0.35,
            diffuse: 0.6,
            specular: 0.15,
            shininess: 8.0,
        }
    }

    /// Fire/flame material - very bright emissive
    pub fn fire() -> Self {
        Material {
            base_color: [255, 120, 20],
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.2,
            shininess: 4.0,
        }
    }

    /// Ice material - blue-white with high specular
    pub fn ice() -> Self {
        Material {
            base_color: [200, 230, 255],
            ambient: 0.4,
            diffuse: 0.4,
            specular: 0.7,
            shininess: 48.0,
        }
    }

    /// Shadow/void material - very dark, minimal lighting response
    pub fn shadow() -> Self {
        Material {
            base_color: [20, 20, 30],
            ambient: 0.8,
            diffuse: 0.2,
            specular: 0.0,
            shininess: 1.0,
        }
    }
}

// ============================================================================
// DEPTH BUFFER
// ============================================================================

/// Depth buffer for proper 3D overlap handling
pub struct DepthBuffer {
    data: Vec<f32>,
    width: usize,
    height: usize,
}

impl DepthBuffer {
    pub fn new(w: u32, h: u32) -> Self {
        DepthBuffer {
            data: vec![f32::NEG_INFINITY; (w * h) as usize],
            width: w as usize,
            height: h as usize,
        }
    }

    /// Test if the given z value is closer than what's in the buffer, and update if so
    pub fn test_and_set(&mut self, x: u32, y: u32, z: f32) -> bool {
        if x >= self.width as u32 || y >= self.height as u32 {
            return false;
        }
        let idx = y as usize * self.width + x as usize;
        if z > self.data[idx] {
            self.data[idx] = z;
            true
        } else {
            false
        }
    }

    /// Get the current depth value at a position
    pub fn get(&self, x: u32, y: u32) -> f32 {
        if x >= self.width as u32 || y >= self.height as u32 {
            return f32::NEG_INFINITY;
        }
        let idx = y as usize * self.width + x as usize;
        self.data[idx]
    }

    /// Clear the depth buffer
    pub fn clear(&mut self) {
        for d in &mut self.data {
            *d = f32::NEG_INFINITY;
        }
    }
}

// ============================================================================
// SHADING
// ============================================================================

/// Calculate shading for a surface normal using Blinn-Phong lighting
pub fn shade_color(normal: (f32, f32, f32), mat: &Material, _view_z: f32) -> Rgba<u8> {
    // Normalize normal
    let len = (normal.0 * normal.0 + normal.1 * normal.1 + normal.2 * normal.2).sqrt();
    let (nx, ny, nz) = if len > 0.0 {
        (normal.0 / len, normal.1 / len, normal.2 / len)
    } else {
        (0.0, 0.0, 1.0)
    };

    // Diffuse lighting (Lambert)
    let dot = -(nx * LIGHT_X + ny * LIGHT_Y + nz * LIGHT_Z);
    let diffuse = dot.max(0.0) * mat.diffuse;

    // Specular (Blinn-Phong)
    let hx = -LIGHT_X;
    let hy = -LIGHT_Y;
    let hz = -LIGHT_Z + 1.0;
    let hlen = (hx * hx + hy * hy + hz * hz).sqrt();
    let spec_dot = (nx * hx / hlen + ny * hy / hlen + nz * hz / hlen).max(0.0);
    let specular = spec_dot.powf(mat.shininess) * mat.specular;

    let intensity = (mat.ambient + diffuse + specular).min(1.5);

    let r = ((mat.base_color[0] as f32 * intensity).min(255.0)) as u8;
    let g = ((mat.base_color[1] as f32 * intensity).min(255.0)) as u8;
    let b = ((mat.base_color[2] as f32 * intensity).min(255.0)) as u8;

    Rgba([r, g, b, 255])
}

// ============================================================================
// 3D PRIMITIVE DRAWING
// ============================================================================


// ============================================================================
// 3D PRIMITIVE DRAWING
// ============================================================================

// Re-export primitives moved to separate module
pub use crate::graphics_gen::primitives::*;


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
            let hash = ((x as u32).wrapping_mul(2654435761) ^ (y as u32).wrapping_mul(2246822519)) as i32;
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
                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            if img.get_pixel(nx as u32, ny as u32)[3] == 0 {
                                is_edge = true;
                                break;
                            }
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
            let gradient = (y - top_visible_height) as f32 / (TILE_HEIGHT - top_visible_height) as f32;
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
pub fn add_carved_inset(
    img: &mut RgbaImage,
    x: u32, y: u32,
    width: u32, height: u32,
    depth: i32,
) {
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

// ============================================================================
// ADVANCED PROCEDURAL NOISE & TEXTURE FUNCTIONS
// ============================================================================

/// Hash function for procedural noise (returns 0.0 to 1.0)
fn hash2d(x: i32, y: i32) -> f32 {
    let n = x.wrapping_mul(374761393).wrapping_add(y.wrapping_mul(668265263));
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
pub fn add_edge_bevel(img: &mut RgbaImage, bevel_size: u32, darken_amount: i32, lighten_amount: i32) {
    let width = img.width();
    let height = img.height();
    
    for y in 0..height {
        for x in 0..width {
            let p = img.get_pixel(x, y);
            if p[3] == 0 { continue; }
            
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
            if p[3] == 0 { continue; }
            
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
pub fn add_gradient_overlay(img: &mut RgbaImage, direction: GradientDirection, start_alpha: u8, end_alpha: u8, color: Rgba<u8>) {
    let width = img.width();
    let height = img.height();
    
    for y in 0..height {
        for x in 0..width {
            let p = img.get_pixel(x, y);
            if p[3] == 0 { continue; }
            
            let t = match direction {
                GradientDirection::TopToBottom => y as f32 / height as f32,
                GradientDirection::LeftToRight => x as f32 / width as f32,
                GradientDirection::TopLeftToBottomRight => (x as f32 + y as f32) / (width + height) as f32,
                GradientDirection::Radial => {
                    let cx = width as f32 / 2.0;
                    let cy = height as f32 / 2.0;
                    let dx = x as f32 - cx;
                    let dy = y as f32 - cy;
                    (dx * dx + dy * dy).sqrt() / (cx.max(cy))
                }
            };
            
            let alpha = (start_alpha as f32 + (end_alpha as f32 - start_alpha as f32) * t.min(1.0)) as u8;
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
pub fn add_3d_border(img: &mut RgbaImage, border_width: u32, light_color: Rgba<u8>, shadow_color: Rgba<u8>) {
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
            if p[3] == 0 { continue; }
            
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
