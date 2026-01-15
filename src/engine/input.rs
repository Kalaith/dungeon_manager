use macroquad::prelude::*;
use crate::state::game_state::GameState;
use crate::state::{GamePhase, InteractionMode, TilePos, MapType, Ownership};
use crate::data::GameData;
use crate::state::entities::EntityId;
use crate::ui::sidebar::{Sidebar, SidebarTab};

use crate::engine::spell_effects;
use crate::engine::creature_ai;
use crate::engine::tile_types::{self, types as tt};
use crate::ui::actions::{ActionQueue, UiAction, SpellTarget};

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
                        action_queue
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
        let map_button_y = screen_height() / 2.0 - 50.0;
        let map_button_width = 160.0;
        let map_button_height = 40.0;
        let map_button_spacing = 10.0;
        let total_width = (map_button_width + map_button_spacing) * 4.0 - map_button_spacing;
        let map_button_start_x = screen_width() / 2.0 - total_width / 2.0;

        let map_types = [
            (MapType::Standard, 0),
            (MapType::Rich, 1),
            (MapType::Hazardous, 2),
            (MapType::Test, 3),
        ];

        if is_mouse_button_pressed(MouseButton::Left) {
            for (map_type, index) in &map_types {
                let btn_x = map_button_start_x + (map_button_width + map_button_spacing) * (*index as f32);
                if mouse_pos.0 >= btn_x
                    && mouse_pos.0 <= btn_x + map_button_width
                    && mouse_pos.1 >= map_button_y
                    && mouse_pos.1 <= map_button_y + map_button_height
                {
                    *selected_map_type = *map_type;
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
                    *selected_map_type,
                );
                *phase = GamePhase::Playing(game_state);
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
    ) -> bool {
        if is_key_pressed(KeyCode::Escape) {
            state.paused = !state.paused;
            // Clear selection when pausing/unpausing
            *interaction_mode = InteractionMode::None;
            sidebar.clear_selection();
        }

        if state.paused {
            // Handle Pause Menu Input
            let screen_center_x = screen_width() / 2.0;
            let screen_center_y = screen_height() / 2.0;
            let button_width = 200.0;
            let button_height = 50.0;
            let spacing = 20.0;
            let start_y = screen_center_y - 50.0;

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

            // Main Menu Button
            let menu_y = start_y + button_height + spacing;
            if is_click && 
               mouse_pos.0 >= screen_center_x - button_width / 2.0 && 
               mouse_pos.0 <= screen_center_x + button_width / 2.0 &&
               mouse_pos.1 >= menu_y && 
               mouse_pos.1 <= menu_y + button_height 
            {
               return true;
            }

            // Exit Button
            let exit_y = start_y + (button_height + spacing) * 2.0;
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
        Self::handle_mode_switching(interaction_mode, sidebar);

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
        }

        // Check if mouse is over UI
        let mouse_over_ui = sidebar.is_mouse_over();

        // Handle spell casting
        Self::handle_spell_casting(state, game_data, sidebar, tile_pos, mouse_over_ui);

        // Handle right-click actions
        Self::handle_right_click(
            state, game_data, interaction_mode, selected_entity, 
            sidebar, tile_pos, mouse_over_ui
        );

        // Handle left-click tile interactions
        if is_mouse_button_pressed(MouseButton::Left) && !mouse_over_ui {
            Self::handle_tile_interaction(
                state, game_data, interaction_mode, held_entity, 
                selected_entity, selected_room, tile_pos, sidebar
            );
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
    fn handle_mode_switching(interaction_mode: &mut InteractionMode, sidebar: &mut Sidebar) {
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
    ) {
        if !is_mouse_button_pressed(MouseButton::Right) || mouse_over_ui {
            return;
        }

        match interaction_mode {
            InteractionMode::Dig => {
                if let Some(tile) = state.get_tile_mut(tile_pos) {
                    tile.marked_for_dig = false;
                }
            }
            InteractionMode::None => {
                // Try to slap a creature, otherwise deselect
                if let Some(entity) = state.entities.at_position_mut(tile_pos).next() {
                    if let Some(creature) = entity.as_creature_mut() {
                        if let Some(monster_data) = game_data.monsters.get(&creature.creature_id) {
                            creature_ai::apply_slap(creature, monster_data, state.time_elapsed);
                            eprintln!("Slapped creature {}!", creature.creature_id);
                            return;
                        }
                    }
                }
                *selected_entity = None;
                sidebar.clear_selection();
            }
            _ => {
                *interaction_mode = InteractionMode::None;
                sidebar.clear_selection();
            }
        }
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
                Self::handle_dig(state, game_data, tile_pos);
            }
            InteractionMode::BuildRoom(room_type) => {
                Self::handle_build_room(state, game_data, room_type, tile_pos);
            }
            InteractionMode::BuildTrap(trap_type) => {
                Self::handle_build_trap(state, game_data, trap_type, tile_pos);
            }
            InteractionMode::PlaceSpawner => {
                Self::handle_place_spawner(state, game_data, tile_pos);
            }
            InteractionMode::Pickup => {
                Self::handle_pickup(state, held_entity, interaction_mode, tile_pos);
            }
            InteractionMode::Drop => {
                Self::handle_drop(state, held_entity, interaction_mode, tile_pos, game_data);
            }
            InteractionMode::Sell => {
                Self::handle_sell(state, game_data, tile_pos);
            }
            InteractionMode::Inspect => {
                Self::handle_inspect(state, selected_entity, selected_room, tile_pos);
            }
            InteractionMode::None => {
                // Select entity or room on click
                let mut found_something = false;

                // Try entity first
                if let Some(entity) = state.entities.at_position(tile_pos).next() {
                    *selected_entity = Some(entity.id);
                    *selected_room = None;
                    found_something = true;
                } else {
                    *selected_entity = None;
                    // Try room
                    if let Some(tile) = state.get_tile(tile_pos) {
                        if let Some(room_id) = tile.room_id {
                            *selected_room = Some(room_id);
                            found_something = true;
                        } else {
                            *selected_room = None;
                        }
                    }
                }

                if found_something {
                    sidebar.switch_to_tab(SidebarTab::Minions);
                }
            }
        }
    }

    fn handle_dig(state: &mut GameState, game_data: &GameData, tile_pos: TilePos) {
        if let Some(tile) = state.get_tile_mut(tile_pos) {
            if tile_types::is_diggable(&tile.tile_type, game_data) && tile.ownership == Ownership::Unclaimed {
                 tile.marked_for_dig = !tile.marked_for_dig;
            }
        }
    }

    fn handle_build_room(state: &mut GameState, game_data: &GameData, room_type: &str, tile_pos: TilePos) {
        let cost = game_data.rooms
            .get(room_type)
            .map(|data| data.build.cost_per_tile)
            .unwrap_or_else(|| panic!("Room type '{}' missing in rooms.json", room_type));

        let can_build = state.player.gold >= cost;
        let is_valid_tile = if let Some(tile) = state.get_tile(tile_pos) {
            tile.ownership == Ownership::Player && tile.room_id.is_none() && tile_types::can_build_room(&tile.tile_type, game_data)
        } else { 
            false 
        };

        if !state.player.is_room_unlocked(room_type) {
            eprintln!("Cannot build {}: Not yet unlocked!", room_type);
            return;
        }

        if can_build && is_valid_tile {
            state.player.gold -= cost;
            if let Some(tile) = state.get_tile_mut(tile_pos) {
                tile.tile_type = room_type.to_string();
            }
            state.detect_and_update_rooms(game_data);
        }
    }

    fn handle_build_trap(state: &mut GameState, game_data: &GameData, trap_type: &str, tile_pos: TilePos) {
        // Check for Workshop requirement
        let has_workshop = state.room_manager.rooms.iter().any(|r| r.room_type == "workshop");
        if !has_workshop {
            eprintln!("Cannot build trap: No functioning Workshop!");
            return;
        }

        // Add to pending builds
        if let Some(tile) = state.get_tile_mut(tile_pos) {
            if tile.ownership == Ownership::Player 
                && tile_types::can_build_room(&tile.tile_type, game_data) 
                && tile.trap.is_none() 
            {
                 // Create trap in "unconstructed" state
                 tile.trap = Some(crate::state::tile_state::TrapState {
                     trap_type: trap_type.to_string(), // Changed from .to_string()
                     constructed: false,
                     construction_progress: 0.0,
                     active: false,
                     funded: false,
                 });
                 
                 state.pending_trap_builds.insert(tile_pos);
                 eprintln!("Trap '{}' placement started at {:?}. Waiting for construction.", trap_type, tile_pos);
            }
        }
    }

    fn handle_place_spawner(state: &mut GameState, game_data: &GameData, tile_pos: TilePos) {
        if state.player.gold >= crate::config::SPAWNER_COST {
            if let Some(tile) = state.get_tile(tile_pos) {
                if tile.ownership == Ownership::Player && tile_types::can_build_room(&tile.tile_type, game_data) {
                    state.player.gold -= crate::config::SPAWNER_COST;
                    if let Some(tile_mut) = state.get_tile_mut(tile_pos) {
                        tile_mut.tile_type = tt::MONSTER_SPAWNER.to_string();
                    }
                }
            }
        }
    }

    fn handle_pickup(
        state: &mut GameState, 
        held_entity: &mut Option<EntityId>, 
        interaction_mode: &mut InteractionMode,
        tile_pos: TilePos
    ) {
        if let Some(entity) = state.entities.at_position(tile_pos).next() {
            *held_entity = Some(entity.id);
            eprintln!("Picked up entity: {}", entity.id);
            *interaction_mode = InteractionMode::Drop;
        }
    }

    fn handle_drop(
        state: &mut GameState, 
        held_entity: &mut Option<EntityId>, 
        interaction_mode: &mut InteractionMode,
        tile_pos: TilePos,
        game_data: &GameData,
    ) {
        if let Some(entity_id) = *held_entity {
            if let Some(tile) = state.get_tile(tile_pos) {
                let is_walkable = tile.ownership == Ownership::Player
                    && tile_types::is_walkable(&tile.tile_type, game_data);

                if is_walkable {
                    if let Some(entity) = state.entities.get_mut(entity_id) {
                        entity.pos = tile_pos;
                        // Reset creature state
                        if let Some(creature) = entity.as_creature_mut() {
                            creature.current_path = None;
                            creature.current_task = None;
                            creature.task_time = 0.0;
                            creature.move_timer = 0.0;
                        }

                        eprintln!("Dropped entity {} at {:?}", entity_id, tile_pos);
                        *held_entity = None;
                        *interaction_mode = InteractionMode::Pickup;
                    }
                }
            }
        }
    }

    fn handle_sell(state: &mut GameState, game_data: &GameData, tile_pos: TilePos) {
        let mut action = None;
        if let Some(tile) = state.get_tile(tile_pos) {
            if tile.marked_for_dig {
                action = Some("unmark");
            } else if state.player.is_room_unlocked(&tile.tile_type) && tile.ownership == Ownership::Player {
                action = Some("sell");
            }
        }

        match action {
            Some("unmark") => {
                if let Some(tile) = state.get_tile_mut(tile_pos) {
                    tile.marked_for_dig = false;
                }
            }
            Some("sell") => {
                state.player.gold += 5;
                if let Some(tile) = state.get_tile_mut(tile_pos) {
                    tile.tile_type = tt::CLAIMED_FLOOR.to_string();
                    tile.room_id = None;
                }
                state.detect_and_update_rooms(game_data);
            }
            _ => {}
        }
    }

    fn handle_inspect(
        state: &GameState, 
        selected_entity: &mut Option<EntityId>, 
        selected_room: &mut Option<usize>,
        tile_pos: TilePos
    ) {
        let mut found_entity = false;
        if let Some(entity) = state.entities.at_position(tile_pos).next() {
            *selected_entity = Some(entity.id);
            *selected_room = None;
            found_entity = true;
        }

        if !found_entity {
            if let Some(tile) = state.get_tile(tile_pos) {
                if let Some(room_id) = tile.room_id {
                    *selected_room = Some(room_id);
                    *selected_entity = None;
                } else {
                    *selected_room = None;
                    *selected_entity = None;
                }
            }
        }
    }
}
