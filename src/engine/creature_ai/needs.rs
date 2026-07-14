use crate::data::monsters::MonsterData;
use crate::data::traits::TraitData;
use crate::data::GameData;
use crate::state::entities::CreatureState;

/// Look up the `TraitData` for each trait tag a creature was authored with, skipping unknown
/// ids. Missing an entry is a data-authoring gap, not an engine error — the trait just has no
/// effect until it's added to `traits.json`.
fn active_traits<'a>(monster_data: &MonsterData, game_data: &'a GameData) -> Vec<&'a TraitData> {
    monster_data
        .traits
        .iter()
        .filter_map(|trait_id| game_data.traits.get(trait_id))
        .collect()
}

pub fn update_needs(
    creature: &mut CreatureState,
    dt: f32,
    monster_data: &MonsterData,
    game_data: &GameData,
) {
    let decay_per_second = dt / 60.0;
    let traits = active_traits(monster_data, game_data);

    for (need_name, need_data) in &monster_data.needs {
        let current = creature.get_need(need_name);
        // product() of an empty iterator is 1.0, so needs with no matching trait are unaffected.
        let decay_multiplier: f32 = traits
            .iter()
            .filter_map(|t| t.need_decay_multipliers.get(need_name))
            .product();
        let decay = need_data.decay_per_minute * decay_per_second * decay_multiplier;
        creature.set_need(need_name.clone(), current - decay);
    }
}

pub fn calculate_mood(
    creature: &CreatureState,
    monster_data: &MonsterData,
    game_data: &GameData,
) -> f32 {
    let base_mood = monster_data.ai.base_mood;
    let mood_penalties = &game_data.config.creature_ai.mood_penalties;
    let mut mood = base_mood;

    let need_count = creature.needs.len() as f32;
    if need_count > 0.0 {
        let total_satisfaction: f32 = creature.needs.values().sum();
        let average_satisfaction = total_satisfaction / need_count;
        let need_modifier = (average_satisfaction - 50.0)
            * game_data.config.creature_ai.task_desirability.need_modifier;
        mood += need_modifier;
    }

    let health_percent = (creature.health / creature.max_health) * 100.0;
    if health_percent < mood_penalties.low_health_threshold {
        mood -= mood_penalties.low_health_penalty;
    }

    if creature.is_angry {
        mood -= mood_penalties.angry_penalty;
    }

    let trait_mood_modifier: f32 = active_traits(monster_data, game_data)
        .iter()
        .map(|t| t.mood_modifier)
        .sum();
    mood += trait_mood_modifier;

    mood.clamp(0.0, 100.0)
}

pub fn update_mood(creature: &mut CreatureState, monster_data: &MonsterData, game_data: &GameData) {
    creature.mood = calculate_mood(creature, monster_data, game_data);
    let traits = active_traits(monster_data, game_data);

    let anger_threshold = monster_data.ai.anger_threshold
        + traits
            .iter()
            .map(|t| t.anger_threshold_modifier)
            .sum::<f32>();
    creature.is_angry = creature.mood < anger_threshold;

    let desertion_threshold = monster_data.ai.desertion_threshold
        + traits
            .iter()
            .map(|t| t.desertion_threshold_modifier)
            .sum::<f32>();
    creature.is_deserting = creature.mood < desertion_threshold;
}

pub fn satisfy_need(creature: &mut CreatureState, need_name: &str, rate: f32, dt: f32) {
    let current = creature.get_need(need_name);
    let increase = rate * dt;
    creature.set_need(need_name.to_string(), current + increase);
}

pub fn apply_slap(
    creature: &mut CreatureState,
    monster_data: &MonsterData,
    game_data: &GameData,
    game_time: f32,
) {
    if game_time - creature.last_slapped < 5.0 {
        return;
    }

    creature.last_slapped = game_time;

    if let Some(&mood_change) = monster_data.ai.discipline_response.get("slap") {
        let discipline_multiplier: f32 = active_traits(monster_data, game_data)
            .iter()
            .map(|t| t.discipline_response_multiplier)
            .product();
        creature.mood = (creature.mood + mood_change * discipline_multiplier).clamp(0.0, 100.0);
    }

    creature.current_task = None;
    creature.task_time = 0.0;
}

pub fn calculate_work_efficiency(creature: &CreatureState, _monster_data: &MonsterData) -> f32 {
    let base_efficiency = 1.0;
    let mood_multiplier = 0.5 + (creature.mood / 100.0);

    let health_percent = creature.health / creature.max_health;
    let health_multiplier = if health_percent < 0.5 {
        0.5 + health_percent
    } else {
        1.0
    };

    base_efficiency * mood_multiplier * health_multiplier
}
