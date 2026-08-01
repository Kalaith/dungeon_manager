//! Traits stitched onto an individual creature rather than shared by its kind.
//!
//! The Flesh Amalgam's "random traits, unstable behaviour": every other
//! creature in the game draws its traits from `monsters.json`, so two goblins
//! behave identically. A grafted creature rolls its own on first sight and
//! keeps them, which makes it the only creature whose behaviour is not
//! predictable from its name.
//!
//! Rolled once and stored on the creature, so the result survives save/load
//! and the creature does not quietly re-roll itself into something else.

use crate::data::GameData;
use crate::state::entities::EntityId;
use crate::state::game_state::GameState;

/// Give every ungrafted creature that should have random traits its traits.
///
/// Returns `(creature_id, grafted)` for each one rolled, for tracing.
pub fn graft_random_traits(
    state: &mut GameState,
    game_data: &GameData,
) -> Vec<(String, Vec<String>)> {
    let pending: Vec<(EntityId, String, u32)> = state
        .entities
        .creatures()
        .filter(|(_, creature)| creature.extra_traits.is_empty())
        .filter_map(|(id, creature)| {
            let count = graft_count(&creature.creature_id, game_data)?;
            Some((id, creature.creature_id.clone(), count))
        })
        .collect();

    if pending.is_empty() {
        return Vec::new();
    }

    let pool: Vec<&String> = {
        let mut names: Vec<&String> = game_data
            .traits
            .iter()
            .filter(|(_, data)| data.graftable)
            .map(|(id, _)| id)
            .collect();
        // `traits` is a HashMap, so iteration order is not stable between runs.
        // Sorting first means the roll depends only on the RNG.
        names.sort();
        names
    };

    let mut rolled = Vec::new();

    for (id, creature_id, count) in pending {
        let picked = pick_distinct(&pool, count as usize);
        if picked.is_empty() {
            // No graftable traits authored: leave the creature alone rather
            // than marking it done, so authoring some later still works.
            continue;
        }

        if let Some(creature) = state
            .entities
            .get_mut(id)
            .and_then(|entity| entity.as_creature_mut())
        {
            creature.extra_traits = picked.clone();
            trace_log!("creatures", "{} grafted with {:?}", creature_id, picked);
            rolled.push((creature_id, picked));
        }
    }

    rolled
}

/// How many traits this creature's kind is grafted with, if any.
fn graft_count(creature_id: &str, game_data: &GameData) -> Option<u32> {
    let data = game_data.monsters.get(creature_id)?;
    data.traits
        .iter()
        .filter_map(|tag| game_data.traits.get(tag))
        .map(|t| t.graft_count)
        .max()
        .filter(|count| *count > 0)
}

/// `count` distinct entries from `pool`, or all of it if it is smaller.
fn pick_distinct(pool: &[&String], count: usize) -> Vec<String> {
    let mut remaining: Vec<&String> = pool.to_vec();
    let mut picked = Vec::new();
    for _ in 0..count.min(remaining.len()) {
        let idx = macroquad_toolkit::rng::gen_range(0, remaining.len());
        picked.push(remaining.remove(idx).clone());
    }
    picked.sort();
    picked
}
