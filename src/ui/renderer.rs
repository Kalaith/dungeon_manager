use crate::data::GameData;
use crate::engine::tile_types;
use crate::state::entities::EntityId;
use crate::state::game_state::GameState;
use crate::state::tile_state::{FogState, TilePos};
use crate::state::{DragSelection, GamePhase, InteractionMode, MapType, Ownership};
use crate::ui::resources::GraphicsCache;
use crate::ui::sidebar::Sidebar;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_surface, draw_ui_text, SurfaceStyle};

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

    pub async fn load_resources(&mut self, game_data: Option<&crate::data::GameData>) {
        // Register the bundled UI font explicitly so text styling is deliberate
        if let Err(e) = macroquad_toolkit::ui::ensure_default_ui_font() {
            eprintln!("Failed to load UI font, falling back to built-in: {}", e);
        }

        match GraphicsCache::load_all(game_data).await {
            Ok(cache) => self.graphics_cache = Some(cache),
            Err(e) => eprintln!("Failed to load graphics: {}", e),
        }
    }

    pub fn draw(
        &mut self,
        phase: &GamePhase,
        state: Option<&mut GameState>,
        interaction_mode: &mut InteractionMode,
        selected_map_type: &MapType,
        hovered_tile: Option<TilePos>,
        held_entity: Option<EntityId>,
        selected_entity: Option<EntityId>,
        selected_room: Option<usize>,
        game_data: &Option<GameData>,
        drag_selection: &DragSelection,
        settings: &crate::state::settings::GameSettings,
    ) {
        clear_background(crate::ui::core::colors::BACKGROUND);

        match phase {
            GamePhase::Loading => {
                draw_ui_text(
                    "Loading Deep Dominion...",
                    screen_width() / 2.0 - 150.0,
                    screen_height() / 2.0,
                    32.0,
                    WHITE,
                );
            }
            GamePhase::MainMenu => {
                crate::ui::menus::draw_main_menu(self.graphics_cache.as_ref(), selected_map_type);
            }
            GamePhase::Settings => {
                crate::ui::menus::draw_settings_menu(settings);
            }
            GamePhase::MissionSelect(progress) => {
                crate::ui::menus::draw_mission_select(progress, game_data.as_ref());
            }
            GamePhase::SkirmishSetup(config) => {
                crate::ui::menus::draw_skirmish_setup(config);
            }
            GamePhase::Playing(_) => {
                if let Some(inner_state) = state {
                    if let Some(ref data) = game_data {
                        self.draw_game(
                            inner_state,
                            interaction_mode,
                            hovered_tile,
                            data,
                            drag_selection,
                        );
                    }
                    self.draw_gui(
                        inner_state,
                        interaction_mode,
                        hovered_tile,
                        held_entity,
                        selected_entity,
                        selected_room,
                        game_data,
                        drag_selection,
                    );
                }
            }
        }
    }

    pub fn draw_game(
        &self,
        state: &GameState,
        interaction_mode: &InteractionMode,
        hovered_tile: Option<TilePos>,
        game_data: &GameData,
        drag_selection: &DragSelection,
    ) {
        let graphics = if let Some(ref cache) = self.graphics_cache {
            cache
        } else {
            return; // Can't render without graphics
        };

        // Construct Camera3D
        let camera = state.camera.get_camera3d();

        set_camera(&camera);

        self.draw_tiles(
            graphics,
            state,
            interaction_mode,
            hovered_tile,
            game_data,
            drag_selection,
        );
        crate::ui::entity_renderer::draw_entities(graphics, state, &camera, game_data);
        self.draw_projectiles(graphics, state, &camera);
        self.draw_markers(state);

        // Draw Mode Cursor / Ghost
        if let Some(pos) = hovered_tile {
            match interaction_mode {
                InteractionMode::SetAttackMarker => {
                    let world_x = pos.x as f32;
                    let world_z = pos.y as f32;
                    draw_cube_wires(vec3(world_x, 0.5, world_z), vec3(1.0, 1.0, 1.0), RED);
                }
                InteractionMode::SetDefendMarker => {
                    let world_x = pos.x as f32;
                    let world_z = pos.y as f32;
                    draw_cube_wires(vec3(world_x, 0.5, world_z), vec3(1.0, 1.0, 1.0), BLUE);
                }
                InteractionMode::Summon(_, _, _) => {
                    let world_x = pos.x as f32;
                    let world_z = pos.y as f32;
                    // Draw ghost green box
                    draw_cube_wires(vec3(world_x, 0.5, world_z), vec3(1.0, 1.0, 1.0), GREEN);
                }
                _ => {}
            }
        }

        set_default_camera(); // Go back to 2D for UI

        // Draw In-Game Cheat Menu Overlay
        // Removed call

        // Draw 2D Cursor Label
        if hovered_tile.is_some() {
            match interaction_mode {
                InteractionMode::SetAttackMarker => {
                    let mouse = mouse_position();
                    draw_ui_text("ATTACK", mouse.0 + 20.0, mouse.1, 20.0, RED);
                }
                InteractionMode::SetDefendMarker => {
                    let mouse = mouse_position();
                    draw_ui_text("DEFEND", mouse.0 + 20.0, mouse.1, 20.0, BLUE);
                }
                _ => {}
            }
        }
    }

    pub fn draw_gui(
        &mut self,
        state: &mut GameState,
        interaction_mode: &mut InteractionMode,
        hovered_tile: Option<TilePos>,
        held_entity: Option<EntityId>,
        selected_entity: Option<EntityId>,
        selected_room: Option<usize>,
        game_data: &Option<GameData>,
        drag_selection: &DragSelection,
    ) {
        let graphics = if let Some(ref cache) = self.graphics_cache {
            cache
        } else {
            return;
        };

        // Draw HUD
        let hud_style = SurfaceStyle::new(crate::ui::core::colors::PANEL_DARK)
            .with_border(1.0, crate::ui::core::colors::PANEL_BORDER)
            .with_top_highlight(2.0, crate::ui::core::colors::ACCENT_GOLD);
        draw_surface(
            Rect::new(0.0, 0.0, screen_width(), crate::ui::core::HUD_HEIGHT),
            &hud_style,
        );

        let mode_text = match interaction_mode {
            InteractionMode::None => "Mode: None (Select tab below)".to_string(),
            InteractionMode::Dig => "Mode: Dig (FREE)".to_string(),
            InteractionMode::BuildRoom(ref room_type) => {
                let lookup_id = crate::data::rooms::room_data_id(room_type);
                let cost = self.get_room_cost(lookup_id, game_data.as_ref());
                format!("Mode: Build {} ({}g)", room_type, cost)
            }
            InteractionMode::PlaceSpawner => {
                let cost = game_data
                    .as_ref()
                    .and_then(|gd| gd.tiles.get("monster_spawner"))
                    .and_then(|t| t.cost)
                    .unwrap_or(50);
                format!("Mode: Place Spawner ({}g)", cost)
            }
            InteractionMode::Summon(_, _, _) => "Mode: Summoning (LMB)".to_string(),
            InteractionMode::Pickup => "Mode: Pickup Minion".to_string(),
            InteractionMode::Drop => "Mode: Drop Minion".to_string(),
            InteractionMode::Sell => "Mode: Sell/Cancel".to_string(),
            InteractionMode::Inspect => "Mode: Inspect (Click unit)".to_string(),
            InteractionMode::BuildTrap(trap_type) => format!("Mode: Build {}", trap_type),
            InteractionMode::SetAttackMarker => "Mode: Set Attack Marker".to_string(),
            InteractionMode::SetDefendMarker => "Mode: Set Defend Marker".to_string(),
            InteractionMode::SaveGame => "Mode: Saving...".to_string(),
        };

        // Color-coded resource readout
        let heart_max = game_data
            .as_ref()
            .map(|gd| gd.config.dungeon.heart_max_health)
            .unwrap_or(1000.0);
        let heart_color = if state.dungeon_heart_health < heart_max * 0.25 {
            crate::ui::core::colors::NEGATIVE
        } else {
            crate::ui::core::colors::POSITIVE
        };
        let segments = [
            (
                format!("Gold: {}/{}", state.player.gold, state.player.max_gold),
                crate::ui::core::colors::ACCENT_GOLD,
            ),
            (
                format!("Mana: {}/{}", state.player.mana, state.player.max_mana),
                crate::ui::core::colors::ACCENT,
            ),
            (
                format!("Food: {}", state.player.food),
                crate::ui::core::colors::POSITIVE,
            ),
            (
                format!(
                    "Mats: {}/{}",
                    state.player.materials, state.player.max_materials
                ),
                crate::ui::core::colors::TEXT_DIM,
            ),
            (
                format!(
                    "Minions: {}/{}",
                    state.player.current_creature_count, state.player.max_creatures
                ),
                crate::ui::core::colors::TEXT,
            ),
            (
                format!("Heart: {:.0}", state.dungeon_heart_health),
                heart_color,
            ),
        ];
        let mut seg_x = 10.0;
        for (text, color) in &segments {
            let dims = draw_ui_text(text, seg_x, 25.0, 18.0, *color);
            seg_x += dims.width + 18.0;
        }

        draw_ui_text(
            &mode_text,
            10.0,
            45.0,
            16.0,
            crate::ui::core::colors::ACCENT,
        );
        let mouse_pos = mouse_position();

        // Draw held entity if any
        if let Some(entity_id) = held_entity {
            let texture_opt: Option<Texture2D> = if let Some(entity) = state.entities.get(entity_id)
            {
                match &entity.entity_type {
                    crate::state::entities::EntityType::Hero(hero_state) => {
                        graphics.get_hero_texture(&hero_state.hero_id, hero_state.visual_seed)
                    }
                    crate::state::entities::EntityType::Creature(creature_state) => graphics
                        .get_creature_texture(
                            &creature_state.creature_id,
                            creature_state.visual_seed,
                        ),
                    crate::state::entities::EntityType::Structure(s) => {
                        graphics.building_texture(&s.building_id).cloned()
                    }
                    crate::state::entities::EntityType::ResourcePile(_) => {
                        graphics.tile_textures.get("gold_pile").cloned()
                    }
                }
            } else {
                None
            };

            if let Some(ref texture) = texture_opt {
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
                &state.room_manager.rooms,
                self.graphics_cache.as_ref(),
            );
        }

        // Draw notifications
        crate::ui::notifications::draw_notifications(state);

        // Draw minimap
        crate::ui::minimap::draw_minimap(state, game_data);

        // Tutorial objective panel
        crate::ui::tutorial::draw_tutorial_panel(state);

        // Scenario intro overlay (blocks the view until dismissed)
        if let Some(ref data) = game_data {
            if !state.paused && !state.game_over {
                crate::ui::tutorial::draw_intro_overlay(state, data);
            }
        }

        if state.paused {
            crate::ui::menus::draw_pause_menu();
        }

        if state.game_over {
            crate::ui::menus::draw_game_over_screen(state, game_data.as_ref());
        }

        // Draw tooltips last so they are on top of everything
        crate::ui::tooltips::draw_tooltips(
            state,
            hovered_tile,
            game_data,
            interaction_mode,
            drag_selection,
            &self.sidebar,
            |room_type, data| self.get_room_cost(room_type, data),
        );
    }

    fn get_room_cost(&self, room_type: &str, game_data: Option<&GameData>) -> i32 {
        if let Some(data) = game_data {
            data.rooms
                .get(room_type)
                .map(|r| r.build.cost_per_tile)
                .unwrap_or_else(|| {
                    eprintln!("Warning: Room type '{}' missing in rooms.json", room_type);
                    100
                })
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
                                    Color::new(
                                        0.4,
                                        0.2,
                                        0.1,
                                        if is_constructing { 0.3 } else { 1.0 },
                                    ),
                                    vec3(1.0, 1.0, 1.0),
                                ), // Full block for door
                                "spike_trap" => (
                                    Color::new(
                                        0.5,
                                        0.5,
                                        0.5,
                                        if is_constructing { 0.3 } else { 1.0 },
                                    ),
                                    vec3(1.0, 0.1, 1.0),
                                ), // Full floor tile for spikes
                                _ => (
                                    Color::new(
                                        0.8,
                                        0.2,
                                        0.2,
                                        if is_constructing { 0.3 } else { 1.0 },
                                    ),
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

    fn draw_projectiles(&self, graphics: &GraphicsCache, state: &GameState, camera: &Camera3D) {
        for projectile in state.projectiles.active_projectiles() {
            let pos = projectile.position();
            let (x, z) = (pos.x, pos.y);
            let projectile_type = &projectile.payload.projectile_type;
            let texture_key = projectile_type.texture_key();

            if let Some(tex) = graphics.projectile_textures.get(texture_key) {
                // Draw projectile slightly above ground level
                let y_height = match projectile_type {
                    crate::state::projectiles::ProjectileType::Melee => 0.5, // At entity level
                    crate::state::projectiles::ProjectileType::Arrow => 0.6, // Slightly higher
                    crate::state::projectiles::ProjectileType::Magic => 0.7, // Floating orb
                };

                // Scale based on projectile type
                let scale = match projectile_type {
                    crate::state::projectiles::ProjectileType::Melee => vec2(0.6, 0.3),
                    crate::state::projectiles::ProjectileType::Arrow => vec2(0.5, 0.3),
                    crate::state::projectiles::ProjectileType::Magic => vec2(0.4, 0.4),
                };

                // Use billboard drawing for projectiles
                crate::draw_utils::draw_billboard(
                    vec3(x, y_height, z),
                    scale,
                    tex,
                    camera.position,
                );
            } else {
                // Fallback: simple colored sphere for debugging
                let color = match projectile_type {
                    crate::state::projectiles::ProjectileType::Melee => ORANGE,
                    crate::state::projectiles::ProjectileType::Arrow => BROWN,
                    crate::state::projectiles::ProjectileType::Magic => PURPLE,
                };
                draw_sphere(vec3(x, 0.5, z), 0.15, None, color);
            }
        }
    }
}
