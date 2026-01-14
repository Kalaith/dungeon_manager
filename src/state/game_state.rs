//! Main game state container
//! Holds the dungeon grid, entities, rooms, and player state

use crate::data::GameData;
use crate::engine::combat::{find_combat_targets, resolve_combat_tick, update_status_effects};
use crate::engine::hero_ai::update_hero_ai;
use crate::engine::map_generator;
use crate::engine::room_validator::Room;
use crate::engine::tile_grid::{self, Grid};
use crate::engine::tile_types::{self, types as tt};
use crate::state::entities::{EntityId, EntityManager, HeroState};
use crate::state::player_state::PlayerState;
use crate::state::tile_state::{self, Ownership, TilePos}; // kept for other usages
use crate::state::room_manager::RoomManager;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapType {
    Standard,   // Balanced resources and hazards
    Rich,       // Lots of gold and gems, few hazards
    Hazardous,  // Many water/lava pools, less resources
    Test,       // Fixed seed for testing
}

pub struct GameState {
    pub dungeon: crate::state::dungeon::Dungeon,
    pub room_manager: RoomManager,
    pub time_elapsed: f32,
    pub tick_accumulator: f32,
    pub camera: crate::state::camera_state::CameraState,
    // Track pending builds
    pub pending_trap_builds: std::collections::HashSet<crate::state::tile_state::TilePos>,

    pub entities: EntityManager,
    pub player: PlayerState,
    pub next_hero_spawn_time: f32,
    pub next_creature_spawn_time: f32,
    pub pay_day_timer: f32,
    pub paused: bool,
}

impl GameState {
    pub fn new(width: usize, height: usize, game_data: &GameData) -> Self {
        Self::new_with_map_type(width, height, game_data, MapType::Standard)
    }

    pub fn new_with_map_type(width: usize, height: usize, game_data: &GameData, map_type: MapType) -> Self {
        let mut dungeon = crate::state::dungeon::Dungeon::new(width, height, game_data, map_type);
        let mut room_manager = RoomManager::new();
        
        // Detect and register starting rooms created by map generator
        room_manager.detect_and_update_rooms(&mut dungeon, game_data);

        let entities = EntityManager::new();

        let mut state = Self {
            dungeon,
            room_manager,
            time_elapsed: 0.0,
            tick_accumulator: 0.0,
            camera: crate::state::camera_state::CameraState::new(width as f32, height as f32),
            pending_trap_builds: HashSet::new(),

            entities,
            player: PlayerState::new(game_data),
            next_hero_spawn_time: 30.0, // Spawn first hero after 30 seconds
            next_creature_spawn_time: 10.0, // Spawn first creature after 10 seconds
            pay_day_timer: 0.0,
            paused: false,
        };

        // Recalculate max gold and other room-based stats
        state.detect_and_update_rooms(game_data);

        // Spawn 3 starting imps
        state.spawn_starting_imps(game_data, 3);

        state
    }

    pub fn update(&mut self, dt: f32, game_data: &GameData) {
        self.time_elapsed += dt;
        self.tick_accumulator += dt;
        self.camera.update(dt); // Update smooth camera zoom

        // Smooth movement interpolation
        for entity in self.entities.all_mut() {
             let target_x = entity.pos.x as f32;
             let target_z = entity.pos.y as f32;
             
             let dx = target_x - entity.visual_pos.0;
             let dz = target_z - entity.visual_pos.1;
             
             // Simple lerp
             let speed = 10.0 * dt;
             entity.visual_pos.0 += dx * speed;
             entity.visual_pos.1 += dz * speed;
             
             // Snap if close
             if dx.abs() < 0.01 && dz.abs() < 0.01 {
                  entity.visual_pos.0 = target_x;
                  entity.visual_pos.1 = target_z;
             }
        }

        // Fixed timestep simulation (10 ticks per second)
        const TICK_RATE: f32 = 0.1;
        while self.tick_accumulator >= TICK_RATE {
            self.tick(game_data, TICK_RATE);
            self.tick_accumulator -= TICK_RATE;
        }
    }

    fn tick(&mut self, game_data: &GameData, dt: f32) {
        // Update spell cooldowns
        crate::engine::spell_effects::update_spell_cooldowns(self, dt);

        // Imp spawning is now handled by the Summon Imp spell

        // Update imps first (they dig tiles)
        crate::engine::imp_ai::update_imp_digging(
            &mut self.dungeon,
            &mut self.entities,
            &mut self.player,
            game_data,
            dt,
        );

        // Update creature AI and movement
        self.update_creature_ai_and_movement(game_data, dt);

        // Execute creature tasks (Work, Eat, Sleep, etc.)
        let creature_ids: Vec<crate::state::entities::EntityId> = self.entities.creatures().map(|(id, _)| id).collect();
        for id in creature_ids {
            self.perform_creature_task(id, game_data, dt);
        }

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

        // Pay Day Logic
        self.pay_day_timer += dt;
        if self.pay_day_timer >= 300.0 { // 5 minutes
            self.pay_day_timer = 0.0;
            self.trigger_pay_day();
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

        // Update fog of war based on claimed tiles and creature positions
        self.update_fog_of_war_system(game_data);
        
        // Update traps
        crate::engine::trap_system::process_trap_construction(
            &mut self.dungeon,
            &mut self.player,
            &mut self.pending_trap_builds,
            game_data,
            dt,
        );

        // Update creature count (excluding imps)
        self.player.current_creature_count = self.count_monsters();
    }

    /// Update fog of war using the tile_grid system
    fn update_fog_of_war_system(&mut self, game_data: &GameData) {
        use std::collections::HashSet;
        
        // Collect claimed tiles
        let mut claimed_tiles = HashSet::new();
        // Access grid via dungeon
        let (width, height) = tile_grid::get_grid_dimensions(&self.dungeon.grid);
        
        for y in 0..height {
            for x in 0..width {
                let pos = TilePos::new(x as i32, y as i32);
                if let Some(tile) = tile_grid::get_tile(&self.dungeon.grid, pos) {
                    if tile.ownership == Ownership::Player {
                        claimed_tiles.insert(pos);
                    }
                }
            }
        }
        
        // Collect creature positions (player's creatures provide vision, except imps)
        let creature_positions: Vec<TilePos> = self.entities
            .creatures()
            .filter(|(_, creature)| creature.creature_id != "imp") // Imps don't provide vision
            .map(|(id, _)| self.entities.get(id))
            .flatten()
            .map(|e| e.pos)
            .collect();
        
        // Update fog of war with sight radius of 5
        self.dungeon.update_fog_of_war(&claimed_tiles, &creature_positions, game_data);
    }

    /// Check if imps have work to do (dig marks exist)  
    pub fn imps_have_work(&self) -> bool {
        self.has_marked_tiles()
    }

    pub fn get_tile(&self, pos: TilePos) -> Option<&crate::state::tile_state::TileState> {
        self.dungeon.get_tile(pos)
    }

    pub fn get_tile_mut(&mut self, pos: TilePos) -> Option<&mut crate::state::tile_state::TileState> {
        self.dungeon.get_tile_mut(pos)
    }

    fn update_creature_ai_and_movement(&mut self, game_data: &GameData, dt: f32) {
        use crate::engine::creature_ai;

        // Delegate to the creature_ai module
        creature_ai::update_creatures(
            &self.dungeon,
            &mut self.entities,
            &self.room_manager,
            game_data,
            dt,
            |task, room_manager| GameState::get_task_target_position_static(task, room_manager),
        );
    }

    /// Static version of get_task_target_position that doesn't require &self
    fn get_task_target_position_static(task: &crate::state::entities::Task, room_manager: &crate::state::room_manager::RoomManager) -> Option<TilePos> {
        use crate::state::entities::Task;

        match task {
            Task::Sleep(room_id)
            | Task::Eat(room_id)
            | Task::Work(room_id)
            | Task::Train(room_id)
            | Task::Research(room_id)
            | Task::DepositGold(room_id)
            | Task::CollectWages(room_id) => {
                room_manager.rooms.iter()
                    .find(|r| r.id == *room_id)
                    .map(|room| room.get_center())
            }
            Task::Dig(pos) => Some(*pos),
            Task::MoveTo(pos) => Some(*pos),
            Task::Attack(entity_id) => None, // Would need entity positions
            Task::Idle | Task::Flee => None,
        }
    }

    fn perform_creature_task(&mut self, creature_id: EntityId, game_data: &GameData, dt: f32) {
        use crate::engine::task_system;

        // Delegate to task_system module
        let result = task_system::execute_task(
            creature_id,
            &mut self.entities,
            &self.room_manager,
            &self.player,
            game_data,
            dt,
            |pos| self.dungeon.get_tile(pos).cloned(),
        );

        // Apply state changes from task result
        if result.gold_change != 0 {
            self.player.add_resources(result.gold_change, 0, 0, 0);
        }
        if result.food_change != 0 {
            self.player.add_resources(0, 0, result.food_change, 0);
        }
        if result.materials_change != 0 {
            self.player.add_resources(0, 0, 0, result.materials_change);
        }
        if let Some(tile_pos) = result.claimed_tile {
            if let Some(tile) = self.get_tile_mut(tile_pos) {
                tile.tile_type = crate::engine::tile_types::types::CLAIMED_FLOOR.to_string();
                tile.ownership = Ownership::Player;
                tile.marked_for_dig = false;
                self.player.claimed_tile_count += 1;
            }
        }
    }

    fn get_task_target_position(&self, task: &crate::state::entities::Task) -> Option<TilePos> {
        use crate::state::entities::Task;

        match task {
            Task::Dig(pos) => Some(*pos),
            Task::MoveTo(pos) => Some(*pos),
            Task::Sleep(room_id) | Task::Eat(room_id) | Task::DepositGold(room_id)
            | Task::Work(room_id) | Task::Train(room_id) | Task::Research(room_id) | Task::CollectWages(room_id) => {
                // Find room center
                self.room_manager.rooms
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
        self.room_manager.detect_and_update_rooms(&mut self.dungeon, game_data);
        
        // Recalculate max gold and mana based on rooms
        let mut max_gold = 0; 
        let mut max_mana = 0;
        
        for room in &self.room_manager.rooms {
            if let Some(room_data) = game_data.rooms.get(&room.room_type) {
                if room_data.effects.gold_storage_capacity > 0 {
                    max_gold += room.tiles.len() as i32 * room_data.effects.gold_storage_capacity;
                }
                if room_data.effects.mana_storage_capacity > 0 {
                    max_mana += room.tiles.len() as i32 * room_data.effects.mana_storage_capacity;
                }
            }
        }
        
        self.player.max_gold = max_gold;
        self.player.max_mana = max_mana;
    }



    fn calculate_room_center(&self, room: &crate::engine::room_validator::Room) -> TilePos {
        self.room_manager.calculate_room_center(room)
    }

    fn spawn_random_hero(&mut self, game_data: &GameData) {
        crate::engine::spawner::SpawnSystem::spawn_random_hero(
            &self.room_manager,
            &mut self.entities,
            game_data
        );
    }

    fn spawn_random_creature(&mut self, game_data: &GameData) {
        crate::engine::spawner::SpawnSystem::spawn_random_creature(
            &mut self.dungeon,
            &self.room_manager,
            &mut self.entities,
            game_data
        );
    }

    /// Count available (unclaimed) lair tiles
    fn count_available_lair_tiles(&self) -> usize {
        self.room_manager.count_available_lair_tiles(&self.dungeon)
    }

    /// Find an available lair tile and claim it for the given entity
    fn find_and_claim_lair_tile(&mut self, entity_id: crate::state::entities::EntityId) -> Option<TilePos> {
        self.room_manager.find_and_claim_lair_tile(&mut self.dungeon, entity_id)
    }

    /// Release lair tile claimed by an entity (when creature dies/leaves)
    fn release_lair_tile(&mut self, entity_id: crate::state::entities::EntityId) {
        self.room_manager.release_lair_tile(&mut self.dungeon, entity_id);
    }

    /// Generate food from hatcheries based on their size
    fn generate_food_from_hatcheries(&mut self, dt: f32) {
        let total_food_generated = self.room_manager.generate_food_from_hatcheries(dt);
        if total_food_generated > 0 {
            self.player.add_resources(0, 0, total_food_generated, 0);
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
                    // Check if they should desert using AI logic
                    if creature_ai::should_desert(creature, monster_data) {
                        deserting_ids.push(creature_id);
                        eprintln!("Creature {} is deserting! (mood: {:.1})",
                            creature.creature_id, creature.mood);
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
    pub fn find_dungeon_heart_position(&self) -> Option<TilePos> {
        for row in &self.dungeon.grid {
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
        for row in &self.dungeon.grid {
            for tile in row {
                if tile.marked_for_dig {
                    return true;
                }
            }
        }
        false
    }

    /// Count how many imps are currently spawned
    pub fn count_imps(&self) -> usize {
        self.entities
            .creatures()
            .filter(|(_, creature)| creature.creature_id == "imp")
            .count()
    }

    /// Maximum number of imps allowed
    pub const MAX_IMPS: usize = 10;

    /// Count how many non-imp monsters are currently spawned
    pub fn count_monsters(&self) -> usize {
        self.entities
            .creatures()
            .filter(|(_, creature)| creature.creature_id != "imp")
            .count()
    }

    /// Spawn starting imps at game initialization
    pub fn spawn_starting_imps(&mut self, game_data: &GameData, count: usize) {
        for _ in 0..count {
            self.spawn_imp(game_data);
        }
        eprintln!("Spawned {} starting imps", count);
    }

    /// Spawn an imp at a claimed floor tile or dungeon heart
    fn spawn_imp(&mut self, game_data: &GameData) {
        // Find a claimed floor tile to spawn on
        let mut spawn_positions = Vec::new();

        for row in &self.dungeon.grid {
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

    /// Trigger Pay Day: All creatures become unpaid and seek wages
    pub fn trigger_pay_day(&mut self) {
        eprintln!("PAY DAY! All creatures are demanding wages!");
        for entity in self.entities.all_mut() {
            if let Some(creature) = entity.as_creature_mut() {
                // Set gold need to 0 (Critical) to force them to collect wages
                creature.set_need("gold".to_string(), 0.0);
            }
        }
        // TODO: Show on-screen notification
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
