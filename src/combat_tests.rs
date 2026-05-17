use crate::data::GameData;
use crate::engine::combat;
use crate::state::entities::{CreatureState, HeroState, Task};
use crate::state::game_state::GameState;
use crate::state::tile_state::TilePos;

#[test]
fn test_ranged_monster_spawns_projectile() {
    // 1. Setup Game
    // Ensure we can find assets.
    // Cargo test runs in workspace root usually.
    let game_data = GameData::load().expect("Failed to load game data");
    let mut game_state = GameState::new(20, 20, &game_data);

    // Clear all default entities (walls, etc.) to ensure a clean test environment
    // This prevents the Warlock from targeting a closer wall instead of the Hero
    game_state.entities = crate::state::entities::EntityManager::new();

    // Set all tiles to floor to ensure Line of Sight is not blocked by residual wall tiles
    let (w, h) = crate::engine::tile_grid::get_grid_dimensions(&game_state.dungeon.grid);
    for y in 0..h {
        for x in 0..w {
            let pos = TilePos::new(x as i32, y as i32);
            if let Some(tile) =
                crate::engine::tile_grid::get_tile_mut(&mut game_state.dungeon.grid, pos)
            {
                tile.tile_type = "floor".to_string();
            }
        }
    }

    // 2. Spawn Ranged Monster (Warlock)
    let warlock_pos = TilePos::new(5, 5);
    let mut warlock_state = CreatureState::new("warlock".to_string(), 1, 100.0, 100.0, 12345);
    // Ensure warlock is happy enough not to desert
    warlock_state.mood = 100.0;

    let warlock_id = game_state
        .entities
        .spawn_creature(warlock_pos, warlock_state);

    // 3. Spawn Hero target
    let hero_pos = TilePos::new(5, 8); // Within range (distance 3)
    let hero_state = HeroState::new("knight".to_string(), 1, 100.0, 50.0, hero_pos, 5.0, 54321);
    let hero_id = game_state.entities.spawn_hero(hero_pos, hero_state);

    // 4. Run update to trigger AI decision
    // Step 1: Decision phase
    game_state.update(0.1, &game_data);

    // Verify Warlock selected Attack task
    let warlock = game_state
        .entities
        .get(warlock_id)
        .unwrap()
        .as_creature()
        .unwrap();

    // Debug info
    println!(
        "Warlock (ID {}) Task: {:?}",
        warlock_id, warlock.current_task
    );
    println!("Hero ID: {}", hero_id);
    println!("Entities in game: {}", game_state.entities.count());
    for entity in game_state.entities.all() {
        println!(
            "Entity {}: Type={:?}, Pos={:?}",
            entity.id, entity.entity_type, entity.pos
        );
    }

    if let Some(Task::Attack(target_id)) = warlock.current_task.clone() {
        if target_id != hero_id {
            let target = game_state.entities.get(target_id).unwrap();
            println!(
                "WRONG TARGET: Targeted Entity {} ({:?}) at {:?}",
                target_id, target.entity_type, target.pos
            );
        }
        assert_eq!(target_id, hero_id, "Warlock should target the hero");
    } else {
        panic!(
            "Warlock did not select Attack task. Current: {:?}",
            warlock.current_task
        );
    }

    // Step 2: Combat phase. Resolve a deterministic full attack interval
    // instead of relying on probabilistic fixed-timestep combat.
    let attacker = game_state.entities.get(warlock_id).unwrap().clone();
    let defender = game_state.entities.get(hero_id).unwrap().clone();
    let result = combat::resolve_combat_tick(&attacker, &defender, 1.0, &game_data);

    if let Some((projectile_type, damage)) = result.projectile_spawned {
        game_state.projectiles.spawn(
            attacker.visual_pos,
            defender.visual_pos,
            &projectile_type,
            warlock_id,
            hero_id,
            damage,
        );
    } else {
        panic!("Ranged warlock attack should spawn a projectile");
    }

    // 5. Verify Projectile Spawned
    let projectiles = game_state.projectiles.active_projectiles();
    assert!(
        !projectiles.is_empty(),
        "Projectile should have been spawned"
    );

    let projectile = &projectiles[0];
    assert_eq!(projectile.attacker_id, warlock_id);
    assert_eq!(projectile.defender_id, hero_id);
    assert!(projectile.damage > 0.0, "Projectile should carry damage");

    println!(
        "Projectile spawned: Type={:?}, Damage={}",
        projectile.projectile_type, projectile.damage
    );

    // 6. Projectile Impact
    // Projectile duration for Magic is 0.4s. Advance only the projectile
    // manager so this assertion is not affected by a new combat tick.
    let impacts = game_state.projectiles.update(0.5);
    for impact in impacts {
        crate::engine::combat::apply_projectile_impact(
            &impact,
            &mut game_state.entities,
            &game_data,
            game_state.time_elapsed,
        );
    }

    // 7. Verify Damage
    let hero = game_state.entities.get(hero_id).unwrap().as_hero().unwrap();
    assert!(
        hero.health < 100.0,
        "Hero should have taken damage. Health: {}",
        hero.health
    );

    // 8. Verify Projectile Removed
    let projectiles_after = game_state.projectiles.active_projectiles();
    assert!(
        projectiles_after.is_empty(),
        "Projectile should be removed after impact"
    );
}
