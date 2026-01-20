//! Balance Calculator - Automated game balance analysis tool
//!
//! Run with: cargo run --bin balance_calculator [mode]
//!
//! Modes:
//!   (none)      - Run standard analysis
//!   simulate    - Run headless combat simulations
//!   waves       - Simulate wave survival scenarios
//!   economy     - Simulate economy over time
//!
//! Analyzes game data and outputs balance metrics for:
//! - Creature efficiency (HP/gold, DPS/gold)
//! - Combat time-to-kill matrices
//! - Economy sustainability
//! - Wave difficulty estimates
//! - Room/trap/spell value analysis

use std::collections::HashMap;
use std::env;

// Include game data at compile time (same as main game)
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
    pub struct SpellData {
        pub id: String,
        pub name: String,
        pub cost: SpellCost,
        pub effects: Vec<SpellEffect>,
        pub cooldown: f32,
    }

    #[derive(Debug, Deserialize)]
    pub struct SpellCost {
        #[serde(default)]
        pub mana: i32,
        #[serde(default)]
        pub gold: i32,
        #[serde(default)]
        pub health: i32,
    }

    #[derive(Debug, Deserialize)]
    pub struct SpellEffect {
        #[serde(rename = "type")]
        pub effect_type: String,
        #[serde(default)]
        pub amount: f32,
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
        let json = include_str!("../../assets/data/monsters.json");
        let list: Vec<MonsterData> = serde_json::from_str(json).expect("Failed to parse monsters.json");
        list.into_iter().map(|m| (m.id.clone(), m)).collect()
    }

    pub fn load_heroes() -> HashMap<String, HeroData> {
        let json = include_str!("../../assets/data/heroes.json");
        let list: Vec<HeroData> = serde_json::from_str(json).expect("Failed to parse heroes.json");
        list.into_iter().map(|h| (h.id.clone(), h)).collect()
    }

    pub fn load_rooms() -> HashMap<String, RoomData> {
        let json = include_str!("../../assets/data/rooms.json");
        let list: Vec<RoomData> = serde_json::from_str(json).expect("Failed to parse rooms.json");
        list.into_iter().map(|r| (r.id.clone(), r)).collect()
    }

    pub fn load_traps() -> HashMap<String, TrapData> {
        let json = include_str!("../../assets/data/traps.json");
        let list: Vec<TrapData> = serde_json::from_str(json).expect("Failed to parse traps.json");
        list.into_iter().map(|t| (t.id.clone(), t)).collect()
    }

    pub fn load_spells() -> HashMap<String, SpellData> {
        let json = include_str!("../../assets/data/dungeon_spells.json");
        let list: Vec<SpellData> = serde_json::from_str(json).expect("Failed to parse spells.json");
        list.into_iter().map(|s| (s.id.clone(), s)).collect()
    }

    pub fn load_config() -> GameConfig {
        let json = include_str!("../../assets/data/game_config.json");
        serde_json::from_str(json).expect("Failed to parse game_config.json")
    }
}

// ============================================================================
// Simple RNG (Linear Congruential Generator)
// ============================================================================

struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        // LCG parameters from Numerical Recipes
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    fn next_f32(&mut self) -> f32 {
        (self.next() as f32) / (u64::MAX as f32)
    }

    fn range(&mut self, min: f32, max: f32) -> f32 {
        min + self.next_f32() * (max - min)
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
        let (damage_min, damage_max) = m.combat.as_ref()
            .and_then(|c| c.damage_range)
            .map(|r| (r[0], r[1]))
            .unwrap_or((5.0, 10.0));
        let attack_speed = m.combat.as_ref()
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
        let (damage_min, damage_max) = h.combat.as_ref()
            .and_then(|c| c.damage_range)
            .map(|r| (r[0], r[1]))
            .unwrap_or((5.0, 10.0));
        let attack_speed = h.combat.as_ref()
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

    fn calculate_damage(&self, target: &CombatUnit, attack_mult: f32, defense_mult: f32) -> f32 {
        let base_damage = (self.damage_min + self.damage_max) / 2.0;
        let attack_damage = base_damage + (self.attack * attack_mult);
        let defense_reduction = target.defense * defense_mult;
        (attack_damage - defense_reduction).max(1.0)
    }

    fn dps(&self, target_defense: f32, attack_mult: f32, defense_mult: f32) -> f32 {
        let base_damage = (self.damage_min + self.damage_max) / 2.0;
        let attack_damage = base_damage + (self.attack * attack_mult);
        let defense_reduction = target_defense * defense_mult;
        let damage_per_hit = (attack_damage - defense_reduction).max(1.0);
        damage_per_hit * self.attack_speed
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
    let dt = 0.1; // simulation timestep

    while attacker.hp > 0.0 && defender.hp > 0.0 && time < 300.0 {
        time += dt;
        attacker_cooldown -= dt;
        defender_cooldown -= dt;

        if attacker_cooldown <= 0.0 {
            let damage = attacker.calculate_damage(&defender, attack_mult, defense_mult);
            defender.hp -= damage;
            attacker_cooldown = 1.0 / attacker.attack_speed;
        }

        if defender_cooldown <= 0.0 && defender.hp > 0.0 {
            let damage = defender.calculate_damage(&attacker, attack_mult, defense_mult);
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
// Headless Simulation Functions
// ============================================================================

fn simulate_combat_random(
    mut attacker: CombatUnit,
    mut defender: CombatUnit,
    attack_mult: f32,
    defense_mult: f32,
    rng: &mut SimpleRng,
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
            // Randomized damage within range
            let base_damage = rng.range(attacker.damage_min, attacker.damage_max);
            let attack_damage = base_damage + (attacker.attack * attack_mult);
            let defense_reduction = defender.defense * defense_mult;
            let damage = (attack_damage - defense_reduction).max(1.0);
            defender.hp -= damage;
            attacker_cooldown = 1.0 / attacker.attack_speed;
        }

        if defender_cooldown <= 0.0 && defender.hp > 0.0 {
            let base_damage = rng.range(defender.damage_min, defender.damage_max);
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

#[derive(Default)]
struct BattleStats {
    total_battles: u32,
    attacker_wins: u32,
    defender_wins: u32,
    avg_duration: f32,
    avg_winner_hp_pct: f32,
    min_duration: f32,
    max_duration: f32,
}

fn run_mass_battles(
    attacker_template: &CombatUnit,
    defender_template: &CombatUnit,
    num_battles: u32,
    attack_mult: f32,
    defense_mult: f32,
    seed: u64,
) -> BattleStats {
    let mut rng = SimpleRng::new(seed);
    let mut stats = BattleStats {
        total_battles: num_battles,
        min_duration: f32::MAX,
        max_duration: 0.0,
        ..Default::default()
    };

    let mut total_duration = 0.0;
    let mut total_hp_pct = 0.0;

    for _ in 0..num_battles {
        let attacker = attacker_template.clone();
        let defender = defender_template.clone();
        let result = simulate_combat_random(attacker, defender, attack_mult, defense_mult, &mut rng);

        total_duration += result.duration_secs;
        stats.min_duration = stats.min_duration.min(result.duration_secs);
        stats.max_duration = stats.max_duration.max(result.duration_secs);

        if result.winner == attacker_template.name {
            stats.attacker_wins += 1;
            total_hp_pct += result.winner_hp_remaining / attacker_template.max_hp;
        } else {
            stats.defender_wins += 1;
            total_hp_pct += result.winner_hp_remaining / defender_template.max_hp;
        }
    }

    stats.avg_duration = total_duration / num_battles as f32;
    stats.avg_winner_hp_pct = total_hp_pct / num_battles as f32 * 100.0;

    stats
}

struct ArmyBattleResult {
    winner: String, // "attacker" or "defender"
    survivors_attacker: u32,
    survivors_defender: u32,
    duration_secs: f32,
}

fn simulate_army_battle(
    mut attackers: Vec<CombatUnit>,
    mut defenders: Vec<CombatUnit>,
    attack_mult: f32,
    defense_mult: f32,
    rng: &mut SimpleRng,
) -> ArmyBattleResult {
    let mut time = 0.0;
    let dt = 0.1;
    let mut cooldowns_atk: Vec<f32> = vec![0.0; attackers.len()];
    let mut cooldowns_def: Vec<f32> = vec![0.0; defenders.len()];

    while !attackers.is_empty() && !defenders.is_empty() && time < 600.0 {
        time += dt;

        // Attackers attack
        for i in 0..attackers.len() {
            cooldowns_atk[i] -= dt;
            if cooldowns_atk[i] <= 0.0 && !defenders.is_empty() {
                // Pick a random defender
                let target_idx = (rng.next() as usize) % defenders.len();
                let base_damage = rng.range(attackers[i].damage_min, attackers[i].damage_max);
                let attack_damage = base_damage + (attackers[i].attack * attack_mult);
                let defense_reduction = defenders[target_idx].defense * defense_mult;
                let damage = (attack_damage - defense_reduction).max(1.0);
                defenders[target_idx].hp -= damage;
                cooldowns_atk[i] = 1.0 / attackers[i].attack_speed;
            }
        }

        // Defenders attack
        for i in 0..defenders.len() {
            cooldowns_def[i] -= dt;
            if cooldowns_def[i] <= 0.0 && !attackers.is_empty() {
                let target_idx = (rng.next() as usize) % attackers.len();
                let base_damage = rng.range(defenders[i].damage_min, defenders[i].damage_max);
                let attack_damage = base_damage + (defenders[i].attack * attack_mult);
                let defense_reduction = attackers[target_idx].defense * defense_mult;
                let damage = (attack_damage - defense_reduction).max(1.0);
                attackers[target_idx].hp -= damage;
                cooldowns_def[i] = 1.0 / defenders[i].attack_speed;
            }
        }

        // Remove dead units
        let mut i = 0;
        while i < defenders.len() {
            if defenders[i].hp <= 0.0 {
                defenders.remove(i);
                cooldowns_def.remove(i);
            } else {
                i += 1;
            }
        }

        let mut i = 0;
        while i < attackers.len() {
            if attackers[i].hp <= 0.0 {
                attackers.remove(i);
                cooldowns_atk.remove(i);
            } else {
                i += 1;
            }
        }
    }

    ArmyBattleResult {
        winner: if attackers.is_empty() { "defender".to_string() } else { "attacker".to_string() },
        survivors_attacker: attackers.len() as u32,
        survivors_defender: defenders.len() as u32,
        duration_secs: time,
    }
}

struct WaveSurvivalResult {
    waves_survived: u32,
    total_heroes_killed: u32,
    final_army_size: u32,
    final_gold: i32,
}

fn simulate_wave_survival(
    monsters: &HashMap<String, data::MonsterData>,
    heroes: &HashMap<String, data::HeroData>,
    config: &data::GameConfig,
    initial_army: Vec<(&str, u32)>, // (monster_id, count)
    num_waves: u32,
    seed: u64,
) -> WaveSurvivalResult {
    let mut rng = SimpleRng::new(seed);
    let attack_mult = config.combat.attack_stat_bonus;
    let defense_mult = config.combat.defense_reduction;

    // Build initial army
    let mut army: Vec<CombatUnit> = Vec::new();
    for (monster_id, count) in &initial_army {
        if let Some(monster) = monsters.get(*monster_id) {
            for _ in 0..*count {
                army.push(CombatUnit::from_monster(monster));
            }
        }
    }

    let mut gold = config.player_starting_resources.gold;
    let mut total_heroes_killed = 0u32;
    let mut waves_survived = 0u32;

    // Hero pool by tier
    let tier1: Vec<_> = heroes.values().filter(|h| h.tier.unwrap_or(1) == 1).collect();
    let tier2: Vec<_> = heroes.values().filter(|h| h.tier.unwrap_or(1) == 2).collect();
    let tier3: Vec<_> = heroes.values().filter(|h| h.tier.unwrap_or(1) == 3).collect();

    for wave in 1..=num_waves {
        if army.is_empty() {
            break;
        }

        // Generate wave heroes based on wave number
        let mut wave_heroes: Vec<CombatUnit> = Vec::new();
        let hero_count = 2 + (wave as f32 * config.hero_waves.wave_scaling_multiplier) as u32;

        for _ in 0..hero_count {
            // Higher waves get higher tier heroes
            let tier_roll = rng.next_f32();
            let hero_pool = if wave >= 8 && tier_roll < 0.3 && !tier3.is_empty() {
                &tier3
            } else if wave >= 4 && tier_roll < 0.5 && !tier2.is_empty() {
                &tier2
            } else if !tier1.is_empty() {
                &tier1
            } else {
                continue;
            };

            if !hero_pool.is_empty() {
                let idx = (rng.next() as usize) % hero_pool.len();
                wave_heroes.push(CombatUnit::from_hero(hero_pool[idx]));
            }
        }

        // Simulate battle
        let heroes_in_wave = wave_heroes.len() as u32;
        let result = simulate_army_battle(army.clone(), wave_heroes, attack_mult, defense_mult, &mut rng);

        if result.winner == "attacker" {
            waves_survived = wave;
            total_heroes_killed += heroes_in_wave;
            // Update army with survivors (simplified: just reduce count)
            army.truncate(result.survivors_attacker as usize);
            // Gold reward per hero killed
            gold += (heroes_in_wave * 10) as i32;
        } else {
            // Army wiped
            army.clear();
            total_heroes_killed += heroes_in_wave - result.survivors_defender;
        }

        // Wage costs (simplified: 1 minute per wave)
        for (monster_id, count) in &initial_army {
            if let Some(monster) = monsters.get(*monster_id) {
                let wage = monster.economy.as_ref().and_then(|e| e.wage_per_minute).unwrap_or(0.0);
                gold -= (wage * *count as f32) as i32;
            }
        }
    }

    WaveSurvivalResult {
        waves_survived,
        total_heroes_killed,
        final_army_size: army.len() as u32,
        final_gold: gold,
    }
}

fn run_simulation_mode(
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

            let stats = run_mass_battles(&attacker, &defender, num_battles, attack_mult, defense_mult, 12345);

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

        let result = simulate_army_battle(attackers, defenders, attack_mult, defense_mult, &mut rng);

        println!("\n{}", label);
        println!("  Winner: {} ({}v{} -> survivors: {} atk, {} def)",
            result.winner.to_uppercase(),
            atk_count, def_count,
            result.survivors_attacker, result.survivors_defender);
        println!("  Duration: {:.1}s", result.duration_secs);
    }
}

fn run_wave_mode(
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
        let result = simulate_wave_survival(monsters, heroes, config, army, 15, 99999);

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
        let result = simulate_wave_survival(monsters, heroes, config, test_army.clone(), 15, seed * 1000 + 42);
        total_waves += result.waves_survived;
        min_waves = min_waves.min(result.waves_survived);
        max_waves = max_waves.max(result.waves_survived);
        println!("  Run {}: {} waves survived, {} heroes killed",
            seed + 1, result.waves_survived, result.total_heroes_killed);
    }

    println!("\n  Average: {:.1} waves (range: {}-{})",
        total_waves as f32 / 10.0, min_waves, max_waves);
}

fn run_economy_mode(
    monsters: &HashMap<String, data::MonsterData>,
    rooms: &HashMap<String, data::RoomData>,
    config: &data::GameConfig,
) {
    print_header("ECONOMY SIMULATION OVER TIME");

    let mut gold = config.player_starting_resources.gold as f32;
    let mut mana = config.player_starting_resources.mana as f32;
    let mut food = config.player_starting_resources.food as f32;

    // Assume a standard room setup
    let hatchery_tiles = 9;
    let treasury_tiles = 4;

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

// ============================================================================
// Analysis Functions
// ============================================================================

fn print_header(title: &str) {
    println!("\n{}", "=".repeat(70));
    println!(" {}", title);
    println!("{}", "=".repeat(70));
}

fn print_subheader(title: &str) {
    println!("\n--- {} ---", title);
}

fn analyze_creature_efficiency(monsters: &HashMap<String, data::MonsterData>) {
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

fn analyze_combat_matchups(
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
                    let result = simulate_combat(creature_unit, hero_unit, attack_mult, defense_mult);

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
            let result = simulate_combat(creature_unit, hero_unit, attack_mult, defense_mult);

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

fn analyze_economy(
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

fn analyze_waves(
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

fn analyze_traps(traps: &HashMap<String, data::TrapData>, heroes: &HashMap<String, data::HeroData>) {
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

fn analyze_spells(spells: &HashMap<String, data::SpellData>) {
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

fn print_recommendations(
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

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("\n{}", "╔══════════════════════════════════════════════════════════════════════╗");
    println!("║           DUNGEON MANAGER - BALANCE CALCULATOR v1.0                   ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    // Load all game data
    println!("\nLoading game data...");
    let monsters = data::load_monsters();
    let heroes = data::load_heroes();
    let rooms = data::load_rooms();
    let traps = data::load_traps();
    let spells = data::load_spells();
    let config = data::load_config();

    println!("  Loaded {} creatures", monsters.len());
    println!("  Loaded {} heroes", heroes.len());
    println!("  Loaded {} rooms", rooms.len());
    println!("  Loaded {} traps", traps.len());
    println!("  Loaded {} spells", spells.len());

    // Run all analyses
    analyze_creature_efficiency(&monsters);
    analyze_combat_matchups(&monsters, &heroes, &config);
    analyze_economy(&monsters, &rooms, &config);
    analyze_waves(&heroes, &config);
    analyze_traps(&traps, &heroes);
    analyze_spells(&spells);
    print_recommendations(&monsters, &config);

    println!("\n{}", "=".repeat(70));
    println!(" Analysis Complete!");
    println!("{}", "=".repeat(70));
}
