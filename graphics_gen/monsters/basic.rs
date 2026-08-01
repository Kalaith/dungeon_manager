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

/// Cultist — a lesser ritual caster. Deliberately humbler than the warlock it
/// stands beside: no staff, no crystal, just a robe, a burning sigil and a
/// knife. The hood is drawn as an empty shadow with two eyes in it, which reads
/// at 64px where a modelled face does not.
pub fn create_cultist_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe = Material::matte(126, 48, 42);
    let hood = Material::matte(92, 34, 30);
    let shadowed = Material::matte(26, 13, 13);
    let sigil = Material::glowing(255, 95, 60);
    let eye = Material::glowing(255, 175, 60);
    let steel = Material::metallic(150, 150, 160);
    let wood = Material::matte(96, 64, 36);

    draw_shadow(&mut img, cx, 58.0, 11.0, 4.0);

    // Robe flaring to the floor
    draw_cone_3d(&mut img, &mut depth, cx, 26.0, 57.0, 6.0, 12.0, &robe);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 30.0, 8.0, 8.0, 7.0, 6.0, &robe);

    // Hood, and the dark where a face would be
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 10.0, 7.5, &hood);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 21.0, 15.0, 4.5, 4.5, 2.0, &shadowed,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 2.2, 21.0, 17.0, 1.3, &eye);
    draw_sphere_3d(&mut img, &mut depth, cx + 2.2, 21.0, 17.0, 1.3, &eye);

    // Sigil burning on the chest
    draw_sphere_3d(&mut img, &mut depth, cx, 33.0, 15.0, 3.0, &sigil);

    // Sacrificial knife, held low
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 12.0,
        34.0,
        42.0,
        12.0,
        1.2,
        &wood,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 12.0,
        28.0,
        12.0,
        1.4,
        6.0,
        1.2,
        &steel,
    );

    img
}

/// Hexbinder — the control caster. Reads as neither warlock (purple, staff) nor
/// cultist (crimson, knife): a tall sickly-green silhouette with a blank mask
/// and three hex-sigils hanging in the air around it. The sigils are the whole
/// identity at this size, so they sit clear of the body outline.
pub fn create_hexbinder_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe = Material::matte(42, 78, 70);
    let trim = Material::matte(26, 50, 46);
    let mask = Material::matte(206, 210, 194);
    let eye = Material::glowing(120, 255, 160);
    let hex = Material::glowing(90, 255, 180);

    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);

    draw_cone_3d(&mut img, &mut depth, cx, 24.0, 57.0, 6.0, 10.0, &robe);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 28.0, 8.0, 7.0, 5.0, 5.0, &trim);

    // Peaked hood behind a long blank mask
    draw_cone_3d(&mut img, &mut depth, cx, 5.0, 21.0, 8.0, 6.5, &trim);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 19.0, 12.0, 5.0, 7.5, 5.0, &mask);
    draw_sphere_3d(&mut img, &mut depth, cx - 2.0, 18.0, 17.0, 1.3, &eye);
    draw_sphere_3d(&mut img, &mut depth, cx + 2.0, 18.0, 17.0, 1.3, &eye);

    // Three sigils hanging in the air, kept off the body so they read
    draw_sphere_3d(&mut img, &mut depth, cx - 14.0, 32.0, 14.0, 2.8, &hex);
    draw_sphere_3d(&mut img, &mut depth, cx + 14.0, 37.0, 14.0, 2.8, &hex);
    draw_sphere_3d(&mut img, &mut depth, cx + 11.0, 22.0, 6.0, 2.3, &hex);

    img
}

/// Gnoll — a fast skirmisher. Built narrow and forward-leaning against the
/// orc's bulk, with the hyena shoulder hump doing most of the identification
/// work; at 64px a silhouette that leans reads as "quick" where any amount of
/// fur detail would not.
pub fn create_gnoll_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let fur = Material::matte(150, 126, 88);
    let dark_fur = Material::matte(104, 84, 58);
    let eye = Material::glowing(255, 190, 70);
    let haft = Material::wood(100, 70, 42);
    let steel = Material::metallic(150, 152, 160);

    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);

    for side in [-1.0f32, 1.0] {
        draw_cylinder_3d(
            &mut img,
            &mut depth,
            cx + side * 6.0,
            42.0,
            58.0,
            5.0,
            3.2,
            &dark_fur,
        );
    }

    // Lean torso, then the hyena hump over the shoulders
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 35.0, 8.0, 6.5, 11.0, 6.5, &fur);
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 24.0, 10.0, 7.0, 4.5, 5.0, &dark_fur,
    );

    // Head thrust forward, long snout
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 3.0,
        20.0,
        13.0,
        5.0,
        4.0,
        4.5,
        &fur,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 9.0,
        21.0,
        13.0,
        4.0,
        2.4,
        2.4,
        &dark_fur,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 1.0,
        11.0,
        18.0,
        13.0,
        1.8,
        &dark_fur,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx + 5.0,
        11.0,
        18.0,
        13.0,
        1.8,
        &dark_fur,
    );
    draw_sphere_3d(&mut img, &mut depth, cx + 5.0, 19.0, 17.0, 1.2, &eye);

    // No tail: drawn as a cone it points up the body's flank and reads as a
    // stray limb, and this projection has no clean way to lay one horizontally.
    // The hump, snout and ears already carry the identification.
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 14.0,
        16.0,
        54.0,
        10.0,
        1.3,
        &haft,
    );
    draw_cone_3d(
        &mut img,
        &mut depth,
        cx - 14.0,
        9.0,
        18.0,
        10.0,
        2.0,
        &steel,
    );

    img
}

/// Kobold — a small trap-savvy fighter. Deliberately the smallest silhouette in
/// the roster: it occupies about half the frame, which is the only reliable way
/// to say "minor" when everything is drawn at the same 64px. Oversized head and
/// a tool it clearly did not forge itself.
pub fn create_kobold_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let scale = Material::matte(112, 122, 78);
    let belly = Material::matte(146, 150, 108);
    let crest = Material::matte(196, 128, 44);
    let eye = Material::glowing(255, 120, 60);
    let leather = Material::leather(96, 70, 46);
    let steel = Material::metallic(140, 142, 150);

    draw_shadow(&mut img, cx, 58.0, 9.0, 3.0);

    for side in [-1.0f32, 1.0] {
        draw_cylinder_3d(
            &mut img,
            &mut depth,
            cx + side * 4.0,
            48.0,
            58.0,
            4.0,
            2.6,
            &scale,
        );
    }

    draw_ellipsoid_3d(&mut img, &mut depth, cx, 42.0, 7.0, 7.0, 7.0, 6.0, &belly);
    draw_sphere_3d(&mut img, &mut depth, cx, 30.0, 10.0, 6.5, &scale);
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx + 4.5,
        32.0,
        12.0,
        4.0,
        2.2,
        2.2,
        &scale,
    );

    // Spined crest
    draw_cone_3d(&mut img, &mut depth, cx, 20.0, 27.0, 10.0, 1.7, &crest);
    draw_cone_3d(&mut img, &mut depth, cx - 3.5, 23.0, 28.0, 9.0, 1.3, &crest);
    draw_cone_3d(&mut img, &mut depth, cx + 3.5, 23.0, 28.0, 9.0, 1.3, &crest);
    draw_sphere_3d(&mut img, &mut depth, cx - 2.2, 29.5, 15.0, 1.2, &eye);
    draw_sphere_3d(&mut img, &mut depth, cx + 2.2, 29.5, 15.0, 1.2, &eye);

    // Satchel of trap parts, and the mallet it maintains them with
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 9.0,
        44.0,
        6.0,
        3.5,
        4.0,
        3.0,
        &leather,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 11.0,
        34.0,
        50.0,
        10.0,
        1.2,
        &leather,
    );
    draw_box_3d(
        &mut img,
        &mut depth,
        cx + 11.0,
        32.0,
        10.0,
        3.0,
        1.6,
        2.0,
        &steel,
    );

    img
}
