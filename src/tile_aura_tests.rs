//! Tests for tile auras.
//!
//! `bone_floor` has declared `hero_fear: 0.1` over a radius of 2 since long
//! before there was a fear mechanic to hang it on. These cover it now that
//! there is.

use crate::data::GameData;
use crate::engine::hero_ai::{current_retreat_threshold, effective_retreat_threshold};
use crate::engine::tile_aura::hero_fear_at;
use crate::state::entities::{HeroGoal, HeroState};
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

const AT: TilePos = TilePos { x: 8, y: 8 };

fn state_with_floor(game_data: &GameData, tile_type: &str) -> GameState {
    let mut state = GameState::new(24, 24, game_data);
    if let Some(tile) = state.dungeon.get_tile_mut(AT) {
        tile.tile_type = tile_type.to_string();
    }
    state
}

#[test]
fn standing_on_bone_floor_is_frightening() {
    let game_data = GameData::load().expect("game data should load");
    let state = state_with_floor(&game_data, "bone_floor");

    let fear = hero_fear_at(AT, &state.dungeon, &game_data);
    assert!(fear > 0.0, "a bone floor should frighten, saw {fear}");
}

#[test]
fn ordinary_floor_frightens_nobody() {
    let game_data = GameData::load().expect("game data should load");
    let state = state_with_floor(&game_data, "claimed_floor");

    assert_eq!(hero_fear_at(AT, &state.dungeon, &game_data), 0.0);
}

/// The aura has a radius, and it is respected in both directions.
#[test]
fn the_aura_reaches_its_authored_radius_and_no_further() {
    let game_data = GameData::load().expect("game data should load");
    let state = state_with_floor(&game_data, "bone_floor");
    let radius = game_data.tiles["bone_floor"]
        .special
        .as_ref()
        .and_then(|s| s.aura.as_ref())
        .map(|a| a.radius as i32)
        .expect("bone_floor should declare an aura radius");

    let inside = TilePos::new(AT.x + radius, AT.y);
    let outside = TilePos::new(AT.x + radius + 2, AT.y);

    assert!(
        hero_fear_at(inside, &state.dungeon, &game_data) > 0.0,
        "a tile at the edge of the radius should still be affected"
    );
    assert_eq!(
        hero_fear_at(outside, &state.dungeon, &game_data),
        0.0,
        "beyond the radius, nothing"
    );
}

/// The point of the whole thing: standing on it actually moves the decision a
/// hero makes, not just a number.
#[test]
fn ambient_fear_raises_a_heros_breaking_point() {
    let game_data = GameData::load().expect("game data should load");
    let state = state_with_floor(&game_data, "bone_floor");
    let data = &game_data.heroes["acolyte"];

    let mut hero = HeroState::new(
        "acolyte".to_string(),
        1,
        data.stats.health,
        data.stats.mana,
        AT,
        1.0,
        0,
    );
    hero.current_goal = HeroGoal::DestroyHeart;

    let calm = current_retreat_threshold(&hero, data, 0.0);
    let ambient = hero_fear_at(AT, &state.dungeon, &game_data);
    let scared = current_retreat_threshold(&hero, data, ambient);

    assert_eq!(calm, effective_retreat_threshold(data));
    assert!(
        scared > calm,
        "standing on bone should raise the breaking point: {scared} vs {calm}"
    );
}
