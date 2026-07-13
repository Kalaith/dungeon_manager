use super::super::core::*;
use image::RgbaImage;

pub fn create_skeleton_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let bone_mat = Material::bone();
    let eye_mat = Material::matte(20, 20, 20);
    let sword_mat = Material::metallic(192, 192, 192);

    draw_shadow(&mut img, cx, 58.0, 8.0, 3.0);
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 8.0, 9.0, &bone_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 20.0, 12.0, 3.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 20.0, 12.0, 3.0, &eye_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 32.0, 54.0, 5.0, 2.0, &bone_mat);
    for i in 0..3 {
        let y = 36.0 + i as f32 * 5.0;
        draw_ellipsoid_3d(&mut img, &mut depth, cx, y, 5.0, 8.0, 2.0, 4.0, &bone_mat);
    }
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 10.0,
        34.0,
        50.0,
        3.0,
        2.0,
        &bone_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 10.0,
        34.0,
        50.0,
        3.0,
        2.0,
        &bone_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 16.0,
        12.0,
        46.0,
        7.0,
        2.0,
        &sword_mat,
    );

    img
}

pub fn create_vampire_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let skin = Material::matte(220, 220, 230);
    let cape = Material::matte(40, 10, 10);
    let clothes = Material::matte(30, 30, 40);
    let eye_mat = Material::glowing(255, 0, 0);

    let cx = SPRITE_SIZE as f32 / 2.0;

    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    // Cape (behind)
    draw_cylinder_3d(&mut img, &mut depth, cx, 15.0, 50.0, -2.0, 14.0, &cape);
    // Body
    draw_cylinder_3d(&mut img, &mut depth, cx, 20.0, 48.0, 10.0, 8.0, &clothes);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 15.0, 12.0, 6.0, &skin);
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 2.0, 14.0, 15.0, 1.5, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 2.0, 14.0, 15.0, 1.5, &eye_mat);
    // Hair
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 10.0, 11.0, 7.0, 4.0, 5.0, &clothes,
    );

    img
}

/// Zombie sprite - NEW
pub fn create_zombie_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let flesh_mat = Material::flesh(100, 140, 80); // Greenish dead flesh
    let eye_mat = Material::glowing(200, 200, 100);
    let rag_mat = Material::matte(80, 70, 60);

    draw_shadow(&mut img, cx, 58.0, 11.0, 4.0);
    // Hunched body
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 3.0,
        40.0,
        5.0,
        10.0,
        14.0,
        9.0,
        &flesh_mat,
    );
    // Head (tilted)
    draw_sphere_3d(&mut img, &mut depth, cx - 2.0, 26.0, 8.0, 8.0, &flesh_mat);
    // One arm raised
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 14.0,
        20.0,
        42.0,
        5.0,
        3.0,
        &flesh_mat,
    );
    // Dragging arm
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 12.0,
        36.0,
        56.0,
        4.0,
        3.0,
        &flesh_mat,
    );
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 24.0, 12.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 25.0, 12.0, 2.0, &eye_mat);
    // Torn clothing
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 45.0, 6.0, 8.0, 6.0, 6.0, &rag_mat);

    img
}

/// Ghost sprite - NEW
pub fn create_ghost_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    // Semi-transparent ghostly material
    let ghost_mat = Material::new(180, 200, 255, 0.6, 0.3, 0.4, 16.0);
    let eye_mat = Material::glowing(100, 150, 255);

    // No shadow for ghost (floating)
    // Wispy body
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 38.0, 8.0, 10.0, 16.0, 10.0, &ghost_mat,
    );
    draw_cylinder_3d(&mut img, &mut depth, cx, 50.0, 60.0, 6.0, 8.0, &ghost_mat);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 10.0, 9.0, &ghost_mat);
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 20.0, 14.0, 2.5, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 20.0, 14.0, 2.5, &eye_mat);
    // Wispy arms
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 12.0,
        30.0,
        45.0,
        6.0,
        4.0,
        &ghost_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 12.0,
        30.0,
        45.0,
        6.0,
        4.0,
        &ghost_mat,
    );

    img
}
