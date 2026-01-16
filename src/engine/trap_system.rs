//! Trap construction and triggering system
//! Handles funding, building, and triggering of traps/doors

use crate::data::GameData;
use crate::state::dungeon::Dungeon;
use crate::state::entities::{EntityId, EntityManager, EntityType};
use crate::state::player_state::PlayerState;
use crate::state::tile_state::TilePos;
use std::collections::HashSet;

/// Cooldown time after a trap triggers before it can trigger again (seconds)
const TRAP_COOLDOWN: f32 = 5.0;

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
    let mut completed_traps = Vec::new();

    // 1. Check pending traps to fund
    let pending: Vec<TilePos> = pending_trap_builds.iter().cloned().collect();
    for pos in &pending {
        if let Some(tile) = dungeon.get_tile_mut(*pos) {
            if let Some(ref mut trap) = tile.trap {
                if !trap.funded && !trap.constructed {
                    let cost = get_trap_cost(&trap.trap_type, game_data);

                    if player.materials >= cost {
                        player.materials -= cost;
                        trap.funded = true;
                        eprintln!("Funded trap at {:?}", pos);
                    }
                }
            }
        }
    }

    // 2. Progress funded traps
    for pos in pending {
        let mut finished = false;
        if let Some(tile) = dungeon.get_tile_mut(pos) {
            if let Some(ref mut trap) = tile.trap {
                if trap.funded && !trap.constructed {
                    let build_time = get_trap_build_time(&trap.trap_type, game_data);

                    trap.construction_progress += dt;
                    if trap.construction_progress >= build_time {
                        trap.constructed = true;
                        trap.active = true;
                        finished = true;
                        eprintln!("Trap construction complete at {:?}", pos);
                    }
                }
            }
        }

        if finished {
            completed_traps.push(pos);
        }
    }

    for pos in &completed_traps {
        pending_trap_builds.remove(pos);
    }

    completed_traps
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
    let mut results = Vec::new();

    // First, update all trap cooldowns
    for y in 0..dungeon.height {
        for x in 0..dungeon.width {
            let pos = TilePos::new(x as i32, y as i32);
            if let Some(tile) = dungeon.get_tile_mut(pos) {
                if let Some(ref mut trap) = tile.trap {
                    if trap.cooldown > 0.0 {
                        trap.cooldown -= dt;
                        if trap.cooldown < 0.0 {
                            trap.cooldown = 0.0;
                        }
                    }
                }
            }
        }
    }

    // Collect hero positions (only heroes trigger traps)
    let hero_positions: Vec<(EntityId, TilePos)> = entities
        .heroes()
        .map(|(id, _)| {
            let entity = entities.get(id).unwrap();
            (id, entity.pos)
        })
        .collect();

    // Collect traps to trigger (must collect first to avoid borrow issues)
    let mut traps_to_trigger: Vec<(TilePos, String, EntityId)> = Vec::new();

    for (hero_id, hero_pos) in hero_positions {
        if let Some(tile) = dungeon.get_tile(hero_pos) {
            if let Some(ref trap) = tile.trap {
                // Only trigger if trap is active, not on cooldown, and not already triggered (for single-use)
                if trap.active && trap.constructed && trap.cooldown <= 0.0 && !trap.triggered {
                    traps_to_trigger.push((hero_pos, trap.trap_type.clone(), hero_id));
                }
            }
        }
    }

    // Now trigger collected traps
    for (hero_pos, trap_type, hero_id) in traps_to_trigger {
        if let Some(trap_data) = game_data.traps.get(&trap_type) {
            let result = trigger_trap(
                hero_pos,
                &trap_type,
                trap_data,
                hero_id,
                entities,
                dungeon,
            );

            if let Some(res) = result {
                results.push(res);
            }
        }
    }

    results
}

/// Trigger a specific trap at a position
fn trigger_trap(
    pos: TilePos,
    trap_type: &str,
    trap_data: &crate::data::traps::TrapData,
    triggering_entity: EntityId,
    entities: &mut EntityManager,
    dungeon: &mut Dungeon,
) -> Option<TrapTriggerResult> {
    let mut affected_entities = Vec::new();
    let mut total_damage = 0.0;

    match trap_type {
        "door" => {
            // Doors don't deal damage, they just block movement
            // Door interaction would be handled elsewhere
            return None;
        }
        "spike_trap" => {
            // Spike trap damages the entity that steps on it
            let damage = trap_data.effects.damage;
            if let Some(entity) = entities.get_mut(triggering_entity) {
                apply_trap_damage(entity, damage);
                affected_entities.push(triggering_entity);
                total_damage = damage;
                eprintln!("Spike trap triggered at {:?}! Dealt {} damage.", pos, damage);
            }

            // Set cooldown
            if let Some(tile) = dungeon.get_tile_mut(pos) {
                if let Some(ref mut trap) = tile.trap {
                    trap.cooldown = TRAP_COOLDOWN;
                }
            }
        }
        "boulder_trap" => {
            // Boulder trap deals AoE damage
            let damage = trap_data.effects.damage;
            let radius = if trap_data.effects.area { 1 } else { 0 };

            // Find all heroes in range
            for (entity_id, _) in entities.heroes() {
                if let Some(entity) = entities.get(entity_id) {
                    let dist = pos.distance_to(&entity.pos);
                    if dist <= radius as f32 + 0.5 {
                        affected_entities.push(entity_id);
                    }
                }
            }

            // Apply damage to all affected
            for entity_id in &affected_entities {
                if let Some(entity) = entities.get_mut(*entity_id) {
                    apply_trap_damage(entity, damage);
                    total_damage += damage;
                }
            }

            eprintln!("Boulder trap triggered at {:?}! Dealt {} damage to {} entities.",
                pos, damage, affected_entities.len());

            // Boulder trap is single-use
            if let Some(tile) = dungeon.get_tile_mut(pos) {
                if let Some(ref mut trap) = tile.trap {
                    trap.triggered = true;
                    trap.active = false;
                }
            }
        }
        "alarm_trap" => {
            // Alarm trap alerts nearby creatures
            let alert_radius = trap_data.effects.alert_radius;

            // Find all player creatures within range
            let mut alerted_count = 0;
            for (creature_id, _) in entities.creatures() {
                if let Some(entity) = entities.get(creature_id) {
                    let dist = pos.distance_to(&entity.pos);
                    if dist <= alert_radius {
                        // Could set creature to "alerted" state or give them a target
                        // For now, just count them
                        alerted_count += 1;
                    }
                }
            }

            eprintln!("Alarm trap triggered at {:?}! Alerted {} creatures.", pos, alerted_count);

            // Set cooldown
            if let Some(tile) = dungeon.get_tile_mut(pos) {
                if let Some(ref mut trap) = tile.trap {
                    trap.cooldown = TRAP_COOLDOWN * 2.0; // Longer cooldown for alarms
                }
            }

            // Alarm doesn't deal damage
            return None;
        }
        _ => {
            eprintln!("Unknown trap type: {}", trap_type);
            return None;
        }
    }

    if affected_entities.is_empty() {
        return None;
    }

    Some(TrapTriggerResult {
        trap_pos: pos,
        trap_type: trap_type.to_string(),
        damage_dealt: total_damage,
        affected_entities,
    })
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
    }
}

