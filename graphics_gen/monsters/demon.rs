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

/// Balor — the endgame demon, and deliberately the largest silhouette in the
/// roster: wings spread behind it, horns clearing the top of the frame, and its
/// own fire burning through cracks in the hide.
pub fn create_balor_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let hide = Material::matte(112, 30, 26);
    let dark_hide = Material::matte(78, 20, 18);
    let wing = Material::matte(56, 18, 18);
    let horn = Material::matte(76, 60, 50);
    let ember = Material::glowing(255, 130, 30);
    let eye = Material::glowing(255, 235, 120);
    // `Material::fire()` is pure ambient, so anything drawn with it comes out a
    // flat orange shape with no form — fine for a 4px flame blob, useless for a
    // sword. This keeps the heat but retains a diffuse and specular response.
    let blade = Material::new(255, 140, 50, 0.6, 0.5, 0.6, 24.0);
    let steel = Material::metallic(70, 60, 62);

    draw_shadow(&mut img, cx, 60.0, 19.0, 7.0);

    // Wings sit at negative z so the depth buffer keeps them behind the torso.
    // They are tall rather than round: as round blobs they merely widened the
    // body instead of reading as wings.
    for side in [-1.0f32, 1.0] {
        draw_ellipsoid_3d(
            &mut img,
            &mut depth,
            cx + side * 17.0,
            22.0,
            -7.0,
            8.0,
            17.0,
            2.5,
            &wing,
        );
        // Leading-edge spar, clearing the shoulder so the outline reads.
        draw_cone_3d(
            &mut img,
            &mut depth,
            cx + side * 21.0,
            6.0,
            28.0,
            -6.0,
            2.5,
            &wing,
        );
    }

    // Legs
    for side in [-1.0f32, 1.0] {
        draw_cylinder_3d(
            &mut img,
            &mut depth,
            cx + side * 8.0,
            44.0,
            58.0,
            6.0,
            4.5,
            &dark_hide,
        );
    }

    // Torso and shoulders — narrower than the bile demon's, so the mass reads
    // as broad-shouldered rather than merely fat.
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 8.0, 12.0, 13.0, 10.0, &hide);
    draw_sphere_3d(&mut img, &mut depth, cx - 14.0, 26.0, 9.0, 7.5, &hide);
    draw_sphere_3d(&mut img, &mut depth, cx + 14.0, 26.0, 9.0, 7.5, &hide);

    // A single burning core in the chest. Scattered embers read as noise at
    // 64px; one bright focal point survives the downscale.
    draw_sphere_3d(&mut img, &mut depth, cx, 34.0, 18.0, 4.0, &ember);
    draw_sphere_3d(&mut img, &mut depth, cx - 6.0, 43.0, 16.0, 1.6, &ember);
    draw_sphere_3d(&mut img, &mut depth, cx + 6.0, 43.0, 16.0, 1.6, &ember);

    // Head and swept horns
    draw_sphere_3d(&mut img, &mut depth, cx, 19.0, 11.0, 8.0, &dark_hide);
    draw_cone_3d(&mut img, &mut depth, cx - 9.0, 6.0, 18.0, 10.0, 4.5, &horn);
    draw_cone_3d(&mut img, &mut depth, cx + 9.0, 6.0, 18.0, 10.0, 4.5, &horn);
    draw_sphere_3d(&mut img, &mut depth, cx - 3.5, 18.0, 18.0, 2.0, &eye);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.5, 18.0, 18.0, 2.0, &eye);

    // Burning sword, held point-up: blade, crossguard, grip.
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 22.0,
        24.0,
        13.0,
        2.5,
        15.0,
        2.0,
        &blade,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 22.0,
        40.0,
        13.0,
        5.0,
        1.8,
        2.0,
        &steel,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 22.0,
        41.0,
        49.0,
        13.0,
        1.6,
        &steel,
    );

    // Fire pooling where it stands
    draw_sphere_3d(&mut img, &mut depth, cx - 11.0, 57.0, 2.0, 4.0, &ember);
    draw_sphere_3d(&mut img, &mut depth, cx + 11.0, 57.0, 2.0, 4.0, &ember);

    img
}

/// Infernal Hound — a fast tracker, drawn in side profile facing right. The
/// horizontal silhouette is the point: every other demon in the roster is an
/// upright mass, so the hound has to read as something built to run at a
/// glance, and at 64px that has to come from the outline rather than detail.
pub fn create_infernal_hound_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let hide = Material::matte(96, 42, 44);
    let dark = Material::matte(58, 24, 28);
    let flame = Material::fire();
    let ember = Material::glowing(255, 150, 40);
    let eye = Material::glowing(255, 230, 90);

    draw_shadow(&mut img, cx, 57.0, 18.0, 5.0);

    // Far-side legs first, set deeper so the near pair reads in front of them.
    draw_cylinder_3d(&mut img, &mut depth, cx + 8.0, 42.0, 56.0, 3.0, 2.4, &dark);
    draw_cylinder_3d(&mut img, &mut depth, cx - 12.0, 42.0, 56.0, 3.0, 2.6, &dark);

    // Long, low body with haunch and chest
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 1.0,
        37.0,
        9.0,
        15.0,
        7.0,
        7.0,
        &hide,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 11.0, 36.0, 10.0, 8.0, &hide);
    draw_sphere_3d(&mut img, &mut depth, cx + 8.0, 37.0, 10.0, 7.0, &hide);

    // Near legs
    draw_cylinder_3d(&mut img, &mut depth, cx + 11.0, 42.0, 58.0, 9.0, 2.6, &hide);
    draw_cylinder_3d(&mut img, &mut depth, cx - 9.0, 42.0, 58.0, 9.0, 2.8, &hide);

    // Neck and head, thrust forward
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 14.0,
        30.0,
        11.0,
        5.0,
        6.0,
        5.0,
        &hide,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 19.0,
        25.0,
        12.0,
        6.0,
        4.5,
        4.5,
        &hide,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 25.0,
        26.0,
        11.0,
        4.0,
        2.6,
        2.6,
        &dark,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx + 16.0,
        15.0,
        24.0,
        13.0,
        2.0,
        &dark,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx + 21.0,
        15.0,
        23.0,
        13.0,
        2.0,
        &dark,
    );
    draw_sphere_3d(&mut img, &mut depth, cx + 21.0, 24.0, 16.0, 1.6, &eye);

    // Mane burning down the spine, cooling to embers at the rump
    draw_sphere_3d(&mut img, &mut depth, cx + 9.0, 26.0, 12.0, 3.4, &flame);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 25.0, 12.0, 4.0, &flame);
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 26.0, 11.0, 3.6, &flame);
    draw_sphere_3d(&mut img, &mut depth, cx - 9.0, 28.0, 10.0, 3.0, &ember);

    // Tail
    draw_cone_3d(&mut img, &mut depth, cx - 19.0, 20.0, 36.0, 8.0, 3.0, &hide);
    draw_sphere_3d(&mut img, &mut depth, cx - 19.0, 19.0, 8.0, 2.6, &ember);

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
