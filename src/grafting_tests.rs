//! Tests for per-instance grafted traits.
//!
//! Every other creature draws its traits from `monsters.json`, so two goblins
//! are interchangeable. These cover the one creature that is not, and the two
//! places grafted traits have to reach: mood/needs, and combat stats — which
//! were separate lookups until the resolver was unified.

use crate::data::GameData;
use crate::engine::combat;
use crate::engine::creature_ai::needs::creature_traits;
use crate::engine::grafting::graft_random_traits;
use crate::state::entities::{CreatureState, EntityId};
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

const SPAWN: TilePos = TilePos { x: 6, y: 6 };

fn state_with(game_data: &GameData, creature_id: &str) -> (GameState, EntityId) {
    let mut state = GameState::new(24, 24, game_data);
    let data = &game_data.monsters[creature_id];
    let creature = CreatureState::new(
        creature_id.to_string(),
        1,
        data.stats.health,
        data.stats.mana,
        1,
    );
    let id = state.entities.spawn_creature(SPAWN, creature);
    (state, id)
}

fn extra_traits_of(state: &GameState, id: EntityId) -> Vec<String> {
    state
        .entities
        .get(id)
        .and_then(|e| e.as_creature())
        .map(|c| c.extra_traits.clone())
        .expect("creature under test")
}

#[test]
fn an_amalgam_is_grafted_with_the_authored_number_of_traits() {
    let game_data = GameData::load().expect("game data should load");
    let expected = game_data.traits["grafted"].graft_count as usize;
    assert!(expected > 0, "the `grafted` trait should graft something");

    let (mut state, id) = state_with(&game_data, "flesh_amalgam");
    graft_random_traits(&mut state, &game_data);

    let grafted = extra_traits_of(&state, id);
    assert_eq!(grafted.len(), expected);

    // Distinct, and every one is a real trait marked graftable.
    let mut sorted = grafted.clone();
    sorted.dedup();
    assert_eq!(sorted.len(), grafted.len(), "grafts should be distinct");
    for tag in &grafted {
        let data = game_data
            .traits
            .get(tag)
            .unwrap_or_else(|| panic!("`{tag}` should be a real trait"));
        assert!(data.graftable, "`{tag}` is not marked graftable");
    }
}

#[test]
fn an_ordinary_creature_is_grafted_with_nothing() {
    let game_data = GameData::load().expect("game data should load");
    let (mut state, id) = state_with(&game_data, "goblin");

    graft_random_traits(&mut state, &game_data);

    assert!(extra_traits_of(&state, id).is_empty());
}

/// Grafting happens once. A creature that re-rolled every tick would change
/// personality continuously, and none of the traits would mean anything.
#[test]
fn grafting_does_not_re_roll() {
    let game_data = GameData::load().expect("game data should load");
    let (mut state, id) = state_with(&game_data, "flesh_amalgam");

    graft_random_traits(&mut state, &game_data);
    let first = extra_traits_of(&state, id);

    for _ in 0..20 {
        let rolled = graft_random_traits(&mut state, &game_data);
        assert!(
            rolled.is_empty(),
            "an already-grafted creature should be skipped"
        );
    }
    assert_eq!(extra_traits_of(&state, id), first);
}

/// The reason the trait resolver was unified: `combat::extract_combat_stats`
/// had its own monster-traits-only lookup, so a grafted `strong` would raise a
/// creature's mood and leave its attack untouched.
#[test]
fn a_grafted_combat_trait_reaches_combat_stats() {
    let game_data = GameData::load().expect("game data should load");
    let (mut state, id) = state_with(&game_data, "flesh_amalgam");

    let base = combat::extract_combat_stats(state.entities.get(id).unwrap(), &game_data).attack;

    if let Some(c) = state.entities.get_mut(id).and_then(|e| e.as_creature_mut()) {
        c.extra_traits = vec!["strong".to_string()];
    }
    let grafted = combat::extract_combat_stats(state.entities.get(id).unwrap(), &game_data).attack;

    let expected = game_data.traits["strong"].attack_multiplier;
    assert!(expected > 1.0, "`strong` should raise attack at all");
    assert!(
        (grafted - base * expected).abs() < 0.01,
        "a grafted `strong` should multiply attack: {grafted} vs {base} * {expected}"
    );
}

#[test]
fn grafted_traits_reach_the_needs_resolver_too() {
    let game_data = GameData::load().expect("game data should load");
    let (mut state, id) = state_with(&game_data, "flesh_amalgam");

    if let Some(c) = state.entities.get_mut(id).and_then(|e| e.as_creature_mut()) {
        c.extra_traits = vec!["glutton".to_string()];
    }

    let creature = state
        .entities
        .get(id)
        .and_then(|e| e.as_creature())
        .unwrap();
    let tags: Vec<&str> =
        creature_traits(creature, &game_data.monsters["flesh_amalgam"], &game_data)
            .iter()
            .map(|t| t.id.as_str())
            .collect();

    assert!(
        tags.contains(&"grafted"),
        "its own kind's traits should remain"
    );
    assert!(
        tags.contains(&"glutton"),
        "the grafted trait should be included"
    );
}
