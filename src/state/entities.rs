//! Entity management system
//! Handles creatures, heroes, and their runtime state

use crate::state::tile_state::TilePos;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for an entity
pub type EntityId = usize;

/// Main entity container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub entity_type: EntityType,
    pub pos: TilePos,
    #[serde(skip, default = "default_visual_pos")]
    pub visual_pos: (f32, f32),
    #[serde(skip, default = "default_damage_time")]
    pub last_damage_time: f32,
}

fn default_damage_time() -> f32 {
    -100.0
}

fn default_visual_pos() -> (f32, f32) {
    (0.0, 0.0)
}

impl Entity {
    /// Create a new creature entity
    pub fn new_creature(id: EntityId, pos: TilePos, creature_state: CreatureState) -> Self {
        Self {
            id,
            entity_type: EntityType::Creature(creature_state),
            pos,
            visual_pos: (pos.x as f32, pos.y as f32),
            last_damage_time: -100.0,
        }
    }

    /// Create a new hero entity
    pub fn new_hero(id: EntityId, pos: TilePos, hero_state: HeroState) -> Self {
        Self {
            id,
            entity_type: EntityType::Hero(hero_state),
            pos,
            visual_pos: (pos.x as f32, pos.y as f32),
            last_damage_time: -100.0,
        }
    }

    /// Create a new structure entity
    pub fn new_structure(id: EntityId, pos: TilePos, structure_state: StructureState) -> Self {
        Self {
            id,
            entity_type: EntityType::Structure(structure_state),
            visual_pos: (pos.x as f32, pos.y as f32),
            pos,
            last_damage_time: -100.0,
        }
    }

    /// Get creature state if this is a creature
    pub fn as_creature(&self) -> Option<&CreatureState> {
        match &self.entity_type {
            EntityType::Creature(state) => Some(state),
            _ => None,
        }
    }

    /// Get mutable creature state if this is a creature
    pub fn as_creature_mut(&mut self) -> Option<&mut CreatureState> {
        match &mut self.entity_type {
            EntityType::Creature(state) => Some(state),
            _ => None,
        }
    }

    /// Get hero state if this is a hero
    pub fn as_hero(&self) -> Option<&HeroState> {
        match &self.entity_type {
            EntityType::Hero(state) => Some(state),
            _ => None,
        }
    }

    /// Get mutable hero state if this is a hero
    pub fn as_hero_mut(&mut self) -> Option<&mut HeroState> {
        match &mut self.entity_type {
            EntityType::Hero(state) => Some(state),
            _ => None,
        }
    }

    /// Check if entity is alive
    pub fn is_alive(&self) -> bool {
        match &self.entity_type {
            EntityType::Creature(state) => state.health > 0.0,
            EntityType::Hero(state) => state.health > 0.0,
            EntityType::Structure(state) => state.health > 0.0,
            EntityType::ResourcePile(_) => true,
        }
    }
}

/// Type of entity (creature or hero)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Creature(CreatureState),
    Hero(HeroState),
    Structure(StructureState),
    ResourcePile(ResourcePileState),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityCategory {
    Monster,
    Hero,
}

/// Runtime state for a static structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureState {
    pub building_id: String, // "town_hall", "barracks"
    pub health: f32,
    pub max_health: f32,
}

impl StructureState {
    pub fn new(building_id: String, max_health: f32) -> Self {
        Self {
            building_id,
            health: max_health,
            max_health,
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
    }
}

/// Runtime state for a resource pile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePileState {
    pub resource_type: String, // "gold"
    pub amount: i32,
}

impl ResourcePileState {
    pub fn new(resource_type: String, amount: i32) -> Self {
        Self {
            resource_type,
            amount,
        }
    }
}

/// Runtime state for a creature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureState {
    /// Reference to creature data ID (e.g., "imp", "goblin")
    pub creature_id: String,

    /// Visual variation seed for unique appearance
    pub visual_seed: u64,

    /// Current level (1-based)
    pub level: u32,

    /// Current health (0.0 = dead)
    pub health: f32,

    /// Maximum health at this level
    pub max_health: f32,

    /// Current mana
    pub mana: f32,

    /// Maximum mana
    pub max_mana: f32,

    /// Current experience points
    pub experience: f32,

    /// Experience required for next level
    pub max_experience: f32,

    /// Timer for training XP ticks
    pub training_timer: f32,

    /// Needs tracking (0-100, higher = more satisfied)
    pub needs: HashMap<String, f32>,

    /// Overall mood (0-100, higher = happier)
    pub mood: f32,

    /// Current task being performed
    pub current_task: Option<Task>,

    /// Time spent on current task
    pub task_time: f32,

    /// Gold carried by this creature
    pub gold_carried: i32,

    /// Whether creature is angry (affects behavior)
    pub is_angry: bool,

    /// Whether creature is considering desertion
    pub is_deserting: bool,

    /// Last time creature was slapped (for cooldown)
    pub last_slapped: f32,

    /// Active status effects
    pub status_effects: Vec<StatusEffect>,

    /// Current path being followed
    pub current_path: Option<Vec<TilePos>>,

    /// Movement speed (tiles per second)
    pub movement_speed: f32,

    /// Time accumulator for movement
    pub move_timer: f32,

    /// Time accumulator for work production
    pub work_timer: f32,
}

impl CreatureState {
    /// Create a new creature state from data
    pub fn new(
        creature_id: String,
        level: u32,
        max_health: f32,
        max_mana: f32,
        visual_seed: u64,
    ) -> Self {
        Self {
            creature_id,
            visual_seed,
            level,
            health: max_health,
            max_health,
            mana: max_mana,
            max_mana,
            experience: 0.0,
            max_experience: 100.0 * (level as f32), // 100 XP per level base
            training_timer: 0.0,
            needs: {
                let mut m = HashMap::new();
                m.insert("sleep".to_string(), 100.0);
                m.insert("food".to_string(), 100.0);
                m.insert("gold".to_string(), 100.0);
                m
            },
            mood: 70.0, // Start at decent mood
            current_task: None,
            task_time: 0.0,
            gold_carried: 0,
            is_angry: false,
            is_deserting: false,
            last_slapped: 0.0,
            status_effects: Vec::new(),
            current_path: None,
            movement_speed: 2.0, // 2 tiles per second default
            move_timer: 0.0,
            work_timer: 0.0,
        }
    }

    /// Get the value of a specific need (0-100)
    pub fn get_need(&self, need_name: &str) -> f32 {
        self.needs.get(need_name).copied().unwrap_or(50.0)
    }

    /// Set a specific need value (clamped to 0-100)
    pub fn set_need(&mut self, need_name: String, value: f32) {
        let clamped = value.clamp(0.0, 100.0);
        self.needs.insert(need_name, clamped);
    }

    /// Get the most urgent need (lowest value)
    pub fn get_most_urgent_need(&self) -> Option<(String, f32)> {
        self.needs
            .iter()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(name, value)| (name.clone(), *value))
    }

    /// Damage creature by amount
    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
    }
}

/// Runtime state for a hero
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroState {
    /// Reference to hero data ID (e.g., "knight", "archer")
    pub hero_id: String,

    /// Visual variation seed for unique appearance
    pub visual_seed: u64,

    /// Current level
    pub level: u32,

    /// Current health
    pub health: f32,

    /// Maximum health
    pub max_health: f32,

    /// Current mana
    pub mana: f32,

    /// Maximum mana
    pub max_mana: f32,

    /// Current goal being pursued
    pub current_goal: HeroGoal,

    /// Target position for current goal
    pub target_pos: Option<TilePos>,

    /// Target room for current goal
    pub target_room_id: Option<usize>,

    /// Gold stolen so far
    pub gold_stolen: i32,

    /// Creatures killed so far
    pub kills: u32,

    /// Whether hero is fleeing
    pub is_fleeing: bool,

    /// Active status effects
    pub status_effects: Vec<StatusEffect>,

    /// Current path being followed
    pub current_path: Option<Vec<TilePos>>,

    /// Movement speed (tiles per second)
    pub movement_speed: f32,

    /// Time accumulator for movement
    pub move_timer: f32,

    /// Spawn position (for retreating/resting)
    pub spawn_pos: TilePos,

    // Digging state
    pub is_digging: bool,
    pub dig_timer: f32,
    pub max_dig_time: f32, // Time to dig one wall
    pub can_dig: bool,     // Tunneled capability

    // Prison/capture state
    /// Whether this hero is captured and being converted
    pub is_captured: bool,
    /// Conversion progress (0.0 to 1.0, at 1.0 hero becomes a creature)
    pub conversion_progress: f32,

    // Wave attack state
    /// Whether this hero stays at base as a defender (true) or joins attack waves (false)
    pub is_defender: bool,
    /// Which wave number this hero is assigned to (0 = not yet assigned)
    pub wave_assigned: u32,

    /// Whether this hero has been converted to the dungeon faction
    pub is_converted: bool,
}

impl HeroState {
    /// Create a new hero state
    pub fn new(
        hero_id: String,
        level: u32,
        max_health: f32,
        max_mana: f32,
        spawn_pos: TilePos,
        dig_time: f32,
        visual_seed: u64,
    ) -> Self {
        let can_dig = true; // Allow all heroes to try and breach walls if path is blocked

        Self {
            hero_id,
            visual_seed,
            level,
            health: max_health,
            max_health,
            mana: max_mana,
            max_mana,
            current_goal: HeroGoal::RestAtSpawn(spawn_pos), // Start by resting/grouping up
            target_pos: Some(spawn_pos),                    // Start with target at spawn
            target_room_id: None,
            gold_stolen: 0,
            kills: 0,
            is_fleeing: false,
            status_effects: Vec::new(),
            current_path: None,
            movement_speed: 1.5, // 1.5 tiles per second default
            move_timer: 0.0,
            spawn_pos,
            is_digging: false,
            dig_timer: 0.0,
            max_dig_time: dig_time,
            can_dig,
            is_captured: false,
            conversion_progress: 0.0,
            is_defender: false, // Assigned later by spawner
            wave_assigned: 0,   // Assigned when wave launches
            is_converted: false,
        }
    }

    /// Damage hero by amount
    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
    }

    /// Check if hero should retreat based on health
    pub fn should_retreat(&self, retreat_threshold: f32) -> bool {
        (self.health / self.max_health) < retreat_threshold
    }
}

/// Creature task types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Task {
    /// Idle/wandering
    Idle,

    /// Digging a specific tile
    Dig(TilePos),

    /// Working in a room at a specific slot
    Work(usize, TilePos), // room_id, target_pos

    /// Sleeping in lair
    Sleep(usize), // room_id

    /// Eating in hatchery
    Eat(usize), // room_id

    /// Training in training room
    Train(usize), // room_id

    /// Researching in library
    Research(usize), // room_id

    /// Depositing gold in treasury
    DepositGold(usize), // room_id

    /// Moving to a position
    MoveTo(TilePos),

    /// Attacking an entity
    Attack(EntityId),

    /// Fleeing from combat
    Flee,

    /// Collecting wages from treasury
    CollectWages(usize), // room_id

    /// Claiming a tile
    ClaimTile(TilePos),

    /// Picking up a resource pile
    PickupResource(EntityId),
}

impl Task {
    /// Get the task type as a string for matching against AI preferences
    pub fn task_type(&self) -> &str {
        match self {
            Task::Idle => "idle",
            Task::Dig(_) => "dig",
            Task::Work(_, _) => "work",
            Task::Sleep(_) => "sleep",
            Task::Eat(_) => "eat",
            Task::Train(_) => "train",
            Task::Research(_) => "research",
            Task::DepositGold(_) => "deposit_gold",
            Task::CollectWages(_) => "collect_wages",
            Task::MoveTo(_) => "move",
            Task::Attack(_) => "attack",
            Task::Flee => "flee",
            Task::ClaimTile(_) => "claim_tile",
            Task::PickupResource(_) => "pickup_resource",
        }
    }
}

/// Hero goal types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeroGoal {
    /// Primary goal: destroy the dungeon heart
    DestroyHeart,

    /// Steal a specific amount of gold
    StealGold(i32),

    /// Kill a specific number of creatures
    KillCreatures(u32),

    /// Sabotage a specific room
    SabotageRoom(usize),

    /// Explore the dungeon (reveal fog)
    Explore,

    /// Rest at spawn location
    RestAtSpawn(TilePos),

    /// Retreat to entrance
    Retreat,
}

/// Status effect on an entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub effect_type: String,
    pub duration: f32,
    pub strength: f32,
}

/// Entity manager for tracking all entities in the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityManager {
    entities: HashMap<EntityId, Entity>,
    next_id: EntityId,
}

impl EntityManager {
    /// Create a new entity manager
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_id: 1,
        }
    }

    /// Spawn a new creature
    pub fn spawn_creature(&mut self, pos: TilePos, creature_state: CreatureState) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;

        let entity = Entity::new_creature(id, pos, creature_state);
        self.entities.insert(id, entity);
        id
    }

    /// Spawn a new hero
    pub fn spawn_hero(&mut self, pos: TilePos, hero_state: HeroState) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;

        let entity = Entity::new_hero(id, pos, hero_state);
        self.entities.insert(id, entity);
        id
    }

    /// Get an entity by ID
    pub fn get(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id)
    }

    /// Spawn a new structure
    pub fn spawn_structure(&mut self, pos: TilePos, structure_state: StructureState) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;

        let entity = Entity::new_structure(id, pos, structure_state);
        self.entities.insert(id, entity);
        id
    }

    /// Spawn a new resource pile
    pub fn spawn_resource_pile(&mut self, pos: TilePos, state: ResourcePileState) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;

        let entity = Entity {
            id,
            entity_type: EntityType::ResourcePile(state),
            pos,
            visual_pos: (pos.x as f32, pos.y as f32),
            last_damage_time: -100.0,
        };
        self.entities.insert(id, entity);
        id
    }

    /// Get a mutable entity by ID
    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    /// Remove an entity (death, despawn, etc.)
    pub fn remove(&mut self, id: EntityId) -> Option<Entity> {
        self.entities.remove(&id)
    }

    /// Count all tracked entities.
    pub fn count(&self) -> usize {
        self.entities.len()
    }

    /// Count all creature entities.
    pub fn count_creatures(&self) -> usize {
        self.entities
            .values()
            .filter(|entity| matches!(entity.entity_type, EntityType::Creature(_)))
            .count()
    }

    /// Remove all entities whose health has reached zero.
    pub fn remove_dead(&mut self) -> usize {
        let before = self.entities.len();
        self.entities.retain(|_, entity| entity.is_alive());
        before - self.entities.len()
    }

    /// Get all entities
    pub fn all(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    /// Get all mutable entities
    pub fn all_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut()
    }

    /// Get the entities map mutably (for combat resolution)
    pub fn entities_mut(&mut self) -> &mut HashMap<EntityId, Entity> {
        &mut self.entities
    }

    /// Get the entities map immutably
    pub fn entities(&self) -> &HashMap<EntityId, Entity> {
        &self.entities
    }

    /// Get all creatures
    pub fn creatures(&self) -> impl Iterator<Item = (EntityId, &CreatureState)> {
        self.entities
            .iter()
            .filter_map(|(id, entity)| entity.as_creature().map(|creature| (*id, creature)))
    }

    /// Get all heroes
    pub fn heroes(&self) -> impl Iterator<Item = (EntityId, &HeroState)> {
        self.entities
            .iter()
            .filter_map(|(id, entity)| entity.as_hero().map(|hero| (*id, hero)))
    }

    /// Get entities at a specific position
    pub fn at_position(&self, pos: TilePos) -> impl Iterator<Item = &Entity> {
        self.entities.values().filter(move |e| e.pos == pos)
    }

    /// Get mutable entities at a specific position
    pub fn at_position_mut(&mut self, pos: TilePos) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut().filter(move |e| e.pos == pos)
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_manager_spawn() {
        let mut manager = EntityManager::new();
        let creature = CreatureState::new("imp".to_string(), 1, 100.0, 20.0, 0);
        let id = manager.spawn_creature(TilePos::new(5, 5), creature);

        assert_eq!(manager.count(), 1);
        assert_eq!(manager.count_creatures(), 1);

        let entity = manager.get(id).unwrap();
        assert_eq!(entity.pos, TilePos::new(5, 5));
    }

    #[test]
    fn test_creature_needs() {
        let mut creature = CreatureState::new("goblin".to_string(), 1, 120.0, 0.0, 0);
        creature.set_need("sleep".to_string(), 80.0);
        creature.set_need("food".to_string(), 30.0);

        assert_eq!(creature.get_need("sleep"), 80.0);
        assert_eq!(creature.get_need("food"), 30.0);

        let (need, value) = creature.get_most_urgent_need().unwrap();
        assert_eq!(need, "food");
        assert_eq!(value, 30.0);
    }

    #[test]
    fn test_remove_dead() {
        let mut manager = EntityManager::new();

        let mut creature1 = CreatureState::new("imp".to_string(), 1, 100.0, 20.0, 0);
        creature1.health = 0.0; // Dead

        let creature2 = CreatureState::new("goblin".to_string(), 1, 120.0, 0.0, 0);
        // Alive

        manager.spawn_creature(TilePos::new(0, 0), creature1);
        manager.spawn_creature(TilePos::new(1, 1), creature2);

        assert_eq!(manager.count(), 2);

        let removed = manager.remove_dead();
        assert_eq!(removed, 1);
        assert_eq!(manager.count(), 1);
    }
}
