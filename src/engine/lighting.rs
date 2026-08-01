//! Where the dungeon is lit, and in what colour.
//!
//! `rooms.json` authors a `visual.light` block — colour, intensity, flicker —
//! for **22 of 24 rooms**, and `tiles.json` does the same for the mana crystal,
//! lava and the ancient rune floor. All of it was inert: the whole palette was
//! designed and nothing ever read a byte of it.
//!
//! The map is built by splatting outward from each source rather than by asking
//! each tile what lights it. Sources are few (room tiles and three tile types)
//! and radii are small, so this is a few thousand operations; the other
//! direction would be every visible tile times its neighbourhood, every frame.

use std::collections::HashMap;

use crate::data::GameData;
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

/// How dark an unlit tile gets. Not zero: a pitch-black dungeon is unplayable,
/// and the point of lighting here is contrast rather than concealment — fog of
/// war is the system that hides things.
const AMBIENT: f32 = 0.45;

/// Tiles per unit of intensity. An intensity of 0.6 reaches ~4 tiles.
const REACH_PER_INTENSITY: f32 = 7.0;

/// Accumulated light per tile, as a linear RGB multiplier before ambient.
#[derive(Default)]
pub struct LightMap {
    tiles: HashMap<TilePos, [f32; 3]>,
}

impl LightMap {
    /// The multiplier to apply to a tile's base colour, ambient included.
    /// `[1.0; 3]` for a fully lit tile, `[AMBIENT; 3]` for one in the dark.
    pub fn multiplier_at(&self, pos: TilePos) -> [f32; 3] {
        let lit = self.tiles.get(&pos).copied().unwrap_or([0.0; 3]);
        [
            (AMBIENT + lit[0]).min(1.0),
            (AMBIENT + lit[1]).min(1.0),
            (AMBIENT + lit[2]).min(1.0),
        ]
    }

    fn add(&mut self, pos: TilePos, colour: [u8; 3], strength: f32) {
        if strength <= 0.0 {
            return;
        }
        let entry = self.tiles.entry(pos).or_insert([0.0; 3]);
        for channel in 0..3 {
            // Normalised so a saturated source tints rather than just brightens:
            // a red torch should leave blue channels dimmer, not equal.
            let tint = colour[channel] as f32 / 255.0;
            entry[channel] = (entry[channel] + tint * strength).min(1.0);
        }
    }

    /// Splat one source over its neighbourhood, falling off with distance.
    fn splat(&mut self, at: TilePos, colour: [u8; 3], intensity: f32) {
        let reach = (intensity * REACH_PER_INTENSITY).round() as i32;
        if reach <= 0 {
            return;
        }
        for dy in -reach..=reach {
            for dx in -reach..=reach {
                let pos = TilePos::new(at.x + dx, at.y + dy);
                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                if distance > reach as f32 {
                    continue;
                }
                let falloff = 1.0 - (distance / reach as f32);
                self.add(pos, colour, intensity * falloff * falloff);
            }
        }
    }
}

/// Build the light map for the current dungeon.
///
/// Room light wins over tile light where both apply, in the sense that both are
/// simply summed — a lava pool inside a lit room is brighter than either alone,
/// which is what you would expect.
pub fn build_light_map(state: &GameState, game_data: &GameData) -> LightMap {
    let mut map = LightMap::default();

    // Rooms: every tile of an active room emits its room's authored light.
    // Emitting from each tile rather than the centre means a long corridor of a
    // room is lit along its length instead of only in the middle.
    for room in state.room_manager.rooms.iter().filter(|r| r.active) {
        let Some(light) = crate::engine::room_validator::room_data_for(room, game_data)
            .map(|data| &data.visual.light)
        else {
            continue;
        };
        for tile in &room.tiles {
            map.splat(*tile, light.color, light.intensity);
        }
    }

    // Tiles that glow on their own: mana crystal, lava, ancient rune floor.
    for row in &state.dungeon.grid {
        for tile in row {
            let Some(light) = game_data
                .tiles
                .get(&tile.tile_type)
                .and_then(|data| data.visual.light.as_ref())
            else {
                continue;
            };
            map.splat(tile.pos, light.color, light.intensity);
        }
    }

    map
}
