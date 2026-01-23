//! Action Processor
//! Processes UI actions and applies them to game state

use crate::state::game_state::GameState;
use crate::state::tile_state::Ownership;
use crate::ui::actions::{ActionQueue, UiAction};
use crate::InteractionMode;

/// Result of processing an action
#[derive(Debug, Default)]
pub struct ActionResult {
    /// New interaction mode if action changed it
    pub new_mode: Option<InteractionMode>,
    /// Clear entity selection
    pub deselect_entity: bool,
    /// Clear room selection
    pub deselect_room: bool,
}

/// Process all queued actions
pub fn process_actions(
    actions: &mut ActionQueue,
    game_state: &mut GameState,
    interaction_mode: &mut InteractionMode,
    selected_spell: &mut Option<String>,
) {
    for action in actions.drain() {
        let result = process_single_action(
            action,
            game_state,
            selected_spell,
        );

        // Apply results
        if let Some(mode) = result.new_mode {
            *interaction_mode = mode;
        }
        if result.deselect_entity {
            // Selection handled elsewhere now
        }
        if result.deselect_room {
            // Selection handled elsewhere now
        }
    }
}

/// Process a single action
fn process_single_action(
    action: UiAction,
    game_state: &mut GameState,
    selected_spell: &mut Option<String>,
) -> ActionResult {
    let mut result = ActionResult::default();

    match action {
        UiAction::TogglePause => {
            game_state.paused = !game_state.paused;
            if game_state.paused {
                result.new_mode = Some(InteractionMode::None);
                result.deselect_entity = true;
                result.deselect_room = true;
                *selected_spell = None;
            }
        }

        UiAction::StartResearch(tech_id) => {
            game_state.player.start_research(tech_id);
        }

        UiAction::SaveGame => {
            if let Err(e) = crate::state::save_system::save_game(game_state, "slot_1") {
                eprintln!("Failed to save game: {}", e);
                game_state.notifications.danger(format!("Save Failed: {}", e));
            } else {
                game_state.notifications.success("Game Saved!");
            }
        }

        UiAction::LoadGame => {
            if crate::state::save_system::save_exists("slot_1") {
                match crate::state::save_system::load_game("slot_1") {
                    Ok(loaded_state) => {
                        *game_state = loaded_state;
                        game_state.notifications.success("Game Loaded!");
                        result.deselect_entity = true;
                        result.deselect_room = true;
                        *selected_spell = None;
                    }
                    Err(e) => {
                        eprintln!("Failed to load game: {}", e);
                        game_state.notifications.danger(format!("Load Failed: {}", e));
                    }
                }
            } else {
                game_state.notifications.warning("No save game found!");
            }
        }

        UiAction::CheatAddGold(amount) => {
            game_state.player.gold += amount;
            game_state.notifications.success(format!("Added {} Gold", amount));
        }

        UiAction::CheatToggleFog => {
            game_state.cheat_fog_enabled = !game_state.cheat_fog_enabled;
            if game_state.cheat_fog_enabled {
                game_state.notifications.info("Fog Enabled");
            } else {
                game_state.notifications.warning("Fog Disabled (Map Revealed)");
            }
        }

        UiAction::CheatInstantDig(pos) => {
            if let Some(tile) = game_state.get_tile_mut(pos) {
                if tile.tile_type != "floor" && tile.tile_type != "claimed_floor" {
                    tile.tile_type = "floor".to_string();
                    tile.ownership = Ownership::Player;
                    tile.marked_for_dig = false;
                }
            }
        }

        UiAction::CheatToggleImmortalHeart => {
            game_state.cheat_immortal_heart = !game_state.cheat_immortal_heart;
            if game_state.cheat_immortal_heart {
                game_state.notifications.success("Immortal Heart ENABLED");
            } else {
                game_state.notifications.warning("Immortal Heart DISABLED");
            }
        }
    }

    result
}
