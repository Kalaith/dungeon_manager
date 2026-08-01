//! Graphics Generation Module
//!
//! This module contains all procedural sprite generation for the dungeon manager game.
//! Each submodule handles a specific category of graphics:
//!
//! - `core`: 3D rendering infrastructure (materials, depth buffer, primitives)
//! - `tiles`: Floor and terrain tile generation
//! - `monsters`: Dungeon creature sprite generation
//! - `heroes`: Hero and adventurer sprite generation
//! - `buildings`: Hero-side building sprite generation
//! - `projectiles`: Projectile and spell effect sprite generation

// Allow unused items - these are exported for external use and future expansion
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

pub mod buildings;
pub mod core;
pub mod heroes;
pub mod icon;
pub mod monsters;
pub mod primitives;
pub mod projectiles;
pub mod tiles;

// Re-export commonly used items from core for external use
pub use core::{
    add_3d_border,
    add_carved_inset,
    add_color_variation,
    add_crack_pattern,
    add_edge_bevel,
    add_fbm_noise,
    add_gradient_overlay,
    add_noise,
    add_outline,
    add_specular_highlights,
    blend_colors,
    create_carved_block,
    create_tile_base,
    // Volumetric wall functions
    create_volumetric_wall,
    darken_color,
    draw_box_3d,
    draw_circle,
    draw_cone_3d,
    draw_cylinder_3d,
    draw_ellipsoid_3d,
    draw_line,
    draw_raised_platform,
    draw_rect,
    draw_shadow,
    draw_sphere_3d,
    draw_torus_3d,
    draw_wedge_3d,
    fbm_noise,
    lighten_color,
    turbulence,
    // Texture functions
    value_noise,
    DepthBuffer,
    GradientDirection,
    Material,
    SPRITE_SIZE,
    TILE_SIZE,
    WALL_HEIGHT,
};

/// Generate all game graphics
pub fn generate_all_graphics() {
    println!("=== Generating All Game Graphics ===\n");

    println!("--- Generating Tiles ---");
    generate_tile_sprites();

    println!("\n--- Generating Monster Sprites ---");
    generate_monster_sprites();

    println!("\n--- Generating Hero Sprites ---");
    generate_hero_sprites();

    println!("\n--- Generating Building Sprites ---");
    generate_building_sprites();

    println!("\n--- Generating Projectile Sprites ---");
    projectiles::generate_projectile_sprites();

    println!(
        "
--- Generating Window Icon ---"
    );
    generate_window_icon();

    println!("\n=== Graphics Generation Complete ===");
}

fn generate_tile_sprites() {
    // Basic terrain
    tiles::save_tile("solid_rock", tiles::create_solid_rock());
    tiles::save_tile("earth", tiles::create_earth());
    tiles::save_tile("claimed_floor", tiles::create_claimed_floor());
    tiles::save_tile("reinforced_wall", tiles::create_reinforced_wall());
    tiles::save_tile("gold_vein", tiles::create_gold_vein());
    tiles::save_tile("gem_seam", tiles::create_gem_seam());
    tiles::save_tile("mana_crystal", tiles::create_mana_crystal());
    tiles::save_tile("lava", tiles::create_lava());
    tiles::save_tile("water", tiles::create_water());
    tiles::save_tile("bridge", tiles::create_bridge());
    tiles::save_tile("gold_pile", tiles::create_gold_pile());
    tiles::save_tile("hero_entrance", tiles::create_hero_entrance());

    // Floor variations
    tiles::save_tile("corrupted_floor", tiles::create_corrupted_floor());
    tiles::save_tile("ancient_rune_floor", tiles::create_ancient_rune_floor());
    tiles::save_tile("wooden_floor", tiles::create_wooden_floor());
    tiles::save_tile("carpet", tiles::create_carpet());
    tiles::save_tile("bone_floor", tiles::create_bone_floor());
    tiles::save_tile("mosaic_floor", tiles::create_mosaic_floor());
    tiles::save_tile("marble_floor", tiles::create_marble_floor());
    tiles::save_tile("grass", tiles::create_grass());
    tiles::save_tile("sand", tiles::create_sand());
    tiles::save_tile("snow", tiles::create_snow());

    // Rooms
    tiles::save_tile("dungeon_heart", tiles::create_dungeon_heart());
    tiles::save_tile("lair", tiles::create_lair());
    tiles::save_tile("hatchery", tiles::create_hatchery());
    tiles::save_tile("treasury", tiles::create_treasury());
    tiles::save_tile("workshop", tiles::create_workshop());
    tiles::save_tile("training_room", tiles::create_training_room());
    tiles::save_tile("library", tiles::create_library());
    tiles::save_tile("prison", tiles::create_prison());
    tiles::save_tile("guard_post", tiles::create_guard_post());
    tiles::save_tile("ritual_circle", tiles::create_ritual_circle());
    tiles::save_tile("monster_spawner", tiles::create_monster_spawner());
    tiles::save_tile("graveyard", tiles::create_graveyard());
    tiles::save_tile("kennel", tiles::create_kennel());
    tiles::save_tile("dungeon_barracks", tiles::create_dungeon_barracks());
    tiles::save_tile("torture_chamber", tiles::create_torture_chamber());
    tiles::save_tile("casino", tiles::create_casino());
    tiles::save_tile("temple", tiles::create_temple());
    tiles::save_tile("scavenger", tiles::create_scavenger());
    // docs/ROOM_SET.md "Commit" set
    tiles::save_tile("vault", tiles::create_vault());
    tiles::save_tile("mana_well", tiles::create_mana_well());
    tiles::save_tile("leisure_den", tiles::create_leisure_den());
    tiles::save_tile("arcane_archive", tiles::create_arcane_archive());
    tiles::save_tile("combat_pit", tiles::create_combat_pit());
    tiles::save_tile("soul_furnace", tiles::create_soul_furnace());
    tiles::save_tile("gatehouse", tiles::create_gatehouse());

    // Traps and doors
    tiles::save_tile("door", tiles::create_door());
    tiles::save_tile("braced_door", tiles::create_braced_door());
    tiles::save_tile("magic_door", tiles::create_magic_door());
    tiles::save_tile("blowgun_trap", tiles::create_blowgun_trap());
    tiles::save_tile("spike_trap", tiles::create_spike_trap());
    tiles::save_tile("boulder_trap", tiles::create_boulder_trap());
    tiles::save_tile("alarm_trap", tiles::create_alarm_trap());
    tiles::save_tile("gas_trap", tiles::create_gas_trap());
    tiles::save_tile("lightning_trap", tiles::create_lightning_trap());
    tiles::save_tile("fire_trap", tiles::create_fire_trap());
}

fn generate_monster_sprites() {
    // Basic creatures
    monsters::save_sprite("monsters", "imp", monsters::create_imp_sprite());
    monsters::save_sprite("monsters", "goblin", monsters::create_goblin_sprite());
    monsters::save_sprite("monsters", "orc", monsters::create_orc_sprite());
    monsters::save_sprite("monsters", "warlock", monsters::create_warlock_sprite());
    monsters::save_sprite("monsters", "troll", monsters::create_troll_sprite());

    // Undead
    monsters::save_sprite("monsters", "skeleton", monsters::create_skeleton_sprite());
    monsters::save_sprite("monsters", "vampire", monsters::create_vampire_sprite());
    monsters::save_sprite("monsters", "zombie", monsters::create_zombie_sprite());
    monsters::save_sprite("monsters", "ghost", monsters::create_ghost_sprite());
    monsters::save_sprite("monsters", "lich", monsters::create_lich_sprite());
    monsters::save_sprite(
        "monsters",
        "grave_hulk",
        monsters::create_grave_hulk_sprite(),
    );

    // Demons
    monsters::save_sprite(
        "monsters",
        "demon_spawn",
        monsters::create_demon_spawn_sprite(),
    );
    monsters::save_sprite(
        "monsters",
        "bile_demon",
        monsters::create_bile_demon_sprite(),
    );
    monsters::save_sprite("monsters", "succubus", monsters::create_succubus_sprite());
    monsters::save_sprite("monsters", "hellhound", monsters::create_hellhound_sprite());
    monsters::save_sprite("monsters", "balor", monsters::create_balor_sprite());
    monsters::save_sprite(
        "monsters",
        "infernal_hound",
        monsters::create_infernal_hound_sprite(),
    );

    // Beasts
    monsters::save_sprite("monsters", "spider", monsters::create_spider_sprite());
    monsters::save_sprite("monsters", "lizard", monsters::create_lizard_sprite());
    monsters::save_sprite("monsters", "bat_swarm", monsters::create_bat_swarm_sprite());

    // Elves
    monsters::save_sprite("monsters", "dark_elf", monsters::create_dark_elf_sprite());
}

fn generate_hero_sprites() {
    // Tier 1 - Basic
    heroes::save_sprite("heroes", "peasant", heroes::create_peasant_sprite());
    heroes::save_sprite("heroes", "scout", heroes::create_scout_sprite());
    heroes::save_sprite("heroes", "acolyte", heroes::create_acolyte_sprite());

    // Tier 2 - Common
    heroes::save_sprite("heroes", "knight", heroes::create_knight_sprite());
    heroes::save_sprite("heroes", "archer", heroes::create_archer_sprite());
    heroes::save_sprite(
        "heroes",
        "battle_cleric",
        heroes::create_battle_cleric_sprite(),
    );
    heroes::save_sprite("heroes", "rogue", heroes::create_rogue_sprite());

    // Tier 3 - Uncommon
    heroes::save_sprite("heroes", "barbarian", heroes::create_barbarian_sprite());
    heroes::save_sprite("heroes", "alchemist", heroes::create_alchemist_sprite());
    heroes::save_sprite("heroes", "paladin", heroes::create_paladin_sprite());
    heroes::save_sprite("heroes", "wizard", heroes::create_wizard_sprite());

    // Tier 4 - Elite
    heroes::save_sprite("heroes", "inquisitor", heroes::create_inquisitor_sprite());
    heroes::save_sprite("heroes", "geomancer", heroes::create_geomancer_sprite());
    heroes::save_sprite(
        "heroes",
        "knight_commander",
        heroes::create_knight_commander_sprite(),
    );
    heroes::save_sprite("heroes", "high_priest", heroes::create_high_priest_sprite());
    heroes::save_sprite("heroes", "archmage", heroes::create_archmage_sprite());
    heroes::save_sprite("heroes", "champion", heroes::create_champion_sprite());

    // Tier 5 - Legendary/Boss
    heroes::save_sprite(
        "heroes",
        "dragon_knight",
        heroes::create_dragon_knight_sprite(),
    );
}

fn generate_building_sprites() {
    buildings::save_hero_building("town_hall", buildings::create_town_hall());
    buildings::save_hero_building("barracks", buildings::create_barracks());
    buildings::save_hero_building("archery_range", buildings::create_archery_range());
    buildings::save_hero_building("church", buildings::create_church());
    buildings::save_hero_building("mage_tower", buildings::create_mage_tower());
    buildings::save_hero_building("stable", buildings::create_stable());
    buildings::save_hero_building("armory", buildings::create_armory());
    buildings::save_hero_building("hero_wall", buildings::create_hero_wall());
    buildings::save_hero_building("hero_gate", buildings::create_hero_gate());
    buildings::save_hero_building("guard_tower", buildings::create_guard_tower());
    buildings::save_hero_building("blacksmith", buildings::create_blacksmith());
    buildings::save_hero_building("tavern", buildings::create_tavern());
}

/// The three sizes `miniquad::conf::Icon` requires.
fn generate_window_icon() {
    for size in [16u32, 32, 64] {
        icon::save_icon(size, icon::create_window_icon(size));
    }
}
