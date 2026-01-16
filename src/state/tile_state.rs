//! Runtime tile state
//! This module defines the runtime state of tiles in the dungeon grid,
//! separate from the static TileData definitions.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

impl TilePos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &TilePos) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn manhattan_distance(&self, other: &TilePos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ownership {
    Unclaimed,
    Player,
    Enemy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FogState {
    Hidden,
    Revealed,
    Visible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrapState {
    pub trap_type: String,
    pub constructed: bool,
    pub construction_progress: f32,
    pub active: bool,
    pub funded: bool,
    /// Cooldown timer - trap cannot trigger while > 0
    #[serde(default)]
    pub cooldown: f32,
    /// Whether the trap has been triggered (for single-use traps)
    #[serde(default)]
    pub triggered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileState {
    pub tile_type: String,
    pub pos: TilePos,
    pub ownership: Ownership,
    pub room_id: Option<usize>,
    pub fog_state: FogState,
    pub resources_remaining: Option<u32>,
    pub marked_for_dig: bool,
    pub claimed_by_entity: Option<usize>, // For lair tiles - which entity owns this space
    pub trap: Option<TrapState>,
}

impl TileState {
    pub fn new(tile_type: String, pos: TilePos) -> Self {
        Self {
            tile_type,
            pos,
            ownership: Ownership::Unclaimed,
            room_id: None,
            fog_state: FogState::Revealed, // Start revealed so players can see the map
            resources_remaining: None,
            marked_for_dig: false,
            claimed_by_entity: None,
            trap: None,
        }
    }

    pub fn with_resources(mut self, amount: u32) -> Self {
        self.resources_remaining = Some(amount);
        self
    }

    pub fn is_walkable(&self, tile_data: &crate::data::TileData) -> bool {
        !tile_data.blocks_movement
    }

    pub fn is_diggable(&self, tile_data: &crate::data::TileData) -> bool {
        tile_data.diggable && !self.marked_for_dig
    }

    pub fn is_claimable(&self, tile_data: &crate::data::TileData) -> bool {
        tile_data.claimable && self.ownership == Ownership::Unclaimed
    }

    pub fn claim(&mut self) {
        self.ownership = Ownership::Player;
        self.fog_state = FogState::Visible;
    }

    pub fn mark_for_dig(&mut self) {
        self.marked_for_dig = true;
    }

    pub fn unmark_for_dig(&mut self) {
        self.marked_for_dig = false;
    }

    pub fn set_room(&mut self, room_id: Option<usize>) {
        self.room_id = room_id;
    }

    pub fn reveal(&mut self) {
        if self.fog_state == FogState::Hidden {
            self.fog_state = FogState::Revealed;
        }
    }

    pub fn make_visible(&mut self) {
        self.fog_state = FogState::Visible;
    }
}
