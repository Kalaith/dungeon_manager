use crate::data::GameData;
use crate::state::dungeon::Dungeon;
use crate::state::entities::{EntityManager, HeroState, CreatureState};
use crate::state::room_manager::RoomManager;
use crate::state::tile_state::{Ownership, TilePos};
use crate::engine::pathfinding::{find_path, Heuristic, PathfindingGrid, Pos};
use crate::engine::tile_types::types as tt;

pub struct SpawnSystem;

impl SpawnSystem {
    pub fn spawn_random_hero(
        room_manager: &RoomManager,
        entities: &mut EntityManager,
        game_data: &GameData
    ) {
        // Find the hero entrance room to spawn from
        let spawn_pos = if let Some(hero_entrance) = room_manager.rooms.iter().find(|r| r.room_type == tt::HERO_ENTRANCE) {
            // Pick a random tile in the hero entrance
            let tiles: Vec<&TilePos> = hero_entrance.tiles.iter().collect();
            if tiles.is_empty() {
                return; // No tiles to spawn from
            }
            *tiles[rand::random::<usize>() % tiles.len()]
        } else {
            eprintln!("Warning: No hero entrance found, cannot spawn heroes");
            return;
        };

        // Pick a random hero type
        let hero_ids: Vec<&String> = game_data.heroes.keys().collect();
        if let Some(&hero_id) = hero_ids.get(rand::random::<usize>() % hero_ids.len()) {
            if let Some(hero_data) = game_data.heroes.get(hero_id) {
                let hero_state = HeroState::new(
                    hero_id.clone(),
                    1, // level
                    hero_data.stats.health, // max_health
                    hero_data.stats.mana, // max_mana
                );
                entities.spawn_hero(spawn_pos, hero_state);
                eprintln!("Spawned hero {} at hero entrance {:?}", hero_id, spawn_pos);
            }
        }
    }

    pub fn spawn_random_creature(
        dungeon: &mut Dungeon,
        room_manager: &RoomManager,
        entities: &mut EntityManager,
        game_data: &GameData
    ) {
        // Find the dungeon heart position
        let heart_pos = dungeon.find_dungeon_heart();
        if heart_pos.is_none() {
            eprintln!("Warning: No dungeon heart found, cannot spawn creatures");
            return;
        }
        let heart_pos = heart_pos.unwrap();

        // Find all monster spawner tiles
        let mut spawner_tiles = Vec::new();
        for row in &dungeon.grid {
            for tile in row {
                if tile.tile_type == tt::MONSTER_SPAWNER && tile.ownership == Ownership::Player {
                    spawner_tiles.push(tile.pos);
                }
            }
        }

        if spawner_tiles.is_empty() {
            eprintln!("Warning: No monster spawners found, cannot spawn creatures");
            return;
        }

        // Create pathfinding grid to check connectivity
        let mut pf_grid = PathfindingGrid::new(dungeon.width, dungeon.height);
        for y in 0..dungeon.height {
            for x in 0..dungeon.width {
                let tile_pos = TilePos::new(x as i32, y as i32);
                if let Some(tile) = dungeon.get_tile(tile_pos) {
                    // Walkable if owned by player
                    let walkable = tile.ownership == Ownership::Player;
                    let pf_pos = Pos::new(x as i32, y as i32);
                    pf_grid.set_walkable(pf_pos, walkable);
                }
            }
        }

        // Find spawners that are connected to the dungeon heart
        let mut connected_spawners = Vec::new();
        for &spawner_pos in &spawner_tiles {
            let start = Pos::new(spawner_pos.x, spawner_pos.y);
            let goal = Pos::new(heart_pos.x, heart_pos.y);

            if find_path(start, goal, &pf_grid, Heuristic::Manhattan, false).is_some() {
                connected_spawners.push(spawner_pos);
            }
        }

        if connected_spawners.is_empty() {
            eprintln!("Warning: No monster spawners connected to dungeon heart");
            return;
        }

        // Check if we have available lair space
        let available_lair_tiles = room_manager.count_available_lair_tiles(dungeon);
        if available_lair_tiles == 0 {
            eprintln!("Warning: No available lair space, cannot spawn creature");
            return;
        }

        // Pick a random connected spawner
        let spawn_pos = connected_spawners[rand::random::<usize>() % connected_spawners.len()];

        // Pick a random creature type (exclude imps - they spawn differently)
        let creature_ids: Vec<&String> = game_data.monsters.keys()
            .filter(|id| *id != "imp")
            .collect();

        if creature_ids.is_empty() {
            return;
        }

        if let Some(&creature_id) = creature_ids.get(rand::random::<usize>() % creature_ids.len()) {
            if let Some(monster_data) = game_data.monsters.get(creature_id) {
                // Create creature with food need initialized
                let mut creature_state = CreatureState::new(
                    creature_id.clone(),
                    1,
                    monster_data.stats.health,
                    monster_data.stats.mana,
                );
                
                // Initialize food need
                creature_state.set_need("food".to_string(), 100.0);
                
                let entity_id = entities.spawn_creature(spawn_pos, creature_state);
                eprintln!("Spawned creature {} at spawner {:?}", creature_id, spawn_pos);
                
                // Claim a lair tile for this creature
                room_manager.find_and_claim_lair_tile(dungeon, entity_id);
            }
        }
    }
}
