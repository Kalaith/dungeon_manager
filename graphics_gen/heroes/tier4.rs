//! Tier 4 heroes - Commander Units, and Tier 5 heroes - Boss Units

use super::super::core::*;
use image::RgbaImage;

// ============================================================================
// TIER 4 HEROES - Commander Units
// ============================================================================

pub fn create_knight_commander_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let armor_mat = Material::metallic(200, 200, 220);
    let plume_mat = Material::matte(180, 0, 0);
    let cape_mat = Material::matte(150, 0, 0);
    let wood_mat = Material::matte(139, 90, 43);
    let banner_mat = Material::metallic(255, 215, 0);

    draw_shadow(&mut img, cx, 58.0, 13.0, 5.0);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 38.0, 7.0, 12.0, 16.0, 11.0, &armor_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 10.0, 10.0, &armor_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 4.0, 18.0, 9.0, 4.0, &plume_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 10.0,
        32.0,
        56.0,
        -2.0,
        8.0,
        &cape_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 16.0,
        6.0,
        52.0,
        5.0,
        2.0,
        &wood_mat,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 22.0,
        12.0,
        6.0,
        8.0,
        6.0,
        3.0,
        &banner_mat,
    );

    img
}

pub fn create_high_priest_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe_mat = Material::matte(240, 230, 200);
    let gold_mat = Material::metallic(255, 215, 0);
    let skin_mat = Material::matte(230, 190, 150);
    let orb_mat = Material::glowing(255, 255, 255);

    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 56.0, 5.0, 12.0, &robe_mat);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 16.0, 8.0, 12.0, 5.0, 6.0, &gold_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 9.0, 6.0, &skin_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 16.0,
        8.0,
        54.0,
        5.0,
        2.0,
        &gold_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 16.0, 6.0, 9.0, 7.0, &orb_mat);

    img
}

pub fn create_archmage_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe_mat = Material::matte(100, 50, 200);
    let hat_mat = Material::matte(80, 40, 180);
    let aura_mat = Material::glowing(150, 100, 255);
    let wood_mat = Material::matte(139, 90, 43);
    let energy_mat = Material::glowing(200, 150, 255);

    draw_sphere_3d(&mut img, &mut depth, cx, 36.0, -8.0, 20.0, &aura_mat);

    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 54.0, 5.0, 12.0, &robe_mat);
    draw_cone_3d(&mut img, &mut depth, cx, 4.0, 28.0, 6.0, 9.0, &hat_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 18.0,
        6.0,
        54.0,
        4.0,
        2.0,
        &wood_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 18.0, 4.0, 10.0, 9.0, &energy_mat);

    img
}

// ============================================================================
// TIER 5 HEROES - Boss Units
// ============================================================================

pub fn create_champion_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let armor_mat = Material::metallic(255, 215, 100);
    let divine_mat = Material::glowing(255, 255, 200);
    let sword_mat = Material::glowing(255, 255, 255);
    let shield_mat = Material::metallic(255, 240, 180);
    let gold_mat = Material::metallic(255, 215, 0);

    draw_sphere_3d(&mut img, &mut depth, cx, 32.0, -10.0, 24.0, &divine_mat);

    draw_shadow(&mut img, cx, 60.0, 14.0, 6.0);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 36.0, 8.0, 14.0, 18.0, 12.0, &armor_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 16.0, 11.0, 11.0, &armor_mat);
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx,
        6.0,
        10.0,
        10.0,
        2.0,
        8.0,
        &divine_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 20.0,
        6.0,
        52.0,
        10.0,
        3.0,
        &sword_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx + 20.0, 4.0, 12.0, 7.0, &divine_mat);
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 20.0,
        34.0,
        12.0,
        9.0,
        12.0,
        7.0,
        &shield_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 20.0,
        28.0,
        42.0,
        14.0,
        2.0,
        &gold_mat,
    );

    img
}

/// Dragon Knight boss - NEW
pub fn create_dragon_knight_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let scale_armor = Material::metallic(100, 150, 100);
    let fire_mat = Material::fire();
    let sword_mat = Material::metallic(200, 180, 100);
    let eye_mat = Material::glowing(255, 150, 0);

    draw_shadow(&mut img, cx, 60.0, 14.0, 6.0);
    // Massive scaled body
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx,
        38.0,
        8.0,
        14.0,
        18.0,
        12.0,
        &scale_armor,
    );
    // Dragon helm
    draw_sphere_3d(&mut img, &mut depth, cx, 18.0, 11.0, 10.0, &scale_armor);
    // Horns
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 8.0,
        6.0,
        16.0,
        10.0,
        3.0,
        &scale_armor,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx + 8.0,
        6.0,
        16.0,
        10.0,
        3.0,
        &scale_armor,
    );
    // Glowing eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 16.0, 16.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 16.0, 16.0, 2.0, &eye_mat);
    // Flame breath
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 18.0, 4.0, &fire_mat);
    // Massive sword
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 20.0,
        4.0,
        54.0,
        10.0,
        3.0,
        &sword_mat,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 20.0,
        2.0,
        12.0,
        6.0,
        4.0,
        3.0,
        &fire_mat,
    );

    img
}
