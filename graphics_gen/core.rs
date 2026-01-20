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

// Light direction (normalized) - from top-left-front
const LIGHT_X: f32 = -0.5;
const LIGHT_Y: f32 = -0.7;
const LIGHT_Z: f32 = 0.5;

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

/// Draw a 3D shaded sphere
pub fn draw_sphere_3d(
    img: &mut RgbaImage,
    depth: &mut DepthBuffer,
    cx: f32,
    cy: f32,
    cz: f32,
    radius: f32,
    mat: &Material,
) {
    let r_int = radius.ceil() as i32;
    let size = img.width() as i32;

    for dy in -r_int..=r_int {
        for dx in -r_int..=r_int {
            let dist_sq = (dx * dx + dy * dy) as f32;
            if dist_sq <= radius * radius {
                let px = (cx + dx as f32) as i32;
                let py = (cy + dy as f32) as i32;
                if px >= 0 && py >= 0 && px < size && py < size {
                    let z_offset = (radius * radius - dist_sq).sqrt();
                    let pz = cz + z_offset;
                    let normal = (dx as f32 / radius, dy as f32 / radius, z_offset / radius);

                    if depth.test_and_set(px as u32, py as u32, pz) {
                        let color = shade_color(normal, mat, pz);
                        img.put_pixel(px as u32, py as u32, color);
                    }
                }
            }
        }
    }
}

/// Draw a 3D shaded ellipsoid
pub fn draw_ellipsoid_3d(
    img: &mut RgbaImage,
    depth: &mut DepthBuffer,
    cx: f32,
    cy: f32,
    cz: f32,
    rx: f32,
    ry: f32,
    rz: f32,
    mat: &Material,
) {
    let max_r = rx.max(ry).max(rz).ceil() as i32;
    let size = img.width() as i32;

    for dy in -max_r..=max_r {
        for dx in -max_r..=max_r {
            let nx = dx as f32 / rx;
            let ny = dy as f32 / ry;
            let dist_sq = nx * nx + ny * ny;
            if dist_sq <= 1.0 {
                let px = (cx + dx as f32) as i32;
                let py = (cy + dy as f32) as i32;
                if px >= 0 && py >= 0 && px < size && py < size {
                    let nz = (1.0 - dist_sq).sqrt();
                    let pz = cz + nz * rz;
                    let normal = (nx, ny, nz);
                    if depth.test_and_set(px as u32, py as u32, pz) {
                        img.put_pixel(px as u32, py as u32, shade_color(normal, mat, pz));
                    }
                }
            }
        }
    }
}

/// Draw a 3D shaded cylinder (vertical)
pub fn draw_cylinder_3d(
    img: &mut RgbaImage,
    depth: &mut DepthBuffer,
    cx: f32,
    y_top: f32,
    y_bot: f32,
    cz: f32,
    radius: f32,
    mat: &Material,
) {
    let r_int = radius.ceil() as i32;
    let size = img.width() as i32;

    for y in (y_top as i32)..=(y_bot as i32) {
        for dx in -r_int..=r_int {
            if (dx as f32).abs() <= radius {
                let px = (cx + dx as f32) as i32;
                if px >= 0 && y >= 0 && px < size && y < size {
                    let z_offset = (radius * radius - (dx as f32) * (dx as f32)).sqrt().max(0.0);
                    let pz = cz + z_offset;
                    let normal = (dx as f32 / radius, 0.0, z_offset / radius);
                    if depth.test_and_set(px as u32, y as u32, pz) {
                        img.put_pixel(px as u32, y as u32, shade_color(normal, mat, pz));
                    }
                }
            }
        }
    }
}

/// Draw a 3D cone (pointing up)
pub fn draw_cone_3d(
    img: &mut RgbaImage,
    depth: &mut DepthBuffer,
    cx: f32,
    y_tip: f32,
    y_base: f32,
    cz: f32,
    base_radius: f32,
    mat: &Material,
) {
    let height = y_base - y_tip;
    if height <= 0.0 {
        return;
    }
    let size = img.width() as i32;

    for y in (y_tip as i32)..=(y_base as i32) {
        let t = (y as f32 - y_tip) / height;
        let r = base_radius * t;
        let r_int = r.ceil() as i32;
        for dx in -r_int..=r_int {
            if (dx as f32).abs() <= r && r > 0.0 {
                let px = (cx + dx as f32) as i32;
                if px >= 0 && y >= 0 && px < size && y < size {
                    let z_offset = (r * r - (dx as f32) * (dx as f32)).sqrt().max(0.0);
                    let pz = cz + z_offset;
                    // Cone normal tilts outward and up
                    let slope = base_radius / height;
                    let normal = (dx as f32 / r * slope, -slope, z_offset / r);
                    if depth.test_and_set(px as u32, y as u32, pz) {
                        img.put_pixel(px as u32, y as u32, shade_color(normal, mat, pz));
                    }
                }
            }
        }
    }
}

/// Draw a 3D torus (donut shape) - NEW
pub fn draw_torus_3d(
    img: &mut RgbaImage,
    depth: &mut DepthBuffer,
    cx: f32,
    cy: f32,
    cz: f32,
    major_radius: f32,  // Distance from center to tube center
    minor_radius: f32,  // Radius of the tube
    mat: &Material,
) {
    let size = img.width() as i32;
    let total_r = (major_radius + minor_radius).ceil() as i32;

    for dy in -total_r..=total_r {
        for dx in -total_r..=total_r {
            let px = (cx + dx as f32) as i32;
            let py = (cy + dy as f32) as i32;

            if px >= 0 && py >= 0 && px < size && py < size {
                // Distance from center in xy plane
                let dist_xy = ((dx * dx + dy * dy) as f32).sqrt();

                // Distance from the tube center
                let tube_center_dist = (dist_xy - major_radius).abs();

                if tube_center_dist <= minor_radius {
                    // Z offset on the tube surface
                    let z_offset = (minor_radius * minor_radius - tube_center_dist * tube_center_dist).sqrt();
                    let pz = cz + z_offset;

                    // Normal calculation for torus
                    let nx = if dist_xy > 0.0 { dx as f32 / dist_xy } else { 0.0 };
                    let ny = if dist_xy > 0.0 { dy as f32 / dist_xy } else { 0.0 };
                    let tube_nx = (dist_xy - major_radius) / minor_radius;
                    let tube_nz = z_offset / minor_radius;

                    let normal = (nx * tube_nx, ny * tube_nx, tube_nz);

                    if depth.test_and_set(px as u32, py as u32, pz) {
                        img.put_pixel(px as u32, py as u32, shade_color(normal, mat, pz));
                    }
                }
            }
        }
    }
}

/// Draw a 3D box/cuboid - NEW
pub fn draw_box_3d(
    img: &mut RgbaImage,
    depth: &mut DepthBuffer,
    cx: f32,
    cy: f32,
    cz: f32,
    half_width: f32,
    half_height: f32,
    half_depth: f32,
    mat: &Material,
) {
    let size = img.width() as i32;

    // Draw front face (facing camera, normal = (0, 0, 1))
    let front_z = cz + half_depth;
    for dy in -(half_height as i32)..=(half_height as i32) {
        for dx in -(half_width as i32)..=(half_width as i32) {
            let px = (cx + dx as f32) as i32;
            let py = (cy + dy as f32) as i32;
            if px >= 0 && py >= 0 && px < size && py < size {
                if depth.test_and_set(px as u32, py as u32, front_z) {
                    img.put_pixel(px as u32, py as u32, shade_color((0.0, 0.0, 1.0), mat, front_z));
                }
            }
        }
    }

    // Draw top face (normal = (0, -1, 0))
    let top_y_start = cy - half_height;
    for dz in 0..=(half_depth as i32) {
        let py = (top_y_start - dz as f32 * 0.5) as i32; // Fake perspective
        let pz = cz + half_depth - dz as f32;
        for dx in -(half_width as i32)..=(half_width as i32) {
            let px = (cx + dx as f32) as i32;
            if px >= 0 && py >= 0 && px < size && py < size {
                if depth.test_and_set(px as u32, py as u32, pz) {
                    img.put_pixel(px as u32, py as u32, shade_color((0.0, -1.0, 0.5), mat, pz));
                }
            }
        }
    }
}

/// Draw a 3D wedge/ramp - NEW
pub fn draw_wedge_3d(
    img: &mut RgbaImage,
    depth: &mut DepthBuffer,
    cx: f32,
    y_top: f32,
    y_bot: f32,
    cz: f32,
    width: f32,
    depth_val: f32,
    mat: &Material,
) {
    let height = y_bot - y_top;
    if height <= 0.0 {
        return;
    }
    let size = img.width() as i32;
    let half_w = width / 2.0;

    for y in (y_top as i32)..=(y_bot as i32) {
        let t = (y as f32 - y_top) / height;
        let current_depth = depth_val * (1.0 - t);

        for dx in -(half_w as i32)..=(half_w as i32) {
            let px = (cx + dx as f32) as i32;
            if px >= 0 && y >= 0 && px < size && y < size {
                let pz = cz + current_depth;
                // Ramp normal tilts backward
                let normal = (0.0, -depth_val / height, 1.0);
                if depth.test_and_set(px as u32, y as u32, pz) {
                    img.put_pixel(px as u32, y as u32, shade_color(normal, mat, pz));
                }
            }
        }
    }
}

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
