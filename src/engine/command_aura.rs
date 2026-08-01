//! Overseers raising the work rate of the creatures around them.
//!
//! This runs as its own pass and caches its result on each creature rather
//! than being computed where it is used. The reason is a borrow, not taste:
//! `calculate_work_efficiency` is called from `task_system` while the working
//! creature is already mutably borrowed out of `EntityManager`, so it cannot
//! scan for neighbours at that point.
//!
//! Recomputed in full every tick, so nothing goes stale when an overseer dies
//! or walks away — `command_bonus` is a cache, not authored state.

use crate::data::GameData;
use crate::state::entities::EntityId;
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

/// Refresh every creature's `command_bonus` from the overseers near it.
pub fn apply_command_auras(state: &mut GameState, game_data: &GameData) {
    // (position, bonus) for every creature currently projecting an aura.
    let commanders: Vec<(TilePos, f32, f32)> = state
        .entities
        .creatures()
        .filter_map(|(id, creature)| {
            let (bonus, radius) = command_aura(&creature.creature_id, game_data)?;
            let pos = state.entities.get(id)?.pos;
            Some((pos, bonus, radius))
        })
        .collect();

    let targets: Vec<(EntityId, TilePos)> = state
        .entities
        .creatures()
        .filter_map(|(id, _)| state.entities.get(id).map(|e| (id, e.pos)))
        .collect();

    for (id, pos) in targets {
        // Best commander wins rather than the bonuses multiplying, so stacking
        // overseers in one room is not an exploit. An overseer does not
        // command itself: the aura is for the workers it is standing over.
        let bonus = commanders
            .iter()
            .filter(|(commander_pos, _, radius)| {
                *commander_pos != pos && pos.distance_to(commander_pos) <= *radius
            })
            .map(|(_, bonus, _)| *bonus)
            .fold(1.0f32, f32::max);

        if let Some(creature) = state
            .entities
            .get_mut(id)
            .and_then(|entity| entity.as_creature_mut())
        {
            creature.command_bonus = bonus;
        }
    }
}

/// The aura this creature projects, if any, as `(multiplier, radius)`.
fn command_aura(creature_id: &str, game_data: &GameData) -> Option<(f32, f32)> {
    let data = game_data.monsters.get(creature_id)?;
    data.traits
        .iter()
        .filter_map(|tag| game_data.traits.get(tag))
        .filter(|t| t.command_efficiency_bonus > 1.0 && t.command_radius > 0.0)
        .map(|t| (t.command_efficiency_bonus, t.command_radius))
        .fold(None, |best: Option<(f32, f32)>, candidate| match best {
            Some(b) if b.0 >= candidate.0 => Some(b),
            _ => Some(candidate),
        })
}
