//! Main game state container
//! Holds the dungeon grid, entities, rooms, and player state

use crate::data::GameData;
use crate::engine::combat::{resolve_combat_tick, update_status_effects};
use crate::engine::hero_ai::update_hero_ai;
use crate::engine::tile_grid;
use crate::state::entities::{EntityId, EntityManager, EntityType};
use crate::state::player_state::PlayerState;
use crate::state::tile_state::{Ownership, TilePos};
use crate::state::room_manager::RoomManager;
use crate::state::projectiles::ProjectileManager;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

/// Extract attack type from an entity type for combat calculations
fn get_entity_attack_type(entity_type: &EntityType, game_data: &GameData) -> String {
    match entity_type {
        EntityType::Creature(state) => game_data.monsters.get(&state.creature_id)
            .map(|d| d.combat.attack_type.clone())
            .unwrap_or_else(|| "melee".to_string()),
        EntityType::Hero(state) => game_data.heroes.get(&state.hero_id)
            .map(|d| d.combat.attack_type.clone())
            .unwrap_or_else(|| "melee".to_string()),
        EntityType::Structure(_) => "none".to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MapType {
    Standard,   // Balanced resources and hazards
    Rich,       // Lots of gold and gems, few hazards
    Hazardous,  // Many water/lava pools, less resources
    Test,       // Fixed seed for testing
    File(String), // Load from specific JSON file
}

#[derive(Serialize, Deserialize)]
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
    pub spawners: Vec<crate::engine::spawner_logic::MonsterSpawner>,
    pub paused: bool,
    pub hero_base: crate::state::hero_base::HeroBase,
    pub game_over: bool,
    pub victory: bool,

    /// Notification system for game events
    pub notifications: crate::state::notifications::NotificationManager,

    /// Dungeon Heart Health
    pub dungeon_heart_health: f32,

    // Markers
    pub attack_marker: Option<TilePos>,
    pub defend_marker: Option<TilePos>,

    /// Active attack projectiles for visual effects
    pub projectiles: ProjectileManager,
}

impl GameState {
    pub fn new(width: usize, height: usize, game_data: &GameData) -> Self {
        Self::new_with_map_type(width, height, game_data, MapType::Standard)
    }

    pub fn new_with_map_type(width: usize, height: usize, game_data: &GameData, map_type: MapType) -> Self {
        let mut entities = EntityManager::new();
        let mut dungeon;
        
        match &map_type {
            MapType::File(path) => {
                println!("Loading map from: {}", path);
                dungeon = crate::state::map_loader::load_map(path, game_data, &mut entities)
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to load map: {}. Falling back to standard generation.", e);
                         crate::state::dungeon::Dungeon::new(width, height, game_data, MapType::Standard)
                    });
            },
            _ => {
                dungeon = crate::state::dungeon::Dungeon::new(width, height, game_data, map_type.clone());
            }
        }

        let mut room_manager = RoomManager::new();
        
        // Detect and register starting rooms (either generated or loaded)
        room_manager.detect_and_update_rooms(&mut dungeon, game_data);

        // Detect Monster Spawners from map tiles
        let mut spawners = Vec::new();
        let (w, h) = tile_grid::get_grid_dimensions(&dungeon.grid);
        for y in 0..h {
            for x in 0..w {
                let pos = TilePos::new(x as i32, y as i32);
                if let Some(tile) = tile_grid::get_tile(&dungeon.grid, pos) {
                    if tile.tile_type == "monster_spawner" {
                        // Randomly pick Spider or Lizard
                        let monster_id = if macroquad::rand::gen_range(0.0f32, 1.0) < 0.5 { "spider" } else { "lizard" };
                        spawners.push(crate::engine::spawner_logic::MonsterSpawner::new(
                            pos, 
                            monster_id.to_string(), 
                            10 // Max 10 monsters
                        ));
                    }
                }
            }
        }

        let mut state = Self {
            dungeon,
            room_manager,
            time_elapsed: 0.0,
            tick_accumulator: 0.0,
            camera: crate::state::camera_state::CameraState::new(width as f32, height as f32),
            pending_trap_builds: HashSet::new(),

            entities,
            player: PlayerState::new(game_data),
            next_hero_spawn_time: 0.0,
            next_creature_spawn_time: 0.0,
            pay_day_timer: 0.0, 
            spawners,
            paused: false,
            hero_base: crate::state::hero_base::HeroBase::new(game_data),
            game_over: false,
            victory: false,
            notifications: crate::state::notifications::NotificationManager::new(),
            dungeon_heart_health: 1000.0,
            attack_marker: None,
            defend_marker: None,
            projectiles: ProjectileManager::new(),
        };

        // Recalculate max gold and other room-based stats
        state.detect_and_update_rooms(game_data);

        // Spawn starting imps only if none exist (loaded map might have them)
        let imp_count = state.entities.all()
            .filter_map(|e| e.as_creature())
            .filter(|c| c.creature_id == "imp")
            .count();
            
        if imp_count == 0 {
            state.spawn_starting_imps(game_data, 3);
        }

        state
    }

    pub fn update(&mut self, dt: f32, game_data: &GameData) {
        self.time_elapsed += dt;
        self.tick_accumulator += dt;
        self.camera.update(dt); // Update smooth camera zoom
        self.notifications.update(dt); // Update notification display timers
        self.projectiles.update(dt); // Update attack projectiles

        // Smooth movement interpolation
        for entity in self.entities.all_mut() {
             let target_x = entity.pos.x as f32;
             let target_z = entity.pos.y as f32;

             let dx = target_x - entity.visual_pos.0;
             let dz = target_z - entity.visual_pos.1;

             // Simple lerp
             let speed = crate::config::MOVEMENT_LERP_SPEED * dt;
             entity.visual_pos.0 += dx * speed;
             entity.visual_pos.1 += dz * speed;
             
             // Snap if close
             if dx.abs() < 0.01 && dz.abs() < 0.01 {
                  entity.visual_pos.0 = target_x;
                  entity.visual_pos.1 = target_z;
             }
        }

        // Fixed timestep simulation (10 ticks per second)
        while self.tick_accumulator >= crate::config::TICK_RATE {
            self.tick(game_data, crate::config::TICK_RATE);
            self.tick_accumulator -= crate::config::TICK_RATE;
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

        // Hero spawning via Hero Base
        crate::engine::hero_spawner::update_hero_spawning(self, game_data, dt);

        // NPC Spawner Update
        crate::engine::spawner_logic::SpawnerSystem::update(
            &mut self.spawners,
            &self.dungeon.grid,
            &mut self.entities,
            game_data,
            dt
        );

        // Creature spawning
        self.next_creature_spawn_time -= dt;
        if self.next_creature_spawn_time <= 0.0 {
            self.spawn_random_creature(game_data);
            let spawn_range = game_data.config.timing.creature_spawn_max_interval - game_data.config.timing.creature_spawn_min_interval;
            self.next_creature_spawn_time = game_data.config.timing.creature_spawn_min_interval + macroquad::rand::gen_range(0.0f32, 1.0) * spawn_range;
        }

        // Pay Day Logic
        self.pay_day_timer += dt;
        if self.pay_day_timer >= game_data.config.timing.pay_day_interval {
            self.pay_day_timer = 0.0;
            self.trigger_pay_day();
        }

        // Update hero AI
        let hero_entities: Vec<EntityId> = self.entities.heroes().map(|(id, _)| id).collect();

        if self.hero_base.enabled {
             crate::engine::hero_spawner::update_hero_spawning(self, game_data, dt);
        }

        // Dungeon Heart Attack Logic
        let heart_pos = self.find_dungeon_heart_position();
        if let Some(target_pos) = heart_pos {
             let mut total_damage = 0.0;
             // Check for adjacent heroes with DestroyHeart goal
             for entity in self.entities.all() {
                if let Some(hero) = entity.as_hero() {
                    if matches!(hero.current_goal, crate::state::entities::HeroGoal::DestroyHeart) {
                        let dx = (entity.pos.x - target_pos.x).abs();
                        let dy = (entity.pos.y - target_pos.y).abs();
                        
                        // Debug log to see why it might fail
                        if dx <= 2 && dy <= 2 {
                             /*eprintln!("Hero {} at {:?} attacking heart at {:?} (dx={}, dy={}, goal={:?})", 
                                hero.hero_id, entity.pos, target_pos, dx, dy, hero.current_goal);*/
                        }

                        // If adjacent (manhattan distance <= 1 ensures N/S/E/W adjacency)
                        // Diagonal adjacency would be dx <= 1 && dy <= 1 && (dx+dy) > 0
                        // Let's allow diagonals for attacking too since they can pathfind diagonally
                        // Also allow dx=0, dy=0 (standing on it) for robustness
                        if dx <= 1 && dy <= 1 {
                             total_damage += 20.0 * dt; // 20 DPS per hero
                             // eprintln!("Applying damage! Heart Health: {:.1} - Damage: {:.1}", self.dungeon_heart_health, 20.0 * dt);
                        }
                    } else {
                        // eprintln!("Hero {} adjacent but NOT DestroyHeart. Goal: {:?}", hero.hero_id, hero.current_goal);
                    }
                }
            }
            
            if total_damage > 0.0 {
                self.dungeon_heart_health -= total_damage;
                eprintln!("Heart taking damage! Health: {:.1} -> {:.1}", self.dungeon_heart_health + total_damage, self.dungeon_heart_health);
                if self.dungeon_heart_health <= 0.0 && !self.game_over {
                    self.game_over = true;
                    self.notifications.danger("DUNGEON HEART DESTROYED! GAME OVER");
                }
            } else {
                 // Check if heroes are nearby but not damaging
                 for entity in self.entities.all() {
                    if let Some(hero) = entity.as_hero() {
                        let dx = (entity.pos.x - target_pos.x).abs();
                        let dy = (entity.pos.y - target_pos.y).abs();
                        if dx <= 5 && dy <= 5 {
                             eprintln!("Review: Hero {} at {:?} (dist {}, {}) Goal: {:?}. Dmg: 0", 
                                hero.hero_id, entity.pos, dx, dy, hero.current_goal);
                        }
                    }
                 }
            }
        }
        for hero_id in hero_entities {
            // Update AI first
            if let Some(entity) = self.entities.get(hero_id) {
                if let Some(hero_state) = entity.as_hero() {
                    // Skip AI for captured heroes
                    if hero_state.is_captured {
                        continue;
                    }

                    let mut hero_state_clone = hero_state.clone();
                    crate::engine::hero_ai::update_hero_ai(entity, &mut hero_state_clone, self, game_data, dt);
                     // Update the entity with the new hero state
                    if let Some(entity_mut) = self.entities.get_mut(hero_id) {
                        if let Some(hero_state_mut) = entity_mut.as_hero_mut() {
                            *hero_state_mut = hero_state_clone;
                        }
                    }
                }
            }

            // Handle hero digging logic
            let (carved_wall, should_move) = self.process_hero_digging(hero_id, game_data, dt);

            if let Some(pos) = carved_wall {
                if let Some(tile) = self.dungeon.get_tile_mut(pos) {
                    tile.tile_type = "claimed_floor".to_string();
                    tile.ownership = crate::state::tile_state::Ownership::Unclaimed;
                }
            }

            // Process movement for hero if not digging
            if should_move {
                crate::engine::movement::process_entity_movement(&mut self.entities, hero_id, dt);
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

        // Handle dead heroes - check for prison capture
        self.handle_prison_captures(game_data);

        // Progress prison conversions
        self.progress_prison_conversions(game_data, dt);

        // Remove dead entities and release their lair space (but not captured heroes)
        let dead_ids: Vec<EntityId> = self.entities.all()
            .filter(|e| {
                if !e.is_alive() {
                    // Don't remove captured heroes
                    if let Some(hero) = e.as_hero() {
                        return !hero.is_captured;
                    }
                    return true;
                }
                false
            })
            .map(|e| e.id)
            .collect();

        for entity_id in dead_ids {
            self.release_lair_tile(entity_id);
            self.entities.remove(entity_id);
        }

        // Update fog of war based on claimed tiles and creature positions
        self.update_fog_of_war_system(game_data);
        
        // Update trap construction
        crate::engine::trap_system::process_trap_construction(
            &mut self.dungeon,
            &mut self.player,
            &mut self.pending_trap_builds,
            game_data,
            dt,
        );

        // Process trap triggers (when heroes step on traps)
        let trap_results = crate::engine::trap_system::process_trap_triggers(
            &mut self.dungeon,
            &mut self.entities,
            game_data,
            dt,
        );

        // Notify about trap triggers
        for result in trap_results {
            self.notifications.info(format!("{} triggered! ({:.0} damage)", result.trap_type, result.damage_dealt));
        }

        // Update creature count (only player creatures, excluding imps and wild monsters)
        self.player.current_creature_count = self.count_player_creatures(game_data) - self.count_imps();

        // Check for Game Over
        self.check_game_over_conditions();
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
        
        // Collect creature positions (only player's creatures provide vision, except imps)
        // Wild/neutral creatures should not reveal fog for the player
        let creature_positions: Vec<TilePos> = self.entities
            .creatures()
            .filter(|(_, creature)| {
                // Exclude imps (workers don't provide vision)
                if creature.creature_id == "imp" { return false; }
                // Only dungeon faction creatures provide vision
                if let Some(monster_data) = game_data.monsters.get(&creature.creature_id) {
                    monster_data.faction == "dungeon"
                } else {
                    false
                }
            })
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
            self.attack_marker,
            self.defend_marker,
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
                    .map(|room: &crate::engine::room_validator::Room| room.get_center())
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
        if result.research_change > 0.0 {
            if let Some(active_tech_id) = &self.player.active_research {
                // Determine cost
                let cost = if let Some(tech) = game_data.technologies.get(active_tech_id) {
                    tech.cost
                } else {
                    100.0 // Fallback
                };
                
                if let Some(completed) = self.player.update_research(result.research_change, cost, dt) {
                    // Research completed!
                    if let Some(tech) = game_data.technologies.get(&completed) {
                        self.player.complete_research(tech);
                        eprintln!("Research Complete: {}", tech.name);
                        self.notifications.success(format!("Research Complete: {}", tech.name));
                    }
                }
            }
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
        
        // Detect Hero Base Buildings
        self.detect_hero_base(game_data);

        // Recalculate max gold and mana based on rooms
        let mut max_gold = 0; 
        let mut max_mana = 0;
        let mut lair_tiles_count = 0;
        
        for room in &self.room_manager.rooms {
            if room.room_type == "lair" {
                lair_tiles_count += room.tiles.len();
            }

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
        
        // Capacity is exactly equal to the number of lair tiles
        self.player.max_creatures = lair_tiles_count;
    }

    fn detect_hero_base(&mut self, game_data: &GameData) {
        // Clear existing buildings to avoid duplicates on re-scan
        self.hero_base.buildings.clear();
        self.hero_base.enabled = false;

        let (w, h) = tile_grid::get_grid_dimensions(&self.dungeon.grid);
        let mut base_detected = false;
        let mut base_center_acc = (0, 0);
        let mut building_count = 0;

        for y in 0..h {
            for x in 0..w {
                let pos = TilePos::new(x as i32, y as i32);
                if let Some(tile) = tile_grid::get_tile(&self.dungeon.grid, pos) {
                    if let Some(building_data) = game_data.hero_buildings.get(&tile.tile_type) {
                        // Found a building tile!
                        let mut building = crate::state::hero_base::HeroBuilding {
                            id: format!("{}_{}_{}", tile.tile_type, pos.x, pos.y),
                            building_type: tile.tile_type.clone(),
                            pos,
                            current_hp: building_data.hp,
                            spawn_timers: building_data.spawn_triggers.iter().map(|t| crate::state::hero_base::SpawnTimer {
                                hero_id: t.hero_id.clone(),
                                time_until_spawn: 1.0, // Start almost immediately for feedback
                            }).collect(),
                            entity_id: None,
                        };

                        // Spawn Structure Entity
                        let structure_state = crate::state::entities::StructureState::new(
                            tile.tile_type.clone(),
                            building_data.hp as f32
                        );
                        let entity_id = self.entities.spawn_structure(pos, structure_state);
                        building.entity_id = Some(entity_id);

                        self.hero_base.buildings.push(building);
                        
                        base_detected = true;
                        base_center_acc.0 += x as i32;
                        base_center_acc.1 += y as i32;
                        building_count += 1;
                    }
                }
            }
        }

        if base_detected {
            self.hero_base.enabled = true;
            if building_count > 0 {
                 self.hero_base.position = TilePos::new(base_center_acc.0 / building_count, base_center_acc.1 / building_count);
            }
            eprintln!("Hero Base detected with {} buildings at {:?}", building_count, self.hero_base.position);
        }
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

    /// Check for creatures with critical needs and make them desert
    fn handle_creature_desertion(&mut self, game_data: &GameData) {
        use crate::engine::creature_ai;

        let mut deserting_ids = Vec::new();

        // Check all creatures for desertion
        for (creature_id, creature) in self.entities.creatures() {
            // Check ALL critical needs, not just food
            let food_need = creature.get_need("food");
            let sleep_need = creature.get_need("sleep");
            let gold_need = creature.get_need("gold");

            // Creature is in critical condition if any need is below threshold
            let desert_threshold = game_data.config.creature_ai.need_desert_threshold;
            let is_critical = food_need < desert_threshold
                || sleep_need < desert_threshold
                || gold_need < desert_threshold;

            if is_critical {
                if let Some(monster_data) = game_data.monsters.get(&creature.creature_id) {
                    // Check if they should desert using AI logic (mood-based)
                    if creature_ai::should_desert(creature, monster_data) {
                        deserting_ids.push(creature_id);
                        eprintln!("Creature {} is deserting! (mood: {:.1}, food: {:.1}, sleep: {:.1}, gold: {:.1})",
                            creature.creature_id, creature.mood, food_need, sleep_need, gold_need);
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

    /// Maximum number of imps allowed (reads from monster data)
    pub fn max_imps(game_data: &GameData) -> usize {
        game_data.monsters.get("imp")
            .map(|m| m.spawn.max_population as usize)
            .unwrap_or(10)
    }

    /// Count how many non-imp monsters are currently spawned
    pub fn count_monsters(&self) -> usize {
        self.entities
            .creatures()
            .filter(|(_, creature)| creature.creature_id != "imp")
            .count()
    }

    /// Count only player-owned creatures (dungeon faction, excluding imps)
    pub fn count_player_creatures(&self, game_data: &GameData) -> usize {
        self.entities
            .creatures()
            .filter(|(_, creature)| {
                if let Some(monster_data) = game_data.monsters.get(&creature.creature_id) {
                    monster_data.faction == "dungeon"
                } else {
                    false
                }
            })
            .count()
    }

    /// Check if a creature belongs to the player (dungeon faction)
    pub fn is_player_creature(&self, creature_id: &str, game_data: &GameData) -> bool {
        if let Some(monster_data) = game_data.monsters.get(creature_id) {
            monster_data.faction == "dungeon"
        } else {
            false
        }
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
        let pos = spawn_positions[macroquad::rand::gen_range(0, spawn_positions.len())];

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
        self.notifications.warning("Pay Day! Creatures demand wages.");
    }



    fn resolve_combat(&mut self, game_data: &GameData, dt: f32) {
        let all_entities: Vec<EntityId> = self.entities.all().map(|e| e.id).collect();

        for &attacker_id in &all_entities {
            let attacker = match self.entities.get(attacker_id) {
                Some(a) => a,
                None => continue,
            };

            let attack_type = get_entity_attack_type(&attacker.entity_type, game_data);
            let attacker_visual_pos = attacker.visual_pos;
            let targets = crate::engine::combat::find_combat_targets(attacker, self.entities.entities(), &self.dungeon, game_data);

            for target_id in targets {
                let defender = match self.entities.get(target_id) {
                    Some(d) => d,
                    None => continue,
                };

                if !crate::engine::combat::in_combat_range(attacker.pos, defender.pos, &attack_type, &self.dungeon.grid, game_data) {
                    continue;
                }

                let defender_visual_pos = defender.visual_pos;
                let result = resolve_combat_tick(attacker, defender, dt, game_data);
                
                // Spawn projectile if damage was dealt
                if result.damage_dealt > 0.0 {
                    self.projectiles.spawn(
                        attacker_visual_pos,
                        defender_visual_pos,
                        &attack_type,
                        attacker_id,
                        target_id,
                    );
                }
                
                crate::engine::combat::apply_combat_result(&result, attacker_id, target_id, self.entities.entities_mut(), game_data);
                break;
            }
        }
    }

    /// Handle capturing dead heroes - teleport to available prison
    fn handle_prison_captures(&mut self, game_data: &GameData) {
        // Find heroes that just died (health <= 0) and aren't already captured
        let mut heroes_to_capture: Vec<EntityId> = Vec::new();

        for (hero_id, hero) in self.entities.heroes() {
            if hero.health <= 0.0 && !hero.is_captured && !hero.is_converted {
                heroes_to_capture.push(hero_id);
            }
        }

        if heroes_to_capture.is_empty() {
             return;
        }

        // Find available prison tiles
        // We'll just look for ANY prison tile for now, ideally one that is empty
        let mut available_prison_tiles = Vec::new();
        for room in &self.room_manager.rooms {
             if room.room_type == "prison" {
                 for &tile_pos in &room.tiles {
                      available_prison_tiles.push(tile_pos);
                 }
             }
        }

        if available_prison_tiles.is_empty() {
             return; // No prison, they just die
        }

        // Shuffle tiles to avoid stacking all in one spot (if possible)
        // For deterministic behavior in this tool we might skip shuffle or use a simple index
        
        for hero_id in heroes_to_capture {
             // Just pick a random one for now
             let random_idx = macroquad::rand::gen_range(0, available_prison_tiles.len());
             let target_pos = available_prison_tiles[random_idx];

             // Capture the hero
             if let Some(entity) = self.entities.get_mut(hero_id) {
                 // Move entity first to avoid borrow issues
                 entity.pos = target_pos;
                 entity.visual_pos = (target_pos.x as f32, target_pos.y as f32);
                 
                 if let Some(hero) = entity.as_hero_mut() {
                     hero.is_captured = true;
                     hero.conversion_progress = 0.0;
                     // Revive slightly so they don't get cleaned up as "dead" immediately
                     hero.health = 10.0; 
                     
                     eprintln!("Hero {} captured and teleported to prison at {:?}!", hero.hero_id, target_pos);
                     self.notifications.success(format!("Hero captured!"));
                 }
             }
        }
    }

    /// Progress prison (Skeleton) and torture (Conversion) logic
    fn progress_prison_conversions(&mut self, game_data: &GameData, dt: f32) {
        // Identify active Torture Chambers with working Succubi
        let mut active_torture_rooms = std::collections::HashSet::new();
        for (_, creature) in self.entities.creatures() {
             if creature.creature_id == "succubus" {
                 if let Some(crate::state::entities::Task::Work(room_id)) = creature.current_task {
                      active_torture_rooms.insert(room_id);
                 }
             }
        }

        // Get conversion rates
        // Prison Rate (for Skeletons)
        let skeleton_rate = 0.05; // Fixed slow rate for now, or fetch from room data

        // Torture Rate (for Conversion)
        let torture_base_rate = 0.1; // Base rate
        
        let mut conversions_to_process: Vec<(EntityId, String, bool)> = Vec::new(); // (Id, Type, IsTorture)

        for (hero_id, hero) in self.entities.heroes() {
            if hero.is_captured && !hero.is_converted {
                if let Some(entity) = self.entities.get(hero_id) {
                    // Check which room they are in
                    if let Some(room) = self.room_manager.get_room_at(entity.pos) {
                        if room.room_type == "prison" {
                             conversions_to_process.push((hero_id, "skeleton".to_string(), false));
                        } else if room.room_type == "torture_chamber" {
                             if active_torture_rooms.contains(&room.id) {
                                  conversions_to_process.push((hero_id, "conversion".to_string(), true));
                             }
                        }
                    }
                }
            }
        }

        // Apply progress
        for (hero_id, action_type, is_torture) in conversions_to_process {
             let mut completed = false;
             let mut hero_name = "".to_string();
             
             if let Some(entity) = self.entities.get_mut(hero_id) {
                 if let Some(hero) = entity.as_hero_mut() {
                     hero_name = hero.hero_id.clone();
                     let rate = if is_torture { torture_base_rate } else { skeleton_rate };
                     hero.conversion_progress += rate * dt;
                     
                     if hero.conversion_progress >= 1.0 {
                         completed = true;
                     }
                 }
             }

             if completed {
                 if is_torture {
                      // Retrieve Hero and convert
                      if let Some(entity) = self.entities.get_mut(hero_id) {
                          let pos = entity.pos;
                          if let Some(hero) = entity.as_hero_mut() {
                               hero.is_converted = true;
                               hero.is_captured = false;
                               hero.health = hero.max_health; // Heal them up
                               // Reset goal to something safe
                               hero.current_goal = crate::state::entities::HeroGoal::RestAtSpawn(pos);
                               self.notifications.success(format!("{} converted to your side!", hero_name));
                          }
                      }
                 } else {
                      // Skeleton time
                      // Remove hero, spawn skeleton
                      let pos = if let Some(e) = self.entities.get(hero_id) { e.pos } else { TilePos::new(0,0) }; // Should be valid
                      self.entities.remove(hero_id);
                      
                      if let Some(monster_data) = game_data.monsters.get("skeleton") {
                            let creature_state = crate::state::entities::CreatureState::new(
                                "skeleton".to_string(),
                                1,
                                monster_data.stats.health,
                                monster_data.stats.mana,
                            );
                            self.entities.spawn_creature(pos, creature_state);
                            self.notifications.success("Captured hero rotted into a Skeleton!");
                      }
                 }
             }
        }
    }

    /// Process hero digging logic with flattened control flow
    /// Returns (carved_wall_pos, should_move)
    fn process_hero_digging(&mut self, hero_id: EntityId, game_data: &GameData, dt: f32) -> (Option<TilePos>, bool) {
        // First, gather the info we need without holding mutable borrows
        let (next_pos, can_dig) = {
            let entity = match self.entities.get(hero_id) {
                Some(e) => e,
                None => return (None, true),
            };
            let hero_state = match entity.as_hero() {
                Some(h) => h,
                None => return (None, true),
            };
            let path = match &hero_state.current_path {
                Some(p) => p,
                None => return (None, true),
            };
            let next_pos = match path.first() {
                Some(&p) => p,
                None => return (None, true),
            };
            (next_pos, hero_state.can_dig)
        };

        // Check tile properties without holding entity borrow
        let is_wall = self.tile_blocks_movement(next_pos, game_data);
        let is_hero_wall = self.dungeon.get_tile(next_pos).map(|t| t.tile_type == "hero_wall").unwrap_or(false);
        
        let is_diggable = self.dungeon.get_tile(next_pos)
            .and_then(|t| game_data.tiles.get(&t.tile_type))
            .map(|td| td.diggable)
            .unwrap_or(false);

        // Now apply changes with mutable borrow
        let entity = match self.entities.get_mut(hero_id) {
            Some(e) => e,
            None => return (None, true),
        };
        let hero_state = match entity.as_hero_mut() {
            Some(h) => h,
            None => return (None, true),
        };

        if !is_wall {
            hero_state.is_digging = false;
            hero_state.dig_timer = 0.0;
            return (None, true);
        }

        if !can_dig {
            hero_state.current_path = None;
            return (None, true);
        }

        // PREVENT DIGGING IF TILE IS NOT DIGGABLE (e.g. Dungeon Heart, Bedrock)
        if !is_diggable || is_hero_wall {
            hero_state.current_path = None;
            hero_state.is_digging = false;
            return (None, true);
        }

        hero_state.is_digging = true;
        hero_state.dig_timer += dt;

        if hero_state.dig_timer >= hero_state.max_dig_time {
            hero_state.dig_timer = 0.0;
            hero_state.is_digging = false;
            return (Some(next_pos), false);
        }

        (None, false)
    }

    /// Check if a tile blocks movement
    fn tile_blocks_movement(&self, pos: TilePos, game_data: &GameData) -> bool {
        let tile = match self.dungeon.get_tile(pos) {
            Some(t) => t,
            None => return false,
        };
        game_data.tiles.get(&tile.tile_type).map(|td| td.blocks_movement).unwrap_or(false)
    }

    /// Check for victory or defeat conditions
    pub fn check_game_over_conditions(&mut self) {
        if self.game_over {
            return;
        }

        // Check Victory: Hero Base destroyed (All buildings)
        if self.hero_base.enabled && self.hero_base.is_defeated() {
            self.game_over = true;
            self.victory = true;
            eprintln!("VICTORY! All Hero Buildings destroyed!");
            self.notifications.success("VICTORY! All hero buildings have been destroyed!");
        }

        // Check Defeat: Dungeon Heart destroyed
        if self.player.dungeon_heart_health <= 0.0 {
            self.game_over = true;
            self.victory = false;
            eprintln!("DEFEAT! Dungeon Heart destroyed!");
            self.notifications.danger("DEFEAT! Your Dungeon Heart was destroyed!");
        }
    }
}
