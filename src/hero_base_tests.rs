//! Tests for hero-base destruction consequences.
//!
//! `hero_buildings.json` has authored a `destruction_effect` for every building
//! since it shipped, and nothing applied any of them: the town hall's
//! `win_game` did not win, and razing the armory did not blunt a single hero.

use crate::data::hero_buildings::DestructionEffect;
use crate::data::GameData;
use crate::state::entities::CreatureState;
use crate::state::hero_base::HeroBase;
use crate::state::tile_state::TilePos;

fn hero_base(game_data: &GameData) -> HeroBase {
    let mut base = HeroBase::new(game_data);
    base.enabled = true;
    base
}

#[test]
fn razing_the_town_hall_defeats_the_hero_base() {
    let game_data = GameData::load().expect("game data should load");
    let mut base = hero_base(&game_data);
    let entities = crate::state::entities::EntityManager::new();

    // A base with a standing building is not defeated...
    base.buildings.push(crate::state::hero_base::HeroBuilding {
        id: "town_hall_0".to_string(),
        building_type: "town_hall".to_string(),
        pos: TilePos::new(1, 1),
        spawn_timers: Vec::new(),
        entity_id: None,
    });
    base.decisive_building_destroyed = false;

    let town_hall = &game_data.hero_buildings["town_hall"];
    assert!(matches!(
        town_hall.destruction_effect,
        DestructionEffect::WinGame
    ));

    base.apply_destruction_effect(&town_hall.destruction_effect);
    assert!(base.is_defeated(&entities), "the town hall should end it");
}

#[test]
fn razed_spawn_buildings_slow_the_garrison_permanently() {
    let game_data = GameData::load().expect("game data should load");
    let mut base = hero_base(&game_data);
    assert_eq!(base.spawn_interval_multiplier(), 1.0);

    base.apply_destruction_effect(&game_data.hero_buildings["barracks"].destruction_effect);
    let after_one = base.spawn_interval_multiplier();
    assert!(after_one > 1.0, "spawning should slow, got {after_one}");

    // Effects stack: a second razed spawn building slows it further.
    base.apply_destruction_effect(&game_data.hero_buildings["tavern"].destruction_effect);
    assert!(base.spawn_interval_multiplier() > after_one);
}

#[test]
fn spawn_and_speed_penalties_stay_bounded() {
    // Razing everything must not divide by zero or freeze the hero faction
    // into immortality.
    let game_data = GameData::load().expect("game data should load");
    let mut base = hero_base(&game_data);

    for _ in 0..20 {
        base.apply_destruction_effect(&DestructionEffect::ReduceSpawnRate { percent: 50 });
        base.apply_destruction_effect(&DestructionEffect::ReduceHeroSpeed { percent: 50 });
    }

    assert!(base.spawn_interval_multiplier().is_finite());
    assert!(base.spawn_interval_multiplier() <= 10.0);
    assert!(base.hero_speed_multiplier() >= 0.1);
}

#[test]
fn razing_the_armory_blunts_heroes_but_not_creatures() {
    let game_data = GameData::load().expect("game data should load");
    let mut base = hero_base(&game_data);
    base.apply_destruction_effect(&game_data.hero_buildings["armory"].destruction_effect);
    assert!(base.hero_attack_penalty > 0.0);

    let mut entities = crate::state::entities::EntityManager::new();
    let knight_data = game_data.heroes.get("knight").expect("knight");
    let knight = crate::state::entities::HeroState::new(
        "knight".to_string(),
        1,
        knight_data.stats.health,
        knight_data.stats.mana,
        TilePos::new(1, 1),
        1.0,
        0,
    );
    let hero_id = entities.spawn_hero(TilePos::new(1, 1), knight);

    let goblin_data = game_data.monsters.get("goblin").expect("goblin");
    let goblin = CreatureState::new(
        "goblin".to_string(),
        1,
        goblin_data.stats.health,
        goblin_data.stats.mana,
        1,
    );
    let creature_id = entities.spawn_creature(TilePos::new(1, 2), goblin);

    let hero = entities.get(hero_id).unwrap();
    let mut hero_stats = crate::engine::combat::extract_combat_stats(hero, &game_data);
    let hero_attack_before = hero_stats.attack;
    crate::engine::combat::apply_hero_supply_penalty(
        hero,
        &mut hero_stats,
        base.hero_attack_penalty,
        base.hero_defense_penalty,
    );
    assert_eq!(
        hero_stats.attack,
        hero_attack_before - base.hero_attack_penalty
    );

    let creature = entities.get(creature_id).unwrap();
    let mut creature_stats = crate::engine::combat::extract_combat_stats(creature, &game_data);
    let creature_attack_before = creature_stats.attack;
    crate::engine::combat::apply_hero_supply_penalty(
        creature,
        &mut creature_stats,
        base.hero_attack_penalty,
        base.hero_defense_penalty,
    );
    assert_eq!(
        creature_stats.attack, creature_attack_before,
        "a creature must not lose stats to the heroes' burnt armory"
    );
}

#[test]
fn every_hero_building_effect_changes_something() {
    // Each of the twelve authored effects has to move some part of the base's
    // state, or it is decorative again.
    let game_data = GameData::load().expect("game data should load");

    for (id, data) in &game_data.hero_buildings {
        let mut base = hero_base(&game_data);
        base.apply_destruction_effect(&data.destruction_effect);

        let changed = base.decisive_building_destroyed
            || base.spawn_rate_penalty_percent > 0.0
            || base.hero_speed_penalty_percent > 0.0
            || base.hero_attack_penalty > 0.0
            || base.hero_defense_penalty > 0.0
            // `open_path` is realised by the tile becoming floor, not by state.
            || matches!(data.destruction_effect, DestructionEffect::OpenPath);

        assert!(changed, "destroying `{id}` changed nothing");
    }
}

#[test]
fn every_destruction_effect_can_explain_itself() {
    // The notification used to say only "Armory destroyed", so the effect
    // landed invisibly. Each authored effect must now have something to tell
    // the player, except `open_path` whose result is the visible floor.
    let game_data = GameData::load().expect("game data should load");

    let mut silent = Vec::new();
    for (id, data) in &game_data.hero_buildings {
        let described = data.destruction_effect.describe();
        match data.destruction_effect {
            DestructionEffect::OpenPath => assert!(
                described.is_none(),
                "`{id}` opens a path; the floor says it better than text"
            ),
            _ => {
                if described.is_none() {
                    silent.push(id.clone());
                }
            }
        }
    }

    assert!(
        silent.is_empty(),
        "these buildings would be razed with no explanation: {silent:?}"
    );
}

#[test]
fn the_description_names_the_number_the_player_earned() {
    // A vague "heroes are weaker now" would be worse than nothing; the point
    // is that the keeper can see what the demolition bought.
    let game_data = GameData::load().expect("game data should load");

    let armory = game_data.hero_buildings["armory"]
        .destruction_effect
        .describe()
        .expect("the armory should explain itself");
    let DestructionEffect::ReduceHeroStats { attack, defense } =
        game_data.hero_buildings["armory"].destruction_effect
    else {
        panic!("fixture assumption: the armory reduces hero stats");
    };
    assert!(
        armory.contains(&attack.to_string()) && armory.contains(&defense.to_string()),
        "`{armory}` should quote the authored numbers {attack}/{defense}"
    );

    let barracks = game_data.hero_buildings["barracks"]
        .destruction_effect
        .describe()
        .expect("the barracks should explain itself");
    let DestructionEffect::ReduceSpawnRate { percent } =
        game_data.hero_buildings["barracks"].destruction_effect
    else {
        panic!("fixture assumption: the barracks reduces spawn rate");
    };
    assert!(
        barracks.contains(&percent.to_string()),
        "`{barracks}` should quote the authored {percent}%"
    );
}
