//! UI core - colors, fonts, and layout constants

use macroquad::prelude::*;

// UI layout constants
pub const HUD_HEIGHT: f32 = 60.0;

// Color scheme (using macroquad-toolkit dark theme style)
pub mod colors {
    use macroquad::prelude::*;

    pub const BACKGROUND: Color = Color::new(0.08, 0.08, 0.12, 1.0);
    pub const PANEL: Color = Color::new(0.12, 0.12, 0.16, 1.0);
    pub const TEXT: Color = Color::new(0.9, 0.9, 0.95, 1.0);
    pub const ACCENT: Color = Color::new(0.4, 0.6, 1.0, 1.0);
    pub const POSITIVE: Color = Color::new(0.3, 0.8, 0.4, 1.0);
    pub const NEGATIVE: Color = Color::new(0.9, 0.3, 0.3, 1.0);

    // Tile colors
    pub const TILE_EARTH: Color = Color::new(0.4, 0.3, 0.2, 1.0);
    pub const TILE_ROCK: Color = Color::new(0.3, 0.3, 0.35, 1.0);
    pub const TILE_CLAIMED: Color = Color::new(0.2, 0.6, 0.3, 1.0);
    pub const TILE_GOLD: Color = Color::new(0.9, 0.7, 0.2, 1.0);
    pub const TILE_WATER: Color = Color::new(0.2, 0.4, 0.8, 1.0);
    pub const TILE_LAVA: Color = Color::new(0.9, 0.3, 0.1, 1.0);

    // Fog colors
    pub const FOG_HIDDEN: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    pub const FOG_REVEALED: Color = Color::new(0.3, 0.3, 0.3, 1.0);
    pub const FOG_VISIBLE: Color = WHITE;
}

/// Get the color for a tile type
pub fn get_tile_color(tile_type: &str) -> Color {
    match tile_type {
        "earth" => colors::TILE_EARTH,
        "solid_rock" => colors::TILE_ROCK,
        "reinforced_wall" => colors::TILE_ROCK,
        "claimed_floor" => colors::TILE_CLAIMED,
        "gold_vein" => colors::TILE_GOLD,
        "gem_seam" => colors::TILE_GOLD,
        "water" => colors::TILE_WATER,
        "lava" => colors::TILE_LAVA,
        "bridge" => Color::new(0.5, 0.4, 0.3, 1.0),
        "corrupted_floor" => Color::new(0.5, 0.2, 0.6, 1.0),
        "ancient_rune_floor" => Color::new(0.5, 0.3, 0.8, 1.0),
        // Room types
        "lair" => Color::new(0.3, 0.3, 0.6, 1.0), // Blue-ish for sleeping
        "hatchery" => Color::new(0.6, 0.5, 0.2, 1.0), // Yellow-ish for food
        "treasury" => Color::new(0.8, 0.6, 0.1, 1.0), // Gold color
        "training_room" => Color::new(0.7, 0.2, 0.2, 1.0), // Red for combat
        "library" => Color::new(0.4, 0.2, 0.6, 1.0), // Purple for magic
        "workshop" => Color::new(0.5, 0.4, 0.3, 1.0), // Brown for crafting
        "dungeon_heart" => Color::new(0.8, 0.1, 0.1, 1.0), // Bright red
        "monster_spawner" => Color::new(0.6, 0.2, 0.8, 1.0), // Purple for spawner
        "stone_path" => Color::new(0.6, 0.6, 0.6, 1.0), // Gray for hero entrance
        _ => Color::new(0.5, 0.5, 0.5, 1.0),
    }
}

