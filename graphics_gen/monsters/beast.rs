use super::super::core::*;
use image::RgbaImage;

pub fn create_hellhound_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let fur = Material::matte(180, 50, 40);
    let eyes = Material::glowing(255, 200, 50);
    let fire_mat = Material::fire();

    let cx = SPRITE_SIZE as f32 / 2.0;

    draw_shadow(&mut img, cx, 58.0, 14.0, 5.0);
    // Body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 10.0, 14.0, 12.0, 10.0, &fur);
    // Heads (two-headed)
    draw_sphere_3d(&mut img, &mut depth, cx - 8.0, 28.0, 12.0, 6.0, &fur);
    draw_sphere_3d(&mut img, &mut depth, cx + 8.0, 28.0, 12.0, 6.0, &fur);
    // Snouts
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 10.0,
        30.0,
        16.0,
        4.0,
        3.0,
        3.0,
        &fur,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 10.0,
        30.0,
        16.0,
        4.0,
        3.0,
        3.0,
        &fur,
    );
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 9.0, 26.0, 15.0, 1.5, &eyes);
    draw_sphere_3d(&mut img, &mut depth, cx + 7.0, 26.0, 15.0, 1.5, &eyes);
    // Fire breath
    draw_sphere_3d(&mut img, &mut depth, cx - 12.0, 32.0, 18.0, 3.0, &fire_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 12.0, 32.0, 18.0, 3.0, &fire_mat);
    // Legs
    draw_cylinder_3d(&mut img, &mut depth, cx - 8.0, 48.0, 58.0, 8.0, 3.0, &fur);
    draw_cylinder_3d(&mut img, &mut depth, cx + 8.0, 48.0, 58.0, 8.0, 3.0, &fur);

    img
}

pub fn create_spider_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let body_mat = Material::matte(40, 40, 45);
    let abdomen_mat = Material::matte(30, 30, 35);
    let eye_mat = Material::glowing(200, 20, 20);

    draw_shadow(&mut img, cx, 58.0, 15.0, 8.0);

    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx,
        36.0,
        20.0,
        12.0,
        10.0,
        15.0,
        &abdomen_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 42.0, 12.0, 8.0, &body_mat);

    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 40.0, 16.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 40.0, 16.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 6.0, 39.0, 15.0, 1.5, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 6.0, 39.0, 15.0, 1.5, &eye_mat);

    let leg_mat = Material::matte(35, 35, 40);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 12.0,
        45.0,
        58.0,
        10.0,
        2.0,
        &leg_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 14.0,
        40.0,
        56.0,
        15.0,
        2.0,
        &leg_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 14.0,
        35.0,
        56.0,
        20.0,
        2.0,
        &leg_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 12.0,
        30.0,
        58.0,
        25.0,
        2.0,
        &leg_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 12.0,
        45.0,
        58.0,
        10.0,
        2.0,
        &leg_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 14.0,
        40.0,
        56.0,
        15.0,
        2.0,
        &leg_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 14.0,
        35.0,
        56.0,
        20.0,
        2.0,
        &leg_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 12.0,
        30.0,
        58.0,
        25.0,
        2.0,
        &leg_mat,
    );

    img
}

pub fn create_lizard_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let scale_mat = Material::metallic(60, 140, 60);
    let shadow_mat = Material::matte(40, 100, 40);
    let eye_mat = Material::glowing(255, 255, 0);

    draw_shadow(&mut img, cx, 58.0, 18.0, 6.0);

    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 5.0,
        40.0,
        58.0,
        -10.0,
        5.0,
        &scale_mat,
    );
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 45.0, 5.0, 10.0, 18.0, 8.0, &scale_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 8.0,
        48.0,
        58.0,
        5.0,
        3.0,
        &shadow_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 8.0,
        48.0,
        58.0,
        5.0,
        3.0,
        &shadow_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 6.0,
        52.0,
        58.0,
        15.0,
        3.0,
        &shadow_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 6.0,
        52.0,
        58.0,
        15.0,
        3.0,
        &shadow_mat,
    );
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 30.0, 8.0, 8.0, 10.0, 6.0, &scale_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 28.0, 10.0, 1.5, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 28.0, 10.0, 1.5, &eye_mat);

    img
}

/// Bat swarm sprite - NEW
pub fn create_bat_swarm_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let bat_mat = Material::matte(40, 30, 50);
    let eye_mat = Material::glowing(255, 50, 50);

    // Multiple small bats
    for (offset_x, offset_y, scale) in [
        (0.0, 0.0, 1.0),
        (-10.0, -8.0, 0.8),
        (12.0, -5.0, 0.7),
        (-8.0, 10.0, 0.6),
        (10.0, 12.0, 0.5),
    ] {
        let bx = cx + offset_x;
        let by = 32.0 + offset_y;
        let r = 4.0 * scale;
        // Body
        draw_sphere_3d(&mut img, &mut depth, bx, by, 5.0, r, &bat_mat);
        // Wings
        draw_ellipsoid_3d(
            &mut img,
            &mut depth,
            bx - r * 2.0,
            by,
            4.0,
            r * 2.0,
            r,
            1.0,
            &bat_mat,
        );
        draw_ellipsoid_3d(
            &mut img,
            &mut depth,
            bx + r * 2.0,
            by,
            4.0,
            r * 2.0,
            r,
            1.0,
            &bat_mat,
        );
        // Eyes
        draw_sphere_3d(
            &mut img,
            &mut depth,
            bx - 1.0,
            by - 1.0,
            7.0,
            0.8 * scale,
            &eye_mat,
        );
        draw_sphere_3d(
            &mut img,
            &mut depth,
            bx + 1.0,
            by - 1.0,
            7.0,
            0.8 * scale,
            &eye_mat,
        );
    }

    img
}

/// Dark elf sprite - NEW
pub fn create_dark_elf_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let skin_mat = Material::matte(80, 70, 90);
    let armor_mat = Material::metallic(40, 40, 50);
    let hair_mat = Material::matte(240, 240, 255);
    let eye_mat = Material::glowing(200, 50, 200);
    let blade_mat = Material::metallic(150, 100, 200);

    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);
    // Body
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 40.0, 6.0, 9.0, 13.0, 7.0, &armor_mat,
    );
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 9.0, 6.0, &skin_mat);
    // Long hair
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 28.0, 5.0, 7.0, 10.0, 5.0, &hair_mat,
    );
    // Pointed ears
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 8.0,
        20.0,
        26.0,
        7.0,
        2.0,
        &skin_mat,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx + 8.0,
        20.0,
        26.0,
        7.0,
        2.0,
        &skin_mat,
    );
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 2.0, 23.0, 12.0, 1.5, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 2.0, 23.0, 12.0, 1.5, &eye_mat);
    // Dual blades
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 14.0,
        18.0,
        46.0,
        6.0,
        1.5,
        &blade_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 14.0,
        18.0,
        46.0,
        6.0,
        1.5,
        &blade_mat,
    );

    img
}
