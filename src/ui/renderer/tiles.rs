//! Map tile rendering.
//!
//! Lifted out of `renderer.rs` when that file reached 792 lines against the
//! 800-line hard limit. `draw_tiles` was a 339-line method that never touched
//! `self`, so it moves out as a free function unchanged.

use crate::data::GameData;
use crate::engine::lighting::LightMap;
use crate::engine::tile_types;
use crate::state::game_state::GameState;
use crate::state::tile_state::{FogState, TilePos};
use crate::state::{DragSelection, InteractionMode, Ownership};
use crate::ui::resources::GraphicsCache;
use macroquad::prelude::*;

pub(super) fn draw_tiles(
    graphics: &GraphicsCache,
    state: &GameState,
    interaction_mode: &InteractionMode,
    hovered_tile: Option<TilePos>,
    game_data: &GameData,
    drag_selection: &DragSelection,
    light_map: &LightMap,
) {
    // Draw grid (Tiles)
    for row in &state.dungeon.grid {
        for tile in row {
            // In 3D: x = x, y = height (0), z = y
            let pos_x = tile.pos.x as f32;
            let pos_z = tile.pos.y as f32;

            // Optimization: simple frustum cull check (distance from camera target)
            let dx = pos_x - state.camera.target.0;
            let dz = pos_z - state.camera.target.2;
            if dx * dx + dz * dz > 2500.0 {
                // Radius 50
                continue;
            }

            // Get texture for this tile type. `barracks` names both a
            // player room and a hero-base building, so ownership is what
            // picks the art — the tile type alone cannot tell them apart.
            let texture_opt = if tile.ownership == Ownership::Enemy {
                graphics
                    .building_texture(&tile.tile_type)
                    .or_else(|| graphics.tile_textures.get(&tile.tile_type))
            } else {
                graphics
                    .tile_textures
                    .get(&tile.tile_type)
                    .or_else(|| graphics.building_texture(&tile.tile_type))
            };

            // Determine visible color based on Fog of War settings
            let fog_enabled = game_data.config.fog_of_war.enabled && state.cheat_fog_enabled;

            let fog_state = if fog_enabled {
                tile.fog_state
            } else {
                FogState::Visible
            };

            // Skip rendering hidden tiles entirely (no black placeholder)
            // BUT still draw dig markers so player can see queued digs
            if fog_state == FogState::Hidden {
                // Draw dig marker wireframe on hidden tiles
                if tile.marked_for_dig {
                    let marker_color = Color::new(1.0, 0.0, 0.0, 0.6); // Red dig marker
                    draw_cube_wires(vec3(pos_x, 0.5, pos_z), vec3(1.0, 1.0, 1.0), marker_color);
                }
                continue;
            }

            let color = match fog_state {
                FogState::Hidden => crate::ui::core::colors::FOG_HIDDEN, // Unreachable now
                FogState::Revealed => crate::ui::core::colors::FOG_REVEALED,
                FogState::Visible => crate::ui::core::colors::FOG_VISIBLE,
            };

            // Tint by the dungeon's own light. Every room and the three glowing
            // tile types have authored a colour and intensity since long before
            // anything read them.
            let lit = light_map.multiplier_at(tile.pos);
            let color = Color::new(
                color.r * lit[0],
                color.g * lit[1],
                color.b * lit[2],
                color.a,
            );

            // Removed simple tint for marked tiles to use overlay instead

            let is_wall = tile_types::is_wall(&tile.tile_type, game_data);

            if let Some(texture) = texture_opt {
                if is_wall {
                    draw_cube(
                        vec3(pos_x, 0.5, pos_z),
                        vec3(1.0, 1.0, 1.0),
                        Some(texture),
                        color,
                    );
                } else {
                    // Render floor as a thick block (0.5 units thick, extending down from 0.0)
                    draw_cube(
                        vec3(pos_x, -0.25, pos_z),
                        vec3(1.0, 0.5, 1.0),
                        Some(texture),
                        color,
                    );
                }

                // Draw Dig Marker Overlay
                if tile.marked_for_dig {
                    // 1. Darken the tile slightly with a semi-transparent black box to make the marker pop
                    // and to indicate "pending change"
                    let overlay_color = Color::new(0.0, 0.0, 0.0, 0.4);
                    let (y_pos, size) = if is_wall {
                        (0.5, vec3(1.01, 1.01, 1.01)) // Slightly larger to avoid z-fighting
                    } else {
                        (0.01, vec3(0.9, 0.1, 0.9)) // Flat on floor
                    };

                    draw_cube(vec3(pos_x, y_pos, pos_z), size, None, overlay_color);

                    // 2. Draw a bright "X" or box wireframe
                    let marker_color = Color::new(1.0, 0.0, 0.0, 0.8); // Bright Red
                    draw_cube_wires(vec3(pos_x, y_pos, pos_z), size, marker_color);
                }

                // Draw trap if present
                if let Some(trap) = &tile.trap {
                    let is_constructing = !trap.constructed;
                    let trap_color = if is_constructing {
                        Color::new(1.0, 1.0, 1.0, 0.3) // Very transparent for "ghost"
                    } else {
                        WHITE
                    };

                    let is_door = matches!(
                        trap.trap_type.as_str(),
                        "door" | "braced_door" | "magic_door"
                    );

                    if let Some(trap_texture) = graphics.tile_textures.get(&trap.trap_type) {
                        if is_door {
                            // A door fills the corridor it blocks, so it
                            // keeps the full block the untextured fallback
                            // always drew rather than lying flat like a trap.
                            draw_cube(
                                vec3(pos_x, 0.5, pos_z),
                                vec3(1.0, 1.0, 1.0),
                                Some(trap_texture),
                                trap_color,
                            );
                        } else {
                            // Draw trap slightly above floor, smaller size to fit well
                            draw_plane(
                                vec3(pos_x, 0.05, pos_z),
                                vec2(0.6, 0.6), // Reduced from 0.8
                                Some(trap_texture),
                                trap_color,
                            );
                        }
                    } else {
                        // Fallback if texture missing (e.g. doors)
                        // Draw a colored box - User requested "full square" for now
                        let (fallback_color, size) = match trap.trap_type.as_str() {
                            "door" | "braced_door" | "magic_door" => (
                                Color::new(0.4, 0.2, 0.1, if is_constructing { 0.3 } else { 1.0 }),
                                vec3(1.0, 1.0, 1.0),
                            ), // Full block for door
                            "spike_trap" => (
                                Color::new(0.5, 0.5, 0.5, if is_constructing { 0.3 } else { 1.0 }),
                                vec3(1.0, 0.1, 1.0),
                            ), // Full floor tile for spikes
                            _ => (
                                Color::new(0.8, 0.2, 0.2, if is_constructing { 0.3 } else { 1.0 }),
                                vec3(1.0, 0.2, 1.0),
                            ), // Generic
                        };

                        draw_cube(
                            // Adjust Y so it sits on floor (floor is at -0.25 with height 0.5, so top is at 0.0)
                            // We want this to sit on top of 0.0.
                            // draw_cube position is center. So y should be size.y/2.0
                            vec3(pos_x, size.y / 2.0, pos_z),
                            size,
                            None,
                            fallback_color,
                        );
                    }
                }
            } else {
                // Fallback to colored plane/cube if texture not found
                let mut tile_color = crate::ui::core::get_tile_color(&tile.tile_type);

                // Apply fog/tint to tile_color
                let fog_state = if game_data.config.fog_of_war.enabled {
                    tile.fog_state
                } else {
                    FogState::Visible
                };

                match fog_state {
                    FogState::Hidden => tile_color = crate::ui::core::colors::FOG_HIDDEN,
                    FogState::Revealed => {
                        tile_color.r *= 0.5;
                        tile_color.g *= 0.5;
                        tile_color.b *= 0.5;
                    }
                    FogState::Visible => {}
                }
                // Removed simple tint for marked tiles

                if is_wall {
                    draw_cube(
                        vec3(pos_x, 0.5, pos_z),
                        vec3(1.0, 1.0, 1.0),
                        None,
                        tile_color,
                    );
                } else {
                    draw_cube(
                        vec3(pos_x, -0.25, pos_z),
                        vec3(1.0, 0.5, 1.0),
                        None,
                        tile_color,
                    );
                }

                // Draw Dig Marker Overlay (Same as textured)
                if tile.marked_for_dig {
                    let overlay_color = Color::new(0.0, 0.0, 0.0, 0.4);
                    let (y_pos, size) = if is_wall {
                        (0.5, vec3(1.01, 1.01, 1.01))
                    } else {
                        (0.01, vec3(0.9, 0.1, 0.9))
                    };

                    draw_cube(vec3(pos_x, y_pos, pos_z), size, None, overlay_color);

                    // 2. Draw a bright "X" or box wireframe
                    let marker_color = Color::new(1.0, 0.0, 0.0, 0.8); // Bright Red
                    draw_cube_wires(vec3(pos_x, y_pos, pos_z), size, marker_color);
                }
            }

            // Draw selection outline if this is the hovered tile AND it's a valid target
            if let Some(hovered_pos) = hovered_tile {
                if tile.pos == hovered_pos && !drag_selection.active {
                    let mut outline_color = None;

                    match interaction_mode {
                        InteractionMode::None => {
                            // Generic selection
                            outline_color = Some(Color::new(1.0, 1.0, 1.0, 0.5));
                        }
                        InteractionMode::Dig => {
                            // Valid dig targets: earth, gold, gems
                            // Also allow marked tiles to be selected (to unmark)
                            if tile_types::is_diggable(&tile.tile_type, game_data) {
                                outline_color = Some(crate::ui::core::colors::ACCENT);
                            }
                        }
                        InteractionMode::BuildRoom(_) => {
                            // Valid build targets: claimed floors owned by player
                            if tile.ownership == Ownership::Player
                                && tile_types::can_build_room(&tile.tile_type, game_data)
                            {
                                outline_color = Some(crate::ui::core::colors::POSITIVE);
                            }
                        }
                        InteractionMode::BuildTrap(_) => {
                            if tile.ownership == Ownership::Player
                                && tile_types::can_build_room(&tile.tile_type, game_data)
                                && tile.trap.is_none()
                            {
                                outline_color = Some(crate::ui::core::colors::POSITIVE);
                            }
                        }
                        InteractionMode::Summon(_, _, _) => {
                            outline_color = Some(GREEN);
                        }
                        InteractionMode::PlaceSpawner => {
                            // Valid spawner location: claimed floors
                            if tile.ownership == Ownership::Player
                                && tile_types::can_build_room(&tile.tile_type, game_data)
                            {
                                outline_color = Some(crate::ui::core::colors::POSITIVE);
                            }
                        }
                        InteractionMode::Pickup => {
                            // Highlight any entity's tile
                            outline_color = Some(Color::new(0.0, 1.0, 0.0, 0.5));
                        }
                        InteractionMode::Drop => {
                            // Highlight any tile
                            outline_color = Some(Color::new(1.0, 1.0, 0.0, 0.5));
                        }
                        InteractionMode::Sell => {
                            // Highlight any markable tile or owned room
                            if tile.marked_for_dig
                                || (tile.ownership == Ownership::Player && tile.room_id.is_some())
                            {
                                outline_color = Some(crate::ui::core::colors::NEGATIVE);
                            }
                        }
                        InteractionMode::Inspect => {
                            outline_color = Some(Color::new(0.0, 0.5, 1.0, 0.5));
                        }
                        InteractionMode::SetAttackMarker => {
                            outline_color = Some(Color::new(0.8, 0.2, 0.2, 0.5));
                        }
                        InteractionMode::SetDefendMarker => {
                            outline_color = Some(Color::new(0.2, 0.2, 0.8, 0.5));
                        }
                        InteractionMode::SaveGame => {}
                    }

                    if let Some(color) = outline_color {
                        // Draw selection wireframe
                        draw_cube_wires(vec3(pos_x, 0.05, pos_z), vec3(1.0, 0.0, 1.0), color);
                    }
                }
            }
        }
    }

    // Draw drag selection preview - single rectangle around entire selection
    if drag_selection.active {
        if let Some((min, max)) = drag_selection.get_selection_rect() {
            // Calculate center and size of the selection rectangle
            let center_x = (min.x as f32 + max.x as f32) / 2.0;
            let center_z = (min.y as f32 + max.y as f32) / 2.0;
            let width = (max.x - min.x + 1) as f32; // +1 because inclusive
            let depth = (max.y - min.y + 1) as f32; // +1 because inclusive

            // Draw a single wireframe rectangle around the entire selection
            // Match wall/dirt tile dimensions: y=0.5 center with height 1.0
            draw_cube_wires(
                vec3(center_x, 0.5, center_z),
                vec3(width, 1.0, depth),
                Color::new(0.0, 1.0, 0.0, 1.0), // Bright green
            );
        }
    }
}
