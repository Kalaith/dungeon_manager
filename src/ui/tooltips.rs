//! Tooltip rendering module
//!
//! Handles drawing tooltips for tiles, entities, and drag selections.

use crate::data::GameData;
use crate::engine::tile_types;
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;
use crate::state::{DragSelection, InteractionMode, Ownership};
use crate::ui::sidebar::Sidebar;
use macroquad::prelude::*;

/// Draw tooltips for hovered tiles and entities
pub fn draw_tooltips(
    state: &GameState,
    hovered_tile: Option<TilePos>,
    game_data: &Option<GameData>,
    interaction_mode: &InteractionMode,
    drag_selection: &DragSelection,
    sidebar: &Sidebar,
    get_room_cost: impl Fn(&str, Option<&GameData>) -> i32,
) {
    if drag_selection.active {
        if let Some(data) = game_data {
            draw_drag_tooltip(
                state,
                data,
                interaction_mode,
                drag_selection,
                &get_room_cost,
            );
        }
        return;
    }

    if let Some(pos) = hovered_tile {
        // Check if mouse is over sidebar, if so, don't show tile tooltip
        if sidebar.is_mouse_over() {
            return;
        }

        if let Some(tile) = state.get_tile(pos) {
            // Determine visible color based on Fog of War settings
            let fog_enabled = game_data
                .as_ref()
                .map(|gd| gd.config.fog_of_war.enabled)
                .unwrap_or(true);
            let fog_state = if fog_enabled {
                tile.fog_state
            } else {
                crate::state::tile_state::FogState::Visible
            };

            if fog_state == crate::state::tile_state::FogState::Hidden {
                return;
            }

            let mut lines = Vec::new();

            // Tile Name
            let tile_name = if let Some(data) = game_data {
                data.tiles
                    .get(&tile.tile_type)
                    .map(|t| t.name.clone())
                    .unwrap_or(tile.tile_type.clone())
            } else {
                tile.tile_type.clone()
            };
            lines.push(tile_name);

            // Room info
            if let Some(room_id) = tile.room_id {
                if let Some(room) = state.room_manager.rooms.iter().find(|r| r.id == room_id) {
                    lines.push(format!("Room: {}", room.room_type));
                }
            }

            // Ownership
            match tile.ownership {
                Ownership::Player => lines.push("Owned (You)".to_string()),
                Ownership::Enemy => lines.push("Owned (Enemy)".to_string()),
                _ => {}
            }

            // Trap/Object
            if let Some(trap) = &tile.trap {
                let status = if trap.constructed {
                    "Active"
                } else {
                    "Building..."
                };
                if let Some(data) = game_data {
                    let trap_name = data
                        .traps
                        .get(&trap.trap_type)
                        .map(|t| t.name.clone())
                        .unwrap_or(trap.trap_type.clone());
                    lines.push(format!("{} ({})", trap_name, status));
                } else {
                    lines.push(format!("{} ({})", trap.trap_type, status));
                }
            }

            // Entities on tile
            let entities: Vec<_> = state.entities.at_position(pos).collect();
            for entity in entities {
                match &entity.entity_type {
                    crate::state::entities::EntityType::Creature(c) => {
                        let name = if let Some(data) = game_data {
                            data.monsters
                                .get(&c.creature_id)
                                .map(|m| m.name.clone())
                                .unwrap_or(c.creature_id.clone())
                        } else {
                            c.creature_id.clone()
                        };
                        lines.push(format!("{} (HP: {:.0})", name, c.health));
                    }
                    crate::state::entities::EntityType::Hero(h) => {
                        let name = if let Some(data) = game_data {
                            data.heroes
                                .get(&h.hero_id)
                                .map(|m| m.name.clone())
                                .unwrap_or(h.hero_id.clone())
                        } else {
                            h.hero_id.clone()
                        };

                        if h.is_converted {
                            lines.push(format!("Minion: {} (Lvl {})", name, h.level));
                        } else if h.is_captured {
                            lines.push(format!(
                                "CAPTURED: {} ({:.0}%)",
                                name,
                                h.conversion_progress * 100.0
                            ));
                        } else {
                            lines.push(format!("Hero: {} (Lvl {})", name, h.level));
                        }
                    }
                    crate::state::entities::EntityType::Structure(s) => {
                        lines.push(format!("Structure: {:.0} HP", s.health));
                    }
                    crate::state::entities::EntityType::ResourcePile(p) => {
                        lines.push(format!("Pile: {} (Amount: {})", p.resource_type, p.amount));
                    }
                }
            }

            // Draw the tooltip box
            let mouse_pos = mouse_position();
            let tooltip_x = mouse_pos.0 + 15.0;
            let tooltip_y = mouse_pos.1 + 15.0;

            let font_size = 18.0;
            let padding = 8.0;
            let mut max_width = 0.0f32;

            for line in &lines {
                let dims = measure_text(line, None, font_size as u16, 1.0);
                if dims.width > max_width {
                    max_width = dims.width;
                }
            }

            let box_width = max_width + padding * 2.0;
            let box_height = (font_size + 4.0) * lines.len() as f32 + padding * 2.0;

            // Adjust if going off screen
            let draw_x = if tooltip_x + box_width > screen_width() {
                tooltip_x - box_width - 30.0
            } else {
                tooltip_x
            };

            let draw_y = if tooltip_y + box_height > screen_height() {
                tooltip_y - box_height - 30.0
            } else {
                tooltip_y
            };

            draw_rectangle(
                draw_x,
                draw_y,
                box_width,
                box_height,
                Color::new(0.1, 0.1, 0.1, 0.9),
            );
            draw_rectangle_lines(draw_x, draw_y, box_width, box_height, 1.0, WHITE);

            for (i, line) in lines.iter().enumerate() {
                draw_text(
                    line,
                    draw_x + padding,
                    draw_y + padding + (i as f32 * (font_size + 4.0)) + font_size - 4.0,
                    font_size,
                    WHITE,
                );
            }
        }
    }
}

/// Draw tooltip showing cost/count for drag selection
pub fn draw_drag_tooltip(
    state: &GameState,
    game_data: &GameData,
    mode: &InteractionMode,
    drag_selection: &DragSelection,
    get_room_cost: impl Fn(&str, Option<&GameData>) -> i32,
) {
    let (min, max) = match drag_selection.get_selection_rect() {
        Some(rect) => rect,
        None => return,
    };

    // Calculate Cost
    let mut total_cost = 0;
    let mut tile_count = 0;

    let tiles: Vec<TilePos> = (min.y..=max.y)
        .flat_map(|y| (min.x..=max.x).map(move |x| TilePos::new(x, y)))
        .collect();

    for pos in tiles {
        if let Some(tile) = state.get_tile(pos) {
            // Check if tile is valid context for the action
            match mode {
                InteractionMode::Dig => {
                    // Count diggable tiles (claimed or unclaimed)
                    if tile_types::is_diggable(&tile.tile_type, game_data) {
                        tile_count += 1;
                    }
                }
                InteractionMode::BuildRoom(room_type) => {
                    if tile.ownership == Ownership::Player
                        && tile.room_id.is_none()
                        && tile_types::can_build_room(&tile.tile_type, game_data)
                    {
                        let lookup_id = if room_type == "training_room" {
                            "training_hall"
                        } else {
                            room_type
                        };
                        let cost = get_room_cost(lookup_id, Some(game_data));
                        total_cost += cost;
                        tile_count += 1;
                    }
                }
                InteractionMode::BuildTrap(trap_type) => {
                    if tile.ownership == Ownership::Player
                        && tile_types::can_build_room(&tile.tile_type, game_data)
                        && tile.trap.is_none()
                    {
                        let cost = game_data.traps.get(trap_type).map(|t| t.cost).unwrap_or(0);
                        total_cost += cost;
                        tile_count += 1;
                    }
                }
                InteractionMode::PlaceSpawner => {
                    if tile.ownership == Ownership::Player
                        && tile_types::can_build_room(&tile.tile_type, game_data)
                    {
                        let cost = game_data
                            .tiles
                            .get("monster_spawner")
                            .and_then(|t| t.cost)
                            .unwrap_or(50);
                        total_cost += cost;
                        tile_count += 1;
                    }
                }
                InteractionMode::Sell => {
                    // Selling rooms gives partial refund
                    if let Some(room_id) = tile.room_id {
                        if let Some(room) =
                            state.room_manager.rooms.iter().find(|r| r.id == room_id)
                        {
                            let lookup_id = if room.room_type == "training_room" {
                                "training_hall"
                            } else {
                                &room.room_type
                            };
                            let cost = get_room_cost(lookup_id, Some(game_data));
                            total_cost -= cost / 2; // Negative cost = Gain
                            tile_count += 1;
                        }
                    }
                    // Selling traps
                    if let Some(trap) = &tile.trap {
                        let cost = game_data
                            .traps
                            .get(&trap.trap_type)
                            .map(|t| t.cost)
                            .unwrap_or(0);
                        total_cost -= cost / 2;
                        tile_count += 1;
                    }
                }
                _ => {}
            }
        }
    }

    // Determine Text and Color
    let (text, color) = if matches!(mode, InteractionMode::Sell) {
        let gain = (-total_cost).max(0);
        if gain > 0 {
            (format!("+{}g", gain), GREEN)
        } else {
            return;
        }
    } else {
        if total_cost > 0 {
            let affordable = state.player.gold >= total_cost;
            let color = if affordable { GREEN } else { RED };
            (format!("-{}g", total_cost), color)
        } else {
            if tile_count > 0 {
                // For non-cost actions like Dig, just show count
                match mode {
                    InteractionMode::Dig => (format!("Dig {} tiles", tile_count), WHITE),
                    _ => return,
                }
            } else {
                return;
            }
        }
    };

    // Draw Tooltip
    let mouse_pos = mouse_position();
    let font_size = 20.0;
    let dims = measure_text(&text, None, font_size as u16, 1.0);

    let padding = 8.0;
    // Position tooltip to the bottom-right of cursor
    let box_x = mouse_pos.0 + 20.0;
    let box_y = mouse_pos.1 + 20.0;
    let box_w = dims.width + padding * 2.0;
    let box_h = dims.height + padding * 2.0;

    // Ensure it stays on screen
    let box_x = if box_x + box_w > screen_width() {
        box_x - box_w - 40.0
    } else {
        box_x
    };
    let box_y = if box_y + box_h > screen_height() {
        box_y - box_h - 40.0
    } else {
        box_y
    };

    let surface = macroquad_toolkit::ui::SurfaceStyle::new(Color::new(0.0, 0.0, 0.0, 0.9))
        .with_border(1.0, color);
    macroquad_toolkit::ui::draw_surface(Rect::new(box_x, box_y, box_w, box_h), &surface);

    // Center text vertically
    let text_y_offset = (box_h - dims.height) / 2.0 + dims.height - 2.0;
    draw_text(
        &text,
        box_x + padding,
        box_y + text_y_offset,
        font_size,
        color,
    );
}
