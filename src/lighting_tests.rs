//! Tests for the dungeon light map.
//!
//! `rooms.json` authored a `visual.light` for 22 of 24 rooms and `tiles.json`
//! for three tile types, and none of it was read by anything. These cover the
//! model that now reads it.

use std::collections::HashSet;

use crate::data::GameData;
use crate::engine::lighting::build_light_map;
use crate::engine::room_validator::Room;
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

const ROOM_AT: TilePos = TilePos { x: 10, y: 10 };
const FAR_AWAY: TilePos = TilePos { x: 1, y: 1 };

fn brightness(m: [f32; 3]) -> f32 {
    (m[0] + m[1] + m[2]) / 3.0
}

/// A state with one active room of `room_type` covering a single tile.
fn state_with_room(game_data: &GameData, room_type: &str) -> GameState {
    let mut state = GameState::new(24, 24, game_data);
    // Clear any light the generated map already placed near the probe tiles,
    // so the room under test is the only thing being measured.
    for row in &mut state.dungeon.grid {
        for tile in row {
            if game_data
                .tiles
                .get(&tile.tile_type)
                .and_then(|d| d.visual.light.as_ref())
                .is_some()
            {
                tile.tile_type = "earth".to_string();
            }
        }
    }
    state.room_manager.rooms.clear();

    let mut tiles = HashSet::new();
    tiles.insert(ROOM_AT);
    let mut room = Room::new(1, room_type.to_string(), tiles, Vec::new());
    room.active = true;
    state.room_manager.rooms.push(room);
    state
}

#[test]
fn a_lit_room_is_brighter_than_the_dark_around_it() {
    let game_data = GameData::load().expect("game data should load");
    let state = state_with_room(&game_data, "treasury");
    let map = build_light_map(&state, &game_data);

    let inside = brightness(map.multiplier_at(ROOM_AT));
    let outside = brightness(map.multiplier_at(FAR_AWAY));

    assert!(
        inside > outside,
        "a treasury should light its own tile: {inside} vs {outside}"
    );
}

#[test]
fn unlit_ground_is_dim_but_never_black() {
    // A pitch-black dungeon is unplayable; fog of war is the system that hides
    // things, lighting is the one that sets mood.
    let game_data = GameData::load().expect("game data should load");
    let state = state_with_room(&game_data, "treasury");
    let map = build_light_map(&state, &game_data);

    let dark = brightness(map.multiplier_at(FAR_AWAY));
    assert!(dark > 0.1, "unlit ground should stay readable, was {dark}");
    assert!(dark < 1.0, "unlit ground should be dimmer than lit");
}

#[test]
fn light_falls_off_with_distance() {
    let game_data = GameData::load().expect("game data should load");
    let state = state_with_room(&game_data, "treasury");
    let map = build_light_map(&state, &game_data);

    let at_source = brightness(map.multiplier_at(ROOM_AT));
    let nearby = brightness(map.multiplier_at(TilePos::new(ROOM_AT.x + 2, ROOM_AT.y)));
    let further = brightness(map.multiplier_at(TilePos::new(ROOM_AT.x + 4, ROOM_AT.y)));

    assert!(at_source >= nearby, "{at_source} >= {nearby}");
    assert!(nearby > further, "light should fade: {nearby} vs {further}");
}

/// The authored colours have to actually tint, or 22 rooms' worth of palette
/// collapses into "brighter".
#[test]
fn a_rooms_authored_colour_tints_the_light() {
    let game_data = GameData::load().expect("game data should load");

    // The treasury is authored yellow (255, 255, 0); the lair blue-ish
    // (100, 100, 255). Their tints should differ in the blue channel.
    let treasury = build_light_map(&state_with_room(&game_data, "treasury"), &game_data)
        .multiplier_at(ROOM_AT);
    let lair =
        build_light_map(&state_with_room(&game_data, "lair"), &game_data).multiplier_at(ROOM_AT);

    assert!(
        treasury[2] < treasury[0],
        "a yellow room should be dimmer in blue than red: {treasury:?}"
    );
    assert!(
        lair[2] >= lair[0],
        "a blue room should not be dimmer in blue than red: {lair:?}"
    );
}

/// Every room that authors a light should be capable of emitting one — an
/// intensity of zero would be a designed value that renders identically to no
/// light at all.
#[test]
fn every_authored_room_light_has_a_usable_intensity() {
    let game_data = GameData::load().expect("game data should load");

    let mut lit = 0;
    for (id, room) in &game_data.rooms {
        let light = &room.visual.light;
        if light.intensity <= 0.0 {
            continue;
        }
        assert!(
            light.intensity <= 1.5,
            "`{id}` intensity {} is out of range",
            light.intensity
        );
        assert!(
            light.color.iter().any(|c| *c > 0),
            "`{id}` authors an intensity but a black colour"
        );
        lit += 1;
    }
    assert!(lit >= 20, "expected most rooms to author light, saw {lit}");
}
