use crate::data::monsters::MonsterData;
use crate::data::traits::TraitData;
use crate::data::GameData;
use crate::engine::combat;
use crate::engine::creature_ai::apply_slap;
use crate::engine::creature_ai::needs::update_needs;
use crate::state::entities::{CreatureState, EntityManager};
use crate::state::tile_state::TilePos;

#[test]
fn strong_trait_multiplies_creature_attack_in_combat_stats() {
    let game_data = GameData::load().expect("game data should load");
    let troll_data = game_data.monsters.get("troll").expect("troll should exist");
    assert!(troll_data.traits.contains(&"strong".to_string()));

    let mut entities = EntityManager::new();
    let creature = CreatureState::new("troll".to_string(), 1, 300.0, 0.0, 1);
    let creature_id = entities.spawn_creature(TilePos::new(0, 0), creature);
    let entity = entities.get(creature_id).unwrap();

    let stats = combat::extract_combat_stats(entity, &game_data);

    // level 1 -> level_multiplier is 1.0, so this isolates the "strong" trait's 1.15x.
    assert_eq!(stats.attack, troll_data.stats.attack * 1.15);
}

#[test]
fn undead_trait_zeroes_food_and_sleep_need_decay() {
    let game_data = GameData::load().expect("game data should load");
    let vampire_data = game_data
        .monsters
        .get("vampire")
        .expect("vampire should exist");
    assert!(vampire_data.traits.contains(&"undead".to_string()));
    // Sanity check: the raw authored decay rates are non-zero, so a real multiplier is needed
    // to zero them out (this isn't just "the data already says 0").
    assert!(vampire_data.needs["food"].decay_per_minute > 0.0);
    assert!(vampire_data.needs["sleep"].decay_per_minute > 0.0);

    let mut creature = CreatureState::new("vampire".to_string(), 1, 150.0, 100.0, 1);
    creature.set_need("food".to_string(), 80.0);
    creature.set_need("sleep".to_string(), 80.0);
    creature.set_need("gold".to_string(), 80.0);

    update_needs(&mut creature, 60.0, vampire_data, &game_data);

    assert_eq!(
        creature.get_need("food"),
        80.0,
        "undead trait should zero out food decay"
    );
    assert_eq!(
        creature.get_need("sleep"),
        80.0,
        "undead trait should zero out sleep decay"
    );
    // Gold isn't covered by the undead trait's need_decay_multipliers, so it should still decay.
    assert!(
        creature.get_need("gold") < 80.0,
        "gold need should still decay normally"
    );
}

#[test]
fn discipline_response_multiplier_dampens_slap_mood_swing() {
    let mut game_data = GameData::load().expect("game data should load");
    game_data.traits.insert(
        "test_mindless".to_string(),
        TraitData {
            id: "test_mindless".to_string(),
            discipline_response_multiplier: 0.2,
            ..Default::default()
        },
    );

    let mut monster_data: MonsterData = serde_json::from_str(
        r#"{
            "id": "test_creature",
            "name": "Test",
            "description": "Test creature",
            "faction": "dungeon",
            "role": "worker",
            "stats": { "health": 100, "mana": 0, "attack": 5, "defense": 2, "speed": 1.0, "carry_capacity": 10, "sight_radius": 5 },
            "needs": {},
            "traits": ["test_mindless"],
            "ai": { "base_mood": 70, "anger_threshold": 30, "desertion_threshold": 20, "task_preferences": {}, "room_desires": {}, "discipline_response": { "slap": -20 } },
            "combat": { "attack_type": "melee", "damage_range": [3, 6], "attack_speed": 1.0, "armor_type": "none", "resistances": {}, "abilities": [] },
            "progression": { "xp_to_level": [0, 100], "stat_growth_per_level": {}, "max_level": 2, "mutations": [] },
            "economy": { "wage_per_minute": 1, "steals_if_unpaid": false, "drops_gold_on_death": [5, 10] },
            "spawn": { "source": "portal", "min_dungeon_reputation": 0, "preferred_rooms": [], "spawn_weight": 1.0, "max_population": 10 },
            "visual": { "sprite": "test", "scale": 1.0, "animations": [], "voice_set": "test" }
        }"#,
    )
    .unwrap();
    monster_data.id = "test_creature".to_string();

    let mut creature = CreatureState::new("test_creature".to_string(), 1, 100.0, 0.0, 1);
    creature.mood = 70.0;

    apply_slap(&mut creature, &monster_data, &game_data, 100.0);

    // Raw slap would be -20; a 0.2 discipline_response_multiplier should shrink it to -4.
    assert_eq!(creature.mood, 66.0);
}
