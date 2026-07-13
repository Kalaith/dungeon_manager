//! Tier 2 heroes - Standard Units

use super::super::core::*;
use image::RgbaImage;

pub fn create_knight_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let armor_mat = Material::metallic(180, 180, 200);
    let plume_mat = Material::matte(200, 0, 0);
    let shield_mat = Material::metallic(100, 100, 120);
    let sword_mat = Material::metallic(220, 220, 220);

    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 40.0, 6.0, 11.0, 15.0, 10.0, &armor_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 9.0, 9.0, &armor_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 8.0, 20.0, 8.0, 3.0, &plume_mat);
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 16.0,
        38.0,
        10.0,
        7.0,
        10.0,
        5.0,
        &shield_mat,
    );
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

    img
}

pub fn create_archer_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let leather_mat = Material::leather(120, 100, 70);
    let skin_mat = Material::matte(230, 190, 150);
    let wood_mat = Material::matte(139, 90, 43);
    let metal_mat = Material::metallic(169, 169, 169);

    draw_shadow(&mut img, cx, 56.0, 10.0, 4.0);
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx,
        40.0,
        5.0,
        9.0,
        13.0,
        8.0,
        &leather_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 8.0, 7.0, &skin_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 16.0,
        14.0,
        50.0,
        4.0,
        2.0,
        &wood_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 10.0,
        28.0,
        30.0,
        6.0,
        1.0,
        &wood_mat,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 4.0,
        27.0,
        30.0,
        7.0,
        2.0,
        &metal_mat,
    );

    img
}

pub fn create_battle_cleric_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let armor_mat = Material::metallic(200, 200, 220);
    let skin_mat = Material::matte(230, 190, 150);
    let gold_mat = Material::metallic(255, 215, 0);
    let wood_mat = Material::matte(139, 90, 43);
    let mace_mat = Material::metallic(150, 150, 150);

    draw_shadow(&mut img, cx, 58.0, 11.0, 5.0);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 40.0, 6.0, 11.0, 14.0, 9.0, &armor_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 8.0, 7.0, &skin_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 36.0, 48.0, 8.0, 2.0, &gold_mat);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 40.0, 9.0, 5.0, 2.0, 2.0, &gold_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 18.0,
        26.0,
        50.0,
        5.0,
        2.0,
        &wood_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx + 18.0, 22.0, 8.0, 5.0, &mace_mat);

    img
}

pub fn create_rogue_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let cloak_mat = Material::matte(60, 60, 80);
    let hood_mat = Material::matte(50, 50, 70);
    let eye_mat = Material::glowing(255, 255, 0);
    let dagger_mat = Material::metallic(192, 192, 192);

    draw_shadow(&mut img, cx, 56.0, 9.0, 4.0);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 40.0, 5.0, 9.0, 13.0, 7.0, &cloak_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 7.0, 8.0, &hood_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 24.0, 10.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 24.0, 10.0, 2.0, &eye_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 14.0,
        32.0,
        48.0,
        5.0,
        1.0,
        &dagger_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 14.0,
        32.0,
        48.0,
        5.0,
        1.0,
        &dagger_mat,
    );

    img
}

pub fn create_barbarian_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let skin = Material::matte(200, 140, 100);
    let leather = Material::leather(120, 80, 40);
    let metal = Material::metallic(180, 180, 180);
    let hair_mat = Material::matte(60, 40, 20);

    let cx = SPRITE_SIZE as f32 / 2.0;

    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    // Muscular body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 7.0, 12.0, 15.0, 10.0, &skin);
    // Kilt
    draw_cylinder_3d(&mut img, &mut depth, cx, 46.0, 56.0, 6.0, 10.0, &leather);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 18.0, 10.0, 8.0, &skin);
    // Wild hair
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 14.0, 8.0, 10.0, 6.0, 8.0, &hair_mat,
    );
    // Huge axe on back
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 10.0,
        10.0,
        52.0,
        5.0,
        2.0,
        &leather,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 14.0,
        16.0,
        8.0,
        5.0,
        10.0,
        2.0,
        &metal,
    );

    img
}

pub fn create_alchemist_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let robe = Material::matte(40, 140, 80);
    let glass = Material::glowing(100, 255, 100);
    let skin_mat = Material::matte(230, 190, 150);

    let cx = SPRITE_SIZE as f32 / 2.0;

    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);
    // Body
    draw_cylinder_3d(&mut img, &mut depth, cx, 30.0, 56.0, 5.0, 10.0, &robe);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 8.0, 7.0, &skin_mat);
    // Goggles
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx,
        20.0,
        10.0,
        8.0,
        3.0,
        4.0,
        &Material::metallic(100, 80, 60),
    );
    // Flask in hand
    draw_sphere_3d(&mut img, &mut depth, cx + 12.0, 36.0, 10.0, 4.0, &glass);
    // Belt with vials
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx,
        42.0,
        46.0,
        7.0,
        11.0,
        &Material::leather(80, 60, 40),
    );

    img
}
