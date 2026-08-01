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

/// Lich - the undead tier's elite caster
///
/// Reads against the vampire's red-caped nobility: no flesh at all, a bare
/// skull over robes, and a soul-light held at the chest where the heart was.
/// `docs/monsters.md` gives it "soul-dependent power scaling", so the light is
/// the character rather than a decoration.
pub fn create_lich_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    // Kept light enough to read at 64px. A first pass used a near-black robe
    // and the skull vanished into it entirely.
    let bone = Material::matte(232, 228, 208);
    let robe = Material::matte(74, 66, 112);
    let trim = Material::matte(120, 104, 168);
    let soul = Material::glowing(120, 245, 210);

    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);

    // Robe: tip at the shoulders, flaring to the hem.
    draw_cone_3d(&mut img, &mut depth, cx, 26.0, 56.0, 8.0, 13.0, &robe);
    // Collar, lighter, to separate robe from skull
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 24.0, 10.0, 9.0, 3.5, 6.0, &trim);

    // Bare skull, clear of the collar
    draw_sphere_3d(&mut img, &mut depth, cx, 14.0, 14.0, 7.0, &bone);
    // Jaw
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 19.0, 15.0, 4.5, 2.5, 4.0, &bone);

    // Eye sockets lit from within
    draw_sphere_3d(&mut img, &mut depth, cx - 2.6, 13.0, 20.0, 1.7, &soul);
    draw_sphere_3d(&mut img, &mut depth, cx + 2.6, 13.0, 20.0, 1.7, &soul);

    // The soul it runs on, carried where the heart was
    draw_sphere_3d(&mut img, &mut depth, cx, 33.0, 16.0, 4.5, &soul);

    // Skeletal arms framing it
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 10.0,
        28.0,
        42.0,
        12.0,
        2.0,
        &bone,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 10.0,
        28.0,
        42.0,
        12.0,
        2.0,
        &bone,
    );

    img
}

/// Grave Hulk - corpse-built juggernaut
///
/// The mass the skeleton and zombie lack: a wide, low silhouette stitched from
/// more than one body, with mismatched arms because it was assembled rather
/// than raised.
pub fn create_grave_hulk_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    // `Material::flesh` renders very dark — the shipped zombie, which uses it,
    // is close to a silhouette at 64px. Matte keeps the mass readable.
    let flesh = Material::matte(186, 172, 138);
    let graft = Material::matte(146, 158, 116);
    let bone = Material::matte(228, 222, 200);
    let eye = Material::glowing(255, 200, 70);

    draw_shadow(&mut img, cx, 60.0, 17.0, 6.0);

    // Bulk: deliberately wider than tall, unlike every other undead here
    draw_ellipsoid_3d(
        &mut img, &mut depth, cx, 44.0, 16.0, 17.0, 12.0, 12.0, &flesh,
    );
    // A second torso grafted on, offset and a different shade so the join reads
    draw_ellipsoid_3d(
        &mut img,
        &mut depth,
        cx - 5.0,
        31.0,
        11.0,
        11.0,
        8.0,
        9.0,
        &graft,
    );

    // Small head sunk between the shoulders
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 22.0, 14.0, 6.5, &graft);
    draw_sphere_3d(&mut img, &mut depth, cx + 1.0, 21.0, 20.0, 1.5, &eye);
    draw_sphere_3d(&mut img, &mut depth, cx + 5.5, 21.0, 20.0, 1.5, &eye);

    // Mismatched arms: one heavy and fleshed, one stripped to bone
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx - 16.0,
        34.0,
        54.0,
        12.0,
        5.0,
        &flesh,
    );
    draw_cylinder_3d(
        &mut img,
        &mut depth,
        cx + 16.0,
        32.0,
        52.0,
        12.0,
        2.5,
        &bone,
    );

    // Bone accents carry the read. Large ellipsoids shade almost black in this
    // renderer — the shipped zombie has the same problem — while spheres and
    // thin cylinders light properly, so the character has to come from the
    // pale bits rather than the mass.
    draw_sphere_3d(&mut img, &mut depth, cx + 12.0, 30.0, 18.0, 4.5, &bone);
    for rib in 0..4u32 {
        let y = 33.0 + rib as f32 * 4.5;
        draw_sphere_3d(&mut img, &mut depth, cx - 9.0, y, 20.0, 2.2, &bone);
    }
    // A jaw, jutting, so the head is findable at a glance
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 26.0, 19.0, 3.0, &bone);

    img
}
