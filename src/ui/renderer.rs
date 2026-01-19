use macroquad::prelude::*;
use crate::state::game_state::GameState;
use crate::state::{GamePhase, InteractionMode, Ownership, MapType, DragSelection};
use crate::ui::resources::GraphicsCache;
use crate::ui::sidebar::Sidebar;
use crate::state::tile_state::{TilePos, FogState};
use crate::data::GameData;
use crate::state::entities::EntityId;
use crate::engine::tile_types;


pub struct GameRenderer {
    pub graphics_cache: Option<GraphicsCache>,
    pub sidebar: Sidebar,
}

impl GameRenderer {
    pub fn new() -> Self {
        Self {
            graphics_cache: None,
            sidebar: Sidebar::new(),
        }
    }

    pub async fn load_resources(&mut self) {
        match GraphicsCache::load_all().await {
            Ok(cache) => self.graphics_cache = Some(cache),
            Err(e) => eprintln!("Failed to load graphics: {}", e),
        }
    }

    pub fn draw(
        &mut self, 
        phase: &GamePhase,
        state: Option<&GameState>,
        interaction_mode: &InteractionMode,
        selected_map_type: &MapType,
        hovered_tile: Option<TilePos>,
        held_entity: Option<EntityId>,
        selected_entity: Option<EntityId>,
        selected_room: Option<usize>,
        game_data: &Option<GameData>,
        drag_selection: &DragSelection,
    ) {
        clear_background(crate::ui::core::colors::BACKGROUND);

        match phase {
            GamePhase::Loading => {
                draw_text(
                    "Loading Deep Dominion...",
                    screen_width() / 2.0 - 150.0,
                    screen_height() / 2.0,
                    32.0,
                    WHITE,
                );
            }
            GamePhase::MainMenu => {
                self.draw_main_menu(selected_map_type);
            }
            GamePhase::Playing(_) => {
                if let Some(inner_state) = state {
                    if let Some(ref data) = game_data {
                        self.draw_game(inner_state, interaction_mode, hovered_tile, held_entity, data, drag_selection);
                    }
                    self.draw_gui(inner_state, interaction_mode, hovered_tile, held_entity, selected_entity, selected_room, game_data);
                }
            }
        }
    }

    fn draw_main_menu(&self, selected_map_type: &MapType) {
        let mouse_pos = mouse_position();

        // Draw title
        draw_text(
            "Deep Dominion",
            screen_width() / 2.0 - 150.0,
            screen_height() / 2.0 - 180.0,
            48.0,
            WHITE,
        );

        // Draw subtitle
        draw_text(
            "Select Map Type:",
            screen_width() / 2.0 - 80.0,
            screen_height() / 2.0 - 90.0,
            20.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
        );

        // Draw map type selection buttons
        let map_button_y = screen_height() / 2.0 - 50.0;
        let map_button_width = 160.0;
        let map_button_height = 40.0;
        let map_button_spacing = 10.0;
        let total_width = (map_button_width + map_button_spacing) * 5.0 - map_button_spacing;
        let map_button_start_x = screen_width() / 2.0 - total_width / 2.0;

        let map_types = [
            (MapType::Standard, "Standard", "Balanced", 0),
            (MapType::Rich, "Rich", "Lots of gold", 1),
            (MapType::Hazardous, "Hazardous", "Many dangers", 2),
            (MapType::Test, "Test", "Fixed seed", 3),
            (MapType::File("assets/maps/level_1.json".to_string()), "Campaign", "Level 1", 4),
        ];

        for (map_type, label, desc, index) in &map_types {
            let btn_x = map_button_start_x + (map_button_width + map_button_spacing) * (*index as f32);

            let is_hovered = mouse_pos.0 >= btn_x
                && mouse_pos.0 <= btn_x + map_button_width
                && mouse_pos.1 >= map_button_y
                && mouse_pos.1 <= map_button_y + map_button_height;

            let is_selected = *selected_map_type == *map_type;

            let button_color = if is_selected {
                Color::new(0.2, 0.7, 0.3, 1.0) // Green for selected
            } else if is_hovered {
                Color::new(0.4, 0.6, 0.9, 1.0) // Blue for hover
            } else {
                Color::new(0.3, 0.3, 0.4, 1.0) // Gray default
            };

            draw_rectangle(btn_x, map_button_y, map_button_width, map_button_height, button_color);
            draw_rectangle_lines(btn_x, map_button_y, map_button_width, map_button_height, 2.0, WHITE);

            draw_text(
                label,
                btn_x + 10.0,
                map_button_y + 20.0,
                18.0,
                WHITE,
            );

            draw_text(
                desc,
                btn_x + 10.0,
                map_button_y + 35.0,
                12.0,
                Color::new(0.8, 0.8, 0.8, 1.0),
            );
        }

        // Draw start button
        let button_x = screen_width() / 2.0 - 100.0;
        let button_y = screen_height() / 2.0 + 60.0;
        let button_width = 200.0;
        let button_height = 50.0;
        let spacing = 20.0;

        let is_hovered = mouse_pos.0 >= button_x
            && mouse_pos.0 <= button_x + button_width
            && mouse_pos.1 >= button_y
            && mouse_pos.1 <= button_y + button_height;

        let button_color = if is_hovered {
            Color::new(0.4, 0.6, 0.9, 1.0)
        } else {
            Color::new(0.3, 0.5, 0.8, 1.0)
        };

        draw_rectangle(button_x, button_y, button_width, button_height, button_color);
        draw_rectangle_lines(button_x, button_y, button_width, button_height, 3.0, WHITE);

        draw_text(
            "START GAME",
            button_x + 35.0,
            button_y + 32.0,
            24.0,
            WHITE,
        );

        // Load Game Button
        let load_y = button_y + button_height + spacing;
        let save_exists = crate::state::save_system::save_exists("slot_1");

        
        let is_load_hovered = mouse_pos.0 >= button_x
            && mouse_pos.0 <= button_x + button_width
            && mouse_pos.1 >= load_y
            && mouse_pos.1 <= load_y + button_height;

        let load_color = if save_exists {
            if is_load_hovered {
                Color::new(0.4, 0.8, 0.4, 1.0)
            } else {
                Color::new(0.3, 0.7, 0.3, 1.0)
            }
        } else {
            Color::new(0.3, 0.3, 0.3, 1.0)
        };

        draw_rectangle(button_x, load_y, button_width, button_height, load_color);
        draw_rectangle_lines(button_x, load_y, button_width, button_height, 3.0, if save_exists { WHITE } else { GRAY });

        draw_text(
            "LOAD GAME",
            button_x + 35.0,
            load_y + 32.0,
            24.0,
            if save_exists { WHITE } else { GRAY },
        );

        // Draw hint
        draw_text(
            "Click to start or press SPACE",
            screen_width() / 2.0 - 120.0,
            screen_height() / 2.0 + 200.0,
            16.0,
            Color::new(0.7, 0.7, 0.7, 1.0),
        );
    }

    fn draw_game(&self, state: &GameState, interaction_mode: &InteractionMode, hovered_tile: Option<TilePos>, held_entity: Option<EntityId>, game_data: &GameData, drag_selection: &DragSelection) {
        let graphics = if let Some(ref cache) = self.graphics_cache {
            cache
        } else {
            return; // Can't render without graphics
        };

        // Construct Camera3D
        let camera = state.camera.get_camera3d();

        set_camera(&camera);

        self.draw_tiles(graphics, state, interaction_mode, hovered_tile, game_data, drag_selection);
        self.draw_entities(graphics, state, &camera);
        self.draw_markers(state);

         // Draw Mode Cursor / Ghost
        if let Some(pos) = hovered_tile {
            match interaction_mode {
                InteractionMode::SetAttackMarker => {
                    let world_x = pos.x as f32;
                    let world_z = pos.y as f32;
                    draw_cube_wires(vec3(world_x, 0.5, world_z), vec3(1.0, 1.0, 1.0), RED);
                },
                InteractionMode::SetDefendMarker => {
                    let world_x = pos.x as f32;
                    let world_z = pos.y as f32;
                    draw_cube_wires(vec3(world_x, 0.5, world_z), vec3(1.0, 1.0, 1.0), BLUE);
                },
                _ => {}
            }
        }

        set_default_camera(); // Go back to 2D for UI

        // Draw 2D Cursor Label
        if let Some(_) = hovered_tile {
            match interaction_mode {
                InteractionMode::SetAttackMarker => {
                    let mouse = mouse_position();
                    draw_text("ATTACK", mouse.0 + 20.0, mouse.1, 20.0, RED);
                },
                 InteractionMode::SetDefendMarker => {
                    let mouse = mouse_position();
                    draw_text("DEFEND", mouse.0 + 20.0, mouse.1, 20.0, BLUE);
                },
                _ => {}
            }
        }
    }


    fn draw_gui(
        &mut self, 
        state: &GameState, 
        interaction_mode: &InteractionMode, 
        hovered_tile: Option<TilePos>,
        held_entity: Option<EntityId>,
        selected_entity: Option<EntityId>,
        selected_room: Option<usize>,
        game_data: &Option<GameData>
    ) {
         let graphics = if let Some(ref cache) = self.graphics_cache {
            cache
        } else {
            return;
        };

        // Draw HUD
        draw_rectangle(0.0, 0.0, screen_width(), crate::ui::core::HUD_HEIGHT, crate::ui::core::colors::PANEL);

        let mode_text = match interaction_mode {
            InteractionMode::None => "Mode: None (Select tab below)".to_string(),
            InteractionMode::Dig => "Mode: Dig (FREE)".to_string(),
            InteractionMode::BuildRoom(room_type) => {
                let lookup_id = if room_type == "training_room" { "training_hall" } else { room_type };
                let cost = self.get_room_cost(lookup_id, game_data);
                format!("Mode: Build {} ({}g)", room_type, cost)
            }
            InteractionMode::PlaceSpawner => {
                let cost = game_data.as_ref()
                    .and_then(|gd| gd.tiles.get("monster_spawner"))
                    .and_then(|t| t.cost)
                    .unwrap_or(50);
                format!("Mode: Place Spawner ({}g)", cost)
            }
            InteractionMode::Pickup => "Mode: Pickup Minion".to_string(),
            InteractionMode::Drop => "Mode: Drop Minion".to_string(),
            InteractionMode::Sell => "Mode: Sell/Cancel".to_string(),
            InteractionMode::Inspect => "Mode: Inspect (Click unit)".to_string(),
            InteractionMode::BuildTrap(trap_type) => format!("Mode: Build {}", trap_type),
            InteractionMode::SetAttackMarker => "Mode: Set Attack Marker".to_string(),
            InteractionMode::SetDefendMarker => "Mode: Set Defend Marker".to_string(),
            InteractionMode::SaveGame => "Mode: Saving...".to_string(),
        };

        draw_text(
            &format!("Gold: {}/{} | Mana: {}/{} | Food: {} | Mats: {}/{} | Minions: {}/{} | Heart: {:.0}",
                state.player.gold, state.player.max_gold,
                state.player.mana, state.player.max_mana,
                state.player.food,
                state.player.materials, state.player.max_materials,
                state.player.current_creature_count, state.player.max_creatures,
                state.dungeon_heart_health),
            10.0,
            25.0,
            18.0,
            crate::ui::core::colors::TEXT,
        );

        draw_text(
            &mode_text,
            10.0,
            45.0,
            16.0,
            crate::ui::core::colors::ACCENT,
        );
        let mouse_pos = mouse_position();
        
        // Draw held entity if any
        if let Some(entity_id) = held_entity {
             let texture_opt = if let Some(entity) = state.entities.get(entity_id) {
                 match &entity.entity_type {
                    crate::state::entities::EntityType::Hero(hero_state) => {
                        graphics.hero_textures.get(&hero_state.hero_id)
                    }
                    crate::state::entities::EntityType::Creature(creature_state) => {
                        graphics.monster_textures.get(&creature_state.creature_id)
                    }
                    crate::state::entities::EntityType::Structure(state) => {
                        graphics.tile_textures.get(&state.building_id)
                    }
                }
             } else { None };

             if let Some(texture) = texture_opt {
                 draw_texture_ex(
                    texture,
                    mouse_pos.0 - 24.0,
                    mouse_pos.1 - 24.0,
                    Color::new(1.0, 1.0, 1.0, 0.8), // Slight transparency applied here
                     DrawTextureParams {
                        dest_size: Some(vec2(48.0, 48.0)),
                        ..Default::default()
                    },
                );
             }
        } else {
             if is_mouse_button_down(MouseButton::Right) && !self.sidebar.is_mouse_over() {
                draw_circle(mouse_pos.0, mouse_pos.1, 5.0, RED);
             }
        }

        // Draw Sidebar
        if let Some(ref data) = game_data {
            self.sidebar.draw(
                interaction_mode, 
                &state.player, 
                data,
                held_entity,
                selected_entity,
                selected_room,
                &state.entities,
                &state.room_manager.rooms
            );
        }



        // Draw notifications
        self.draw_notifications(state);
        
        // Draw minimap
        self.draw_minimap(state, game_data);

        if state.paused {
            self.draw_pause_menu();
        }

        if state.game_over {
            self.draw_game_over_screen(state.victory);
        }

        // Draw tooltips last so they are on top of everything
        self.draw_tooltips(state, hovered_tile, game_data);
    }

    fn draw_minimap(&self, state: &GameState, game_data: &Option<GameData>) {
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
                    let fog_enabled = game_data.as_ref().map(|gd| gd.config.fog_of_war.enabled).unwrap_or(true);
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
                        } else if tile.ownership == Ownership::Enemy {
                             // Hero base floor
                             Color::new(0.8, 0.2, 0.2, 1.0)
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
        
        // Draw Hero Base (approximate if known)
        if state.hero_base.enabled {
             let base_pos = state.hero_base.position;
             let bx = start_x + (base_pos.x as f32 / grid_w as f32) * map_width;
             let by = start_y + (base_pos.y as f32 / grid_h as f32) * map_height;
             draw_circle(bx, by, 3.0, RED);
        }
    }

    fn draw_tooltips(
        &self, 
        state: &GameState, 
        hovered_tile: Option<TilePos>, 
        game_data: &Option<GameData>
    ) {
        if let Some(pos) = hovered_tile {
            // Check if mouse is over sidebar, if so, don't show tile tooltip
            if self.sidebar.is_mouse_over() {
                return;
            }

            if let Some(tile) = state.get_tile(pos) {
                let mut lines = Vec::new();
                
                // Tile Name
                let tile_name = if let Some(data) = game_data {
                     data.tiles.get(&tile.tile_type).map(|t| t.name.clone()).unwrap_or(tile.tile_type.clone())
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
                     let status = if trap.constructed { "Active" } else { "Building..." };
                     if let Some(data) = game_data {
                         let trap_name = data.traps.get(&trap.trap_type).map(|t| t.name.clone()).unwrap_or(trap.trap_type.clone());
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
                                 data.monsters.get(&c.creature_id).map(|m| m.name.clone()).unwrap_or(c.creature_id.clone())
                             } else {
                                 c.creature_id.clone()
                             };
                             lines.push(format!("{} (HP: {:.0})", name, c.health));
                        }
                        crate::state::entities::EntityType::Hero(h) => {
                             let name = if let Some(data) = game_data {
                                 data.heroes.get(&h.hero_id).map(|m| m.name.clone()).unwrap_or(h.hero_id.clone())
                             } else {
                                 h.hero_id.clone()
                             };
                             
                             if h.is_converted {
                                 lines.push(format!("Minion: {} (Lvl {})", name, h.level));
                             } else if h.is_captured {
                                 lines.push(format!("CAPTURED: {} ({:.0}%)", name, h.conversion_progress * 100.0));
                             } else {
                                 lines.push(format!("Hero: {} (Lvl {})", name, h.level));
                             }
                        }
                        crate::state::entities::EntityType::Structure(s) => {
                             lines.push(format!("Structure: {:.0} HP", s.health));
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

                draw_rectangle(draw_x, draw_y, box_width, box_height, Color::new(0.1, 0.1, 0.1, 0.9));
                draw_rectangle_lines(draw_x, draw_y, box_width, box_height, 1.0, WHITE);

                for (i, line) in lines.iter().enumerate() {
                    draw_text(
                        line,
                        draw_x + padding,
                        draw_y + padding + (i as f32 * (font_size + 4.0)) + font_size - 4.0,
                        font_size,
                        WHITE
                    );
                }
            }
        }
    }

    fn draw_notifications(&self, state: &GameState) {
        use crate::state::notifications::NotificationType;

        let notifications = state.notifications.get_notifications();
        if notifications.is_empty() {
            return;
        }

        let notification_width = 300.0;
        let notification_height = 40.0;
        let padding = 10.0;
        let start_x = screen_width() - notification_width - 20.0;
        let start_y = crate::ui::core::HUD_HEIGHT + 20.0;

        for (i, notification) in notifications.iter().enumerate() {
            let y = start_y + (notification_height + padding) * i as f32;
            let opacity = notification.opacity();

            // Background color based on type
            let bg_color = match notification.notification_type {
                NotificationType::Success => Color::new(0.1, 0.5, 0.1, 0.9 * opacity),
                NotificationType::Info => Color::new(0.2, 0.3, 0.5, 0.9 * opacity),
                NotificationType::Warning => Color::new(0.6, 0.5, 0.1, 0.9 * opacity),
                NotificationType::Danger => Color::new(0.6, 0.1, 0.1, 0.9 * opacity),
            };

            // Border color
            let border_color = match notification.notification_type {
                NotificationType::Success => Color::new(0.2, 0.8, 0.2, opacity),
                NotificationType::Info => Color::new(0.3, 0.5, 0.8, opacity),
                NotificationType::Warning => Color::new(0.9, 0.7, 0.1, opacity),
                NotificationType::Danger => Color::new(0.9, 0.2, 0.2, opacity),
            };

            // Draw background
            draw_rectangle(start_x, y, notification_width, notification_height, bg_color);
            draw_rectangle_lines(start_x, y, notification_width, notification_height, 2.0, border_color);

            // Draw text
            let text_color = Color::new(1.0, 1.0, 1.0, opacity);
            draw_text(
                &notification.message,
                start_x + 10.0,
                y + 26.0,
                18.0,
                text_color,
            );
        }
    }

    fn get_room_cost(&self, room_type: &str, game_data: &Option<GameData>) -> i32 {
         if let Some(data) = game_data {
             data.rooms.get(room_type)
                 .map(|r| r.build.cost_per_tile)
                 .unwrap_or_else(|| panic!("Room type '{}' missing in rooms.json", room_type))
         } else {
             0 // Should not happen during gameplay
         }
    }

    fn draw_tiles(
        &self,
        graphics: &GraphicsCache,
        state: &GameState,
        interaction_mode: &InteractionMode,
        hovered_tile: Option<TilePos>,
        game_data: &GameData,
        drag_selection: &DragSelection,
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

                // Get texture for this tile type
                let texture_opt = graphics.tile_textures.get(&tile.tile_type);
                
                // Determine visible color based on Fog of War settings
                let fog_state = if game_data.config.fog_of_war.enabled {
                    tile.fog_state
                } else {
                    FogState::Visible
                };

                let mut color = match fog_state {
                    FogState::Hidden => crate::ui::core::colors::FOG_HIDDEN,
                    FogState::Revealed => crate::ui::core::colors::FOG_REVEALED,
                    FogState::Visible => crate::ui::core::colors::FOG_VISIBLE,
                };

                // Show marked tiles with yellow tint
                if tile.marked_for_dig {
                    color = Color::new(1.0, 1.0, 0.3, 1.0);
                }

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

                    // Draw trap if present
                    if let Some(trap) = &tile.trap {
                        let is_constructing = !trap.constructed;
                        let trap_color = if is_constructing {
                            Color::new(1.0, 1.0, 1.0, 0.3) // Very transparent for "ghost"
                        } else {
                            WHITE
                        };
                        
                        if let Some(trap_texture) = graphics.tile_textures.get(&trap.trap_type) {
                            // Draw trap slightly above floor, smaller size to fit well
                            draw_plane(
                                vec3(pos_x, 0.05, pos_z), 
                                vec2(0.6, 0.6), // Reduced from 0.8
                                Some(trap_texture),
                                trap_color,
                            );
                        } else {
                            // Fallback if texture missing (e.g. doors)
                            // Draw a colored box - User requested "full square" for now
                            let (fallback_color, size) = match trap.trap_type.as_str() {
                                "door" => (Color::new(0.4, 0.2, 0.1, if is_constructing { 0.3 } else { 1.0 }), vec3(1.0, 1.0, 1.0)), // Full block for door
                                "spike_trap" => (Color::new(0.5, 0.5, 0.5, if is_constructing { 0.3 } else { 1.0 }), vec3(1.0, 0.1, 1.0)), // Full floor tile for spikes
                                _ => (Color::new(0.8, 0.2, 0.2, if is_constructing { 0.3 } else { 1.0 }), vec3(1.0, 0.2, 1.0)), // Generic
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
                    if tile.marked_for_dig {
                        tile_color = Color::new(1.0, 0.8, 0.0, 1.0);
                    }

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
                                    || (tile.ownership == Ownership::Player
                                        && tile.room_id.is_some())
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
                let width = (max.x - min.x + 1) as f32;  // +1 because inclusive
                let depth = (max.y - min.y + 1) as f32;  // +1 because inclusive
                
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




    fn draw_entities(&self, graphics: &GraphicsCache, state: &GameState, camera: &Camera3D) {
        // Collect and sort entities by distance from camera (far to near) for proper transparency
        let mut sorted_entities: Vec<_> = state.entities.all().collect();
        sorted_entities.sort_by(|a, b| {
            let dist_a = (camera.position - vec3(a.visual_pos.0, 0.5, a.visual_pos.1)).length_squared();
            let dist_b = (camera.position - vec3(b.visual_pos.0, 0.5, b.visual_pos.1)).length_squared();
            // Sort far to near (largest distance first)
            dist_b.partial_cmp(&dist_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Draw sorted entities
        for entity in sorted_entities {
            let (x, z) = entity.visual_pos;

            // Handle Structures separately (draw as blocks/cubes)
            if let crate::state::entities::EntityType::Structure(state) = &entity.entity_type {
                if let Some(tex) = graphics.tile_textures.get(&state.building_id) {
                    draw_cube(
                        vec3(x, 0.5, z),
                        vec3(1.0, 1.0, 1.0),
                        Some(tex),
                        WHITE
                    );
                    continue;
                }
            }

            let texture = match &entity.entity_type {
                crate::state::entities::EntityType::Creature(c) => {
                    graphics.monster_textures.get(&c.creature_id)
                }
                crate::state::entities::EntityType::Hero(h) => {
                    graphics.hero_textures.get(&h.hero_id)
                }
                crate::state::entities::EntityType::Structure(_) => None, // Should not happen due to block above
            };

            if let Some(tex) = texture {
                crate::draw_utils::draw_billboard(
                    vec3(x, 0.5, z),
                    vec2(0.8, 0.8),
                    tex,
                    camera.position,
                );
            } else {
                // Fallback to simple colored cylinder/sphere/cube
                let color = match &entity.entity_type {
                    crate::state::entities::EntityType::Hero(_) => Color::new(0.2, 0.8, 0.2, 1.0),
                    crate::state::entities::EntityType::Creature(_) => {
                        Color::new(0.8, 0.2, 0.2, 1.0)
                    }
                    crate::state::entities::EntityType::Structure(_) => Color::new(0.5, 0.5, 0.5, 1.0),
                };
                draw_cube_wires(vec3(x, 0.5, z), vec3(0.5, 1.0, 0.5), color);
            }
        }
    }

    pub fn draw_pause_menu(&self) {
        // Semi-transparent overlay
        draw_rectangle(
            0.0, 0.0, 
            screen_width(), screen_height(), 
            Color::new(0.0, 0.0, 0.0, 0.7)
        );

        let screen_center_x = screen_width() / 2.0;
        let screen_center_y = screen_height() / 2.0;
        let button_width = 200.0;
        let button_height = 50.0;
        let spacing = 20.0;
        let start_y = screen_center_y - 100.0; // Adjusted to fit more buttons

        // Title
        let title = "PAUSED";
        let title_dims = measure_text(title, None, 60, 1.0);
        draw_text(
            title, 
            screen_center_x - title_dims.width / 2.0, 
            start_y - 80.0, 
            60.0, 
            WHITE
        );

        // Resume Button
        let resume_y = start_y;
        draw_rectangle(
            screen_center_x - button_width / 2.0, resume_y, 
            button_width, button_height, 
            Color::new(0.2, 0.6, 0.2, 1.0)
        );
        let resume_text = "RESUME";
        let resume_dims = measure_text(resume_text, None, 30, 1.0);
        draw_text(
            resume_text, 
            screen_center_x - resume_dims.width / 2.0, 
            resume_y + 35.0, 
            30.0, 
            WHITE
        );

        // Save Game Button
        let save_y = start_y + button_height + spacing;
        draw_rectangle(
            screen_center_x - button_width / 2.0, save_y, 
            button_width, button_height, 
            Color::new(0.3, 0.5, 0.7, 1.0)
        );
        let save_text = "SAVE GAME";
        let save_dims = measure_text(save_text, None, 30, 1.0);
        draw_text(
            save_text, 
            screen_center_x - save_dims.width / 2.0, 
            save_y + 35.0, 
            30.0, 
            WHITE
        );

        // Load Game Button
        let load_y = start_y + (button_height + spacing) * 2.0;
        let save_exists = crate::state::save_system::save_exists("slot_1");
        let load_color = if save_exists {
            Color::new(0.3, 0.7, 0.4, 1.0)
        } else {
            Color::new(0.3, 0.3, 0.3, 1.0) // Grayed out if no save
        };
        draw_rectangle(
            screen_center_x - button_width / 2.0, load_y, 
            button_width, button_height, 
            load_color
        );
        let load_text = "LOAD GAME";
        let load_dims = measure_text(load_text, None, 30, 1.0);
        draw_text(
            load_text, 
            screen_center_x - load_dims.width / 2.0, 
            load_y + 35.0, 
            30.0, 
            if save_exists { WHITE } else { GRAY }
        );

        // Main Menu Button
        let menu_y = start_y + (button_height + spacing) * 3.0;
        draw_rectangle(
            screen_center_x - button_width / 2.0, menu_y, 
            button_width, button_height, 
            Color::new(0.6, 0.4, 0.2, 1.0)
        );
        let menu_text = "MAIN MENU";
        let menu_dims = measure_text(menu_text, None, 30, 1.0);
        draw_text(
            menu_text, 
            screen_center_x - menu_dims.width / 2.0, 
            menu_y + 35.0, 
            30.0, 
            WHITE
        );

        // Exit Button
        let exit_y = start_y + (button_height + spacing) * 4.0;
        draw_rectangle(
            screen_center_x - button_width / 2.0, exit_y, 
            button_width, button_height, 
            Color::new(0.8, 0.2, 0.2, 1.0)
        );
        let exit_text = "EXIT";
        let exit_dims = measure_text(exit_text, None, 30, 1.0);
        draw_text(
            exit_text, 
            screen_center_x - exit_dims.width / 2.0, 
            exit_y + 35.0, 
            30.0, 
            WHITE
        );
    }

    pub fn draw_game_over_screen(&self, victory: bool) {
        // Semi-transparent overlay
        draw_rectangle(
            0.0, 0.0, 
            screen_width(), screen_height(), 
            Color::new(0.0, 0.0, 0.0, 0.8)
        );

        let screen_center_x = screen_width() / 2.0;
        let screen_center_y = screen_height() / 2.0;

        let title = if victory { "VICTORY" } else { "DEFEAT" };
        let color = if victory { crate::ui::core::colors::POSITIVE } else { crate::ui::core::colors::NEGATIVE };
        
        // Title
        let title_dims = measure_text(title, None, 80, 1.0);
        draw_text(
            title, 
            screen_center_x - title_dims.width / 2.0, 
            screen_center_y - 100.0, 
            80.0, 
            color
        );

        // Subtitle
        let subtitle = if victory { "The Hero Base has been destroyed!" } else { "Your Dungeon Heart has fallen!" };
        let sub_dims = measure_text(subtitle, None, 40, 1.0);
        draw_text(
            subtitle, 
            screen_center_x - sub_dims.width / 2.0, 
            screen_center_y - 20.0, 
            40.0, 
            WHITE
        );

        // Instructions
        let instr = "Press ESC to Exit";
        let instr_dims = measure_text(instr, None, 30, 1.0);
        draw_text(
            instr, 
            screen_center_x - instr_dims.width / 2.0, 
            screen_center_y + 60.0, 
            30.0, 
            GRAY
        );
    }

    /// Check if a tile is valid for the current interaction mode (used for drag selection visualization)
    fn is_valid_tile_for_mode(tile: &crate::state::tile_state::TileState, mode: &InteractionMode, game_data: &GameData) -> bool {
        match mode {
            InteractionMode::Dig => {
                tile_types::is_diggable(&tile.tile_type, game_data) 
                    && tile.ownership == Ownership::Unclaimed
            }
            InteractionMode::BuildRoom(_) => {
                tile.ownership == Ownership::Player
                    && tile.room_id.is_none()
                    && tile_types::can_build_room(&tile.tile_type, game_data)
            }
            InteractionMode::BuildTrap(_) => {
                tile.ownership == Ownership::Player
                    && tile_types::can_build_room(&tile.tile_type, game_data)
                    && tile.trap.is_none()
            }
            InteractionMode::PlaceSpawner => {
                tile.ownership == Ownership::Player
                    && tile_types::can_build_room(&tile.tile_type, game_data)
            }
            _ => false,
        }
    }

    fn draw_markers(&self, state: &GameState) {
        let draw_flag = |pos: TilePos, color: Color| {
            let x = pos.x as f32;
            let z = pos.y as f32;
            
            // Pole (center of tile)
            draw_cylinder(vec3(x, 0.0, z), 0.05, 0.05, 2.0, None, BROWN);
            
            // Flag Banner
            draw_cube(vec3(x + 0.3, 1.5, z), vec3(0.6, 0.5, 0.05), None, color);
        };

        if let Some(pos) = state.attack_marker {
            draw_flag(pos, RED);
        }
        if let Some(pos) = state.defend_marker {
            draw_flag(pos, BLUE);
        }
    }
}




