//! Tier 3 heroes - Elite Units

use super::super::core::*;
use image::RgbaImage;

pub fn create_paladin_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let armor_mat = Material::metallic(220, 200, 100);
    let glow_mat = Material::glowing(255, 255, 200);
    let sword_mat = Material::glowing(255, 255, 255);
    let shield_mat = Material::metallic(200, 200, 220);
    let gold_mat = Material::metallic(255, 215, 0);

    draw_shadow(&mut img, cx, 58.0, 13.0, 5.0);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 38.0, 7.0, 12.0, 16.0, 11.0, &armor_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 10.0, 10.0, &armor_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 34.0, -5.0, 18.0, &glow_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 18.0,
        10.0,
        48.0,
        9.0,
        2.0,
        &sword_mat,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 18.0,
        36.0,
        10.0,
        8.0,
        11.0,
        6.0,
        &shield_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 18.0,
        30.0,
        42.0,
        12.0,
        1.0,
        &gold_mat,
    );

    img
}

pub fn create_wizard_sprite() -> RgbaImage {
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
    draw_cylinder_3d(&mut img, &mut depth, cx, 36.0, 56.0, 5.0, 10.0, &robe_mat);
    draw_cone_3d(&mut img, &mut depth, cx, 6.0, 28.0, 6.0, 8.0, &hat_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 28.0, 8.0, 6.0, &skin_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 44.0, 8.0, 2.0, &star_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 48.0, 8.0, 2.0, &star_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 14.0,
        10.0,
        54.0,
        4.0,
        2.0,
        &wood_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 14.0, 8.0, 8.0, 6.0, &orb_mat);

    img
}

pub fn create_inquisitor_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe_mat = Material::matte(140, 30, 30);
    let hood_mat = Material::matte(60, 10, 10);
    let eye_mat = Material::glowing(255, 0, 0);
    let sword_mat = Material::metallic(220, 220, 220);
    let fire_mat = Material::glowing(255, 100, 0);

    draw_shadow(&mut img, cx, 58.0, 11.0, 4.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 56.0, 5.0, 11.0, &robe_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 7.0, 8.0, &hood_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 24.0, 11.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 24.0, 11.0, 2.0, &eye_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 16.0,
        14.0,
        48.0,
        8.0,
        2.0,
        &sword_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx + 16.0, 12.0, 10.0, 5.0, &fire_mat);

    img
}

pub fn create_geomancer_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let rock_armor = Material::stone(100, 90, 80);
    let crystal = Material::glowing(200, 100, 200);

    let cx = SPRITE_SIZE as f32 / 2.0;

    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    // Bulky Rock Armor Body
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx,
        38.0,
        8.0,
        12.0,
        16.0,
        10.0,
        &rock_armor,
    );
    // Shoulders
    draw_sphere_3d(
        &mut img,
        &mut depth,
        cx - 10.0,
        30.0,
        12.0,
        5.0,
        &rock_armor,
    );
    draw_sphere_3d(
        &mut img,
        &mut depth,
        cx + 10.0,
        30.0,
        12.0,
        5.0,
        &rock_armor,
    );
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 18.0, 10.0, 7.0, &rock_armor);
    // Floating crystals
    draw_sphere_3d(&mut img, &mut depth, cx - 8.0, 8.0, 15.0, 3.0, &crystal);
    draw_sphere_3d(&mut img, &mut depth, cx + 8.0, 12.0, 12.0, 2.5, &crystal);
    draw_sphere_3d(&mut img, &mut depth, cx, 6.0, 18.0, 4.0, &crystal);

    img
}
