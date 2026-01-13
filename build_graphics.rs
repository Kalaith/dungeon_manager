//! Procedural graphics generator for Deep Dominion
//! Generates consistent isometric tiles, rooms, and sprites

use image::{ImageBuffer, Rgba, RgbaImage};
use std::path::Path;

const TILE_SIZE: u32 = 64;
const TILE_WIDTH: u32 = 64;
const TILE_HEIGHT: u32 = 32;
const SPRITE_SIZE: u32 = 64;

fn main() {
    println!("Generating game graphics...");

    // Create assets directory
    std::fs::create_dir_all("assets/tiles").unwrap();
    std::fs::create_dir_all("assets/sprites/monsters").unwrap();
    std::fs::create_dir_all("assets/sprites/heroes").unwrap();

    // Generate all tiles
    generate_tiles();

    // Generate all sprites
    generate_monster_sprites();
    generate_hero_sprites();

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
}

fn save_tile(name: &str, img: RgbaImage) {
    let path = format!("assets/tiles/{}.png", name);
    img.save(&path).unwrap();
    println!("Generated: {}", path);
}

fn save_sprite(category: &str, name: &str, img: RgbaImage) {
    let path = format!("assets/sprites/{}/{}.png", category, name);
    img.save(&path).unwrap();
    println!("Generated: {}", path);
}

// Create isometric diamond base
fn create_iso_tile_base(color: Rgba<u8>) -> RgbaImage {
    let mut img = RgbaImage::new(TILE_WIDTH, TILE_HEIGHT);

    // Draw isometric diamond
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let center_x = TILE_WIDTH / 2;
            let fx = x as f32;
            let fy = y as f32;

            // Isometric diamond check
            let left = (center_x as f32 - fx) + fy * 2.0;
            let right = (fx - center_x as f32) + fy * 2.0;

            if left >= 0.0 && right >= 0.0 && fy < TILE_HEIGHT as f32 {
                img.put_pixel(x, y, color);
            }
        }
    }

    img
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
    let mut img = create_iso_tile_base(Rgba([60, 60, 65, 255]));
    // Add some texture
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 && (x + y) % 7 == 0 {
                img.put_pixel(x, y, Rgba([50, 50, 55, 255]));
            }
        }
    }
    add_outline(&mut img, Rgba([40, 40, 45, 255]));
    img
}

fn create_earth() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([101, 67, 33, 255])); // Brown
    // Add dirt texture
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 && (x * 3 + y * 5) % 11 < 4 {
                img.put_pixel(x, y, Rgba([91, 60, 28, 255]));
            }
        }
    }
    add_outline(&mut img, Rgba([70, 45, 20, 255]));
    img
}

fn create_claimed_floor() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([45, 45, 50, 255])); // Dark gray
    // Add grid pattern
    for y in (0..TILE_HEIGHT).step_by(8) {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 {
                img.put_pixel(x, y, Rgba([35, 35, 40, 255]));
            }
        }
    }
    add_outline(&mut img, Rgba([30, 30, 35, 255]));
    img
}

fn create_reinforced_wall() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([70, 70, 75, 255]));
    // Add metal bands
    for y in (4..TILE_HEIGHT).step_by(10) {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 {
                img.put_pixel(x, y, Rgba([120, 120, 130, 255]));
            }
        }
    }
    add_outline(&mut img, Rgba([50, 50, 55, 255]));
    img
}

fn create_gold_vein() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([80, 70, 50, 255])); // Dark rock
    // Add gold streaks
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 {
                if (x + y * 2) % 13 < 3 {
                    img.put_pixel(x, y, Rgba([255, 215, 0, 255])); // Gold
                }
            }
        }
    }
    add_outline(&mut img, Rgba([60, 50, 30, 255]));
    img
}

fn create_gem_seam() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([70, 70, 80, 255]));
    // Add colorful gems
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 {
                let val = (x * 7 + y * 11) % 19;
                if val < 2 {
                    img.put_pixel(x, y, Rgba([100, 100, 255, 255])); // Blue gem
                } else if val < 4 {
                    img.put_pixel(x, y, Rgba([200, 100, 200, 255])); // Purple gem
                }
            }
        }
    }
    add_outline(&mut img, Rgba([50, 50, 60, 255]));
    img
}

fn create_mana_crystal() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([40, 40, 60, 255])); // Dark blue-gray rock
    // Add glowing blue crystals
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;
    
    // Draw crystal cluster
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 {
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
    }
    
    // Add highlights
    img.put_pixel(center_x, center_y - 8, Rgba([255, 255, 255, 255]));
    img.put_pixel(center_x - 5, center_y, Rgba([200, 200, 255, 255]));
    img.put_pixel(center_x + 5, center_y, Rgba([200, 200, 255, 255]));

    add_outline(&mut img, Rgba([30, 30, 50, 255]));
    img
}

fn create_lava() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([255, 69, 0, 255])); // Orange-red
    // Add lava bubbles
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 {
                if (x * 5 + y * 7) % 17 < 5 {
                    img.put_pixel(x, y, Rgba([255, 140, 0, 255])); // Brighter orange
                }
            }
        }
    }
    add_outline(&mut img, Rgba([200, 50, 0, 255]));
    img
}

fn create_water() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([30, 60, 120, 255])); // Dark blue
    // Add water ripples
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 {
                if (x + y) % 9 < 2 {
                    img.put_pixel(x, y, Rgba([50, 80, 140, 255]));
                }
            }
        }
    }
    add_outline(&mut img, Rgba([20, 40, 80, 255]));
    img
}

fn create_bridge() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([100, 90, 80, 255])); // Stone
    // Add planks
    for x in (0..TILE_WIDTH).step_by(6) {
        for y in 0..TILE_HEIGHT {
            if img.get_pixel(x, y)[3] > 0 {
                img.put_pixel(x, y, Rgba([80, 70, 60, 255]));
            }
        }
    }
    add_outline(&mut img, Rgba([70, 60, 50, 255]));
    img
}

fn create_corrupted_floor() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([60, 30, 80, 255])); // Purple-dark
    // Add corruption effect
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 {
                if (x * 3 + y * 5) % 13 < 4 {
                    img.put_pixel(x, y, Rgba([80, 40, 100, 255]));
                }
            }
        }
    }
    add_outline(&mut img, Rgba([40, 20, 60, 255]));
    img
}

fn create_ancient_rune_floor() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([50, 50, 60, 255]));
    // Add glowing runes
    for y in (4..TILE_HEIGHT).step_by(8) {
        for x in (4..TILE_WIDTH).step_by(8) {
            if img.get_pixel(x, y)[3] > 0 {
                img.put_pixel(x, y, Rgba([100, 150, 255, 255])); // Blue glow
            }
        }
    }
    add_outline(&mut img, Rgba([40, 40, 50, 255]));
    img
}

// Room tiles
fn create_dungeon_heart() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([139, 0, 0, 255])); // Dark red
    // Add heart pattern
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 {
                let dx = (x as i32 - center_x as i32).abs();
                let dy = (y as i32 - center_y as i32).abs();
                if dx + dy < 8 {
                    img.put_pixel(x, y, Rgba([255, 0, 0, 255])); // Bright red center
                }
            }
        }
    }
    add_outline(&mut img, Rgba([100, 0, 0, 255]));
    img
}

fn create_lair() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([70, 50, 40, 255])); // Brown
    // Add bedding pattern
    for y in (8..TILE_HEIGHT).step_by(4) {
        for x in 8..TILE_WIDTH-8 {
            if img.get_pixel(x, y)[3] > 0 {
                img.put_pixel(x, y, Rgba([90, 70, 50, 255]));
            }
        }
    }
    add_outline(&mut img, Rgba([50, 35, 25, 255]));
    img
}

fn create_hatchery() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([85, 70, 50, 255])); // Brown-tan
    // Add hay/straw texture
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 && (x * 7 + y * 3) % 11 < 3 {
                img.put_pixel(x, y, Rgba([200, 180, 80, 255])); // Yellow straw
            }
        }
    }
    add_outline(&mut img, Rgba([65, 50, 35, 255]));
    img
}

fn create_treasury() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([218, 165, 32, 255])); // Goldenrod
    // Add coin pattern
    for y in (0..TILE_HEIGHT).step_by(6) {
        for x in (0..TILE_WIDTH).step_by(8) {
            if img.get_pixel(x, y)[3] > 0 {
                img.put_pixel(x, y, Rgba([255, 215, 0, 255])); // Bright gold
            }
        }
    }
    add_outline(&mut img, Rgba([184, 134, 11, 255]));
    img
}

fn create_workshop() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([60, 60, 65, 255])); // Dark gray
    // Add forge glow
    let center_y = TILE_HEIGHT / 2;
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 && y > center_y {
                img.put_pixel(x, y, Rgba([255, 100, 0, 255])); // Orange forge glow
            }
        }
    }
    add_outline(&mut img, Rgba([40, 40, 45, 255]));
    img
}

fn create_training_room() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([80, 60, 50, 255])); // Brown-gray
    // Add weapon rack pattern
    for x in (8..TILE_WIDTH-8).step_by(12) {
        for y in 4..TILE_HEIGHT-4 {
            if img.get_pixel(x, y)[3] > 0 {
                img.put_pixel(x, y, Rgba([150, 150, 160, 255])); // Metal
            }
        }
    }
    add_outline(&mut img, Rgba([60, 45, 35, 255]));
    img
}

fn create_library() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([70, 50, 80, 255])); // Purple-brown
    // Add book shelf pattern
    for y in (0..TILE_HEIGHT).step_by(8) {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 && x % 6 < 4 {
                img.put_pixel(x, y, Rgba([139, 69, 19, 255])); // Saddle brown
            }
        }
    }
    add_outline(&mut img, Rgba([50, 35, 60, 255]));
    img
}

fn create_prison() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([50, 50, 55, 255])); // Dark gray
    // Add bars
    for x in (8..TILE_WIDTH-8).step_by(6) {
        for y in 0..TILE_HEIGHT {
            if img.get_pixel(x, y)[3] > 0 {
                img.put_pixel(x, y, Rgba([100, 100, 110, 255])); // Gray bars
            }
        }
    }
    add_outline(&mut img, Rgba([30, 30, 35, 255]));
    img
}

fn create_guard_post() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([80, 70, 60, 255])); // Stone
    // Add watchtower elements
    let center_x = TILE_WIDTH / 2;
    for y in 0..TILE_HEIGHT/2 {
        for x in center_x-4..center_x+4 {
            if img.get_pixel(x, y)[3] > 0 {
                img.put_pixel(x, y, Rgba([120, 110, 100, 255]));
            }
        }
    }
    add_outline(&mut img, Rgba([60, 50, 40, 255]));
    img
}

fn create_ritual_circle() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([40, 20, 50, 255])); // Dark purple
    // Add pentagram/circle
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 {
                let dx = (x as i32 - center_x as i32).abs();
                let dy = (y as i32 - center_y as i32).abs();
                if dx + dy > 6 && dx + dy < 10 {
                    img.put_pixel(x, y, Rgba([200, 50, 50, 255])); // Red circle
                }
            }
        }
    }
    add_outline(&mut img, Rgba([30, 15, 40, 255]));
    img
}

fn create_monster_spawner() -> RgbaImage {
    let mut img = create_iso_tile_base(Rgba([80, 20, 100, 255])); // Purple
    // Add portal swirl
    let center_x = TILE_WIDTH / 2;
    let center_y = TILE_HEIGHT / 2;
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            if img.get_pixel(x, y)[3] > 0 {
                let dx = x as i32 - center_x as i32;
                let dy = y as i32 - center_y as i32;
                if (dx * dx + dy * dy * 2) < 100 {
                    img.put_pixel(x, y, Rgba([150, 50, 200, 255])); // Bright purple
                }
            }
        }
    }
    add_outline(&mut img, Rgba([60, 15, 80, 255]));
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

// Monster sprites - each with distinctive shape
fn create_imp_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Small red demon body
    draw_circle(&mut img, center, center + 4, 10, Rgba([200, 50, 50, 255]));
    // Head
    draw_circle(&mut img, center, center - 6, 8, Rgba([220, 60, 60, 255]));
    // Horns
    draw_rect(&mut img, center - 10, center - 12, 3, 8, Rgba([100, 20, 20, 255]));
    draw_rect(&mut img, center + 7, center - 12, 3, 8, Rgba([100, 20, 20, 255]));
    // Eyes (yellow)
    img.put_pixel(center - 3, center - 8, Rgba([255, 255, 0, 255]));
    img.put_pixel(center + 3, center - 8, Rgba([255, 255, 0, 255]));
    // Tail
    draw_rect(&mut img, center - 2, center + 10, 4, 8, Rgba([180, 40, 40, 255]));

    img
}

fn create_goblin_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Green hunched body
    draw_circle(&mut img, center, center + 6, 12, Rgba([50, 150, 50, 255]));
    // Large head
    draw_circle(&mut img, center, center - 4, 10, Rgba([60, 160, 60, 255]));
    // Pointed ears
    draw_rect(&mut img, center - 14, center - 6, 4, 8, Rgba([70, 170, 70, 255]));
    draw_rect(&mut img, center + 10, center - 6, 4, 8, Rgba([70, 170, 70, 255]));
    // Eyes
    img.put_pixel(center - 4, center - 6, Rgba([255, 0, 0, 255]));
    img.put_pixel(center + 4, center - 6, Rgba([255, 0, 0, 255]));
    // Crude weapon
    draw_rect(&mut img, center + 12, center - 8, 3, 16, Rgba([139, 90, 43, 255]));

    img
}

fn create_orc_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Large muscular green body
    draw_circle(&mut img, center, center + 4, 14, Rgba([100, 180, 80, 255]));
    // Head
    draw_circle(&mut img, center, center - 8, 10, Rgba([110, 190, 90, 255]));
    // Tusks
    draw_rect(&mut img, center - 6, center - 4, 2, 6, Rgba([240, 240, 230, 255]));
    draw_rect(&mut img, center + 4, center - 4, 2, 6, Rgba([240, 240, 230, 255]));
    // Eyes
    img.put_pixel(center - 3, center - 10, Rgba([255, 0, 0, 255]));
    img.put_pixel(center + 3, center - 10, Rgba([255, 0, 0, 255]));
    // Battle axe
    draw_rect(&mut img, center + 14, center - 12, 8, 3, Rgba([169, 169, 169, 255]));
    draw_rect(&mut img, center + 17, center - 10, 2, 16, Rgba([139, 90, 43, 255]));

    img
}

fn create_warlock_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Purple robed figure
    draw_circle(&mut img, center, center + 8, 12, Rgba([80, 40, 120, 255]));
    // Hood/head
    draw_circle(&mut img, center, center - 6, 9, Rgba([60, 30, 90, 255]));
    // Glowing eyes
    img.put_pixel(center - 3, center - 8, Rgba([150, 255, 150, 255]));
    img.put_pixel(center + 3, center - 8, Rgba([150, 255, 150, 255]));
    // Staff with glowing crystal
    draw_rect(&mut img, center - 12, center - 16, 2, 24, Rgba([101, 67, 33, 255]));
    draw_circle(&mut img, center - 11, center - 16, 4, Rgba([200, 100, 255, 255]));

    img
}

fn create_troll_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Very large hunched body
    draw_circle(&mut img, center, center + 6, 16, Rgba([120, 140, 100, 255]));
    // Large head
    draw_circle(&mut img, center, center - 6, 12, Rgba([130, 150, 110, 255]));
    // Eyes
    img.put_pixel(center - 5, center - 8, Rgba([255, 255, 0, 255]));
    img.put_pixel(center + 5, center - 8, Rgba([255, 255, 0, 255]));
    // Huge club
    draw_circle(&mut img, center + 18, center - 8, 6, Rgba([101, 67, 33, 255]));
    draw_rect(&mut img, center + 16, center - 2, 4, 12, Rgba([101, 67, 33, 255]));

    img
}

fn create_skeleton_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Skull
    draw_circle(&mut img, center, center - 6, 8, Rgba([240, 240, 240, 255]));
    // Eye sockets (black)
    draw_circle(&mut img, center - 3, center - 8, 2, Rgba([0, 0, 0, 255]));
    draw_circle(&mut img, center + 3, center - 8, 2, Rgba([0, 0, 0, 255]));
    // Ribcage
    draw_rect(&mut img, center - 8, center + 2, 16, 2, Rgba([220, 220, 220, 255]));
    draw_rect(&mut img, center - 8, center + 6, 16, 2, Rgba([220, 220, 220, 255]));
    draw_rect(&mut img, center - 8, center + 10, 16, 2, Rgba([220, 220, 220, 255]));
    // Spine
    draw_rect(&mut img, center - 1, center + 2, 2, 12, Rgba([230, 230, 230, 255]));
    // Sword
    draw_rect(&mut img, center + 12, center - 12, 2, 20, Rgba([192, 192, 192, 255]));

    img
}

fn create_demon_spawn_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Dark red demon body
    draw_circle(&mut img, center, center + 2, 14, Rgba([180, 20, 20, 255]));
    // Horned head
    draw_circle(&mut img, center, center - 10, 10, Rgba([200, 30, 30, 255]));
    // Large horns
    draw_rect(&mut img, center - 12, center - 18, 4, 12, Rgba([80, 0, 0, 255]));
    draw_rect(&mut img, center + 8, center - 18, 4, 12, Rgba([80, 0, 0, 255]));
    // Glowing eyes
    img.put_pixel(center - 3, center - 12, Rgba([255, 100, 0, 255]));
    img.put_pixel(center + 3, center - 12, Rgba([255, 100, 0, 255]));
    // Claws
    draw_rect(&mut img, center - 16, center + 4, 4, 6, Rgba([100, 10, 10, 255]));
    draw_rect(&mut img, center + 12, center + 4, 4, 6, Rgba([100, 10, 10, 255]));
    // Fire effect at feet
    draw_circle(&mut img, center - 8, center + 14, 3, Rgba([255, 140, 0, 255]));
    draw_circle(&mut img, center + 8, center + 14, 3, Rgba([255, 140, 0, 255]));

    img
}

// Hero sprites - distinctive shapes for each class
fn create_peasant_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Simple cloth body (brown)
    draw_circle(&mut img, center, center + 6, 10, Rgba([160, 140, 100, 255]));
    // Head (skin tone)
    draw_circle(&mut img, center, center - 6, 7, Rgba([230, 190, 150, 255]));
    // Eyes
    img.put_pixel(center - 2, center - 8, Rgba([0, 0, 0, 255]));
    img.put_pixel(center + 2, center - 8, Rgba([0, 0, 0, 255]));
    // Pitchfork
    draw_rect(&mut img, center + 10, center - 16, 2, 22, Rgba([139, 90, 43, 255]));
    draw_rect(&mut img, center + 6, center - 16, 10, 2, Rgba([169, 169, 169, 255]));

    img
}

fn create_scout_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Lean leather-clad body
    draw_circle(&mut img, center, center + 4, 10, Rgba([100, 120, 80, 255]));
    // Head
    draw_circle(&mut img, center, center - 8, 7, Rgba([230, 190, 150, 255]));
    // Eyes
    img.put_pixel(center - 2, center - 10, Rgba([0, 100, 0, 255]));
    img.put_pixel(center + 2, center - 10, Rgba([0, 100, 0, 255]));
    // Bow
    draw_rect(&mut img, center - 14, center - 8, 2, 16, Rgba([139, 90, 43, 255]));

    img
}

fn create_acolyte_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // White and gold robes
    draw_circle(&mut img, center, center + 6, 11, Rgba([240, 240, 250, 255]));
    // Head
    draw_circle(&mut img, center, center - 6, 7, Rgba([230, 190, 150, 255]));
    // Eyes
    img.put_pixel(center - 2, center - 8, Rgba([100, 100, 200, 255]));
    img.put_pixel(center + 2, center - 8, Rgba([100, 100, 200, 255]));
    // Holy symbol (golden cross)
    draw_rect(&mut img, center - 1, center + 2, 2, 10, Rgba([255, 215, 0, 255]));
    draw_rect(&mut img, center - 4, center + 5, 8, 2, Rgba([255, 215, 0, 255]));

    img
}

fn create_knight_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Silver armor body
    draw_circle(&mut img, center, center + 4, 12, Rgba([180, 180, 200, 255]));
    // Helmet
    draw_circle(&mut img, center, center - 8, 8, Rgba([160, 160, 180, 255]));
    // Helmet plume
    draw_rect(&mut img, center - 2, center - 16, 4, 8, Rgba([200, 0, 0, 255]));
    // Shield
    draw_circle(&mut img, center - 14, center + 2, 6, Rgba([100, 100, 120, 255]));
    // Sword
    draw_rect(&mut img, center + 12, center - 12, 2, 20, Rgba([220, 220, 220, 255]));

    img
}

fn create_archer_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Leather armor
    draw_circle(&mut img, center, center + 4, 10, Rgba([120, 100, 70, 255]));
    // Head
    draw_circle(&mut img, center, center - 8, 7, Rgba([230, 190, 150, 255]));
    // Eyes
    img.put_pixel(center - 2, center - 10, Rgba([0, 0, 0, 255]));
    img.put_pixel(center + 2, center - 10, Rgba([0, 0, 0, 255]));
    // Longbow
    draw_rect(&mut img, center - 16, center - 12, 2, 20, Rgba([139, 90, 43, 255]));
    // Arrow
    draw_rect(&mut img, center - 15, center - 4, 12, 1, Rgba([139, 90, 43, 255]));
    draw_rect(&mut img, center - 3, center - 5, 3, 3, Rgba([169, 169, 169, 255]));

    img
}

fn create_battle_cleric_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Chainmail + robes
    draw_circle(&mut img, center, center + 4, 12, Rgba([200, 200, 220, 255]));
    // Head
    draw_circle(&mut img, center, center - 8, 7, Rgba([230, 190, 150, 255]));
    // Holy symbol on chest
    draw_rect(&mut img, center - 1, center + 2, 2, 8, Rgba([255, 215, 0, 255]));
    draw_rect(&mut img, center - 4, center + 4, 8, 2, Rgba([255, 215, 0, 255]));
    // Mace
    draw_circle(&mut img, center + 16, center - 4, 4, Rgba([150, 150, 150, 255]));
    draw_rect(&mut img, center + 14, center, 4, 12, Rgba([139, 90, 43, 255]));

    img
}

fn create_rogue_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Dark leather body
    draw_circle(&mut img, center, center + 4, 10, Rgba([60, 60, 80, 255]));
    // Hooded head
    draw_circle(&mut img, center, center - 6, 8, Rgba([50, 50, 70, 255]));
    // Just eyes visible
    img.put_pixel(center - 2, center - 8, Rgba([255, 255, 0, 255]));
    img.put_pixel(center + 2, center - 8, Rgba([255, 255, 0, 255]));
    // Daggers
    draw_rect(&mut img, center - 12, center + 2, 1, 10, Rgba([192, 192, 192, 255]));
    draw_rect(&mut img, center + 12, center + 2, 1, 10, Rgba([192, 192, 192, 255]));

    img
}

fn create_paladin_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Golden glowing armor
    draw_circle(&mut img, center, center + 2, 14, Rgba([220, 200, 100, 255]));
    // Helmet with divine glow
    draw_circle(&mut img, center, center - 10, 9, Rgba([255, 230, 150, 255]));
    // Holy aura
    draw_circle(&mut img, center, center - 4, 20, Rgba([255, 255, 200, 80]));
    // Holy sword (glowing)
    draw_rect(&mut img, center + 14, center - 14, 3, 24, Rgba([255, 255, 255, 255]));
    // Shield with cross
    draw_circle(&mut img, center - 16, center + 2, 7, Rgba([200, 200, 220, 255]));
    draw_rect(&mut img, center - 17, center, 2, 6, Rgba([255, 215, 0, 255]));
    draw_rect(&mut img, center - 19, center + 2, 6, 2, Rgba([255, 215, 0, 255]));

    img
}

fn create_wizard_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Blue robes
    draw_circle(&mut img, center, center + 6, 11, Rgba([50, 80, 200, 255]));
    // Pointed wizard hat
    draw_rect(&mut img, center - 6, center - 14, 12, 8, Rgba([40, 70, 180, 255]));
    draw_rect(&mut img, center - 3, center - 22, 6, 8, Rgba([40, 70, 180, 255]));
    // Face
    draw_circle(&mut img, center, center - 6, 6, Rgba([230, 190, 150, 255]));
    // Stars on robe
    img.put_pixel(center - 4, center + 4, Rgba([255, 255, 0, 255]));
    img.put_pixel(center + 4, center + 8, Rgba([255, 255, 0, 255]));
    // Glowing staff
    draw_rect(&mut img, center - 14, center - 18, 2, 28, Rgba([139, 90, 43, 255]));
    draw_circle(&mut img, center - 13, center - 18, 5, Rgba([100, 200, 255, 255]));

    img
}

fn create_inquisitor_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Dark red and black robes
    draw_circle(&mut img, center, center + 6, 12, Rgba([140, 30, 30, 255]));
    // Grim hood
    draw_circle(&mut img, center, center - 6, 8, Rgba([60, 10, 10, 255]));
    // Eyes
    img.put_pixel(center - 3, center - 8, Rgba([255, 0, 0, 255]));
    img.put_pixel(center + 3, center - 8, Rgba([255, 0, 0, 255]));
    // Flaming sword
    draw_rect(&mut img, center + 12, center - 12, 3, 20, Rgba([220, 220, 220, 255]));
    draw_circle(&mut img, center + 13, center - 14, 4, Rgba([255, 100, 0, 255]));

    img
}

fn create_knight_commander_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Ornate armor
    draw_circle(&mut img, center, center + 2, 14, Rgba([200, 200, 220, 255]));
    // Helmet with plume
    draw_circle(&mut img, center, center - 10, 9, Rgba([180, 180, 200, 255]));
    draw_rect(&mut img, center - 3, center - 20, 6, 10, Rgba([180, 0, 0, 255]));
    // Cape
    draw_rect(&mut img, center - 16, center, 8, 16, Rgba([150, 0, 0, 255]));
    // Banner
    draw_rect(&mut img, center + 12, center - 20, 2, 28, Rgba([139, 90, 43, 255]));
    draw_rect(&mut img, center + 14, center - 20, 10, 8, Rgba([255, 215, 0, 255]));

    img
}

fn create_high_priest_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Elaborate white and gold robes
    draw_circle(&mut img, center, center + 6, 13, Rgba([240, 230, 200, 255]));
    // Ornate headdress
    draw_circle(&mut img, center, center - 8, 8, Rgba([255, 215, 0, 255]));
    draw_rect(&mut img, center - 10, center - 16, 20, 4, Rgba([255, 215, 0, 255]));
    // Face
    draw_circle(&mut img, center, center - 6, 6, Rgba([230, 190, 150, 255]));
    // Ornate staff
    draw_rect(&mut img, center - 16, center - 20, 3, 32, Rgba([255, 215, 0, 255]));
    draw_circle(&mut img, center - 15, center - 20, 6, Rgba([255, 255, 255, 255]));

    img
}

fn create_archmage_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Purple robes crackling with energy
    draw_circle(&mut img, center, center + 4, 14, Rgba([100, 50, 200, 255]));
    // Floating effect (aura)
    draw_circle(&mut img, center, center, 22, Rgba([150, 100, 255, 60]));
    // Wizard hat
    draw_rect(&mut img, center - 8, center - 16, 16, 10, Rgba([80, 40, 180, 255]));
    draw_rect(&mut img, center - 4, center - 26, 8, 10, Rgba([80, 40, 180, 255]));
    // Glowing staff with massive energy
    draw_rect(&mut img, center - 18, center - 22, 3, 34, Rgba([139, 90, 43, 255]));
    draw_circle(&mut img, center - 17, center - 22, 8, Rgba([200, 150, 255, 255]));

    img
}

fn create_champion_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let center = SPRITE_SIZE / 2;

    // Radiant golden armor
    draw_circle(&mut img, center, center, 16, Rgba([255, 215, 100, 255]));
    // Divine aura
    draw_circle(&mut img, center, center - 4, 26, Rgba([255, 255, 200, 100]));
    // Helmet
    draw_circle(&mut img, center, center - 12, 10, Rgba([255, 230, 150, 255]));
    // Divine greatsword (glowing bright)
    draw_rect(&mut img, center + 16, center - 18, 4, 32, Rgba([255, 255, 255, 255]));
    draw_circle(&mut img, center + 18, center - 20, 6, Rgba([255, 255, 200, 255]));
    // Shield with holy symbol
    draw_circle(&mut img, center - 18, center, 8, Rgba([255, 240, 180, 255]));
    draw_rect(&mut img, center - 19, center - 2, 2, 8, Rgba([255, 215, 0, 255]));
    draw_rect(&mut img, center - 22, center + 1, 8, 2, Rgba([255, 215, 0, 255]));

    img
}
