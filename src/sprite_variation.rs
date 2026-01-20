//! Runtime sprite variation system
//!
//! Generates unique sprite variations for entities at spawn time,
//! allowing visual diversity without pre-generating many image files.

use macroquad::prelude::*;
use std::collections::HashMap;

/// Seed for generating deterministic variations
pub type VariationSeed = u64;

/// Color variation that can be applied to specific sprite regions
#[derive(Debug, Clone, Copy)]
pub struct ColorVariation {
    /// Hue shift in degrees (-180 to 180)
    pub hue_shift: f32,
    /// Saturation multiplier (0.0 to 2.0, 1.0 = no change)
    pub saturation: f32,
    /// Brightness multiplier (0.0 to 2.0, 1.0 = no change)
    pub brightness: f32,
}

impl Default for ColorVariation {
    fn default() -> Self {
        Self {
            hue_shift: 0.0,
            saturation: 1.0,
            brightness: 1.0,
        }
    }
}

impl ColorVariation {
    pub fn new(hue_shift: f32, saturation: f32, brightness: f32) -> Self {
        Self {
            hue_shift,
            saturation,
            brightness,
        }
    }

    /// Create a random variation from a seed
    pub fn from_seed(seed: u64, variation_strength: f32) -> Self {
        let mut rng = SimpleRng::new(seed);
        Self {
            hue_shift: rng.next_f32_range(-30.0, 30.0) * variation_strength,
            saturation: 1.0 + rng.next_f32_range(-0.2, 0.2) * variation_strength,
            brightness: 1.0 + rng.next_f32_range(-0.15, 0.15) * variation_strength,
        }
    }
}

/// Defines which parts of a sprite can be varied
#[derive(Debug, Clone)]
pub struct SpriteVariationConfig {
    /// Name of this config (e.g., "wizard", "knight")
    pub name: String,
    /// Color regions that can be varied, with their target hue ranges
    pub color_regions: Vec<ColorRegion>,
    /// Overall variation strength (0.0 to 1.0)
    pub variation_strength: f32,
}

/// A region of color that can be varied
#[derive(Debug, Clone)]
pub struct ColorRegion {
    /// Name of this region (e.g., "robe", "staff", "skin")
    pub name: String,
    /// Target hue range to affect (in degrees, 0-360)
    pub hue_min: f32,
    pub hue_max: f32,
    /// Target saturation range (0.0 to 1.0)
    pub sat_min: f32,
    pub sat_max: f32,
    /// How much this region can vary
    pub variation_scale: f32,
}

impl ColorRegion {
    pub fn new(name: &str, hue_min: f32, hue_max: f32, sat_min: f32, sat_max: f32, variation_scale: f32) -> Self {
        Self {
            name: name.to_string(),
            hue_min,
            hue_max,
            sat_min,
            sat_max,
            variation_scale,
        }
    }

    /// Check if a color falls within this region
    pub fn matches(&self, hue: f32, saturation: f32) -> bool {
        let hue_match = if self.hue_min <= self.hue_max {
            hue >= self.hue_min && hue <= self.hue_max
        } else {
            // Wrap around (e.g., red: 350-10)
            hue >= self.hue_min || hue <= self.hue_max
        };
        let sat_match = saturation >= self.sat_min && saturation <= self.sat_max;
        hue_match && sat_match
    }
}

/// Visual variations for an individual entity
#[derive(Debug, Clone, Default)]
pub struct EntityVisualVariation {
    /// Random seed for this entity's variations
    pub seed: VariationSeed,
    /// Pre-computed color variations per region
    pub region_variations: HashMap<String, ColorVariation>,
}

impl EntityVisualVariation {
    /// Generate variations from a seed
    pub fn from_seed(seed: VariationSeed, config: &SpriteVariationConfig) -> Self {
        let mut region_variations = HashMap::new();
        let mut region_seed = seed;

        for region in &config.color_regions {
            let variation = ColorVariation::from_seed(
                region_seed,
                config.variation_strength * region.variation_scale,
            );
            region_variations.insert(region.name.clone(), variation);
            region_seed = region_seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        }

        Self {
            seed,
            region_variations,
        }
    }

    /// Get variation for a specific region
    pub fn get_region_variation(&self, region_name: &str) -> ColorVariation {
        self.region_variations
            .get(region_name)
            .copied()
            .unwrap_or_default()
    }
}

/// Cache for generated sprite variations
pub struct SpriteVariationCache {
    /// Cached textures by (base_sprite_id, variation_seed)
    cache: HashMap<(String, VariationSeed), Texture2D>,
    /// Variation configs per sprite type
    configs: HashMap<String, SpriteVariationConfig>,
}

impl SpriteVariationCache {
    pub fn new() -> Self {
        let mut cache = Self {
            cache: HashMap::new(),
            configs: HashMap::new(),
        };
        cache.register_default_configs();
        cache
    }

    /// Register default variation configs for all unit types
    fn register_default_configs(&mut self) {
        // Wizard/Warlock - robe and staff colors can vary
        self.configs.insert(
            "wizard".to_string(),
            SpriteVariationConfig {
                name: "wizard".to_string(),
                color_regions: vec![
                    ColorRegion::new("robe", 220.0, 280.0, 0.3, 1.0, 1.5),      // Blue/purple robes
                    ColorRegion::new("staff_gem", 0.0, 360.0, 0.5, 1.0, 2.0),   // Staff gem - high variation
                    ColorRegion::new("trim", 30.0, 60.0, 0.3, 1.0, 1.0),        // Gold trim
                ],
                variation_strength: 0.8,
            },
        );

        self.configs.insert(
            "warlock".to_string(),
            SpriteVariationConfig {
                name: "warlock".to_string(),
                color_regions: vec![
                    ColorRegion::new("robe", 260.0, 320.0, 0.2, 1.0, 1.2),      // Purple/dark robes
                    ColorRegion::new("magic_glow", 0.0, 360.0, 0.4, 1.0, 2.0),  // Magic effects
                    ColorRegion::new("skin", 20.0, 50.0, 0.2, 0.6, 0.5),        // Skin tone
                ],
                variation_strength: 0.7,
            },
        );

        // Knight - armor tint and plume color
        self.configs.insert(
            "knight".to_string(),
            SpriteVariationConfig {
                name: "knight".to_string(),
                color_regions: vec![
                    ColorRegion::new("armor", 0.0, 360.0, 0.0, 0.3, 0.3),       // Metal armor (low sat)
                    ColorRegion::new("plume", 0.0, 360.0, 0.5, 1.0, 2.0),       // Helmet plume
                    ColorRegion::new("cloth", 0.0, 60.0, 0.4, 1.0, 1.0),        // Cloth/cape
                ],
                variation_strength: 0.6,
            },
        );

        // Goblin - skin tone and equipment
        self.configs.insert(
            "goblin".to_string(),
            SpriteVariationConfig {
                name: "goblin".to_string(),
                color_regions: vec![
                    ColorRegion::new("skin", 60.0, 150.0, 0.3, 0.8, 0.8),       // Green skin
                    ColorRegion::new("cloth", 0.0, 360.0, 0.3, 1.0, 1.5),       // Clothing
                    ColorRegion::new("eyes", 0.0, 60.0, 0.5, 1.0, 1.0),         // Eye color
                ],
                variation_strength: 0.7,
            },
        );

        // Orc - skin and war paint
        self.configs.insert(
            "orc".to_string(),
            SpriteVariationConfig {
                name: "orc".to_string(),
                color_regions: vec![
                    ColorRegion::new("skin", 60.0, 140.0, 0.3, 0.7, 0.6),       // Green/brown skin
                    ColorRegion::new("armor", 0.0, 60.0, 0.2, 0.8, 0.8),        // Leather/metal
                    ColorRegion::new("warpaint", 0.0, 360.0, 0.5, 1.0, 1.5),    // War paint
                ],
                variation_strength: 0.6,
            },
        );

        // Skeleton - bone color and equipment
        self.configs.insert(
            "skeleton".to_string(),
            SpriteVariationConfig {
                name: "skeleton".to_string(),
                color_regions: vec![
                    ColorRegion::new("bone", 30.0, 60.0, 0.1, 0.4, 0.4),        // Bone color
                    ColorRegion::new("glow", 180.0, 240.0, 0.3, 1.0, 1.2),      // Soul glow
                ],
                variation_strength: 0.5,
            },
        );

        // Imp - skin and glow (dark maroon sprite with low saturation)
        self.configs.insert(
            "imp".to_string(),
            SpriteVariationConfig {
                name: "imp".to_string(),
                color_regions: vec![
                    ColorRegion::new("skin", 340.0, 40.0, 0.15, 1.0, 1.2),      // Dark red/maroon skin (wraps around)
                    ColorRegion::new("wings", 340.0, 60.0, 0.1, 0.8, 0.8),      // Wing membrane
                    ColorRegion::new("horns", 0.0, 60.0, 0.05, 0.4, 0.6),       // Dark horns/features
                ],
                variation_strength: 0.8,
            },
        );

        // Troll - skin and moss
        self.configs.insert(
            "troll".to_string(),
            SpriteVariationConfig {
                name: "troll".to_string(),
                color_regions: vec![
                    ColorRegion::new("skin", 80.0, 160.0, 0.2, 0.6, 0.7),       // Gray-green skin
                    ColorRegion::new("moss", 80.0, 140.0, 0.4, 0.9, 0.8),       // Moss/growth
                ],
                variation_strength: 0.5,
            },
        );

        // Archer
        self.configs.insert(
            "archer".to_string(),
            SpriteVariationConfig {
                name: "archer".to_string(),
                color_regions: vec![
                    ColorRegion::new("cloth", 60.0, 150.0, 0.3, 0.8, 1.0),      // Green clothing
                    ColorRegion::new("leather", 20.0, 50.0, 0.3, 0.7, 0.5),     // Brown leather
                    ColorRegion::new("feather", 0.0, 360.0, 0.4, 1.0, 1.5),     // Arrow fletching
                ],
                variation_strength: 0.6,
            },
        );

        // Vampire
        self.configs.insert(
            "vampire".to_string(),
            SpriteVariationConfig {
                name: "vampire".to_string(),
                color_regions: vec![
                    ColorRegion::new("cape", 0.0, 360.0, 0.3, 1.0, 1.2),        // Cape color
                    ColorRegion::new("skin", 0.0, 40.0, 0.0, 0.3, 0.3),         // Pale skin
                    ColorRegion::new("eyes", 0.0, 60.0, 0.6, 1.0, 1.5),         // Eye glow
                ],
                variation_strength: 0.7,
            },
        );

        // Spider
        self.configs.insert(
            "spider".to_string(),
            SpriteVariationConfig {
                name: "spider".to_string(),
                color_regions: vec![
                    ColorRegion::new("body", 0.0, 360.0, 0.1, 0.5, 0.6),        // Body color
                    ColorRegion::new("markings", 0.0, 60.0, 0.5, 1.0, 1.2),     // Pattern markings
                ],
                variation_strength: 0.5,
            },
        );

        // Default config for unspecified types
        self.configs.insert(
            "default".to_string(),
            SpriteVariationConfig {
                name: "default".to_string(),
                color_regions: vec![
                    ColorRegion::new("primary", 0.0, 360.0, 0.2, 1.0, 1.0),
                    ColorRegion::new("secondary", 0.0, 360.0, 0.2, 1.0, 0.8),
                ],
                variation_strength: 0.5,
            },
        );
    }

    /// Get or create a varied texture
    pub fn get_or_create(
        &mut self,
        base_sprite_id: &str,
        seed: VariationSeed,
        base_texture: &Texture2D,
    ) -> Texture2D {
        let key = (base_sprite_id.to_string(), seed);

        // Return cached if exists
        if let Some(texture) = self.cache.get(&key) {
            return texture.clone();
        }

        // Generate new variation
        let config = self.configs
            .get(base_sprite_id)
            .or_else(|| self.configs.get("default"))
            .cloned()
            .unwrap();

        let variation = EntityVisualVariation::from_seed(seed, &config);
        let new_texture = apply_variation(base_texture, &config, &variation);

        self.cache.insert(key, new_texture.clone());
        new_texture
    }

    /// Get the variation config for a sprite type
    pub fn get_config(&self, sprite_id: &str) -> Option<&SpriteVariationConfig> {
        self.configs.get(sprite_id).or_else(|| self.configs.get("default"))
    }

    /// Clear the cache (useful when base textures change)
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache size for debugging
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

impl Default for SpriteVariationCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Apply variations to a base texture and create a new texture
fn apply_variation(
    base_texture: &Texture2D,
    config: &SpriteVariationConfig,
    variation: &EntityVisualVariation,
) -> Texture2D {
    let image = base_texture.get_texture_data();
    let width = image.width();
    let height = image.height();
    let mut new_image = Image::gen_image_color(width as u16, height as u16, Color::new(0.0, 0.0, 0.0, 0.0));

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x as u32, y as u32);

            // Skip fully transparent pixels
            if pixel.a < 0.01 {
                new_image.set_pixel(x as u32, y as u32, pixel);
                continue;
            }

            // Convert to HSV
            let (h, s, v) = rgb_to_hsv(pixel.r, pixel.g, pixel.b);

            // Find which region this pixel belongs to and apply variation
            let mut best_match: Option<(&ColorRegion, &ColorVariation)> = None;

            for region in &config.color_regions {
                if region.matches(h, s) {
                    if let Some(var) = variation.region_variations.get(&region.name) {
                        best_match = Some((region, var));
                        break;
                    }
                }
            }

            let (new_r, new_g, new_b) = if let Some((_, var)) = best_match {
                // Apply the variation
                let new_h = (h + var.hue_shift).rem_euclid(360.0);
                let new_s = (s * var.saturation).clamp(0.0, 1.0);
                let new_v = (v * var.brightness).clamp(0.0, 1.0);
                hsv_to_rgb(new_h, new_s, new_v)
            } else {
                (pixel.r, pixel.g, pixel.b)
            };

            new_image.set_pixel(
                x as u32,
                y as u32,
                Color::new(new_r, new_g, new_b, pixel.a),
            );
        }
    }

    Texture2D::from_image(&new_image)
}

/// Convert RGB to HSV
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    (h, s, v)
}

/// Convert HSV to RGB
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (r + m, g + m, b + m)
}

/// Simple deterministic RNG for variation generation
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed.wrapping_add(1) }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
        min + self.next_f32() * (max - min)
    }
}

/// Generate a variation seed from entity ID and spawn time
pub fn generate_variation_seed(entity_id: usize, spawn_frame: u64) -> VariationSeed {
    let mut seed = entity_id as u64;
    seed = seed.wrapping_mul(0x517cc1b727220a95);
    seed ^= spawn_frame;
    seed = seed.wrapping_mul(0x2545F4914F6CDD1D);
    seed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_variation_from_seed() {
        let v1 = ColorVariation::from_seed(12345, 1.0);
        let v2 = ColorVariation::from_seed(12345, 1.0);
        let v3 = ColorVariation::from_seed(54321, 1.0);

        // Same seed should give same result
        assert_eq!(v1.hue_shift, v2.hue_shift);
        assert_eq!(v1.saturation, v2.saturation);

        // Different seed should give different result
        assert_ne!(v1.hue_shift, v3.hue_shift);
    }

    #[test]
    fn test_rgb_hsv_roundtrip() {
        let test_colors = [
            (1.0, 0.0, 0.0), // Red
            (0.0, 1.0, 0.0), // Green
            (0.0, 0.0, 1.0), // Blue
            (0.5, 0.5, 0.5), // Gray
            (1.0, 0.5, 0.25), // Orange-ish
        ];

        for (r, g, b) in test_colors {
            let (h, s, v) = rgb_to_hsv(r, g, b);
            let (r2, g2, b2) = hsv_to_rgb(h, s, v);
            assert!((r - r2).abs() < 0.01, "Red mismatch: {} vs {}", r, r2);
            assert!((g - g2).abs() < 0.01, "Green mismatch: {} vs {}", g, g2);
            assert!((b - b2).abs() < 0.01, "Blue mismatch: {} vs {}", b, b2);
        }
    }

    #[test]
    fn test_color_region_matching() {
        let region = ColorRegion::new("test", 350.0, 10.0, 0.5, 1.0, 1.0); // Red, wraps around

        assert!(region.matches(0.0, 0.7));   // Red at hue 0
        assert!(region.matches(355.0, 0.7)); // Red at hue 355
        assert!(!region.matches(180.0, 0.7)); // Blue, shouldn't match
        assert!(!region.matches(0.0, 0.3));  // Red but low saturation
    }
}
