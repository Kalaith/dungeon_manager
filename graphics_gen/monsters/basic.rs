use super::super::core::*;
use image::RgbaImage;

pub fn create_imp_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let body_mat = Material::matte(200, 50, 50);
    let horn_mat = Material::matte(100, 20, 20);
    let eye_mat = Material::glowing(255, 255, 0);

    draw_shadow(&mut img, cx, 54.0, 10.0, 4.0);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 38.0, 5.0, 9.0, 11.0, 8.0, &body_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 8.0, 8.0, &body_mat);
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 8.0,
        12.0,
        22.0,
        6.0,
        3.0,
        &horn_mat,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx + 8.0,
        12.0,
        22.0,
        6.0,
        3.0,
        &horn_mat,
    );
    draw_cylinder_3d(&mut img, &mut depth, cx, 46.0, 58.0, 3.0, 3.0, &body_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 22.0, 12.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 22.0, 12.0, 2.0, &eye_mat);

    img
}

pub fn create_goblin_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let body_mat = Material::matte(50, 150, 50);
    let ear_mat = Material::matte(70, 170, 70);
    let eye_mat = Material::glowing(255, 50, 0);
    let wood_mat = Material::matte(139, 90, 43);

    draw_shadow(&mut img, cx, 56.0, 11.0, 4.0);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 42.0, 5.0, 11.0, 13.0, 9.0, &body_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 8.0, 10.0, &body_mat);
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 12.0,
        18.0,
        28.0,
        5.0,
        4.0,
        &ear_mat,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx + 12.0,
        18.0,
        28.0,
        5.0,
        4.0,
        &ear_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 24.0, 12.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 24.0, 12.0, 2.0, &eye_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 16.0,
        20.0,
        44.0,
        4.0,
        3.0,
        &wood_mat,
    );

    img
}

pub fn create_orc_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let body_mat = Material::matte(100, 180, 80);
    let tusk_mat = Material::bone();
    let eye_mat = Material::glowing(255, 50, 0);
    let axe_mat = Material::metallic(169, 169, 169);
    let wood_mat = Material::matte(139, 90, 43);

    draw_shadow(&mut img, cx, 58.0, 14.0, 5.0);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 40.0, 6.0, 13.0, 16.0, 11.0, &body_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 9.0, 10.0, &body_mat);
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 5.0,
        28.0,
        36.0,
        10.0,
        2.0,
        &tusk_mat,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx + 5.0,
        28.0,
        36.0,
        10.0,
        2.0,
        &tusk_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 20.0, 13.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 20.0, 13.0, 2.0, &eye_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 20.0,
        16.0,
        48.0,
        5.0,
        2.0,
        &wood_mat,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 24.0,
        18.0,
        8.0,
        8.0,
        4.0,
        3.0,
        &axe_mat,
    );

    img
}

pub fn create_warlock_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe_mat = Material::matte(80, 40, 120);
    let hood_mat = Material::matte(60, 30, 90);
    let eye_mat = Material::glowing(150, 255, 150);
    let wood_mat = Material::matte(101, 67, 33);
    let crystal_mat = Material::glowing(200, 100, 255);

    draw_shadow(&mut img, cx, 58.0, 11.0, 4.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 56.0, 5.0, 11.0, &robe_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 7.0, 9.0, &hood_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 24.0, 12.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 24.0, 12.0, 2.0, &eye_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 14.0,
        12.0,
        54.0,
        4.0,
        2.0,
        &wood_mat,
    );
    draw_sphere_3d(
        &mut img,
        &mut depth,
        cx - 14.0,
        10.0,
        8.0,
        5.0,
        &crystal_mat,
    );

    img
}

pub fn create_troll_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let body_mat = Material::matte(120, 140, 100);
    let eye_mat = Material::glowing(255, 255, 0);
    let club_mat = Material::matte(101, 67, 33);

    draw_shadow(&mut img, cx, 60.0, 16.0, 6.0);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 42.0, 6.0, 15.0, 18.0, 13.0, &body_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 10.0, 12.0, &body_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 16.0,
        32.0,
        52.0,
        4.0,
        5.0,
        &body_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 16.0,
        32.0,
        52.0,
        4.0,
        5.0,
        &body_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 5.0, 20.0, 14.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 5.0, 20.0, 14.0, 2.0, &eye_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 22.0,
        18.0,
        50.0,
        6.0,
        4.0,
        &club_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx + 22.0, 14.0, 8.0, 7.0, &club_mat);

    img
}
