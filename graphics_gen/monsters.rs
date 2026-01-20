//! Monster sprite generation
//!
//! Generates all dungeon creature sprites using 3D shading.

use image::RgbaImage;
use super::core::*;

// ============================================================================
// BASIC MONSTERS
// ============================================================================

pub fn create_imp_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let body_mat = Material::matte(200, 50, 50);
    let horn_mat = Material::matte(100, 20, 20);
    let eye_mat = Material::glowing(255, 255, 0);

    draw_shadow(&mut img, cx, 54.0, 10.0, 4.0);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 5.0, 9.0, 11.0, 8.0, &body_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 8.0, 8.0, &body_mat);
    draw_cone_3d(&mut img, &mut depth, cx - 8.0, 12.0, 22.0, 6.0, 3.0, &horn_mat);
    draw_cone_3d(&mut img, &mut depth, cx + 8.0, 12.0, 22.0, 6.0, 3.0, &horn_mat);
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
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 42.0, 5.0, 11.0, 13.0, 9.0, &body_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 8.0, 10.0, &body_mat);
    draw_cone_3d(&mut img, &mut depth, cx - 12.0, 18.0, 28.0, 5.0, 4.0, &ear_mat);
    draw_cone_3d(&mut img, &mut depth, cx + 12.0, 18.0, 28.0, 5.0, 4.0, &ear_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 24.0, 12.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 24.0, 12.0, 2.0, &eye_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 20.0, 44.0, 4.0, 3.0, &wood_mat);

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
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 6.0, 13.0, 16.0, 11.0, &body_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 9.0, 10.0, &body_mat);
    draw_cone_3d(&mut img, &mut depth, cx - 5.0, 28.0, 36.0, 10.0, 2.0, &tusk_mat);
    draw_cone_3d(&mut img, &mut depth, cx + 5.0, 28.0, 36.0, 10.0, 2.0, &tusk_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 20.0, 13.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 20.0, 13.0, 2.0, &eye_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 20.0, 16.0, 48.0, 5.0, 2.0, &wood_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx + 24.0, 18.0, 8.0, 8.0, 4.0, 3.0, &axe_mat);

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
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 12.0, 54.0, 4.0, 2.0, &wood_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 14.0, 10.0, 8.0, 5.0, &crystal_mat);

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
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 42.0, 6.0, 15.0, 18.0, 13.0, &body_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 10.0, 12.0, &body_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 16.0, 32.0, 52.0, 4.0, 5.0, &body_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 32.0, 52.0, 4.0, 5.0, &body_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 5.0, 20.0, 14.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 5.0, 20.0, 14.0, 2.0, &eye_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 22.0, 18.0, 50.0, 6.0, 4.0, &club_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 22.0, 14.0, 8.0, 7.0, &club_mat);

    img
}

// ============================================================================
// UNDEAD MONSTERS
// ============================================================================

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
    draw_cylinder_3d(&mut img, &mut depth, cx - 10.0, 34.0, 50.0, 3.0, 2.0, &bone_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 10.0, 34.0, 50.0, 3.0, 2.0, &bone_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 12.0, 46.0, 7.0, 2.0, &sword_mat);

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
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 10.0, 11.0, 7.0, 4.0, 5.0, &clothes);

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
    draw_ellipsoid_3d(&mut img, &mut depth, cx + 3.0, 40.0, 5.0, 10.0, 14.0, 9.0, &flesh_mat);
    // Head (tilted)
    draw_sphere_3d(&mut img, &mut depth, cx - 2.0, 26.0, 8.0, 8.0, &flesh_mat);
    // One arm raised
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 20.0, 42.0, 5.0, 3.0, &flesh_mat);
    // Dragging arm
    draw_cylinder_3d(&mut img, &mut depth, cx - 12.0, 36.0, 56.0, 4.0, 3.0, &flesh_mat);
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
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 8.0, 10.0, 16.0, 10.0, &ghost_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 50.0, 60.0, 6.0, 8.0, &ghost_mat);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 10.0, 9.0, &ghost_mat);
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 20.0, 14.0, 2.5, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 20.0, 14.0, 2.5, &eye_mat);
    // Wispy arms
    draw_cylinder_3d(&mut img, &mut depth, cx - 12.0, 30.0, 45.0, 6.0, 4.0, &ghost_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 12.0, 30.0, 45.0, 6.0, 4.0, &ghost_mat);

    img
}

// ============================================================================
// DEMON MONSTERS
// ============================================================================

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
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 6.0, 13.0, 16.0, 11.0, &body_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 9.0, 10.0, &body_mat);
    draw_cone_3d(&mut img, &mut depth, cx - 10.0, 4.0, 18.0, 7.0, 4.0, &horn_mat);
    draw_cone_3d(&mut img, &mut depth, cx + 10.0, 4.0, 18.0, 7.0, 4.0, &horn_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 18.0, 13.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 18.0, 13.0, 2.0, &eye_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 16.0, 32.0, 48.0, 4.0, 4.0, &body_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 32.0, 48.0, 4.0, 4.0, &body_mat);
    draw_cone_3d(&mut img, &mut depth, cx - 18.0, 48.0, 56.0, 3.0, 3.0, &claw_mat);
    draw_cone_3d(&mut img, &mut depth, cx + 18.0, 48.0, 56.0, 3.0, 3.0, &claw_mat);

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
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 10.0, 18.0, 18.0, 16.0, &flab);
    // Head integrated into body
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 12.0, 10.0, &flab);
    // Horns
    draw_cone_3d(&mut img, &mut depth, cx - 10.0, 10.0, 22.0, 10.0, 3.0, &horns);
    draw_cone_3d(&mut img, &mut depth, cx + 10.0, 10.0, 22.0, 10.0, 3.0, &horns);
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
    draw_ellipsoid_3d(&mut img, &mut depth, cx - 14.0, 30.0, -4.0, 10.0, 14.0, 3.0, &wing_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx + 14.0, 30.0, -4.0, 10.0, 14.0, 3.0, &wing_mat);
    // Body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 8.0, 8.0, 14.0, 7.0, &skin_mat);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 10.0, 7.0, &skin_mat);
    // Hair
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 18.0, 8.0, 8.0, 6.0, 6.0, &hair_mat);
    // Horns
    draw_cone_3d(&mut img, &mut depth, cx - 6.0, 10.0, 18.0, 8.0, 2.0, &horn_mat);
    draw_cone_3d(&mut img, &mut depth, cx + 6.0, 10.0, 18.0, 8.0, 2.0, &horn_mat);
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 2.0, 21.0, 14.0, 1.5, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 2.0, 21.0, 14.0, 1.5, &eye_mat);
    // Tail
    draw_cylinder_3d(&mut img, &mut depth, cx - 5.0, 50.0, 60.0, 5.0, 2.0, &skin_mat);

    img
}

// ============================================================================
// BEAST MONSTERS
// ============================================================================

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
    draw_ellipsoid_3d(&mut img, &mut depth, cx - 10.0, 30.0, 16.0, 4.0, 3.0, 3.0, &fur);
    draw_ellipsoid_3d(&mut img, &mut depth, cx + 10.0, 30.0, 16.0, 4.0, 3.0, 3.0, &fur);
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

    draw_ellipsoid_3d(&mut img, &mut depth, cx, 36.0, 20.0, 12.0, 10.0, 15.0, &abdomen_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 42.0, 12.0, 8.0, &body_mat);

    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 40.0, 16.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 40.0, 16.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 6.0, 39.0, 15.0, 1.5, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 6.0, 39.0, 15.0, 1.5, &eye_mat);

    let leg_mat = Material::matte(35, 35, 40);
    draw_cylinder_3d(&mut img, &mut depth, cx - 12.0, 45.0, 58.0, 10.0, 2.0, &leg_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 40.0, 56.0, 15.0, 2.0, &leg_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 35.0, 56.0, 20.0, 2.0, &leg_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 12.0, 30.0, 58.0, 25.0, 2.0, &leg_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 12.0, 45.0, 58.0, 10.0, 2.0, &leg_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 40.0, 56.0, 15.0, 2.0, &leg_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 35.0, 56.0, 20.0, 2.0, &leg_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 12.0, 30.0, 58.0, 25.0, 2.0, &leg_mat);

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

    draw_cone_3d(&mut img, &mut depth, cx - 5.0, 40.0, 58.0, -10.0, 5.0, &scale_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 45.0, 5.0, 10.0, 18.0, 8.0, &scale_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 8.0, 48.0, 58.0, 5.0, 3.0, &shadow_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 8.0, 48.0, 58.0, 5.0, 3.0, &shadow_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 6.0, 52.0, 58.0, 15.0, 3.0, &shadow_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 6.0, 52.0, 58.0, 15.0, 3.0, &shadow_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 30.0, 8.0, 8.0, 10.0, 6.0, &scale_mat);
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
        draw_ellipsoid_3d(&mut img, &mut depth, bx - r * 2.0, by, 4.0, r * 2.0, r, 1.0, &bat_mat);
        draw_ellipsoid_3d(&mut img, &mut depth, bx + r * 2.0, by, 4.0, r * 2.0, r, 1.0, &bat_mat);
        // Eyes
        draw_sphere_3d(&mut img, &mut depth, bx - 1.0, by - 1.0, 7.0, 0.8 * scale, &eye_mat);
        draw_sphere_3d(&mut img, &mut depth, bx + 1.0, by - 1.0, 7.0, 0.8 * scale, &eye_mat);
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
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 6.0, 9.0, 13.0, 7.0, &armor_mat);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 9.0, 6.0, &skin_mat);
    // Long hair
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 28.0, 5.0, 7.0, 10.0, 5.0, &hair_mat);
    // Pointed ears
    draw_cone_3d(&mut img, &mut depth, cx - 8.0, 20.0, 26.0, 7.0, 2.0, &skin_mat);
    draw_cone_3d(&mut img, &mut depth, cx + 8.0, 20.0, 26.0, 7.0, 2.0, &skin_mat);
    // Eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 2.0, 23.0, 12.0, 1.5, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 2.0, 23.0, 12.0, 1.5, &eye_mat);
    // Dual blades
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 18.0, 46.0, 6.0, 1.5, &blade_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 18.0, 46.0, 6.0, 1.5, &blade_mat);

    img
}

// ============================================================================
// SAVE FUNCTION
// ============================================================================

pub fn save_sprite(category: &str, name: &str, img: RgbaImage) {
    let path = format!("assets/sprites/{}/{}.png", category, name);
    img.save(&path).unwrap();
    println!("Generated: {}", path);
}
