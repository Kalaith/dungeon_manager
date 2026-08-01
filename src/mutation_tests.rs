//! Tests for creature mutation.
//!
//! `progression.mutations` was authored long before anything read it, and the
//! one entry in the data pointed at a creature that did not exist. These cover
//! the engine that now reads it, and specifically the fail-closed behaviour:
//! an unrecognised condition must prevent the mutation, not wave it through.

use std::collections::HashSet;

use crate::data::GameData;
use crate::engine::mutation::apply_mutations;
use crate::engine::room_validator::Room;
use crate::state::entities::{CreatureState, EntityId};
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

const SPAWN: TilePos = TilePos { x: 4, y: 4 };

/// A state holding one creature at `level`, plus `tiles` tiles of `room_type`.
/// Returns the state plus the id of the creature under test — `GameState::new`
/// seeds its own starting creatures, so "the only creature" is not a safe
/// assumption in a fixture.
fn state_with(
    game_data: &GameData,
    creature_id: &str,
    level: u32,
    room_type: &str,
    tiles: usize,
) -> (GameState, EntityId) {
    let mut state = GameState::new(24, 24, game_data);
    let data = &game_data.monsters[creature_id];

    let mut creature = CreatureState::new(
        creature_id.to_string(),
        level,
        data.stats.health,
        data.stats.mana,
        1,
    );
    creature.level = level;
    let id = state.entities.spawn_creature(SPAWN, creature);

    if tiles > 0 {
        let positions: HashSet<TilePos> = (0..tiles)
            .map(|i| TilePos::new(10 + i as i32, 10))
            .collect();
        let mut room = Room::new(1, room_type.to_string(), positions, Vec::new());
        // `Room::new` leaves rooms inactive, and `room_tiles` only counts
        // active ones — a half-built room should not push a creature over an
        // evolution threshold. The fixture has to say so explicitly.
        room.active = true;
        state.room_manager.rooms.push(room);
    }
    (state, id)
}

/// What the creature under test currently is.
fn kind_of(state: &GameState, id: EntityId) -> String {
    state
        .entities
        .get(id)
        .and_then(|e| e.as_creature())
        .map(|c| c.creature_id.clone())
        .expect("the creature under test should still exist")
}

#[test]
fn a_drilled_goblin_becomes_a_hobgoblin() {
    // The mutation authored in the shipped data: level 5 plus 20 training-hall
    // tiles. Nothing read it until now, and its target did not exist.
    let game_data = GameData::load().expect("game data should load");
    let (mut state, id) = state_with(&game_data, "goblin", 5, "training_hall", 20);

    let changed = apply_mutations(&mut state, &game_data);

    assert!(changed.contains(&("goblin".to_string(), "hobgoblin".to_string())));
    assert_eq!(kind_of(&state, id), "hobgoblin");
}

#[test]
fn an_undertrained_goblin_stays_a_goblin() {
    let game_data = GameData::load().expect("game data should load");

    // Right level, not enough training hall.
    let (mut small_room, small_id) = state_with(&game_data, "goblin", 5, "training_hall", 4);
    apply_mutations(&mut small_room, &game_data);
    assert_eq!(kind_of(&small_room, small_id), "goblin");

    // Enough training hall, too low a level.
    let (mut low_level, low_id) = state_with(&game_data, "goblin", 1, "training_hall", 20);
    apply_mutations(&mut low_level, &game_data);
    assert_eq!(kind_of(&low_level, low_id), "goblin");
}

#[test]
fn a_beastling_raised_in_a_kennel_becomes_a_hellhound() {
    let game_data = GameData::load().expect("game data should load");
    let (mut state, id) = state_with(&game_data, "beastling", 3, "kennel", 9);

    apply_mutations(&mut state, &game_data);

    assert_eq!(kind_of(&state, id), "hellhound");
}

/// The branch that makes "evolves based on room exposure" mean something: the
/// same creature at the same level becomes something else entirely.
#[test]
fn a_beastling_raised_in_a_fighting_pit_becomes_an_ogre() {
    let game_data = GameData::load().expect("game data should load");
    let (mut state, id) = state_with(&game_data, "beastling", 3, "combat_pit", 9);

    apply_mutations(&mut state, &game_data);

    assert_eq!(kind_of(&state, id), "ogre");
}

#[test]
fn mutation_carries_the_wound_across_rather_than_healing() {
    let game_data = GameData::load().expect("game data should load");
    let (mut state, id) = state_with(&game_data, "beastling", 3, "kennel", 9);

    if let Some(c) = state.entities.get_mut(id).and_then(|e| e.as_creature_mut()) {
        c.health = c.max_health * 0.25;
    }

    apply_mutations(&mut state, &game_data);

    let creature = state
        .entities
        .get(id)
        .and_then(|e| e.as_creature())
        .expect("creature");
    let fraction = creature.health / creature.max_health;
    assert!(
        (fraction - 0.25).abs() < 0.01,
        "a quarter-health beastling should be a quarter-health hellhound, was {fraction}"
    );
}

/// Fail-closed is the whole safety property: a condition the engine does not
/// understand must block the mutation. The opposite default would fire every
/// unrecognised mutation on the first tick.
#[test]
fn an_unrecognised_condition_blocks_the_mutation() {
    let mut game_data = GameData::load().expect("game data should load");

    let beastling = game_data
        .monsters
        .get_mut("beastling")
        .expect("beastling should exist");
    for mutation in &mut beastling.progression.mutations {
        mutation.conditions.clear();
        mutation
            .conditions
            .insert("phase_of_the_moon".to_string(), serde_json::json!(1));
    }

    let (mut state, id) = state_with(&game_data, "beastling", 5, "kennel", 20);
    apply_mutations(&mut state, &game_data);
    assert_eq!(
        kind_of(&state, id),
        "beastling",
        "an unknown condition key must not be treated as satisfied"
    );
}

/// The two-step chain: an amalgam left beside a ritual circle corrupts. This
/// is the only place a Void-Touched can come from — it has no portal roll and
/// no tech unlock, so if this stops working the creature becomes unreachable.
#[test]
fn an_amalgam_beside_a_ritual_circle_becomes_void_touched() {
    let game_data = GameData::load().expect("game data should load");
    let (mut state, id) = state_with(&game_data, "flesh_amalgam", 4, "ritual_circle", 12);

    apply_mutations(&mut state, &game_data);

    assert_eq!(kind_of(&state, id), "void_touched");
}

#[test]
fn an_amalgam_without_a_ritual_circle_stays_itself() {
    let game_data = GameData::load().expect("game data should load");
    let (mut state, id) = state_with(&game_data, "flesh_amalgam", 5, "graveyard", 20);

    apply_mutations(&mut state, &game_data);

    assert_eq!(kind_of(&state, id), "flesh_amalgam");
}
