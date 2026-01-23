//! Trap construction and triggering system
//! Handles funding, building, and triggering of traps/doors

use crate::data::GameData;
use crate::state::dungeon::Dungeon;
use crate::state::entities::{EntityId, EntityManager, EntityType};
use crate::state::player_state::PlayerState;
use crate::state::tile_state::TilePos;
use std::collections::HashSet;

/// Get the material cost for a trap type from game data
pub fn get_trap_cost(trap_type: &str, game_data: &GameData) -> i32 {
    game_data.traps
        .get(trap_type)
        .map(|data| data.cost)
        .unwrap_or(50) // Default fallback
}

/// Get the build time for a trap type in seconds from game data
pub fn get_trap_build_time(trap_type: &str, game_data: &GameData) -> f32 {
    game_data.traps
        .get(trap_type)
        .map(|data| data.build_time)
        .unwrap_or(5.0) // Default fallback
}

/// Process trap construction progress
/// Returns positions of completed traps
pub fn process_trap_construction(
    dungeon: &mut Dungeon,
    player: &mut PlayerState,
    pending_trap_builds: &mut HashSet<TilePos>,
    game_data: &GameData,
    dt: f32,
) -> Vec<TilePos> {
    let pending: Vec<TilePos> = pending_trap_builds.iter().cloned().collect();

    for pos in &pending {
        try_fund_trap(dungeon, player, *pos, game_data);
    }

    let completed_traps: Vec<TilePos> = pending.into_iter()
        .filter(|pos| progress_trap_construction(dungeon, *pos, game_data, dt))
        .collect();

    for pos in &completed_traps {
        pending_trap_builds.remove(pos);
    }

    completed_traps
}

/// Try to fund a trap at the given position
fn try_fund_trap(dungeon: &mut Dungeon, player: &mut PlayerState, pos: TilePos, game_data: &GameData) {
    let tile = match dungeon.get_tile_mut(pos) {
        Some(t) => t,
        None => return,
    };
    let trap = match tile.trap.as_mut() {
        Some(t) => t,
        None => return,
    };

    if trap.funded || trap.constructed {
        return;
    }

    let cost = get_trap_cost(&trap.trap_type, game_data);
    if player.materials >= cost {
        player.materials -= cost;
        trap.funded = true;
        eprintln!("Funded trap at {:?}", pos);
    }
}

/// Progress trap construction, returns true if completed
fn progress_trap_construction(dungeon: &mut Dungeon, pos: TilePos, game_data: &GameData, dt: f32) -> bool {
    let tile = match dungeon.get_tile_mut(pos) {
        Some(t) => t,
        None => return false,
    };
    let trap = match tile.trap.as_mut() {
        Some(t) => t,
        None => return false,
    };

    if !trap.funded || trap.constructed {
        return false;
    }

    let build_time = get_trap_build_time(&trap.trap_type, game_data);
    trap.construction_progress += dt;

    if trap.construction_progress >= build_time {
        trap.constructed = true;
        trap.active = true;
        eprintln!("Trap construction complete at {:?}", pos);
        return true;
    }

    false
}

/// Result of a trap trigger
#[derive(Debug)]
pub struct TrapTriggerResult {
    pub trap_pos: TilePos,
    pub trap_type: String,
    pub damage_dealt: f32,
    pub affected_entities: Vec<EntityId>,
}

/// Process trap triggers when entities step on them
/// Call this every tick to check for trap activations
pub fn process_trap_triggers(
    dungeon: &mut Dungeon,
    entities: &mut EntityManager,
    game_data: &GameData,
    dt: f32,
) -> Vec<TrapTriggerResult> {
    update_trap_cooldowns(dungeon, dt);

    let hero_positions: Vec<(EntityId, TilePos)> = entities.heroes()
        .filter_map(|(id, _)| entities.get(id).map(|e| (id, e.pos)))
        .collect();

    let traps_to_trigger: Vec<(TilePos, String, EntityId)> = hero_positions.into_iter()
        .filter_map(|(hero_id, hero_pos)| get_triggerable_trap(dungeon, hero_pos, hero_id))
        .collect();

    traps_to_trigger.into_iter()
        .filter_map(|(pos, trap_type, hero_id)| {
            let trap_data = game_data.traps.get(&trap_type)?;
            trigger_trap(pos, &trap_type, trap_data, hero_id, entities, dungeon, game_data)
        })
        .collect()
}

/// Update cooldowns for all traps
fn update_trap_cooldowns(dungeon: &mut Dungeon, dt: f32) {
    for y in 0..dungeon.height {
        for x in 0..dungeon.width {
            let pos = TilePos::new(x as i32, y as i32);
            if let Some(tile) = dungeon.get_tile_mut(pos) {
                if let Some(trap) = tile.trap.as_mut() {
                    trap.cooldown = (trap.cooldown - dt).max(0.0);
                }
            }
        }
    }
}

/// Check if a trap at the given position is triggerable and return its info
fn get_triggerable_trap(dungeon: &Dungeon, pos: TilePos, hero_id: EntityId) -> Option<(TilePos, String, EntityId)> {
    let tile = dungeon.get_tile(pos)?;
    let trap = tile.trap.as_ref()?;

    if trap.active && trap.constructed && trap.cooldown <= 0.0 && !trap.triggered {
        Some((pos, trap.trap_type.clone(), hero_id))
    } else {
        None
    }
}

/// Trigger a specific trap at a position
fn trigger_trap(
    pos: TilePos,
    trap_type: &str,
    trap_data: &crate::data::traps::TrapData,
    triggering_entity: EntityId,
    entities: &mut EntityManager,
    dungeon: &mut Dungeon,
    game_data: &GameData,
) -> Option<TrapTriggerResult> {
    match trap_type {
        "door" => None,
        "spike_trap" => trigger_damage_trap(pos, trap_data, triggering_entity, entities, dungeon),
        "blowgun_trap" => trigger_damage_trap(pos, trap_data, triggering_entity, entities, dungeon),
        "boulder_trap" => trigger_boulder_trap(pos, trap_data, entities, dungeon),
        "alarm_trap" => { trigger_alarm_trap(pos, trap_data, entities, dungeon, game_data); None }
        _ => { eprintln!("Unknown trap type: {}", trap_type); None }
    }
}

fn trigger_damage_trap(pos: TilePos, trap_data: &crate::data::traps::TrapData, triggering_entity: EntityId, entities: &mut EntityManager, dungeon: &mut Dungeon) -> Option<TrapTriggerResult> {
    let damage = trap_data.effects.damage;
    let entity = entities.get_mut(triggering_entity)?;
    let cooldown = trap_data.effects.cooldown.unwrap_or(5.0);

    apply_trap_damage(entity, damage);
    eprintln!("{} triggered at {:?}! Dealt {} damage.", trap_data.name, pos, damage);
    set_trap_cooldown(dungeon, pos, cooldown);

    Some(TrapTriggerResult { trap_pos: pos, trap_type: trap_data.id.clone(), damage_dealt: damage, affected_entities: vec![triggering_entity] })
}

fn trigger_boulder_trap(pos: TilePos, trap_data: &crate::data::traps::TrapData, entities: &mut EntityManager, dungeon: &mut Dungeon) -> Option<TrapTriggerResult> {
    let damage = trap_data.effects.damage;
    let radius = if trap_data.effects.area { 
        trap_data.effects.area_radius.unwrap_or(1.5) 
    } else { 
        trap_data.effects.single_radius.unwrap_or(0.5) 
    };

    let affected_entities: Vec<EntityId> = entities.heroes()
        .filter_map(|(id, _)| entities.get(id).map(|e| (id, e.pos)))
        .filter(|(_, e_pos)| pos.distance_to(e_pos) <= radius)
        .map(|(id, _)| id)
        .collect();

    if affected_entities.is_empty() {
        return None;
    }

    let mut total_damage = 0.0;
    for entity_id in &affected_entities {
        if let Some(entity) = entities.get_mut(*entity_id) {
            apply_trap_damage(entity, damage);
            total_damage += damage;
        }
    }

    eprintln!("Boulder trap triggered at {:?}! Dealt {} damage to {} entities.", pos, damage, affected_entities.len());
    set_trap_disabled(dungeon, pos);

    Some(TrapTriggerResult { trap_pos: pos, trap_type: "boulder_trap".to_string(), damage_dealt: total_damage, affected_entities })
}

fn trigger_alarm_trap(pos: TilePos, trap_data: &crate::data::traps::TrapData, entities: &EntityManager, dungeon: &mut Dungeon, game_data: &GameData) {
    let alert_radius = trap_data.effects.alert_radius;

    let alerted_count = entities.creatures()
        .filter_map(|(id, _)| entities.get(id).map(|e| e.pos))
        .filter(|e_pos| pos.distance_to(e_pos) <= alert_radius)
        .count();

    eprintln!("Alarm trap triggered at {:?}! Alerted {} creatures.", pos, alerted_count);
    let cooldown = game_data.config.traps.default_cooldown;
    let cooldown_multiplier = trap_data.effects.cooldown_multiplier.unwrap_or(2.0);
    set_trap_cooldown(dungeon, pos, cooldown * cooldown_multiplier);
}

fn set_trap_cooldown(dungeon: &mut Dungeon, pos: TilePos, cooldown: f32) {
    if let Some(tile) = dungeon.get_tile_mut(pos) {
        if let Some(trap) = tile.trap.as_mut() {
            trap.cooldown = cooldown;
        }
    }
}

fn set_trap_disabled(dungeon: &mut Dungeon, pos: TilePos) {
    if let Some(tile) = dungeon.get_tile_mut(pos) {
        if let Some(trap) = tile.trap.as_mut() {
            trap.triggered = true;
            trap.active = false;
        }
    }
}

/// Apply trap damage to an entity
fn apply_trap_damage(entity: &mut crate::state::entities::Entity, damage: f32) {
    match &mut entity.entity_type {
        EntityType::Hero(hero) => {
            hero.health = (hero.health - damage).max(0.0);
            eprintln!("Hero {} took {} trap damage (HP: {}/{})",
                hero.hero_id, damage, hero.health, hero.max_health);
        }
        EntityType::Creature(creature) => {
            creature.health = (creature.health - damage).max(0.0);
            eprintln!("Creature {} took {} trap damage (HP: {}/{})",
                creature.creature_id, damage, creature.health, creature.max_health);
        }
        EntityType::Structure(_) => {
            // Structures don't take trap damage
        }
        EntityType::ResourcePile(_) => {} // Piles don't take trap damage
    }
}

