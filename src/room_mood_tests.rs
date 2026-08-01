//! Tests for the generic room `happiness_modifier` hook.
//!
//! Twelve rooms shipped a tuned `happiness_modifier` — lair +10 through torture
//! chamber -10 — that nothing read. These cover the hook that gives them
//! effect, and specifically that it stays *generic*: the value is read off
//! whatever room the creature stands in, with no room-type branch, which is
//! what lets an amenity room be authored as pure data.

use std::collections::HashSet;

use crate::data::GameData;
use crate::engine::creature_ai::needs::{calculate_mood, room_happiness_at};
use crate::engine::room_validator::Room;
use crate::state::entities::CreatureState;
use crate::state::room_manager::RoomManager;
use crate::state::tile_state::TilePos;

/// A room of `room_type` covering the single tile at `pos`.
fn room_manager_with(room_type: &str, pos: TilePos) -> RoomManager {
    let mut manager = RoomManager::new();
    let mut tiles = HashSet::new();
    tiles.insert(pos);
    manager
        .rooms
        .push(Room::new(0, room_type.to_string(), tiles, Vec::new()));
    manager
}

fn goblin(game_data: &GameData) -> (CreatureState, crate::data::monsters::MonsterData) {
    let data = game_data
        .monsters
        .get("goblin")
        .expect("goblin should exist")
        .clone();
    (
        CreatureState::new("goblin".to_string(), 1, data.stats.health, 0.0, 1),
        data,
    )
}

#[test]
fn room_happiness_reads_the_authored_modifier() {
    let game_data = GameData::load().expect("game data should load");
    let pos = TilePos::new(3, 3);

    let lair = room_manager_with("lair", pos);
    assert_eq!(room_happiness_at(pos, &lair, &game_data), 10.0);

    let torture = room_manager_with("torture_chamber", pos);
    assert_eq!(room_happiness_at(pos, &torture, &game_data), -10.0);
}

#[test]
fn room_happiness_is_zero_outside_any_room() {
    let game_data = GameData::load().expect("game data should load");
    let lair = room_manager_with("lair", TilePos::new(3, 3));

    // A tile the room does not cover.
    assert_eq!(
        room_happiness_at(TilePos::new(9, 9), &lair, &game_data),
        0.0
    );
    // And no rooms at all.
    assert_eq!(
        room_happiness_at(TilePos::new(3, 3), &RoomManager::new(), &game_data),
        0.0
    );
}

#[test]
fn room_happiness_resolves_the_training_hall_alias() {
    // The grid stores `training_room` while the data calls it `training_hall`;
    // without going through `room_data_id` the lookup silently returns 0 and
    // the room's -2 never applies.
    let game_data = GameData::load().expect("game data should load");
    let pos = TilePos::new(3, 3);
    let hall = room_manager_with("training_room", pos);

    assert_eq!(room_happiness_at(pos, &hall, &game_data), -2.0);
}

#[test]
fn standing_in_a_room_shifts_mood_by_its_modifier() {
    let game_data = GameData::load().expect("game data should load");
    let (creature, monster_data) = goblin(&game_data);

    let neutral = calculate_mood(&creature, &monster_data, &game_data, 0.0);
    let in_lair = calculate_mood(&creature, &monster_data, &game_data, 10.0);
    let in_torture = calculate_mood(&creature, &monster_data, &game_data, -10.0);

    assert_eq!(in_lair, neutral + 10.0);
    assert_eq!(in_torture, neutral - 10.0);
}

#[test]
fn mood_stays_clamped_with_an_extreme_room_modifier() {
    let game_data = GameData::load().expect("game data should load");
    let (creature, monster_data) = goblin(&game_data);

    assert_eq!(
        calculate_mood(&creature, &monster_data, &game_data, 1000.0),
        100.0
    );
    assert_eq!(
        calculate_mood(&creature, &monster_data, &game_data, -1000.0),
        0.0
    );
}

#[test]
fn every_room_with_a_modifier_resolves_through_the_hook() {
    // The generic claim, checked against the shipped data rather than a fixture:
    // whatever `rooms.json` authors is what a creature standing there feels.
    let game_data = GameData::load().expect("game data should load");
    let pos = TilePos::new(1, 1);

    let mut checked = 0;
    for (id, room) in &game_data.rooms {
        if room.effects.happiness_modifier == 0 {
            continue;
        }
        let tile_type = crate::data::rooms::room_tile_type(id);
        let manager = room_manager_with(tile_type, pos);
        assert_eq!(
            room_happiness_at(pos, &manager, &game_data),
            room.effects.happiness_modifier as f32,
            "room `{id}` did not resolve its happiness_modifier"
        );
        checked += 1;
    }
    assert!(
        checked >= 12,
        "expected the shipped rooms to carry modifiers, saw {checked}"
    );
}

/// The Ironbound's whole identity is the `construct` trait: "immune to morale,
/// eats nothing". Both halves go through machinery that already existed
/// (`need_decay_multipliers` and `desertion_threshold_modifier`), so this
/// checks the trait actually reaches them rather than trusting the JSON.
#[cfg(test)]
mod construct_trait {
    use crate::data::GameData;
    use crate::engine::creature_ai::needs::{update_mood, update_needs};
    use crate::state::entities::CreatureState;

    #[test]
    fn a_construct_never_gets_hungry_tired_or_greedy() {
        let game_data = GameData::load().expect("game data should load");
        let data = &game_data.monsters["ironbound"];
        let mut creature = CreatureState::new("ironbound".to_string(), 1, 340.0, 0.0, 1);

        // Ten simulated minutes. An ordinary creature would be starving.
        update_needs(&mut creature, 600.0, data, &game_data);

        for need in ["food", "sleep", "gold"] {
            assert_eq!(
                creature.get_need(need),
                100.0,
                "`{need}` should not decay for a construct"
            );
        }
    }

    #[test]
    fn a_construct_does_not_desert_even_at_rock_bottom_mood() {
        let game_data = GameData::load().expect("game data should load");
        let data = &game_data.monsters["ironbound"];
        let mut creature = CreatureState::new("ironbound".to_string(), 1, 340.0, 0.0, 1);

        // Drive every need to zero, which is as unhappy as a creature gets.
        for need in ["food", "sleep", "gold"] {
            creature.set_need(need.to_string(), 0.0);
        }
        update_mood(&mut creature, data, &game_data, -10.0);

        assert!(
            !creature.is_deserting,
            "a construct should never walk out (mood {})",
            creature.mood
        );
    }

    /// The comparison that gives the test above its meaning: an ordinary
    /// creature in the same state *does* starve, so the construct's immunity is
    /// the trait rather than a quirk of the fixture.
    #[test]
    fn an_ordinary_creature_in_the_same_state_does_get_hungry() {
        let game_data = GameData::load().expect("game data should load");
        let data = &game_data.monsters["orc"];
        let mut creature = CreatureState::new("orc".to_string(), 1, 200.0, 0.0, 1);

        update_needs(&mut creature, 600.0, data, &game_data);

        assert!(
            creature.get_need("food") < 100.0,
            "an orc should get hungry over ten minutes"
        );
    }
}
