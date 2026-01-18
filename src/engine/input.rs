use macroquad::prelude::*;
use crate::state::game_state::GameState;
use crate::state::{GamePhase, InteractionMode, TilePos, MapType, DragSelection};
use crate::data::GameData;
use crate::state::entities::EntityId;
use crate::ui::sidebar::{Sidebar, SidebarTab};

use crate::engine::spell_effects;
use crate::engine::creature_ai;
use crate::ui::actions::ActionQueue;

pub struct InputHandler;

impl InputHandler {
    pub fn update(
        dt: f32,
        phase: &mut GamePhase,
        game_data: &Option<GameData>,
        interaction_mode: &mut InteractionMode,
        selected_map_type: &mut MapType,
        hovered_tile: &mut Option<TilePos>,
        held_entity: &mut Option<EntityId>,
        selected_entity: &mut Option<EntityId>,
        selected_room: &mut Option<usize>,
        sidebar: &mut Sidebar,
        action_queue: &mut ActionQueue,
        drag_selection: &mut DragSelection,
    ) {
        match phase {
            GamePhase::Loading => {
                // Loading is handled asynchronously in main loop
            }
            GamePhase::MainMenu => {
                Self::handle_main_menu(selected_map_type, phase, game_data);
            }
            GamePhase::Playing(state) => {
                if let Some(ref data) = game_data {
                    if !state.paused {
                        state.update(dt, data);
                    }
                    
                    if Self::handle_playing(
                        dt, state, data, interaction_mode, hovered_tile, 
                        held_entity, selected_entity, selected_room, sidebar,
                        action_queue, drag_selection
                    ) {
                        *phase = GamePhase::MainMenu;
                    }
                }
            }
        }
    }

    fn handle_main_menu(
        selected_map_type: &mut MapType, 
        phase: &mut GamePhase, 
        game_data: &Option<GameData>
    ) {
        let mouse_pos = mouse_position();

        // Map type selection buttons (coords matched from original)
        // Map type selection buttons (coords matched from original)
        let map_button_y = screen_height() / 2.0 - 50.0;
        let map_button_width = 160.0;
        let map_button_height = 40.0;
        let map_button_spacing = 10.0;
        let total_width = (map_button_width + map_button_spacing) * 5.0 - map_button_spacing; // 5 buttons now
        let map_button_start_x = screen_width() / 2.0 - total_width / 2.0;

        let map_types = [
            (MapType::Standard, 0),
            (MapType::Rich, 1),
            (MapType::Hazardous, 2),
            (MapType::Test, 3),
            (MapType::File("assets/maps/level_1.json".to_string()), 4),
        ];

        if is_mouse_button_pressed(MouseButton::Left) {
            for (map_type, index) in &map_types {
                let btn_x = map_button_start_x + (map_button_width + map_button_spacing) * (*index as f32);
                if mouse_pos.0 >= btn_x
                    && mouse_pos.0 <= btn_x + map_button_width
                    && mouse_pos.1 >= map_button_y
                    && mouse_pos.1 <= map_button_y + map_button_height
                {
                    *selected_map_type = map_type.clone();
                }
            }
        }

        // Check for SPACE key or mouse click on start button
        let button_x = screen_width() / 2.0 - 100.0;
        let button_y = screen_height() / 2.0 + 60.0;
        let button_width = 200.0;
        let button_height = 50.0;

        let should_start = is_key_pressed(KeyCode::Space)
            || (is_mouse_button_pressed(MouseButton::Left)
                && mouse_pos.0 >= button_x
                && mouse_pos.0 <= button_x + button_width
                && mouse_pos.1 >= button_y
                && mouse_pos.1 <= button_y + button_height);

        if should_start {
            if let Some(ref data) = game_data {
                let game_state = GameState::new_with_map_type(
                    50,
                    50,
                    data,
                    selected_map_type.clone(),
                );
                *phase = GamePhase::Playing(game_state);
            }
        }
        
        // Handle Load Game
         let load_y = button_y + button_height + 20.0; // Spacing logic matched with renderer
         let should_load = is_mouse_button_pressed(MouseButton::Left)
            && mouse_pos.0 >= button_x
            && mouse_pos.0 <= button_x + button_width
            && mouse_pos.1 >= load_y
            && mouse_pos.1 <= load_y + button_height;

        if should_load {
             // Check if save exists first
            if crate::state::save_system::save_exists("slot_1") {
                match crate::state::save_system::load_game("slot_1") {
                    Ok(loaded_state) => {
                        println!("Game loaded successfully!");
                        *phase = GamePhase::Playing(loaded_state);
                    }
                    Err(e) => {
                        eprintln!("Failed to load game: {}", e);
                    }
                }
            }
        }
    }

    fn handle_playing(
        dt: f32,
        state: &mut GameState,
        game_data: &GameData,
        interaction_mode: &mut InteractionMode,
        hovered_tile: &mut Option<TilePos>,
        held_entity: &mut Option<EntityId>,
        selected_entity: &mut Option<EntityId>,
        selected_room: &mut Option<usize>,
        sidebar: &mut Sidebar,
        action_queue: &mut ActionQueue,
        drag_selection: &mut DragSelection,
    ) -> bool {
        // Handle Game Over Input
        if state.game_over {
            if is_key_pressed(KeyCode::Escape) {
                return true; // Return to Main Menu
            }
            return false; // Block other input
        }

        if is_key_pressed(KeyCode::Escape) {
            state.paused = !state.paused;
            // Clear selection when pausing/unpausing
            *interaction_mode = InteractionMode::None;
            sidebar.clear_selection();
            drag_selection.cancel();
        }

        if state.paused {
            // Handle Pause Menu Input
            let screen_center_x = screen_width() / 2.0;
            let screen_center_y = screen_height() / 2.0;
            let button_width = 200.0;
            let button_height = 50.0;
            let spacing = 20.0;
            let start_y = screen_center_y - 100.0; // Matches renderer

            let mouse_pos = mouse_position();
            let is_click = is_mouse_button_pressed(MouseButton::Left);

            // Resume Button
            let resume_y = start_y;
            if is_click && 
               mouse_pos.0 >= screen_center_x - button_width / 2.0 && 
               mouse_pos.0 <= screen_center_x + button_width / 2.0 &&
               mouse_pos.1 >= resume_y && 
               mouse_pos.1 <= resume_y + button_height 
            {
                state.paused = false;
            }

            // Save Game Button
            let save_y = start_y + button_height + spacing;
            if is_click && 
               mouse_pos.0 >= screen_center_x - button_width / 2.0 && 
               mouse_pos.0 <= screen_center_x + button_width / 2.0 &&
               mouse_pos.1 >= save_y && 
               mouse_pos.1 <= save_y + button_height 
            {
                match crate::state::save_system::save_game(state, "slot_1") {
                    Ok(_) => {
                        state.notifications.success("Game saved successfully!");
                        eprintln!("Game saved to slot_1");
                    }
                    Err(e) => {
                        state.notifications.danger(format!("Save failed: {}", e));
                        eprintln!("Failed to save game: {}", e);
                    }
                }
            }

            // Load Game Button
            let load_y = start_y + (button_height + spacing) * 2.0;
            if is_click && 
               mouse_pos.0 >= screen_center_x - button_width / 2.0 && 
               mouse_pos.0 <= screen_center_x + button_width / 2.0 &&
               mouse_pos.1 >= load_y && 
               mouse_pos.1 <= load_y + button_height 
            {
                if crate::state::save_system::save_exists("slot_1") {
                    match crate::state::save_system::load_game("slot_1") {
                        Ok(loaded_state) => {
                            *state = loaded_state;
                            state.notifications.success("Game loaded!");
                            eprintln!("Game loaded from slot_1");
                        }
                        Err(e) => {
                            state.notifications.danger(format!("Load failed: {}", e));
                            eprintln!("Failed to load game: {}", e);
                        }
                    }
                }
            }

            // Main Menu Button
            let menu_y = start_y + (button_height + spacing) * 3.0;
            if is_click && 
               mouse_pos.0 >= screen_center_x - button_width / 2.0 && 
               mouse_pos.0 <= screen_center_x + button_width / 2.0 &&
               mouse_pos.1 >= menu_y && 
               mouse_pos.1 <= menu_y + button_height 
            {
               return true;
            }

            // Exit Button
            let exit_y = start_y + (button_height + spacing) * 4.0;
            if is_click && 
               mouse_pos.0 >= screen_center_x - button_width / 2.0 && 
               mouse_pos.0 <= screen_center_x + button_width / 2.0 &&
               mouse_pos.1 >= exit_y && 
               mouse_pos.1 <= exit_y + button_height 
            {
                std::process::exit(0);
            }
            
            return false; // Skip normal game input when paused
        }

        // Camera controls
        Self::handle_camera_controls(dt, state);

        // Set up Camera3D for input handling
        let camera = state.camera.get_camera3d();

        // Mode switching via keyboard
        Self::handle_mode_switching(interaction_mode, sidebar, drag_selection);

        // Update sidebar layout
        sidebar.update_layout();

        // Get hovered tile position
        let mouse_pos = mouse_position();
        let tile_pos = crate::engine::tile_grid::screen_to_tile(
            mouse_pos.0,
            mouse_pos.1,
            &camera,
            0.0, 0.0,
            Some(&state.dungeon.grid),
            game_data
        );
        *hovered_tile = Some(tile_pos);

        // Handle Sidebar Input
        if let Some(new_mode) = sidebar.handle_input(
            &state.player, 
            game_data, 
            interaction_mode,
            *held_entity,
            action_queue
        ) {
            *interaction_mode = new_mode;
            drag_selection.cancel(); // Cancel any active drag when mode changes
        }

        // Check if mouse is over UI
        let mouse_over_ui = sidebar.is_mouse_over();

        // Handle spell casting
        Self::handle_spell_casting(state, game_data, sidebar, tile_pos, mouse_over_ui);

        // Handle right-click actions
        Self::handle_right_click(
            state, game_data, interaction_mode, selected_entity, 
            sidebar, tile_pos, mouse_over_ui, drag_selection
        );

        // Handle drag selection for applicable modes
        if Self::is_drag_mode(interaction_mode) {
            // Start drag on mouse press
            if is_mouse_button_pressed(MouseButton::Left) && !mouse_over_ui {
                drag_selection.start(tile_pos);
            }

            // Update drag while held
            if is_mouse_button_down(MouseButton::Left) && drag_selection.active {
                drag_selection.update(tile_pos);
            }

            // Finalize drag on release
            if is_mouse_button_released(MouseButton::Left) && drag_selection.active {
                // If mouse is over UI on release, cancel the drag
                if mouse_over_ui {
                    drag_selection.cancel();
                } else if let Some((min, max)) = drag_selection.finish() {
                    Self::apply_drag_action(state, game_data, interaction_mode, min, max);
                }
            }
        } else {
            // Single-click modes (Pickup, Drop, Inspect, None)
            if is_mouse_button_pressed(MouseButton::Left) && !mouse_over_ui {
                Self::handle_tile_interaction(
                    state, game_data, interaction_mode, held_entity, 
                    selected_entity, selected_room, tile_pos, sidebar
                );
            }
        }
        
        false
    }


    /// Handle WASD camera movement, Q/E rotation, and scroll zoom
    fn handle_camera_controls(dt: f32, state: &mut GameState) {
        let camera_speed = 30.0 * dt;
        let (sin, cos) = (state.camera.angle + std::f32::consts::FRAC_PI_2).sin_cos();

        // Forward/Back (W/S)
        if is_key_down(KeyCode::W) {
            state.camera.target.0 -= camera_speed * cos;
            state.camera.target.2 -= camera_speed * sin;
        }
        if is_key_down(KeyCode::S) {
            state.camera.target.0 += camera_speed * cos;
            state.camera.target.2 += camera_speed * sin;
        }

        // Left/Right (A/D)
        if is_key_down(KeyCode::A) {
            state.camera.target.0 -= camera_speed * sin;
            state.camera.target.2 += camera_speed * cos;
        }
        if is_key_down(KeyCode::D) {
            state.camera.target.0 += camera_speed * sin;
            state.camera.target.2 -= camera_speed * cos;
        }

        // Rotation (Q/E)
        let rotation_speed = 2.0 * dt;
        if is_key_down(KeyCode::Q) {
            state.camera.angle -= rotation_speed;
        }
        if is_key_down(KeyCode::E) {
            state.camera.angle += rotation_speed;
        }

        // Zoom control (scroll wheel)
        let scroll = mouse_wheel().1;
        if scroll > 0.0 {
            state.camera.zoom_in();
        } else if scroll < 0.0 {
            state.camera.zoom_out();
        }
    }

    /// Handle keyboard shortcuts for switching interaction modes
    fn handle_mode_switching(interaction_mode: &mut InteractionMode, sidebar: &mut Sidebar, drag_selection: &mut DragSelection) {
        let old_mode = interaction_mode.clone();
        if is_key_pressed(KeyCode::Key1) { *interaction_mode = InteractionMode::Dig; }
        if is_key_pressed(KeyCode::Key2) { *interaction_mode = InteractionMode::BuildRoom("lair".to_string()); }
        if is_key_pressed(KeyCode::Key3) { *interaction_mode = InteractionMode::BuildRoom("hatchery".to_string()); }
        if is_key_pressed(KeyCode::Key4) { *interaction_mode = InteractionMode::BuildRoom("treasury".to_string()); }
        if is_key_pressed(KeyCode::Key5) { *interaction_mode = InteractionMode::PlaceSpawner; }
        if is_key_pressed(KeyCode::T) { *interaction_mode = InteractionMode::BuildRoom("training_room".to_string()); }
        if is_key_pressed(KeyCode::L) { *interaction_mode = InteractionMode::BuildRoom("library".to_string()); }
        if is_key_pressed(KeyCode::Escape) {
            *interaction_mode = InteractionMode::None;
            sidebar.clear_selection();
            drag_selection.cancel();
        }
        // Cancel drag if mode changed
        if *interaction_mode != old_mode {
            drag_selection.cancel();
        }
    }

    /// Handle spell casting when a spell is selected
    fn handle_spell_casting(
        state: &mut GameState,
        game_data: &GameData,
        sidebar: &Sidebar,
        tile_pos: TilePos,
        mouse_over_ui: bool,
    ) {
        if let Some(spell_id) = sidebar.get_selected_spell().cloned() {
            if is_mouse_button_pressed(MouseButton::Left) && !mouse_over_ui {
                let target_pos = Some(tile_pos);
                let target_entity = None;

                let cast_result = spell_effects::cast_spell(
                    &spell_id,
                    state,
                    game_data,
                    target_pos,
                    target_entity,
                );

                match cast_result {
                    spell_effects::CastResult::Success => {
                        eprintln!("Spell cast successfully: {}", spell_id);
                        state.player.record_spell_cast(spell_id.clone());
                    }
                    spell_effects::CastResult::MaxCapReached => {
                        eprintln!("Cannot cast {}: Maximum unit capacity reached!", spell_id);
                    }
                    spell_effects::CastResult::InsufficientMana => {
                        eprintln!("Cannot cast {}: Not enough mana!", spell_id);
                    }
                    spell_effects::CastResult::InsufficientGold => {
                        eprintln!("Cannot cast {}: Not enough gold!", spell_id);
                    }
                    spell_effects::CastResult::OnCooldown => {
                        eprintln!("Cannot cast {}: Spell is on cooldown!", spell_id);
                    }
                    _ => eprintln!("Spell cast failed: {:?}", cast_result),
                }
            }
        }
    }

    /// Handle right-click actions (cancel, unmark, slap)
    fn handle_right_click(
        state: &mut GameState,
        game_data: &GameData,
        interaction_mode: &mut InteractionMode,
        selected_entity: &mut Option<EntityId>,
        sidebar: &mut Sidebar,
        tile_pos: TilePos,
        mouse_over_ui: bool,
        drag_selection: &mut DragSelection,
    ) {
        if !is_mouse_button_pressed(MouseButton::Right) || mouse_over_ui {
            return;
        }

        drag_selection.cancel();

        match interaction_mode {
            InteractionMode::Dig => {
                if let Some(tile) = state.get_tile_mut(tile_pos) {
                    tile.marked_for_dig = false;
                }
            }
            InteractionMode::None => {
                if !Self::try_slap_creature(state, game_data, tile_pos) {
                    *selected_entity = None;
                    sidebar.clear_selection();
                }
            }
            _ => {
                *interaction_mode = InteractionMode::None;
                sidebar.clear_selection();
            }
        }
    }

    /// Try to slap a creature at the given position, returns true if successful
    fn try_slap_creature(state: &mut GameState, game_data: &GameData, tile_pos: TilePos) -> bool {
        let entity = match state.entities.at_position_mut(tile_pos).next() {
            Some(e) => e,
            None => return false,
        };
        let creature = match entity.as_creature_mut() {
            Some(c) => c,
            None => return false,
        };
        let monster_data = match game_data.monsters.get(&creature.creature_id) {
            Some(d) => d,
            None => return false,
        };

        creature_ai::apply_slap(creature, monster_data, state.time_elapsed);
        eprintln!("Slapped creature {}!", creature.creature_id);
        true
    }

    /// Handle left-click tile interactions based on current mode
    fn handle_tile_interaction(
        state: &mut GameState,
        game_data: &GameData,
        interaction_mode: &mut InteractionMode,
        held_entity: &mut Option<EntityId>,
        selected_entity: &mut Option<EntityId>,
        selected_room: &mut Option<usize>,
        tile_pos: TilePos,
        sidebar: &mut Sidebar,
    ) {
        match interaction_mode {
            InteractionMode::Dig => {
                crate::engine::input_handlers::handle_dig(state, game_data, tile_pos);
            }
            InteractionMode::BuildRoom(room_type) => {
                crate::engine::input_handlers::handle_build_room(state, game_data, room_type, tile_pos);
            }
            InteractionMode::BuildTrap(trap_type) => {
                crate::engine::input_handlers::handle_build_trap(state, game_data, trap_type, tile_pos);
            }
            InteractionMode::PlaceSpawner => {
                crate::engine::input_handlers::handle_place_spawner(state, game_data, tile_pos);
            }
            InteractionMode::Pickup => {
                crate::engine::input_handlers::handle_pickup(state, held_entity, interaction_mode, tile_pos);
            }
            InteractionMode::Drop => {
                crate::engine::input_handlers::handle_drop(state, held_entity, interaction_mode, tile_pos, game_data);
            }
            InteractionMode::Sell => {
                crate::engine::input_handlers::handle_sell(state, game_data, tile_pos);
            }
            InteractionMode::Inspect => {
                crate::engine::input_handlers::handle_inspect(state, selected_entity, selected_room, tile_pos);
            }
            InteractionMode::None => {
                let found = crate::engine::input_handlers::select_entity_or_room(state, selected_entity, selected_room, tile_pos);
                if found {
                    sidebar.switch_to_tab(SidebarTab::Minions);
                }
            }
            InteractionMode::SetAttackMarker => {
                Self::handle_set_marker(state, "attack", tile_pos);
            }
            InteractionMode::SetDefendMarker => {
                Self::handle_set_marker(state, "defend", tile_pos);
            }
            InteractionMode::SaveGame => {
                // No tile interaction for SaveGame
            }
        }
    }

    fn handle_set_marker(state: &mut GameState, marker_type: &str, tile_pos: TilePos) {
        let (width, height) = crate::engine::tile_grid::get_grid_dimensions(&state.dungeon.grid);
        // Allow placing markers on any walkable tile or known tile?
        // For now, allow anywhere inside map bounds
        if tile_pos.x >= 0 && tile_pos.y >= 0 && tile_pos.x < width as i32 && tile_pos.y < height as i32 {
            if marker_type == "attack" {
                state.attack_marker = Some(tile_pos);
                eprintln!("Attack marker set at {:?}", tile_pos);
                state.notifications.info("Attack flag updated");
            } else if marker_type == "defend" {
                 state.defend_marker = Some(tile_pos);
                 eprintln!("Defend marker set at {:?}", tile_pos);
                 state.notifications.info("Defend flag updated");
            }
        }
    }

    /// Check if the current interaction mode supports drag selection
    fn is_drag_mode(mode: &InteractionMode) -> bool {
        matches!(
            mode,
            InteractionMode::Dig
                | InteractionMode::BuildRoom(_)
                | InteractionMode::BuildTrap(_)
                | InteractionMode::PlaceSpawner
        )
    }

    /// Apply the drag action to all tiles in the selection
    fn apply_drag_action(
        state: &mut GameState,
        game_data: &GameData,
        mode: &InteractionMode,
        min: TilePos,
        max: TilePos,
    ) {
        let tiles: Vec<TilePos> = (min.y..=max.y)
            .flat_map(|y| (min.x..=max.x).map(move |x| TilePos::new(x, y)))
            .collect();

        match mode {
            InteractionMode::Dig => {
                crate::engine::input_handlers::handle_dig_multi(state, game_data, &tiles);
            },
            InteractionMode::BuildRoom(room_type) => {
                crate::engine::input_handlers::handle_build_room_multi(state, game_data, room_type, &tiles);
            }
            InteractionMode::BuildTrap(trap_type) => {
                crate::engine::input_handlers::handle_build_trap_multi(state, game_data, trap_type, &tiles);
            }
            InteractionMode::PlaceSpawner => {
                crate::engine::input_handlers::handle_place_spawner_multi(state, game_data, &tiles);
            }
            _ => {}
        }
    }
}
