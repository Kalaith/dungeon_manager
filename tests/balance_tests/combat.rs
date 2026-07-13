//! Creature/hero health & wage balance tests, plus combat simulation balance tests.

use super::data;

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
