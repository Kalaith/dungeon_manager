//! Tests for creature-driven hero threat.
//!
//! "Attracts attention" is the Void-Touched's whole cost. Threat sets both the
//! gap between hero waves and how fast the hero garrison replenishes, so a
//! creature that raises it is a real trade rather than flavour text.

use crate::data::GameData;
use crate::state::entities::CreatureState;
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

fn spawn(state: &mut GameState, game_data: &GameData, creature_id: &str, at: TilePos) {
    let data = &game_data.monsters[creature_id];
    let creature = CreatureState::new(
        creature_id.to_string(),
        1,
        data.stats.health,
        data.stats.mana,
        1,
    );
    state.entities.spawn_creature(at, creature);
}

#[test]
fn a_void_touched_creature_raises_the_hero_threat() {
    let game_data = GameData::load().expect("game data should load");

    let quiet = GameState::new(24, 24, &game_data);
    let baseline = quiet.effective_threat_multiplier(&game_data);

    let mut noticed = GameState::new(24, 24, &game_data);
    spawn(&mut noticed, &game_data, "void_touched", TilePos::new(5, 5));

    assert!(
        noticed.effective_threat_multiplier(&game_data) > baseline,
        "keeping a Void-Touched should draw more attention than not"
    );
}

/// The control: an ordinary creature is not noticed at all, so the effect is
/// the trait rather than simply having an army.
#[test]
fn an_ordinary_creature_draws_no_extra_attention() {
    let game_data = GameData::load().expect("game data should load");

    let quiet = GameState::new(24, 24, &game_data);
    let baseline = quiet.effective_threat_multiplier(&game_data);

    let mut ordinary = GameState::new(24, 24, &game_data);
    for i in 0..5 {
        spawn(&mut ordinary, &game_data, "goblin", TilePos::new(5 + i, 5));
    }

    assert_eq!(
        ordinary.effective_threat_multiplier(&game_data),
        baseline,
        "goblins should not shorten the wave clock"
    );
}

#[test]
fn the_creature_threat_contribution_is_capped() {
    // Otherwise a stack of them would push the wave interval to nearly zero.
    let game_data = GameData::load().expect("game data should load");
    let baseline = GameState::new(24, 24, &game_data).effective_threat_multiplier(&game_data);

    let mut horde = GameState::new(24, 24, &game_data);
    for i in 0..40 {
        spawn(
            &mut horde,
            &game_data,
            "void_touched",
            TilePos::new(2 + (i % 18), 2 + (i / 18)),
        );
    }

    let drawn = horde.effective_threat_multiplier(&game_data);
    assert!(
        drawn <= baseline * 2.0 + 0.001,
        "creature threat should cap at doubling, was {drawn} vs {baseline}"
    );
    assert!(drawn > baseline, "but it should still be raised at all");
}

/// A corpse attracts nothing. Threat should track the living horde, or killing
/// your own Void-Touched would not relieve the pressure it caused.
#[test]
fn a_dead_void_touched_stops_drawing_attention() {
    let game_data = GameData::load().expect("game data should load");
    let baseline = GameState::new(24, 24, &game_data).effective_threat_multiplier(&game_data);

    let mut state = GameState::new(24, 24, &game_data);
    spawn(&mut state, &game_data, "void_touched", TilePos::new(5, 5));
    assert!(state.effective_threat_multiplier(&game_data) > baseline);

    let id = state
        .entities
        .creatures()
        .find(|(_, c)| c.creature_id == "void_touched")
        .map(|(id, _)| id)
        .expect("the void-touched should exist");
    if let Some(c) = state.entities.get_mut(id).and_then(|e| e.as_creature_mut()) {
        c.health = 0.0;
    }

    assert_eq!(
        state.effective_threat_multiplier(&game_data),
        baseline,
        "a dead one should stop attracting attention"
    );
}
