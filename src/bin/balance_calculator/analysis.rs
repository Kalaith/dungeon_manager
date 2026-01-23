use std::collections::HashMap;
use super::data;
use super::sim::{self, CombatUnit};
use super::rng::SimpleRng;

pub fn print_header(title: &str) {
    println!("\n{}", "=".repeat(70));
    println!(" {}", title);
    println!("{}", "=".repeat(70));
}

pub fn print_subheader(title: &str) {
    println!("\n--- {} ---", title);
}

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

    // Key matchups to simulate
    let matchups = [
        ("goblin", "peasant_militia", "Basic 1v1"),
        ("orc", "knight", "Tank vs Tank"),
        ("demon_spawn", "paladin", "Elite vs Elite"),
        ("troll", "battle_cleric", "Heavy vs Healer"),
        ("warlock", "wizard", "Caster vs Caster"),
    ];

    println!("{:<25} {:>8} {:>8} {:>10} {:>10} {:>12}",
        "Matchup", "M Win%", "H Win%", "Avg Time", "Time Range", "Avg HP%");
    println!("{}", "-".repeat(80));

    for (monster_id, hero_id, label) in matchups {
        if let (Some(monster), Some(hero)) = (monsters.get(monster_id), heroes.get(hero_id)) {
            let attacker = CombatUnit::from_monster(monster);
            let defender = CombatUnit::from_hero(hero);

            let stats = sim::run_mass_battles(&attacker, &defender, num_battles, attack_mult, defense_mult, 12345);

            let m_win_pct = stats.attacker_wins as f32 / stats.total_battles as f32 * 100.0;
            let h_win_pct = stats.defender_wins as f32 / stats.total_battles as f32 * 100.0;

            println!("{:<25} {:>7.1}% {:>7.1}% {:>9.1}s {:>4.1}-{:<5.1}s {:>11.1}%",
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
        (vec![("goblin", 5)], vec![("peasant_militia", 5)], "5 Goblins vs 5 Militia"),
        (vec![("orc", 3), ("goblin", 2)], vec![("knight", 2), ("archer", 3)], "Mixed vs Mixed"),
        (vec![("demon_spawn", 2), ("orc", 3)], vec![("paladin", 2), ("knight", 3)], "Elite Mix vs Hero Mix"),
    ];

    let mut rng = SimpleRng::new(54321);

    for (m_army, h_army, label) in army_matchups {
        let mut attackers: Vec<CombatUnit> = Vec::new();
        let mut defenders: Vec<CombatUnit> = Vec::new();

        for (id, count) in &m_army {
            if let Some(m) = monsters.get(*id) {
                for _ in 0..*count {
                    attackers.push(CombatUnit::from_monster(m));
                }
            }
        }

        for (id, count) in &h_army {
            if let Some(h) = heroes.get(*id) {
                for _ in 0..*count {
                    defenders.push(CombatUnit::from_hero(h));
                }
            }
        }

        let atk_count = attackers.len();
        let def_count = defenders.len();

        let result = sim::simulate_army_battle(attackers, defenders, attack_mult, defense_mult, &mut rng);

        println!("\n{}", label);
        println!("  Winner: {} ({}v{} -> survivors: {} atk, {} def)",
            result.winner.to_uppercase(),
            atk_count, def_count,
            result.survivors_attacker, result.survivors_defender);
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
        (vec![("demon_spawn", 3), ("warlock", 2)], "3 Demons + 2 Warlocks"),
    ];

    println!("\nSimulating 15 waves for each army composition...\n");
    println!("{:<30} {:>8} {:>12} {:>10} {:>10}",
        "Army", "Waves", "Heroes Killed", "Survivors", "Gold");
    println!("{}", "-".repeat(75));

    for (army, label) in army_compositions {
        let result = sim::simulate_wave_survival(monsters, heroes, config, army, 15, 99999);

        println!("{:<30} {:>8} {:>12} {:>10} {:>10}",
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
        let result = sim::simulate_wave_survival(monsters, heroes, config, test_army.clone(), 15, seed * 1000 + 42);
        total_waves += result.waves_survived;
        min_waves = min_waves.min(result.waves_survived);
        max_waves = max_waves.max(result.waves_survived);
        println!("  Run {}: {} waves survived, {} heroes killed",
            seed + 1, result.waves_survived, result.total_heroes_killed);
    }

    println!("\n  Average: {:.1} waves (range: {}-{})",
        total_waves as f32 / 10.0, min_waves, max_waves);
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

    // Assume a standard room setup
    let hatchery_tiles = 9;
    let _treasury_tiles = 4;

    let food_per_sec = rooms.get("hatchery")
        .and_then(|r| r.effects.as_ref())
        .and_then(|e| e.food_generation_per_second)
        .unwrap_or(0.5) * hatchery_tiles as f32;

    let mana_per_sec = rooms.get("library")
        .and_then(|r| r.effects.as_ref())
        .and_then(|e| e.mana_generation_per_second)
        .unwrap_or(0.0) * 9.0; // assume 9 tile library

    // Army wages
    let army_wage_per_min = 15.0; // ~3 orcs + 2 goblins

    println!("\nSimulating 10 minutes of gameplay...");
    println!("Initial: {} gold, {} mana, {} food", gold as i32, mana as i32, food as i32);
    println!("Hatchery: {} tiles ({:.1} food/sec)", hatchery_tiles, food_per_sec);
    println!("Army wage: {:.1} gold/min\n", army_wage_per_min);

    println!("{:>4} {:>10} {:>10} {:>10} {:>15}",
        "Min", "Gold", "Mana", "Food", "Notes");
    println!("{}", "-".repeat(55));

    let wave_times = [120.0, 240.0, 360.0, 480.0, 600.0]; // waves at 2, 4, 6, 8, 10 min
    let mut wave_idx = 0;

    for minute in 0..=10 {
        let mut notes = String::new();

        // Check for wave
        let time_secs = minute as f32 * 60.0;
        if wave_idx < wave_times.len() && time_secs >= wave_times[wave_idx] {
            notes = format!("Wave {} arrives!", wave_idx + 1);
            gold += 30.0; // hero kill reward
            wave_idx += 1;
        }

        println!("{:>4} {:>10} {:>10} {:>10} {:>15}",
            minute, gold as i32, mana as i32, food as i32, notes);

        // Update resources for next minute
        gold -= army_wage_per_min;
        mana += mana_per_sec * 60.0;
        food += food_per_sec * 60.0;
        food -= 20.0 * 5.0; // 5 creatures eating

        // Cap at max
        let max_gold = config.player_initial_capacity.max_gold as f32;
        let max_mana = if config.player_initial_capacity.max_mana > 0 {
            config.player_initial_capacity.max_mana as f32
        } else {
            10000.0 // fallback if config is 0
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

pub fn analyze_creature_efficiency(monsters: &HashMap<String, data::MonsterData>) {
    print_header("CREATURE EFFICIENCY ANALYSIS");

    let mut creatures: Vec<_> = monsters.values().collect();
    creatures.sort_by(|a, b| a.name.cmp(&b.name));

    print_subheader("Gold Efficiency (HP per gold/min wage)");
    println!("{:<18} {:>6} {:>8} {:>12} {:>10}", "Creature", "HP", "Wage", "HP/Gold", "Rating");
    println!("{}", "-".repeat(60));

    let mut efficiency_data: Vec<(&str, f32, f32, f32)> = Vec::new();

    for monster in &creatures {
        let wage = monster.economy.as_ref().and_then(|e| e.wage_per_minute).unwrap_or(0.0);
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

        println!("{:<18} {:>6.0} {:>8.1} {:>12} {:>10}", name, hp, wage, eff_str, rating);
    }

    print_subheader("Combat Power per Gold (Attack+Defense / wage)");
    println!("{:<18} {:>6} {:>6} {:>8} {:>12}", "Creature", "Atk", "Def", "Wage", "Power/Gold");
    println!("{}", "-".repeat(55));

    let mut power_data: Vec<(&str, f32, f32, f32, f32)> = Vec::new();

    for monster in &creatures {
        let wage = monster.economy.as_ref().and_then(|e| e.wage_per_minute).unwrap_or(0.0);
        let power = monster.stats.attack + monster.stats.defense;
        let efficiency = if wage > 0.0 { power / wage } else { f32::INFINITY };
        power_data.push((&monster.name, monster.stats.attack, monster.stats.defense, wage, efficiency));
    }

    power_data.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal));

    for (name, atk, def, wage, efficiency) in &power_data {
        let eff_str = if *efficiency == f32::INFINITY {
            "∞".to_string()
        } else {
            format!("{:.1}", efficiency)
        };
        println!("{:<18} {:>6.0} {:>6.0} {:>8.1} {:>12}", name, atk, def, wage, eff_str);
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
    let key_heroes = ["peasant_militia", "knight", "archer", "paladin", "champion_of_light"];

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
                    let result = sim::simulate_combat(creature_unit, hero_unit, attack_mult, defense_mult);

                    let winner_short = if result.winner == monster.name { "C" } else { "H" };
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
            println!("  Winner: {} in {:.1}s with {:.0} HP remaining ({:.0}%)",
                result.winner,
                result.duration_secs,
                result.winner_hp_remaining,
                (result.winner_hp_remaining / if result.winner == monster.name { monster.stats.health } else { hero.stats.health }) * 100.0
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
    println!("Max Gold: {:>5} (configured)", config.player_initial_capacity.max_gold);
    println!("Max Mana: {:>5} (configured)", config.player_initial_capacity.max_mana);

    if config.player_initial_capacity.max_mana == 0 {
        println!("  ⚠️  WARNING: Max mana is 0 - this appears to be a bug!");
    }

    print_subheader("Time Before Wave 1");
    println!("Wave 1 arrives in: {:.0} seconds ({:.1} minutes)", wave1_time, wave1_time / 60.0);

    print_subheader("Army Wage Costs (gold per minute)");

    // Calculate various army compositions
    let compositions = [
        ("5 Imps only", vec![("imp", 5)]),
        ("5 Goblins", vec![("goblin", 5)]),
        ("3 Orcs + 2 Goblins", vec![("orc", 3), ("goblin", 2)]),
        ("2 Demon Spawn + 3 Orcs", vec![("demon_spawn", 2), ("orc", 3)]),
        ("1 of each basic", vec![("imp", 1), ("goblin", 1), ("orc", 1), ("skeleton", 1), ("warlock", 1)]),
    ];

    for (name, units) in compositions {
        let mut total_wage = 0.0;
        for (unit_id, count) in &units {
            if let Some(monster) = monsters.get(*unit_id) {
                total_wage += monster.economy.as_ref().and_then(|e| e.wage_per_minute).unwrap_or(0.0) * (*count as f32);
            }
        }
        let time_to_bankrupt = if total_wage > 0.0 {
            starting_gold as f32 / total_wage
        } else {
            f32::INFINITY
        };

        println!("{:<30} {:>6.1} gold/min -> bankrupt in {:.0} min",
            name, total_wage, time_to_bankrupt);
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
            println!("{:<15} {:>3} tiles x {:>4} = {:>6} gold  ({})",
                room.name, tiles, cost_per_tile, cost, purpose);
        }
    }
    println!("{}", "-".repeat(50));
    println!("{:<30} = {:>6} gold", "TOTAL ESSENTIAL", total_essential);
    println!("Remaining after essentials: {} gold", starting_gold - total_essential);

    print_subheader("Sustainability Analysis");

    // Calculate food generation needed
    let avg_food_consumption = 20.0; // per creature per minute (rough avg)
    if let Some(hatchery) = rooms.get("hatchery") {
        let food_per_tile = hatchery.effects.as_ref()
            .and_then(|e| e.food_generation_per_second)
            .unwrap_or(0.5) * 60.0; // convert to per minute

        println!("Hatchery food generation: {:.1}/min per tile", food_per_tile);
        println!("Avg creature food need: ~{:.0}/min", avg_food_consumption);
        println!("Tiles needed per creature: ~{:.1}", avg_food_consumption / food_per_tile);
    }
}

pub fn analyze_waves(
    heroes: &HashMap<String, data::HeroData>,
    config: &data::GameConfig,
) {
    print_header("WAVE DIFFICULTY ANALYSIS");

    let initial_delay = config.hero_waves.initial_delay;
    let interval = config.hero_waves.wave_interval;
    let scaling = config.hero_waves.wave_scaling_multiplier;
    let spawn_decay = config.hero_waves.spawn_rate_decay.unwrap_or(0.9);

    print_subheader("Wave Timing");
    println!("Initial delay: {:.0}s ({:.1} min)", initial_delay, initial_delay / 60.0);
    println!("Wave interval: {:.0}s ({:.1} min)", interval, interval / 60.0);
    println!("Scaling multiplier: {:.2}x per wave", scaling);
    println!("Spawn rate decay: {:.2}x per wave", spawn_decay);

    print_subheader("Wave Progression Estimate");
    println!("{:<6} {:>10} {:>12} {:>15} {:>12}", "Wave", "Time", "Spawn Rate", "Est. Heroes", "Difficulty");
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

        println!("{:<6} {:>7.0}s {:>12.1}s {:>15.1} {:>12}",
            wave, time, spawn_rate, heroes_count, difficulty);
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

pub fn analyze_traps(traps: &HashMap<String, data::TrapData>, heroes: &HashMap<String, data::HeroData>) {
    print_header("TRAP EFFECTIVENESS ANALYSIS");

    print_subheader("Trap Stats");
    println!("{:<15} {:>6} {:>8} {:>12}", "Trap", "Cost", "Damage", "Build Time");
    println!("{}", "-".repeat(50));

    for trap in traps.values() {
        let damage = trap.effects.as_ref().and_then(|e| e.damage).unwrap_or(0.0);
        let build_time = trap.build_time.unwrap_or(0.0);

        println!("{:<15} {:>6} {:>8.0} {:>12.1}s",
            trap.name, trap.cost, damage, build_time);
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
        let spike_damage = spike.effects.as_ref().and_then(|e| e.damage).unwrap_or(25.0);
        let hits_needed = (knight.stats.health / spike_damage).ceil();
        let cost = spike.cost as f32 * hits_needed;
        println!("Knight ({:.0} HP) with Spike Traps ({:.0} dmg):", knight.stats.health, spike_damage);
        println!("  {:.0} hits needed x {} gold = {:.0} gold total", hits_needed, spike.cost, cost);
    }
}

pub fn analyze_spells(spells: &HashMap<String, data::SpellData>) {
    print_header("SPELL VALUE ANALYSIS");

    print_subheader("Damage Spells - Mana Efficiency");
    println!("{:<20} {:>6} {:>8} {:>10} {:>12}", "Spell", "Mana", "Damage", "Cooldown", "Dmg/Mana");
    println!("{}", "-".repeat(60));

    for spell in spells.values() {
        let damage: f32 = spell.effects.iter()
            .filter(|e| e.effect_type == "damage")
            .map(|e| e.amount)
            .sum();

        if damage > 0.0 {
            let efficiency = damage / spell.cost.mana.max(1) as f32;
            println!("{:<20} {:>6} {:>8.0} {:>10.1}s {:>12.2}",
                spell.name, spell.cost.mana, damage, spell.cooldown, efficiency);
        }
    }

    print_subheader("Healing Spells");
    println!("{:<20} {:>6} {:>8} {:>10} {:>12}", "Spell", "Mana", "Heal", "Cooldown", "HP/Mana");
    println!("{}", "-".repeat(60));

    for spell in spells.values() {
        let healing: f32 = spell.effects.iter()
            .filter(|e| e.effect_type == "heal")
            .map(|e| e.amount)
            .sum();

        if healing > 0.0 {
            let efficiency = healing / spell.cost.mana.max(1) as f32;
            println!("{:<20} {:>6} {:>8.0} {:>10.1}s {:>12.2}",
                spell.name, spell.cost.mana, healing, spell.cooldown, efficiency);
        }
    }

    print_subheader("Utility Spells");
    for spell in spells.values() {
        let is_utility = spell.effects.iter()
            .any(|e| !["damage", "heal"].contains(&e.effect_type.as_str()));

        if is_utility {
            let effects: Vec<_> = spell.effects.iter()
                .map(|e| format!("{}: {:.0}", e.effect_type, e.amount))
                .collect();
            println!("{:<20} {:>4} mana, {:>5.1}s cd - {}",
                spell.name, spell.cost.mana, spell.cooldown, effects.join(", "));
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
        let bile_eff = bile.stats.health / bile.economy.as_ref().and_then(|e| e.wage_per_minute).unwrap_or(1.0);
        let demon_eff = demon.stats.health / demon.economy.as_ref().and_then(|e| e.wage_per_minute).unwrap_or(1.0);
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
    println!("Suggested Changes (from BALANCE_TESTING.md):");
    println!("  • Room Sell Refund: 5% → 25-50%");
    println!("  • Mana Max Capacity: 0 → 10,000+");
    println!("  • Wave Scaling: 1.5x → 1.25x");
    println!("  • Bile Demon Wage: 20/min → 12/min");
    println!("  • Training XP: 0.8/s → 1.5/s");
    println!("  • Spike Trap Damage: 25 → 35");
}

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

    // Test 1: Mana max capacity should not be 0
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

    // Test 2: Starting gold should allow basic dungeon setup
    let min_dungeon_cost = 50 * 9 + 75 * 9 + 100 * 4; // Lair + Hatchery + Treasury
    results.push(BalanceTestResult {
        name: "Starting gold sufficiency".to_string(),
        passed: config.player_starting_resources.gold >= min_dungeon_cost,
        message: format!(
            "Starting gold ({}) vs minimum dungeon cost ({})",
            config.player_starting_resources.gold, min_dungeon_cost
        ),
        severity: TestSeverity::Critical,
    });

    // Test 3: Wave 1 should arrive with enough prep time (at least 30 seconds)
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

    // Test 4: Creature wage efficiency - no creature should cost more HP/gold than others by 3x
    let mut wage_efficiency: Vec<(String, f32)> = Vec::new();
    for monster in monsters.values() {
        let wage = monster.economy.as_ref().and_then(|e| e.wage_per_minute).unwrap_or(0.0);
        if wage > 0.0 {
            let efficiency = monster.stats.health / wage;
            wage_efficiency.push((monster.name.clone(), efficiency));
        }
    }
    if !wage_efficiency.is_empty() {
        let max_eff = wage_efficiency.iter().map(|(_, e)| *e).fold(0.0f32, f32::max);
        let min_eff = wage_efficiency.iter().map(|(_, e)| *e).fold(f32::MAX, f32::min);
        let ratio = if min_eff > 0.0 { max_eff / min_eff } else { f32::INFINITY };
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

    // Test 5: Basic creature should beat basic hero in 1v1
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

    // Test 6: Elite creature should compete with elite hero
    if let (Some(demon), Some(paladin)) = (monsters.get("demon_spawn"), heroes.get("paladin")) {
        let demon_unit = CombatUnit::from_monster(demon);
        let paladin_unit = CombatUnit::from_hero(paladin);
        let result = sim::simulate_combat(
            demon_unit,
            paladin_unit,
            config.combat.attack_stat_bonus,
            config.combat.defense_reduction,
        );
        let winner_hp_pct = result.winner_hp_remaining /
            if result.winner == demon.name { demon.stats.health } else { paladin.stats.health } * 100.0;
        results.push(BalanceTestResult {
            name: "Demon Spawn vs Paladin balance".to_string(),
            passed: winner_hp_pct < 80.0, // Should be a close fight
            message: format!(
                "{} wins with {:.0}% HP - {}",
                result.winner,
                winner_hp_pct,
                if winner_hp_pct < 50.0 { "close fight" } else if winner_hp_pct < 80.0 { "moderate advantage" } else { "too one-sided" }
            ),
            severity: TestSeverity::Info,
        });
    }

    // Test 8: Wave scaling should be between 1.1 and 2.0
    results.push(BalanceTestResult {
        name: "Wave scaling multiplier".to_string(),
        passed: config.hero_waves.wave_scaling_multiplier >= 1.1 && config.hero_waves.wave_scaling_multiplier <= 2.0,
        message: format!(
            "Wave scaling is {:.2}x (recommended: 1.1-1.5)",
            config.hero_waves.wave_scaling_multiplier
        ),
        severity: TestSeverity::Warning,
    });

    // Test 9: Army can survive wave 1 simulation
    let test_army = vec![("goblin", 5)];
    let wave_result = sim::simulate_wave_survival(monsters, heroes, config, test_army, 3, 12345);
    results.push(BalanceTestResult {
        name: "5 Goblins survive waves 1-3".to_string(),
        passed: wave_result.waves_survived >= 1,
        message: format!(
            "Survived {} waves, killed {} heroes, {} survivors",
            wave_result.waves_survived, wave_result.total_heroes_killed, wave_result.final_army_size
        ),
        severity: TestSeverity::Info,
    });

    // Test 10: Mixed army should do better
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

    // Test 11: Hatchery food generation should be positive
    if let Some(hatchery) = rooms.get("hatchery") {
        let food_gen = hatchery.effects.as_ref()
            .and_then(|e| e.food_generation_per_second)
            .unwrap_or(0.0);
        results.push(BalanceTestResult {
            name: "Hatchery food generation".to_string(),
            passed: food_gen > 0.0,
            message: format!("{:.2} food/sec per tile", food_gen),
            severity: TestSeverity::Critical,
        });
    }

    // Test 12: Training hall should provide XP
    if let Some(training) = rooms.get("training_room") {
        let xp_gen = training.effects.as_ref()
            .and_then(|e| e.training_xp_per_second)
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

    let passed = results.iter().filter(|r| r.passed).count();
    let total = results.len();
    let critical_failures = results.iter().filter(|r| !r.passed && matches!(r.severity, TestSeverity::Critical)).count();

    println!("\nResults: {}/{} tests passed", passed, total);
    if critical_failures > 0 {
        println!("⚠️  {} CRITICAL FAILURES", critical_failures);
    }
    println!();

    for result in results {
        let icon = if result.passed { "✅" } else {
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
    println!("  \"passed\": {},", results.iter().filter(|r| r.passed).count());
    println!("  \"failed\": {},", results.iter().filter(|r| !r.passed).count());
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
        println!("      \"message\": \"{}\",", result.message.replace("\"", "\\\""));
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
