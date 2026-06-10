use crate::data::GameData;
use crate::engine::pathfinding::{find_path, Heuristic, PathfindingGrid, Pos};
use crate::engine::tile_types::types as tt;
use crate::state::dungeon::Dungeon;
use crate::state::entities::{CreatureState, EntityManager};
use crate::state::room_manager::RoomManager;
use crate::state::tile_state::{Ownership, TilePos};

pub struct SpawnSystem;

impl SpawnSystem {
    pub fn spawn_random_creature(
        dungeon: &mut Dungeon,
        room_manager: &RoomManager,
        entities: &mut EntityManager,
        game_data: &GameData,
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
        let spawn_pos =
            connected_spawners[macroquad_toolkit::rng::gen_range(0, connected_spawners.len())];

        // Determine potential creatures based on existing rooms
        let mut potential_creatures = Vec::new();

        // Goblins are always available if we have lair space (checked above)
        potential_creatures.push("goblin");

        // Check for specific rooms to unlock other creatures
        let has_workshop = room_manager.rooms.iter().any(|r| r.room_type == "workshop");
        let has_library = room_manager.rooms.iter().any(|r| r.room_type == "library");

        if has_workshop {
            potential_creatures.push("orc");
            potential_creatures.push("troll");
        }

        if has_library {
            potential_creatures.push("warlock");
        }

        // Filter out creature IDs that don't exist in game_data
        let valid_creatures: Vec<&str> = potential_creatures
            .into_iter()
            .filter(|id| game_data.monsters.contains_key(*id))
            .collect();

        if valid_creatures.is_empty() {
            eprintln!("Warning: No valid creatures to spawn based on current rooms");
            return;
        }

        if let Some(&creature_id) =
            valid_creatures.get(macroquad_toolkit::rng::gen_range(0, valid_creatures.len()))
        {
            if let Some(monster_data) = game_data.monsters.get(creature_id) {
                // Generate visual variation seed
                let visual_seed = macroquad_toolkit::rng::random_u64();
                // Create creature with food need initialized
                let mut creature_state = CreatureState::new(
                    creature_id.to_string(),
                    1,
                    monster_data.stats.health,
                    monster_data.stats.mana,
                    visual_seed,
                );

                // Initialize food need
                creature_state.set_need("food".to_string(), 100.0);

                let entity_id = entities.spawn_creature(spawn_pos, creature_state);
                eprintln!(
                    "Spawned creature {} at spawner {:?}",
                    creature_id, spawn_pos
                );

                // Claim a lair tile for this creature
                room_manager.find_and_claim_lair_tile(dungeon, entity_id);
            }
        }
    }
}
