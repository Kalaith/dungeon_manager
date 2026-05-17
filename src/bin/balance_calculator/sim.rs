use super::data;
use super::rng::SimpleRng;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CombatUnit {
    pub name: String,
    pub hp: f32,
    pub max_hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub damage_min: f32,
    pub damage_max: f32,
    pub attack_speed: f32,
}

impl CombatUnit {
    pub fn from_monster(m: &data::MonsterData) -> Self {
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

    pub fn from_hero(h: &data::HeroData) -> Self {
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

    pub fn calculate_damage(
        &self,
        target: &CombatUnit,
        attack_mult: f32,
        defense_mult: f32,
    ) -> f32 {
        let base_damage = (self.damage_min + self.damage_max) / 2.0;
        let attack_damage = base_damage + (self.attack * attack_mult);
        let defense_reduction = target.defense * defense_mult;
        (attack_damage - defense_reduction).max(1.0)
    }

    pub fn dps(&self, target_defense: f32, attack_mult: f32, defense_mult: f32) -> f32 {
        let base_damage = (self.damage_min + self.damage_max) / 2.0;
        let attack_damage = base_damage + (self.attack * attack_mult);
        let defense_reduction = target_defense * defense_mult;
        let damage_per_hit = (attack_damage - defense_reduction).max(1.0);
        damage_per_hit * self.attack_speed
    }
}

pub struct CombatResult {
    pub winner: String,
    pub duration_secs: f32,
    pub winner_hp_remaining: f32,
}

pub fn simulate_combat(
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

pub fn simulate_combat_random(
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
pub struct BattleStats {
    pub total_battles: u32,
    pub attacker_wins: u32,
    pub defender_wins: u32,
    pub avg_duration: f32,
    pub avg_winner_hp_pct: f32,
    pub min_duration: f32,
    pub max_duration: f32,
}

pub fn run_mass_battles(
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
        let result =
            simulate_combat_random(attacker, defender, attack_mult, defense_mult, &mut rng);

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

pub struct ArmyBattleResult {
    pub winner: String, // "attacker" or "defender"
    pub survivors_attacker: u32,
    pub survivors_defender: u32,
    pub duration_secs: f32,
}

pub fn simulate_army_battle(
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
        winner: if attackers.is_empty() {
            "defender".to_string()
        } else {
            "attacker".to_string()
        },
        survivors_attacker: attackers.len() as u32,
        survivors_defender: defenders.len() as u32,
        duration_secs: time,
    }
}

pub struct WaveSurvivalResult {
    pub waves_survived: u32,
    pub total_heroes_killed: u32,
    pub final_army_size: u32,
    pub final_gold: i32,
}

pub fn simulate_wave_survival(
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
    let tier1: Vec<_> = heroes
        .values()
        .filter(|h| h.tier.unwrap_or(1) == 1)
        .collect();
    let tier2: Vec<_> = heroes
        .values()
        .filter(|h| h.tier.unwrap_or(1) == 2)
        .collect();
    let tier3: Vec<_> = heroes
        .values()
        .filter(|h| h.tier.unwrap_or(1) == 3)
        .collect();

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
        let result = simulate_army_battle(
            army.clone(),
            wave_heroes,
            attack_mult,
            defense_mult,
            &mut rng,
        );

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
                let wage = monster
                    .economy
                    .as_ref()
                    .and_then(|e| e.wage_per_minute)
                    .unwrap_or(0.0);
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
