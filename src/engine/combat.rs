//! Combat resolution system
//!
//! Handles damage calculation, status effects, and combat outcomes
//! between creatures and heroes.

use crate::data::GameData;
use crate::state::entities::{Entity, EntityId, StatusEffect};
use crate::state::tile_state::TilePos;
use std::collections::HashMap;

/// Result of a combat tick
#[derive(Debug, Clone)]
pub struct CombatResult {
    pub damage_dealt: f32,
    pub status_applied: Vec<StatusEffect>,
    pub defender_died: bool,
    pub projectile_spawned: Option<(String, f32)>, // (type, damage)
}

/// Combat statistics for an entity
#[derive(Debug, Clone)]
pub struct CombatStats {
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
    pub attack_type: String,
    pub damage_range: [f32; 2],
    pub attack_speed: f32,
    pub resistances: HashMap<String, f32>,
    pub level: u32,
    pub abilities: Vec<String>,
}

/// Resolve one tick of combat between two entities
///
/// `defender_room_defense` is the `creature_defense_modifier` of the room the
/// defender is standing in (see `room_validator::room_defense_at`). It is
/// passed in rather than looked up here so combat stays a pure function of its
/// arguments, the same way `calculate_mood` takes its room modifier.
pub fn resolve_combat_tick(
    attacker: &Entity,
    defender: &Entity,
    dt: f32,
    game_data: &GameData,
    defender_room_defense: f32,
) -> CombatResult {
    if is_stunned(attacker) {
        return CombatResult {
            damage_dealt: 0.0,
            status_applied: Vec::new(),
            defender_died: false,
            projectile_spawned: None,
        };
    }

    let attacker_stats = extract_combat_stats(attacker, game_data);
    let mut defender_stats = extract_combat_stats(defender, game_data);
    defender_stats.defense =
        fortified_defense(defender, defender_stats.defense, defender_room_defense);

    // Calculate if attack hits this tick
    let attacks_per_second = attacker_stats.attack_speed;
    let attack_chance = attacks_per_second * dt;

    if macroquad_toolkit::rng::gen_range(0.0f32, 1.0) > attack_chance {
        // No attack this tick
        return CombatResult {
            damage_dealt: 0.0,
            status_applied: Vec::new(),
            defender_died: false,
            projectile_spawned: None,
        };
    }

    // Calculate damage
    let base_damage = calculate_damage(&attacker_stats, &defender_stats, game_data);
    let actual_damage = base_damage.max(0.0);

    // Determine if we should spawn a projectile or apply instant damage
    let is_melee = attacker_stats.attack_type == "melee";

    if !is_melee && actual_damage > 0.0 {
        // Ranged/Magic attack - spawn projectile. Status effects are rolled now (attacker
        // abilities are known now) but only actually applied once the projectile lands, in
        // apply_projectile_impact.
        return CombatResult {
            damage_dealt: 0.0, // Defer damage
            status_applied: generate_status_effects(&attacker_stats.abilities, game_data),
            defender_died: false,
            projectile_spawned: Some((attacker_stats.attack_type.clone(), actual_damage)),
        };
    }

    // Apply damage instantly (Melee)
    let defender_would_die = defender.is_alive() && (defender_stats.health - actual_damage) <= 0.0;

    let status_effects = generate_status_effects(&attacker_stats.abilities, game_data);

    // Log combat using all stats for debug
    if actual_damage > 0.0 {
        eprintln!(
            "Combat: {} (Lvl {}) hit {} (Lvl {}) for {:.1} damage (Type: {})",
            get_type_name(&attacker.entity_type),
            attacker_stats.level,
            get_type_name(&defender.entity_type),
            defender_stats.level,
            actual_damage,
            attacker_stats.attack_type
        );
    }

    CombatResult {
        damage_dealt: actual_damage,
        status_applied: status_effects,
        defender_died: defender_would_die,
        projectile_spawned: None,
    }
}

/// True if the entity has an active "stun" status effect (can't attack this tick).
fn is_stunned(entity: &Entity) -> bool {
    let status_effects = match &entity.entity_type {
        crate::state::entities::EntityType::Creature(state) => &state.status_effects,
        crate::state::entities::EntityType::Hero(state) => &state.status_effects,
        crate::state::entities::EntityType::Structure(_)
        | crate::state::entities::EntityType::ResourcePile(_) => return false,
    };
    status_effects.iter().any(|e| e.effect_type == "stun")
}

fn get_type_name(entity_type: &crate::state::entities::EntityType) -> String {
    match entity_type {
        crate::state::entities::EntityType::Creature(c) => c.creature_id.clone(),
        crate::state::entities::EntityType::Hero(h) => h.hero_id.clone(),
        crate::state::entities::EntityType::Structure(s) => s.building_id.clone(),
        crate::state::entities::EntityType::ResourcePile(_) => "gold_pile".to_string(),
    }
}

/// Extract combat stats from an entity
pub fn extract_combat_stats(entity: &Entity, game_data: &GameData) -> CombatStats {
    match &entity.entity_type {
        crate::state::entities::EntityType::Creature(creature_state) => {
            let creature_data = game_data
                .monsters
                .get(&creature_state.creature_id)
                .expect("Creature data not found");

            // Calculate level bonuses
            let level_multiplier = 1.0
                + (creature_state.level - 1) as f32
                    * game_data.config.combat.creature_level_multiplier;

            // Trait-driven attack/defense multipliers (data-driven; see traits.json)
            let trait_data: Vec<_> = creature_data
                .traits
                .iter()
                .filter_map(|trait_id| game_data.traits.get(trait_id))
                .collect();
            let attack_multiplier: f32 = trait_data.iter().map(|t| t.attack_multiplier).product();
            let defense_multiplier: f32 = trait_data.iter().map(|t| t.defense_multiplier).product();

            CombatStats {
                health: creature_state.health,
                attack: creature_data.stats.attack * level_multiplier * attack_multiplier,
                defense: creature_data.stats.defense * level_multiplier * defense_multiplier,
                attack_type: creature_data.combat.attack_type.clone(),
                damage_range: creature_data.combat.damage_range,
                attack_speed: creature_data.combat.attack_speed,
                resistances: creature_data.combat.resistances.clone(),
                level: creature_state.level,
                abilities: creature_data.combat.abilities.clone(),
            }
        }
        crate::state::entities::EntityType::Hero(hero_state) => {
            let hero_data = game_data
                .heroes
                .get(&hero_state.hero_id)
                .expect("Hero data not found");

            // Calculate level bonuses
            let level_multiplier =
                1.0 + (hero_state.level - 1) as f32 * game_data.config.combat.hero_level_multiplier;

            CombatStats {
                health: hero_state.health,
                attack: hero_data.stats.attack * level_multiplier,
                defense: hero_data.stats.defense * level_multiplier,
                attack_type: hero_data.combat.attack_type.clone(),
                damage_range: hero_data.combat.damage_range,
                attack_speed: hero_data.combat.attack_speed,
                resistances: hero_data.combat.resistances.clone(),
                level: hero_state.level,
                // Hero abilities (HeroAbilityData) have a richer trigger/effect shape than
                // simple on-hit procs and aren't wired into combat yet; see TODO.md.
                abilities: Vec::new(),
            }
        }
        crate::state::entities::EntityType::Structure(structure_state) => CombatStats {
            health: structure_state.health,
            attack: 0.0,
            defense: game_data.config.combat.building_base_defense,
            attack_type: "none".to_string(),
            damage_range: [0.0, 0.0],
            attack_speed: game_data.config.combat.building_attack_speed,
            resistances: HashMap::new(),
            level: 1,
            abilities: Vec::new(),
        },
        crate::state::entities::EntityType::ResourcePile(_) => CombatStats {
            health: 1.0,
            attack: 0.0,
            defense: 0.0,
            attack_type: "none".to_string(),
            damage_range: [0.0, 0.0],
            attack_speed: 1.0,
            resistances: HashMap::new(),
            level: 1,
            abilities: Vec::new(),
        },
    }
}

/// A defender's effective defence once the room they stand in is accounted for.
///
/// A fortified room protects the keeper's own creatures, and only those: a hero
/// who fights their way into a gatehouse gets no benefit from the keeper's
/// stonework, and structures have their own flat defence.
pub fn fortified_defense(defender: &Entity, defense: f32, room_defense: f32) -> f32 {
    match defender.entity_type {
        crate::state::entities::EntityType::Creature(_) => defense + room_defense,
        _ => defense,
    }
}

/// Calculate damage from attacker to defender
pub fn calculate_damage(
    attacker: &CombatStats,
    defender: &CombatStats,
    game_data: &GameData,
) -> f32 {
    // Base damage from attacker's range
    let base_damage = if attacker.damage_range[1] > attacker.damage_range[0] {
        let range = attacker.damage_range[1] - attacker.damage_range[0];
        attacker.damage_range[0] + macroquad_toolkit::rng::gen_range(0.0f32, 1.0) * range
    } else {
        attacker.damage_range[0]
    };

    // Add attack stat bonus
    let attack_damage = base_damage + attacker.attack * game_data.config.combat.attack_stat_bonus;

    // Apply defense reduction
    let defense_reduction = defender.defense * game_data.config.combat.defense_reduction;
    let pre_resist_damage = (attack_damage - defense_reduction).max(0.0);

    // Apply elemental resistances
    let resistance_multiplier = calculate_resistance_multiplier(attacker, defender);
    let final_damage = pre_resist_damage * resistance_multiplier;

    // Ensure minimum damage of 1.0 so battles don't stalemate
    final_damage.max(1.0)
}

/// Calculate resistance multiplier based on attack type and defender resistances
fn calculate_resistance_multiplier(attacker: &CombatStats, defender: &CombatStats) -> f32 {
    let base_resistance = defender
        .resistances
        .get(&attacker.attack_type)
        .copied()
        .unwrap_or(0.0);

    // Convert resistance percentage to multiplier
    // Positive resistance reduces damage, negative increases
    1.0 - (base_resistance / 100.0)
}

/// Roll each of the attacker's combat abilities against the data-driven
/// `game_data.config.status_effects.ability_effects` table, returning the status effects that
/// proc'd on this landed hit. Abilities with no entry in that table (e.g. ones that aren't a
/// poison/burn/freeze/stun proc, like a flat damage bonus) are silently skipped here.
fn generate_status_effects(abilities: &[String], game_data: &GameData) -> Vec<StatusEffect> {
    let mut effects = Vec::new();
    for ability in abilities {
        if let Some(ability_effect) = game_data.config.status_effects.ability_effects.get(ability) {
            if macroquad_toolkit::rng::gen_range(0.0f32, 1.0) < ability_effect.proc_chance {
                effects.push(StatusEffect {
                    effect_type: ability_effect.status_type.clone(),
                    duration: ability_effect.duration,
                    strength: ability_effect.strength,
                });
            }
        }
    }
    effects
}

/// Apply combat result to entities
pub fn apply_combat_result(
    result: &CombatResult,
    attacker_id: EntityId,
    defender_id: EntityId,
    entities: &mut HashMap<EntityId, Entity>,
    game_data: &GameData,
    current_time: f32,
) {
    // Apply damage to defender
    if let Some(defender) = entities.get_mut(&defender_id) {
        if result.damage_dealt > 0.0 {
            defender.last_damage_time = current_time;
        }
        match &mut defender.entity_type {
            crate::state::entities::EntityType::Creature(state) => {
                state.take_damage(result.damage_dealt);
            }
            crate::state::entities::EntityType::Hero(state) => {
                state.take_damage(result.damage_dealt);
            }
            crate::state::entities::EntityType::Structure(state) => {
                state.take_damage(result.damage_dealt);
            }
            crate::state::entities::EntityType::ResourcePile(_) => {}
        }
    }

    // Apply status effects to defender. "freeze" slows movement immediately on application;
    // combat::update_status_effects reverts the slow when the effect's duration runs out.
    if let Some(defender) = entities.get_mut(&defender_id) {
        for effect in &result.status_applied {
            match &mut defender.entity_type {
                crate::state::entities::EntityType::Creature(state) => {
                    if effect.effect_type == "freeze" && effect.strength != 0.0 {
                        state.movement_speed *= effect.strength;
                    }
                    state.status_effects.push(effect.clone());
                }
                crate::state::entities::EntityType::Hero(state) => {
                    if effect.effect_type == "freeze" && effect.strength != 0.0 {
                        state.movement_speed *= effect.strength;
                    }
                    state.status_effects.push(effect.clone());
                }
                crate::state::entities::EntityType::Structure(_) => {
                    // Structures don't take status effects yet
                }
                crate::state::entities::EntityType::ResourcePile(_) => {}
            }
        }
    }

    // Handle death and experience separately to avoid borrow issues
    let defender_died = result.defender_died;
    let victim_level = if defender_died {
        if let Some(defender) = entities.get(&defender_id) {
            match &defender.entity_type {
                crate::state::entities::EntityType::Creature(state) => state.level,
                crate::state::entities::EntityType::Hero(_) => 0, // Heroes don't give XP
                crate::state::entities::EntityType::Structure(_) => {
                    game_data.config.combat.building_xp_reward as u32
                }
                crate::state::entities::EntityType::ResourcePile(_) => 0,
            }
        } else {
            0
        }
    } else {
        0
    };

    // Award experience to attacker if defender died
    if defender_died && victim_level > 0 {
        if let Some(attacker) = entities.get_mut(&attacker_id) {
            award_experience(attacker, victim_level, game_data);
        }
    }
}

/// Award experience to attacker for killing a creature
fn award_experience(attacker: &mut Entity, victim_level: u32, game_data: &GameData) {
    let exp_gain = victim_level * game_data.config.combat.xp_per_victim_level as u32;

    match &mut attacker.entity_type {
        crate::state::entities::EntityType::Creature(state) => {
            state.experience += exp_gain as f32;

            // Check for level up
            // Use same max_experience field from CreatureState
            if state.experience >= state.max_experience {
                level_up_creature(state, game_data);
            }
        }
        crate::state::entities::EntityType::Hero(_state) => {
            // Heroes don't level up in combat in this simplified system
            // Could be extended to award hero XP
        }
        crate::state::entities::EntityType::Structure(_) => {}
        crate::state::entities::EntityType::ResourcePile(_) => {}
    }
}

/// Apply projectile impact
pub fn apply_projectile_impact(
    impact: &crate::state::projectiles::Impact,
    entities: &mut crate::state::entities::EntityManager,
    game_data: &GameData,
    current_time: f32,
) {
    // Apply damage to defender
    if let Some(defender) = entities.get_mut(impact.defender_id) {
        // Use apply_combat_result mechanics but simplified
        let damage = impact.damage;

        if damage > 0.0 {
            defender.last_damage_time = current_time;
        }

        match &mut defender.entity_type {
            crate::state::entities::EntityType::Creature(state) => {
                state.take_damage(damage);
            }
            crate::state::entities::EntityType::Hero(state) => {
                state.take_damage(damage);
            }
            crate::state::entities::EntityType::Structure(state) => {
                state.take_damage(damage);
            }
            crate::state::entities::EntityType::ResourcePile(_) => {}
        }
    }

    // XP Awarding needs to happen safely.
    // Check if defender died
    let defender_dead_and_level = if let Some(defender) = entities.get(impact.defender_id) {
        if !defender.is_alive() {
            match &defender.entity_type {
                crate::state::entities::EntityType::Creature(state) => Some(state.level),
                crate::state::entities::EntityType::Structure(_) => {
                    Some(game_data.config.combat.building_xp_reward as u32)
                }
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(level) = defender_dead_and_level {
        if let Some(attacker) = entities.get_mut(impact.attacker_id) {
            award_experience(attacker, level, game_data);
        }
    }
}

/// Level up a creature
fn level_up_creature(state: &mut crate::state::entities::CreatureState, game_data: &GameData) {
    if state.level >= game_data.config.combat.max_creature_level {
        return; // Max level cap
    }

    state.level += 1;
    state.experience = 0.0; // Reset for next level
    state.max_experience *= game_data.config.combat.xp_requirement_multiplier; // Scaling XP requirement

    // Increase stats (simplified - in full game would use progression data)
    state.max_health += game_data.config.combat.level_up_health_bonus;
    state.health = state.max_health; // Full heal on level up

    // Could also increase attack, defense, etc.
}

// Placeholder - will act validation in next step

/// Get the attack range for a given attack type
pub fn get_attack_range(attack_type: &str, game_data: &GameData) -> i32 {
    match attack_type {
        "melee" => game_data.config.combat_ranges.melee,
        "ranged" => game_data.config.combat_ranges.ranged,
        "magic" => game_data.config.combat_ranges.magic,
        _ => game_data.config.combat_ranges.melee,
    }
}

/// Calculate Manhattan distance between two positions (public for use elsewhere)
pub fn manhattan_distance(a: TilePos, b: TilePos) -> i32 {
    calculate_manhattan_distance(a, b)
}

/// Check if two entities are in combat range and have line of sight
pub fn in_combat_range(
    attacker_pos: TilePos,
    defender_pos: TilePos,
    attack_type: &str,
    dungeon_grid: &[Vec<crate::state::tile_state::TileState>],
    game_data: &GameData,
) -> bool {
    let distance = calculate_manhattan_distance(attacker_pos, defender_pos);
    let max_range = get_attack_range(attack_type, game_data);

    if distance > max_range {
        return false;
    }

    // Line of Sight Check
    // Melee always hits if adjacent (distance <= 1)
    if distance <= 1 {
        return true;
    }

    check_line_of_sight(attacker_pos, defender_pos, dungeon_grid, game_data)
}

/// Calculate Manhattan distance between positions
fn calculate_manhattan_distance(a: TilePos, b: TilePos) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

/// Simple line of sight check using Bresenham's algorithm
fn check_line_of_sight(
    start: TilePos,
    end: TilePos,
    grid: &[Vec<crate::state::tile_state::TileState>],
    game_data: &GameData,
) -> bool {
    let (x0, y0, x1, y1) = (start.x, start.y, end.x, end.y);
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let (mut x, mut y) = (x0, y0);

    loop {
        if x == x1 && y == y1 {
            return true;
        }

        if !(x == x0 && y == y0) && tile_blocks_vision(x, y, grid, game_data) {
            return false;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

/// Check if a tile at the given coordinates blocks vision
fn tile_blocks_vision(
    x: i32,
    y: i32,
    grid: &[Vec<crate::state::tile_state::TileState>],
    game_data: &GameData,
) -> bool {
    let row = match grid.get(y as usize) {
        Some(r) => r,
        None => return false,
    };
    let tile = match row.get(x as usize) {
        Some(t) => t,
        None => return false,
    };
    game_data
        .tiles
        .get(&tile.tile_type)
        .map(|td| td.blocks_vision)
        .unwrap_or(false)
}

/// Get detection range for an entity (how far they can see enemies to engage)
pub fn get_detection_range(entity: &Entity, game_data: &GameData) -> i32 {
    match &entity.entity_type {
        crate::state::entities::EntityType::Creature(state) => game_data
            .monsters
            .get(&state.creature_id)
            .map(|data| data.stats.sight_radius as i32)
            .unwrap_or(8),
        crate::state::entities::EntityType::Hero(state) => game_data
            .heroes
            .get(&state.hero_id)
            .map(|data| data.stats.sight_radius as i32)
            .unwrap_or(8),
        _ => 8,
    }
}

/// Find potential combat targets for an entity within detection range, sorted by priority (heroes > buildings) then distance
/// This uses DETECTION range (sight), not attack range - creatures will chase enemies they can see
pub fn find_combat_targets(
    entity: &Entity,
    entities: &HashMap<EntityId, Entity>,
    dungeon: &crate::state::dungeon::Dungeon,
    game_data: &GameData,
) -> Vec<EntityId> {
    // (EntityId, Distance, Priority)
    // Priority: 0 = Hero/Creature (High), 1 = Structure (Low)
    let mut targets: Vec<(EntityId, i32, u8)> = Vec::new();

    let detection_range = get_detection_range(entity, game_data);

    for (other_id, other_entity) in entities {
        if *other_id == entity.id {
            continue; // Don't target self
        }

        if !other_entity.is_alive() {
            continue; // Skip dead entities
        }

        // Check if entities are hostile
        if are_hostile(entity, other_entity, game_data) {
            let distance = calculate_manhattan_distance(entity.pos, other_entity.pos);

            // Check if within detection range (sight range)
            if distance <= detection_range {
                // Also check line of sight for ranged detection
                if distance <= 1
                    || check_line_of_sight(entity.pos, other_entity.pos, &dungeon.grid, game_data)
                {
                    let priority = match other_entity.entity_type {
                        crate::state::entities::EntityType::Structure(_) => 1,
                        _ => 0,
                    };
                    targets.push((*other_id, distance, priority));
                }
            }
        }
    }

    // Sort by priority (asc) then distance (asc)
    // This ensures Heroes/Creatures (0) are targeted before Structures (1)
    // Within same priority, closest target is preferred
    targets.sort_by_key(|(_, dist, priority)| (*priority, *dist));

    targets.into_iter().map(|(id, _, _)| id).collect()
}

/// Get the attack type for an entity
pub fn get_entity_attack_type(entity: &Entity, game_data: &GameData) -> String {
    match &entity.entity_type {
        crate::state::entities::EntityType::Creature(state) => game_data
            .monsters
            .get(&state.creature_id)
            .map(|data| data.combat.attack_type.clone())
            .unwrap_or_else(|| "melee".to_string()),
        crate::state::entities::EntityType::Hero(state) => game_data
            .heroes
            .get(&state.hero_id)
            .map(|data| data.combat.attack_type.clone())
            .unwrap_or_else(|| "melee".to_string()),
        crate::state::entities::EntityType::Structure(_) => "none".to_string(),
        crate::state::entities::EntityType::ResourcePile(_) => "none".to_string(),
    }
}

/// Check if two entities are hostile to each other
fn are_hostile(entity_a: &Entity, entity_b: &Entity, game_data: &GameData) -> bool {
    if matches!(
        entity_a.entity_type,
        crate::state::entities::EntityType::ResourcePile(_)
    ) || matches!(
        entity_b.entity_type,
        crate::state::entities::EntityType::ResourcePile(_)
    ) {
        return false;
    }

    if entity_a.owner.is_hostile_to(&entity_b.owner) {
        return true;
    }

    if entity_a.owner == entity_b.owner {
        return false;
    }

    let faction_a = get_faction(entity_a, game_data);
    let faction_b = get_faction(entity_b, game_data);

    match (faction_a.as_str(), faction_b.as_str()) {
        ("resource", _) | (_, "resource") => false, // Resources are neutral
        ("dungeon", "hero") => true,
        ("hero", "dungeon") => true,
        ("wild", "wild") => false, // Wild creatures don't attack each other
        ("wild", "dungeon") => true, // Wild attacks dungeon creatures
        ("wild", "hero") => true,  // Wild attacks heroes
        ("dungeon", "wild") => true, // Dungeon creatures attack wild
        ("hero", "wild") => true,  // Heroes attack wild
        ("hero", "hero") => false,
        ("dungeon", "dungeon") => false, // Friendly fire off
        _ => false,
    }
}

fn get_faction(entity: &Entity, game_data: &GameData) -> String {
    if entity.owner != crate::state::OwnerId::Neutral {
        return entity.owner.label();
    }

    match &entity.entity_type {
        crate::state::entities::EntityType::Creature(c) => game_data
            .monsters
            .get(&c.creature_id)
            .map(|m| m.faction.clone())
            .unwrap_or("dungeon".to_string()),

        crate::state::entities::EntityType::Hero(h) => {
            if h.is_converted {
                "dungeon".to_string()
            } else {
                "hero".to_string()
            }
        }
        crate::state::entities::EntityType::Structure(_) => "hero".to_string(), // Structures belong to hero faction
        crate::state::entities::EntityType::ResourcePile(_) => "resource".to_string(),
    }
}

/// Sum the poison/burn damage-per-second entries in a status effect list for this tick.
fn dot_damage(status_effects: &[StatusEffect], dt: f32) -> f32 {
    status_effects
        .iter()
        .filter(|e| e.effect_type == "poison" || e.effect_type == "burn")
        .map(|e| e.strength * dt)
        .sum()
}

/// Which movement-speed multipliers just expired and need to be divided back out.
fn expired_speed_multipliers(status_effects: &[StatusEffect]) -> Vec<f32> {
    status_effects
        .iter()
        .filter(|e| {
            e.duration <= 0.0
                && (e.effect_type == "speed_modifier" || e.effect_type == "freeze")
                && e.strength != 0.0
        })
        .map(|e| e.strength)
        .collect()
}

/// Update status effects on an entity: ticks duration down, applies poison/burn damage over
/// time, and reverts freeze/speed_modifier movement-speed changes once they expire. Stun has no
/// per-tick effect here; combat::resolve_combat_tick checks for it directly before an attack.
pub fn update_status_effects(entity: &mut Entity, dt: f32) {
    match &mut entity.entity_type {
        crate::state::entities::EntityType::Creature(state) => {
            let dot = dot_damage(&state.status_effects, dt);
            if dot > 0.0 {
                state.health = (state.health - dot).max(0.0);
            }

            for effect in &mut state.status_effects {
                effect.duration -= dt;
            }
            let expired = expired_speed_multipliers(&state.status_effects);
            state.status_effects.retain(|effect| effect.duration > 0.0);
            for multiplier in expired {
                state.movement_speed /= multiplier;
            }
        }
        crate::state::entities::EntityType::Hero(state) => {
            let dot = dot_damage(&state.status_effects, dt);
            if dot > 0.0 {
                state.health = (state.health - dot).max(0.0);
            }

            for effect in &mut state.status_effects {
                effect.duration -= dt;
            }
            let expired = expired_speed_multipliers(&state.status_effects);
            state.status_effects.retain(|effect| effect.duration > 0.0);
            for multiplier in expired {
                state.movement_speed /= multiplier;
            }
        }
        crate::state::entities::EntityType::Structure(_) => {}
        crate::state::entities::EntityType::ResourcePile(_) => {}
    }
}
