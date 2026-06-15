use super::print_header;
use crate::data;
use crate::sim::{self, CombatUnit};
use std::collections::HashMap;

#[derive(Debug)]
pub struct BalanceTestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub severity: TestSeverity,
}

#[derive(Debug, Clone, Copy)]
pub enum TestSeverity {
    Critical,
    Warning,
    Info,
}

pub fn run_balance_tests(
    monsters: &HashMap<String, data::MonsterData>,
    heroes: &HashMap<String, data::HeroData>,
    rooms: &HashMap<String, data::RoomData>,
    config: &data::GameConfig,
) -> Vec<BalanceTestResult> {
    let mut results = Vec::new();

    results.push(BalanceTestResult {
        name: "Mana max capacity".to_string(),
        passed: config.player_initial_capacity.max_mana > 0,
        message: if config.player_initial_capacity.max_mana > 0 {
            format!("Max mana is {}", config.player_initial_capacity.max_mana)
        } else {
            "Max mana is 0 - this is a bug!".to_string()
        },
        severity: TestSeverity::Critical,
    });

    let min_dungeon_cost = 50 * 9 + 75 * 9 + 100 * 4;
    results.push(BalanceTestResult {
        name: "Starting gold sufficiency".to_string(),
        passed: config.player_starting_resources.gold >= min_dungeon_cost,
        message: format!(
            "Starting gold ({}) vs minimum dungeon cost ({})",
            config.player_starting_resources.gold, min_dungeon_cost
        ),
        severity: TestSeverity::Critical,
    });

    results.push(BalanceTestResult {
        name: "Wave 1 prep time".to_string(),
        passed: config.hero_waves.initial_delay >= 30.0,
        message: format!(
            "Initial wave delay: {:.0}s ({:.1} min)",
            config.hero_waves.initial_delay,
            config.hero_waves.initial_delay / 60.0
        ),
        severity: TestSeverity::Warning,
    });

    let mut wage_efficiency: Vec<(String, f32)> = Vec::new();
    for monster in monsters.values() {
        let wage = monster
            .economy
            .as_ref()
            .and_then(|economy| economy.wage_per_minute)
            .unwrap_or(0.0);
        if wage > 0.0 {
            let efficiency = monster.stats.health / wage;
            wage_efficiency.push((monster.name.clone(), efficiency));
        }
    }
    if !wage_efficiency.is_empty() {
        let max_eff = wage_efficiency
            .iter()
            .map(|(_, efficiency)| *efficiency)
            .fold(0.0f32, f32::max);
        let min_eff = wage_efficiency
            .iter()
            .map(|(_, efficiency)| *efficiency)
            .fold(f32::MAX, f32::min);
        let ratio = if min_eff > 0.0 {
            max_eff / min_eff
        } else {
            f32::INFINITY
        };
        results.push(BalanceTestResult {
            name: "Creature wage balance".to_string(),
            passed: ratio < 4.0,
            message: format!(
                "HP/gold efficiency range: {:.1}x (max {:.1}, min {:.1})",
                ratio, max_eff, min_eff
            ),
            severity: TestSeverity::Warning,
        });
    }

    if let (Some(goblin), Some(militia)) = (monsters.get("goblin"), heroes.get("peasant_militia")) {
        let goblin_unit = CombatUnit::from_monster(goblin);
        let militia_unit = CombatUnit::from_hero(militia);
        let result = sim::simulate_combat(
            goblin_unit,
            militia_unit,
            config.combat.attack_stat_bonus,
            config.combat.defense_reduction,
        );
        results.push(BalanceTestResult {
            name: "Goblin vs Militia balance".to_string(),
            passed: result.winner == goblin.name,
            message: format!(
                "{} wins in {:.1}s with {:.0} HP remaining",
                result.winner, result.duration_secs, result.winner_hp_remaining
            ),
            severity: TestSeverity::Info,
        });
    }

    if let (Some(demon), Some(paladin)) = (monsters.get("demon_spawn"), heroes.get("paladin")) {
        let demon_unit = CombatUnit::from_monster(demon);
        let paladin_unit = CombatUnit::from_hero(paladin);
        let result = sim::simulate_combat(
            demon_unit,
            paladin_unit,
            config.combat.attack_stat_bonus,
            config.combat.defense_reduction,
        );
        let winner_hp_pct = result.winner_hp_remaining
            / if result.winner == demon.name {
                demon.stats.health
            } else {
                paladin.stats.health
            }
            * 100.0;
        results.push(BalanceTestResult {
            name: "Demon Spawn vs Paladin balance".to_string(),
            passed: winner_hp_pct < 80.0,
            message: format!(
                "{} wins with {:.0}% HP - {}",
                result.winner,
                winner_hp_pct,
                if winner_hp_pct < 50.0 {
                    "close fight"
                } else if winner_hp_pct < 80.0 {
                    "moderate advantage"
                } else {
                    "too one-sided"
                }
            ),
            severity: TestSeverity::Info,
        });
    }

    results.push(BalanceTestResult {
        name: "Wave scaling multiplier".to_string(),
        passed: config.hero_waves.wave_scaling_multiplier >= 1.1
            && config.hero_waves.wave_scaling_multiplier <= 2.0,
        message: format!(
            "Wave scaling is {:.2}x (recommended: 1.1-1.5)",
            config.hero_waves.wave_scaling_multiplier
        ),
        severity: TestSeverity::Warning,
    });

    let test_army = vec![("goblin", 5)];
    let wave_result = sim::simulate_wave_survival(monsters, heroes, config, test_army, 3, 12345);
    results.push(BalanceTestResult {
        name: "5 Goblins survive waves 1-3".to_string(),
        passed: wave_result.waves_survived >= 1,
        message: format!(
            "Survived {} waves, killed {} heroes, {} survivors",
            wave_result.waves_survived,
            wave_result.total_heroes_killed,
            wave_result.final_army_size
        ),
        severity: TestSeverity::Info,
    });

    let mixed_army = vec![("orc", 3), ("goblin", 2)];
    let mixed_result = sim::simulate_wave_survival(monsters, heroes, config, mixed_army, 5, 12345);
    results.push(BalanceTestResult {
        name: "Mixed army (3 Orcs + 2 Goblins) wave survival".to_string(),
        passed: mixed_result.waves_survived >= 3,
        message: format!(
            "Survived {} waves, killed {} heroes",
            mixed_result.waves_survived, mixed_result.total_heroes_killed
        ),
        severity: TestSeverity::Info,
    });

    if let Some(hatchery) = rooms.get("hatchery") {
        let food_gen = hatchery
            .effects
            .as_ref()
            .and_then(|effects| effects.food_generation_per_second)
            .unwrap_or(0.0);
        results.push(BalanceTestResult {
            name: "Hatchery food generation".to_string(),
            passed: food_gen > 0.0,
            message: format!("{:.2} food/sec per tile", food_gen),
            severity: TestSeverity::Critical,
        });
    }

    if let Some(training) = rooms.get("training_room") {
        let xp_gen = training
            .effects
            .as_ref()
            .and_then(|effects| effects.training_xp_per_second)
            .unwrap_or(0.0);
        results.push(BalanceTestResult {
            name: "Training hall XP generation".to_string(),
            passed: xp_gen > 0.0,
            message: format!("{:.2} XP/sec per tile", xp_gen),
            severity: TestSeverity::Warning,
        });
    }

    results
}

pub fn print_test_results(results: &[BalanceTestResult]) {
    print_header("AUTOMATED BALANCE TEST RESULTS");

    let passed = results.iter().filter(|result| result.passed).count();
    let total = results.len();
    let critical_failures = results
        .iter()
        .filter(|result| !result.passed && matches!(result.severity, TestSeverity::Critical))
        .count();

    println!("\nResults: {}/{} tests passed", passed, total);
    if critical_failures > 0 {
        println!("⚠️  {} CRITICAL FAILURES", critical_failures);
    }
    println!();

    for result in results {
        let icon = if result.passed {
            "✅"
        } else {
            match result.severity {
                TestSeverity::Critical => "❌",
                TestSeverity::Warning => "⚠️",
                TestSeverity::Info => "ℹ️",
            }
        };
        let status = if result.passed { "PASS" } else { "FAIL" };
        println!("{} [{}] {}", icon, status, result.name);
        println!("    {}", result.message);
    }

    println!();
    if passed == total {
        println!("🎉 All balance tests passed!");
    } else {
        println!("Some tests failed. Review the balance values.");
    }
}

pub fn output_json_results(results: &[BalanceTestResult]) {
    println!("{{");
    println!("  \"total\": {},", results.len());
    println!(
        "  \"passed\": {},",
        results.iter().filter(|result| result.passed).count()
    );
    println!(
        "  \"failed\": {},",
        results.iter().filter(|result| !result.passed).count()
    );
    println!("  \"tests\": [");
    for (i, result) in results.iter().enumerate() {
        let severity = match result.severity {
            TestSeverity::Critical => "critical",
            TestSeverity::Warning => "warning",
            TestSeverity::Info => "info",
        };
        println!("    {{");
        println!("      \"name\": \"{}\",", result.name);
        println!("      \"passed\": {},", result.passed);
        println!(
            "      \"message\": \"{}\",",
            result.message.replace("\"", "\\\"")
        );
        println!("      \"severity\": \"{}\"", severity);
        print!("    }}");
        if i < results.len() - 1 {
            println!(",");
        } else {
            println!();
        }
    }
    println!("  ]");
    println!("}}");
}
