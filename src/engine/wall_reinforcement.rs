//! Stone Wardens turning plain rock into wall a hero cannot dig through.
//!
//! This is its own per-tick pass rather than part of `creature_ai` for two
//! reasons: `update_creatures` takes the dungeon immutably, and reinforcement
//! writes to it; and `creature_ai.rs` is already at the file-size limit.
//!
//! The mechanic is real rather than cosmetic because `diggable` is enforced —
//! `hero_digging::process_hero_digging` refuses an undiggable tile and
//! `build_hero_pathfinding_grid` will not route a digging hero through one. A
//! reinforced wall genuinely closes a tunnel route into the dungeon.

use crate::data::GameData;
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

/// The tile a reinforced wall becomes. Nothing else in `tiles.json` is
/// authored `diggable: false` among the walls, so this is the only target.
const REINFORCED: &str = "reinforced_wall";

/// Advance every stonebinding creature's timer and convert walls that are due.
///
/// Returns how many tiles were reinforced this tick, for tracing.
pub fn reinforce_walls(state: &mut GameState, game_data: &GameData, dt: f32) -> usize {
    let workers: Vec<(crate::state::entities::EntityId, f32)> = state
        .entities
        .creatures()
        .filter_map(|(id, creature)| {
            let seconds = reinforce_interval(&creature.creature_id, game_data)?;
            Some((id, seconds))
        })
        .collect();

    let mut reinforced = 0;

    for (id, seconds) in workers {
        let Some(pos) = state.entities.get(id).map(|e| e.pos) else {
            continue;
        };

        let due = {
            let Some(creature) = state
                .entities
                .get_mut(id)
                .and_then(|entity| entity.as_creature_mut())
            else {
                continue;
            };
            creature.reinforce_timer += dt;
            if creature.reinforce_timer < seconds {
                continue;
            }
            // Only spend the timer when there is actually something to shore
            // up, so a warden standing in open floor does not bank progress it
            // then dumps the instant it walks past a wall.
            true
        };

        if !due {
            continue;
        }

        match next_wall_to_reinforce(state, game_data, pos) {
            Some(target) => {
                if let Some(tile) = state.dungeon.get_tile_mut(target) {
                    tile.tile_type = REINFORCED.to_string();
                }
                if let Some(creature) = state
                    .entities
                    .get_mut(id)
                    .and_then(|entity| entity.as_creature_mut())
                {
                    creature.reinforce_timer = 0.0;
                }
                reinforced += 1;
                trace_log!("creatures", "Wall reinforced at {:?}", target);
            }
            None => {
                // Hold the timer at the threshold rather than resetting it:
                // the warden is ready, just not next to anything worth doing.
                if let Some(creature) = state
                    .entities
                    .get_mut(id)
                    .and_then(|entity| entity.as_creature_mut())
                {
                    creature.reinforce_timer = seconds;
                }
            }
        }
    }

    reinforced
}

/// Seconds between reinforcements for this creature, or `None` if it does not
/// do this at all. Derived from traits, so any creature can be authored into
/// the job without an engine change.
fn reinforce_interval(creature_id: &str, game_data: &GameData) -> Option<f32> {
    let data = game_data.monsters.get(creature_id)?;
    data.traits
        .iter()
        .filter_map(|tag| game_data.traits.get(tag))
        .map(|t| t.wall_reinforce_seconds)
        .filter(|seconds| *seconds > 0.0)
        // Fastest trait wins if a creature somehow carries two.
        .fold(None, |best: Option<f32>, s| {
            Some(best.map_or(s, |b| b.min(s)))
        })
}

/// An orthogonally adjacent tile worth reinforcing, if any.
///
/// "Worth reinforcing" is expressed entirely in tile data rather than by naming
/// tile types: it must block movement (so it is a wall, not floor), be diggable
/// (so it is not already reinforced), and carry no `resources` — which is what
/// keeps a warden from sealing away a gold vein or gem seam the imps still want.
fn next_wall_to_reinforce(
    state: &GameState,
    game_data: &GameData,
    from: TilePos,
) -> Option<TilePos> {
    const NEIGHBOURS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    NEIGHBOURS.iter().find_map(|(dx, dy)| {
        let pos = TilePos::new(from.x + dx, from.y + dy);
        let tile = state.dungeon.get_tile(pos)?;
        let data = game_data.tiles.get(&tile.tile_type)?;

        let is_plain_wall = data.blocks_movement
            && data.diggable
            && data.resources.is_none()
            && tile.trap.is_none();

        is_plain_wall.then_some(pos)
    })
}
