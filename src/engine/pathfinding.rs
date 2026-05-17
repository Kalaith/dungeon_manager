//! Compatibility re-exports for pathfinding.
//!
//! The implementation lives in `macroquad-toolkit`; this module keeps the
//! existing `crate::engine::pathfinding::*` path stable for dungeon code.

#[allow(unused_imports)]
pub use macroquad_toolkit::pathfinding::{
    find_path, find_path_with, CacheStats, Heuristic, Path, PathCache, PathfindingGrid, Pos,
};
