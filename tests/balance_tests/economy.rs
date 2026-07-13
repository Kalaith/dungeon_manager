//! Resource Economy and Wave System balance tests.

use super::data;

// ============================================================================
// Resource Economy Tests
// ============================================================================

#[test]
fn test_starting_gold_allows_basic_dungeon() {
    let config = data::load_config();
    let rooms = data::load_rooms();

    // Calculate minimum dungeon cost: Lair (9 tiles) + Hatchery (9 tiles) + Treasury (4 tiles)
    let lair_cost = rooms
        .get("lair")
        .and_then(|r| r.build.as_ref())
        .map(|b| b.cost_per_tile * 9)
        .unwrap_or(450);
    let hatchery_cost = rooms
        .get("hatchery")
        .and_then(|r| r.build.as_ref())
        .map(|b| b.cost_per_tile * 9)
        .unwrap_or(675);
    let treasury_cost = rooms
        .get("treasury")
        .and_then(|r| r.build.as_ref())
        .map(|b| b.cost_per_tile * 4)
        .unwrap_or(400);

    let min_cost = lair_cost + hatchery_cost + treasury_cost;

    assert!(
        config.player_starting_resources.gold >= min_cost,
        "Starting gold ({}) should be >= minimum dungeon cost ({})",
        config.player_starting_resources.gold,
        min_cost
    );
}

#[test]
fn test_mana_capacity_not_zero() {
    let config = data::load_config();
    // This test will fail if max_mana is 0, highlighting the bug
    assert!(
        config.player_initial_capacity.max_mana > 0,
        "Max mana capacity should not be 0 (current: {})",
        config.player_initial_capacity.max_mana
    );
}

#[test]
fn test_starting_food_positive() {
    let config = data::load_config();
    assert!(
        config.player_starting_resources.food > 0,
        "Starting food should be positive"
    );
}

#[test]
fn test_starting_mana_positive() {
    let config = data::load_config();
    assert!(
        config.player_starting_resources.mana > 0,
        "Starting mana should be positive"
    );
}

// ============================================================================
// Wave System Tests
// ============================================================================

#[test]
fn test_wave_initial_delay_reasonable() {
    let config = data::load_config();
    assert!(
        config.hero_waves.initial_delay >= 30.0,
        "Initial wave delay ({:.0}s) should be at least 30 seconds",
        config.hero_waves.initial_delay
    );
    assert!(
        config.hero_waves.initial_delay <= 3600.0,
        "Initial wave delay ({:.0}s) should be at most 1 hour",
        config.hero_waves.initial_delay
    );
}

#[test]
fn test_wave_scaling_multiplier_reasonable() {
    let config = data::load_config();
    assert!(
        config.hero_waves.wave_scaling_multiplier >= 1.0,
        "Wave scaling ({:.2}x) should be at least 1.0",
        config.hero_waves.wave_scaling_multiplier
    );
    assert!(
        config.hero_waves.wave_scaling_multiplier <= 3.0,
        "Wave scaling ({:.2}x) should be at most 3.0",
        config.hero_waves.wave_scaling_multiplier
    );
}

#[test]
fn test_wave_interval_reasonable() {
    let config = data::load_config();
    assert!(
        config.hero_waves.wave_interval >= 60.0,
        "Wave interval ({:.0}s) should be at least 1 minute",
        config.hero_waves.wave_interval
    );
    assert!(
        config.hero_waves.wave_interval <= 600.0,
        "Wave interval ({:.0}s) should be at most 10 minutes",
        config.hero_waves.wave_interval
    );
}
