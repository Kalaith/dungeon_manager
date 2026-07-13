use super::super::core::*;
use image::RgbaImage;

pub fn create_demon_spawn_sprite() -> RgbaImage {
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
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 40.0, 6.0, 13.0, 16.0, 11.0, &body_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 9.0, 10.0, &body_mat);
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 10.0,
        4.0,
        18.0,
        7.0,
        4.0,
        &horn_mat,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx + 10.0,
        4.0,
        18.0,
        7.0,
        4.0,
        &horn_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 18.0, 13.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 18.0, 13.0, 2.0, &eye_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 16.0,
        32.0,
        48.0,
        4.0,
        4.0,
        &body_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 16.0,
        32.0,
        48.0,
        4.0,
        4.0,
        &body_mat,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 18.0,
        48.0,
        56.0,
        3.0,
        3.0,
        &claw_mat,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx + 18.0,
        48.0,
        56.0,
        3.0,
        3.0,
        &claw_mat,
    );

    img
}

pub fn create_bile_demon_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let flab = Material::matte(180, 200, 50);
    let horns = Material::bone();
    let eye_mat = Material::glowing(255, 100, 0);

    let cx = SPRITE_SIZE as f32 / 2.0;

    draw_shadow(&mut img, cx, 60.0, 18.0, 7.0);
    // Huge Body
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 38.0, 10.0, 18.0, 18.0, 16.0, &flab,
    );
    // Head integrated into body
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 12.0, 10.0, &flab);
    // Horns
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 10.0,
        10.0,
        22.0,
        10.0,
        3.0,
        &horns,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx + 10.0,
        10.0,
        22.0,
        10.0,
        3.0,
        &horns,
    );
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 18.0, 16.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 18.0, 16.0, 2.0, &eye_mat);

    img
}

/// Succubus sprite - NEW
pub fn create_succubus_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let skin_mat = Material::flesh(220, 160, 180);
    let wing_mat = Material::matte(80, 40, 60);
    let horn_mat = Material::matte(60, 20, 40);
    let eye_mat = Material::glowing(255, 100, 200);
    let hair_mat = Material::matte(40, 20, 30);

    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);
    // Wings (behind)
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 14.0,
        30.0,
        -4.0,
        10.0,
        14.0,
        3.0,
        &wing_mat,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 14.0,
        30.0,
        -4.0,
        10.0,
        14.0,
        3.0,
        &wing_mat,
    );
    // Body
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 40.0, 8.0, 8.0, 14.0, 7.0, &skin_mat,
    );
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 10.0, 7.0, &skin_mat);
    // Hair
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 18.0, 8.0, 8.0, 6.0, 6.0, &hair_mat,
    );
    // Horns
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 6.0,
        10.0,
        18.0,
        8.0,
        2.0,
        &horn_mat,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx + 6.0,
        10.0,
        18.0,
        8.0,
        2.0,
        &horn_mat,
    );
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 2.0, 21.0, 14.0, 1.5, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 2.0, 21.0, 14.0, 1.5, &eye_mat);
    // Tail
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 5.0,
        50.0,
        60.0,
        5.0,
        2.0,
        &skin_mat,
    );

    img
}
