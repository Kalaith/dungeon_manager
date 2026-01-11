//! UI core - colors, fonts, and layout constants

use macroquad::prelude::*;

// Tile rendering constants
pub const TILE_WIDTH: f32 = 64.0;
pub const TILE_HEIGHT: f32 = 32.0;

// UI layout constants
pub const HUD_HEIGHT: f32 = 60.0;
pub const SIDEBAR_WIDTH: f32 = 250.0;

// Color scheme (using macroquad-toolkit dark theme style)
pub mod colors {
    use macroquad::prelude::*;

    pub const BACKGROUND: Color = Color::new(0.08, 0.08, 0.12, 1.0);
    pub const PANEL: Color = Color::new(0.12, 0.12, 0.16, 1.0);
    pub const TEXT: Color = Color::new(0.9, 0.9, 0.95, 1.0);
    pub const TEXT_DIM: Color = Color::new(0.6, 0.6, 0.65, 1.0);
    pub const ACCENT: Color = Color::new(0.4, 0.6, 1.0, 1.0);
    pub const POSITIVE: Color = Color::new(0.3, 0.8, 0.4, 1.0);
    pub const WARNING: Color = Color::new(1.0, 0.7, 0.2, 1.0);
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

/// Draw a simple isometric tile
pub fn draw_iso_tile(x: f32, y: f32, width: f32, height: f32, color: Color) {
    // Draw diamond shape for isometric tile
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    let top = (x, y - half_height);
    let right = (x + half_width, y);
    let bottom = (x, y + half_height);
    let left = (x - half_width, y);

    // Fill
    draw_triangle(top.into(), right.into(), bottom.into(), color);
    draw_triangle(top.into(), left.into(), bottom.into(), color);

    // Outline
    let outline_color = Color::new(0.0, 0.0, 0.0, 0.3);
    draw_line(top.0, top.1, right.0, right.1, 1.0, outline_color);
    draw_line(right.0, right.1, bottom.0, bottom.1, 1.0, outline_color);
    draw_line(bottom.0, bottom.1, left.0, left.1, 1.0, outline_color);
    draw_line(left.0, left.1, top.0, top.1, 1.0, outline_color);
}
