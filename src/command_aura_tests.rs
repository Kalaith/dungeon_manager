//! Tests for the Overseer's command aura and the Archivist's own multiplier.
//!
//! Both feed `calculate_work_efficiency`, which is the single term
//! `execute_work` and `execute_research` both scale by — so proving the
//! multiplier lands there proves it reaches production and research alike.

use crate::data::GameData;
use crate::engine::command_aura::apply_command_auras;
use crate::engine::creature_ai::calculate_work_efficiency;
use crate::state::entities::CreatureState;
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

const WORKER_POS: TilePos = TilePos { x: 8, y: 8 };

fn efficiency_of(state: &GameState, game_data: &GameData, creature_id: &str) -> f32 {
    let (_, creature) = state
        .entities
        .creatures()
        .find(|(_, c)| c.creature_id == creature_id)
        .expect("creature should exist");
    let data = &game_data.monsters[creature_id];
    calculate_work_efficiency(creature, data, game_data)
}

/// Place a goblin worker, optionally with a companion `distance` tiles away.
fn state_with(game_data: &GameData, companion: Option<(&str, i32)>) -> GameState {
    let mut state = GameState::new(24, 24, game_data);
    let mut worker = CreatureState::new("goblin".to_string(), 1, 120.0, 0.0, 1);
    worker.mood = 60.0;
    state.entities.spawn_creature(WORKER_POS, worker);

    if let Some((id, distance)) = companion {
        let data = &game_data.monsters[id];
        let mut mate = CreatureState::new(id.to_string(), 1, data.stats.health, 0.0, 2);
        mate.mood = 60.0;
        state
            .entities
            .spawn_creature(TilePos::new(WORKER_POS.x + distance, WORKER_POS.y), mate);
    }
    state
}

#[test]
fn an_overseer_nearby_raises_a_workers_efficiency() {
    let game_data = GameData::load().expect("game data should load");

    let mut alone = state_with(&game_data, None);
    apply_command_auras(&mut alone, &game_data);
    let solo = efficiency_of(&alone, &game_data, "goblin");

    let mut watched = state_with(&game_data, Some(("overseer", 2)));
    apply_command_auras(&mut watched, &game_data);
    let commanded = efficiency_of(&watched, &game_data, "goblin");

    assert!(
        solo > 0.0,
        "the worker should have some efficiency to begin with"
    );
    assert!(
        commanded > solo,
        "an overseer two tiles away should raise output: {commanded} vs {solo}"
    );
}

#[test]
fn the_aura_does_not_reach_across_the_map() {
    let game_data = GameData::load().expect("game data should load");

    let mut alone = state_with(&game_data, None);
    apply_command_auras(&mut alone, &game_data);
    let solo = efficiency_of(&alone, &game_data, "goblin");

    // Well outside the authored `command_radius`.
    let mut distant = state_with(&game_data, Some(("overseer", 12)));
    apply_command_auras(&mut distant, &game_data);
    let far = efficiency_of(&distant, &game_data, "goblin");

    assert_eq!(
        far, solo,
        "an overseer across the map should change nothing"
    );
}

/// The control: an ordinary creature standing at the same distance does
/// nothing, so the bonus is the trait rather than mere company.
#[test]
fn an_ordinary_neighbour_changes_nothing() {
    let game_data = GameData::load().expect("game data should load");

    let mut alone = state_with(&game_data, None);
    apply_command_auras(&mut alone, &game_data);
    let solo = efficiency_of(&alone, &game_data, "goblin");

    let mut paired = state_with(&game_data, Some(("orc", 2)));
    apply_command_auras(&mut paired, &game_data);
    let with_orc = efficiency_of(&paired, &game_data, "goblin");

    assert_eq!(with_orc, solo, "only `commanding` should matter");
}

/// The aura is for the creatures being overseen, not for the overseer's own
/// output — otherwise stacking two of them would be a self-buff loop.
#[test]
fn an_overseer_does_not_command_itself() {
    let game_data = GameData::load().expect("game data should load");

    let mut solo = GameState::new(24, 24, &game_data);
    let mut boss = CreatureState::new("overseer".to_string(), 1, 140.0, 40.0, 1);
    boss.mood = 60.0;
    solo.entities.spawn_creature(WORKER_POS, boss);
    apply_command_auras(&mut solo, &game_data);

    let (_, creature) = solo
        .entities
        .creatures()
        .find(|(_, c)| c.creature_id == "overseer")
        .expect("overseer");
    assert_eq!(
        creature.command_bonus, 1.0,
        "a lone overseer should not be commanding itself"
    );
}

#[test]
fn the_archivist_out_researches_an_equally_happy_warlock() {
    // Same mood, same health fraction: the only difference is `scholarship`.
    let game_data = GameData::load().expect("game data should load");
    let mut state = GameState::new(24, 24, &game_data);

    for id in ["archivist", "warlock"] {
        let data = &game_data.monsters[id];
        let mut c = CreatureState::new(id.to_string(), 1, data.stats.health, data.stats.mana, 1);
        c.mood = 60.0;
        state
            .entities
            .spawn_creature(TilePos::new(2, if id == "archivist" { 2 } else { 6 }), c);
    }
    apply_command_auras(&mut state, &game_data);

    let archivist = efficiency_of(&state, &game_data, "archivist");
    let warlock = efficiency_of(&state, &game_data, "warlock");

    assert!(
        archivist > warlock,
        "scholarship should raise research output: {archivist} vs {warlock}"
    );
}
