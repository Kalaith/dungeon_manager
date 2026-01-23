//! Minimap rendering module
//!
//! Handles drawing the minimap in the corner of the screen.

use macroquad::prelude::*;
use crate::state::game_state::GameState;
use crate::state::Ownership;
use crate::state::tile_state::{TilePos, FogState};
use crate::data::GameData;

/// Draw the minimap showing the dungeon layout
pub fn draw_minimap(state: &GameState, game_data: &Option<GameData>) {
    if game_data.is_none() { return; }

    let map_width = 150.0;
    let map_height = 150.0;
    let padding = 10.0;
    let start_x = screen_width() - map_width - padding;
    let start_y = screen_height() - map_height - padding;

    // Background
    draw_rectangle(start_x, start_y, map_width, map_height, Color::new(0.0, 0.0, 0.0, 0.8));
    draw_rectangle_lines(start_x, start_y, map_width, map_height, 2.0, WHITE);

    // Get grid dims
    let (grid_w, grid_h) = crate::engine::tile_grid::get_grid_dimensions(&state.dungeon.grid);
    if grid_w == 0 || grid_h == 0 { return; }

    let tile_w = map_width / grid_w as f32;
    let tile_h = map_height / grid_h as f32;

    // Draw tiles (simplified)
    for y in 0..grid_h {
        for x in 0..grid_w {
            let pos = TilePos::new(x as i32, y as i32);
            if let Some(tile) = state.get_tile(pos) {
                 // Check Fog of War
                let config_enabled = game_data.as_ref().map(|gd| gd.config.fog_of_war.enabled).unwrap_or(true);
                let fog_enabled = config_enabled && state.cheat_fog_enabled;

                let fog_state = if fog_enabled {
                    tile.fog_state
                } else {
                    FogState::Visible
                };

                if fog_state == FogState::Hidden {
                    continue; // Draw nothing (black background)
                }

                let mut color = if tile.tile_type == "water" {
                    BLUE
                } else if tile.tile_type == "lava" {
                    RED
                } else if tile.tile_type == "earth" {
                    DARKGRAY
                } else if tile.tile_type == "gold_vein" {
                    GOLD
                } else if tile.tile_type == "claimed_floor" {
                    if tile.ownership == Ownership::Player {
                        GREEN
                    } else {
                        GRAY
                    }
                } else if crate::engine::tile_types::is_wall(&tile.tile_type, game_data.as_ref().unwrap()) {
                     Color::new(0.2, 0.2, 0.2, 1.0)
                } else {
                    LIGHTGRAY
                };

                // Specific room colors
                if tile.room_id.is_some() && tile.ownership == Ownership::Player {
                    color = Color::new(0.0, 0.8, 0.0, 1.0); // Bright green for rooms
                }

                if tile.tile_type == "dungeon_heart" {
                     color = PURPLE;
                }

                if fog_state == FogState::Revealed {
                     // Dim visited but currently unseen areas
                     color.r *= 0.5;
                     color.g *= 0.5;
                     color.b *= 0.5;
                }

                draw_rectangle(
                    start_x + x as f32 * tile_w,
                    start_y + y as f32 * tile_h,
                    tile_w,
                    tile_h,
                    color
                );
            }
        }
    }

    // Draw Camera Viewport Rect
    let cam_x = state.camera.target.0; // In world units (roughly tile coords)
    let cam_z = state.camera.target.2;

    // Approximate visible area (zoom dependant, but let's assume specific fixed size for now or just a dot)
    // Camera sees roughly 20x20 tiles depending on zoom
    let view_w = 20.0 * tile_w;
    let view_h = 20.0 * tile_h;

    let cam_map_x = start_x + (cam_x / grid_w as f32) * map_width;
    let cam_map_y = start_y + (cam_z / grid_h as f32) * map_height;

    draw_rectangle_lines(
        cam_map_x - view_w / 2.0,
        cam_map_y - view_h / 2.0,
        view_w,
        view_h,
        1.0,
        WHITE
    );

    // Draw Hero Base (only if not under fog of war)
    if state.hero_base.enabled {
        let config_enabled = game_data.as_ref().map(|gd| gd.config.fog_of_war.enabled).unwrap_or(true);
        let fog_enabled = config_enabled && state.cheat_fog_enabled;
        
        // Check if hero base area is revealed
        let base_pos = state.hero_base.position;
        let base_visible = if fog_enabled {
            // Check if any tile near base is visible/revealed
            if let Some(tile) = state.get_tile(base_pos) {
                tile.fog_state != FogState::Hidden
            } else {
                false
            }
        } else {
            true // No fog, always visible
        };
        
        if base_visible {
            let bx = start_x + (base_pos.x as f32 / grid_w as f32) * map_width;
            let by = start_y + (base_pos.y as f32 / grid_h as f32) * map_height;
            draw_circle(bx, by, 3.0, RED);
        }
    }
}
