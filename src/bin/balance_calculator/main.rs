#![allow(dead_code)]

mod analysis;
mod data;
mod rng;
mod sim;

use analysis::TestSeverity;
use std::env;

fn print_usage() {
    println!("Usage: balance_calculator [MODE] [OPTIONS]");
    println!();
    println!("Modes:");
    println!("  (none)      Run full analysis with all reports");
    println!("  simulate    Run headless combat simulations");
    println!("  waves       Run wave survival simulations");
    println!("  economy     Run economy over time simulation");
    println!("  test        Run automated balance tests");
    println!("  test-json   Run tests and output JSON results");
    println!();
    println!("Examples:");
    println!("  cargo run --bin balance_calculator");
    println!("  cargo run --bin balance_calculator simulate");
    println!("  cargo run --bin balance_calculator test");
    println!("  cargo run --bin balance_calculator test-json > results.json");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("");

    // Load all game data
    let monsters = data::load_monsters();
    let heroes = data::load_heroes();
    let rooms = data::load_rooms();
    let traps = data::load_traps();
    let spells = data::load_spells();
    let config = data::load_config();

    match mode {
        "help" | "--help" | "-h" => {
            print_usage();
        }
        "simulate" => {
            println!(
                "\n{}",
                "╔══════════════════════════════════════════════════════════════════════╗"
            );
            println!("║           DUNGEON MANAGER - COMBAT SIMULATION                        ║");
            println!("╚══════════════════════════════════════════════════════════════════════╝");
            println!("\nLoading game data...");
            println!(
                "  Loaded {} creatures, {} heroes",
                monsters.len(),
                heroes.len()
            );
            analysis::run_simulation_mode(&monsters, &heroes, &config);
        }
        "waves" => {
            println!(
                "\n{}",
                "╔══════════════════════════════════════════════════════════════════════╗"
            );
            println!("║           DUNGEON MANAGER - WAVE SIMULATION                          ║");
            println!("╚══════════════════════════════════════════════════════════════════════╝");
            println!("\nLoading game data...");
            println!(
                "  Loaded {} creatures, {} heroes",
                monsters.len(),
                heroes.len()
            );
            analysis::run_wave_mode(&monsters, &heroes, &config);
        }
        "economy" => {
            println!(
                "\n{}",
                "╔══════════════════════════════════════════════════════════════════════╗"
            );
            println!("║           DUNGEON MANAGER - ECONOMY SIMULATION                       ║");
            println!("╚══════════════════════════════════════════════════════════════════════╝");
            println!("\nLoading game data...");
            println!(
                "  Loaded {} creatures, {} rooms",
                monsters.len(),
                rooms.len()
            );
            analysis::run_economy_mode(&monsters, &rooms, &config);
        }
        "test" => {
            let results = analysis::run_balance_tests(&monsters, &heroes, &rooms, &config);
            analysis::print_test_results(&results);

            // Exit with error code if any critical tests failed
            let critical_failures = results
                .iter()
                .filter(|r| !r.passed && matches!(r.severity, TestSeverity::Critical))
                .count();
            if critical_failures > 0 {
                std::process::exit(1);
            }
        }
        "test-json" => {
            let results = analysis::run_balance_tests(&monsters, &heroes, &rooms, &config);
            analysis::output_json_results(&results);

            let critical_failures = results
                .iter()
                .filter(|r| !r.passed && matches!(r.severity, TestSeverity::Critical))
                .count();
            if critical_failures > 0 {
                std::process::exit(1);
            }
        }
        "" => {
            println!(
                "\n{}",
                "╔══════════════════════════════════════════════════════════════════════╗"
            );
            println!("║           DUNGEON MANAGER - BALANCE CALCULATOR v1.0                   ║");
            println!("╚══════════════════════════════════════════════════════════════════════╝");

            println!("\nLoading game data...");
            println!("  Loaded {} creatures", monsters.len());
            println!("  Loaded {} heroes", heroes.len());
            println!("  Loaded {} rooms", rooms.len());
            println!("  Loaded {} traps", traps.len());
            println!("  Loaded {} spells", spells.len());

            // Run all analyses
            analysis::analyze_creature_efficiency(&monsters);
            analysis::analyze_combat_matchups(&monsters, &heroes, &config);
            analysis::analyze_economy(&monsters, &rooms, &config);
            analysis::analyze_waves(&heroes, &config);
            analysis::analyze_traps(&traps, &heroes);
            analysis::analyze_spells(&spells);
            analysis::print_recommendations(&monsters, &config);

            println!("\n{}", "=".repeat(70));
            println!(" Analysis Complete!");
            println!(" Run with 'test' mode for automated balance verification.");
            println!("{}", "=".repeat(70));
        }
        other => {
            eprintln!("Unknown mode: {}", other);
            eprintln!();
            print_usage();
            std::process::exit(1);
        }
    }
}
