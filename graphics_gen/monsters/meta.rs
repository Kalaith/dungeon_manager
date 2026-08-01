use super::super::core::*;
use image::RgbaImage;

/// Overseer — a command unit. The read is entirely posture: it stands upright
/// and still where every other creature is drawn hunched or mid-stride, and it
/// carries a baton rather than a weapon. A floating ring of authority above the
/// head does the rest, since at 64px "in charge" has to be an icon.
pub fn create_overseer_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe = Material::matte(92, 54, 96);
    let trim = Material::matte(184, 152, 60);
    let skin = Material::flesh(148, 128, 108);
    let eye = Material::glowing(255, 210, 120);
    let baton = Material::metallic(178, 150, 66);

    draw_shadow(&mut img, cx, 58.0, 11.0, 4.0);

    // Upright, straight-shouldered stance
    draw_cone_3d(&mut img, &mut depth, cx, 30.0, 57.0, 6.0, 10.0, &robe);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 30.0, 9.0, 10.0, 4.5, 6.0, &trim);
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 11.0, 6.5, &skin);
    draw_sphere_3d(&mut img, &mut depth, cx - 2.2, 20.0, 16.0, 1.3, &eye);
    draw_sphere_3d(&mut img, &mut depth, cx + 2.2, 20.0, 16.0, 1.3, &eye);

    // Ring of authority, held above the head by nothing at all
    draw_torus_3d(&mut img, &mut depth, cx, 14.0, 14.0, 6.0, 1.1, &trim);

    // Baton, held out rather than raised
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 13.0,
        30.0,
        44.0,
        12.0,
        1.3,
        &baton,
    );
    draw_sphere_3d(&mut img, &mut depth, cx + 13.0, 29.0, 12.0, 2.4, &trim);

    img
}

/// Archivist — a research creature. Built around one silhouette cue: the stack
/// of books it is permanently carrying, which is wider than its own body and
/// therefore survives the downscale better than any robe detail could.
pub fn create_archivist_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe = Material::matte(58, 72, 104);
    let hood = Material::matte(42, 54, 80);
    let skin = Material::flesh(150, 132, 112);
    let eye = Material::glowing(150, 220, 255);
    let tome_a = Material::leather(122, 62, 44);
    let tome_b = Material::leather(74, 96, 58);
    let tome_c = Material::leather(96, 78, 120);
    let page = Material::matte(224, 216, 190);

    draw_shadow(&mut img, cx, 58.0, 11.0, 4.0);

    draw_cone_3d(&mut img, &mut depth, cx - 2.0, 30.0, 57.0, 6.0, 10.0, &robe);
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 2.0,
        31.0,
        9.0,
        8.5,
        4.5,
        6.0,
        &robe,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 1.0, 22.0, 10.0, 6.5, &hood);
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 1.0,
        24.0,
        14.0,
        4.0,
        3.5,
        2.0,
        &skin,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 24.0, 17.0, 1.2, &eye);
    draw_sphere_3d(&mut img, &mut depth, cx + 1.0, 24.0, 17.0, 1.2, &eye);

    // The stack: three tomes carried at the hip, deliberately overhanging the
    // body outline so the shape is unmistakable at a glance.
    for (i, mat) in [&tome_a, &tome_b, &tome_c].iter().enumerate() {
        let y = 40.0 - i as f32 * 4.4;
        draw_box_3d(&mut img, &mut depth, cx + 10.0, y, 11.0, 5.5, 2.0, 4.0, mat);
        // Page block inset on the outer edge only, so the spine still reads.
        draw_box_3d(
            &mut img,
            &mut depth,
            cx + 11.6,
            y,
            11.5,
            3.2,
            1.2,
            3.4,
            &page,
        );
    }

    img
}

/// Flesh Amalgam — a grafted thing assembled from parts that do not match.
/// Every other creature is drawn symmetrically; this one deliberately is not.
/// Mismatched limb colours, one arm far heavier than the other and a head set
/// off-centre do the work, because "stitched together" has to be legible from
/// the outline rather than from any surface detail at 64px.
pub fn create_flesh_amalgam_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    // Three donors, three skin tones — the graft is the point.
    let pale = Material::flesh(186, 158, 148);
    let grey = Material::flesh(132, 132, 124);
    let ruddy = Material::flesh(158, 106, 96);
    let thread = Material::matte(70, 54, 50);
    let eye = Material::glowing(255, 236, 180);
    let small_eye = Material::glowing(180, 255, 200);

    draw_shadow(&mut img, cx, 59.0, 15.0, 5.5);

    // Legs of different lengths and different donors
    draw_cylinder_3d(&mut img, &mut depth, cx - 7.0, 44.0, 58.0, 5.0, 4.5, &grey);
    draw_cylinder_3d(&mut img, &mut depth, cx + 7.0, 48.0, 58.0, 6.0, 3.5, &ruddy);

    // Trunk, lopsided
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 1.0,
        38.0,
        8.0,
        12.0,
        11.0,
        10.0,
        &pale,
    );
    draw_sphere_3d(&mut img, &mut depth, cx + 7.0, 42.0, 11.0, 6.0, &ruddy);

    // One heavy arm, one withered
    draw_sphere_3d(&mut img, &mut depth, cx - 15.0, 30.0, 8.0, 7.5, &grey);
    draw_sphere_3d(&mut img, &mut depth, cx - 17.0, 44.0, 7.0, 6.0, &grey);
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 30.0, 46.0, 8.0, 2.2, &pale);

    // Head off-centre, with a second smaller one that never finished
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 21.0, 12.0, 7.0, &pale);
    draw_sphere_3d(&mut img, &mut depth, cx + 7.0, 26.0, 13.0, 3.6, &ruddy);
    draw_sphere_3d(&mut img, &mut depth, cx - 5.5, 20.0, 18.0, 1.6, &eye);
    draw_sphere_3d(&mut img, &mut depth, cx - 0.5, 21.5, 18.0, 1.1, &eye);
    draw_sphere_3d(&mut img, &mut depth, cx + 7.5, 25.0, 16.0, 1.0, &small_eye);

    // Sutures where the donors meet
    for (x, y, z) in [
        (-1.0f32, 29.0f32, 17.0f32),
        (3.0, 33.0, 16.0),
        (-9.0, 36.0, 15.0),
    ] {
        draw_sphere_3d(&mut img, &mut depth, cx + x, y, z, 1.2, &thread);
    }

    img
}

/// Void-Touched — what a Flesh Amalgam becomes if it spends long enough beside
/// a ritual circle. The read is "the same silhouette, wrong": the amalgam's
/// lopsided build is kept, but the flesh is drained to near-black and the seams
/// glow through it. Nothing here is a new shape — corruption should look like
/// something happening *to* a creature, not like a different creature.
pub fn create_void_touched_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let void_flesh = Material::matte(46, 38, 62);
    let darker = Material::matte(32, 26, 46);
    let bruised = Material::matte(62, 44, 78);
    let rift = Material::glowing(180, 110, 255);
    let eye = Material::glowing(228, 190, 255);

    draw_shadow(&mut img, cx, 59.0, 16.0, 5.5);

    // Same lopsided frame as the amalgam it came from
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 7.0,
        44.0,
        58.0,
        5.0,
        4.5,
        &darker,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 7.0,
        48.0,
        58.0,
        6.0,
        3.5,
        &darker,
    );
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 1.0,
        38.0,
        8.0,
        12.0,
        11.0,
        10.0,
        &void_flesh,
    );
    draw_sphere_3d(&mut img, &mut depth, cx + 7.0, 42.0, 11.0, 6.0, &bruised);
    draw_sphere_3d(&mut img, &mut depth, cx - 15.0, 30.0, 8.0, 7.5, &void_flesh);
    draw_sphere_3d(&mut img, &mut depth, cx - 17.0, 44.0, 7.0, 6.0, &darker);
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 14.0,
        30.0,
        46.0,
        8.0,
        2.2,
        &void_flesh,
    );
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 21.0, 12.0, 7.0, &void_flesh);
    draw_sphere_3d(&mut img, &mut depth, cx + 7.0, 26.0, 13.0, 3.6, &bruised);

    // The seams have opened. Where the amalgam had sutures, this has light.
    for (x, y, z, r) in [
        (-1.0f32, 29.0f32, 18.0f32, 2.0f32),
        (3.0, 33.0, 17.0, 1.7),
        (-9.0, 36.0, 16.0, 1.7),
        (-14.0, 30.0, 15.0, 1.5),
    ] {
        draw_sphere_3d(&mut img, &mut depth, cx + x, y, z, r, &rift);
    }
    draw_sphere_3d(&mut img, &mut depth, cx - 5.5, 20.0, 18.0, 1.8, &eye);
    draw_sphere_3d(&mut img, &mut depth, cx - 0.5, 21.5, 18.0, 1.3, &eye);
    draw_sphere_3d(&mut img, &mut depth, cx + 7.5, 25.0, 16.0, 1.2, &eye);

    img
}
