//! Building sprite generation for hero base structures
//!
//! Generates hero faction buildings that spawn heroes.

use super::core::*;
use image::RgbaImage;

// ============================================================================
// MAIN BUILDINGS
// ============================================================================

pub fn create_town_hall() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let stone_mat = Material::stone(180, 160, 140);
    let gold_mat = Material::metallic(218, 165, 32);
    let wood_mat = Material::wood(139, 90, 43);
    let banner_mat = Material::matte(180, 40, 40);
    let light_mat = Material::glowing(255, 200, 100);

    draw_shadow(&mut img, cx, 52.0, 22.0, 8.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 20.0, 50.0, 5.0, 20.0, &stone_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 10.0, 30.0, 0.0, 8.0, &stone_mat);
    draw_cone_3d(&mut img, &mut depth, cx, -5.0, 10.0, 0.0, 10.0, &gold_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 12.0,
        15.0,
        40.0,
        15.0,
        2.0,
        &wood_mat,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 16.0,
        20.0,
        15.0,
        5.0,
        8.0,
        2.0,
        &banner_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 8.0, 30.0, 24.0, 3.0, &light_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 8.0, 30.0, 24.0, 3.0, &light_mat);

    add_noise(&mut img, 8);
    img
}

pub fn create_barracks() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let wood_mat = Material::wood(120, 80, 40);
    let dark_wood_mat = Material::wood(80, 50, 20);
    let shield_mat = Material::metallic(160, 160, 180);
    let red_mat = Material::matte(150, 30, 30);

    draw_shadow(&mut img, cx, 52.0, 20.0, 8.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 20.0, 50.0, 5.0, 18.0, &wood_mat);
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx,
        5.0,
        20.0,
        5.0,
        22.0,
        &dark_wood_mat,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 12.0,
        40.0,
        22.0,
        4.0,
        6.0,
        2.0,
        &shield_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 12.0, 40.0, 24.0, 2.0, &red_mat);
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 12.0,
        40.0,
        22.0,
        4.0,
        6.0,
        2.0,
        &shield_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx + 12.0, 40.0, 24.0, 2.0, &red_mat);

    add_noise(&mut img, 10);
    img
}

pub fn create_archery_range() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let wood_mat = Material::wood(139, 90, 43);
    let target_white = Material::matte(220, 220, 220);
    let target_red = Material::matte(200, 20, 20);
    let hay_mat = Material::matte(200, 180, 100);

    draw_shadow(&mut img, cx, 54.0, 26.0, 8.0);
    for i in 0..5 {
        let x = 12.0 + i as f32 * 10.0;
        draw_cylinder_3d(&mut img, &mut depth, x, 40.0, 55.0, 20.0, 2.0, &wood_mat);
    }
    draw_cylinder_3d(&mut img, &mut depth, cx, 42.0, 44.0, 20.0, 25.0, &wood_mat);

    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 10.0,
        30.0,
        45.0,
        10.0,
        2.0,
        &wood_mat,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 10.0,
        25.0,
        8.0,
        5.0,
        5.0,
        2.0,
        &hay_mat,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 10.0,
        25.0,
        6.0,
        4.0,
        4.0,
        2.0,
        &target_white,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 10.0, 25.0, 5.0, 1.5, &target_red);

    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 15.0,
        35.0,
        50.0,
        5.0,
        2.0,
        &wood_mat,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 15.0,
        30.0,
        3.0,
        5.0,
        5.0,
        2.0,
        &hay_mat,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 15.0,
        30.0,
        2.0,
        4.0,
        4.0,
        2.0,
        &target_white,
    );
    draw_sphere_3d(&mut img, &mut depth, cx + 15.0, 30.0, 1.0, 1.5, &target_red);

    add_noise(&mut img, 15);
    img
}

pub fn create_church() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let stone_mat = Material::stone(220, 220, 230);
    let roof_mat = Material::stone(60, 60, 80);
    let gold_mat = Material::metallic(218, 165, 32);
    let glass_mat = Material::glowing(100, 150, 255);

    draw_shadow(&mut img, cx, 52.0, 20.0, 8.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 25.0, 50.0, 5.0, 18.0, &stone_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 5.0, 25.0, 0.0, 6.0, &stone_mat);
    draw_cone_3d(&mut img, &mut depth, cx, -15.0, 5.0, 0.0, 7.0, &roof_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, -22.0, -15.0, 0.0, 1.0, &gold_mat);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, -19.0, 0.0, 4.0, 1.0, 1.0, &gold_mat,
    );
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 35.0, 22.0, 6.0, 8.0, 2.0, &glass_mat,
    );

    add_noise(&mut img, 5);
    img
}

pub fn create_mage_tower() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let stone_mat = Material::stone(70, 70, 100);
    let roof_mat = Material::stone(40, 30, 80);
    let crystal_mat = Material::crystal(150, 100, 255);
    let aura_mat = Material::glowing(100, 200, 255);

    draw_shadow(&mut img, cx, 56.0, 14.0, 6.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 10.0, 55.0, 5.0, 10.0, &stone_mat);
    draw_cone_3d(&mut img, &mut depth, cx, -15.0, 10.0, 5.0, 12.0, &roof_mat);
    draw_sphere_3d(
        &mut img,
        &mut depth,
        cx - 12.0,
        20.0,
        15.0,
        3.0,
        &crystal_mat,
    );
    draw_sphere_3d(
        &mut img,
        &mut depth,
        cx + 12.0,
        30.0,
        10.0,
        4.0,
        &crystal_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 5.0, 20.0, 3.0, &crystal_mat);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 50.0, 5.0, 15.0, 3.0, 15.0, &aura_mat,
    );

    add_noise(&mut img, 8);
    img
}

pub fn create_stable() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let wood_mat = Material::wood(120, 80, 40);
    let dark_wood_mat = Material::wood(90, 60, 30);
    let hay_mat = Material::matte(200, 180, 100);

    draw_shadow(&mut img, cx, 52.0, 22.0, 8.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 25.0, 50.0, 5.0, 20.0, &wood_mat);
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx,
        10.0,
        25.0,
        5.0,
        22.0,
        &dark_wood_mat,
    );
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 40.0, 20.0, 10.0, 10.0, 5.0, &hay_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 15.0,
        45.0,
        50.0,
        15.0,
        3.0,
        &dark_wood_mat,
    );

    add_noise(&mut img, 12);
    img
}

pub fn create_armory() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let stone_mat = Material::stone(100, 100, 110);
    let iron_mat = Material::metallic(80, 80, 90);
    let fire_mat = Material::fire();

    draw_shadow(&mut img, cx, 52.0, 24.0, 8.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 20.0, 50.0, 5.0, 22.0, &stone_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 25.0, 30.0, 4.0, 23.0, &iron_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 40.0, 45.0, 4.0, 23.0, &iron_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 15.0,
        20.0,
        50.0,
        10.0,
        4.0,
        &stone_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx + 15.0, 18.0, 10.0, 3.0, &fire_mat);
    draw_sphere_3d(
        &mut img,
        &mut depth,
        cx + 16.0,
        12.0,
        12.0,
        4.0,
        &Material::matte(200, 200, 200),
    );

    add_noise(&mut img, 10);
    img
}

// ============================================================================
// DEFENSIVE BUILDINGS
// ============================================================================

pub fn create_hero_wall() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let stone_mat = Material::stone(140, 140, 150);

    draw_shadow(&mut img, cx, 56.0, 26.0, 6.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 15.0, 55.0, 0.0, 25.0, &stone_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 15.0,
        5.0,
        15.0,
        0.0,
        6.0,
        &stone_mat,
    );
    draw_cylinder_3d(&mut img, &mut depth, cx, 5.0, 15.0, 0.0, 6.0, &stone_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 15.0,
        5.0,
        15.0,
        0.0,
        6.0,
        &stone_mat,
    );

    add_noise(&mut img, 5);
    img
}

pub fn create_hero_gate() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let wood_mat = Material::wood(100, 70, 30);
    let iron_mat = Material::metallic(60, 60, 70);
    let torch_mat = Material::glowing(255, 180, 50);

    draw_shadow(&mut img, cx, 56.0, 24.0, 8.0);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 20.0,
        5.0,
        55.0,
        0.0,
        6.0,
        &wood_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 20.0,
        5.0,
        55.0,
        0.0,
        6.0,
        &wood_mat,
    );
    draw_cylinder_3d(&mut img, &mut depth, cx, 10.0, 50.0, 5.0, 20.0, &wood_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 15.0, 20.0, 4.0, 21.0, &iron_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 40.0, 45.0, 4.0, 21.0, &iron_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 20.0, 15.0, -2.0, 3.0, &torch_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 20.0, 15.0, -2.0, 3.0, &torch_mat);

    add_noise(&mut img, 8);
    img
}

/// Guard tower - NEW
pub fn create_guard_tower() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let stone_mat = Material::stone(140, 130, 120);
    let wood_mat = Material::wood(100, 70, 40);
    let torch_mat = Material::glowing(255, 180, 50);

    draw_shadow(&mut img, cx, 56.0, 16.0, 6.0);
    // Tower base
    draw_cylinder_3d(&mut img, &mut depth, cx, 30.0, 55.0, 5.0, 14.0, &stone_mat);
    // Tower body
    draw_cylinder_3d(&mut img, &mut depth, cx, 10.0, 30.0, 5.0, 10.0, &stone_mat);
    // Roof platform
    draw_cylinder_3d(&mut img, &mut depth, cx, 5.0, 10.0, 5.0, 14.0, &wood_mat);
    // Crenellations
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 10.0,
        0.0,
        8.0,
        5.0,
        4.0,
        &stone_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 10.0,
        0.0,
        8.0,
        5.0,
        4.0,
        &stone_mat,
    );
    // Torches
    draw_sphere_3d(&mut img, &mut depth, cx - 12.0, 20.0, 16.0, 2.0, &torch_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 12.0, 20.0, 16.0, 2.0, &torch_mat);

    add_noise(&mut img, 6);
    img
}

/// Blacksmith forge - NEW
pub fn create_blacksmith() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let stone_mat = Material::stone(100, 90, 80);
    let metal_mat = Material::metallic(140, 140, 150);
    let fire_mat = Material::fire();
    let smoke_mat = Material::matte(180, 180, 180);

    draw_shadow(&mut img, cx, 56.0, 20.0, 8.0);
    // Main structure
    draw_cylinder_3d(&mut img, &mut depth, cx, 25.0, 55.0, 5.0, 18.0, &stone_mat);
    // Forge opening
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx,
        38.0,
        20.0,
        8.0,
        8.0,
        4.0,
        &Material::shadow(),
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 40.0, 18.0, 4.0, &fire_mat);
    // Chimney
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 10.0,
        10.0,
        30.0,
        5.0,
        5.0,
        &stone_mat,
    );
    // Smoke
    draw_sphere_3d(&mut img, &mut depth, cx + 10.0, 5.0, 8.0, 4.0, &smoke_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 12.0, 0.0, 10.0, 3.0, &smoke_mat);
    // Anvil outside
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 14.0,
        50.0,
        15.0,
        4.0,
        3.0,
        3.0,
        &metal_mat,
    );

    add_noise(&mut img, 8);
    img
}

/// Inn/Tavern - NEW
pub fn create_tavern() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let wood_mat = Material::wood(130, 100, 70);
    let roof_mat = Material::matte(60, 50, 40);
    let light_mat = Material::glowing(255, 220, 150);
    let sign_mat = Material::wood(100, 70, 40);

    draw_shadow(&mut img, cx, 56.0, 22.0, 8.0);
    // Main building
    draw_cylinder_3d(&mut img, &mut depth, cx, 25.0, 55.0, 5.0, 20.0, &wood_mat);
    // Roof
    draw_cone_3d(&mut img, &mut depth, cx, 8.0, 25.0, 5.0, 24.0, &roof_mat);
    // Windows with warm light
    draw_sphere_3d(&mut img, &mut depth, cx - 8.0, 35.0, 24.0, 4.0, &light_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 8.0, 35.0, 24.0, 4.0, &light_mat);
    // Sign
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 18.0,
        30.0,
        35.0,
        20.0,
        1.0,
        &sign_mat,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 22.0,
        28.0,
        20.0,
        5.0,
        4.0,
        1.0,
        &sign_mat,
    );

    add_noise(&mut img, 10);
    img
}

// ============================================================================
// SAVE FUNCTION
// ============================================================================

pub fn save_hero_building(name: &str, img: RgbaImage) {
    let path = format!("assets/tiles/hero_buildings/{}.png", name);
    let flipped = image::imageops::flip_vertical(&img);
    flipped.save(&path).unwrap();
    println!("Generated: {}", path);
}
