//! Tests for who and what a spell may be aimed at.
//!
//! `valid_targets` is authored on 13 of the 17 spells and `requires_visibility`
//! on 7, and the cast check read neither. For creature-targeted spells that
//! meant the only test was "is *something* selected", so `heal` — declared
//! `["friendly"]` — could be cast on an invading knight, and `chickenify`
//! (`["enemy"]`) on one of your own goblins.

use crate::data::GameData;
use crate::engine::spell_effects::{can_cast_spell, CastResult};
use crate::state::entities::{CreatureState, HeroState};
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

fn playable_state(game_data: &GameData) -> GameState {
    let mut state = GameState::new_for_scenario(game_data, "dark_beginnings");
    // Cast checks come first on cooldown and cost; give the keeper the means
    // so a refusal can only be about targeting.
    state.player.mana = state.player.max_mana;
    state.player.gold = state.player.max_gold;
    state.player.spell_cooldowns.clear();
    state
}

fn spawn_goblin(state: &mut GameState, game_data: &GameData) -> crate::state::entities::EntityId {
    let data = game_data.monsters.get("goblin").expect("goblin");
    let goblin = CreatureState::new(
        "goblin".to_string(),
        1,
        data.stats.health,
        data.stats.mana,
        1,
    );
    state.entities.spawn_creature(TilePos::new(4, 4), goblin)
}

fn spawn_knight(state: &mut GameState, game_data: &GameData) -> crate::state::entities::EntityId {
    let data = game_data.heroes.get("knight").expect("knight");
    let knight = HeroState::new(
        "knight".to_string(),
        1,
        data.stats.health,
        data.stats.mana,
        TilePos::new(5, 5),
        1.0,
        0,
    );
    state.entities.spawn_hero(TilePos::new(5, 5), knight)
}

#[test]
fn a_friendly_spell_refuses_an_enemy() {
    let game_data = GameData::load().expect("game data should load");
    let mut state = playable_state(&game_data);
    let knight = spawn_knight(&mut state, &game_data);

    let heal = &game_data.spells["heal"];
    assert_eq!(
        heal.targeting.valid_targets,
        vec!["friendly".to_string()],
        "fixture assumption: heal is authored friendly-only"
    );

    let result = can_cast_spell(heal, &state, &game_data, None, Some(knight));
    assert!(
        matches!(result, CastResult::WrongAllegiance),
        "healing an invading knight should be refused, got {result:?}"
    );
}

#[test]
fn a_friendly_spell_accepts_your_own_creature() {
    let game_data = GameData::load().expect("game data should load");
    let mut state = playable_state(&game_data);
    let goblin = spawn_goblin(&mut state, &game_data);

    let heal = &game_data.spells["heal"];
    let result = can_cast_spell(heal, &state, &game_data, None, Some(goblin));
    assert!(
        matches!(result, CastResult::Success),
        "healing your own goblin should work, got {result:?}"
    );
}

#[test]
fn an_enemy_spell_refuses_your_own_creature() {
    let game_data = GameData::load().expect("game data should load");
    let mut state = playable_state(&game_data);
    let goblin = spawn_goblin(&mut state, &game_data);

    let chickenify = &game_data.spells["chickenify"];
    assert_eq!(
        chickenify.targeting.valid_targets,
        vec!["enemy".to_string()],
        "fixture assumption: chickenify is authored enemy-only"
    );

    let result = can_cast_spell(chickenify, &state, &game_data, None, Some(goblin));
    assert!(
        matches!(result, CastResult::WrongAllegiance),
        "chickenifying your own goblin should be refused, got {result:?}"
    );
}

#[test]
fn every_creature_spell_enforces_the_allegiance_it_declares() {
    // The general claim, checked against the shipped spell book rather than
    // three hand-picked spells.
    let game_data = GameData::load().expect("game data should load");

    let mut checked = 0;
    for (id, spell) in &game_data.spells {
        if spell.targeting.target_type != "creature" || spell.targeting.valid_targets.is_empty() {
            continue;
        }
        let wants_friendly = spell
            .targeting
            .valid_targets
            .iter()
            .any(|t| t == "friendly" || t == "ally");

        let mut state = playable_state(&game_data);
        let own = spawn_goblin(&mut state, &game_data);
        let foe = spawn_knight(&mut state, &game_data);

        let (should_pass, should_fail) = if wants_friendly {
            (own, foe)
        } else {
            (foe, own)
        };

        assert!(
            matches!(
                can_cast_spell(spell, &state, &game_data, None, Some(should_fail)),
                CastResult::WrongAllegiance
            ),
            "`{id}` accepted a target its valid_targets excludes"
        );
        assert!(
            matches!(
                can_cast_spell(spell, &state, &game_data, None, Some(should_pass)),
                CastResult::Success
            ),
            "`{id}` refused a target its valid_targets allows"
        );
        checked += 1;
    }

    assert!(
        checked >= 4,
        "expected several creature-targeted spells, saw {checked}"
    );
}

#[test]
fn a_spell_with_no_declared_targets_accepts_anyone() {
    // Empty `valid_targets` must stay permissive — 4 spells author none, and
    // tightening them would be a balance change smuggled in as plumbing.
    let game_data = GameData::load().expect("game data should load");
    let mut state = playable_state(&game_data);
    let knight = spawn_knight(&mut state, &game_data);

    let mut permissive = game_data.spells["heal"].clone();
    permissive.targeting.valid_targets.clear();

    assert!(matches!(
        can_cast_spell(&permissive, &state, &game_data, None, Some(knight)),
        CastResult::Success
    ));
}
