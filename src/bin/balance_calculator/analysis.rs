use super::data;
use super::sim::{self, CombatUnit};
use std::collections::HashMap;

mod balance_tests;
mod modes;

pub use balance_tests::{output_json_results, print_test_results, run_balance_tests, TestSeverity};
pub use modes::{run_economy_mode, run_simulation_mode, run_wave_mode};

pub fn print_header(title: &str) {
    println!("\n{}", "=".repeat(70));
    println!(" {}", title);
    println!("{}", "=".repeat(70));
}

pub fn print_subheader(title: &str) {
    println!("\n--- {} ---", title);
}

pub fn analyze_creature_efficiency(monsters: &HashMap<String, data::MonsterData>) {
    print_header("CREATURE EFFICIENCY ANALYSIS");

    let mut creatures: Vec<_> = monsters.values().collect();
    creatures.sort_by(|a, b| a.name.cmp(&b.name));

    print_subheader("Gold Efficiency (HP per gold/min wage)");
    println!(
        "{:<18} {:>6} {:>8} {:>12} {:>10}",
        "Creature", "HP", "Wage", "HP/Gold", "Rating"
    );
    println!("{}", "-".repeat(60));

    let mut efficiency_data: Vec<(&str, f32, f32, f32)> = Vec::new();

    for monster in &creatures {
        let wage = monster
            .economy
            .as_ref()
            .and_then(|e| e.wage_per_minute)
            .unwrap_or(0.0);
        let hp = monster.stats.health;
        let efficiency = if wage > 0.0 { hp / wage } else { f32::INFINITY };
        efficiency_data.push((&monster.name, hp, wage, efficiency));
    }

    efficiency_data.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));

    for (name, hp, wage, efficiency) in &efficiency_data {
        let rating = if *efficiency == f32::INFINITY {
            "FREE!".to_string()
        } else if *efficiency > 50.0 {
            "GREAT".to_string()
        } else if *efficiency > 30.0 {
            "Good".to_string()
        } else if *efficiency > 15.0 {
            "Fair".to_string()
        } else {
            "POOR".to_string()
        };

        let eff_str = if *efficiency == f32::INFINITY {
            "∞".to_string()
        } else {
            format!("{:.1}", efficiency)
        };

        println!(
            "{:<18} {:>6.0} {:>8.1} {:>12} {:>10}",
            name, hp, wage, eff_str, rating
        );
    }

    print_subheader("Combat Power per Gold (Attack+Defense / wage)");
    println!(
        "{:<18} {:>6} {:>6} {:>8} {:>12}",
        "Creature", "Atk", "Def", "Wage", "Power/Gold"
    );
    println!("{}", "-".repeat(55));

    let mut power_data: Vec<(&str, f32, f32, f32, f32)> = Vec::new();

    for monster in &creatures {
        let wage = monster
            .economy
            .as_ref()
            .and_then(|e| e.wage_per_minute)
            .unwrap_or(0.0);
        let power = monster.stats.attack + monster.stats.defense;
        let efficiency = if wage > 0.0 {
            power / wage
        } else {
            f32::INFINITY
        };
        power_data.push((
            &monster.name,
            monster.stats.attack,
            monster.stats.defense,
            wage,
            efficiency,
        ));
    }

    power_data.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal));

    for (name, atk, def, wage, efficiency) in &power_data {
        let eff_str = if *efficiency == f32::INFINITY {
            "∞".to_string()
        } else {
            format!("{:.1}", efficiency)
        };
        println!(
            "{:<18} {:>6.0} {:>6.0} {:>8.1} {:>12}",
            name, atk, def, wage, eff_str
        );
    }
}

pub fn analyze_combat_matchups(
    monsters: &HashMap<String, data::MonsterData>,
    heroes: &HashMap<String, data::HeroData>,
    config: &data::GameConfig,
) {
    print_header("COMBAT TIME-TO-KILL MATRIX");

    let attack_mult = config.combat.attack_stat_bonus;
    let defense_mult = config.combat.defense_reduction;

    // Select key creatures and heroes for the matrix
    let key_creatures = ["imp", "goblin", "orc", "demon_spawn", "troll"];
    let key_heroes = [
        "peasant_militia",
        "knight",
        "archer",
        "paladin",
        "champion_of_light",
    ];

    print_subheader("Creature vs Hero (seconds to kill, winner)");

    // Header row
    print!("{:<14}", "");
    for hero_id in &key_heroes {
        if let Some(hero) = heroes.get(*hero_id) {
            print!("{:>14}", &hero.name[..hero.name.len().min(12)]);
        }
    }
    println!();
    println!("{}", "-".repeat(14 + 14 * key_heroes.len()));

    for creature_id in &key_creatures {
        if let Some(monster) = monsters.get(*creature_id) {
            print!("{:<14}", &monster.name[..monster.name.len().min(12)]);

            for hero_id in &key_heroes {
                if let Some(hero) = heroes.get(*hero_id) {
                    let creature_unit = CombatUnit::from_monster(monster);
                    let hero_unit = CombatUnit::from_hero(hero);
                    let result =
                        sim::simulate_combat(creature_unit, hero_unit, attack_mult, defense_mult);

                    let winner_short = if result.winner == monster.name {
                        "C"
                    } else {
                        "H"
                    };
                    print!("{:>10.1}s({})", result.duration_secs, winner_short);
                } else {
                    print!("{:>14}", "-");
                }
            }
            println!();
        }
    }

    println!("\nLegend: C = Creature wins, H = Hero wins");

    print_subheader("1v1 Analysis: Key Matchups");

    let matchups = [
        ("orc", "knight", "Tank vs Tank"),
        ("demon_spawn", "paladin", "Elite vs Elite"),
        ("goblin", "peasant_militia", "Basic vs Basic"),
        ("warlock", "wizard", "Mage vs Mage"),
        ("demon_spawn", "champion_of_light", "Elite vs Boss"),
    ];

    for (creature_id, hero_id, description) in matchups {
        if let (Some(monster), Some(hero)) = (monsters.get(creature_id), heroes.get(hero_id)) {
            let creature_unit = CombatUnit::from_monster(monster);
            let hero_unit = CombatUnit::from_hero(hero);
            let result = sim::simulate_combat(creature_unit, hero_unit, attack_mult, defense_mult);

            println!("\n{}: {} vs {}", description, monster.name, hero.name);
            println!(
                "  Winner: {} in {:.1}s with {:.0} HP remaining ({:.0}%)",
                result.winner,
                result.duration_secs,
                result.winner_hp_remaining,
                (result.winner_hp_remaining
                    / if result.winner == monster.name {
                        monster.stats.health
                    } else {
                        hero.stats.health
                    })
                    * 100.0
            );
        }
    }
}

pub fn analyze_economy(
    monsters: &HashMap<String, data::MonsterData>,
    rooms: &HashMap<String, data::RoomData>,
    config: &data::GameConfig,
) {
    print_header("ECONOMY ANALYSIS");

    let starting_gold = config.player_starting_resources.gold;
    let wave1_time = config.hero_waves.initial_delay;

    print_subheader("Starting Resources");
    println!("Gold:  {:>8}", starting_gold);
    println!("Mana:  {:>8}", config.player_starting_resources.mana);
    println!("Food:  {:>8}", config.player_starting_resources.food);
    println!(
        "Max Gold: {:>5} (configured)",
        config.player_initial_capacity.max_gold
    );
    println!(
        "Max Mana: {:>5} (configured)",
        config.player_initial_capacity.max_mana
    );

    if config.player_initial_capacity.max_mana == 0 {
        println!("  ⚠️  WARNING: Max mana is 0 - this appears to be a bug!");
    }

    print_subheader("Time Before Wave 1");
    println!(
        "Wave 1 arrives in: {:.0} seconds ({:.1} minutes)",
        wave1_time,
        wave1_time / 60.0
    );

    print_subheader("Army Wage Costs (gold per minute)");

    // Calculate various army compositions
    let compositions = [
        ("5 Imps only", vec![("imp", 5)]),
        ("5 Goblins", vec![("goblin", 5)]),
        ("3 Orcs + 2 Goblins", vec![("orc", 3), ("goblin", 2)]),
        (
            "2 Demon Spawn + 3 Orcs",
            vec![("demon_spawn", 2), ("orc", 3)],
        ),
        (
            "1 of each basic",
            vec![
                ("imp", 1),
                ("goblin", 1),
                ("orc", 1),
                ("skeleton", 1),
                ("warlock", 1),
            ],
        ),
    ];

    for (name, units) in compositions {
        let mut total_wage = 0.0;
        for (unit_id, count) in &units {
            if let Some(monster) = monsters.get(*unit_id) {
                total_wage += monster
                    .economy
                    .as_ref()
                    .and_then(|e| e.wage_per_minute)
                    .unwrap_or(0.0)
                    * (*count as f32);
            }
        }
        let time_to_bankrupt = if total_wage > 0.0 {
            starting_gold as f32 / total_wage
        } else {
            f32::INFINITY
        };

        println!(
            "{:<30} {:>6.1} gold/min -> bankrupt in {:.0} min",
            name, total_wage, time_to_bankrupt
        );
    }

    print_subheader("Room Costs (minimum functional size)");

    let essential_rooms = [
        ("lair", 9, "Creature housing"),
        ("hatchery", 9, "Food production"),
        ("treasury", 4, "Gold storage"),
        ("training_room", 9, "Leveling"),
    ];

    let mut total_essential = 0;
    for (room_id, tiles, purpose) in essential_rooms {
        if let Some(room) = rooms.get(room_id) {
            let cost_per_tile = room.build.as_ref().map(|b| b.cost_per_tile).unwrap_or(0);
            let cost = cost_per_tile * tiles;
            total_essential += cost;
            println!(
                "{:<15} {:>3} tiles x {:>4} = {:>6} gold  ({})",
                room.name, tiles, cost_per_tile, cost, purpose
            );
        }
    }
    println!("{}", "-".repeat(50));
    println!("{:<30} = {:>6} gold", "TOTAL ESSENTIAL", total_essential);
    println!(
        "Remaining after essentials: {} gold",
        starting_gold - total_essential
    );

    print_subheader("Sustainability Analysis");

    // Calculate food generation needed
    let avg_food_consumption = 20.0; // per creature per minute (rough avg)
    if let Some(hatchery) = rooms.get("hatchery") {
        let food_per_tile = hatchery
            .effects
            .as_ref()
            .and_then(|e| e.food_generation_per_second)
            .unwrap_or(0.5)
            * 60.0; // convert to per minute

        println!(
            "Hatchery food generation: {:.1}/min per tile",
            food_per_tile
        );
        println!("Avg creature food need: ~{:.0}/min", avg_food_consumption);
        println!(
            "Tiles needed per creature: ~{:.1}",
            avg_food_consumption / food_per_tile
        );
    }
}

pub fn analyze_waves(heroes: &HashMap<String, data::HeroData>, config: &data::GameConfig) {
    print_header("WAVE DIFFICULTY ANALYSIS");

    let initial_delay = config.hero_waves.initial_delay;
    let interval = config.hero_waves.wave_interval;
    let scaling = config.hero_waves.wave_scaling_multiplier;
    let spawn_decay = config.hero_waves.spawn_rate_decay.unwrap_or(0.9);

    print_subheader("Wave Timing");
    println!(
        "Initial delay: {:.0}s ({:.1} min)",
        initial_delay,
        initial_delay / 60.0
    );
    println!(
        "Wave interval: {:.0}s ({:.1} min)",
        interval,
        interval / 60.0
    );
    println!("Scaling multiplier: {:.2}x per wave", scaling);
    println!("Spawn rate decay: {:.2}x per wave", spawn_decay);

    print_subheader("Wave Progression Estimate");
    println!(
        "{:<6} {:>10} {:>12} {:>15} {:>12}",
        "Wave", "Time", "Spawn Rate", "Est. Heroes", "Difficulty"
    );
    println!("{}", "-".repeat(60));

    let base_heroes = 3.0;
    let base_spawn_rate = 60.0;

    for wave in 1..=15 {
        let time = initial_delay + (interval * (wave - 1) as f32);
        let spawn_rate = (base_spawn_rate * spawn_decay.powi(wave - 1)).max(5.0);
        let heroes_count = base_heroes + (wave as f32 * scaling);

        let difficulty = if wave <= 3 {
            "Easy"
        } else if wave <= 6 {
            "Medium"
        } else if wave <= 10 {
            "Hard"
        } else {
            "EXTREME"
        };

        println!(
            "{:<6} {:>7.0}s {:>12.1}s {:>15.1} {:>12}",
            wave, time, spawn_rate, heroes_count, difficulty
        );
    }

    print_subheader("Hero Tier Distribution");
    let mut tiers: HashMap<u32, Vec<&str>> = HashMap::new();
    for hero in heroes.values() {
        let tier = hero.tier.unwrap_or(1);
        tiers.entry(tier).or_default().push(&hero.name);
    }

    for tier in 1..=5 {
        if let Some(names) = tiers.get(&tier) {
            println!("Tier {}: {}", tier, names.join(", "));
        }
    }
}

pub fn analyze_traps(
    traps: &HashMap<String, data::TrapData>,
    heroes: &HashMap<String, data::HeroData>,
) {
    print_header("TRAP EFFECTIVENESS ANALYSIS");

    print_subheader("Trap Stats");
    println!(
        "{:<15} {:>6} {:>8} {:>12}",
        "Trap", "Cost", "Damage", "Build Time"
    );
    println!("{}", "-".repeat(50));

    for trap in traps.values() {
        let damage = trap.effects.as_ref().and_then(|e| e.damage).unwrap_or(0.0);
        let build_time = trap.build_time.unwrap_or(0.0);

        println!(
            "{:<15} {:>6} {:>8.0} {:>12.1}s",
            trap.name, trap.cost, damage, build_time
        );
    }

    print_subheader("Traps Required to Kill (assuming all hits)");

    let key_heroes = ["peasant_militia", "knight", "paladin", "champion_of_light"];

    print!("{:<15}", "");
    for trap in traps.values() {
        print!("{:>12}", &trap.name[..trap.name.len().min(10)]);
    }
    println!();
    println!("{}", "-".repeat(15 + 12 * traps.len()));

    for hero_id in key_heroes {
        if let Some(hero) = heroes.get(hero_id) {
            print!("{:<15}", &hero.name[..hero.name.len().min(13)]);

            for trap in traps.values() {
                let damage = trap.effects.as_ref().and_then(|e| e.damage).unwrap_or(0.0);
                if damage > 0.0 {
                    let hits = (hero.stats.health / damage).ceil();
                    print!("{:>12.0}", hits);
                } else {
                    print!("{:>12}", "-");
                }
            }
            println!();
        }
    }

    print_subheader("Gold Cost to Kill with Traps");

    if let (Some(spike), Some(knight)) = (traps.get("spike_trap"), heroes.get("knight")) {
        let spike_damage = spike
            .effects
            .as_ref()
            .and_then(|e| e.damage)
            .unwrap_or(25.0);
        let hits_needed = (knight.stats.health / spike_damage).ceil();
        let cost = spike.cost as f32 * hits_needed;
        println!(
            "Knight ({:.0} HP) with Spike Traps ({:.0} dmg):",
            knight.stats.health, spike_damage
        );
        println!(
            "  {:.0} hits needed x {} gold = {:.0} gold total",
            hits_needed, spike.cost, cost
        );
    }
}

pub fn analyze_spells(spells: &HashMap<String, data::SpellData>) {
    print_header("SPELL VALUE ANALYSIS");

    print_subheader("Damage Spells - Mana Efficiency");
    println!(
        "{:<20} {:>6} {:>8} {:>10} {:>12}",
        "Spell", "Mana", "Damage", "Cooldown", "Dmg/Mana"
    );
    println!("{}", "-".repeat(60));

    for spell in spells.values() {
        let damage: f32 = spell
            .effects
            .iter()
            .filter(|e| e.effect_type == "damage")
            .map(|e| e.amount)
            .sum();

        if damage > 0.0 {
            let efficiency = damage / spell.cost.mana.max(1) as f32;
            println!(
                "{:<20} {:>6} {:>8.0} {:>10.1}s {:>12.2}",
                spell.name, spell.cost.mana, damage, spell.cooldown, efficiency
            );
        }
    }

    print_subheader("Healing Spells");
    println!(
        "{:<20} {:>6} {:>8} {:>10} {:>12}",
        "Spell", "Mana", "Heal", "Cooldown", "HP/Mana"
    );
    println!("{}", "-".repeat(60));

    for spell in spells.values() {
        let healing: f32 = spell
            .effects
            .iter()
            .filter(|e| e.effect_type == "heal")
            .map(|e| e.amount)
            .sum();

        if healing > 0.0 {
            let efficiency = healing / spell.cost.mana.max(1) as f32;
            println!(
                "{:<20} {:>6} {:>8.0} {:>10.1}s {:>12.2}",
                spell.name, spell.cost.mana, healing, spell.cooldown, efficiency
            );
        }
    }

    print_subheader("Utility Spells");
    for spell in spells.values() {
        let is_utility = spell
            .effects
            .iter()
            .any(|e| !["damage", "heal"].contains(&e.effect_type.as_str()));

        if is_utility {
            let effects: Vec<_> = spell
                .effects
                .iter()
                .map(|e| format!("{}: {:.0}", e.effect_type, e.amount))
                .collect();
            println!(
                "{:<20} {:>4} mana, {:>5.1}s cd - {}",
                spell.name,
                spell.cost.mana,
                spell.cooldown,
                effects.join(", ")
            );
        }
    }
}

pub fn print_recommendations(
    monsters: &HashMap<String, data::MonsterData>,
    config: &data::GameConfig,
) {
    print_header("BALANCE RECOMMENDATIONS");

    println!("\nBased on analysis, consider these adjustments:\n");

    // Check for issues
    let mut issues = Vec::new();

    // Mana cap issue
    if config.player_initial_capacity.max_mana == 0 {
        issues.push("🔴 CRITICAL: max_mana is 0 in config - likely a bug");
    }

    // Bile Demon efficiency
    if let (Some(bile), Some(demon)) = (monsters.get("bile_demon"), monsters.get("demon_spawn")) {
        let bile_eff = bile.stats.health
            / bile
                .economy
                .as_ref()
                .and_then(|e| e.wage_per_minute)
                .unwrap_or(1.0);
        let demon_eff = demon.stats.health
            / demon
                .economy
                .as_ref()
                .and_then(|e| e.wage_per_minute)
                .unwrap_or(1.0);
        if bile_eff < demon_eff * 0.5 {
            issues.push("🟡 Bile Demon (20g/min) is much less efficient than Demon Spawn (8g/min)");
        }
    }

    // Wave scaling
    if config.hero_waves.wave_scaling_multiplier > 1.3 {
        issues.push("🟡 Wave scaling (1.5x) may be too aggressive for casual play");
    }

    // Print issues
    if issues.is_empty() {
        println!("✅ No major balance issues detected!");
    } else {
        println!("Issues Found:");
        for issue in &issues {
            println!("  {}", issue);
        }
    }

    println!("\n{}", "-".repeat(60));
    println!("Suggested Changes (see TODO.md, balance):");
    println!("  • Room Sell Refund: 5% → 25-50%");
    println!("  • Mana Max Capacity: 0 → 10,000+");
    println!("  • Wave Scaling: 1.5x → 1.25x");
    println!("  • Bile Demon Wage: 20/min → 12/min");
    println!("  • Training XP: 0.8/s → 1.5/s");
    println!("  • Spike Trap Damage: 25 → 35");
}
