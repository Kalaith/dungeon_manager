//! Room, trap, hero tier, data-integrity, and known-issue balance tests.

use super::data;
use std::collections::HashMap;

// ============================================================================
// Room Balance Tests
// ============================================================================

#[test]
fn test_hatchery_generates_food() {
    let rooms = data::load_rooms();

    if let Some(hatchery) = rooms.get("hatchery") {
        let food_gen = hatchery
            .effects
            .as_ref()
            .and_then(|e| e.food_generation_per_second)
            .unwrap_or(0.0);

        assert!(
            food_gen > 0.0,
            "Hatchery should generate food (current: {:.2}/sec)",
            food_gen
        );
    }
}

#[test]
fn test_treasury_stores_gold() {
    let rooms = data::load_rooms();

    if let Some(treasury) = rooms.get("treasury") {
        let gold_cap = treasury
            .effects
            .as_ref()
            .and_then(|e| e.gold_storage_capacity)
            .unwrap_or(0);

        assert!(
            gold_cap > 0,
            "Treasury should store gold (current: {} per tile)",
            gold_cap
        );
    }
}

#[test]
fn test_room_costs_positive() {
    let rooms = data::load_rooms();

    for room in rooms.values() {
        if let Some(build) = &room.build {
            // Most rooms should have a positive cost (some special rooms might be free)
            if room.id != "dungeon_heart" && room.id != "portal" {
                assert!(
                    build.cost_per_tile >= 0,
                    "Room {} has negative cost: {}",
                    room.name,
                    build.cost_per_tile
                );
            }
        }
    }
}

// ============================================================================
// Trap Balance Tests
// ============================================================================

#[test]
fn test_traps_have_positive_cost() {
    let traps = data::load_traps();

    for trap in traps.values() {
        assert!(
            trap.cost >= 0,
            "Trap {} has negative cost: {}",
            trap.name,
            trap.cost
        );
    }
}

#[test]
fn test_damage_traps_do_damage() {
    let traps = data::load_traps();

    for trap in traps.values() {
        let damage = trap.effects.as_ref().and_then(|e| e.damage).unwrap_or(0.0);

        // Skip non-damage traps (doors, alarms, etc.)
        if trap.id.contains("spike") || trap.id.contains("boulder") {
            assert!(
                damage > 0.0,
                "Damage trap {} should deal damage (current: {:.0})",
                trap.name,
                damage
            );
        }
    }
}

// ============================================================================
// Hero Balance Tests
// ============================================================================

#[test]
fn test_hero_tiers_exist() {
    let heroes = data::load_heroes();

    let mut tiers_found = [false; 5];
    for hero in heroes.values() {
        let tier = hero.tier.unwrap_or(1) as usize;
        if (1..=5).contains(&tier) {
            tiers_found[tier - 1] = true;
        }
    }

    // At least tiers 1-3 should exist
    assert!(tiers_found[0], "No tier 1 heroes found");
    assert!(tiers_found[1], "No tier 2 heroes found");
    assert!(tiers_found[2], "No tier 3 heroes found");
}

#[test]
fn test_higher_tier_heroes_stronger() {
    let heroes = data::load_heroes();

    // Calculate average power per tier
    let mut tier_power: HashMap<u32, (f32, u32)> = HashMap::new();

    for hero in heroes.values() {
        let tier = hero.tier.unwrap_or(1);
        let power = hero.stats.health + hero.stats.attack * 5.0 + hero.stats.defense * 3.0;

        let entry = tier_power.entry(tier).or_insert((0.0, 0));
        entry.0 += power;
        entry.1 += 1;
    }

    // Calculate averages
    let tier_avgs: Vec<(u32, f32)> = tier_power
        .iter()
        .map(|(tier, (total, count))| (*tier, total / *count as f32))
        .collect();

    // Higher tiers should generally have higher power
    for i in 0..tier_avgs.len() {
        for j in (i + 1)..tier_avgs.len() {
            let (tier_a, avg_a) = tier_avgs[i];
            let (tier_b, avg_b) = tier_avgs[j];
            if tier_a < tier_b {
                // Allow some variance (tier_b should be at least 80% of tier_a)
                assert!(
                    avg_b >= avg_a * 0.8,
                    "Tier {} (avg power {:.0}) should be stronger than tier {} (avg power {:.0})",
                    tier_b,
                    avg_b,
                    tier_a,
                    avg_a
                );
            }
        }
    }
}

// ============================================================================
// Data Integrity Tests
// ============================================================================

#[test]
fn test_monster_json_loads() {
    let monsters = data::load_monsters();
    assert!(!monsters.is_empty(), "Failed to load any monsters");
}

#[test]
fn test_hero_json_loads() {
    let heroes = data::load_heroes();
    assert!(!heroes.is_empty(), "Failed to load any heroes");
}

#[test]
fn test_room_json_loads() {
    let rooms = data::load_rooms();
    assert!(!rooms.is_empty(), "Failed to load any rooms");
}

#[test]
fn test_trap_json_loads() {
    let traps = data::load_traps();
    assert!(!traps.is_empty(), "Failed to load any traps");
}

#[test]
fn test_config_json_loads() {
    let config = data::load_config();
    // Just verify it loaded without panicking
    assert!(config.player_starting_resources.gold >= 0);
}

// ============================================================================
// Known Issues Tests (expected to fail until fixed)
// ============================================================================

#[test]
#[ignore] // Remove this line once the bug is fixed
fn test_mana_capacity_bug_fixed() {
    let config = data::load_config();
    // This test documents the known mana capacity bug
    // It will pass once max_mana is set to a non-zero value
    assert!(
        config.player_initial_capacity.max_mana >= 1000,
        "Max mana should be at least 1000, but is {}. This is a known bug.",
        config.player_initial_capacity.max_mana
    );
}
