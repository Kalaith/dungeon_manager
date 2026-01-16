//! Procedural graphics generator for Deep Dominion
//! Generates consistent isometric tiles, rooms, and sprites
// TODO: Remove this global warning suppression after cleaning up unused imports,
// dead code, and other warnings identified in the codebase review
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use image::{ImageBuffer, Rgba, RgbaImage};
use std::path::Path;

const TILE_SIZE: u32 = 64;
const TILE_WIDTH: u32 = 64;
const TILE_HEIGHT: u32 = 64;
const SPRITE_SIZE: u32 = 64;

fn main() {
    println!("Generating game graphics...");

    // Create assets directory
    std::fs::create_dir_all("assets/tiles").unwrap();
    std::fs::create_dir_all("assets/sprites/monsters").unwrap();
    std::fs::create_dir_all("assets/sprites/heroes").unwrap();
    std::fs::create_dir_all("assets/tiles/hero_buildings").unwrap();

    // Generate all tiles
    generate_tiles();

    // Generate all sprites
    generate_monster_sprites();
    generate_hero_sprites();

    save_hero_building("town_hall", create_town_hall());
    save_hero_building("barracks", create_barracks());
    save_hero_building("archery_range", create_archery_range());
    save_hero_building("church", create_church());
    save_hero_building("mage_tower", create_mage_tower());
    save_hero_building("stable", create_stable());
    save_hero_building("armory", create_armory());
    save_hero_building("hero_wall", create_hero_wall());
    save_hero_building("hero_gate", create_hero_gate());
    
    println!("Graphics generation complete!");
}

fn generate_tiles() {
    // Tiles
    save_tile("solid_rock", create_solid_rock());
    save_tile("earth", create_earth());
    save_tile("claimed_floor", create_claimed_floor());
    save_tile("reinforced_wall", create_reinforced_wall());
    save_tile("gold_vein", create_gold_vein());
    save_tile("gem_seam", create_gem_seam());
    save_tile("lava", create_lava());
    save_tile("water", create_water());
    save_tile("bridge", create_bridge());
    save_tile("corrupted_floor", create_corrupted_floor());
    save_tile("corrupted_floor", create_corrupted_floor());
    save_tile("ancient_rune_floor", create_ancient_rune_floor());
    save_tile("mana_crystal", create_mana_crystal());

    // Rooms
    save_tile("dungeon_heart", create_dungeon_heart());
    save_tile("lair", create_lair());
    save_tile("hatchery", create_hatchery());
    save_tile("treasury", create_treasury());
    save_tile("workshop", create_workshop());
    save_tile("training_room", create_training_room());
    save_tile("library", create_library());
    save_tile("prison", create_prison());
    save_tile("guard_post", create_guard_post());
    save_tile("ritual_circle", create_ritual_circle());
    save_tile("monster_spawner", create_monster_spawner());
    
    // Traps
    save_tile("door", create_door());
    save_tile("spike_trap", create_spike_trap());
    save_tile("boulder_trap", create_boulder_trap());
    save_tile("alarm_trap", create_alarm_trap());
}

fn save_tile(name: &str, img: RgbaImage) {
    let path = format!("assets/tiles/{}.png", name);
    img.save(&path).unwrap();
    println!("Generated: {}", path);
}

fn save_hero_building(name: &str, img: RgbaImage) {
    let path = format!("assets/tiles/hero_buildings/{}.png", name);
    let flipped = image::imageops::flip_vertical(&img);
    flipped.save(&path).unwrap();
    println!("Generated: {}", path);
}

fn save_sprite(category: &str, name: &str, img: RgbaImage) {
    let path = format!("assets/sprites/{}/{}.png", category, name);
    img.save(&path).unwrap();
    println!("Generated: {}", path);
}

// Create square base for 3D tiles
fn create_tile_base(color: Rgba<u8>) -> RgbaImage {
    let mut img = RgbaImage::new(TILE_WIDTH, TILE_HEIGHT);

    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            img.put_pixel(x, y, color);
        }
    }

    img
}

fn add_noise(img: &mut RgbaImage, amount: i32) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for p in img.pixels_mut() {
        if p[3] > 0 {
            let noise = rng.gen_range(-amount..=amount);
            for i in 0..3 {
                 let val = p[i] as i32 + noise;
                 p[i] = val.clamp(0, 255) as u8;
            }
        }
    }
}

fn add_outline(img: &mut RgbaImage, outline_color: Rgba<u8>) {
    let width = img.width();
    let height = img.height();
    let mut outline_pixels = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            if pixel[3] > 0 {
                // Check if any neighbor is transparent (edge)
                let mut is_edge = false;
                for dy in -1..=1i32 {
                    for dx in -1..=1i32 {
                        if dx == 0 && dy == 0 { continue; }
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            if img.get_pixel(nx as u32, ny as u32)[3] == 0 {
                                is_edge = true;
                                break;
                            }
                        }
                    }
                    if is_edge { break; }
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

// Tile generators
fn create_solid_rock() -> RgbaImage {
    let mut img = create_tile_base(Rgba([60, 60, 65, 255]));
    // Add some texture
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if (x + y) % 7 == 0 {
                img.put_pixel(x, y, Rgba([50, 50, 55, 255]));
            }
        }
    }
    add_noise(&mut img, 10);
    img
}

fn create_earth() -> RgbaImage {
    let mut img = create_tile_base(Rgba([101, 67, 33, 255])); // Brown
    // Add dirt texture
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if (x * 3 + y * 5) % 11 < 4 {
                img.put_pixel(x, y, Rgba([91, 60, 28, 255]));
            }
        }
    }
    add_noise(&mut img, 15);
    img
}

fn create_claimed_floor() -> RgbaImage {
    let mut img = create_tile_base(Rgba([45, 45, 50, 255])); // Dark gray
    // Add grid pattern
    for y in (0..TILE_HEIGHT).step_by(16) {
        for x in 0..TILE_WIDTH {
            img.put_pixel(x, y, Rgba([35, 35, 40, 255]));
        }
    }
    for x in (0..TILE_WIDTH).step_by(16) {
        for y in 0..TILE_HEIGHT {
            img.put_pixel(x, y, Rgba([35, 35, 40, 255]));
        }
    }
    img
}

fn create_reinforced_wall() -> RgbaImage {
    let mut img = create_tile_base(Rgba([70, 70, 75, 255]));
    // Add metal bands
    for y in (4..TILE_HEIGHT).step_by(10) {
        for x in 0..TILE_WIDTH {
             img.put_pixel(x, y, Rgba([120, 120, 130, 255]));
        }
    }
    // Rivets
    for y in (4..TILE_HEIGHT).step_by(10) {
        for x in (4..TILE_WIDTH).step_by(16) {
             img.put_pixel(x, y, Rgba([150, 150, 160, 255]));
        }
    }
    img
}

fn create_gold_vein() -> RgbaImage {
    let mut img = create_tile_base(Rgba([80, 70, 50, 255])); // Dark rock
    // Add gold streaks
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
             if (x + y * 2) % 13 < 3 {
                 img.put_pixel(x, y, Rgba([255, 215, 0, 255])); // Gold
             }
        }
    }
    img
}

fn create_gem_seam() -> RgbaImage {
    let mut img = create_tile_base(Rgba([70, 70, 80, 255]));
    // Add colorful gems
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
             let val = (x * 7 + y * 11) % 19;
             if val < 2 {
                 img.put_pixel(x, y, Rgba([100, 100, 255, 255])); // Blue gem
             } else if val < 4 {
                 img.put_pixel(x, y, Rgba([200, 100, 200, 255])); // Purple gem
             }
        }
    }
    img
}

fn create_mana_crystal() -> RgbaImage {
    let mut img = create_tile_base(Rgba([40, 40, 60, 255])); // Dark blue-gray rock
    // Add glowing blue crystals
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;
    
    // Draw crystal cluster
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
             // Crystal formation logic
             let dx = (x as i32 - center_x as i32).abs();
             let dy = (y as i32 - center_y as i32) as f32;
             
             // Central large crystal
             if dx < 4 && dy < 0.0 && dy > -12.0 {
                  img.put_pixel(x, y, Rgba([100, 200, 255, 255])); // Cyan core
             }
             
             // Side crystals
             if (dx > 4 && dx < 8) && (dy > -5.0 && dy < 2.0) {
                 img.put_pixel(x, y, Rgba([50, 150, 255, 255])); // Blue side
             }
             
             // Glow effect
             if dx < 10 && dy.abs() < 8.0 && (x + y as u32) % 5 == 0 {
                 let pixel = img.get_pixel(x, y);
                 // Lighten existing color for glow
                 if pixel[0] < 100 { // If it's rock base
                      img.put_pixel(x, y, Rgba([60, 60, 90, 255]));
                 }
             }
        }
    }
    
    // Add highlights
    img.put_pixel(center_x, center_y - 8, Rgba([255, 255, 255, 255]));
    img.put_pixel(center_x - 5, center_y, Rgba([200, 200, 255, 255]));
    img.put_pixel(center_x + 5, center_y, Rgba([200, 200, 255, 255]));

    img
}

fn create_lava() -> RgbaImage {
    let mut img = create_tile_base(Rgba([255, 69, 0, 255])); // Orange-red
    // Add lava bubbles
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
             if (x * 5 + y * 7) % 17 < 5 {
                 img.put_pixel(x, y, Rgba([255, 140, 0, 255])); // Brighter orange
             }
        }
    }
    img
}

fn create_water() -> RgbaImage {
    let mut img = create_tile_base(Rgba([30, 60, 120, 255])); // Dark blue
    // Add water ripples
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
             if (x + y) % 9 < 2 {
                 img.put_pixel(x, y, Rgba([50, 80, 140, 255]));
             }
        }
    }
    img
}

fn create_bridge() -> RgbaImage {
    let mut img = create_tile_base(Rgba([100, 90, 80, 255])); // Stone
    // Add planks
    for x in (0..TILE_WIDTH).step_by(16) {
        for y in 0..TILE_HEIGHT {
             img.put_pixel(x, y, Rgba([80, 70, 60, 255]));
             if x > 0 { img.put_pixel(x-1, y, Rgba([80, 70, 60, 255])); }
        }
    }
    // Nails
    for x in (8..TILE_WIDTH).step_by(16) {
        img.put_pixel(x, 4, Rgba([50, 45, 40, 255]));
        img.put_pixel(x, TILE_HEIGHT-4, Rgba([50, 45, 40, 255]));
    }
    img
}

fn create_corrupted_floor() -> RgbaImage {
    let mut img = create_tile_base(Rgba([60, 30, 80, 255])); // Purple-dark
    // Add corruption effect
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
             if (x * 3 + y * 5) % 13 < 4 {
                 img.put_pixel(x, y, Rgba([80, 40, 100, 255]));
             }
        }
    }
    img
}

fn create_ancient_rune_floor() -> RgbaImage {
    let mut img = create_tile_base(Rgba([50, 50, 60, 255]));
    // Add glowing runes
    for y in (8..TILE_HEIGHT).step_by(16) {
        for x in (8..TILE_WIDTH).step_by(16) {
             // Draw small rune symbol
             draw_rect(&mut img, x, y, 6, 2, Rgba([100, 150, 255, 255]));
             draw_rect(&mut img, x+2, y-2, 2, 6, Rgba([100, 150, 255, 255]));
        }
    }
    img
}

// Room tiles
fn create_dungeon_heart() -> RgbaImage {
    let mut img = create_tile_base(Rgba([139, 0, 0, 255])); // Dark red
    // Add heart pattern
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
             let dx = (x as i32 - center_x as i32).abs();
             let dy = (y as i32 - center_y as i32).abs();
             if dx + dy < 16 {
                 img.put_pixel(x, y, Rgba([255, 0, 0, 255])); // Bright red center
             }
        }
    }
    img
}

fn create_lair() -> RgbaImage {
    let mut img = create_tile_base(Rgba([70, 50, 40, 255])); // Brown
    // Add bedding pattern
    // Concentric straw nests?
    for i in 0..3 {
        let cx = 16 + i * 20;
        let cy = 16 + (i%2) * 20;
        draw_circle(&mut img, cx, cy, 10, Rgba([90, 70, 50, 255]));
    }
    add_noise(&mut img, 10);
    img
}

fn create_hatchery() -> RgbaImage {
    let mut img = create_tile_base(Rgba([85, 70, 50, 255])); // Brown-tan
    // Add hay/straw texture
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
             if (x * 7 + y * 3) % 11 < 3 {
                 img.put_pixel(x, y, Rgba([200, 180, 80, 255])); // Yellow straw
             }
        }
    }
    img
}

fn create_treasury() -> RgbaImage {
    let mut img = create_tile_base(Rgba([218, 165, 32, 255])); // Goldenrod
    // Add coin pattern
    for y in (0..TILE_HEIGHT).step_by(8) {
        for x in (0..TILE_WIDTH).step_by(8) {
             img.put_pixel(x, y, Rgba([255, 215, 0, 255])); // Bright gold
             img.put_pixel(x+1, y, Rgba([255, 255, 100, 255])); // Highlight
        }
    }
    img
}

fn create_workshop() -> RgbaImage {
    let mut img = create_tile_base(Rgba([60, 60, 65, 255])); // Dark gray
    // Add forge glow
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
             if y > TILE_HEIGHT - 20 && x > TILE_WIDTH - 20 {
                 img.put_pixel(x, y, Rgba([255, 100, 0, 255])); // Orange forge glow
             }
        }
    }
    // Anvil
    draw_rect(&mut img, 16, 16, 20, 10, Rgba([30, 30, 30, 255]));
    img
}

fn create_training_room() -> RgbaImage {
    let mut img = create_tile_base(Rgba([80, 60, 50, 255])); // Brown-gray
    // Add weapon racks
    draw_rect(&mut img, 10, 10, 4, 40, Rgba([100, 80, 70, 255]));
    draw_rect(&mut img, 30, 10, 4, 40, Rgba([100, 80, 70, 255]));
    draw_rect(&mut img, 50, 10, 4, 40, Rgba([100, 80, 70, 255]));
    img
}

fn create_library() -> RgbaImage {
    let mut img = create_tile_base(Rgba([70, 50, 80, 255])); // Purple-brown
    // Add book shelf pattern
    for x in (0..TILE_WIDTH).step_by(16) {
        draw_rect(&mut img, x+4, 4, 8, 56, Rgba([139, 69, 19, 255])); // Bookshelf top down?
        // Actually top down view of a floor...
        // Let's make it floor boards
    }
    img
}

fn create_prison() -> RgbaImage {
    let mut img = create_tile_base(Rgba([50, 50, 55, 255])); // Dark gray
    // Add bars shadow?
    for x in (8..TILE_WIDTH).step_by(16) {
        for y in 0..TILE_HEIGHT {
             img.put_pixel(x, y, Rgba([30, 30, 35, 255])); // Bar shadow
        }
    }
    img
}

fn create_guard_post() -> RgbaImage {
    let mut img = create_tile_base(Rgba([80, 70, 60, 255])); // Stone
    // Add shield emblem on floor
    let cx = TILE_WIDTH / 2;
    let cy = TILE_HEIGHT / 2;
    draw_rect(&mut img, cx-10, cy-10, 20, 20, Rgba([100, 90, 80, 255]));
    draw_rect(&mut img, cx-5, cy-5, 10, 10, Rgba([150, 140, 130, 255]));
    img
}

fn create_ritual_circle() -> RgbaImage {
    let mut img = create_tile_base(Rgba([40, 20, 50, 255])); // Dark purple
    // Add pentagram/circle
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
             let dx = (x as i32 - center_x as i32).abs();
             let dy = (y as i32 - center_y as i32).abs();
             if dx*dx + dy*dy > 100 && dx*dx + dy*dy < 150 {
                 img.put_pixel(x, y, Rgba([200, 50, 50, 255])); // Red circle ring
             }
        }
    }
    img
}

fn create_monster_spawner() -> RgbaImage {
    let mut img = create_tile_base(Rgba([80, 20, 100, 255])); // Purple
    // Add portal swirl
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
             let dx = x as i32 - center_x as i32;
             let dy = y as i32 - center_y as i32;
             if (dx * dx + dy * dy) < 400 {
                 if (dx*dx+dy*dy) % 20 < 10 {
                     img.put_pixel(x, y, Rgba([150, 50, 200, 255])); // Bright purple swirl
                 }
             }
        }
    }
    img
}

// Trap tiles
fn create_door() -> RgbaImage {
    let mut img = create_tile_base(Rgba([100, 60, 30, 255])); // Wood
    // Planks
    for x in (0..TILE_WIDTH).step_by(8) {
        for y in 0..TILE_HEIGHT {
            img.put_pixel(x, y, Rgba([90, 50, 25, 255]));
        }
    }
    // Frame
    draw_rect(&mut img, 0, 0, TILE_WIDTH, 4, Rgba([60, 40, 20, 255]));
    draw_rect(&mut img, 0, TILE_HEIGHT-4, TILE_WIDTH, 4, Rgba([60, 40, 20, 255]));
    draw_rect(&mut img, 0, 0, 4, TILE_HEIGHT, Rgba([60, 40, 20, 255]));
    draw_rect(&mut img, TILE_WIDTH-4, 0, 4, TILE_HEIGHT, Rgba([60, 40, 20, 255]));
    
    // Handle
    draw_circle(&mut img, TILE_WIDTH-10, TILE_HEIGHT/2, 4, Rgba([200, 200, 200, 255]));
    
    img
}

fn create_spike_trap() -> RgbaImage {
    let mut img = create_solid_rock();
    // Spikes holes
    for y in (10..TILE_HEIGHT-10).step_by(10) {
        for x in (10..TILE_WIDTH-10).step_by(10) {
             draw_circle(&mut img, x, y, 2, Rgba([30, 30, 30, 255]));
             // Shiny spike tip
             img.put_pixel(x, y, Rgba([200, 200, 200, 255]));
        }
    }
    img
}

fn create_boulder_trap() -> RgbaImage {
    let mut img = create_solid_rock();
    // Large boulder
    let cx = TILE_WIDTH / 2;
    let cy = TILE_HEIGHT / 2;
    draw_circle(&mut img, cx, cy, 20, Rgba([80, 80, 85, 255]));
    // Shading
    draw_circle(&mut img, cx-5, cy-5, 10, Rgba([100, 100, 105, 255]));
    img
}

fn create_alarm_trap() -> RgbaImage {
    let mut img = create_solid_rock();
    // Mechanical mechanism
    let cx = TILE_WIDTH / 2;
    let cy = TILE_HEIGHT / 2;
    draw_rect(&mut img, cx-10, cy-10, 20, 20, Rgba([150, 100, 50, 255])); // Bronze plate
    draw_circle(&mut img, cx, cy, 8, Rgba([200, 50, 50, 255])); // Red light/gem
    // Wires
    draw_rect(&mut img, 0, cy, TILE_WIDTH, 2, Rgba([100, 100, 100, 255]));
    img
}


// Sprite generators
fn generate_monster_sprites() {
    save_sprite("monsters", "imp", create_imp_sprite());
    save_sprite("monsters", "goblin", create_goblin_sprite());
    save_sprite("monsters", "orc", create_orc_sprite());
    save_sprite("monsters", "warlock", create_warlock_sprite());
    save_sprite("monsters", "troll", create_troll_sprite());
    save_sprite("monsters", "skeleton", create_skeleton_sprite());
    save_sprite("monsters", "demon_spawn", create_demon_spawn_sprite());
    save_sprite("monsters", "spider", create_spider_sprite());
    save_sprite("monsters", "lizard", create_lizard_sprite());
}

fn generate_hero_sprites() {
    save_sprite("heroes", "peasant_militia", create_peasant_sprite());
    save_sprite("heroes", "scout", create_scout_sprite());
    save_sprite("heroes", "acolyte", create_acolyte_sprite());
    save_sprite("heroes", "knight", create_knight_sprite());
    save_sprite("heroes", "archer", create_archer_sprite());
    save_sprite("heroes", "battle_cleric", create_battle_cleric_sprite());
    save_sprite("heroes", "rogue", create_rogue_sprite());
    save_sprite("heroes", "paladin", create_paladin_sprite());
    save_sprite("heroes", "wizard", create_wizard_sprite());
    save_sprite("heroes", "inquisitor", create_inquisitor_sprite());
    save_sprite("heroes", "knight_commander", create_knight_commander_sprite());
    save_sprite("heroes", "high_priest", create_high_priest_sprite());
    save_sprite("heroes", "archmage", create_archmage_sprite());
    save_sprite("heroes", "champion_of_light", create_champion_sprite());
}

// Helper to draw a filled rectangle
fn draw_rect(img: &mut RgbaImage, x: u32, y: u32, w: u32, h: u32, color: Rgba<u8>) {
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

// Helper to draw a circle
fn draw_circle(img: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
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

// ============================================================================
// 3D RENDERING INFRASTRUCTURE
// ============================================================================

// Light direction (normalized) - from top-left-front
const LIGHT_X: f32 = -0.5;
const LIGHT_Y: f32 = -0.7;
const LIGHT_Z: f32 = 0.5;

/// Material properties for 3D shading
#[derive(Clone, Copy)]
struct Material {
    base_color: [u8; 3],
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl Material {
    fn matte(r: u8, g: u8, b: u8) -> Self {
        Material { base_color: [r, g, b], ambient: 0.3, diffuse: 0.7, specular: 0.0, shininess: 1.0 }
    }
    fn metallic(r: u8, g: u8, b: u8) -> Self {
        Material { base_color: [r, g, b], ambient: 0.2, diffuse: 0.5, specular: 0.8, shininess: 32.0 }
    }
    fn glowing(r: u8, g: u8, b: u8) -> Self {
        Material { base_color: [r, g, b], ambient: 0.9, diffuse: 0.1, specular: 0.3, shininess: 8.0 }
    }
    fn bone() -> Self {
        Material { base_color: [240, 235, 220], ambient: 0.4, diffuse: 0.6, specular: 0.2, shininess: 4.0 }
    }
    fn leather(r: u8, g: u8, b: u8) -> Self {
        Material { base_color: [r, g, b], ambient: 0.25, diffuse: 0.7, specular: 0.1, shininess: 2.0 }
    }
}

/// Calculate shading for a surface normal
fn shade_color(normal: (f32, f32, f32), mat: &Material, view_z: f32) -> Rgba<u8> {
    // Normalize normal
    let len = (normal.0*normal.0 + normal.1*normal.1 + normal.2*normal.2).sqrt();
    let (nx, ny, nz) = if len > 0.0 { (normal.0/len, normal.1/len, normal.2/len) } else { (0.0, 0.0, 1.0) };
    
    // Diffuse lighting
    let dot = -(nx * LIGHT_X + ny * LIGHT_Y + nz * LIGHT_Z);
    let diffuse = dot.max(0.0) * mat.diffuse;
    
    // Specular (Blinn-Phong)
    let hx = -LIGHT_X;
    let hy = -LIGHT_Y;
    let hz = -LIGHT_Z + 1.0;
    let hlen = (hx*hx + hy*hy + hz*hz).sqrt();
    let spec_dot = (nx * hx/hlen + ny * hy/hlen + nz * hz/hlen).max(0.0);
    let specular = spec_dot.powf(mat.shininess) * mat.specular;
    
    let intensity = (mat.ambient + diffuse + specular).min(1.5);
    
    let r = ((mat.base_color[0] as f32 * intensity).min(255.0)) as u8;
    let g = ((mat.base_color[1] as f32 * intensity).min(255.0)) as u8;
    let b = ((mat.base_color[2] as f32 * intensity).min(255.0)) as u8;
    
    Rgba([r, g, b, 255])
}

/// Depth buffer for proper 3D overlap
struct DepthBuffer {
    data: Vec<f32>,
    width: usize,
    height: usize,
}

impl DepthBuffer {
    fn new(w: u32, h: u32) -> Self {
        DepthBuffer {
            data: vec![f32::NEG_INFINITY; (w * h) as usize],
            width: w as usize,
            height: h as usize,
        }
    }
    
    fn test_and_set(&mut self, x: u32, y: u32, z: f32) -> bool {
        if x >= self.width as u32 || y >= self.height as u32 { return false; }
        let idx = y as usize * self.width + x as usize;
        if z > self.data[idx] {
            self.data[idx] = z;
            true
        } else {
            false
        }
    }
}

/// Draw a 3D shaded sphere
fn draw_sphere_3d(img: &mut RgbaImage, depth: &mut DepthBuffer, cx: f32, cy: f32, cz: f32, radius: f32, mat: &Material) {
    let r_int = radius.ceil() as i32;
    for dy in -r_int..=r_int {
        for dx in -r_int..=r_int {
            let dist_sq = (dx * dx + dy * dy) as f32;
            if dist_sq <= radius * radius {
                let px = (cx + dx as f32) as i32;
                let py = (cy + dy as f32) as i32;
                if px >= 0 && py >= 0 && px < SPRITE_SIZE as i32 && py < SPRITE_SIZE as i32 {
                    // Calculate z on sphere surface
                    let z_offset = (radius * radius - dist_sq).sqrt();
                    let pz = cz + z_offset;
                    
                    // Normal at this point
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
fn draw_ellipsoid_3d(img: &mut RgbaImage, depth: &mut DepthBuffer, cx: f32, cy: f32, cz: f32, rx: f32, ry: f32, rz: f32, mat: &Material) {
    let max_r = rx.max(ry).max(rz).ceil() as i32;
    for dy in -max_r..=max_r {
        for dx in -max_r..=max_r {
            let nx = dx as f32 / rx;
            let ny = dy as f32 / ry;
            let dist_sq = nx * nx + ny * ny;
            if dist_sq <= 1.0 {
                let px = (cx + dx as f32) as i32;
                let py = (cy + dy as f32) as i32;
                if px >= 0 && py >= 0 && px < SPRITE_SIZE as i32 && py < SPRITE_SIZE as i32 {
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
fn draw_cylinder_3d(img: &mut RgbaImage, depth: &mut DepthBuffer, cx: f32, y_top: f32, y_bot: f32, cz: f32, radius: f32, mat: &Material) {
    let r_int = radius.ceil() as i32;
    for y in (y_top as i32)..=(y_bot as i32) {
        for dx in -r_int..=r_int {
            if (dx as f32).abs() <= radius {
                let px = (cx + dx as f32) as i32;
                if px >= 0 && y >= 0 && px < SPRITE_SIZE as i32 && y < SPRITE_SIZE as i32 {
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
fn draw_cone_3d(img: &mut RgbaImage, depth: &mut DepthBuffer, cx: f32, y_tip: f32, y_base: f32, cz: f32, base_radius: f32, mat: &Material) {
    let height = y_base - y_tip;
    if height <= 0.0 { return; }
    for y in (y_tip as i32)..=(y_base as i32) {
        let t = (y as f32 - y_tip) / height;
        let r = base_radius * t;
        let r_int = r.ceil() as i32;
        for dx in -r_int..=r_int {
            if (dx as f32).abs() <= r && r > 0.0 {
                let px = (cx + dx as f32) as i32;
                if px >= 0 && y >= 0 && px < SPRITE_SIZE as i32 && y < SPRITE_SIZE as i32 {
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

/// Draw a simple ground shadow (ellipse)
fn draw_shadow(img: &mut RgbaImage, cx: f32, cy: f32, rx: f32, ry: f32) {
    for dy in -(ry as i32)..=(ry as i32) {
        for dx in -(rx as i32)..=(rx as i32) {
            let nx = dx as f32 / rx;
            let ny = dy as f32 / ry;
            if nx * nx + ny * ny <= 1.0 {
                let px = (cx + dx as f32) as i32;
                let py = (cy + dy as f32) as i32;
                if px >= 0 && py >= 0 && px < SPRITE_SIZE as i32 && py < SPRITE_SIZE as i32 {
                    let pixel = img.get_pixel(px as u32, py as u32);
                    if pixel[3] == 0 {
                        img.put_pixel(px as u32, py as u32, Rgba([20, 20, 25, 100]));
                    }
                }
            }
        }
    }
}

// ============================================================================
// 3D SPRITE GENERATORS
// ============================================================================

// Monster sprites - 3D shaded versions
fn create_imp_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let body_mat = Material::matte(200, 50, 50);
    let horn_mat = Material::matte(100, 20, 20);
    let eye_mat = Material::glowing(255, 255, 0);
    
    // Shadow
    draw_shadow(&mut img, cx, 54.0, 10.0, 4.0);
    // Body (ellipsoid)
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 5.0, 9.0, 11.0, 8.0, &body_mat);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 8.0, 8.0, &body_mat);
    // Horns
    draw_cone_3d(&mut img, &mut depth, cx - 8.0, 12.0, 22.0, 6.0, 3.0, &horn_mat);
    draw_cone_3d(&mut img, &mut depth, cx + 8.0, 12.0, 22.0, 6.0, 3.0, &horn_mat);
    // Tail
    draw_cylinder_3d(&mut img, &mut depth, cx, 46.0, 58.0, 3.0, 3.0, &body_mat);
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 22.0, 12.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 22.0, 12.0, 2.0, &eye_mat);
    
    img
}

fn create_goblin_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let body_mat = Material::matte(50, 150, 50);
    let ear_mat = Material::matte(70, 170, 70);
    let eye_mat = Material::glowing(255, 50, 0);
    let wood_mat = Material::matte(139, 90, 43);
    
    draw_shadow(&mut img, cx, 56.0, 11.0, 4.0);
    // Hunched body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 42.0, 5.0, 11.0, 13.0, 9.0, &body_mat);
    // Large head
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 8.0, 10.0, &body_mat);
    // Pointed ears
    draw_cone_3d(&mut img, &mut depth, cx - 12.0, 18.0, 28.0, 5.0, 4.0, &ear_mat);
    draw_cone_3d(&mut img, &mut depth, cx + 12.0, 18.0, 28.0, 5.0, 4.0, &ear_mat);
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 24.0, 12.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 24.0, 12.0, 2.0, &eye_mat);
    // Crude club
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 20.0, 44.0, 4.0, 3.0, &wood_mat);
    
    img
}

fn create_orc_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let body_mat = Material::matte(100, 180, 80);
    let tusk_mat = Material::bone();
    let eye_mat = Material::glowing(255, 50, 0);
    let axe_mat = Material::metallic(169, 169, 169);
    let wood_mat = Material::matte(139, 90, 43);
    
    draw_shadow(&mut img, cx, 58.0, 14.0, 5.0);
    // Large muscular body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 6.0, 13.0, 16.0, 11.0, &body_mat);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 9.0, 10.0, &body_mat);
    // Tusks
    draw_cone_3d(&mut img, &mut depth, cx - 5.0, 28.0, 36.0, 10.0, 2.0, &tusk_mat);
    draw_cone_3d(&mut img, &mut depth, cx + 5.0, 28.0, 36.0, 10.0, 2.0, &tusk_mat);
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 20.0, 13.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 20.0, 13.0, 2.0, &eye_mat);
    // Battle axe handle
    draw_cylinder_3d(&mut img, &mut depth, cx + 20.0, 16.0, 48.0, 5.0, 2.0, &wood_mat);
    // Axe head
    draw_ellipsoid_3d(&mut img, &mut depth, cx + 24.0, 18.0, 8.0, 8.0, 4.0, 3.0, &axe_mat);
    
    img
}

fn create_warlock_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let robe_mat = Material::matte(80, 40, 120);
    let hood_mat = Material::matte(60, 30, 90);
    let eye_mat = Material::glowing(150, 255, 150);
    let wood_mat = Material::matte(101, 67, 33);
    let crystal_mat = Material::glowing(200, 100, 255);
    
    draw_shadow(&mut img, cx, 58.0, 11.0, 4.0);
    // Robed body (cylinder)
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 56.0, 5.0, 11.0, &robe_mat);
    // Hood
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 7.0, 9.0, &hood_mat);
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 24.0, 12.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 24.0, 12.0, 2.0, &eye_mat);
    // Staff
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 12.0, 54.0, 4.0, 2.0, &wood_mat);
    // Crystal on staff
    draw_sphere_3d(&mut img, &mut depth, cx - 14.0, 10.0, 8.0, 5.0, &crystal_mat);
    
    img
}

fn create_troll_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let body_mat = Material::matte(120, 140, 100);
    let eye_mat = Material::glowing(255, 255, 0);
    let club_mat = Material::matte(101, 67, 33);
    
    draw_shadow(&mut img, cx, 60.0, 16.0, 6.0);
    // Very large hunched body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 42.0, 6.0, 15.0, 18.0, 13.0, &body_mat);
    // Large head
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 10.0, 12.0, &body_mat);
    // Arms
    draw_cylinder_3d(&mut img, &mut depth, cx - 16.0, 32.0, 52.0, 4.0, 5.0, &body_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 32.0, 52.0, 4.0, 5.0, &body_mat);
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 5.0, 20.0, 14.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 5.0, 20.0, 14.0, 2.0, &eye_mat);
    // Huge club
    draw_cylinder_3d(&mut img, &mut depth, cx + 22.0, 18.0, 50.0, 6.0, 4.0, &club_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 22.0, 14.0, 8.0, 7.0, &club_mat);
    
    img
}

fn create_skeleton_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let bone_mat = Material::bone();
    let eye_mat = Material::matte(20, 20, 20);
    let sword_mat = Material::metallic(192, 192, 192);
    
    draw_shadow(&mut img, cx, 58.0, 8.0, 3.0);
    // Skull
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 8.0, 9.0, &bone_mat);
    // Eye sockets
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 20.0, 12.0, 3.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 20.0, 12.0, 3.0, &eye_mat);
    // Spine
    draw_cylinder_3d(&mut img, &mut depth, cx, 32.0, 54.0, 5.0, 2.0, &bone_mat);
    // Ribs
    for i in 0..3 {
        let y = 36.0 + i as f32 * 5.0;
        draw_ellipsoid_3d(&mut img, &mut depth, cx, y, 5.0, 8.0, 2.0, 4.0, &bone_mat);
    }
    // Arms
    draw_cylinder_3d(&mut img, &mut depth, cx - 10.0, 34.0, 50.0, 3.0, 2.0, &bone_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 10.0, 34.0, 50.0, 3.0, 2.0, &bone_mat);
    // Sword
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 12.0, 46.0, 7.0, 2.0, &sword_mat);
    
    img
}

fn create_demon_spawn_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let body_mat = Material::matte(180, 20, 20);
    let horn_mat = Material::matte(80, 0, 0);
    let eye_mat = Material::glowing(255, 100, 0);
    let fire_mat = Material::glowing(255, 140, 0);
    let claw_mat = Material::matte(100, 10, 10);
    
    // Fire at feet
    draw_sphere_3d(&mut img, &mut depth, cx - 8.0, 56.0, 2.0, 4.0, &fire_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 8.0, 56.0, 2.0, 4.0, &fire_mat);
    
    draw_shadow(&mut img, cx, 58.0, 13.0, 5.0);
    // Dark red demon body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 6.0, 13.0, 16.0, 11.0, &body_mat);
    // Horned head
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 9.0, 10.0, &body_mat);
    // Large horns
    draw_cone_3d(&mut img, &mut depth, cx - 10.0, 4.0, 18.0, 7.0, 4.0, &horn_mat);
    draw_cone_3d(&mut img, &mut depth, cx + 10.0, 4.0, 18.0, 7.0, 4.0, &horn_mat);
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 18.0, 13.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 18.0, 13.0, 2.0, &eye_mat);
    // Claws/arms
    draw_cylinder_3d(&mut img, &mut depth, cx - 16.0, 32.0, 48.0, 4.0, 4.0, &body_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 32.0, 48.0, 4.0, 4.0, &body_mat);
    draw_cone_3d(&mut img, &mut depth, cx - 18.0, 48.0, 56.0, 3.0, 3.0, &claw_mat);
    draw_cone_3d(&mut img, &mut depth, cx + 18.0, 48.0, 56.0, 3.0, 3.0, &claw_mat);
    
    img
}


// Hero sprites - 3D shaded versions
fn create_peasant_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let cloth_mat = Material::matte(160, 140, 100);
    let skin_mat = Material::matte(230, 190, 150);
    let wood_mat = Material::matte(139, 90, 43);
    let metal_mat = Material::metallic(169, 169, 169);
    
    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);
    // Body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 42.0, 5.0, 9.0, 12.0, 8.0, &cloth_mat);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 8.0, 7.0, &skin_mat);
    // Pitchfork handle
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 14.0, 50.0, 4.0, 2.0, &wood_mat);
    // Pitchfork tines
    draw_cylinder_3d(&mut img, &mut depth, cx + 10.0, 10.0, 18.0, 5.0, 1.0, &metal_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 10.0, 18.0, 5.0, 1.0, &metal_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 18.0, 10.0, 18.0, 5.0, 1.0, &metal_mat);
    
    img
}

fn create_scout_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let leather_mat = Material::leather(100, 120, 80);
    let skin_mat = Material::matte(230, 190, 150);
    let wood_mat = Material::matte(139, 90, 43);
    
    draw_shadow(&mut img, cx, 56.0, 9.0, 4.0);
    // Lean body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 5.0, 9.0, 13.0, 7.0, &leather_mat);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 8.0, 7.0, &skin_mat);
    // Hood
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 6.0, 8.0, &leather_mat);
    // Bow
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 18.0, 48.0, 4.0, 2.0, &wood_mat);
    
    img
}

fn create_acolyte_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let robe_mat = Material::matte(240, 240, 250);
    let skin_mat = Material::matte(230, 190, 150);
    let gold_mat = Material::metallic(255, 215, 0);
    
    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);
    // Robed body
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 56.0, 5.0, 11.0, &robe_mat);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 8.0, 7.0, &skin_mat);
    // Holy symbol (cross)
    draw_cylinder_3d(&mut img, &mut depth, cx, 38.0, 52.0, 8.0, 2.0, &gold_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 42.0, 9.0, 5.0, 2.0, 2.0, &gold_mat);
    
    img
}

fn create_knight_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let armor_mat = Material::metallic(180, 180, 200);
    let plume_mat = Material::matte(200, 0, 0);
    let shield_mat = Material::metallic(100, 100, 120);
    let sword_mat = Material::metallic(220, 220, 220);
    
    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    // Armored body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 6.0, 11.0, 15.0, 10.0, &armor_mat);
    // Helmet
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 9.0, 9.0, &armor_mat);
    // Plume
    draw_cylinder_3d(&mut img, &mut depth, cx, 8.0, 20.0, 8.0, 3.0, &plume_mat);
    // Shield
    draw_ellipsoid_3d(&mut img, &mut depth, cx - 16.0, 38.0, 10.0, 7.0, 10.0, 5.0, &shield_mat);
    // Sword
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 14.0, 48.0, 8.0, 2.0, &sword_mat);
    
    img
}

fn create_archer_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let leather_mat = Material::leather(120, 100, 70);
    let skin_mat = Material::matte(230, 190, 150);
    let wood_mat = Material::matte(139, 90, 43);
    let metal_mat = Material::metallic(169, 169, 169);
    
    draw_shadow(&mut img, cx, 56.0, 10.0, 4.0);
    // Body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 5.0, 9.0, 13.0, 8.0, &leather_mat);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 8.0, 7.0, &skin_mat);
    // Longbow
    draw_cylinder_3d(&mut img, &mut depth, cx - 16.0, 14.0, 50.0, 4.0, 2.0, &wood_mat);
    // Arrow
    draw_cylinder_3d(&mut img, &mut depth, cx - 10.0, 28.0, 30.0, 6.0, 1.0, &wood_mat);
    draw_cone_3d(&mut img, &mut depth, cx - 4.0, 27.0, 30.0, 7.0, 2.0, &metal_mat);
    
    img
}

fn create_battle_cleric_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let armor_mat = Material::metallic(200, 200, 220);
    let skin_mat = Material::matte(230, 190, 150);
    let gold_mat = Material::metallic(255, 215, 0);
    let wood_mat = Material::matte(139, 90, 43);
    let mace_mat = Material::metallic(150, 150, 150);
    
    draw_shadow(&mut img, cx, 58.0, 11.0, 5.0);
    // Armored body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 6.0, 11.0, 14.0, 9.0, &armor_mat);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 8.0, 7.0, &skin_mat);
    // Holy symbol
    draw_cylinder_3d(&mut img, &mut depth, cx, 36.0, 48.0, 8.0, 2.0, &gold_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 9.0, 5.0, 2.0, 2.0, &gold_mat);
    // Mace handle
    draw_cylinder_3d(&mut img, &mut depth, cx + 18.0, 26.0, 50.0, 5.0, 2.0, &wood_mat);
    // Mace head
    draw_sphere_3d(&mut img, &mut depth, cx + 18.0, 22.0, 8.0, 5.0, &mace_mat);
    
    img
}

fn create_rogue_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let cloak_mat = Material::matte(60, 60, 80);
    let hood_mat = Material::matte(50, 50, 70);
    let eye_mat = Material::glowing(255, 255, 0);
    let dagger_mat = Material::metallic(192, 192, 192);
    
    draw_shadow(&mut img, cx, 56.0, 9.0, 4.0);
    // Dark body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 5.0, 9.0, 13.0, 7.0, &cloak_mat);
    // Hooded head
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 7.0, 8.0, &hood_mat);
    // Eyes (only visible part)
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 24.0, 10.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 24.0, 10.0, 2.0, &eye_mat);
    // Daggers
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 32.0, 48.0, 5.0, 1.0, &dagger_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 32.0, 48.0, 5.0, 1.0, &dagger_mat);
    
    img
}

fn create_paladin_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let armor_mat = Material::metallic(220, 200, 100);
    let glow_mat = Material::glowing(255, 255, 200);
    let sword_mat = Material::glowing(255, 255, 255);
    let shield_mat = Material::metallic(200, 200, 220);
    let gold_mat = Material::metallic(255, 215, 0);
    
    draw_shadow(&mut img, cx, 58.0, 13.0, 5.0);
    // Golden armored body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 7.0, 12.0, 16.0, 11.0, &armor_mat);
    // Helmet with glow
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 10.0, 10.0, &armor_mat);
    // Holy aura (behind)
    draw_sphere_3d(&mut img, &mut depth, cx, 34.0, -5.0, 18.0, &glow_mat);
    // Holy sword
    draw_cylinder_3d(&mut img, &mut depth, cx + 18.0, 10.0, 48.0, 9.0, 2.0, &sword_mat);
    // Shield
    draw_ellipsoid_3d(&mut img, &mut depth, cx - 18.0, 36.0, 10.0, 8.0, 11.0, 6.0, &shield_mat);
    // Cross on shield
    draw_cylinder_3d(&mut img, &mut depth, cx - 18.0, 30.0, 42.0, 12.0, 1.0, &gold_mat);
    
    img
}

fn create_wizard_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let robe_mat = Material::matte(50, 80, 200);
    let hat_mat = Material::matte(40, 70, 180);
    let skin_mat = Material::matte(230, 190, 150);
    let wood_mat = Material::matte(139, 90, 43);
    let orb_mat = Material::glowing(100, 200, 255);
    let star_mat = Material::glowing(255, 255, 0);
    
    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);
    // Robed body
    draw_cylinder_3d(&mut img, &mut depth, cx, 36.0, 56.0, 5.0, 10.0, &robe_mat);
    // Wizard hat (cone)
    draw_cone_3d(&mut img, &mut depth, cx, 6.0, 28.0, 6.0, 8.0, &hat_mat);
    // Face
    draw_sphere_3d(&mut img, &mut depth, cx, 28.0, 8.0, 6.0, &skin_mat);
    // Stars on robe
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 44.0, 8.0, 2.0, &star_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 48.0, 8.0, 2.0, &star_mat);
    // Staff
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 10.0, 54.0, 4.0, 2.0, &wood_mat);
    // Orb on staff
    draw_sphere_3d(&mut img, &mut depth, cx - 14.0, 8.0, 8.0, 6.0, &orb_mat);
    
    img
}

fn create_inquisitor_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let robe_mat = Material::matte(140, 30, 30);
    let hood_mat = Material::matte(60, 10, 10);
    let eye_mat = Material::glowing(255, 0, 0);
    let sword_mat = Material::metallic(220, 220, 220);
    let fire_mat = Material::glowing(255, 100, 0);
    
    draw_shadow(&mut img, cx, 58.0, 11.0, 4.0);
    // Dark robed body
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 56.0, 5.0, 11.0, &robe_mat);
    // Grim hood
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 7.0, 8.0, &hood_mat);
    // Red eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 24.0, 11.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 24.0, 11.0, 2.0, &eye_mat);
    // Flaming sword
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 14.0, 48.0, 8.0, 2.0, &sword_mat);
    // Fire on sword
    draw_sphere_3d(&mut img, &mut depth, cx + 16.0, 12.0, 10.0, 5.0, &fire_mat);
    
    img
}

fn create_knight_commander_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let armor_mat = Material::metallic(200, 200, 220);
    let plume_mat = Material::matte(180, 0, 0);
    let cape_mat = Material::matte(150, 0, 0);
    let wood_mat = Material::matte(139, 90, 43);
    let banner_mat = Material::metallic(255, 215, 0);
    
    draw_shadow(&mut img, cx, 58.0, 13.0, 5.0);
    // Ornate armored body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 7.0, 12.0, 16.0, 11.0, &armor_mat);
    // Helmet
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 10.0, 10.0, &armor_mat);
    // Plume
    draw_cylinder_3d(&mut img, &mut depth, cx, 4.0, 18.0, 9.0, 4.0, &plume_mat);
    // Cape (behind)
    draw_cylinder_3d(&mut img, &mut depth, cx - 10.0, 32.0, 56.0, -2.0, 8.0, &cape_mat);
    // Banner pole
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 6.0, 52.0, 5.0, 2.0, &wood_mat);
    // Banner
    draw_ellipsoid_3d(&mut img, &mut depth, cx + 22.0, 12.0, 6.0, 8.0, 6.0, 3.0, &banner_mat);
    
    img
}

fn create_high_priest_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let robe_mat = Material::matte(240, 230, 200);
    let gold_mat = Material::metallic(255, 215, 0);
    let skin_mat = Material::matte(230, 190, 150);
    let orb_mat = Material::glowing(255, 255, 255);
    
    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    // Elaborate robes
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 56.0, 5.0, 12.0, &robe_mat);
    // Ornate headdress
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 16.0, 8.0, 12.0, 5.0, 6.0, &gold_mat);
    // Face
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 9.0, 6.0, &skin_mat);
    // Golden staff
    draw_cylinder_3d(&mut img, &mut depth, cx - 16.0, 8.0, 54.0, 5.0, 2.0, &gold_mat);
    // Divine orb
    draw_sphere_3d(&mut img, &mut depth, cx - 16.0, 6.0, 9.0, 7.0, &orb_mat);
    
    img
}

fn create_archmage_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let robe_mat = Material::matte(100, 50, 200);
    let hat_mat = Material::matte(80, 40, 180);
    let aura_mat = Material::glowing(150, 100, 255);
    let wood_mat = Material::matte(139, 90, 43);
    let energy_mat = Material::glowing(200, 150, 255);
    
    // Magical aura (drawn first, behind)
    draw_sphere_3d(&mut img, &mut depth, cx, 36.0, -8.0, 20.0, &aura_mat);
    
    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    // Purple robes
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 54.0, 5.0, 12.0, &robe_mat);
    // Wizard hat (tall cone)
    draw_cone_3d(&mut img, &mut depth, cx, 4.0, 28.0, 6.0, 9.0, &hat_mat);
    // Staff
    draw_cylinder_3d(&mut img, &mut depth, cx - 18.0, 6.0, 54.0, 4.0, 2.0, &wood_mat);
    // Massive energy orb
    draw_sphere_3d(&mut img, &mut depth, cx - 18.0, 4.0, 10.0, 9.0, &energy_mat);
    
    img
}

fn create_champion_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;
    
    let armor_mat = Material::metallic(255, 215, 100);
    let divine_mat = Material::glowing(255, 255, 200);
    let sword_mat = Material::glowing(255, 255, 255);
    let shield_mat = Material::metallic(255, 240, 180);
    let gold_mat = Material::metallic(255, 215, 0);
    
    // Divine aura (behind)
    draw_sphere_3d(&mut img, &mut depth, cx, 32.0, -10.0, 24.0, &divine_mat);
    
    draw_shadow(&mut img, cx, 60.0, 14.0, 6.0);
    // Radiant golden armor
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 36.0, 8.0, 14.0, 18.0, 12.0, &armor_mat);
    // Helmet
    draw_sphere_3d(&mut img, &mut depth, cx, 16.0, 11.0, 11.0, &armor_mat);
    // Halo
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 6.0, 10.0, 10.0, 2.0, 8.0, &divine_mat);
    // Divine greatsword
    draw_cylinder_3d(&mut img, &mut depth, cx + 20.0, 6.0, 52.0, 10.0, 3.0, &sword_mat);
    // Sword glow
    draw_sphere_3d(&mut img, &mut depth, cx + 20.0, 4.0, 12.0, 7.0, &divine_mat);
    // Shield
    draw_ellipsoid_3d(&mut img, &mut depth, cx - 20.0, 34.0, 12.0, 9.0, 12.0, 7.0, &shield_mat);
    // Cross on shield
    draw_cylinder_3d(&mut img, &mut depth, cx - 20.0, 28.0, 42.0, 14.0, 2.0, &gold_mat);
    
    img
}

fn create_spider_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let body_mat = Material::matte(40, 40, 45); // Dark grey/black
    let abdomen_mat = Material::matte(30, 30, 35); // Slightly darker
    let eye_mat = Material::glowing(200, 20, 20); // Red glowing eyes

    draw_shadow(&mut img, cx, 58.0, 15.0, 8.0);

    // Abdomen (large back part)
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 36.0, 20.0, 12.0, 10.0, 15.0, &abdomen_mat);

    // Cephalothorax (head/body)
    draw_sphere_3d(&mut img, &mut depth, cx, 42.0, 12.0, 8.0, &body_mat);

    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 40.0, 16.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 40.0, 16.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 6.0, 39.0, 15.0, 1.5, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 6.0, 39.0, 15.0, 1.5, &eye_mat);

    // Legs (8 legs, simplified as angled cylinders)
    let leg_mat = Material::matte(35, 35, 40);
    // Left legs
    draw_cylinder_3d(&mut img, &mut depth, cx - 12.0, 45.0, 58.0, 10.0, 2.0, &leg_mat); // Front
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 40.0, 56.0, 15.0, 2.0, &leg_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 35.0, 56.0, 20.0, 2.0, &leg_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 12.0, 30.0, 58.0, 25.0, 2.0, &leg_mat); // Back

    // Right legs
    draw_cylinder_3d(&mut img, &mut depth, cx + 12.0, 45.0, 58.0, 10.0, 2.0, &leg_mat); // Front
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 40.0, 56.0, 15.0, 2.0, &leg_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 35.0, 56.0, 20.0, 2.0, &leg_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 12.0, 30.0, 58.0, 25.0, 2.0, &leg_mat); // Back
    
    img
}

fn create_lizard_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let scale_mat = Material::metallic(60, 140, 60); // Green scales
    let shadow_mat = Material::matte(40, 100, 40); // Darker underside
    let eye_mat = Material::glowing(255, 255, 0); // Yellow eyes

    draw_shadow(&mut img, cx, 58.0, 18.0, 6.0);

    // Tail (behind)
    draw_cone_3d(&mut img, &mut depth, cx - 5.0, 40.0, 58.0, -10.0, 5.0, &scale_mat);

    // Body (long ellipsoid)
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 45.0, 5.0, 10.0, 18.0, 8.0, &scale_mat);

    // Underside/Legs
    draw_cylinder_3d(&mut img, &mut depth, cx - 8.0, 48.0, 58.0, 5.0, 3.0, &shadow_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 8.0, 48.0, 58.0, 5.0, 3.0, &shadow_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 6.0, 52.0, 58.0, 15.0, 3.0, &shadow_mat); // Back legs
    draw_cylinder_3d(&mut img, &mut depth, cx + 6.0, 52.0, 58.0, 15.0, 3.0, &shadow_mat);

    // Head
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 30.0, 8.0, 8.0, 10.0, 6.0, &scale_mat);

    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 28.0, 10.0, 1.5, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 28.0, 10.0, 1.5, &eye_mat);

    img
}

// Hero Building Sprites
fn create_town_hall() -> RgbaImage {
    let mut img = create_tile_base(Rgba([180, 160, 140, 255])); // Stone base
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let stone_mat = Material::matte(180, 160, 140);
    let gold_mat = Material::metallic(218, 165, 32);
    let wood_mat = Material::matte(139, 90, 43);
    let banner_mat = Material::matte(180, 40, 40);
    let light_mat = Material::glowing(255, 200, 100);

    // Main hall body
    draw_cylinder_3d(&mut img, &mut depth, cx, 20.0, 50.0, 5.0, 20.0, &stone_mat);
    
    // Central tower
    draw_cylinder_3d(&mut img, &mut depth, cx, 10.0, 30.0, 0.0, 8.0, &stone_mat);
    
    // Golden roof on tower
    draw_cone_3d(&mut img, &mut depth, cx, -5.0, 10.0, 0.0, 10.0, &gold_mat);
    
    // Banner
    draw_cylinder_3d(&mut img, &mut depth, cx + 12.0, 15.0, 40.0, 15.0, 2.0, &wood_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx + 16.0, 20.0, 15.0, 5.0, 8.0, 2.0, &banner_mat);

    // Windows
    draw_sphere_3d(&mut img, &mut depth, cx - 8.0, 30.0, 24.0, 3.0, &light_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 8.0, 30.0, 24.0, 3.0, &light_mat);

    add_noise(&mut img, 8);
    img
}

fn create_barracks() -> RgbaImage {
    let mut img = create_tile_base(Rgba([139, 90, 43, 255])); // Wood base
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let wood_mat = Material::matte(120, 80, 40);
    let dark_wood_mat = Material::matte(80, 50, 20);
    let shield_mat = Material::metallic(160, 160, 180);
    let red_mat = Material::matte(150, 30, 30);

    // Main building
    draw_cylinder_3d(&mut img, &mut depth, cx, 20.0, 50.0, 5.0, 18.0, &wood_mat);
    
    // Roof (conical for now, simplified)
    draw_cone_3d(&mut img, &mut depth, cx, 5.0, 20.0, 5.0, 22.0, &dark_wood_mat);

    // Weapon racks / Shields outside
    draw_ellipsoid_3d(&mut img, &mut depth, cx - 12.0, 40.0, 22.0, 4.0, 6.0, 2.0, &shield_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 12.0, 40.0, 24.0, 2.0, &red_mat);

    draw_ellipsoid_3d(&mut img, &mut depth, cx + 12.0, 40.0, 22.0, 4.0, 6.0, 2.0, &shield_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 12.0, 40.0, 24.0, 2.0, &red_mat);

    add_noise(&mut img, 10);
    img
}

fn create_archery_range() -> RgbaImage {
    let mut img = create_tile_base(Rgba([100, 120, 80, 255])); // Grass/Dirt base
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let wood_mat = Material::matte(139, 90, 43);
    let target_white = Material::matte(220, 220, 220);
    let target_red = Material::matte(200, 20, 20);
    let hay_mat = Material::matte(200, 180, 100);

    // Fence posts
    for i in 0..5 {
        let x = 12.0 + i as f32 * 10.0;
        draw_cylinder_3d(&mut img, &mut depth, x, 40.0, 55.0, 20.0, 2.0, &wood_mat);
    }
    // Rails
    draw_cylinder_3d(&mut img, &mut depth, cx, 42.0, 44.0, 20.0, 25.0, &wood_mat);

    // Target 1
    draw_cylinder_3d(&mut img, &mut depth, cx - 10.0, 30.0, 45.0, 10.0, 2.0, &wood_mat); // Stand
    draw_ellipsoid_3d(&mut img, &mut depth, cx - 10.0, 25.0, 8.0, 5.0, 5.0, 2.0, &hay_mat); // Hay backing
    draw_ellipsoid_3d(&mut img, &mut depth, cx - 10.0, 25.0, 6.0, 4.0, 4.0, 2.0, &target_white);
    draw_sphere_3d(&mut img, &mut depth, cx - 10.0, 25.0, 5.0, 1.5, &target_red); // Bullseye

    // Target 2
    draw_cylinder_3d(&mut img, &mut depth, cx + 15.0, 35.0, 50.0, 5.0, 2.0, &wood_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx + 15.0, 30.0, 3.0, 5.0, 5.0, 2.0, &hay_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx + 15.0, 30.0, 2.0, 4.0, 4.0, 2.0, &target_white);
    draw_sphere_3d(&mut img, &mut depth, cx + 15.0, 30.0, 1.0, 1.5, &target_red);

    add_noise(&mut img, 15);
    img
}

fn create_church() -> RgbaImage {
    let mut img = create_tile_base(Rgba([230, 230, 240, 255])); // White stone base
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let stone_mat = Material::matte(220, 220, 230);
    let roof_mat = Material::matte(60, 60, 80); // Dark slate roof
    let gold_mat = Material::metallic(218, 165, 32);
    let glass_mat = Material::glowing(100, 150, 255);

    // Main Nave
    draw_cylinder_3d(&mut img, &mut depth, cx, 25.0, 50.0, 5.0, 18.0, &stone_mat);

    // Steeple
    draw_cylinder_3d(&mut img, &mut depth, cx, 5.0, 25.0, 0.0, 6.0, &stone_mat);
    draw_cone_3d(&mut img, &mut depth, cx, -15.0, 5.0, 0.0, 7.0, &roof_mat);

    // Cross
    draw_cylinder_3d(&mut img, &mut depth, cx, -22.0, -15.0, 0.0, 1.0, &gold_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, -19.0, -18.0, 0.0, 4.0, &gold_mat); // Horizontal bar via wide cylinder trick? No, better use ellipsoid
    draw_ellipsoid_3d(&mut img, &mut depth, cx, -19.0, 0.0, 4.0, 1.0, 1.0, &gold_mat);

    // Stained Glass Window
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 35.0, 22.0, 6.0, 8.0, 2.0, &glass_mat);

    add_noise(&mut img, 5);
    img
}

fn create_mage_tower() -> RgbaImage {
    let mut img = create_tile_base(Rgba([80, 80, 120, 255])); // Dark blue base
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let stone_mat = Material::matte(70, 70, 100);
    let roof_mat = Material::matte(40, 30, 80);
    let crystal_mat = Material::glowing(150, 100, 255);
    let aura_mat = Material::glowing(100, 200, 255);

    // Tall Tower
    draw_cylinder_3d(&mut img, &mut depth, cx, 10.0, 55.0, 5.0, 10.0, &stone_mat);
    
    // Spire Roof
    draw_cone_3d(&mut img, &mut depth, cx, -15.0, 10.0, 5.0, 12.0, &roof_mat);

    // Floating Crystals
    draw_sphere_3d(&mut img, &mut depth, cx - 12.0, 20.0, 15.0, 3.0, &crystal_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 12.0, 30.0, 10.0, 4.0, &crystal_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 5.0, 20.0, 3.0, &crystal_mat);

    // Arcane Aura at base
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 50.0, 5.0, 15.0, 3.0, 15.0, &aura_mat);

    add_noise(&mut img, 8);
    img
}

fn create_stable() -> RgbaImage {
    let mut img = create_tile_base(Rgba([160, 120, 80, 255])); // Dirth path base
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let wood_mat = Material::matte(120, 80, 40);
    let dark_wood_mat = Material::matte(90, 60, 30);
    let hay_mat = Material::matte(200, 180, 100);
    
    // Main Barn structure
    draw_cylinder_3d(&mut img, &mut depth, cx, 25.0, 50.0, 5.0, 20.0, &wood_mat);
    draw_cone_3d(&mut img, &mut depth, cx, 10.0, 25.0, 5.0, 22.0, &dark_wood_mat);

    // Open door with hay
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 20.0, 10.0, 10.0, 5.0, &hay_mat);

    // Trough
    draw_cylinder_3d(&mut img, &mut depth, cx - 15.0, 45.0, 50.0, 15.0, 3.0, &dark_wood_mat);

    add_noise(&mut img, 12);
    img
}

fn create_armory() -> RgbaImage {
    let mut img = create_tile_base(Rgba([120, 120, 120, 255])); // Grey stone base
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let stone_mat = Material::matte(100, 100, 110);
    let iron_mat = Material::metallic(80, 80, 90);
    let fire_mat = Material::glowing(255, 100, 0);

    // Heavy stone building
    draw_cylinder_3d(&mut img, &mut depth, cx, 20.0, 50.0, 5.0, 22.0, &stone_mat);
    
    // Iron banding
    draw_cylinder_3d(&mut img, &mut depth, cx, 25.0, 30.0, 4.0, 23.0, &iron_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 40.0, 45.0, 4.0, 23.0, &iron_mat);

    // Forge glow / Chimney
    draw_cylinder_3d(&mut img, &mut depth, cx + 15.0, 20.0, 50.0, 10.0, 4.0, &stone_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 15.0, 18.0, 10.0, 3.0, &fire_mat); // Smoke/Fire

    // Smoking top
    draw_sphere_3d(&mut img, &mut depth, cx + 16.0, 12.0, 12.0, 4.0, &Material::matte(200, 200, 200));

    add_noise(&mut img, 10);
    img
}

fn create_hero_wall() -> RgbaImage {
    let mut img = create_tile_base(Rgba([150, 150, 150, 255])); 
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let stone_mat = Material::matte(140, 140, 150);
    
    // Wall block
    draw_cylinder_3d(&mut img, &mut depth, cx, 15.0, 55.0, 0.0, 25.0, &stone_mat);

    // Crenellations
    draw_cylinder_3d(&mut img, &mut depth, cx - 15.0, 5.0, 15.0, 0.0, 6.0, &stone_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 5.0, 15.0, 0.0, 6.0, &stone_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 15.0, 5.0, 15.0, 0.0, 6.0, &stone_mat);

    add_noise(&mut img, 5);
    img
}

fn create_hero_gate() -> RgbaImage {
    let mut img = create_tile_base(Rgba([120, 100, 80, 255])); 
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let wood_mat = Material::matte(100, 70, 30);
    let iron_mat = Material::metallic(60, 60, 70);
    let torch_mat = Material::glowing(255, 180, 50);

    // Frame pillars
    draw_cylinder_3d(&mut img, &mut depth, cx - 20.0, 5.0, 55.0, 0.0, 6.0, &wood_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 20.0, 5.0, 55.0, 0.0, 6.0, &wood_mat);

    // Gate door
    draw_cylinder_3d(&mut img, &mut depth, cx, 10.0, 50.0, 5.0, 20.0, &wood_mat);

    // Iron reinforcement
    draw_cylinder_3d(&mut img, &mut depth, cx, 15.0, 20.0, 4.0, 21.0, &iron_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 40.0, 45.0, 4.0, 21.0, &iron_mat);

    // Torches
    draw_sphere_3d(&mut img, &mut depth, cx - 20.0, 15.0, -2.0, 3.0, &torch_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 20.0, 15.0, -2.0, 3.0, &torch_mat);

    add_noise(&mut img, 8);
    img
}
