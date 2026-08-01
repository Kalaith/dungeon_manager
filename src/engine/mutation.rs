//! Creatures evolving into other creatures once their conditions are met.
//!
//! `progression.mutations` has been authored since before this file existed —
//! the goblin declares a `hobgoblin` form gated on level and training-hall size
//! — and nothing read it. The condition vocabulary was already implied by that
//! one entry, so this implements it rather than inventing a new one:
//!
//! - `level_at_least`: the creature's own level.
//! - `<room_type>_tiles`: total tiles the keeper owns of that room type, which
//!   is what "evolves based on room exposure" means for the Beastling.
//!
//! **Unknown condition keys fail closed.** A typo makes the mutation never
//! fire rather than fire immediately, which is the safe direction: a mutation
//! that never happens is invisible, but one that fires on turn one replaces a
//! creature the player just paid for.

use crate::data::monsters::MutationData;
use crate::data::GameData;
use crate::state::entities::EntityId;
use crate::state::game_state::GameState;
use crate::state::room_manager::RoomManager;

/// Suffix marking a condition key as a room-tile count.
const TILES_SUFFIX: &str = "_tiles";

/// Evolve any creature whose mutation conditions are now satisfied.
///
/// Returns `(from, to)` for each creature that changed, so callers can tell
/// the player about it.
pub fn apply_mutations(state: &mut GameState, game_data: &GameData) -> Vec<(String, String)> {
    let candidates: Vec<(EntityId, String, u32)> = state
        .entities
        .creatures()
        .map(|(id, creature)| (id, creature.creature_id.clone(), creature.level))
        .collect();

    let mut changed = Vec::new();

    for (id, creature_id, level) in candidates {
        let Some(data) = game_data.monsters.get(&creature_id) else {
            continue;
        };
        if data.progression.mutations.is_empty() {
            continue;
        }

        // First satisfied mutation wins, so authoring order is the priority
        // order — which is what a JSON list already looks like.
        let Some(target) = data
            .progression
            .mutations
            .iter()
            .find(|m| conditions_met(m, level, &state.room_manager))
            .map(|m| m.id.clone())
        else {
            continue;
        };

        let Some(new_data) = game_data.monsters.get(&target) else {
            // Guarded by `every_mutation_target_exists`; if it ever fires here
            // the creature is left alone rather than turned into nothing.
            continue;
        };

        let Some(creature) = state
            .entities
            .get_mut(id)
            .and_then(|entity| entity.as_creature_mut())
        else {
            continue;
        };

        // Carry the wound across rather than the absolute health: a mutation
        // is not a heal, and a creature that evolves mid-fight should not be
        // rewarded for it.
        let health_fraction = (creature.health / creature.max_health).clamp(0.0, 1.0);
        creature.creature_id = target.clone();
        creature.max_health = new_data.stats.health;
        creature.health = new_data.stats.health * health_fraction;
        creature.max_mana = new_data.stats.mana;
        creature.mana = creature.mana.min(new_data.stats.mana);
        creature.movement_speed = new_data.stats.speed;

        trace_log!("creatures", "{} mutated into {}", creature_id, target);
        changed.push((creature_id, target));
    }

    for (from, to) in &changed {
        let from_name = display_name(from, game_data);
        let to_name = display_name(to, game_data);
        state
            .player
            .notify(format!("{from_name} has become a {to_name}"));
    }

    changed
}

fn display_name(creature_id: &str, game_data: &GameData) -> String {
    game_data
        .monsters
        .get(creature_id)
        .map(|d| d.name.clone())
        .unwrap_or_else(|| creature_id.to_string())
}

/// Whether every condition on this mutation holds.
fn conditions_met(mutation: &MutationData, level: u32, rooms: &RoomManager) -> bool {
    mutation.conditions.iter().all(|(key, value)| {
        let Some(required) = value.as_u64() else {
            return false;
        };
        match key.as_str() {
            "level_at_least" => level as u64 >= required,
            key if key.ends_with(TILES_SUFFIX) => {
                let room_type = &key[..key.len() - TILES_SUFFIX.len()];
                room_tiles(rooms, room_type) >= required
            }
            // Unknown key: fail closed. See the module docs.
            _ => false,
        }
    })
}

/// Total tiles the keeper owns across every active room of this type.
fn room_tiles(rooms: &RoomManager, room_type: &str) -> u64 {
    rooms
        .rooms
        .iter()
        .filter(|room| room.active && room.room_type == room_type)
        .map(|room| room.tiles.len() as u64)
        .sum()
}
