//! Material system and shading (Blinn-Phong lighting model)

use image::Rgba;

// The renderer's axes: +x is screen-right, +y is screen-down *and* into the
// scene's near ground, +z is altitude (a point at height z is drawn at
// `screen_y = y - z * TILT`). So an upward-facing surface has `nz ~= 1`.
//
// `LIGHT_*` is the unit vector pointing *from a surface toward the light* — a
// key light above, well to the left, and slightly in front of the subject.
//
// The angle away from the view axis is what produces form. The camera looks
// along `(0, TILT, 1)`, so a light near that axis lights every visible normal
// almost equally and bodies render flat; this one sits ~38 degrees off it, far
// enough that the right flank falls to ambient and a terminator crosses the
// subject.
const LIGHT_X: f32 = -0.55;
const LIGHT_Y: f32 = 0.10;
const LIGHT_Z: f32 = 0.83;

// The camera looks down the tilted ray `wy = py + wz * TILT`, so the direction
// from a surface toward the viewer is `(0, TILT, 1)` normalized. Blinn-Phong's
// half-vector is `normalize(L + V)`; both are constant here, so it is folded to
// a constant rather than recomputed per pixel.
const HALF_X: f32 = -0.2909;
const HALF_Y: f32 = 0.2894;
const HALF_Z: f32 = 0.9119;

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

    // Diffuse lighting (Lambert). `LIGHT_*` points toward the light, so a
    // surface facing it has a positive dot product — no negation.
    let dot = nx * LIGHT_X + ny * LIGHT_Y + nz * LIGHT_Z;
    let diffuse = dot.max(0.0) * mat.diffuse;

    // Specular (Blinn-Phong)
    let spec_dot = (nx * HALF_X + ny * HALF_Y + nz * HALF_Z).max(0.0);
    let specular = spec_dot.powf(mat.shininess) * mat.specular;

    let intensity = (mat.ambient + diffuse + specular).min(1.5);

    let r = ((mat.base_color[0] as f32 * intensity).min(255.0)) as u8;
    let g = ((mat.base_color[1] as f32 * intensity).min(255.0)) as u8;
    let b = ((mat.base_color[2] as f32 * intensity).min(255.0)) as u8;

    Rgba([r, g, b, 255])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn luma(px: Rgba<u8>) -> f32 {
        0.299 * px[0] as f32 + 0.587 * px[1] as f32 + 0.114 * px[2] as f32
    }

    /// The light vector must be normalized, or every diffuse term is silently
    /// scaled and `ambient + diffuse` no longer means what the materials say.
    #[test]
    fn light_and_half_vectors_are_unit_length() {
        for (name, v) in [
            ("light", (LIGHT_X, LIGHT_Y, LIGHT_Z)),
            ("half", (HALF_X, HALF_Y, HALF_Z)),
        ] {
            let len = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt();
            assert!(
                (len - 1.0).abs() < 0.01,
                "{name} vector length {len} is not ~1.0"
            );
        }
    }

    /// `HALF_*` is `normalize(L + V)` precomputed. It is therefore a second
    /// copy of the light direction, and the copy nothing recomputes is the one
    /// that goes stale — move the key light and the specular highlight silently
    /// keeps pointing at the old one. Derive it here and compare.
    #[test]
    fn half_vector_matches_the_light_it_was_derived_from() {
        // View direction: the camera's ray is `wy = py + wz * TILT`, so moving
        // toward the viewer is `(0, TILT, 1)`.
        let tilt = crate::graphics_gen::core::TILT;
        let v_len = (tilt * tilt + 1.0).sqrt();
        let view = (0.0, tilt / v_len, 1.0 / v_len);

        let sum = (LIGHT_X + view.0, LIGHT_Y + view.1, LIGHT_Z + view.2);
        let len = (sum.0 * sum.0 + sum.1 * sum.1 + sum.2 * sum.2).sqrt();
        let expected = (sum.0 / len, sum.1 / len, sum.2 / len);

        for (axis, got, want) in [
            ("x", HALF_X, expected.0),
            ("y", HALF_Y, expected.1),
            ("z", HALF_Z, expected.2),
        ] {
            assert!(
                (got - want).abs() < 0.005,
                "HALF_{} is {got}, but normalize(LIGHT + VIEW) gives {want} — \
                 recompute the half-vector after moving the light",
                axis.to_uppercase()
            );
        }
    }

    /// The bug this test exists for: the diffuse dot product was negated, so
    /// `nz = 1` (an upward-facing surface, which is most of a large rounded
    /// body in this projection) received *zero* diffuse light. Big creatures
    /// rendered as near-black silhouettes while small spheres — mostly rim —
    /// looked fine.
    #[test]
    fn upward_facing_surfaces_are_lit_not_black() {
        let mat = Material::flesh(120, 140, 110);
        let top = shade_color((0.0, 0.0, 1.0), &mat, 0.0);
        let ambient_only = mat.base_color[1] as f32 * mat.ambient;

        assert!(
            luma(top) > ambient_only * 1.5,
            "top surface {top:?} is at ambient level — diffuse is not reaching it"
        );
    }

    /// Shading has to have a direction: the side facing the key light is
    /// brighter than the side facing away, and the underside is darkest.
    #[test]
    fn shading_falls_off_away_from_the_key_light() {
        let mat = Material::matte(180, 180, 180);

        let toward = luma(shade_color((LIGHT_X, LIGHT_Y, LIGHT_Z), &mat, 0.0));
        let top = luma(shade_color((0.0, 0.0, 1.0), &mat, 0.0));
        let away = luma(shade_color((-LIGHT_X, -LIGHT_Y, -LIGHT_Z), &mat, 0.0));
        let under = luma(shade_color((0.0, 0.0, -1.0), &mat, 0.0));

        assert!(toward > top, "surface facing the light should be brightest");
        assert!(top > away, "top should out-light the shadow side");
        assert!(
            (away - under).abs() < 1.0,
            "both fully-shadowed normals should sit at ambient"
        );
        assert!(under > 0.0, "ambient should keep shadows off pure black");
    }
}
