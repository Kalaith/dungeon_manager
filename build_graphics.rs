//! Procedural graphics generator for Deep Dominion
//!
//! This is the main entry point for generating all game graphics.
//! The actual generation code is organized into modules under `graphics_gen/`:
//!
//! - `core`: 3D rendering infrastructure (materials, depth buffer, primitives)
//! - `tiles`: Floor and terrain tile generation
//! - `monsters`: Dungeon creature sprite generation
//! - `heroes`: Hero and adventurer sprite generation
//! - `buildings`: Hero-side building sprite generation
//! - `projectiles`: Projectile and spell effect sprite generation

mod graphics_gen;

fn main() {
    println!("=== Deep Dominion Graphics Generator ===\n");

    // Create all asset directories
    create_asset_directories();

    // Generate all graphics using the modular system
    graphics_gen::generate_all_graphics();

    println!("\n=== All graphics generated successfully! ===");
}

fn create_asset_directories() {
    let directories = [
        "assets/tiles",
        "assets/tiles/hero_buildings",
        "assets/sprites/monsters",
        "assets/sprites/heroes",
        "assets/sprites/projectiles",
    ];

    for dir in directories {
        std::fs::create_dir_all(dir).unwrap();
        println!("Created directory: {}", dir);
    }
    println!();
}
