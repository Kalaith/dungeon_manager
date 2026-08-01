use super::super::core::*;
use image::RgbaImage;

/// Ogre — an area-control brute. Drawn hunched, but with the head clearing the
/// shoulders: sinking it between them (the obvious way to say "brute") just
/// deletes it, and what is left reads as a headless lump with ears. A heavy
/// brow ridge does the work instead. The maul head is wider than it is tall so
/// it reads as a hammer rather than a signboard.
pub fn create_ogre_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let hide = Material::matte(124, 108, 90);
    let dark_hide = Material::matte(90, 78, 64);
    let tusk = Material::bone();
    let eye = Material::glowing(220, 190, 90);
    let haft = Material::wood(104, 72, 40);
    let iron = Material::metallic(88, 90, 96);

    draw_shadow(&mut img, cx, 60.0, 20.0, 7.0);

    for side in [-1.0f32, 1.0] {
        draw_cylinder_3d(
            &mut img,
            &mut depth,
            cx + side * 9.0,
            44.0,
            58.0,
            5.0,
            5.5,
            &dark_hide,
        );
    }

    // Wide, low torso
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 8.0, 14.0, 11.0, 10.0, &hide);

    // Shoulders sit low and wide, leaving the head clear above them
    draw_sphere_3d(&mut img, &mut depth, cx - 15.0, 31.0, 9.0, 7.5, &dark_hide);
    draw_sphere_3d(&mut img, &mut depth, cx + 15.0, 31.0, 9.0, 7.5, &dark_hide);
    draw_sphere_3d(&mut img, &mut depth, cx, 21.0, 13.0, 7.0, &hide);

    // Heavy brow, underbite, small mean eyes beneath
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 17.5, 16.0, 6.5, 2.0, 3.0, &dark_hide,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 2.8, 21.0, 19.0, 1.4, &eye);
    draw_sphere_3d(&mut img, &mut depth, cx + 2.8, 21.0, 19.0, 1.4, &eye);
    draw_cone_3d(&mut img, &mut depth, cx - 3.2, 19.0, 26.0, 18.0, 1.4, &tusk);
    draw_cone_3d(&mut img, &mut depth, cx + 3.2, 19.0, 26.0, 18.0, 1.4, &tusk);

    // Maul
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 21.0,
        24.0,
        52.0,
        12.0,
        2.0,
        &haft,
    );
    draw_box_3d(
        &mut img,
        &mut depth,
        cx + 21.0,
        21.0,
        12.0,
        5.5,
        3.0,
        3.5,
        &iron,
    );

    img
}

/// Ironbound — a construct. Everything here is a box: it is the only creature
/// in the roster with no curved surface, which is the whole point of the
/// silhouette. Cold blue rune-light rather than the demons' fire, and no face —
/// a single lit slit where eyes would be.
pub fn create_ironbound_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let iron = Material::metallic(98, 102, 110);
    let dark_iron = Material::metallic(62, 66, 74);
    let rune = Material::glowing(120, 200, 255);

    draw_shadow(&mut img, cx, 59.0, 14.0, 5.0);

    for side in [-1.0f32, 1.0] {
        draw_box_3d(
            &mut img,
            &mut depth,
            cx + side * 7.0,
            50.0,
            5.0,
            4.0,
            8.0,
            4.0,
            &dark_iron,
        );
        draw_cylinder_3d(
            &mut img,
            &mut depth,
            cx + side * 15.0,
            30.0,
            46.0,
            8.0,
            3.0,
            &dark_iron,
        );
        draw_box_3d(
            &mut img,
            &mut depth,
            cx + side * 15.0,
            48.0,
            8.0,
            3.5,
            3.5,
            3.5,
            &iron,
        );
        draw_box_3d(
            &mut img,
            &mut depth,
            cx + side * 13.0,
            24.0,
            9.0,
            5.0,
            4.5,
            5.0,
            &iron,
        );
    }

    // Slab torso
    draw_box_3d(&mut img, &mut depth, cx, 34.0, 8.0, 11.0, 13.0, 8.0, &iron);

    // Bound rune burning in the chest — the thing keeping it standing
    draw_sphere_3d(&mut img, &mut depth, cx, 34.0, 17.0, 3.5, &rune);

    // Featureless head with a single lit slit
    draw_box_3d(
        &mut img, &mut depth, cx, 17.0, 11.0, 5.0, 5.0, 4.5, &dark_iron,
    );
    draw_box_3d(&mut img, &mut depth, cx, 17.0, 16.0, 3.5, 0.9, 0.5, &rune);

    img
}
