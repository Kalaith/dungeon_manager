//! Tile type constants and helper functions
//! Centralizes all tile type string literals to avoid magic strings

/// Tile type string constants
pub mod types {
    // Terrain tiles
    pub const SOLID_ROCK: &str = "solid_rock";
    pub const EARTH: &str = "earth";
    pub const BEDROCK: &str = "bedrock";
    
    // Resource tiles
    pub const GOLD_VEIN: &str = "gold_vein";
    pub const GEM_SEAM: &str = "gem_seam";
    pub const MANA_CRYSTAL: &str = "mana_crystal";
    
    // Floor tiles
    pub const CLAIMED_FLOOR: &str = "claimed_floor";
    pub const REINFORCED_WALL: &str = "reinforced_wall";
    
    // Special tiles
    pub const DUNGEON_HEART: &str = "dungeon_heart";
    pub const MONSTER_SPAWNER: &str = "monster_spawner";
    pub const HERO_ENTRANCE: &str = "hero_entrance";
    
    // Room tiles
    pub const LAIR: &str = "lair";
    pub const HATCHERY: &str = "hatchery";
    pub const TREASURY: &str = "treasury";
    pub const TRAINING_ROOM: &str = "training_room";
    pub const LIBRARY: &str = "library";
    pub const WORKSHOP: &str = "workshop";
}

/// Check if a tile type is a wall/blocking tile
pub fn is_wall(tile_type: &str) -> bool {
    matches!(
        tile_type,
        types::EARTH | types::GOLD_VEIN | types::GEM_SEAM | types::MANA_CRYSTAL | types::BEDROCK | types::SOLID_ROCK
    )
}

/// Check if a tile type is diggable
pub fn is_diggable(tile_type: &str) -> bool {
    matches!(
        tile_type,
        types::EARTH | types::GOLD_VEIN | types::GEM_SEAM | types::MANA_CRYSTAL
    )
}

/// Check if a tile type is a resource tile
pub fn is_resource(tile_type: &str) -> bool {
    matches!(
        tile_type,
        types::GOLD_VEIN | types::GEM_SEAM | types::MANA_CRYSTAL
    )
}

/// Check if a tile type is walkable for creatures
pub fn is_walkable(tile_type: &str) -> bool {
    matches!(
        tile_type,
        types::CLAIMED_FLOOR
            | types::LAIR
            | types::HATCHERY
            | types::TREASURY
            | types::TRAINING_ROOM
            | types::LIBRARY
            | types::WORKSHOP
            | types::DUNGEON_HEART
            | types::MONSTER_SPAWNER
    )
}

/// Check if a tile type is a room tile
pub fn is_room(tile_type: &str) -> bool {
    matches!(
        tile_type,
        types::LAIR
            | types::HATCHERY
            | types::TREASURY
            | types::TRAINING_ROOM
            | types::LIBRARY
            | types::WORKSHOP
    )
}

/// Check if a tile type can have a room built on it
pub fn can_build_room(tile_type: &str) -> bool {
    tile_type == types::CLAIMED_FLOOR
}

/// Check if a tile type is secure (blocks vision/movement)
pub fn is_secure(tile_type: &str) -> bool {
    matches!(
        tile_type,
        types::SOLID_ROCK | types::EARTH | types::REINFORCED_WALL | types::BEDROCK
    )
}

/// Check if a tile type blocks line of sight
pub fn blocks_vision(tile_type: &str) -> bool {
    matches!(
        tile_type,
        types::EARTH | types::SOLID_ROCK | types::GOLD_VEIN | types::GEM_SEAM | types::BEDROCK
    )
}
