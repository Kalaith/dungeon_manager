//! Tier 1 heroes - Basic Units

use super::super::core::*;
use image::RgbaImage;

pub fn create_peasant_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let cloth_mat = Material::matte(160, 140, 100);
    let skin_mat = Material::matte(230, 190, 150);
    let wood_mat = Material::matte(139, 90, 43);
    let metal_mat = Material::metallic(169, 169, 169);

    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 42.0, 5.0, 9.0, 12.0, 8.0, &cloth_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 8.0, 7.0, &skin_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 14.0,
        14.0,
        50.0,
        4.0,
        2.0,
        &wood_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 10.0,
        10.0,
        18.0,
        5.0,
        1.0,
        &metal_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 14.0,
        10.0,
        18.0,
        5.0,
        1.0,
        &metal_mat,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 18.0,
        10.0,
        18.0,
        5.0,
        1.0,
        &metal_mat,
    );

    img
}

pub fn create_scout_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let leather_mat = Material::leather(100, 120, 80);
    let skin_mat = Material::matte(230, 190, 150);
    let wood_mat = Material::matte(139, 90, 43);

    draw_shadow(&mut img, cx, 56.0, 9.0, 4.0);
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx,
        40.0,
        5.0,
        9.0,
        13.0,
        7.0,
        &leather_mat,
    );
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 8.0, 7.0, &skin_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 6.0, 8.0, &leather_mat);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 14.0,
        18.0,
        48.0,
        4.0,
        2.0,
        &wood_mat,
    );

    img
}

pub fn create_acolyte_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe_mat = Material::matte(240, 240, 250);
    let skin_mat = Material::matte(230, 190, 150);
    let gold_mat = Material::metallic(255, 215, 0);

    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 56.0, 5.0, 11.0, &robe_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 8.0, 7.0, &skin_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 38.0, 52.0, 8.0, 2.0, &gold_mat);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 42.0, 9.0, 5.0, 2.0, 2.0, &gold_mat,
    );

    img
}
