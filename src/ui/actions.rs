//! UI Action system
//! Decouples UI events from game state mutations

use crate::state::entities::EntityId;
use crate::state::tile_state::TilePos;

/// Represents a player action from the UI
#[derive(Debug, Clone)]
pub enum UiAction {
    // Mode switching
    ChangeMode(crate::InteractionMode),
    
    // Tile actions
    MarkTileForDig(TilePos),
    UnmarkTileForDig(TilePos),
    BuildRoomTile { room_type: String, pos: TilePos },
    PlaceTrap { trap_type: String, pos: TilePos },
    PlaceSpawner(TilePos),
    SellTile(TilePos),
    
    // Entity actions
    PickupEntity(EntityId),
    DropEntity { entity_id: EntityId, pos: TilePos },
    SelectEntity(EntityId),
    DeselectEntity,
    SlapCreature(EntityId),
    
    // Room actions
    SelectRoom(usize),
    DeselectRoom,
    
    // Spell actions
    SelectSpell(String),
    ClearSpellSelection,
    CastSpell { spell_id: String, target: SpellTarget },
    
    // UI actions
    StartResearch(String),
    SaveGame,
    Cancel,
    TogglePause,
}

/// Target for spell casting
#[derive(Debug, Clone)]
pub enum SpellTarget {
    Tile(TilePos),
    Entity(EntityId),
    None,
}

/// Queue for collecting UI actions during a frame
#[derive(Default)]
pub struct ActionQueue {
    actions: Vec<UiAction>,
}

impl ActionQueue {
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }

    /// Add an action to the queue
    pub fn push(&mut self, action: UiAction) {
        self.actions.push(action);
    }

    /// Take all actions from the queue
    pub fn drain(&mut self) -> Vec<UiAction> {
        std::mem::take(&mut self.actions)
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Get number of pending actions
    pub fn len(&self) -> usize {
        self.actions.len()
    }
}
