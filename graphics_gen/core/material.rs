//! Material system and shading (Blinn-Phong lighting model)

use image::Rgba;

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
    pub fn new(
        r: u8,
        g: u8,
        b: u8,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Self {
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
