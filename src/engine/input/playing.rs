//! Per-frame input handling while a game is in progress: game-over and pause
//! menus, camera controls, interaction-mode switching, and dispatch to the
//! tile/spell interaction handlers in `tile_actions`.

use crate::data::GameData;
use crate::state::entities::EntityId;
use crate::state::game_state::GameState;
use crate::state::{DragSelection, InteractionMode, TilePos};
use crate::ui::actions::ActionQueue;
use crate::ui::sidebar::Sidebar;
use macroquad::prelude::*;

use super::tile_actions;

/// Handle all input for the `Playing` phase. Returns `true` when the player
/// asked to return to the main menu.
pub(super) fn handle_playing(
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
        if state.victory && state.has_pending_campaign_mission(game_data) {
            let next_rect = crate::ui::menu_layout::game_over_next_mission();
            let mouse_pos = mouse_position();
            let next_clicked = is_mouse_button_pressed(MouseButton::Left)
                && next_rect.contains(vec2(mouse_pos.0, mouse_pos.1));

            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) || next_clicked {
                if let Some(next_state) = state.start_pending_campaign_mission(game_data) {
                    *state = next_state;
                    *interaction_mode = InteractionMode::None;
                    sidebar.clear_selection();
                    drag_selection.cancel();
                }
                return false;
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            return true; // Return to Main Menu
        }
        return false; // Block other input
    }

    if is_key_pressed(KeyCode::Escape) {
        action_queue.push(crate::ui::actions::UiAction::TogglePause);

        // Clear selection logic remains here as Sidebar is UI-owned
        *interaction_mode = InteractionMode::None;
        sidebar.clear_selection();
        drag_selection.cancel();
    }

    if state.paused {
        // Handle Pause Menu Input (rects shared with ui::menus)
        let layout = crate::ui::menu_layout::pause_menu();
        let mouse = mouse_position();
        let mouse = vec2(mouse.0, mouse.1);
        let clicked = |rect: macroquad::math::Rect| {
            is_mouse_button_pressed(MouseButton::Left) && rect.contains(mouse)
        };

        if clicked(layout.resume) {
            state.paused = false;
        }

        if clicked(layout.save) {
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

        if clicked(layout.load) && crate::state::save_system::save_exists("slot_1") {
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

        if clicked(layout.main_menu) {
            return true;
        }

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(exit) = layout.exit {
            if clicked(exit) {
                std::process::exit(0);
            }
        }

        return false; // Skip normal game input when paused
    }

    // Camera controls
    handle_camera_controls(dt, state);

    // Set up Camera3D for input handling
    let camera = state.camera.get_camera3d();

    // Mode switching via keyboard
    handle_mode_switching(interaction_mode, sidebar, drag_selection);

    // Update sidebar layout
    sidebar.update_layout();

    // Get hovered tile position
    let mouse_pos = mouse_position();
    let tile_pos = crate::engine::tile_grid::screen_to_tile(
        mouse_pos.0,
        mouse_pos.1,
        &camera,
        0.0,
        0.0,
        Some(&state.dungeon.grid),
        game_data,
    );
    *hovered_tile = Some(tile_pos);

    // Handle Sidebar Input
    if let Some(new_mode) = sidebar.handle_input(
        &state.player,
        game_data,
        interaction_mode,
        *held_entity,
        action_queue,
    ) {
        *interaction_mode = new_mode;
        drag_selection.cancel(); // Cancel any active drag when mode changes
    }

    // Check if mouse is over UI
    let mouse_over_ui = sidebar.is_mouse_over();

    // Handle spell casting
    tile_actions::handle_spell_casting(state, game_data, sidebar, tile_pos, mouse_over_ui);

    // Handle right-click actions
    tile_actions::handle_right_click(
        state,
        game_data,
        interaction_mode,
        selected_entity,
        sidebar,
        tile_pos,
        mouse_over_ui,
        drag_selection,
    );

    // Handle drag selection for applicable modes
    if tile_actions::is_drag_mode(interaction_mode) {
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
                tile_actions::apply_drag_action(
                    state,
                    game_data,
                    interaction_mode,
                    min,
                    max,
                    sidebar,
                    action_queue,
                );
            }
        }
    } else {
        // Single-click modes (Pickup, Drop, Inspect, None)
        if is_mouse_button_pressed(MouseButton::Left) && !mouse_over_ui {
            tile_actions::handle_tile_interaction(
                state,
                game_data,
                interaction_mode,
                held_entity,
                selected_entity,
                selected_room,
                tile_pos,
                sidebar,
                action_queue,
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
fn handle_mode_switching(
    interaction_mode: &mut InteractionMode,
    sidebar: &mut Sidebar,
    drag_selection: &mut DragSelection,
) {
    let old_mode = interaction_mode.clone();
    if is_key_pressed(KeyCode::Key1) {
        *interaction_mode = InteractionMode::Dig;
    }
    if is_key_pressed(KeyCode::Key2) {
        *interaction_mode = InteractionMode::BuildRoom("lair".to_string());
    }
    if is_key_pressed(KeyCode::Key3) {
        *interaction_mode = InteractionMode::BuildRoom("hatchery".to_string());
    }
    if is_key_pressed(KeyCode::Key4) {
        *interaction_mode = InteractionMode::BuildRoom("treasury".to_string());
    }
    if is_key_pressed(KeyCode::Key5) {
        *interaction_mode = InteractionMode::PlaceSpawner;
    }
    if is_key_pressed(KeyCode::T) {
        *interaction_mode = InteractionMode::BuildRoom("training_room".to_string());
    }
    if is_key_pressed(KeyCode::L) {
        *interaction_mode = InteractionMode::BuildRoom("library".to_string());
    }
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
