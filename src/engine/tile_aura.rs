//! Tiles that affect whoever stands near them.
//!
//! `tiles.json` has declared an aura on `bone_floor` — `hero_fear: 0.1` over a
//! radius of 2 — since long before anything could act on it. There was no fear
//! mechanic to hang it on until the Ogre's `terrify` gave `fear` a meaning, so
//! the data sat inert and the field guard listed it as dead.
//!
//! Read at the point of use rather than applied as a status, the same shape as
//! `needs::room_happiness_at`. That avoids the obvious trap: an aura applied as
//! a timed status every tick would stack without limit, and a hero who stood on
//! a bone floor for ten seconds would be permanently terrified.

use crate::data::GameData;
use crate::state::dungeon::Dungeon;
use crate::state::tile_state::TilePos;

/// Aura effect key for "frightens heroes standing near this".
const HERO_FEAR: &str = "hero_fear";

/// Total `hero_fear` from every aura tile covering `pos`.
///
/// Scans outward from the hero rather than over the map: auras have small
/// radii, so this is a handful of tile lookups per hero per tick, where
/// sweeping every tile for auras would be O(map).
pub fn hero_fear_at(pos: TilePos, dungeon: &Dungeon, game_data: &GameData) -> f32 {
    const MAX_AURA_RADIUS: i32 = 4;

    let mut total = 0.0;
    for dy in -MAX_AURA_RADIUS..=MAX_AURA_RADIUS {
        for dx in -MAX_AURA_RADIUS..=MAX_AURA_RADIUS {
            let at = TilePos::new(pos.x + dx, pos.y + dy);
            let Some(tile) = dungeon.get_tile(at) else {
                continue;
            };
            let Some(aura) = game_data
                .tiles
                .get(&tile.tile_type)
                .and_then(|data| data.special.as_ref())
                .and_then(|special| special.aura.as_ref())
            else {
                continue;
            };

            if pos.distance_to(&at) > aura.radius as f32 {
                continue;
            }
            // Unknown effect keys are ignored rather than guessed at; the
            // content-wiring test is what stops them being authored at all.
            total += aura.effects.get(HERO_FEAR).copied().unwrap_or(0.0);
        }
    }
    total
}
