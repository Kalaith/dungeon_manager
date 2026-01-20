//! Hero sprite generation
//!
//! Generates all hero character sprites using 3D shading.

use image::RgbaImage;
use super::core::*;

// ============================================================================
// TIER 1 HEROES - Basic Units
// ============================================================================

pub fn create_peasant_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let cloth_mat = Material::matte(160, 140, 100);
    let skin_mat = Material::matte(230, 190, 150);
    let wood_mat = Material::matte(139, 90, 43);
    let metal_mat = Material::metallic(169, 169, 169);

    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 42.0, 5.0, 9.0, 12.0, 8.0, &cloth_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 8.0, 7.0, &skin_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 14.0, 50.0, 4.0, 2.0, &wood_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 10.0, 10.0, 18.0, 5.0, 1.0, &metal_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 10.0, 18.0, 5.0, 1.0, &metal_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 18.0, 10.0, 18.0, 5.0, 1.0, &metal_mat);

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
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 5.0, 9.0, 13.0, 7.0, &leather_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 8.0, 7.0, &skin_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 6.0, 8.0, &leather_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 18.0, 48.0, 4.0, 2.0, &wood_mat);

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
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 42.0, 9.0, 5.0, 2.0, 2.0, &gold_mat);

    img
}

// ============================================================================
// TIER 2 HEROES - Standard Units
// ============================================================================

pub fn create_knight_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let armor_mat = Material::metallic(180, 180, 200);
    let plume_mat = Material::matte(200, 0, 0);
    let shield_mat = Material::metallic(100, 100, 120);
    let sword_mat = Material::metallic(220, 220, 220);

    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 6.0, 11.0, 15.0, 10.0, &armor_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 9.0, 9.0, &armor_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 8.0, 20.0, 8.0, 3.0, &plume_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx - 16.0, 38.0, 10.0, 7.0, 10.0, 5.0, &shield_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 14.0, 48.0, 8.0, 2.0, &sword_mat);

    img
}

pub fn create_archer_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let leather_mat = Material::leather(120, 100, 70);
    let skin_mat = Material::matte(230, 190, 150);
    let wood_mat = Material::matte(139, 90, 43);
    let metal_mat = Material::metallic(169, 169, 169);

    draw_shadow(&mut img, cx, 56.0, 10.0, 4.0);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 5.0, 9.0, 13.0, 8.0, &leather_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 8.0, 7.0, &skin_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 16.0, 14.0, 50.0, 4.0, 2.0, &wood_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 10.0, 28.0, 30.0, 6.0, 1.0, &wood_mat);
    draw_cone_3d(&mut img, &mut depth, cx - 4.0, 27.0, 30.0, 7.0, 2.0, &metal_mat);

    img
}

pub fn create_battle_cleric_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let armor_mat = Material::metallic(200, 200, 220);
    let skin_mat = Material::matte(230, 190, 150);
    let gold_mat = Material::metallic(255, 215, 0);
    let wood_mat = Material::matte(139, 90, 43);
    let mace_mat = Material::metallic(150, 150, 150);

    draw_shadow(&mut img, cx, 58.0, 11.0, 5.0);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 6.0, 11.0, 14.0, 9.0, &armor_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 8.0, 7.0, &skin_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 36.0, 48.0, 8.0, 2.0, &gold_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 9.0, 5.0, 2.0, 2.0, &gold_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 18.0, 26.0, 50.0, 5.0, 2.0, &wood_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 18.0, 22.0, 8.0, 5.0, &mace_mat);

    img
}

pub fn create_rogue_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let cloak_mat = Material::matte(60, 60, 80);
    let hood_mat = Material::matte(50, 50, 70);
    let eye_mat = Material::glowing(255, 255, 0);
    let dagger_mat = Material::metallic(192, 192, 192);

    draw_shadow(&mut img, cx, 56.0, 9.0, 4.0);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 40.0, 5.0, 9.0, 13.0, 7.0, &cloak_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 7.0, 8.0, &hood_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 24.0, 10.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 24.0, 10.0, 2.0, &eye_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 32.0, 48.0, 5.0, 1.0, &dagger_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 14.0, 32.0, 48.0, 5.0, 1.0, &dagger_mat);

    img
}

pub fn create_barbarian_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let skin = Material::matte(200, 140, 100);
    let leather = Material::leather(120, 80, 40);
    let metal = Material::metallic(180, 180, 180);
    let hair_mat = Material::matte(60, 40, 20);

    let cx = SPRITE_SIZE as f32 / 2.0;

    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    // Muscular body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 7.0, 12.0, 15.0, 10.0, &skin);
    // Kilt
    draw_cylinder_3d(&mut img, &mut depth, cx, 46.0, 56.0, 6.0, 10.0, &leather);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 18.0, 10.0, 8.0, &skin);
    // Wild hair
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 14.0, 8.0, 10.0, 6.0, 8.0, &hair_mat);
    // Huge axe on back
    draw_cylinder_3d(&mut img, &mut depth, cx + 10.0, 10.0, 52.0, 5.0, 2.0, &leather);
    draw_ellipsoid_3d(&mut img, &mut depth, cx + 14.0, 16.0, 8.0, 5.0, 10.0, 2.0, &metal);

    img
}

pub fn create_alchemist_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let robe = Material::matte(40, 140, 80);
    let glass = Material::glowing(100, 255, 100);
    let skin_mat = Material::matte(230, 190, 150);

    let cx = SPRITE_SIZE as f32 / 2.0;

    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);
    // Body
    draw_cylinder_3d(&mut img, &mut depth, cx, 30.0, 56.0, 5.0, 10.0, &robe);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 22.0, 8.0, 7.0, &skin_mat);
    // Goggles
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 20.0, 10.0, 8.0, 3.0, 4.0, &Material::metallic(100, 80, 60));
    // Flask in hand
    draw_sphere_3d(&mut img, &mut depth, cx + 12.0, 36.0, 10.0, 4.0, &glass);
    // Belt with vials
    draw_cylinder_3d(&mut img, &mut depth, cx, 42.0, 46.0, 7.0, 11.0, &Material::leather(80, 60, 40));

    img
}

// ============================================================================
// TIER 3 HEROES - Elite Units
// ============================================================================

pub fn create_paladin_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let armor_mat = Material::metallic(220, 200, 100);
    let glow_mat = Material::glowing(255, 255, 200);
    let sword_mat = Material::glowing(255, 255, 255);
    let shield_mat = Material::metallic(200, 200, 220);
    let gold_mat = Material::metallic(255, 215, 0);

    draw_shadow(&mut img, cx, 58.0, 13.0, 5.0);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 7.0, 12.0, 16.0, 11.0, &armor_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 10.0, 10.0, &armor_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 34.0, -5.0, 18.0, &glow_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 18.0, 10.0, 48.0, 9.0, 2.0, &sword_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx - 18.0, 36.0, 10.0, 8.0, 11.0, 6.0, &shield_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 18.0, 30.0, 42.0, 12.0, 1.0, &gold_mat);

    img
}

pub fn create_wizard_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe_mat = Material::matte(50, 80, 200);
    let hat_mat = Material::matte(40, 70, 180);
    let skin_mat = Material::matte(230, 190, 150);
    let wood_mat = Material::matte(139, 90, 43);
    let orb_mat = Material::glowing(100, 200, 255);
    let star_mat = Material::glowing(255, 255, 0);

    draw_shadow(&mut img, cx, 58.0, 10.0, 4.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 36.0, 56.0, 5.0, 10.0, &robe_mat);
    draw_cone_3d(&mut img, &mut depth, cx, 6.0, 28.0, 6.0, 8.0, &hat_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 28.0, 8.0, 6.0, &skin_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 4.0, 44.0, 8.0, 2.0, &star_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 4.0, 48.0, 8.0, 2.0, &star_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 14.0, 10.0, 54.0, 4.0, 2.0, &wood_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 14.0, 8.0, 8.0, 6.0, &orb_mat);

    img
}

pub fn create_inquisitor_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe_mat = Material::matte(140, 30, 30);
    let hood_mat = Material::matte(60, 10, 10);
    let eye_mat = Material::glowing(255, 0, 0);
    let sword_mat = Material::metallic(220, 220, 220);
    let fire_mat = Material::glowing(255, 100, 0);

    draw_shadow(&mut img, cx, 58.0, 11.0, 4.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 56.0, 5.0, 11.0, &robe_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 7.0, 8.0, &hood_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 24.0, 11.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 24.0, 11.0, 2.0, &eye_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 14.0, 48.0, 8.0, 2.0, &sword_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 16.0, 12.0, 10.0, 5.0, &fire_mat);

    img
}

pub fn create_geomancer_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let rock_armor = Material::stone(100, 90, 80);
    let crystal = Material::glowing(200, 100, 200);

    let cx = SPRITE_SIZE as f32 / 2.0;

    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    // Bulky Rock Armor Body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 8.0, 12.0, 16.0, 10.0, &rock_armor);
    // Shoulders
    draw_sphere_3d(&mut img, &mut depth, cx - 10.0, 30.0, 12.0, 5.0, &rock_armor);
    draw_sphere_3d(&mut img, &mut depth, cx + 10.0, 30.0, 12.0, 5.0, &rock_armor);
    // Head
    draw_sphere_3d(&mut img, &mut depth, cx, 18.0, 10.0, 7.0, &rock_armor);
    // Floating crystals
    draw_sphere_3d(&mut img, &mut depth, cx - 8.0, 8.0, 15.0, 3.0, &crystal);
    draw_sphere_3d(&mut img, &mut depth, cx + 8.0, 12.0, 12.0, 2.5, &crystal);
    draw_sphere_3d(&mut img, &mut depth, cx, 6.0, 18.0, 4.0, &crystal);

    img
}

// ============================================================================
// TIER 4 HEROES - Commander Units
// ============================================================================

pub fn create_knight_commander_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let armor_mat = Material::metallic(200, 200, 220);
    let plume_mat = Material::matte(180, 0, 0);
    let cape_mat = Material::matte(150, 0, 0);
    let wood_mat = Material::matte(139, 90, 43);
    let banner_mat = Material::metallic(255, 215, 0);

    draw_shadow(&mut img, cx, 58.0, 13.0, 5.0);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 7.0, 12.0, 16.0, 11.0, &armor_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 20.0, 10.0, 10.0, &armor_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx, 4.0, 18.0, 9.0, 4.0, &plume_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 10.0, 32.0, 56.0, -2.0, 8.0, &cape_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 16.0, 6.0, 52.0, 5.0, 2.0, &wood_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx + 22.0, 12.0, 6.0, 8.0, 6.0, 3.0, &banner_mat);

    img
}

pub fn create_high_priest_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe_mat = Material::matte(240, 230, 200);
    let gold_mat = Material::metallic(255, 215, 0);
    let skin_mat = Material::matte(230, 190, 150);
    let orb_mat = Material::glowing(255, 255, 255);

    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 56.0, 5.0, 12.0, &robe_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 16.0, 8.0, 12.0, 5.0, 6.0, &gold_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 26.0, 9.0, 6.0, &skin_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 16.0, 8.0, 54.0, 5.0, 2.0, &gold_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 16.0, 6.0, 9.0, 7.0, &orb_mat);

    img
}

pub fn create_archmage_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let robe_mat = Material::matte(100, 50, 200);
    let hat_mat = Material::matte(80, 40, 180);
    let aura_mat = Material::glowing(150, 100, 255);
    let wood_mat = Material::matte(139, 90, 43);
    let energy_mat = Material::glowing(200, 150, 255);

    draw_sphere_3d(&mut img, &mut depth, cx, 36.0, -8.0, 20.0, &aura_mat);

    draw_shadow(&mut img, cx, 58.0, 12.0, 5.0);
    draw_cylinder_3d(&mut img, &mut depth, cx, 34.0, 54.0, 5.0, 12.0, &robe_mat);
    draw_cone_3d(&mut img, &mut depth, cx, 4.0, 28.0, 6.0, 9.0, &hat_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 18.0, 6.0, 54.0, 4.0, 2.0, &wood_mat);
    draw_sphere_3d(&mut img, &mut depth, cx - 18.0, 4.0, 10.0, 9.0, &energy_mat);

    img
}

// ============================================================================
// TIER 5 HEROES - Boss Units
// ============================================================================

pub fn create_champion_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let armor_mat = Material::metallic(255, 215, 100);
    let divine_mat = Material::glowing(255, 255, 200);
    let sword_mat = Material::glowing(255, 255, 255);
    let shield_mat = Material::metallic(255, 240, 180);
    let gold_mat = Material::metallic(255, 215, 0);

    draw_sphere_3d(&mut img, &mut depth, cx, 32.0, -10.0, 24.0, &divine_mat);

    draw_shadow(&mut img, cx, 60.0, 14.0, 6.0);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 36.0, 8.0, 14.0, 18.0, 12.0, &armor_mat);
    draw_sphere_3d(&mut img, &mut depth, cx, 16.0, 11.0, 11.0, &armor_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 6.0, 10.0, 10.0, 2.0, 8.0, &divine_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx + 20.0, 6.0, 52.0, 10.0, 3.0, &sword_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 20.0, 4.0, 12.0, 7.0, &divine_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx - 20.0, 34.0, 12.0, 9.0, 12.0, 7.0, &shield_mat);
    draw_cylinder_3d(&mut img, &mut depth, cx - 20.0, 28.0, 42.0, 14.0, 2.0, &gold_mat);

    img
}

/// Dragon Knight boss - NEW
pub fn create_dragon_knight_sprite() -> RgbaImage {
    let mut img = RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE);
    let mut depth = DepthBuffer::new(SPRITE_SIZE, SPRITE_SIZE);
    let cx = SPRITE_SIZE as f32 / 2.0;

    let scale_armor = Material::metallic(100, 150, 100);
    let fire_mat = Material::fire();
    let sword_mat = Material::metallic(200, 180, 100);
    let eye_mat = Material::glowing(255, 150, 0);

    draw_shadow(&mut img, cx, 60.0, 14.0, 6.0);
    // Massive scaled body
    draw_ellipsoid_3d(&mut img, &mut depth, cx, 38.0, 8.0, 14.0, 18.0, 12.0, &scale_armor);
    // Dragon helm
    draw_sphere_3d(&mut img, &mut depth, cx, 18.0, 11.0, 10.0, &scale_armor);
    // Horns
    draw_cone_3d(&mut img, &mut depth, cx - 8.0, 6.0, 16.0, 10.0, 3.0, &scale_armor);
    draw_cone_3d(&mut img, &mut depth, cx + 8.0, 6.0, 16.0, 10.0, 3.0, &scale_armor);
    // Glowing eyes
    draw_sphere_3d(&mut img, &mut depth, cx - 3.0, 16.0, 16.0, 2.0, &eye_mat);
    draw_sphere_3d(&mut img, &mut depth, cx + 3.0, 16.0, 16.0, 2.0, &eye_mat);
    // Flame breath
    draw_sphere_3d(&mut img, &mut depth, cx, 24.0, 18.0, 4.0, &fire_mat);
    // Massive sword
    draw_cylinder_3d(&mut img, &mut depth, cx + 20.0, 4.0, 54.0, 10.0, 3.0, &sword_mat);
    draw_ellipsoid_3d(&mut img, &mut depth, cx + 20.0, 2.0, 12.0, 6.0, 4.0, 3.0, &fire_mat);

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
