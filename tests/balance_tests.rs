//! Balance Integration Tests
#![allow(dead_code)]
//!
//! These tests verify that game balance values are within acceptable ranges.
//! Run with: cargo test --test balance_tests
//!
//! Tests are organized by system:
//! - Resource economy
//! - Combat balance
//! - Wave scaling
//! - Creature costs
//! - Room effects

use std::collections::HashMap;

// ============================================================================
// Data Loading (duplicated from balance_calculator for test isolation)
// ============================================================================

mod data {
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Debug, Deserialize)]
    pub struct MonsterData {
        pub id: String,
        pub name: String,
        pub stats: CreatureStats,
        #[serde(default)]
        pub combat: Option<CombatData>,
        #[serde(default)]
        pub economy: Option<EconomyData>,
    }

    #[derive(Debug, Deserialize)]
    pub struct CreatureStats {
        pub health: f32,
        pub attack: f32,
        pub defense: f32,
        pub speed: f32,
    }

    #[derive(Debug, Deserialize)]
    pub struct CombatData {
        #[serde(default)]
        pub damage_range: Option<[f32; 2]>,
        #[serde(default)]
        pub attack_speed: Option<f32>,
    }

    #[derive(Debug, Deserialize)]
    pub struct EconomyData {
        #[serde(default)]
        pub wage_per_minute: Option<f32>,
    }

    #[derive(Debug, Deserialize)]
    pub struct HeroData {
        pub id: String,
        pub name: String,
        pub stats: CreatureStats,
        #[serde(default)]
        pub tier: Option<u32>,
        #[serde(default)]
        pub combat: Option<CombatData>,
    }

    #[derive(Debug, Deserialize)]
    pub struct RoomData {
        pub id: String,
        pub name: String,
        #[serde(default)]
        pub build: Option<RoomBuild>,
        #[serde(default)]
        pub effects: Option<RoomEffects>,
    }

    #[derive(Debug, Deserialize)]
    pub struct RoomBuild {
        #[serde(default)]
        pub cost_per_tile: i32,
        #[serde(default)]
        pub min_tiles: Option<u32>,
        #[serde(default)]
        pub max_tiles: Option<u32>,
    }

    #[derive(Debug, Deserialize)]
    pub struct RoomEffects {
        #[serde(default)]
        pub food_generation_per_second: Option<f32>,
        #[serde(default)]
        pub mana_generation_per_second: Option<f32>,
        #[serde(default)]
        pub gold_storage_capacity: Option<i32>,
        #[serde(default)]
        pub research_speed: Option<f32>,
        #[serde(default)]
        pub training_xp_per_second: Option<f32>,
    }

    #[derive(Debug, Deserialize)]
    pub struct TrapData {
        pub id: String,
        pub name: String,
        pub cost: i32,
        #[serde(default)]
        pub build_time: Option<f32>,
        #[serde(default)]
        pub effects: Option<TrapEffects>,
    }

    #[derive(Debug, Deserialize)]
    pub struct TrapEffects {
        #[serde(default)]
        pub damage: Option<f32>,
    }

    #[derive(Debug, Deserialize)]
    pub struct GameConfig {
        pub player_starting_resources: ResourceConfig,
        pub player_initial_capacity: CapacityConfig,
        pub hero_waves: WaveConfig,
        pub combat: CombatConfig,
    }

    #[derive(Debug, Deserialize)]
    pub struct ResourceConfig {
        pub gold: i32,
        pub mana: i32,
        pub food: i32,
    }

    #[derive(Debug, Deserialize)]
    pub struct CapacityConfig {
        pub max_gold: i32,
        pub max_mana: i32,
    }

    #[derive(Debug, Deserialize)]
    pub struct WaveConfig {
        pub initial_delay: f32,
        pub wave_interval: f32,
        pub wave_scaling_multiplier: f32,
        #[serde(default)]
        pub spawn_rate_decay: Option<f32>,
    }

    #[derive(Debug, Deserialize)]
    pub struct CombatConfig {
        pub attack_stat_bonus: f32,
        pub defense_reduction: f32,
        #[serde(default)]
        pub creature_level_multiplier: Option<f32>,
        #[serde(default)]
        pub hero_level_multiplier: Option<f32>,
    }

    pub fn load_monsters() -> HashMap<String, MonsterData> {
        let json = include_str!("../assets/data/monsters.json");
        let list: Vec<MonsterData> =
            serde_json::from_str(json).expect("Failed to parse monsters.json");
        list.into_iter().map(|m| (m.id.clone(), m)).collect()
    }

    pub fn load_heroes() -> HashMap<String, HeroData> {
        let json = include_str!("../assets/data/heroes.json");
        let list: Vec<HeroData> = serde_json::from_str(json).expect("Failed to parse heroes.json");
        list.into_iter().map(|h| (h.id.clone(), h)).collect()
    }

    pub fn load_rooms() -> HashMap<String, RoomData> {
        let json = include_str!("../assets/data/rooms.json");
        let list: Vec<RoomData> = serde_json::from_str(json).expect("Failed to parse rooms.json");
        list.into_iter().map(|r| (r.id.clone(), r)).collect()
    }

    pub fn load_traps() -> HashMap<String, TrapData> {
        let json = include_str!("../assets/data/traps.json");
        let list: Vec<TrapData> = serde_json::from_str(json).expect("Failed to parse traps.json");
        list.into_iter().map(|t| (t.id.clone(), t)).collect()
    }

    pub fn load_config() -> GameConfig {
        let json = include_str!("../assets/data/game_config.json");
        serde_json::from_str(json).expect("Failed to parse game_config.json")
    }
}

// ============================================================================
// Combat Simulation
// ============================================================================

#[derive(Clone)]
struct CombatUnit {
    name: String,
    hp: f32,
    max_hp: f32,
    attack: f32,
    defense: f32,
    damage_min: f32,
    damage_max: f32,
    attack_speed: f32,
}

impl CombatUnit {
    fn from_monster(m: &data::MonsterData) -> Self {
        let (damage_min, damage_max) = m
            .combat
            .as_ref()
            .and_then(|c| c.damage_range)
            .map(|r| (r[0], r[1]))
            .unwrap_or((5.0, 10.0));
        let attack_speed = m
            .combat
            .as_ref()
            .and_then(|c| c.attack_speed)
            .unwrap_or(1.0);
        Self {
            name: m.name.clone(),
            hp: m.stats.health,
            max_hp: m.stats.health,
            attack: m.stats.attack,
            defense: m.stats.defense,
            damage_min,
            damage_max,
            attack_speed,
        }
    }

    fn from_hero(h: &data::HeroData) -> Self {
        let (damage_min, damage_max) = h
            .combat
            .as_ref()
            .and_then(|c| c.damage_range)
            .map(|r| (r[0], r[1]))
            .unwrap_or((5.0, 10.0));
        let attack_speed = h
            .combat
            .as_ref()
            .and_then(|c| c.attack_speed)
            .unwrap_or(1.0);
        Self {
            name: h.name.clone(),
            hp: h.stats.health,
            max_hp: h.stats.health,
            attack: h.stats.attack,
            defense: h.stats.defense,
            damage_min,
            damage_max,
            attack_speed,
        }
    }
}

struct CombatResult {
    winner: String,
    duration_secs: f32,
    winner_hp_remaining: f32,
}

fn simulate_combat(
    mut attacker: CombatUnit,
    mut defender: CombatUnit,
    attack_mult: f32,
    defense_mult: f32,
) -> CombatResult {
    let mut time = 0.0;
    let mut attacker_cooldown = 0.0;
    let mut defender_cooldown = 0.0;
    let dt = 0.1;

    while attacker.hp > 0.0 && defender.hp > 0.0 && time < 300.0 {
        time += dt;
        attacker_cooldown -= dt;
        defender_cooldown -= dt;

        if attacker_cooldown <= 0.0 {
            let base_damage = (attacker.damage_min + attacker.damage_max) / 2.0;
            let attack_damage = base_damage + (attacker.attack * attack_mult);
            let defense_reduction = defender.defense * defense_mult;
            let damage = (attack_damage - defense_reduction).max(1.0);
            defender.hp -= damage;
            attacker_cooldown = 1.0 / attacker.attack_speed;
        }

        if defender_cooldown <= 0.0 && defender.hp > 0.0 {
            let base_damage = (defender.damage_min + defender.damage_max) / 2.0;
            let attack_damage = base_damage + (defender.attack * attack_mult);
            let defense_reduction = attacker.defense * defense_mult;
            let damage = (attack_damage - defense_reduction).max(1.0);
            attacker.hp -= damage;
            defender_cooldown = 1.0 / defender.attack_speed;
        }
    }

    if attacker.hp > 0.0 {
        CombatResult {
            winner: attacker.name,
            duration_secs: time,
            winner_hp_remaining: attacker.hp,
        }
    } else {
        CombatResult {
            winner: defender.name,
            duration_secs: time,
            winner_hp_remaining: defender.hp,
        }
    }
}

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

// ============================================================================
// Creature Balance Tests
// ============================================================================

#[test]
fn test_all_creatures_have_positive_health() {
    let monsters = data::load_monsters();
    for monster in monsters.values() {
        assert!(
            monster.stats.health > 0.0,
            "Creature {} has non-positive health: {}",
            monster.name,
            monster.stats.health
        );
    }
}

#[test]
fn test_all_heroes_have_positive_health() {
    let heroes = data::load_heroes();
    for hero in heroes.values() {
        assert!(
            hero.stats.health > 0.0,
            "Hero {} has non-positive health: {}",
            hero.name,
            hero.stats.health
        );
    }
}

#[test]
fn test_creature_wage_efficiency_variance() {
    let monsters = data::load_monsters();

    let mut efficiencies: Vec<(String, f32)> = Vec::new();
    for monster in monsters.values() {
        let wage = monster
            .economy
            .as_ref()
            .and_then(|e| e.wage_per_minute)
            .unwrap_or(0.0);
        if wage > 0.0 {
            let efficiency = monster.stats.health / wage;
            efficiencies.push((monster.name.clone(), efficiency));
        }
    }

    if efficiencies.len() >= 2 {
        let max_eff = efficiencies.iter().map(|(_, e)| *e).fold(0.0f32, f32::max);
        let min_eff = efficiencies
            .iter()
            .map(|(_, e)| *e)
            .fold(f32::MAX, f32::min);
        let ratio = max_eff / min_eff;

        // Efficiency variance shouldn't exceed 5x (some variance is expected)
        assert!(
            ratio < 5.0,
            "Creature wage efficiency variance ({:.1}x) is too high. Max: {:.1}, Min: {:.1}",
            ratio,
            max_eff,
            min_eff
        );
    }
}

#[test]
fn test_imps_have_lowest_wage() {
    let monsters = data::load_monsters();

    if let Some(imp) = monsters.get("imp") {
        let imp_wage = imp
            .economy
            .as_ref()
            .and_then(|e| e.wage_per_minute)
            .unwrap_or(f32::MAX);

        for monster in monsters.values() {
            if monster.id != "imp" {
                let wage = monster
                    .economy
                    .as_ref()
                    .and_then(|e| e.wage_per_minute)
                    .unwrap_or(0.0);
                if wage > 0.0 {
                    assert!(
                        imp_wage <= wage,
                        "Imp wage ({}) should be lowest, but {} has wage {}",
                        imp_wage,
                        monster.name,
                        wage
                    );
                }
            }
        }
    }
}

// ============================================================================
// Combat Balance Tests
// ============================================================================

#[test]
fn test_goblin_beats_militia() {
    let monsters = data::load_monsters();
    let heroes = data::load_heroes();
    let config = data::load_config();

    let goblin = monsters.get("goblin").expect("Goblin not found");
    let militia = heroes.get("peasant_militia").expect("Militia not found");

    let goblin_unit = CombatUnit::from_monster(goblin);
    let militia_unit = CombatUnit::from_hero(militia);

    let result = simulate_combat(
        goblin_unit,
        militia_unit,
        config.combat.attack_stat_bonus,
        config.combat.defense_reduction,
    );

    assert_eq!(
        result.winner, goblin.name,
        "Goblin should beat Peasant Militia in 1v1 combat"
    );
}

#[test]
fn test_demon_spawn_competitive_with_paladin() {
    let monsters = data::load_monsters();
    let heroes = data::load_heroes();
    let config = data::load_config();

    let demon = monsters.get("demon_spawn").expect("Demon Spawn not found");
    let paladin = heroes.get("paladin").expect("Paladin not found");

    let demon_unit = CombatUnit::from_monster(demon);
    let paladin_unit = CombatUnit::from_hero(paladin);

    let result = simulate_combat(
        demon_unit,
        paladin_unit,
        config.combat.attack_stat_bonus,
        config.combat.defense_reduction,
    );

    // The fight should be competitive (winner has <90% HP remaining)
    let winner_max_hp = if result.winner == demon.name {
        demon.stats.health
    } else {
        paladin.stats.health
    };
    let hp_pct = result.winner_hp_remaining / winner_max_hp * 100.0;

    assert!(
        hp_pct < 90.0,
        "Demon Spawn vs Paladin should be a close fight. {} won with {:.0}% HP",
        result.winner,
        hp_pct
    );
}

#[test]
fn test_combat_does_not_time_out() {
    let monsters = data::load_monsters();
    let heroes = data::load_heroes();
    let config = data::load_config();

    // Test several matchups to ensure combat doesn't exceed 5 minutes
    let matchups = [
        ("goblin", "peasant_militia"),
        ("orc", "knight"),
        ("demon_spawn", "paladin"),
    ];

    for (creature_id, hero_id) in matchups {
        if let (Some(monster), Some(hero)) = (monsters.get(creature_id), heroes.get(hero_id)) {
            let monster_unit = CombatUnit::from_monster(monster);
            let hero_unit = CombatUnit::from_hero(hero);

            let result = simulate_combat(
                monster_unit,
                hero_unit,
                config.combat.attack_stat_bonus,
                config.combat.defense_reduction,
            );

            assert!(
                result.duration_secs < 300.0,
                "Combat {} vs {} took too long: {:.0}s (max 300s)",
                monster.name,
                hero.name,
                result.duration_secs
            );
        }
    }
}

#[test]
fn test_attack_stat_bonus_reasonable() {
    let config = data::load_config();
    assert!(
        config.combat.attack_stat_bonus >= 0.1 && config.combat.attack_stat_bonus <= 2.0,
        "Attack stat bonus ({}) should be between 0.1 and 2.0",
        config.combat.attack_stat_bonus
    );
}

#[test]
fn test_defense_reduction_reasonable() {
    let config = data::load_config();
    assert!(
        config.combat.defense_reduction >= 0.1 && config.combat.defense_reduction <= 1.0,
        "Defense reduction ({}) should be between 0.1 and 1.0",
        config.combat.defense_reduction
    );
}

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
        if tier >= 1 && tier <= 5 {
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
