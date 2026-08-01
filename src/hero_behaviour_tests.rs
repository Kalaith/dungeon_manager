//! Tests for the hero personality model.
//!
//! `heroes.json` authors a `behavior` block and a `bravery` stat for all
//! twenty heroes — a scout is trap-aware at 0.9, a militiaman at 0.3; a
//! champion is brave at 95, a peasant at 30 — and hero AI read none of it.
//! Every hero broke at exactly the same health fraction and walked into every
//! trap with identical carelessness.

use crate::data::GameData;
use crate::engine::hero_ai::effective_retreat_threshold;

#[test]
fn nerve_lowers_the_retreat_threshold_below_the_authored_baseline() {
    let game_data = GameData::load().expect("game data should load");

    for (id, hero) in &game_data.heroes {
        if hero.behavior.will_fight_to_death {
            continue;
        }
        let baseline = hero.ai.threat_response.retreat_below_health;
        let effective = effective_retreat_threshold(hero);

        assert!(
            effective <= baseline + f32::EPSILON,
            "`{id}` should not break *sooner* than authored: {effective} vs {baseline}"
        );
        // Capped at a 60% reduction, so nobody becomes unbreakable by accident.
        assert!(
            effective >= baseline * 0.4 - 1e-4,
            "`{id}` reduced too far: {effective} vs {baseline}"
        );
    }
}

#[test]
fn a_braver_hero_fights_on_longer_than_a_timid_one() {
    let game_data = GameData::load().expect("game data should load");

    // Compare the *proportion* of the authored baseline each hero keeps, so
    // heroes with different baselines are still comparable. Restricted to
    // heroes who can retreat at all and whose baseline is non-zero — the
    // champion of light is authored `will_fight_to_death` with a 0.0
    // threshold, which is a separate case and its own test.
    let comparable: Vec<_> = game_data
        .heroes
        .iter()
        .filter(|(_, hero)| {
            !hero.behavior.will_fight_to_death && hero.ai.threat_response.retreat_below_health > 0.0
        })
        .collect();
    assert!(
        comparable.len() >= 2,
        "need at least two retreating heroes to compare"
    );

    let nerve =
        |hero: &crate::data::HeroData| hero.stats.bravery / 100.0 + hero.behavior.fear_resistance;
    let kept = |hero: &crate::data::HeroData| {
        effective_retreat_threshold(hero) / hero.ai.threat_response.retreat_below_health
    };

    let (timid_id, timid) = comparable
        .iter()
        .min_by(|a, b| nerve(a.1).total_cmp(&nerve(b.1)))
        .expect("a most timid hero");
    let (stout_id, stout) = comparable
        .iter()
        .max_by(|a, b| nerve(a.1).total_cmp(&nerve(b.1)))
        .expect("a stoutest hero");

    assert!(
        kept(stout) < kept(timid),
        "`{stout_id}` (nerve {:.2}) should hold out proportionally longer than `{timid_id}` \
         (nerve {:.2}), but kept {:.3} vs {:.3}",
        nerve(stout),
        nerve(timid),
        kept(stout),
        kept(timid),
    );
}

#[test]
fn a_hero_who_fights_to_the_death_never_voluntarily_retreats() {
    let game_data = GameData::load().expect("game data should load");

    let mut checked = 0;
    for (id, hero) in &game_data.heroes {
        if !hero.behavior.will_fight_to_death {
            continue;
        }
        assert_eq!(
            effective_retreat_threshold(hero),
            0.0,
            "`{id}` is authored to fight to the death"
        );
        checked += 1;
    }
    assert!(
        checked > 0,
        "no hero is authored `will_fight_to_death`; this test proves nothing"
    );
}

#[test]
fn trap_awareness_is_authored_across_a_real_range() {
    // The wiring is a probability roll, so what is worth pinning is that the
    // data actually differentiates heroes — otherwise the roll is noise.
    let game_data = GameData::load().expect("game data should load");

    let mut lowest = f32::MAX;
    let mut highest = f32::MIN;
    for hero in game_data.heroes.values() {
        lowest = lowest.min(hero.behavior.trap_awareness);
        highest = highest.max(hero.behavior.trap_awareness);
    }

    assert!(
        lowest >= 0.0 && highest <= 1.0,
        "awareness must be a probability"
    );
    assert!(
        highest - lowest > 0.3,
        "heroes should differ meaningfully in trap awareness, saw {lowest}..{highest}"
    );
    assert!(
        game_data.heroes["scout"].behavior.trap_awareness
            > game_data.heroes["peasant_militia"].behavior.trap_awareness,
        "a scout should out-spot a militiaman"
    );
}

// ---------------------------------------------------------------------------
// Whether a hero on the attack ever reconsiders
// ---------------------------------------------------------------------------

use crate::engine::hero_ai::should_reconsider_goal;
use crate::state::entities::{HeroGoal, HeroState};
use crate::state::tile_state::TilePos;

fn attacker(game_data: &GameData, hero_id: &str, health_fraction: f32) -> HeroState {
    let data = &game_data.heroes[hero_id];
    let mut hero = HeroState::new(
        hero_id.to_string(),
        1,
        data.stats.health,
        data.stats.mana,
        TilePos::new(1, 1),
        1.0,
        0,
    );
    hero.health = hero.max_health * health_fraction;
    hero.current_goal = HeroGoal::DestroyHeart;
    hero
}

#[test]
fn a_healthy_attacker_presses_on() {
    let game_data = GameData::load().expect("game data should load");
    let hero = attacker(&game_data, "acolyte", 1.0);

    assert!(
        !should_reconsider_goal(&hero, &game_data),
        "an unhurt attacker should not stop to think about it"
    );
}

#[test]
fn a_broken_attacker_reconsiders() {
    // The bug this covers: `DestroyHeart` never re-evaluated, so an attacker
    // could not retreat no matter how badly hurt. A live wave traced zero goal
    // transitions in 2500 frames.
    let game_data = GameData::load().expect("game data should load");
    let threshold =
        crate::engine::hero_ai::effective_retreat_threshold(&game_data.heroes["acolyte"]);
    assert!(
        threshold > 0.0,
        "the acolyte should be able to break at all"
    );

    let hero = attacker(&game_data, "acolyte", threshold * 0.5);

    assert!(
        should_reconsider_goal(&hero, &game_data),
        "an attacker below their breaking point should reconsider"
    );
}

#[test]
fn an_attacker_who_fights_to_the_death_never_reconsiders() {
    let game_data = GameData::load().expect("game data should load");

    let mut checked = 0;
    for (id, data) in &game_data.heroes {
        if !data.behavior.will_fight_to_death {
            continue;
        }
        // A sliver of health left, and still no reconsidering.
        let hero = attacker(&game_data, id, 0.01);
        assert!(
            !should_reconsider_goal(&hero, &game_data),
            "`{id}` is authored to fight to the death"
        );
        checked += 1;
    }
    assert!(checked >= 5, "expected several such heroes, saw {checked}");
}

#[test]
fn a_resting_hero_waits_for_the_wave() {
    let game_data = GameData::load().expect("game data should load");
    let mut hero = attacker(&game_data, "acolyte", 0.05);
    hero.current_goal = HeroGoal::RestAtSpawn(TilePos::new(1, 1));

    assert!(
        !should_reconsider_goal(&hero, &game_data),
        "a resting hero should stay put until the wave launches"
    );
}
