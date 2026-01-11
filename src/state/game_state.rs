//! Main game state container
//! Holds the dungeon grid, entities, rooms, and player state

use crate::data::GameData;
use crate::engine::combat::{find_combat_targets, resolve_combat_tick, update_status_effects};
use crate::engine::hero_ai::update_hero_ai;
use crate::engine::room_validator::Room;
use crate::engine::tile_grid::{self, Grid};
use crate::state::entities::{EntityId, EntityManager, HeroState};
use crate::state::player_state::PlayerState;
use crate::state::tile_state::{self, Ownership, TilePos};
use std::collections::HashSet;

pub struct GameState {
    pub grid: Grid,
    pub width: usize,
    pub height: usize,
    pub time_elapsed: f32,
    pub tick_accumulator: f32,
    pub camera_x: f32,
    pub camera_y: f32,
    pub zoom: f32,
    pub rooms: Vec<Room>,
    pub entities: EntityManager,
    pub player: PlayerState,
    pub next_room_id: usize,
    pub next_hero_spawn_time: f32,
    pub next_creature_spawn_time: f32,
}

impl GameState {
    pub fn new(width: usize, height: usize, game_data: &GameData) -> Self {
        let mut grid = tile_grid::create_grid(width, height, game_data);
        let mut rooms = Vec::new();
        let mut next_room_id = 0;

        // Create player's starting area in the center
        let center_x = width / 2;
        let center_y = height / 2;
        let start_size = 5;

        let mut start_tiles = std::collections::HashSet::new();
        for x in (center_x - start_size)..=(center_x + start_size) {
            for y in (center_y - start_size)..=(center_y + start_size) {
                if x >= 0 && x < width && y >= 0 && y < height {
                    start_tiles.insert(TilePos::new(x as i32, y as i32));
                }
            }
        }

        // Convert starting tiles to floor tiles
        for &pos in &start_tiles {
            if let Some(tile) = tile_grid::get_tile_mut(&mut grid, pos) {
                tile.tile_type = "claimed_floor".to_string();
                tile.ownership = Ownership::Player;
                tile.room_id = Some(next_room_id);
            }
        }

        rooms.push(crate::engine::room_validator::Room {
            id: next_room_id,
            room_type: "entrance".to_string(),
            tiles: start_tiles.clone(),
            quality: 1.0,
            active: true,
        });
        next_room_id += 1;

        // Place a dungeon heart in the center
        let heart_pos = TilePos::new(center_x as i32, center_y as i32);
        if let Some(tile) = tile_grid::get_tile_mut(&mut grid, heart_pos) {
            tile.tile_type = "dungeon_heart".to_string();
            tile.ownership = Ownership::Player;
        }

        // Create hero entrance at the edge of the map (top-left corner)
        let hero_entrance_x = 5;
        let hero_entrance_y = 5;
        let hero_entrance_size = 3;

        let mut hero_entrance_tiles = std::collections::HashSet::new();
        for x in (hero_entrance_x - hero_entrance_size)..=(hero_entrance_x + hero_entrance_size) {
            for y in (hero_entrance_y - hero_entrance_size)..=(hero_entrance_y + hero_entrance_size) {
                if x >= 0 && x < width && y >= 0 && y < height {
                    hero_entrance_tiles.insert(TilePos::new(x as i32, y as i32));
                }
            }
        }

        // Convert hero entrance tiles to unclaimed floor (neutral zone)
        for &pos in &hero_entrance_tiles {
            if let Some(tile) = tile_grid::get_tile_mut(&mut grid, pos) {
                tile.tile_type = "stone_path".to_string();
                tile.ownership = Ownership::Unclaimed;
                tile.room_id = Some(next_room_id);
            }
        }

        rooms.push(crate::engine::room_validator::Room {
            id: next_room_id,
            room_type: "hero_entrance".to_string(),
            tiles: hero_entrance_tiles.clone(),
            quality: 1.0,
            active: true,
        });
        next_room_id += 1;

        let mut entities = EntityManager::new();

        // Don't spawn initial creatures - let them spawn from spawners
        // Players must build spawners and have lair space first

        Self {
            grid,
            width,
            height,
            time_elapsed: 0.0,
            tick_accumulator: 0.0,
            camera_x: 400.0,
            camera_y: 200.0,
            zoom: 1.0,
            rooms,
            entities,
            player: PlayerState::new(),
            next_room_id,
            next_hero_spawn_time: 5.0, // Spawn first hero after 5 seconds
            next_creature_spawn_time: 10.0, // Spawn first creature after 10 seconds
        }
    }

    pub fn update(&mut self, dt: f32, game_data: &GameData) {
        self.time_elapsed += dt;
        self.tick_accumulator += dt;

        // Fixed timestep simulation (10 ticks per second)
        const TICK_RATE: f32 = 0.1;
        while self.tick_accumulator >= TICK_RATE {
            self.tick(game_data, TICK_RATE);
            self.tick_accumulator -= TICK_RATE;
        }
    }

    fn tick(&mut self, game_data: &GameData, dt: f32) {
        // Imp auto-spawning when tiles are marked for digging
        if self.has_marked_tiles() {
            let imp_count = self.count_imps();
            if imp_count < 5 {
                // Spawn an imp if we don't have enough
                self.spawn_imp(game_data);
            }
        }

        // Update imps first (they dig tiles)
        self.update_imp_digging(game_data, dt);

        // Update creature AI and movement
        self.update_creature_ai_and_movement(game_data, dt);

        // Hero spawning
        self.next_hero_spawn_time -= dt;
        if self.next_hero_spawn_time <= 0.0 {
            self.spawn_random_hero(game_data);
            self.next_hero_spawn_time = 10.0 + rand::random::<f32>() * 10.0; // 10-20 seconds
        }

        // Creature spawning
        self.next_creature_spawn_time -= dt;
        if self.next_creature_spawn_time <= 0.0 {
            self.spawn_random_creature(game_data);
            self.next_creature_spawn_time = 15.0 + rand::random::<f32>() * 15.0; // 15-30 seconds
        }

        // Update hero AI
        let hero_entities: Vec<EntityId> = self.entities.heroes().map(|(id, _)| id).collect();
        for hero_id in hero_entities {
            if let Some(entity) = self.entities.get(hero_id) {
                if let Some(hero_state) = entity.as_hero() {
                    let mut hero_state_clone = hero_state.clone();
                    update_hero_ai(entity, &mut hero_state_clone, self, game_data, dt);
                    
                    // Update the entity with the new hero state
                    if let Some(entity_mut) = self.entities.get_mut(hero_id) {
                        if let Some(hero_state_mut) = entity_mut.as_hero_mut() {
                            *hero_state_mut = hero_state_clone;
                        }
                    }
                }
            }
        }

        // Resolve combat
        self.resolve_combat(game_data, dt);

        // Update status effects for all entities
        let all_entity_ids: Vec<EntityId> = self.entities.all().map(|e| e.id).collect();
        for entity_id in all_entity_ids {
            if let Some(entity) = self.entities.get_mut(entity_id) {
                update_status_effects(entity, dt);
            }
        }

        // Generate food from hatcheries
        self.generate_food_from_hatcheries(dt);

        // Check for starving creatures that need to desert
        self.handle_creature_desertion(game_data);

        // Remove dead entities and release their lair space
        let dead_ids: Vec<EntityId> = self.entities.all()
            .filter(|e| !e.is_alive())
            .map(|e| e.id)
            .collect();

        for entity_id in dead_ids {
            self.release_lair_tile(entity_id);
            self.entities.remove(entity_id);
        }
    }

    pub fn get_tile(&self, pos: TilePos) -> Option<&crate::state::tile_state::TileState> {
        tile_grid::get_tile(&self.grid, pos)
    }

    pub fn get_tile_mut(&mut self, pos: TilePos) -> Option<&mut crate::state::tile_state::TileState> {
        tile_grid::get_tile_mut(&mut self.grid, pos)
    }

    fn update_creature_ai_and_movement(&mut self, game_data: &GameData, dt: f32) {
        use crate::engine::creature_ai;
        use crate::engine::pathfinding::{find_path, Heuristic, PathfindingGrid, Pos};

        let creature_ids: Vec<EntityId> = self.entities.creatures()
            .filter(|(_, c)| c.creature_id != "imp") // Imps have their own AI (digging)
            .map(|(id, _)| id)
            .collect();

        for creature_id in creature_ids {
            // Get current position and state first
            let (current_pos, needs_new_task, needs_path, task_target) = {
                let entity = match self.entities.get(creature_id) {
                    Some(e) => e,
                    None => continue,
                };

                let creature = match entity.as_creature() {
                    Some(c) => c,
                    None => continue,
                };

                let needs_new_task = creature.current_path.is_none() && creature.current_task.is_none();
                let needs_path = creature.current_path.is_none() && creature.current_task.is_some();
                let task_target = creature.current_task.as_ref().and_then(|task| self.get_task_target_position(task));

                (entity.pos, needs_new_task, needs_path, task_target)
            };

            // Update needs and mood
            {
                let entity = self.entities.get_mut(creature_id).unwrap();
                let creature = entity.as_creature_mut().unwrap();

                if let Some(monster_data) = game_data.monsters.get(&creature.creature_id) {
                    creature_ai::update_needs(creature, dt, monster_data);
                    creature_ai::update_mood(creature, monster_data);

                    // Debug: Log creature needs periodically
                    if creature.move_timer == 0.0 {
                        eprintln!("[AI] Creature {} at {:?}: mood={:.1}, needs={:?}, task={:?}",
                            creature.creature_id, current_pos, creature.mood,
                            creature.needs, creature.current_task);
                    }
                }
            }

            // Handle movement along path
            let (should_move, next_waypoint) = {
                let entity = self.entities.get_mut(creature_id).unwrap();
                let creature = entity.as_creature_mut().unwrap();

                let mut should_move = false;
                let mut next_waypoint = None;

                if let Some(ref mut path) = creature.current_path {
                    if !path.is_empty() {
                        creature.move_timer += dt;
                        let move_interval = 1.0 / creature.movement_speed;

                        if creature.move_timer >= move_interval {
                            creature.move_timer = 0.0;
                            should_move = true;
                            next_waypoint = path.first().copied();
                        }
                    }
                }

                (should_move, next_waypoint)
            }; // Borrow ends here

            // Apply movement (now we can mutate entity)
            if should_move {
                if let Some(next_pos) = next_waypoint {
                    let entity = self.entities.get_mut(creature_id).unwrap();
                    entity.pos = next_pos;

                    let creature = entity.as_creature_mut().unwrap();
                    if let Some(ref mut path) = creature.current_path {
                        path.remove(0);
                        if path.is_empty() {
                            creature.current_path = None;
                        }
                    }
                }
            }

            // Decide new task if needed
            if needs_new_task {
                // First, get creature data immutably to pass to decide_task
                let new_task = {
                    let entity = self.entities.get(creature_id).unwrap();
                    let creature = entity.as_creature().unwrap();

                    let task = creature_ai::decide_task(
                        creature,
                        current_pos,
                        self,
                        game_data,
                    );

                    eprintln!("[AI] Creature {} decided new task: {:?}", creature.creature_id, task);
                    task
                }; // Immutable borrow ends here

                // Now apply the new task with a mutable borrow
                if let Some(entity) = self.entities.get_mut(creature_id) {
                    if let Some(creature) = entity.as_creature_mut() {
                        creature.current_task = new_task;
                    }
                }
            }

            // Pathfind to task if needed
            if needs_path {
                // Special case for Idle: pick a random wander position
                let target_pos = if task_target.is_none() {
                    // Check if this is an Idle task
                    let is_idle = {
                        let entity = self.entities.get(creature_id).unwrap();
                        let creature = entity.as_creature().unwrap();
                        matches!(creature.current_task, Some(crate::state::entities::Task::Idle))
                    };

                    if is_idle {
                        // Pick a random walkable tile within 5 tiles
                        let wander_radius = 5;
                        let mut attempts = 0;
                        let mut wander_pos = None;

                        while attempts < 10 && wander_pos.is_none() {
                            let dx = rand::random::<i32>() % (wander_radius * 2 + 1) - wander_radius;
                            let dy = rand::random::<i32>() % (wander_radius * 2 + 1) - wander_radius;
                            let candidate = TilePos::new(current_pos.x + dx, current_pos.y + dy);

                            if let Some(tile) = self.get_tile(candidate) {
                                // Only wander on claimed/room tiles
                                let is_walkable = matches!(
                                    tile.tile_type.as_str(),
                                    "claimed_floor" | "lair" | "hatchery" | "treasury"
                                    | "training_room" | "library" | "workshop"
                                    | "dungeon_heart" | "monster_spawner"
                                );
                                if is_walkable {
                                    wander_pos = Some(candidate);
                                }
                            }
                            attempts += 1;
                        }

                        eprintln!("[AI] Idle creature wandering to {:?}", wander_pos);
                        wander_pos
                    } else {
                        task_target
                    }
                } else {
                    task_target
                };

                if let Some(target_pos) = target_pos {
                    eprintln!("[AI] Pathfinding from {:?} to {:?}", current_pos, target_pos);

                    if current_pos != target_pos {
                        // Create pathfinding grid
                        let mut pf_grid = PathfindingGrid::new(self.width, self.height);

                        // Mark walkable tiles
                        for y in 0..self.height {
                            for x in 0..self.width {
                                let tile_pos = TilePos::new(x as i32, y as i32);
                                if let Some(tile) = self.get_tile(tile_pos) {
                                    // Only claimed floors and room tiles are walkable
                                    let walkable = matches!(
                                        tile.tile_type.as_str(),
                                        "claimed_floor" | "lair" | "hatchery" | "treasury"
                                        | "training_room" | "library" | "workshop"
                                        | "dungeon_heart" | "monster_spawner"
                                    );
                                    pf_grid.set_walkable(
                                        Pos::new(x as i32, y as i32),
                                        walkable,
                                    );
                                }
                            }
                        }

                        // Find path
                        let start = Pos::new(current_pos.x, current_pos.y);
                        let goal = Pos::new(target_pos.x, target_pos.y);

                        if let Some(path) = find_path(
                            start,
                            goal,
                            &pf_grid,
                            Heuristic::Manhattan,
                            false,
                        ) {
                            eprintln!("[AI] Path found with {} waypoints", path.waypoints.len());

                            // Convert Pos to TilePos and assign
                            let entity = self.entities.get_mut(creature_id).unwrap();
                            let creature = entity.as_creature_mut().unwrap();

                            creature.current_path = Some(
                                path.waypoints
                                    .iter()
                                    .map(|p| TilePos::new(p.x, p.y))
                                    .collect(),
                            );
                        } else {
                            eprintln!("[AI] No path found from {:?} to {:?}", current_pos, target_pos);
                        }
                    } else {
                        eprintln!("[AI] Already at target, performing task");
                        // Already at target, perform task
                        self.perform_creature_task(creature_id, game_data, dt);
                    }
                } else {
                    eprintln!("[AI] No target position for task");
                }
            }
        }
    }

    fn perform_creature_task(&mut self, creature_id: EntityId, game_data: &GameData, dt: f32) {
        use crate::engine::creature_ai;
        use crate::state::entities::Task;

        let entity = match self.entities.get(creature_id) {
            Some(e) => e,
            None => return,
        };

        let creature = match entity.as_creature() {
            Some(c) => c,
            None => return,
        };

        if let Some(ref task) = creature.current_task {
            match task {
                Task::Sleep(room_id) => {
                    // Find the room
                    if let Some(room) = self.rooms.iter().find(|r| r.id == *room_id) {
                        if room.room_type == "lair" {
                            // Satisfy sleep need
                            if let Some(creature) = self.entities.get_mut(creature_id)
                                .and_then(|e| e.as_creature_mut()) {
                                creature_ai::satisfy_need(creature, "sleep", 10.0, dt);
                            }
                        }
                    }
                }
                Task::Eat(room_id) => {
                    if let Some(room) = self.rooms.iter().find(|r| r.id == *room_id) {
                        if room.room_type == "hatchery" {
                            // Check if we have food available
                            if self.player.food > 0 {
                                // Consume food (1 unit per tick while eating)
                                let food_consumed = (dt * 5.0).min(self.player.food as f32) as i32;
                                self.player.food -= food_consumed;

                                // Satisfy food need
                                if let Some(creature) = self.entities.get_mut(creature_id)
                                    .and_then(|e| e.as_creature_mut()) {
                                    creature_ai::satisfy_need(creature, "food", food_consumed as f32 * 2.0, dt);
                                }
                            } else {
                                // No food available - creature stays hungry
                                eprintln!("Warning: No food available in hatchery");
                            }
                        }
                    }
                }
                Task::DepositGold(room_id) => {
                    if let Some(room) = self.rooms.iter().find(|r| r.id == *room_id) {
                        if room.room_type == "treasury" {
                            if let Some(creature) = self.entities.get_mut(creature_id)
                                .and_then(|e| e.as_creature_mut()) {
                                // Deposit gold
                                self.player.gold += creature.gold_carried;
                                creature.gold_carried = 0;
                                creature_ai::satisfy_need(creature, "gold", 10.0, dt);
                                creature.current_task = None; // Task complete
                            }
                        }
                    }
                }
                Task::Dig(tile_pos) => {
                    // Imp digging logic (will add later)
                    if let Some(tile) = self.get_tile_mut(*tile_pos) {
                        if tile.marked_for_dig {
                            tile.tile_type = "claimed_floor".to_string();
                            tile.ownership = Ownership::Player;
                            tile.marked_for_dig = false;
                            self.player.claimed_tile_count += 1;

                            // Task complete
                            if let Some(creature) = self.entities.get_mut(creature_id)
                                .and_then(|e| e.as_creature_mut()) {
                                creature.current_task = None;
                            }
                        }
                    }
                }
                _ => {
                    // Other tasks not implemented yet
                }
            }
        }
    }

    fn get_task_target_position(&self, task: &crate::state::entities::Task) -> Option<TilePos> {
        use crate::state::entities::Task;

        match task {
            Task::Dig(pos) => Some(*pos),
            Task::MoveTo(pos) => Some(*pos),
            Task::Sleep(room_id) | Task::Eat(room_id) | Task::DepositGold(room_id)
            | Task::Work(room_id) | Task::Train(room_id) | Task::Research(room_id) => {
                // Find room center
                self.rooms
                    .iter()
                    .find(|r| r.id == *room_id)
                    .map(|room| {
                        let center = self.calculate_room_center(room);
                        center
                    })
            }
            _ => None,
        }
    }

    pub fn detect_and_update_rooms(&mut self, game_data: &GameData) {
        use crate::engine::room_validator;
        use std::collections::HashSet;

        // Clear room_id from all tiles
        for row in &mut self.grid {
            for tile in row {
                tile.room_id = None;
            }
        }

        self.rooms.clear();

        // Collect all detected rooms first (without mutating grid)
        let mut detected_rooms = Vec::new();

        // Detect rooms for each room type
        let room_types = vec!["lair", "hatchery", "treasury", "training_room", "library", "workshop"];

        for room_type in room_types {
            // Find all tiles of this type
            let mut visited = HashSet::new();

            for row in &self.grid {
                for tile in row {
                    if visited.contains(&tile.pos) {
                        continue;
                    }

                    if tile.tile_type == room_type && tile.ownership == Ownership::Player {
                        // Flood fill to find contiguous room
                        let room_tiles = self.flood_fill_room(tile.pos, room_type, &mut visited);

                        if !room_tiles.is_empty() {
                            // Check min size
                            if let Some(room_data) = game_data.rooms.get(room_type) {
                                if room_tiles.len() >= room_data.build.min_tiles as usize {
                                    // Create room
                                    let room_id = self.next_room_id;
                                    self.next_room_id += 1;

                                    // Calculate quality
                                    let room = room_validator::Room {
                                        id: room_id,
                                        room_type: room_type.to_string(),
                                        tiles: room_tiles.clone(),
                                        quality: 100.0, // Simplified for now
                                        active: true,
                                    };

                                    let quality = room_validator::calculate_room_quality(&room, room_data);

                                    let mut final_room = room;
                                    final_room.quality = quality;

                                    let tile_count = room_tiles.len();
                                    detected_rooms.push((final_room, room_tiles));
                                    eprintln!("Detected {} room with {} tiles (quality: {:.1})",
                                        room_type, tile_count, quality);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Now apply room_ids to tiles (separate mutable pass)
        for (room, room_tiles) in detected_rooms {
            let room_id = room.id;

            for &pos in &room_tiles {
                if let Some(tile) = tile_grid::get_tile_mut(&mut self.grid, pos) {
                    tile.room_id = Some(room_id);
                }
            }

            self.rooms.push(room);
        }
    }

    fn flood_fill_room(
        &self,
        start: TilePos,
        room_type: &str,
        visited: &mut HashSet<TilePos>,
    ) -> HashSet<TilePos> {
        use std::collections::VecDeque;

        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(pos) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }

            if let Some(tile) = tile_grid::get_tile(&self.grid, pos) {
                if tile.tile_type == room_type && tile.ownership == Ownership::Player {
                    visited.insert(pos);
                    result.insert(pos);

                    // Add 4-way neighbors
                    let neighbors = [
                        TilePos::new(pos.x + 1, pos.y),
                        TilePos::new(pos.x - 1, pos.y),
                        TilePos::new(pos.x, pos.y + 1),
                        TilePos::new(pos.x, pos.y - 1),
                    ];

                    for neighbor in neighbors {
                        if !visited.contains(&neighbor) {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }

        result
    }

    fn calculate_room_center(&self, room: &crate::engine::room_validator::Room) -> TilePos {
        if room.tiles.is_empty() {
            return TilePos::new(0, 0);
        }

        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for tile in &room.tiles {
            min_x = min_x.min(tile.x);
            max_x = max_x.max(tile.x);
            min_y = min_y.min(tile.y);
            max_y = max_y.max(tile.y);
        }

        TilePos::new((min_x + max_x) / 2, (min_y + max_y) / 2)
    }

    fn spawn_random_hero(&mut self, game_data: &GameData) {
        // Find the hero entrance room to spawn from
        let spawn_pos = if let Some(hero_entrance) = self.rooms.iter().find(|r| r.room_type == "hero_entrance") {
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
                self.entities.spawn_hero(spawn_pos, hero_state);
                eprintln!("Spawned hero {} at hero entrance {:?}", hero_id, spawn_pos);
            }
        }
    }

    fn spawn_random_creature(&mut self, game_data: &GameData) {
        use crate::engine::pathfinding::{find_path, Heuristic, PathfindingGrid, Pos};

        // Find the dungeon heart position
        let heart_pos = self.find_dungeon_heart_position();
        if heart_pos.is_none() {
            eprintln!("Warning: No dungeon heart found, cannot spawn creatures");
            return;
        }
        let heart_pos = heart_pos.unwrap();

        // Find all monster spawner tiles
        let mut spawner_tiles = Vec::new();
        for row in &self.grid {
            for tile in row {
                if tile.tile_type == "monster_spawner" && tile.ownership == Ownership::Player {
                    spawner_tiles.push(tile.pos);
                }
            }
        }

        if spawner_tiles.is_empty() {
            eprintln!("Warning: No monster spawners found, cannot spawn creatures");
            return;
        }

        // Create pathfinding grid to check connectivity
        let mut pf_grid = PathfindingGrid::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let tile_pos = TilePos::new(x as i32, y as i32);
                if let Some(tile) = self.get_tile(tile_pos) {
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
        let available_lair_tiles = self.count_available_lair_tiles();
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
                let mut creature_state = crate::state::entities::CreatureState::new(
                    creature_id.clone(),
                    1, // level
                    monster_data.stats.health, // max_health
                    monster_data.stats.mana, // max_mana
                );

                // Initialize needs from monster data
                for (need_name, need_data) in &monster_data.needs {
                    creature_state.set_need(need_name.clone(), 50.0); // Start at 50% satisfied
                }

                let entity_id = self.entities.spawn_creature(spawn_pos, creature_state);

                // Claim a lair tile for this creature
                if let Some(lair_tile_pos) = self.find_and_claim_lair_tile(entity_id) {
                    eprintln!("Spawned creature {} at {:?}, claimed lair at {:?}",
                        creature_id, spawn_pos, lair_tile_pos);
                } else {
                    // Couldn't claim lair (shouldn't happen since we checked), remove creature
                    self.entities.remove(entity_id);
                    eprintln!("Failed to claim lair space, creature not spawned");
                }
            }
        }
    }

    /// Count available (unclaimed) lair tiles
    fn count_available_lair_tiles(&self) -> usize {
        let mut count = 0;
        for row in &self.grid {
            for tile in row {
                if tile.tile_type == "lair"
                   && tile.ownership == Ownership::Player
                   && tile.claimed_by_entity.is_none() {
                    count += 1;
                }
            }
        }
        count
    }

    /// Find an available lair tile and claim it for the given entity
    fn find_and_claim_lair_tile(&mut self, entity_id: crate::state::entities::EntityId) -> Option<TilePos> {
        // Find first available lair tile
        for row in &mut self.grid {
            for tile in row {
                if tile.tile_type == "lair"
                   && tile.ownership == Ownership::Player
                   && tile.claimed_by_entity.is_none() {
                    tile.claimed_by_entity = Some(entity_id);
                    return Some(tile.pos);
                }
            }
        }
        None
    }

    /// Release lair tile claimed by an entity (when creature dies/leaves)
    fn release_lair_tile(&mut self, entity_id: crate::state::entities::EntityId) {
        for row in &mut self.grid {
            for tile in row {
                if tile.claimed_by_entity == Some(entity_id) {
                    tile.claimed_by_entity = None;
                    eprintln!("Released lair tile at {:?}", tile.pos);
                    return;
                }
            }
        }
    }

    /// Generate food from hatcheries based on their size
    fn generate_food_from_hatcheries(&mut self, dt: f32) {
        let mut total_food_generated = 0;

        for room in &self.rooms {
            if room.room_type == "hatchery" && room.active {
                // Each hatchery tile generates 1 food per second
                let food_rate = room.tiles.len() as f32;
                total_food_generated += (food_rate * dt) as i32;
            }
        }

        if total_food_generated > 0 {
            self.player.add_resources(0, 0, total_food_generated);
        }
    }

    /// Check for starving creatures and make them desert
    fn handle_creature_desertion(&mut self, game_data: &GameData) {
        use crate::engine::creature_ai;

        let mut deserting_ids = Vec::new();

        // Check all creatures for desertion
        for (creature_id, creature) in self.entities.creatures() {
            // Check if food need is critically low
            let food_need = creature.get_need("food");

            if food_need < 10.0 {
                // Creature is starving
                if let Some(monster_data) = game_data.monsters.get(&creature.creature_id) {
                    // Check if mood is below desertion threshold
                    if creature.mood < monster_data.ai.desertion_threshold as f32 {
                        deserting_ids.push(creature_id);
                        eprintln!("Creature {} is deserting due to starvation (food: {:.1}, mood: {:.1})",
                            creature.creature_id, food_need, creature.mood);
                    }
                }
            }
        }

        // Remove deserting creatures
        for creature_id in deserting_ids {
            self.release_lair_tile(creature_id);
            self.entities.remove(creature_id);
        }
    }

    /// Find the dungeon heart tile position
    fn find_dungeon_heart_position(&self) -> Option<TilePos> {
        for row in &self.grid {
            for tile in row {
                if tile.tile_type == "dungeon_heart" && tile.ownership == Ownership::Player {
                    return Some(tile.pos);
                }
            }
        }
        None
    }

    /// Check if any tiles are marked for digging
    fn has_marked_tiles(&self) -> bool {
        for row in &self.grid {
            for tile in row {
                if tile.marked_for_dig {
                    return true;
                }
            }
        }
        false
    }

    /// Count how many imps are currently spawned
    fn count_imps(&self) -> usize {
        self.entities
            .creatures()
            .filter(|(_, creature)| creature.creature_id == "imp")
            .count()
    }

    /// Spawn an imp at a claimed floor tile or dungeon heart
    fn spawn_imp(&mut self, game_data: &GameData) {
        // Find a claimed floor tile to spawn on
        let mut spawn_positions = Vec::new();

        for row in &self.grid {
            for tile in row {
                if tile.ownership == Ownership::Player
                   && (tile.tile_type == "claimed_floor" || tile.tile_type == "dungeon_heart") {
                    spawn_positions.push(tile.pos);
                }
            }
        }

        if spawn_positions.is_empty() {
            // No claimed tiles yet, can't spawn imp
            return;
        }

        // Pick a random spawn position
        let pos = spawn_positions[rand::random::<usize>() % spawn_positions.len()];

        // Spawn the imp
        if let Some(monster_data) = game_data.monsters.get("imp") {
            let creature_state = crate::state::entities::CreatureState::new(
                "imp".to_string(),
                1,
                monster_data.stats.health,
                monster_data.stats.mana,
            );
            self.entities.spawn_creature(pos, creature_state);
            eprintln!("Spawned imp at {:?}", pos);
        }
    }

    /// Update imp digging behavior
    fn update_imp_digging(&mut self, game_data: &GameData, dt: f32) {
        use crate::engine::pathfinding::{find_path, Heuristic, PathfindingGrid, Pos};

        // Get all imp IDs
        let imp_ids: Vec<EntityId> = self.entities
            .creatures()
            .filter(|(_, creature)| creature.creature_id == "imp")
            .map(|(id, _)| id)
            .collect();

        for imp_id in imp_ids {
            // Get imp position
            let imp_pos = {
                let entity = match self.entities.get(imp_id) {
                    Some(e) => e,
                    None => continue,
                };
                entity.pos
            };

            // Check if imp has a path and is moving
            let (has_path, should_check_dig) = {
                let entity = self.entities.get(imp_id).unwrap();
                let creature = entity.as_creature().unwrap();
                (creature.current_path.is_some(), creature.current_path.is_none())
            };

            // If imp has no path, find nearest marked tile
            if should_check_dig {
                if let Some(marked_pos) = self.find_nearest_marked_tile(imp_pos) {
                    // Check if imp is already at the marked tile
                    if imp_pos == marked_pos {
                        // Dig the tile!
                        if let Some(tile) = self.get_tile_mut(marked_pos) {
                            if tile.marked_for_dig {
                                // Check if tile has resources (gold)
                                let gold_gained = if tile.tile_type == "gold_vein" {
                                    // Gold veins give 50 gold when dug
                                    if let Some(resources) = tile.resources_remaining {
                                        50.min(resources as i32)
                                    } else {
                                        50
                                    }
                                } else {
                                    0
                                };

                                // Convert to claimed floor
                                tile.tile_type = "claimed_floor".to_string();
                                tile.ownership = Ownership::Player;
                                tile.marked_for_dig = false;
                                tile.resources_remaining = None;
                                self.player.claimed_tile_count += 1;

                                // Give gold to player
                                if gold_gained > 0 {
                                    self.player.gold += gold_gained;
                                    eprintln!("Imp dug gold vein at {:?}, gained {} gold", marked_pos, gold_gained);
                                } else {
                                    eprintln!("Imp dug tile at {:?}", marked_pos);
                                }
                            }
                        }
                    } else {
                        // Pathfind to the marked tile
                        let mut pf_grid = PathfindingGrid::new(self.width, self.height);

                        // Mark walkable tiles (claimed floors and marked earth)
                        for y in 0..self.height {
                            for x in 0..self.width {
                                let tile_pos = TilePos::new(x as i32, y as i32);
                                if let Some(tile) = self.get_tile(tile_pos) {
                                    let walkable = tile.ownership == Ownership::Player
                                        || tile.marked_for_dig;
                                    let pf_pos = Pos::new(x as i32, y as i32);
                                    pf_grid.set_walkable(pf_pos, walkable);
                                }
                            }
                        }

                        // Find path
                        let start = Pos::new(imp_pos.x, imp_pos.y);
                        let goal = Pos::new(marked_pos.x, marked_pos.y);

                        if let Some(path) = find_path(start, goal, &pf_grid, Heuristic::Manhattan, false) {
                            // Convert to TilePos path
                            let waypoints: Vec<TilePos> = path.waypoints
                                .into_iter()
                                .map(|p| TilePos::new(p.x, p.y))
                                .collect();

                            // Assign path to imp
                            if let Some(entity) = self.entities.get_mut(imp_id) {
                                if let Some(creature) = entity.as_creature_mut() {
                                    creature.current_path = Some(waypoints);
                                }
                            }
                        }
                    }
                } else {
                    // No marked tiles - imp should wander
                    // Pick a random walkable tile within 5 tiles
                    let wander_radius = 5;
                    let mut attempts = 0;
                    let mut wander_pos = None;

                    while attempts < 10 && wander_pos.is_none() {
                        let dx = rand::random::<i32>() % (wander_radius * 2 + 1) - wander_radius;
                        let dy = rand::random::<i32>() % (wander_radius * 2 + 1) - wander_radius;
                        let candidate = TilePos::new(imp_pos.x + dx, imp_pos.y + dy);

                        if let Some(tile) = self.get_tile(candidate) {
                            // Only wander on claimed/room tiles
                            let is_walkable = matches!(
                                tile.tile_type.as_str(),
                                "claimed_floor" | "lair" | "hatchery" | "treasury"
                                | "training_room" | "library" | "workshop"
                                | "dungeon_heart" | "monster_spawner"
                            );
                            if is_walkable {
                                wander_pos = Some(candidate);
                            }
                        }
                        attempts += 1;
                    }

                    // Pathfind to wander position
                    if let Some(target_pos) = wander_pos {
                        if imp_pos != target_pos {
                            let mut pf_grid = PathfindingGrid::new(self.width, self.height);

                            // Mark walkable tiles
                            for y in 0..self.height {
                                for x in 0..self.width {
                                    let tile_pos = TilePos::new(x as i32, y as i32);
                                    if let Some(tile) = self.get_tile(tile_pos) {
                                        let walkable = tile.ownership == Ownership::Player;
                                        pf_grid.set_walkable(Pos::new(x as i32, y as i32), walkable);
                                    }
                                }
                            }

                            // Find path
                            let start = Pos::new(imp_pos.x, imp_pos.y);
                            let goal = Pos::new(target_pos.x, target_pos.y);

                            if let Some(path) = find_path(start, goal, &pf_grid, Heuristic::Manhattan, false) {
                                let waypoints: Vec<TilePos> = path.waypoints
                                    .into_iter()
                                    .map(|p| TilePos::new(p.x, p.y))
                                    .collect();

                                if let Some(entity) = self.entities.get_mut(imp_id) {
                                    if let Some(creature) = entity.as_creature_mut() {
                                        creature.current_path = Some(waypoints);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Handle movement along path (same logic as other creatures)
            let (should_move, next_waypoint) = {
                let entity = self.entities.get_mut(imp_id).unwrap();
                let creature = entity.as_creature_mut().unwrap();

                let mut should_move = false;
                let mut next_waypoint = None;

                if let Some(ref mut path) = creature.current_path {
                    if !path.is_empty() {
                        creature.move_timer += dt;
                        let move_interval = 1.0 / creature.movement_speed;

                        if creature.move_timer >= move_interval {
                            creature.move_timer = 0.0;
                            should_move = true;
                            next_waypoint = path.first().copied();
                        }
                    }
                }

                (should_move, next_waypoint)
            };

            if should_move {
                if let Some(next_pos) = next_waypoint {
                    let entity = self.entities.get_mut(imp_id).unwrap();
                    entity.pos = next_pos;

                    let creature = entity.as_creature_mut().unwrap();
                    if let Some(ref mut path) = creature.current_path {
                        path.remove(0);
                        if path.is_empty() {
                            creature.current_path = None;
                        }
                    }
                }
            }
        }
    }

    /// Find the nearest marked tile to a given position
    fn find_nearest_marked_tile(&self, pos: TilePos) -> Option<TilePos> {
        let mut nearest = None;
        let mut min_distance = f32::INFINITY;

        for row in &self.grid {
            for tile in row {
                if tile.marked_for_dig {
                    let dx = (tile.pos.x - pos.x).abs() as f32;
                    let dy = (tile.pos.y - pos.y).abs() as f32;
                    let distance = dx + dy; // Manhattan distance

                    if distance < min_distance {
                        min_distance = distance;
                        nearest = Some(tile.pos);
                    }
                }
            }
        }

        nearest
    }

    fn resolve_combat(&mut self, game_data: &GameData, dt: f32) {
        let all_entities: Vec<EntityId> = self.entities.all().map(|e| e.id).collect();
        
        for &attacker_id in &all_entities {
            if let Some(attacker) = self.entities.get(attacker_id) {
                // Find combat targets
                let targets = find_combat_targets(attacker, self.entities.entities(), game_data);
                
                for target_id in targets {
                    if let Some(defender) = self.entities.get(target_id) {
                        // Check if in combat range
                        if crate::engine::combat::in_combat_range(attacker.pos, defender.pos, 
                            match &attacker.entity_type {
                                crate::state::entities::EntityType::Creature(state) => {
                                    game_data.monsters.get(&state.creature_id)
                                        .map(|d| d.combat.attack_type.clone())
                                        .unwrap_or_else(|| "melee".to_string())
                                }
                                crate::state::entities::EntityType::Hero(state) => {
                                    game_data.heroes.get(&state.hero_id)
                                        .map(|d| d.combat.attack_type.clone())
                                        .unwrap_or_else(|| "melee".to_string())
                                }
                            }.as_str()) {
                            
                            // Resolve combat tick
                            let result = resolve_combat_tick(attacker, defender, dt, game_data);
                            
                            // Apply damage and effects
                            crate::engine::combat::apply_combat_result(
                                &result, attacker_id, target_id, self.entities.entities_mut()
                            );
                            
                            // Only process one combat per attacker per tick
                            break;
                        }
                    }
                }
            }
        }
    }
}
