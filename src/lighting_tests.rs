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

/// The Shadow Stalker's whole identity: the same creature hits harder in the
/// dark. This is the first thing that makes the keeper's *lighting* a tactical
/// decision — building a lit room beside a corridor now weakens the creature
/// patrolling it.
#[cfg(test)]
mod darkness_combat {
    use super::*;
    use crate::engine::combat;
    use crate::engine::lighting::cache_creature_darkness;
    use crate::state::entities::{CreatureState, EntityId};

    const STAND_AT: TilePos = TilePos { x: 4, y: 4 };

    fn attack_of(creature_id: &str, lit_room: Option<&str>) -> f32 {
        let game_data = GameData::load().expect("game data should load");
        let mut state = if let Some(room_type) = lit_room {
            let mut s = state_with_room(&game_data, room_type);
            // Put the room under the creature rather than off at ROOM_AT.
            s.room_manager.rooms.clear();
            let mut tiles = HashSet::new();
            tiles.insert(STAND_AT);
            let mut room = Room::new(1, room_type.to_string(), tiles, Vec::new());
            room.active = true;
            s.room_manager.rooms.push(room);
            s
        } else {
            state_with_room(&game_data, "treasury")
        };
        if lit_room.is_none() {
            state.room_manager.rooms.clear();
        }

        let data = &game_data.monsters[creature_id];
        let creature = CreatureState::new(
            creature_id.to_string(),
            1,
            data.stats.health,
            data.stats.mana,
            1,
        );
        let id: EntityId = state.entities.spawn_creature(STAND_AT, creature);

        state.light_map = build_light_map(&state, &game_data);
        cache_creature_darkness(&mut state);

        combat::extract_combat_stats(state.entities.get(id).unwrap(), &game_data).attack
    }

    #[test]
    fn a_shadow_stalker_hits_harder_in_the_dark() {
        let in_the_dark = attack_of("shadow_stalker", None);
        let under_a_lamp = attack_of("shadow_stalker", Some("treasury"));

        assert!(
            in_the_dark > under_a_lamp,
            "darkness should raise its attack: {in_the_dark} vs {under_a_lamp}"
        );
    }

    /// The control: an ordinary creature is indifferent to the light, so the
    /// effect is the `lightless` trait rather than the lighting model leaking
    /// into everyone's combat stats.
    #[test]
    fn an_ordinary_creature_fights_the_same_either_way() {
        let dark = attack_of("orc", None);
        let lit = attack_of("orc", Some("treasury"));

        assert_eq!(dark, lit, "only `lightless` should care about the light");
    }
}

/// `behavior.light_preference` — authored for all twenty heroes as `bright`,
/// `dark` or `any`, and read by nothing until the lighting pass existed.
#[cfg(test)]
mod light_preference {
    use super::*;
    use crate::engine::lighting::discomfort_for;

    fn map_with_light(
        game_data: &GameData,
        room_type: Option<&str>,
    ) -> crate::engine::lighting::LightMap {
        let state = match room_type {
            Some(rt) => state_with_room(game_data, rt),
            None => {
                let mut s = state_with_room(game_data, "treasury");
                s.room_manager.rooms.clear();
                s
            }
        };
        build_light_map(&state, game_data)
    }

    #[test]
    fn a_light_preferring_hero_is_less_steady_in_the_dark() {
        let game_data = GameData::load().expect("game data should load");
        let dark = map_with_light(&game_data, None);
        let lit = map_with_light(&game_data, Some("treasury"));

        let in_dark = discomfort_for("bright", ROOM_AT, &dark);
        let in_light = discomfort_for("bright", ROOM_AT, &lit);

        assert!(
            in_dark > in_light,
            "a bright-preferring hero should be more unsettled in the dark: {in_dark} vs {in_light}"
        );
    }

    /// The rogue is the only hero authored `dark`, and it should read the
    /// opposite way — otherwise the field is just "everyone fears the dark".
    #[test]
    fn a_dark_preferring_hero_reads_the_other_way() {
        let game_data = GameData::load().expect("game data should load");
        assert_eq!(
            game_data.heroes["rogue"].behavior.light_preference, "dark",
            "this test assumes the rogue prefers the dark"
        );

        let dark = map_with_light(&game_data, None);
        let lit = map_with_light(&game_data, Some("treasury"));

        assert!(
            discomfort_for("dark", ROOM_AT, &lit) > discomfort_for("dark", ROOM_AT, &dark),
            "a dark-preferring hero should be uneasy under a lamp, not in the dark"
        );
    }

    #[test]
    fn an_indifferent_hero_is_unaffected_either_way() {
        let game_data = GameData::load().expect("game data should load");
        let dark = map_with_light(&game_data, None);
        let lit = map_with_light(&game_data, Some("treasury"));

        assert_eq!(discomfort_for("any", ROOM_AT, &dark), 0.0);
        assert_eq!(discomfort_for("any", ROOM_AT, &lit), 0.0);
    }

    /// Fail-quiet by design, but only for values the guard in
    /// `content_wiring_tests` will not let into the data.
    #[test]
    fn an_unrecognised_preference_has_no_effect() {
        let game_data = GameData::load().expect("game data should load");
        let dark = map_with_light(&game_data, None);

        assert_eq!(discomfort_for("candlelit", ROOM_AT, &dark), 0.0);
    }
}

/// End-to-end: the preference has to change what a hero actually *does*, not
/// just what a helper returns.
#[cfg(test)]
mod light_preference_behaviour {
    use super::*;
    use crate::engine::hero_ai::{effective_retreat_threshold, should_reconsider_goal};
    use crate::engine::lighting::discomfort_for;
    use crate::state::entities::{HeroGoal, HeroState};

    #[test]
    fn a_wounded_knight_breaks_off_in_the_dark_but_presses_on_in_the_light() {
        let game_data = GameData::load().expect("game data should load");
        let data = &game_data.heroes["knight"];
        assert_eq!(
            data.behavior.light_preference, "bright",
            "this test assumes the knight prefers the light"
        );
        assert!(
            !data.behavior.will_fight_to_death,
            "a hero who never retreats cannot demonstrate this"
        );

        let dark_map = {
            let mut s = state_with_room(&game_data, "treasury");
            s.room_manager.rooms.clear();
            build_light_map(&s, &game_data)
        };
        let dark_fear = discomfort_for("bright", ROOM_AT, &dark_map);
        assert!(dark_fear > 0.0, "the dark should unsettle a bright hero");

        // A wound between the calm and frightened breaking points.
        let base = effective_retreat_threshold(data);
        let scared = base + dark_fear * (1.0 - data.behavior.fear_resistance.clamp(0.0, 1.0));
        let mut hero = HeroState::new(
            "knight".to_string(),
            1,
            data.stats.health,
            data.stats.mana,
            ROOM_AT,
            1.0,
            0,
        );
        hero.current_goal = HeroGoal::DestroyHeart;
        hero.health = hero.max_health * ((base + scared) / 2.0);

        assert!(
            !should_reconsider_goal(&hero, &game_data, 0.0),
            "under a lamp this wound should not break the knight"
        );
        assert!(
            should_reconsider_goal(&hero, &game_data, dark_fear),
            "the same wound in the dark should"
        );
    }
}
