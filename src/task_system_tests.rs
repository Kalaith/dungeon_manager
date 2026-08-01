//! Tests for task execution.
//!
//! Lifted out of `task_system.rs` when that file reached 801 lines against the
//! 800-line hard limit. They only ever exercised the public `execute_task`, so
//! nothing had to be widened to move them.

use crate::data::GameData;
use crate::engine::room_validator::Room;
use crate::engine::task_system::*;
use crate::state::entities::{CreatureState, EntityManager};
use crate::state::entities::{EntityId, Task};
use crate::state::player_state::PlayerState;
use crate::state::room_manager::RoomManager;
use crate::state::tile_state::TilePos;
use std::collections::HashSet;

#[test]
fn workshop_work_task_manufactures_lowest_stock_unlocked_trap() {
    let game_data = GameData::load().expect("game data should load");
    let mut entities = EntityManager::new();
    let monster_data = game_data.monsters.get("goblin").unwrap();
    let mut creature = CreatureState::new(
        "goblin".to_string(),
        1,
        monster_data.stats.health,
        monster_data.stats.mana,
        1,
    );
    creature.current_task = Some(Task::Work(42, TilePos::new(2, 2)));
    let creature_id = entities.spawn_creature(TilePos::new(2, 2), creature);

    let mut room_manager = RoomManager::new();
    let mut room = Room::new(
        42,
        "workshop".to_string(),
        [TilePos::new(2, 2)].into_iter().collect::<HashSet<_>>(),
        Vec::new(),
    );
    room.active = true;
    room_manager.rooms.push(room);

    let mut player = PlayerState::new(&game_data);
    player.unlock_trap("spike_trap".to_string());
    player.add_trap_inventory("door".to_string(), 2);

    let result = execute_task(
        creature_id,
        &mut entities,
        &room_manager,
        &player,
        &game_data,
        game_data.config.task_execution.work_timer_threshold * 2.0,
    );

    assert_eq!(result.manufactured_trap, Some("spike_trap".to_string()));
    assert_eq!(result.materials_change, 0.0);
}

/// A single active room of `room_type` with one creature standing in it,
/// tasked to research there.
fn research_fixture(
    game_data: &GameData,
    room_type: &str,
) -> (EntityId, EntityManager, RoomManager) {
    let mut entities = EntityManager::new();
    let monster_data = game_data.monsters.get("warlock").unwrap();
    let mut creature = CreatureState::new(
        "warlock".to_string(),
        1,
        monster_data.stats.health,
        monster_data.stats.mana,
        1,
    );
    creature.current_task = Some(Task::Research(7));
    let creature_id = entities.spawn_creature(TilePos::new(2, 2), creature);

    let mut room_manager = RoomManager::new();
    let mut room = Room::new(
        7,
        room_type.to_string(),
        [TilePos::new(2, 2)].into_iter().collect::<HashSet<_>>(),
        Vec::new(),
    );
    room.active = true;
    room_manager.rooms.push(room);

    (creature_id, entities, room_manager)
}

fn research_produced(game_data: &GameData, room_type: &str) -> f32 {
    let (creature_id, mut entities, room_manager) = research_fixture(game_data, room_type);
    let player = PlayerState::new(game_data);
    execute_task(
        creature_id,
        &mut entities,
        &room_manager,
        &player,
        game_data,
        1.0,
    )
    .research_change
}

#[test]
fn any_room_in_the_research_family_produces_research() {
    // The point of the generalization: research used to mean
    // `room_type == "library"`, so a second research room produced nothing
    // no matter how it was staffed.
    let game_data = GameData::load().expect("game data should load");

    assert!(research_produced(&game_data, "library") > 0.0);
    assert!(research_produced(&game_data, "arcane_archive") > 0.0);
}

#[test]
fn a_room_outside_the_research_family_produces_none() {
    let game_data = GameData::load().expect("game data should load");
    assert_eq!(research_produced(&game_data, "treasury"), 0.0);
}

#[test]
fn room_research_rate_scales_output() {
    // `rooms.json` authored the library's `research_rate: 1.0` against a
    // struct field named `research_per_minute`, so serde dropped it and
    // every research room ran at the flat global rate. This is the check
    // that the authored number reaches the engine.
    let game_data = GameData::load().expect("game data should load");

    let library = research_produced(&game_data, "library");
    let archive = research_produced(&game_data, "arcane_archive");

    let library_rate = game_data.rooms["library"].effects.research_rate;
    let archive_rate = game_data.rooms["arcane_archive"].effects.research_rate;
    assert!(
        archive_rate > library_rate,
        "archive should out-research a library"
    );

    // Same creature, same dt — the only difference is the room's rate.
    let expected = library * (archive_rate / library_rate);
    assert!(
        (archive - expected).abs() < 1e-4,
        "archive produced {archive}, expected {expected}"
    );
}

/// XP a level-1 goblin gains from one training tick in `room_type`.
fn xp_from_one_training_tick(game_data: &GameData, room_type: &str) -> f32 {
    let mut entities = EntityManager::new();
    let monster_data = game_data.monsters.get("goblin").unwrap();
    let mut creature = CreatureState::new(
        "goblin".to_string(),
        1,
        monster_data.stats.health,
        monster_data.stats.mana,
        1,
    );
    creature.current_task = Some(Task::Train(9));
    // High enough that a single level-up cannot reset experience to 0 and
    // hide the difference between rooms.
    creature.max_experience = 10_000.0;
    let creature_id = entities.spawn_creature(TilePos::new(2, 2), creature);

    let mut room_manager = RoomManager::new();
    let mut room = Room::new(
        9,
        room_type.to_string(),
        [TilePos::new(2, 2)].into_iter().collect::<HashSet<_>>(),
        Vec::new(),
    );
    room.active = true;
    room_manager.rooms.push(room);

    let player = PlayerState::new(game_data);
    execute_task(
        creature_id,
        &mut entities,
        &room_manager,
        &player,
        game_data,
        game_data.config.task_execution.training_timer_threshold * 2.0,
    );

    entities
        .get(creature_id)
        .and_then(|e| e.as_creature())
        .map(|c| c.experience)
        .unwrap_or(0.0)
}

#[test]
fn any_room_in_the_train_family_grants_experience() {
    let game_data = GameData::load().expect("game data should load");

    assert!(xp_from_one_training_tick(&game_data, "training_room") > 0.0);
    assert!(xp_from_one_training_tick(&game_data, "combat_pit") > 0.0);
}

#[test]
fn a_room_outside_the_train_family_grants_none() {
    let game_data = GameData::load().expect("game data should load");
    assert_eq!(xp_from_one_training_tick(&game_data, "library"), 0.0);
}

#[test]
fn room_training_rate_scales_experience() {
    let game_data = GameData::load().expect("game data should load");

    let hall = xp_from_one_training_tick(&game_data, "training_room");
    let pit = xp_from_one_training_tick(&game_data, "combat_pit");

    let hall_rate = game_data.rooms["training_hall"].effects.training_rate;
    let pit_rate = game_data.rooms["combat_pit"].effects.training_rate;
    assert!(pit_rate > hall_rate, "the pit should out-train the hall");

    let expected = hall * (pit_rate / hall_rate);
    assert!(
        (pit - expected).abs() < 1e-4,
        "pit granted {pit}, expected {expected}"
    );
}

#[test]
fn unpaid_theft_prone_creature_loses_gold_satisfaction_faster_than_docile_one() {
    let game_data = GameData::load().expect("game data should load");

    let mut room_manager = RoomManager::new();
    let mut room = Room::new(
        42,
        "treasury".to_string(),
        [TilePos::new(2, 2)].into_iter().collect::<HashSet<_>>(),
        Vec::new(),
    );
    room.active = true;
    room_manager.rooms.push(room);

    let mut player = PlayerState::new(&game_data);
    player.gold = 0; // treasury is empty: nobody gets paid this tick

    // goblin has economy.steals_if_unpaid = true, imp has it = false
    let goblin_data = game_data.monsters.get("goblin").unwrap();
    assert!(goblin_data.economy.steals_if_unpaid);
    let imp_data = game_data.monsters.get("imp").unwrap();
    assert!(!imp_data.economy.steals_if_unpaid);

    let mut goblin_entities = EntityManager::new();
    let mut goblin = CreatureState::new(
        "goblin".to_string(),
        1,
        goblin_data.stats.health,
        goblin_data.stats.mana,
        1,
    );
    goblin.current_task = Some(Task::CollectWages(42));
    goblin.set_need("gold".to_string(), 50.0);
    let goblin_id = goblin_entities.spawn_creature(TilePos::new(2, 2), goblin);

    let mut imp_entities = EntityManager::new();
    let mut imp = CreatureState::new(
        "imp".to_string(),
        1,
        imp_data.stats.health,
        imp_data.stats.mana,
        1,
    );
    imp.current_task = Some(Task::CollectWages(42));
    imp.set_need("gold".to_string(), 50.0);
    let imp_id = imp_entities.spawn_creature(TilePos::new(2, 2), imp);

    let goblin_result = execute_task(
        goblin_id,
        &mut goblin_entities,
        &room_manager,
        &player,
        &game_data,
        1.0,
    );
    let imp_result = execute_task(
        imp_id,
        &mut imp_entities,
        &room_manager,
        &player,
        &game_data,
        1.0,
    );

    // Nobody actually got paid (treasury is empty)
    assert_eq!(goblin_result.gold_change, 0.0);
    assert_eq!(imp_result.gold_change, 0.0);

    let goblin_gold_need = goblin_entities
        .get(goblin_id)
        .and_then(|e| e.as_creature())
        .unwrap()
        .get_need("gold");
    let imp_gold_need = imp_entities
        .get(imp_id)
        .and_then(|e| e.as_creature())
        .unwrap()
        .get_need("gold");

    assert!(
        goblin_gold_need < 50.0,
        "unpaid creature should lose gold satisfaction"
    );
    assert!(
        imp_gold_need < 50.0,
        "unpaid creature should lose gold satisfaction"
    );
    assert!(
        goblin_gold_need < imp_gold_need,
        "theft-prone creature should resent going unpaid more than a docile one"
    );
}
