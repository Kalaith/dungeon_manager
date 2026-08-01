//! Tests for the Stone Warden's wall reinforcement.
//!
//! The point of this mechanic is that it is *not* cosmetic: `diggable` is
//! enforced in `hero_digging`, so converting a wall genuinely closes a tunnel
//! route. These cover both halves — that a warden converts, and that what it
//! converts actually stops a hero.

use crate::data::GameData;
use crate::engine::wall_reinforcement::reinforce_walls;
use crate::state::entities::CreatureState;
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

const WARDEN_POS: TilePos = TilePos { x: 6, y: 6 };
const WALL_POS: TilePos = TilePos { x: 6, y: 5 };

/// A dungeon with claimed floor under the warden and one plain wall north of
/// it, with the other three neighbours cleared so the target is unambiguous.
fn state_with_one_wall(game_data: &GameData) -> GameState {
    let mut state = GameState::new(20, 20, game_data);
    for (dx, dy, tile) in [
        (0, 0, "claimed_floor"),
        (0, -1, "earth"),
        (1, 0, "claimed_floor"),
        (0, 1, "claimed_floor"),
        (-1, 0, "claimed_floor"),
    ] {
        let pos = TilePos::new(WARDEN_POS.x + dx, WARDEN_POS.y + dy);
        if let Some(t) = state.dungeon.get_tile_mut(pos) {
            t.tile_type = tile.to_string();
            t.trap = None;
        }
    }
    state
}

fn spawn_warden(state: &mut GameState, creature_id: &str) {
    let creature = CreatureState::new(creature_id.to_string(), 1, 300.0, 0.0, 1);
    state.entities.spawn_creature(WARDEN_POS, creature);
}

#[test]
fn a_stone_warden_turns_the_wall_beside_it_into_reinforced_wall() {
    let game_data = GameData::load().expect("game data should load");
    let mut state = state_with_one_wall(&game_data);
    spawn_warden(&mut state, "stone_warden");

    // One tick shorter than the interval: nothing yet.
    reinforce_walls(&mut state, &game_data, 5.0);
    assert_eq!(
        state.dungeon.get_tile(WALL_POS).unwrap().tile_type,
        "earth",
        "the warden should not finish instantly"
    );

    // Past the authored interval.
    reinforce_walls(&mut state, &game_data, 60.0);
    assert_eq!(
        state.dungeon.get_tile(WALL_POS).unwrap().tile_type,
        "reinforced_wall",
        "the warden should have shored the wall beside it"
    );
}

/// The control: an ordinary creature standing in exactly the same spot for the
/// same time changes nothing, so the effect is the trait and not the fixture.
#[test]
fn an_ordinary_creature_reinforces_nothing() {
    let game_data = GameData::load().expect("game data should load");
    let mut state = state_with_one_wall(&game_data);
    spawn_warden(&mut state, "troll");

    reinforce_walls(&mut state, &game_data, 600.0);

    assert_eq!(
        state.dungeon.get_tile(WALL_POS).unwrap().tile_type,
        "earth",
        "only a stonebinding creature should reinforce"
    );
}

/// Resource tiles are excluded by data, not by naming them: a warden must not
/// seal away a gold vein the imps are still working.
#[test]
fn a_warden_will_not_seal_a_resource_tile() {
    let game_data = GameData::load().expect("game data should load");
    let mut state = state_with_one_wall(&game_data);
    if let Some(t) = state.dungeon.get_tile_mut(WALL_POS) {
        t.tile_type = "gold_vein".to_string();
    }
    spawn_warden(&mut state, "stone_warden");

    reinforce_walls(&mut state, &game_data, 600.0);

    assert_eq!(
        state.dungeon.get_tile(WALL_POS).unwrap().tile_type,
        "gold_vein",
        "a gold vein is a wall, but not one worth sealing"
    );
}

/// The property the whole mechanic rests on: what the warden produces is a tile
/// heroes cannot dig. If `reinforced_wall` ever became diggable, reinforcement
/// would silently turn into decoration.
#[test]
fn what_the_warden_produces_is_a_tile_heroes_cannot_dig() {
    let game_data = GameData::load().expect("game data should load");

    let reinforced = game_data
        .tiles
        .get("reinforced_wall")
        .expect("reinforced_wall should exist in tiles.json");
    let earth = game_data.tiles.get("earth").expect("earth should exist");

    assert!(earth.diggable, "the fixture assumes earth is diggable");
    assert!(
        !reinforced.diggable,
        "reinforced_wall must be undiggable or the Stone Warden does nothing"
    );
    assert!(
        reinforced.blocks_movement,
        "reinforced_wall must still be a wall"
    );
}
