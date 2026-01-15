//! Action Processor
//! Processes UI actions and applies them to game state

use crate::data::GameData;
use crate::state::game_state::GameState;
use crate::state::tile_state::{Ownership, TilePos};
use crate::state::entities::EntityId;
use crate::ui::actions::{ActionQueue, SpellTarget, UiAction};
use crate::engine::tile_types::{self, types as tt};
use crate::engine::spell_effects;
use crate::InteractionMode;

/// Result of processing an action
#[derive(Debug, Default)]
pub struct ActionResult {
    /// New interaction mode if action changed it
    pub new_mode: Option<InteractionMode>,
    /// Entity to select
    pub select_entity: Option<EntityId>,
    /// Room to select
    pub select_room: Option<usize>,
    /// Entity currently held
    pub held_entity: Option<EntityId>,
    /// Clear entity selection
    pub deselect_entity: bool,
    /// Clear room selection
    pub deselect_room: bool,
    /// Show message to player
    pub message: Option<String>,
}

/// Process all queued actions
pub fn process_actions(
    actions: &mut ActionQueue,
    game_state: &mut GameState,
    game_data: &GameData,
    interaction_mode: &mut InteractionMode,
    held_entity: &mut Option<EntityId>,
    selected_entity: &mut Option<EntityId>,
    selected_room: &mut Option<usize>,
    selected_spell: &mut Option<String>,
) {
    for action in actions.drain() {
        let result = process_single_action(
            action,
            game_state,
            game_data,
            interaction_mode,
            *held_entity,
            selected_spell,
        );

        // Apply results
        if let Some(mode) = result.new_mode {
            *interaction_mode = mode;
        }
        if let Some(entity_id) = result.select_entity {
            *selected_entity = Some(entity_id);
        }
        if result.deselect_entity {
            *selected_entity = None;
        }
        if let Some(room_id) = result.select_room {
            *selected_room = Some(room_id);
        }
        if result.deselect_room {
            *selected_room = None;
        }
        if let Some(entity_id) = result.held_entity {
            *held_entity = Some(entity_id);
        }
    }
}

/// Process a single action
fn process_single_action(
    action: UiAction,
    game_state: &mut GameState,
    game_data: &GameData,
    _current_mode: &InteractionMode,
    held_entity: Option<EntityId>,
    selected_spell: &mut Option<String>,
) -> ActionResult {
    let mut result = ActionResult::default();

    match action {
        UiAction::ChangeMode(mode) => {
            result.new_mode = Some(mode);
        }

        UiAction::MarkTileForDig(pos) => {
            if let Some(tile) = game_state.get_tile_mut(pos) {
                if tile_types::is_diggable(&tile.tile_type, game_data) && tile.ownership == Ownership::Unclaimed {
                    tile.marked_for_dig = true;
                }
            }
        }

        UiAction::UnmarkTileForDig(pos) => {
            if let Some(tile) = game_state.get_tile_mut(pos) {
                tile.marked_for_dig = false;
            }
        }

        UiAction::BuildRoomTile { room_type, pos } => {
            process_build_room(game_state, game_data, &room_type, pos);
        }

        UiAction::PlaceTrap { trap_type, pos } => {
            process_place_trap(game_state, game_data, &trap_type, pos);
        }

        UiAction::PlaceSpawner(pos) => {
            process_place_spawner(game_state, game_data, pos);
        }

        UiAction::SellTile(pos) => {
            process_sell_tile(game_state, pos);
        }

        UiAction::PickupEntity(entity_id) => {
            // Mark entity as held
            result.held_entity = Some(entity_id);
            result.select_entity = Some(entity_id);
        }

        UiAction::DropEntity { entity_id, pos } => {
            process_drop_entity(game_state, game_data, entity_id, pos);
            result.held_entity = None;
        }

        UiAction::SelectEntity(entity_id) => {
            result.select_entity = Some(entity_id);
            result.deselect_room = true;
        }

        UiAction::DeselectEntity => {
            result.deselect_entity = true;
        }

        UiAction::SlapCreature(entity_id) => {
            process_slap_creature(game_state, game_data, entity_id);
        }

        UiAction::SelectRoom(room_id) => {
            result.select_room = Some(room_id);
            result.deselect_entity = true;
        }

        UiAction::DeselectRoom => {
            result.deselect_room = true;
        }

        UiAction::SelectSpell(spell_id) => {
            *selected_spell = Some(spell_id);
        }

        UiAction::ClearSpellSelection => {
            *selected_spell = None;
        }

        UiAction::CastSpell { spell_id, target } => {
            process_cast_spell(game_state, game_data, &spell_id, target);
            *selected_spell = None;
        }

        UiAction::Cancel => {
            result.new_mode = Some(InteractionMode::None);
            *selected_spell = None;
            result.deselect_entity = true;
            result.deselect_room = true;
        }

        UiAction::TogglePause => {
            // TODO: Implement pause
        }

        UiAction::StartResearch(tech_id) => {
            game_state.player.start_research(tech_id);
        }
    }

    result
}

fn process_build_room(
    game_state: &mut GameState,
    game_data: &GameData,
    room_type: &str,
    pos: TilePos,
) {
    // Get room cost from game data
    let cost = game_data.rooms.get(room_type)
        .map(|r| r.build.cost_per_tile as i32)
        .unwrap_or_else(|| panic!("Room type '{}' missing in rooms.json", room_type));

    let is_valid_tile = if let Some(tile) = game_state.get_tile(pos) {
        tile.ownership == Ownership::Player 
            && tile.room_id.is_none() 
            && tile_types::can_build_room(&tile.tile_type, game_data)
    } else {
        false
    };

    if game_state.player.gold >= cost && is_valid_tile {
        game_state.player.gold -= cost;
        
        if let Some(tile) = game_state.get_tile_mut(pos) {
            tile.tile_type = room_type.to_string();
            // Room ID will be assigned by room_manager on next scan
        }
    }
}

fn process_place_trap(
    game_state: &mut GameState,
    game_data: &GameData,
    trap_type: &str,
    pos: TilePos,
) {
    let cost = game_data.traps.get(trap_type)
        .map(|t| t.cost)
        .unwrap_or(10);

    let is_valid = if let Some(tile) = game_state.get_tile(pos) {
        tile.ownership == Ownership::Player
            && tile_types::can_build_room(&tile.tile_type, game_data)
            && tile.trap.is_none()
    } else {
        false
    };

    if game_state.player.materials >= cost && is_valid {
        game_state.player.materials -= cost;
        
        if let Some(tile) = game_state.get_tile_mut(pos) {
            tile.trap = Some(crate::state::tile_state::TrapState {
                trap_type: trap_type.to_string(),
                constructed: false,
                construction_progress: 0.0,
                active: false,
                funded: true,
            });
        }
    }
}

fn process_place_spawner(game_state: &mut GameState, game_data: &GameData, pos: TilePos) {
    let is_valid = if let Some(tile) = game_state.get_tile(pos) {
        tile.ownership == Ownership::Player && tile_types::can_build_room(&tile.tile_type, game_data)
    } else {
        false
    };

    if game_state.player.gold >= crate::config::SPAWNER_COST && is_valid {
        game_state.player.gold -= crate::config::SPAWNER_COST;
        
        if let Some(tile) = game_state.get_tile_mut(pos) {
            tile.tile_type = tt::MONSTER_SPAWNER.to_string();
        }
    }
}

fn process_sell_tile(game_state: &mut GameState, pos: TilePos) {
    // Check if tile is owned and calculate refund first
    let refund = if let Some(tile) = game_state.get_tile(pos) {
        if tile.ownership == Ownership::Player {
            if tile.room_id.is_some() { 5 } else { 0 }
        } else {
            return; // Not player-owned, exit early
        }
    } else {
        return;
    };

    // Apply refund
    game_state.player.gold += refund;
    
    // Now mutate the tile
    if let Some(tile) = game_state.get_tile_mut(pos) {
        tile.tile_type = tt::CLAIMED_FLOOR.to_string();
        tile.room_id = None;
        tile.trap = None;
    }
}

fn process_drop_entity(game_state: &mut GameState, game_data: &GameData, entity_id: EntityId, pos: TilePos) {
    // Check if target tile is walkable
    let is_valid = if let Some(tile) = game_state.get_tile(pos) {
        tile_types::is_walkable(&tile.tile_type, game_data)
    } else {
        false
    };

    if is_valid {
        if let Some(entity) = game_state.entities.get_mut(entity_id) {
            entity.pos = pos;
        }
    }
}

fn process_slap_creature(
    game_state: &mut GameState,
    game_data: &GameData,
    entity_id: EntityId,
) {
    use crate::engine::creature_ai;

    if let Some(entity) = game_state.entities.get_mut(entity_id) {
        if let Some(creature) = entity.as_creature_mut() {
            if let Some(monster_data) = game_data.monsters.get(&creature.creature_id) {
                creature_ai::apply_slap(creature, monster_data, game_state.time_elapsed);
            }
        }
    }
}

fn process_cast_spell(
    game_state: &mut GameState,
    game_data: &GameData,
    spell_id: &str,
    target: SpellTarget,
) {
    // Convert SpellTarget to the existing API format
    let (target_pos, target_entity) = match target {
        SpellTarget::Tile(pos) => (Some(pos), None),
        SpellTarget::Entity(entity_id) => (None, Some(entity_id)),
        SpellTarget::None => (None, None),
    };

    // Use the existing cast_spell function which handles all validation and effects
    spell_effects::cast_spell(
        spell_id,
        game_state,
        game_data,
        target_pos,
        target_entity,
    );
}
