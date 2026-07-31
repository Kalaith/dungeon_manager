//! Tests for the rules governing where and whether a room may be built.
//!
//! `global_rooms_required` was the sharp one: it was read, but only to draw a
//! tooltip. The Arcane Archive's build button said "Requires: Library" and the
//! engine let you build it without one — a promise the UI made and the
//! simulation did not keep. `max_instances` and `forbidden_if` were read
//! nowhere at all.

use std::collections::HashSet;

use crate::data::GameData;
use crate::engine::room_validator::{
    dungeon_permits_room, tile_permits_room, PlacementRefusal, Room,
};
use crate::state::tile_state::{Ownership, TilePos, TileState};

fn active_room(id: usize, room_type: &str) -> Room {
    let mut tiles = HashSet::new();
    tiles.insert(TilePos::new(id as i32, 0));
    let mut room = Room::new(id, room_type.to_string(), tiles, Vec::new());
    room.active = true;
    room
}

fn player_floor() -> TileState {
    let mut tile = TileState::new("claimed_floor".to_string(), TilePos::new(3, 3));
    tile.ownership = Ownership::Player;
    tile
}

#[test]
fn a_room_needing_a_prerequisite_is_refused_without_it() {
    let game_data = GameData::load().expect("game data should load");
    let archive = &game_data.rooms["arcane_archive"];
    assert_eq!(
        archive.requirements.global_rooms_required,
        vec!["library".to_string()],
        "fixture assumption: the archive requires a library"
    );

    assert_eq!(
        dungeon_permits_room(archive, &[]),
        Err(PlacementRefusal::MissingPrerequisite("library".to_string()))
    );

    let with_library = [active_room(1, "library")];
    assert_eq!(dungeon_permits_room(archive, &with_library), Ok(()));
}

#[test]
fn every_authored_prerequisite_is_enforced() {
    // Whatever `rooms.json` says a room needs, an empty dungeon must refuse.
    let game_data = GameData::load().expect("game data should load");

    let mut checked = 0;
    for (id, room) in &game_data.rooms {
        let Some(required) = room.requirements.global_rooms_required.first() else {
            continue;
        };
        assert_eq!(
            dungeon_permits_room(room, &[]),
            Err(PlacementRefusal::MissingPrerequisite(required.clone())),
            "`{id}` should be refused without its prerequisite"
        );
        checked += 1;
    }
    assert!(checked >= 5, "expected several gated rooms, saw {checked}");
}

#[test]
fn an_unrestricted_room_is_always_permitted() {
    let game_data = GameData::load().expect("game data should load");
    let lair = &game_data.rooms["lair"];
    assert!(lair.requirements.global_rooms_required.is_empty());
    assert_eq!(dungeon_permits_room(lair, &[]), Ok(()));
}

#[test]
fn max_instances_caps_the_count() {
    let game_data = GameData::load().expect("game data should load");
    // Nothing ships with a cap, so synthesise one rather than pretend.
    let mut capped = game_data.rooms["lair"].clone();
    capped.requirements.max_instances = 2;

    assert_eq!(dungeon_permits_room(&capped, &[]), Ok(()));
    assert_eq!(
        dungeon_permits_room(&capped, &[active_room(1, "lair")]),
        Ok(())
    );
    assert_eq!(
        dungeon_permits_room(&capped, &[active_room(1, "lair"), active_room(2, "lair")]),
        Err(PlacementRefusal::AlreadyAtLimit(2))
    );
}

#[test]
fn forbidden_if_blocks_a_conflicting_room() {
    let game_data = GameData::load().expect("game data should load");
    let mut exclusive = game_data.rooms["lair"].clone();
    exclusive.requirements.forbidden_if = vec!["graveyard".to_string()];

    assert_eq!(dungeon_permits_room(&exclusive, &[]), Ok(()));
    assert_eq!(
        dungeon_permits_room(&exclusive, &[active_room(1, "graveyard")]),
        Err(PlacementRefusal::ForbiddenBy("graveyard".to_string()))
    );
}

#[test]
fn a_room_refuses_terrain_its_data_does_not_allow() {
    let game_data = GameData::load().expect("game data should load");
    let lair = &game_data.rooms["lair"];
    assert_eq!(
        lair.build.allowed_terrain,
        vec!["claimed_floor".to_string()]
    );

    assert!(tile_permits_room(lair, &player_floor(), &game_data));

    let mut wrong_terrain = player_floor();
    wrong_terrain.tile_type = "lava".to_string();
    assert!(!tile_permits_room(lair, &wrong_terrain, &game_data));
}

#[test]
fn requires_claimed_and_can_overlap_are_honoured() {
    let game_data = GameData::load().expect("game data should load");
    let lair = &game_data.rooms["lair"];

    let mut unclaimed = player_floor();
    unclaimed.ownership = Ownership::Unclaimed;
    assert!(!tile_permits_room(lair, &unclaimed, &game_data));

    let mut already_a_room = player_floor();
    already_a_room.room_id = Some(4);
    assert!(!tile_permits_room(lair, &already_a_room, &game_data));

    // ...unless the room is authored to overlap.
    let mut overlapping = lair.clone();
    overlapping.build.can_overlap = true;
    assert!(tile_permits_room(&overlapping, &already_a_room, &game_data));
}

#[test]
fn every_shipped_room_can_still_be_built_on_ordinary_claimed_floor() {
    // These rules replaced hardcoded conditions that matched every shipped
    // room exactly. Nothing should have become unbuildable.
    let game_data = GameData::load().expect("game data should load");
    let tile = player_floor();

    for (id, room) in &game_data.rooms {
        if id == "dungeon_heart" {
            continue; // Placed by the map, never built.
        }
        assert!(
            tile_permits_room(room, &tile, &game_data),
            "`{id}` became unbuildable on plain claimed floor"
        );
    }
}
