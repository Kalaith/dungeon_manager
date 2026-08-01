//! Tests for player-facing feedback from deep engine layers.
//!
//! Refusals used to be `eprintln!`, so a player who tried to build an
//! unresearched room, place a trap with no Workshop, or filled their treasury
//! saw nothing happen at all. These check the messages actually reach the
//! notification manager, and that the noisiest one does not repeat.

use crate::data::GameData;
use crate::engine::input_handlers;
use crate::state::game_state::GameState;
use crate::state::player_state::PlayerState;
use crate::state::tile_state::{Ownership, TilePos};

fn state_with_claimed_floor(game_data: &GameData, pos: TilePos) -> GameState {
    let mut state = GameState::new_for_scenario(game_data, "dark_beginnings");
    if let Some(tile) = state.dungeon.get_tile_mut(pos) {
        tile.tile_type = "claimed_floor".to_string();
        tile.ownership = Ownership::Player;
        tile.room_id = None;
    }
    state.player.gold = state.player.max_gold;
    state.player.mana = state.player.max_mana;
    state
}

#[test]
fn building_an_unresearched_room_tells_the_player_why() {
    let game_data = GameData::load().expect("game data should load");
    let pos = TilePos::new(6, 6);
    let mut state = state_with_claimed_floor(&game_data, pos);

    // The Combat Pit is gated behind `blood_sport`.
    assert!(!state.player.is_room_unlocked("combat_pit"));
    let before = state.notifications.count();

    input_handlers::handle_build_room(&mut state, &game_data, "combat_pit", pos);

    assert!(
        state.notifications.count() > before,
        "a refused build should say something"
    );
}

#[test]
fn placing_a_trap_without_a_workshop_tells_the_player_why() {
    let game_data = GameData::load().expect("game data should load");
    let pos = TilePos::new(6, 6);
    let mut state = state_with_claimed_floor(&game_data, pos);
    state.player.unlock_trap("spike_trap".to_string());

    let has_workshop = state
        .room_manager
        .rooms
        .iter()
        .any(|r| r.active && r.room_type == "workshop");
    assert!(!has_workshop, "fixture assumption: no workshop yet");

    let before = state.notifications.count();
    input_handlers::handle_build_trap(&mut state, &game_data, "spike_trap", pos);

    assert!(
        state.notifications.count() > before,
        "a refused trap placement should say something"
    );
}

#[test]
fn a_repeating_condition_is_reported_once_per_episode() {
    // Both the treasury overflow and the spawn checks fire every tick, so an
    // unconditional message would bury the screen.
    let game_data = GameData::load().expect("game data should load");
    let mut player = PlayerState::new(&game_data);

    player.warn_once("treasury_full", "first");
    assert_eq!(player.pending_messages.len(), 1, "the first time is news");

    player.warn_once("treasury_full", "second");
    assert_eq!(
        player.pending_messages.len(),
        1,
        "the same condition next tick is not"
    );

    player.clear_warning("treasury_full");
    player.warn_once("treasury_full", "third");
    assert_eq!(
        player.pending_messages.len(),
        2,
        "after the condition clears, its return is news again"
    );
}

#[test]
fn different_conditions_do_not_mask_each_other() {
    // A single bool per warning would not have survived the spawner's four
    // reasons; the keyed set is why they stay independent.
    let game_data = GameData::load().expect("game data should load");
    let mut player = PlayerState::new(&game_data);

    player.warn_once("spawn_no_lair", "no lair");
    player.warn_once("spawn_unreachable", "cut off");
    assert_eq!(player.pending_messages.len(), 2);

    player.clear_warning("spawn_no_lair");
    player.warn_once("spawn_unreachable", "cut off again");
    assert_eq!(
        player.pending_messages.len(),
        2,
        "clearing one key must not re-arm another"
    );
}

#[test]
fn a_spawn_with_nowhere_to_sleep_tells_the_player() {
    let game_data = GameData::load().expect("game data should load");
    let mut state = GameState::new_for_scenario(&game_data, "dark_beginnings");
    state.player.pending_messages.clear();
    state.player.warned_keys.clear();

    // `dark_beginnings` starts with no lair dug, so the spawn tick has
    // nowhere to put a creature.
    state.spawn_random_creature(&game_data);

    assert!(
        !state.player.pending_messages.is_empty() || !state.player.warned_keys.is_empty(),
        "a blocked spawn should report why"
    );
}

#[test]
fn queued_engine_messages_reach_the_player() {
    let game_data = GameData::load().expect("game data should load");
    let mut player = PlayerState::new(&game_data);
    assert!(player.pending_messages.is_empty());

    player.notify("the dungeon groans");
    assert_eq!(player.pending_messages.len(), 1);
}
