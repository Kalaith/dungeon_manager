use super::{print_header, print_subheader};
use crate::data;
use crate::rng::SimpleRng;
use crate::sim::{self, CombatUnit};
use std::collections::HashMap;

pub fn run_simulation_mode(
    monsters: &HashMap<String, data::MonsterData>,
    heroes: &HashMap<String, data::HeroData>,
    config: &data::GameConfig,
) {
    print_header("HEADLESS COMBAT SIMULATION");

    let attack_mult = config.combat.attack_stat_bonus;
    let defense_mult = config.combat.defense_reduction;
    let num_battles = 1000;

    println!("\nRunning {} simulations per matchup...\n", num_battles);

    let matchups = [
        ("goblin", "peasant_militia", "Basic 1v1"),
        ("orc", "knight", "Tank vs Tank"),
        ("demon_spawn", "paladin", "Elite vs Elite"),
        ("troll", "battle_cleric", "Heavy vs Healer"),
        ("warlock", "wizard", "Caster vs Caster"),
    ];

    println!(
        "{:<25} {:>8} {:>8} {:>10} {:>10} {:>12}",
        "Matchup", "M Win%", "H Win%", "Avg Time", "Time Range", "Avg HP%"
    );
    println!("{}", "-".repeat(80));

    for (monster_id, hero_id, label) in matchups {
        if let (Some(monster), Some(hero)) = (monsters.get(monster_id), heroes.get(hero_id)) {
            let attacker = CombatUnit::from_monster(monster);
            let defender = CombatUnit::from_hero(hero);

            let stats = sim::run_mass_battles(
                &attacker,
                &defender,
                num_battles,
                attack_mult,
                defense_mult,
                12345,
            );

            let m_win_pct = stats.attacker_wins as f32 / stats.total_battles as f32 * 100.0;
            let h_win_pct = stats.defender_wins as f32 / stats.total_battles as f32 * 100.0;

            println!(
                "{:<25} {:>7.1}% {:>7.1}% {:>9.1}s {:>4.1}-{:<5.1}s {:>11.1}%",
                label,
                m_win_pct,
                h_win_pct,
                stats.avg_duration,
                stats.min_duration,
                stats.max_duration,
                stats.avg_winner_hp_pct
            );
        }
    }

    print_subheader("Army vs Army Simulations (5v5)");

    let army_matchups = [
        (
            vec![("goblin", 5)],
            vec![("peasant_militia", 5)],
            "5 Goblins vs 5 Militia",
        ),
        (
            vec![("orc", 3), ("goblin", 2)],
            vec![("knight", 2), ("archer", 3)],
            "Mixed vs Mixed",
        ),
        (
            vec![("demon_spawn", 2), ("orc", 3)],
            vec![("paladin", 2), ("knight", 3)],
            "Elite Mix vs Hero Mix",
        ),
    ];

    let mut rng = SimpleRng::new(54321);

    for (m_army, h_army, label) in army_matchups {
        let mut attackers: Vec<CombatUnit> = Vec::new();
        let mut defenders: Vec<CombatUnit> = Vec::new();

        for (id, count) in &m_army {
            if let Some(monster) = monsters.get(*id) {
                for _ in 0..*count {
                    attackers.push(CombatUnit::from_monster(monster));
                }
            }
        }

        for (id, count) in &h_army {
            if let Some(hero) = heroes.get(*id) {
                for _ in 0..*count {
                    defenders.push(CombatUnit::from_hero(hero));
                }
            }
        }

        let atk_count = attackers.len();
        let def_count = defenders.len();

        let result =
            sim::simulate_army_battle(attackers, defenders, attack_mult, defense_mult, &mut rng);

        println!("\n{}", label);
        println!(
            "  Winner: {} ({}v{} -> survivors: {} atk, {} def)",
            result.winner.to_uppercase(),
            atk_count,
            def_count,
            result.survivors_attacker,
            result.survivors_defender
        );
        println!("  Duration: {:.1}s", result.duration_secs);
    }
}

pub fn run_wave_mode(
    monsters: &HashMap<String, data::MonsterData>,
    heroes: &HashMap<String, data::HeroData>,
    config: &data::GameConfig,
) {
    print_header("WAVE SURVIVAL SIMULATION");

    let army_compositions = [
        (vec![("imp", 5)], "5 Imps"),
        (vec![("goblin", 5)], "5 Goblins"),
        (vec![("orc", 3), ("goblin", 2)], "3 Orcs + 2 Goblins"),
        (vec![("demon_spawn", 2), ("orc", 3)], "2 Demons + 3 Orcs"),
        (
            vec![("demon_spawn", 3), ("warlock", 2)],
            "3 Demons + 2 Warlocks",
        ),
    ];

    println!("\nSimulating 15 waves for each army composition...\n");
    println!(
        "{:<30} {:>8} {:>12} {:>10} {:>10}",
        "Army", "Waves", "Heroes Killed", "Survivors", "Gold"
    );
    println!("{}", "-".repeat(75));

    for (army, label) in army_compositions {
        let result = sim::simulate_wave_survival(monsters, heroes, config, army, 15, 99999);

        println!(
            "{:<30} {:>8} {:>12} {:>10} {:>10}",
            label,
            result.waves_survived,
            result.total_heroes_killed,
            result.final_army_size,
            result.final_gold
        );
    }

    print_subheader("Multiple Runs (variance check)");

    let test_army = vec![("orc", 3), ("goblin", 2)];
    println!("\nRunning 10 simulations with '3 Orcs + 2 Goblins':\n");

    let mut total_waves = 0;
    let mut min_waves = u32::MAX;
    let mut max_waves = 0;

    for seed in 0..10 {
        let result = sim::simulate_wave_survival(
            monsters,
            heroes,
            config,
            test_army.clone(),
            15,
            seed * 1000 + 42,
        );
        total_waves += result.waves_survived;
        min_waves = min_waves.min(result.waves_survived);
        max_waves = max_waves.max(result.waves_survived);
        println!(
            "  Run {}: {} waves survived, {} heroes killed",
            seed + 1,
            result.waves_survived,
            result.total_heroes_killed
        );
    }

    println!(
        "\n  Average: {:.1} waves (range: {}-{})",
        total_waves as f32 / 10.0,
        min_waves,
        max_waves
    );
}

pub fn run_economy_mode(
    _monsters: &HashMap<String, data::MonsterData>,
    rooms: &HashMap<String, data::RoomData>,
    config: &data::GameConfig,
) {
    print_header("ECONOMY SIMULATION OVER TIME");

    let mut gold = config.player_starting_resources.gold as f32;
    let mut mana = config.player_starting_resources.mana as f32;
    let mut food = config.player_starting_resources.food as f32;

    let hatchery_tiles = 9;

    let food_per_sec = rooms
        .get("hatchery")
        .and_then(|room| room.effects.as_ref())
        .and_then(|effects| effects.food_generation_per_second)
        .unwrap_or(0.5)
        * hatchery_tiles as f32;

    let mana_per_sec = rooms
        .get("library")
        .and_then(|room| room.effects.as_ref())
        .and_then(|effects| effects.mana_generation_per_second)
        .unwrap_or(0.0)
        * 9.0;

    let army_wage_per_min = 15.0;

    println!("\nSimulating 10 minutes of gameplay...");
    println!(
        "Initial: {} gold, {} mana, {} food",
        gold as i32, mana as i32, food as i32
    );
    println!(
        "Hatchery: {} tiles ({:.1} food/sec)",
        hatchery_tiles, food_per_sec
    );
    println!("Army wage: {:.1} gold/min\n", army_wage_per_min);

    println!(
        "{:>4} {:>10} {:>10} {:>10} {:>15}",
        "Min", "Gold", "Mana", "Food", "Notes"
    );
    println!("{}", "-".repeat(55));

    let wave_times = [120.0, 240.0, 360.0, 480.0, 600.0];
    let mut wave_idx = 0;

    for minute in 0..=10 {
        let mut notes = String::new();

        let time_secs = minute as f32 * 60.0;
        if wave_idx < wave_times.len() && time_secs >= wave_times[wave_idx] {
            notes = format!("Wave {} arrives!", wave_idx + 1);
            gold += 30.0;
            wave_idx += 1;
        }

        println!(
            "{:>4} {:>10} {:>10} {:>10} {:>15}",
            minute, gold as i32, mana as i32, food as i32, notes
        );

        gold -= army_wage_per_min;
        mana += mana_per_sec * 60.0;
        food += food_per_sec * 60.0;
        food -= 20.0 * 5.0;

        let max_gold = config.player_initial_capacity.max_gold as f32;
        let max_mana = if config.player_initial_capacity.max_mana > 0 {
            config.player_initial_capacity.max_mana as f32
        } else {
            10000.0
        };

        gold = gold.min(max_gold);
        mana = mana.min(max_mana);
        food = food.max(0.0);
    }

    print_subheader("Analysis");

    if gold < 0.0 {
        println!("⚠️  BANKRUPTCY: Gold went negative!");
        println!("   Consider: Lower wages, faster gold income, or smaller army");
    } else {
        println!("✅ Economy stable after 10 minutes");
    }

    if food < 10.0 {
        println!("⚠️  STARVATION RISK: Food very low");
        println!("   Consider: More hatchery tiles or fewer creatures");
    }
}
