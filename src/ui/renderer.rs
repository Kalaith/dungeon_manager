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
        selected_map_type: MapType,
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
                    self.draw_gui(inner_state, interaction_mode, held_entity, selected_entity, selected_room, game_data);
                }
            }
        }
    }

    fn draw_main_menu(&self, selected_map_type: MapType) {
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
        let total_width = (map_button_width + map_button_spacing) * 4.0 - map_button_spacing;
        let map_button_start_x = screen_width() / 2.0 - total_width / 2.0;

        let map_types = [
            (MapType::Standard, "Standard", "Balanced", 0),
            (MapType::Rich, "Rich", "Lots of gold", 1),
            (MapType::Hazardous, "Hazardous", "Many dangers", 2),
            (MapType::Test, "Test", "Fixed seed", 3),
        ];

        for (map_type, label, desc, index) in &map_types {
            let btn_x = map_button_start_x + (map_button_width + map_button_spacing) * (*index as f32);

            let is_hovered = mouse_pos.0 >= btn_x
                && mouse_pos.0 <= btn_x + map_button_width
                && mouse_pos.1 >= map_button_y
                && mouse_pos.1 <= map_button_y + map_button_height;

            let is_selected = selected_map_type == *map_type;

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

        // Draw hint
        draw_text(
            "Click to start or press SPACE",
            screen_width() / 2.0 - 120.0,
            screen_height() / 2.0 + 140.0,
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

        set_default_camera(); // Go back to 2D for UI
    }


    fn draw_gui(
        &mut self, 
        state: &GameState, 
        interaction_mode: &InteractionMode, 
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
            InteractionMode::PlaceSpawner => format!("Mode: Place Spawner ({}g)", crate::config::SPAWNER_COST),
            InteractionMode::Pickup => "Mode: Pickup Minion".to_string(),
            InteractionMode::Drop => "Mode: Drop Minion".to_string(),
            InteractionMode::Sell => "Mode: Sell/Cancel".to_string(),
            InteractionMode::Inspect => "Mode: Inspect (Click unit)".to_string(),
            InteractionMode::BuildTrap(trap_type) => format!("Mode: Build {}", trap_type),
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

        if state.paused {
            self.draw_pause_menu();
        }

        if state.game_over {
            self.draw_game_over_screen(state.victory);
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
                let fog_state = if crate::config::FOG_OF_WAR_ENABLED {
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
                    let fog_state = if crate::config::FOG_OF_WAR_ENABLED {
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
        let start_y = screen_center_y - 50.0;

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

        // Main Menu Button
        let menu_y = start_y + button_height + spacing;
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
        let exit_y = start_y + (button_height + spacing) * 2.0;
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

}




