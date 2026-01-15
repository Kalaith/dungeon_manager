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
    pub attacker_died: bool,
}

/// Combat statistics for an entity
#[derive(Debug, Clone)]
pub struct CombatStats {
    pub health: f32,
    pub max_health: f32,
    pub attack: f32,
    pub defense: f32,
    pub attack_type: String,
    pub damage_range: [f32; 2],
    pub attack_speed: f32,
    pub armor_type: String,
    pub resistances: HashMap<String, f32>,
    pub level: u32,
}

/// Resolve one tick of combat between two entities
pub fn resolve_combat_tick(
    attacker: &Entity,
    defender: &Entity,
    dt: f32,
    game_data: &GameData,
) -> CombatResult {
    let attacker_stats = extract_combat_stats(attacker, game_data);
    let defender_stats = extract_combat_stats(defender, game_data);

    // Calculate if attack hits this tick
    let attacks_per_second = attacker_stats.attack_speed;
    let attack_chance = attacks_per_second * dt;

    if rand::random::<f32>() > attack_chance {
        // No attack this tick
        return CombatResult {
            damage_dealt: 0.0,
            status_applied: Vec::new(),
            defender_died: false,
            attacker_died: false,
        };
    }

    // Calculate damage
    let base_damage = calculate_damage(&attacker_stats, &defender_stats);

    // Apply damage
    let actual_damage = base_damage.max(0.0);

    // Check for death
    let defender_would_die = defender.is_alive() && (defender_stats.health - actual_damage) <= 0.0;
    
    // Check for counterattack death (use attacker_died field)
    let attacker_would_die = if defender_would_die {
        false // Dead defenders can't counterattack
    } else {
        // Small chance of counterattack death based on level difference
        let level_diff = defender_stats.level as i32 - attacker_stats.level as i32;
        level_diff > 3 && rand::random::<f32>() < 0.05
    };

    // Generate status effects (simplified)
    let status_effects = generate_status_effects(attacker, defender, game_data);
    
    // Log combat using all stats for debug
    if actual_damage > 0.0 {
        eprintln!(
            "[Combat] Lv{} {} ({}/{} HP, {}) hit Lv{} {} ({}/{} HP, {}) for {:.1} damage",
            attacker_stats.level, attacker_stats.attack_type,
            attacker_stats.health as i32, attacker_stats.max_health as i32, attacker_stats.armor_type,
            defender_stats.level, defender_stats.armor_type,
            defender_stats.health as i32, defender_stats.max_health as i32, defender_stats.armor_type,
            actual_damage
        );
    }

    CombatResult {
        damage_dealt: actual_damage,
        status_applied: status_effects,
        defender_died: defender_would_die,
        attacker_died: attacker_would_die,
    }
}

/// Extract combat stats from an entity
pub fn extract_combat_stats(entity: &Entity, game_data: &GameData) -> CombatStats {
    match &entity.entity_type {
        crate::state::entities::EntityType::Creature(creature_state) => {
            let creature_data = game_data.monsters.get(&creature_state.creature_id)
                .expect("Creature data not found");

            // Calculate level bonuses
            let level_multiplier = 1.0 + (creature_state.level - 1) as f32 * 0.1;
            let health_bonus = (creature_state.level - 1) as f32 * creature_data.progression.stat_growth_per_level.get("health").copied().unwrap_or(10.0);

            CombatStats {
                health: creature_state.health,
                max_health: creature_state.max_health + health_bonus,
                attack: creature_data.stats.attack * level_multiplier,
                defense: creature_data.stats.defense * level_multiplier,
                attack_type: creature_data.combat.attack_type.clone(),
                damage_range: creature_data.combat.damage_range,
                attack_speed: creature_data.combat.attack_speed,
                armor_type: creature_data.combat.armor_type.clone(),
                resistances: creature_data.combat.resistances.clone(),
                level: creature_state.level,
            }
        }
        crate::state::entities::EntityType::Hero(hero_state) => {
            let hero_data = game_data.heroes.get(&hero_state.hero_id)
                .expect("Hero data not found");

            // Calculate level bonuses
            let level_multiplier = 1.0 + (hero_state.level - 1) as f32 * 0.15;
            let health_bonus = (hero_state.level - 1) as f32 * hero_data.progression.stat_growth_per_level.get("health").copied().unwrap_or(15.0);

            CombatStats {
                health: hero_state.health,
                max_health: hero_state.max_health + health_bonus,
                attack: hero_data.stats.attack * level_multiplier,
                defense: hero_data.stats.defense * level_multiplier,
                attack_type: hero_data.combat.attack_type.clone(),
                damage_range: hero_data.combat.damage_range,
                attack_speed: hero_data.combat.attack_speed,
                armor_type: hero_data.combat.armor_type.clone(),
                resistances: hero_data.combat.resistances.clone(),
                level: hero_state.level,
            }
        }
    }
}

/// Calculate damage from attacker to defender
pub fn calculate_damage(attacker: &CombatStats, defender: &CombatStats) -> f32 {
    // Base damage from attacker's range
    let base_damage = if attacker.damage_range[1] > attacker.damage_range[0] {
        let range = attacker.damage_range[1] - attacker.damage_range[0];
        attacker.damage_range[0] + rand::random::<f32>() * range
    } else {
        attacker.damage_range[0]
    };

    // Add attack stat bonus
    let attack_damage = base_damage + attacker.attack * 0.5;

    // Apply defense reduction
    let defense_reduction = defender.defense * 0.3;
    let pre_resist_damage = (attack_damage - defense_reduction).max(0.0);

    // Apply elemental resistances
    let resistance_multiplier = calculate_resistance_multiplier(attacker, defender);
    let final_damage = pre_resist_damage * resistance_multiplier;

    final_damage.max(0.0)
}

/// Calculate resistance multiplier based on attack type and defender resistances
fn calculate_resistance_multiplier(attacker: &CombatStats, defender: &CombatStats) -> f32 {
    let base_resistance = defender.resistances.get(&attacker.attack_type).copied().unwrap_or(0.0);

    // Convert resistance percentage to multiplier
    // Positive resistance reduces damage, negative increases
    1.0 - (base_resistance / 100.0)
}

/// Generate status effects from combat (simplified)
fn generate_status_effects(
    _attacker: &Entity,
    _defender: &Entity,
    _game_data: &GameData,
) -> Vec<StatusEffect> {
    // Simplified: no status effects for now
    // In full implementation, this would check abilities and generate effects
    Vec::new()
}

/// Apply combat result to entities
pub fn apply_combat_result(
    result: &CombatResult,
    attacker_id: EntityId,
    defender_id: EntityId,
    entities: &mut HashMap<EntityId, Entity>,
) {
    // Apply damage to defender
    if let Some(defender) = entities.get_mut(&defender_id) {
        match &mut defender.entity_type {
            crate::state::entities::EntityType::Creature(state) => {
                state.take_damage(result.damage_dealt);
            }
            crate::state::entities::EntityType::Hero(state) => {
                state.take_damage(result.damage_dealt);
            }
        }
    }

    // Apply status effects to defender
    if let Some(defender) = entities.get_mut(&defender_id) {
        for effect in &result.status_applied {
            match &mut defender.entity_type {
                crate::state::entities::EntityType::Creature(state) => {
                    state.status_effects.push(effect.clone());
                }
                crate::state::entities::EntityType::Hero(state) => {
                    state.status_effects.push(effect.clone());
                }
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
            award_experience(attacker, victim_level);
        }
    }
}

/// Award experience to attacker for killing a creature
fn award_experience(attacker: &mut Entity, victim_level: u32) {
    let exp_gain = victim_level as u32 * 10;

    match &mut attacker.entity_type {
        crate::state::entities::EntityType::Creature(state) => {
            state.experience += exp_gain as f32;

            // Check for level up
            // Use same max_experience field from CreatureState
            if state.experience >= state.max_experience {
                level_up_creature(state);
            }
        }
        crate::state::entities::EntityType::Hero(state) => {
            // Heroes don't level up in combat in this simplified system
            // Could be extended to award hero XP
        }
    }
}

/// Calculate experience needed for next level
fn calculate_exp_needed(current_level: u32) -> u32 {
    // Simple exponential growth
    100 * (2u32.pow(current_level - 1))
}

/// Level up a creature
fn level_up_creature(state: &mut crate::state::entities::CreatureState) {
    if state.level >= 5 {
        return; // Max level cap
    }

    state.level += 1;
    state.experience = 0.0; // Reset for next level
    state.max_experience *= 1.5; // Scaling XP requirement

    // Increase stats (simplified - in full game would use progression data)
    state.max_health += 10.0;
    state.health = state.max_health; // Full heal on level up

    // Could also increase attack, defense, etc.
}

/// Check if two entities are in combat range
pub fn in_combat_range(attacker_pos: TilePos, defender_pos: TilePos, attack_type: &str) -> bool {
    let distance = calculate_manhattan_distance(attacker_pos, defender_pos);

    match attack_type {
        "melee" => distance <= 1,
        "ranged" => distance <= 5,
        "magic" => distance <= 8,
        _ => distance <= 1,
    }
}

/// Calculate Manhattan distance between positions
fn calculate_manhattan_distance(a: TilePos, b: TilePos) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

/// Find potential combat targets for an entity
pub fn find_combat_targets(
    entity: &Entity,
    entities: &HashMap<EntityId, Entity>,
    game_data: &GameData,
) -> Vec<EntityId> {
    let mut targets = Vec::new();

    for (other_id, other_entity) in entities {
        if *other_id == entity.id {
            continue; // Don't target self
        }

        // Check if entities are hostile
        if are_hostile(entity, other_entity, game_data) {
            // Check if in range
            let attack_type = match &entity.entity_type {
                crate::state::entities::EntityType::Creature(state) => {
                    game_data.monsters.get(&state.creature_id)
                        .map(|data| data.combat.attack_type.clone())
                        .unwrap_or_else(|| "melee".to_string())
                }
                crate::state::entities::EntityType::Hero(state) => {
                    game_data.heroes.get(&state.hero_id)
                        .map(|data| data.combat.attack_type.clone())
                        .unwrap_or_else(|| "melee".to_string())
                }
            };

            if in_combat_range(entity.pos, other_entity.pos, &attack_type) {
                targets.push(*other_id);
            }
        }
    }

    targets
}

/// Check if two entities are hostile to each other
fn are_hostile(entity_a: &Entity, entity_b: &Entity, game_data: &GameData) -> bool {
    let faction_a = get_faction(entity_a, game_data);
    let faction_b = get_faction(entity_b, game_data);

    match (faction_a.as_str(), faction_b.as_str()) {
        ("dungeon", "hero") => true,
        ("hero", "dungeon") => true,
        ("wild", _) => true, // Wild monsters attack everyone
        (_, "wild") => true, // Everyone attacks wild monsters
        ("hero", "hero") => false,
        ("dungeon", "dungeon") => false, // Friendly fire off
        _ => false,
    }
}

fn get_faction(entity: &Entity, game_data: &GameData) -> String {
    match &entity.entity_type {
        crate::state::entities::EntityType::Creature(c) => {
            game_data.monsters.get(&c.creature_id)
                .map(|m| m.faction.clone())
                .unwrap_or("dungeon".to_string())
        },
        crate::state::entities::EntityType::Hero(_) => "hero".to_string(),
    }
}

/// Update status effects on an entity
pub fn update_status_effects(entity: &mut Entity, dt: f32) {
    match &mut entity.entity_type {
        crate::state::entities::EntityType::Creature(state) => {
            state.status_effects.retain_mut(|effect| {
                effect.duration -= dt;
                effect.duration > 0.0
            });
        }
        crate::state::entities::EntityType::Hero(state) => {
            state.status_effects.retain_mut(|effect| {
                effect.duration -= dt;
                effect.duration > 0.0
            });
        }
    }
}