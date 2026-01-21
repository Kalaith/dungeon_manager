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

/// Solve quadratic equation: at^2 + bt + c = 0
/// Returns the largest valid root (closest to camera/top), if any
fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<f32> {
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        None
    } else {
        let sqrt_disc = disc.sqrt();
        // We generally want the largest Z, which often corresponds to the "front" or "top"
        // depending on the exact setup.
        // In our projection:
        // higher Z = higher "altitude"
        // We want the largest Z that is visible.
        Some((-b + sqrt_disc) / (2.0 * a))
    }
}

/// Draw a 3D shaded sphere using analytical intersection
pub fn draw_sphere_3d(
    img: &mut RgbaImage,
    depth: &mut DepthBuffer,
    cx: f32,
    cy: f32,
    cz: f32,
    radius: f32,
    mat: &Material,
) {
    // Bounding box in world space ... mapped to screen?
    // Screen footprint:
    // sphere at (cx, cy, cz) projects to (cx, cy - cz * TILT)
    // but the Z extent of sphere is [cz-r, cz+r].
    // So Y range is [cy - (cz+r)*TILT - r, cy - (cz-r)*TILT + r] roughly.
    // It's safer to just iterate a conservative box.
    // Center on screen:
    let screen_cx = cx;
    let screen_cy = cy - cz * TILT;
    // The "radius" on screen is somewhat complex due to tilt, but a conservative box is:
    // width: r
    // height: r + r * TILT
    let bb_w = (radius + 2.0).ceil() as i32;
    let bb_h = (radius * (1.0 + TILT) + 2.0).ceil() as i32;

    let size_w = img.width() as i32;
    let size_h = img.height() as i32;

    for dy in -bb_h..=bb_h {
        for dx in -bb_w..=bb_w {
            let px = screen_cx as i32 + dx;
            let py = screen_cy as i32 + dy;

            if px < 0 || px >= size_w || py < 0 || py >= size_h {
                continue;
            }

            // Ray: wx = px
            //      wy = py + wz * TILT
            // Sphere: (wx - cx)^2 + (wy - cy)^2 + (wz - cz)^2 = r^2
            // Substitute:
            // (px - cx)^2 + (py + wz*TILT - cy)^2 + (wz - cz)^2 = r^2
            // Let dx_val = px - cx
            // Let dy_val = py - cy
            // dx_val^2 + (dy_val + wz*TILT)^2 + (wz - cz)^2 - r^2 = 0
            // dx_val^2 + dy_val^2 + 2*dy_val*wz*TILT + wz^2*TILT^2 + wz^2 - 2*wz*cz + cz^2 - r^2 = 0
            // Group wz:
            // wz^2(TILT^2 + 1) + wz(2*dy_val*TILT - 2*cz) + (dx_val^2 + dy_val^2 + cz^2 - r^2) = 0

            let dx_val = px as f32 - cx;
            let dy_val = py as f32 - cy;

            let a = TILT * TILT + 1.0;
            let b = 2.0 * (dy_val * TILT - cz);
            let c_coeff = dx_val * dx_val + dy_val * dy_val + cz * cz - radius * radius;

            if let Some(wz) = solve_quadratic(a, b, c_coeff) {
                // Check if this point is actually valid... real roots mean intersection.
                let wx = px as f32;
                let wy = py as f32 + wz * TILT;

                // Calculate normal
                let nx = (wx - cx) / radius;
                let ny = (wy - cy) / radius;
                let nz = (wz - cz) / radius;

                if depth.test_and_set(px as u32, py as u32, wz) {
                    img.put_pixel(px as u32, py as u32, shade_color((nx, ny, nz), mat, wz));
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
    let screen_cx = cx;
    let screen_cy = cy - cz * TILT;
    let _max_r = rx.max(ry).max(rz);
    let bb_w = (rx + 2.0).ceil() as i32;
    // Height includes the Z projection
    let bb_h = (ry + rz * TILT + 2.0).ceil() as i32;

    let size_w = img.width() as i32;
    let size_h = img.height() as i32;

    for dy in -bb_h..=bb_h {
        for dx in -bb_w..=bb_w {
            let px = screen_cx as i32 + dx;
            let py = screen_cy as i32 + dy;

            if px < 0 || px >= size_w || py < 0 || py >= size_h {
                continue;
            }

            // Ellipsoid: ((wx-cx)/rx)^2 + ((wy-cy)/ry)^2 + ((wz-cz)/rz)^2 = 1
            // Ray: wx = px, wy = py + wz*TILT
            // Let dx_val = px - cx, dy_val = py - cy
            // (dx_val/rx)^2 + ((dy_val + wz*TILT)/ry)^2 + ((wz-cz)/rz)^2 - 1 = 0

            let dx_val = px as f32 - cx;
            let dy_val = py as f32 - cy;

            // Expand terms:
            // Term 1: dx_val^2 / rx^2
            // Term 2: (dy_val^2 + 2*dy_val*wz*TILT + wz^2*TILT^2) / ry^2
            // Term 3: (wz^2 - 2*wz*cz + cz^2) / rz^2

            // Coeffs for wz^2:
            // TILT^2 / ry^2 + 1 / rz^2
            let a = (TILT * TILT) / (ry * ry) + 1.0 / (rz * rz);

            // Coeffs for wz:
            // 2*dy_val*TILT / ry^2 - 2*cz / rz^2
            let b = (2.0 * dy_val * TILT) / (ry * ry) - (2.0 * cz) / (rz * rz);

            // Constant:
            // dx_val^2/rx^2 + dy_val^2/ry^2 + cz^2/rz^2 - 1
            let c_coeff = (dx_val * dx_val) / (rx * rx) + (dy_val * dy_val) / (ry * ry) + (cz * cz) / (rz * rz) - 1.0;

            if let Some(wz) = solve_quadratic(a, b, c_coeff) {
                let wx = px as f32;
                let wy = py as f32 + wz * TILT;

                // Normal for ellipsoid:
                // n = 2(p-c)/r^2 ... normalized
                let mut nx = (wx - cx) / (rx * rx);
                let mut ny = (wy - cy) / (ry * ry);
                let mut nz = (wz - cz) / (rz * rz);
                let len = (nx * nx + ny * ny + nz * nz).sqrt();
                nx /= len; ny /= len; nz /= len;

                if depth.test_and_set(px as u32, py as u32, wz) {
                    img.put_pixel(px as u32, py as u32, shade_color((nx, ny, nz), mat, wz));
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
    let screen_cx = cx;
    // Projected top and bottom centers
    let p_top_y = y_top - cz * TILT;
    let p_bot_y = y_bot - cz * TILT;
    let min_screen_y = p_top_y.min(p_bot_y) - radius * TILT;
    let max_screen_y = p_top_y.max(p_bot_y) + radius * TILT;

    let bb_w = (radius + 2.0).ceil() as i32;
    let bb_h_top = (y_top as f32 - min_screen_y).abs().ceil() as i32 + 5;
    let bb_h_bot = (max_screen_y - y_top as f32).abs().ceil() as i32 + 5;

    let size_w = img.width() as i32;
    let size_h = img.height() as i32;

    // Bounding box iteration centered around cylinder axis projection
    for dy in -bb_h_top..=bb_h_bot {
        for dx in -bb_w..=bb_w {
            let px = screen_cx as i32 + dx;
            let py = (y_top - cz * TILT) as i32 + dy; // Relative to top center projection

            if px < 0 || px >= size_w || py < 0 || py >= size_h {
                continue;
            }

            // Ray casting for Cylinder x^2 + (z-cz)^2 = r^2
            // wx = px
            // wy = py + wz * TILT
            // x = wx - cx = px - cx
            // z = wz - cz
            // (px - cx)^2 + (wz - cz)^2 = r^2
            //
            // This is actually independent of TILT for the intersection!
            // (wz - cz)^2 = r^2 - (px - cx)^2
            // wz = cz +/- sqrt(r^2 - (px - cx)^2)

            let dx_val = px as f32 - cx;
            let disc = radius * radius - dx_val * dx_val;

            if disc >= 0.0 {
                let sqrt_disc = disc.sqrt();
                // We have two possible Z values on the infinite cylinder: front and back
                // Front is larger Z (closer to viewer)
                let wz_front = cz + sqrt_disc;
                let wz_back = cz - sqrt_disc;

                let mut best_z = f32::NEG_INFINITY;
                let mut best_normal = (0.0, 0.0, 0.0);
                let mut hit = false;

                // 1. Test Cylinder Body
                // For a given wx, wz, we find the corresponding wy on the ray:
                // wy = py + wz * TILT
                // We check if this wy is within [y_top, y_bot]
                // We test both front and back faces, though front usually obscures back
                for wz in [wz_front, wz_back] {
                     let wy = py as f32 + wz * TILT;
                     if wy >= y_top && wy <= y_bot {
                         if wz > best_z {
                             best_z = wz;
                             best_normal = (dx_val / radius, 0.0, (wz - cz) / radius);
                             hit = true;
                         }
                     }
                }

                // 2. Test Top Cap (Plane y = y_top)
                // Ray: wy = py + wz * TILT = y_top
                // wz = (y_top - py) / TILT
                if TILT.abs() > 0.001 {
                    let wz_top = (y_top - (py as f32)) / TILT;
                    let wx_top = px as f32;
                    // Check if inside circle: (wx - cx)^2 + (wz - cz)^2 <= r^2
                    if (wx_top - cx).powi(2) + (wz_top - cz).powi(2) <= radius.powi(2) {
                         if wz_top > best_z {
                             best_z = wz_top;
                             best_normal = (0.0, -1.0, 0.0);
                             hit = true;
                         }
                    }
                }

                // 3. Test Bottom Cap (Plane y = y_bot)
                // Ray: wy = py + wz * TILT = y_bot
                if TILT.abs() > 0.001 {
                    let wz_bot = (y_bot - (py as f32)) / TILT;
                    let wx_bot = px as f32;
                    if (wx_bot - cx).powi(2) + (wz_bot - cz).powi(2) <= radius.powi(2) {
                        // Usually bottom is obscured, but if we see under it or via clipping...
                        // Or if looking from above, we shouldn't see bottom, but let's be correct.
                        // Actually if we look from "top-front", we see top cap.
                         if wz_bot > best_z {
                             best_z = wz_bot;
                             best_normal = (0.0, 1.0, 0.0);
                             hit = true;
                         }
                    }
                }

                if hit {
                    if depth.test_and_set(px as u32, py as u32, best_z) {
                        img.put_pixel(px as u32, py as u32, shade_color(best_normal, mat, best_z));
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
    let screen_cx = cx;
    let height = y_base - y_tip;
    if height <= 0.0 { return; }

    // Rough bounding box
    let slope = base_radius / height;

    let bb_w = (base_radius + 2.0).ceil() as i32;
    // Y-range estimation
    let min_screen_y = y_tip - cz * TILT - 5.0;
    let max_screen_y = y_base - cz * TILT + base_radius * TILT + 5.0;
    let bb_h = (max_screen_y - min_screen_y).ceil() as i32;

    let size_w = img.width() as i32;
    let size_h = img.height() as i32;

    for dy in 0..bb_h {
        let py = min_screen_y as i32 + dy;
        for dx in -bb_w..=bb_w {
            let px = screen_cx as i32 + dx;

            if px < 0 || px >= size_w || py < 0 || py >= size_h {
                continue;
            }

            let mut best_z = f32::NEG_INFINITY;
            let mut best_normal = (0.0, 0.0, 0.0);
            let mut hit = false;

            // Cone: x^2 + z_local^2 = (radius_at_y)^2
            // radius_at_y = (y - y_tip) * slope
            // x = px - cx
            // z_local = wz - cz
            // y = py + wz*TILT
            //
            // (px - cx)^2 + (wz - cz)^2 = ((py + wz*TILT - y_tip) * slope)^2
            // Let dx = px - cx
            // Let dy_tip = py - y_tip
            // dx^2 + (wz - cz)^2 = slope^2 * (dy_tip + wz*TILT)^2
            // dx^2 + wz^2 - 2*wz*cz + cz^2 = slope^2 * (dy_tip^2 + 2*dy_tip*wz*TILT + wz^2*TILT^2)
            //
            // Group wz^2:
            // 1 - slope^2*TILT^2
            //
            // Group wz:
            // -2*cz - slope^2 * 2 * dy_tip * TILT
            //
            // Constant:
            // dx^2 + cz^2 - slope^2 * dy_tip^2

            let dx_val = px as f32 - cx;
            let dy_tip = py as f32 - y_tip;

            let a = 1.0 - slope * slope * TILT * TILT;
            let b = -2.0 * cz - slope * slope * 2.0 * dy_tip * TILT;
            let c_coeff = dx_val * dx_val + cz * cz - slope * slope * dy_tip * dy_tip;

            let disc = b * b - 4.0 * a * c_coeff;

            // 1. Side Intersection
            if disc >= 0.0 {
                let sqrt_disc = disc.sqrt();
                // Candidates
                let z1 = (-b + sqrt_disc) / (2.0 * a);
                let z2 = (-b - sqrt_disc) / (2.0 * a);

                for wz in [z1, z2] {
                    // Check bounds: y must be between y_tip and y_base
                    let wy = py as f32 + wz * TILT;
                    if wy >= y_tip && wy <= y_base {
                        // Check if we are "inside" the cone volume (it's a double cone mathematically)
                        // radius_at_y must be positive
                        if (wy - y_tip) >= 0.0 {
                            if wz > best_z {
                                best_z = wz;
                                // Normal:
                                // Vector along slope.
                                // Horizontal part is (x, z). Vertical is 'slope'.
                                // normalize((x, -r*slope, z)) approximately?
                                // Precise: gradient of f(x,y,z) = x^2 + z^2 - s^2(y-y0)^2 = 0
                                // (2x, -2s^2(y-y0), 2z)
                                let nx = 2.0 * dx_val;
                                let nz = 2.0 * (wz - cz);
                                let ny = -2.0 * slope * slope * (wy - y_tip);
                                let len = (nx*nx + ny*ny + nz*nz).sqrt();
                                best_normal = (nx/len, ny/len, nz/len);
                                hit = true;
                            }
                        }
                    }
                }
            }

            // 2. Base Cap (Plane y = y_base)
            if TILT.abs() > 0.001 {
                let wz_base = (y_base - py as f32) / TILT;
                let wx_base = px as f32;
                // Check dist from axis
                if (wx_base - cx).powi(2) + (wz_base - cz).powi(2) <= base_radius.powi(2) {
                    if wz_base > best_z {
                        best_z = wz_base;
                        best_normal = (0.0, 1.0, 0.0);
                        hit = true;
                    }
                }
            }

            if hit {
                 if depth.test_and_set(px as u32, py as u32, best_z) {
                    img.put_pixel(px as u32, py as u32, shade_color(best_normal, mat, best_z));
                }
            }
        }
    }
}

/// Draw a 3D torus (donut shape)
pub fn draw_torus_3d(
    img: &mut RgbaImage,
    depth: &mut DepthBuffer,
    cx: f32,
    cy: f32,
    cz: f32,
    major_radius: f32,
    minor_radius: f32,
    mat: &Material,
) {
    // Torus intersection is quartic (4th degree), which is hard to solve analytically fast.
    // For now, we unfortunately stick to a ray-marching or dense point cloud approach for torus?
    // Or we revert to the previous "splatting" approach BUT with TILT projection applied.
    // This is a reasonable compromise for complex shapes.

    let size = img.width() as i32;
    let _total_r = (major_radius + minor_radius).ceil() as i32;

    // We iterate PARAMETICALLY over the surface and project.
    // This is essentially what the old code did, but we need to respect the projection now.
    // To avoid gaps, we need valid step size.
    let _step = 0.5; // Half pixel steps
    
    let circum = 2.0 * std::f32::consts::PI * major_radius;
    let tube_circum = 2.0 * std::f32::consts::PI * minor_radius;
    
    let u_steps = (circum * 2.0) as i32; // Over-sample
    let v_steps = (tube_circum * 2.0) as i32;

    for i in 0..u_steps {
        let u = (i as f32 / u_steps as f32) * 2.0 * std::f32::consts::PI;
        let cos_u = u.cos();
        let sin_u = u.sin();

        for j in 0..v_steps {
            let v = (j as f32 / v_steps as f32) * 2.0 * std::f32::consts::PI;
            let cos_v = v.cos();
            let sin_v = v.sin();

            // Torus surface point
            // lying flat on XY plane? Code implies "distance from center in xy plane", so vertical torus?
            // "Z offset on the tube surface" -> implies Torus is in XY plane, tube thickens in Z.
            // (major + minor*cos(v)) * cos(u)
            // (major + minor*cos(v)) * sin(u)
            // minor * sin(v)
            
            let wx = cx + (major_radius + minor_radius * cos_v) * cos_u;
            let wy = cy + (major_radius + minor_radius * cos_v) * sin_u;
            let wz = cz + minor_radius * sin_v;

            // Project
            let px = wx.round() as i32;
            let py = (wy - wz * TILT).round() as i32;

             if px >= 0 && py >= 0 && px < size && py < size {
                 let normal_x = cos_v * cos_u;
                 let normal_y = cos_v * sin_u;
                 let normal_z = sin_v;
                 
                 if depth.test_and_set(px as u32, py as u32, wz) {
                     img.put_pixel(px as u32, py as u32, shade_color((normal_x, normal_y, normal_z), mat, wz));
                 }
             }
        }
    }
}

/// Draw a 3D box/cuboid
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
    // 6 Faces.
    // We can define the 6 planes and assume valid range.
    // Or just "paint" the 3 visible faces based on camera.
    // Camera is roughly (0, -inf, +inf) looking down-forward.
    // So we see Front (Z+), Top (Y-), and maybe Left/Right or Bottom depending on angle.
    // With TILT=0.5, we look from "below" in screen Y terms? 
    // Screen Y = World Y - Z * TILT. Low Wy -> Low Sy. High Z -> Low Sy.
    // We look from Y- (Top) and Z+ (Front).

    let x_min = cx - half_width;
    let x_max = cx + half_width;
    let y_min = cy - half_height;
    let y_max = cy + half_height;
    let _z_min = cz; // Box starts at cz? Original used cz+half_depth. 
    // Original draw_box_3d used cz as center? No, it used cz as back?
    // "front_z = cz + half_depth"
    // "pz = cz + half_depth - dz"
    // Let's assume (cx, cy, cz) is center.
    let z_front = cz + half_depth;
    let z_back = cz - half_depth;

    let _faces = [
        // Normal, Constant (N*P = k), u_range, v_range, u_index, v_index...
        // Simplified: Draw 3 visible faces.
        // Front: Z = z_front. Visible? Yes.
        // Top: Y = y_min. Visible? Yes.
        // Right: X = x_max. Visible? Maybe.
        // Left: X = x_min. Visible? Maybe.
        // Bottom: Y = y_max. Obscured.
        // Back: Z = z_back. Obscured.
        
        // Face 1: Front (Z+)
        (0.0f32, 0.0f32, 1.0f32, z_front, 0, 1), // N, z_val, axes
        // Face 2: Top (Y-)
        (0.0, -1.0, 0.0, y_min, 0, 2), 
        // Face 3: X+
        (1.0, 0.0, 0.0, x_max, 1, 2),
        // Face 4: X-
        (-1.0, 0.0, 0.0, x_min, 1, 2),
    ];

    let size_w = img.width() as i32;
    let size_h = img.height() as i32;

    // Bounding box on screen is union of all corners projected.
    // Corners:
    let mut min_px = 10000;
    let mut max_px = -10000;
    let mut min_py = 10000;
    let mut max_py = -10000;

    for x in [x_min, x_max] {
        for y in [y_min, y_max] {
            for z in [z_back, z_front] {
                let px = x.round() as i32;
                let py = (y - z * TILT).round() as i32;
                min_px = min_px.min(px);
                max_px = max_px.max(px);
                min_py = min_py.min(py);
                max_py = max_py.max(py);
            }
        }
    }
    
    // Expand a bit
    min_px -= 2; max_px += 2; min_py -= 2; max_py += 2;

    for py in min_py..=max_py {
        for px in min_px..=max_px {
             if px < 0 || px >= size_w || py < 0 || py >= size_h { continue; }
             
             let mut best_z = f32::NEG_INFINITY;
             let mut best_n = (0.0, 0.0, 0.0);
             let mut hit = false;

             // Check Front Face (Z = z_front)
             // wy = py + z_front * TILT
             // Check bounds x, y
             let wy_front = py as f32 + z_front * TILT;
             if px as f32 >= x_min && px as f32 <= x_max && wy_front >= y_min && wy_front <= y_max {
                 if z_front > best_z {
                     best_z = z_front;
                     best_n = (0.0, 0.0, 1.0);
                     hit = true;
                 }
             }

            // Check Top Face (Y = y_min)
            // Ray: wy = py + wz * TILT = y_min -> wz = (y_min - py) / TILT
            let wz_top = (y_min - py as f32) / TILT;
            if TILT.abs() > 0.001 {
                if px as f32 >= x_min && px as f32 <= x_max && wz_top >= z_back && wz_top <= z_front {
                    if wz_top > best_z {
                         best_z = wz_top;
                         best_n = (0.0, -1.0, 0.0);
                         hit = true;
                    }
                }
            }

            // Check Side Faces...
             if hit {
                 if depth.test_and_set(px as u32, py as u32, best_z) {
                     img.put_pixel(px as u32, py as u32, shade_color(best_n, mat, best_z));
                 }
             }
        }
    }
}

/// Draw a 3D wedge/ramp
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
    // Ramp surface equation: z = depth_val * (1 - (y - y_top)/(y_bot - y_top))
    // Let h = y_bot - y_top
    // z = depth_val * (1 - (y-y_top)/h)
    // z = depth_val * (1 - y/h + y_top/h)
    // z + y * (depth_val/h) = depth_val * (1 + y_top/h)
    // Plane n = (0, depth_val, h) dot (x, y, z) = ...?
    // Gradient is (0, depth_val/h, 1) assuming z(y).
    // Let slope = depth_val/h.
    // z = depth_val - slope*(y - y_top)
    
    // Ray: y = py + wz * TILT
    // wz = depth_val - slope*(py + wz*TILT - y_top)
    // wz = depth_val - slope*py - slope*wz*TILT + slope*y_top
    // wz(1 + slope*TILT) = depth_val - slope*py + slope*y_top
    
    let h = y_bot - y_top;
    if h <= 0.0 { return; }
    let slope = depth_val / h;
    
    // Box bounds...
    // just splat for now to save complexity, or simple iteration
    let half_w = width / 2.0;
    
    let size_w = img.width() as i32;
    let size_h = img.height() as i32;

    let min_py = (y_top - (cz+depth_val)*TILT).round() as i32 - 5;
    let max_py = (y_bot - cz*TILT).round() as i32 + 5;
    let min_px = (cx - half_w).round() as i32;
    let max_px = (cx + half_w).round() as i32;

    for py in min_py..=max_py {
        for px in min_px..=max_px {
            if px < 0 || px >= size_w || py < 0 || py >= size_h { continue; }
            
            // Solve wz
            // wz(1 + slope*TILT) = depth_val - slope*(py - y_top)
            let denom = 1.0 + slope * TILT;
            if denom.abs() > 0.0001 {
                let wz = (depth_val - slope * (py as f32 - y_top)) / denom;
                
                // Check bounds
                // wx = px. Inside width?
                if (px as f32) >= cx - half_w && (px as f32) <= cx + half_w {
                     // Check y range
                     let wy = py as f32 + wz * TILT;
                     if wy >= y_top && wy <= y_bot {
                         // Normal
                         // z = C - s*y
                         // 0 = C - s*y - z
                         // N = (0, -s, -1) -> (0, s, 1) to point out
                         let ny = slope;
                         let nz = 1.0;
                         let len = (ny*ny + nz*nz).sqrt();
                         
                         if depth.test_and_set(px as u32, py as u32, wz + cz) {
                             img.put_pixel(px as u32, py as u32, shade_color((0.0, ny/len, nz/len), mat, wz + cz));
                         }
                     }
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
