//! Spell effects system
//! Handles casting and applying spell effects to the game state

use crate::data::spells::{SpellData, SpellEffect};
use crate::data::GameData;
use crate::state::entities::{CreatureState, EntityId};
use crate::state::game_state::GameState;
use crate::state::tile_state::{Ownership, TilePos};

/// Result of a spell cast attempt
#[derive(Debug, Clone)]
pub enum CastResult {
    Success,
    InsufficientMana,
    InsufficientGold,
    InvalidTarget,
    OnCooldown,
    OutOfRange,
    MaxCapReached,
}

/// Active spell cooldown tracker
#[derive(Debug, Clone)]
pub struct SpellCooldown {
    pub spell_id: String,
    pub remaining_time: f32,
}

/// Get active spell cooldowns as SpellCooldown structs
pub fn get_active_cooldowns(game_state: &GameState) -> Vec<SpellCooldown> {
    game_state.player.spell_cooldowns
        .iter()
        .map(|(spell_id, remaining)| SpellCooldown {
            spell_id: spell_id.clone(),
            remaining_time: *remaining,
        })
        .collect()
}

/// Check if a spell can be cast
pub fn can_cast_spell(
    spell: &SpellData,
    game_state: &GameState,
    target_pos: Option<TilePos>,
    target_entity: Option<EntityId>,
) -> CastResult {
    // Check cooldown
    if game_state.player.spell_cooldowns.contains_key(&spell.id) {
        return CastResult::OnCooldown;
    }

    // Check mana cost
    if game_state.player.mana < spell.cost.mana {
        return CastResult::InsufficientMana;
    }

    // Check gold cost
    if game_state.player.gold < spell.cost.gold {
        return CastResult::InsufficientGold;
    }

    // Special handling for summon_imps - check max cap and calculate dynamic cost
    if spell.id == "summon_imps" {
        const MAX_IMPS: usize = 10;
        let current_imps = game_state.count_imps();
        if current_imps >= MAX_IMPS {
            return CastResult::MaxCapReached;
        }
        // Dynamic cost: 10 base + 5 per existing imp
        let dynamic_cost = 10 + (current_imps as i32 * 5);
        if game_state.player.mana < dynamic_cost {
            return CastResult::InsufficientMana;
        }
    }

    // Check targeting validity and range
    match spell.targeting.target_type.as_str() {
        "tile" | "area" => {
            if target_pos.is_none() {
                return CastResult::InvalidTarget;
            }
            // Check range if specified
            if let Some(pos) = target_pos {
                if let Some(heart_pos) = game_state.find_dungeon_heart_position() {
                    let distance = ((pos.x - heart_pos.x).abs() + (pos.y - heart_pos.y).abs()) as u32;
                    if spell.targeting.range > 0 && distance > spell.targeting.range {
                        return CastResult::OutOfRange;
                    }
                }
            }
        }
        "creature" => {
            if target_entity.is_none() {
                return CastResult::InvalidTarget;
            }
        }
        "room" => {
            if target_pos.is_none() {
                return CastResult::InvalidTarget;
            }
        }
        "global" => {
            // No target needed
        }
        _ => {}
    }

    CastResult::Success
}

/// Cast a spell and apply its effects
pub fn cast_spell(
    spell_id: &str,
    game_state: &mut GameState,
    game_data: &GameData,
    target_pos: Option<TilePos>,
    target_entity: Option<EntityId>,
) -> CastResult {
    let spell = match game_data.spells.get(spell_id) {
        Some(s) => s,
        None => return CastResult::InvalidTarget,
    };

    // Check if cast is valid
    let can_cast = can_cast_spell(spell, game_state, target_pos, target_entity);
    if !matches!(can_cast, CastResult::Success) {
        return can_cast;
    }

    // Deduct costs - special handling for summon_imps
    if spell_id == "summon_imps" {
        let current_imps = game_state.count_imps();
        let dynamic_cost = 10 + (current_imps as i32 * 5);
        game_state.player.mana -= dynamic_cost;
        eprintln!("Cast spell: {} (dynamic mana: {}, imps: {})", spell.name, dynamic_cost, current_imps);
    } else {
        game_state.player.mana -= spell.cost.mana;
        game_state.player.gold -= spell.cost.gold;
        eprintln!("Cast spell: {} (mana: {}, gold: {})", spell.name, spell.cost.mana, spell.cost.gold);
    }

    // Apply effects
    for effect in &spell.effects {
        apply_spell_effect(effect, game_state, game_data, target_pos, target_entity);
    }

    // Set cooldown
    if spell.cooldown > 0.0 {
        game_state
            .player
            .spell_cooldowns
            .insert(spell_id.to_string(), spell.cooldown);
    }

    CastResult::Success
}

/// Apply a single spell effect
fn apply_spell_effect(
    effect: &SpellEffect,
    game_state: &mut GameState,
    game_data: &GameData,
    target_pos: Option<TilePos>,
    target_entity: Option<EntityId>,
) {
    match effect.effect_type.as_str() {
        "damage" => {
            if let Some(entity_id) = target_entity {
                apply_damage_effect(entity_id, effect, game_state);
            } else if let Some(pos) = target_pos {
                // Area damage - find all entities in range (radius 1 default)
                let area_radius = 1;
                let entities_in_area: Vec<EntityId> = game_state
                    .entities
                    .all()
                    .filter(|e| {
                        let dist = ((e.pos.x - pos.x).pow(2) + (e.pos.y - pos.y).pow(2)) as f32;
                        dist.sqrt() <= area_radius as f32
                    })
                    .map(|e| e.id)
                    .collect();

                for entity_id in entities_in_area {
                    apply_damage_effect(entity_id, effect, game_state);
                }
            }
        }
        "heal" => {
            if let Some(entity_id) = target_entity {
                apply_heal_effect(entity_id, effect, game_state);
            }
        }
        "stat_modifier" => {
            if let Some(entity_id) = target_entity {
                apply_stat_modifier(entity_id, effect, game_state);
            }
        }
        "status_apply" => {
            if let Some(entity_id) = target_entity {
                apply_status_effect(entity_id, effect, game_state);
            }
        }
        "tile_transform" => {
            if let Some(pos) = target_pos {
                apply_tile_transform(pos, effect, game_state, game_data);
            }
        }
        "spawn_entity" => {
            if let Some(pos) = target_pos {
                spawn_entity_effect(pos, effect, game_state, game_data);
            }
        }
        "reveal_map" => {
            if let Some(pos) = target_pos {
                reveal_map_effect(pos, effect, game_state);
            }
        }
        _ => {
            eprintln!("Unknown spell effect type: {}", effect.effect_type);
        }
    }
}

/// Apply damage to an entity
fn apply_damage_effect(entity_id: EntityId, effect: &SpellEffect, game_state: &mut GameState) {
    if let Some(entity) = game_state.entities.get_mut(entity_id) {
        let damage = effect.amount;

        match &mut entity.entity_type {
            crate::state::entities::EntityType::Creature(creature) => {
                creature.health = (creature.health - damage).max(0.0);
                eprintln!(
                    "Spell damage: {} took {} damage (HP: {}/{})",
                    creature.creature_id, damage, creature.health, creature.max_health
                );
            }
            crate::state::entities::EntityType::Hero(hero) => {
                hero.health = (hero.health - damage).max(0.0);
                eprintln!(
                    "Spell damage: {} took {} damage (HP: {}/{})",
                    hero.hero_id, damage, hero.health, hero.max_health
                );
            }
        }
    }
}

/// Apply healing to an entity
fn apply_heal_effect(entity_id: EntityId, effect: &SpellEffect, game_state: &mut GameState) {
    if let Some(entity) = game_state.entities.get_mut(entity_id) {
        let heal_amount = effect.amount;

        match &mut entity.entity_type {
            crate::state::entities::EntityType::Creature(creature) => {
                creature.health = (creature.health + heal_amount).min(creature.max_health);
                eprintln!(
                    "Spell heal: {} healed {} HP (HP: {}/{})",
                    creature.creature_id, heal_amount, creature.health, creature.max_health
                );
            }
            crate::state::entities::EntityType::Hero(hero) => {
                hero.health = (hero.health + heal_amount).min(hero.max_health);
                eprintln!(
                    "Spell heal: {} healed {} HP (HP: {}/{})",
                    hero.hero_id, heal_amount, hero.health, hero.max_health
                );
            }
        }
    }
}

/// Apply stat modifier (e.g., speed boost)
fn apply_stat_modifier(entity_id: EntityId, effect: &SpellEffect, game_state: &mut GameState) {
    if let Some(entity) = game_state.entities.get_mut(entity_id) {
        if let crate::state::entities::EntityType::Creature(creature) = &mut entity.entity_type {
            if effect.stat.as_deref() == Some("speed") {
                let multiplier = effect.multiplier.unwrap_or(1.0);
                creature.movement_speed *= multiplier;
                eprintln!(
                    "Spell buff: {} speed multiplied by {} (new speed: {})",
                    creature.creature_id, multiplier, creature.movement_speed
                );

                // Could add duration tracking here for temporary buffs
            }
        }
    }
}

/// Apply status effect
fn apply_status_effect(entity_id: EntityId, effect: &SpellEffect, game_state: &mut GameState) {
    if let Some(entity) = game_state.entities.get_mut(entity_id) {
        if let crate::state::entities::EntityType::Creature(creature) = &mut entity.entity_type {
            if let Some(status) = &effect.status {
                let duration = effect.duration.unwrap_or(10.0);

                // Add status effect
                creature.status_effects.push(crate::state::entities::StatusEffect {
                    effect_type: status.clone(),
                    duration,
                    strength: if effect.amount > 0.0 { effect.amount } else { 1.0 },
                });

                eprintln!(
                    "Status applied: {} gained '{}' for {} seconds",
                    creature.creature_id, status, duration
                );
            }
        }
    }
}

/// Transform tile type
fn apply_tile_transform(pos: TilePos, effect: &SpellEffect, game_state: &mut GameState, game_data: &GameData) {
    let area_radius = effect.radius.unwrap_or(0); // Default to single tile if not specified

    for dy in -(area_radius as i32)..=(area_radius as i32) {
        for dx in -(area_radius as i32)..=(area_radius as i32) {
            let target_pos = TilePos::new(pos.x + dx, pos.y + dy);

            if let Some(tile) = game_state.get_tile_mut(target_pos) {
                // Check if tile matches 'from' type (if specified)
                if let Some(from_type) = &effect.from_tile {
                    if tile.tile_type != *from_type {
                        continue;
                    }
                }

                // Transform to 'to' type
                if let Some(to_type) = &effect.to_tile {
                    tile.tile_type = to_type.clone();
                    
                    // If the new tile type is not claimable (e.g. earth, rock), reset ownership
                    // This ensures player-owned floor transformed to earth becomes neutral/diggable
                    if !crate::engine::tile_types::is_claimable(to_type, game_data) {
                        tile.ownership = Ownership::Unclaimed;
                        tile.room_id = None;
                        tile.marked_for_dig = false;
                    }
                    
                    eprintln!("Tile transformed at {:?} to {}", target_pos, to_type);
                }
            }
        }
    }
}

/// Spawn entity effect
fn spawn_entity_effect(
    pos: TilePos,
    effect: &SpellEffect,
    game_state: &mut GameState,
    game_data: &GameData,
) {
    if let Some(entity_type) = &effect.entity {
        // Try to spawn an imp
        if entity_type == "imp" {
            const MAX_IMPS: usize = 10;
            if game_state.count_imps() >= MAX_IMPS {
                eprintln!("Cannot summon imp: max cap of {} reached", MAX_IMPS);
                return;
            }

            if let Some(monster_data) = game_data.monsters.get("imp") {
                let creature_state = CreatureState::new(
                    "imp".to_string(),
                    1,
                    monster_data.stats.health,
                    monster_data.stats.mana,
                );

                game_state.entities.spawn_creature(pos, creature_state);

                eprintln!("Spell summoned imp at {:?} (total imps: {})", pos, game_state.count_imps());
            }
        }
    }
}

/// Reveal map effect
fn reveal_map_effect(center: TilePos, _effect: &SpellEffect, game_state: &mut GameState) {
    let radius = 8; // Default reveal radius

    for dy in -(radius as i32)..=(radius as i32) {
        for dx in -(radius as i32)..=(radius as i32) {
            let target_pos = TilePos::new(center.x + dx, center.y + dy);

            if let Some(tile) = game_state.get_tile_mut(target_pos) {
                tile.fog_state = crate::state::tile_state::FogState::Visible;
            }
        }
    }

    eprintln!("Revealed map around {:?} with radius {}", center, radius);
}

/// Update spell cooldowns (call once per second)
pub fn update_spell_cooldowns(game_state: &mut GameState, dt: f32) {
    let mut completed_cooldowns = Vec::new();

    for (spell_id, remaining_time) in game_state.player.spell_cooldowns.iter_mut() {
        *remaining_time -= dt;
        if *remaining_time <= 0.0 {
            completed_cooldowns.push(spell_id.clone());
        }
    }

    for spell_id in completed_cooldowns {
        game_state.player.spell_cooldowns.remove(&spell_id);
    }
}
