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
fn the_treasury_overflow_warning_fires_once_per_episode() {
    // The overflow itself happens on every dig, so an unconditional message
    // would bury the screen.
    let game_data = GameData::load().expect("game data should load");
    let mut player = PlayerState::new(&game_data);

    assert!(player.should_warn_treasury_full(), "first overflow is news");
    assert!(
        !player.should_warn_treasury_full(),
        "the second dig in a row is not"
    );

    player.clear_treasury_full_warning();
    assert!(
        player.should_warn_treasury_full(),
        "after the vault has room again, the next overflow is news"
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
