use crate::data::scenario::{EventAction, EventTrigger, ScenarioDefinition, ScenarioEvent};
use crate::data::GameData;
use crate::state::entities::CreatureState;
use crate::state::game_state::GameState;
use crate::state::tile_state::FogState;
use crate::state::tile_state::TilePos;

pub fn update_scenario_events(state: &mut GameState, game_data: &GameData) {
    let Some(scenario_id) = state.active_scenario_id.clone() else {
        return;
    };
    let Some(scenario) = game_data.scenarios.get(&scenario_id) else {
        return;
    };

    update_action_points(state, scenario);

    let events_to_fire: Vec<ScenarioEvent> = scenario
        .events
        .iter()
        .filter(|event| {
            let already_fired = state
                .scenario_runtime
                .as_ref()
                .map(|runtime| runtime.fired_events.contains(&event.id))
                .unwrap_or(false);
            (!event.once || !already_fired) && trigger_matches(event, state)
        })
        .cloned()
        .collect();

    for event in events_to_fire {
        for action in &event.actions {
            apply_action(state, game_data, action);
        }
        if let Some(runtime) = &mut state.scenario_runtime {
            runtime.mark_event_fired(&event.id);
        }
    }
}

fn trigger_matches(event: &ScenarioEvent, state: &GameState) -> bool {
    match &event.trigger {
        EventTrigger::TimeElapsed { seconds } => state.time_elapsed >= *seconds,
        EventTrigger::ObjectiveComplete { objective } => state
            .scenario_runtime
            .as_ref()
            .map(|runtime| runtime.completed_objectives.contains(objective))
            .unwrap_or(false),
        EventTrigger::RoomClaimed { room, owner } => {
            owner.is_player_controlled()
                && state
                    .room_manager
                    .rooms
                    .iter()
                    .any(|active_room| active_room.room_type == *room)
        }
        EventTrigger::ActionPointReached { id, owner } => state
            .scenario_runtime
            .as_ref()
            .map(|runtime| runtime.action_point_reached(id, owner))
            .unwrap_or(false),
        EventTrigger::DungeonBreached { owner } => state.entities.heroes().any(|(id, _)| {
            let Some(entity) = state.entities.get(id) else {
                return false;
            };
            let Some(tile) = state.dungeon.get_tile(entity.pos) else {
                return false;
            };
            tile.owner == *owner
        }),
    }
}

fn apply_action(state: &mut GameState, game_data: &GameData, action: &EventAction) {
    match action {
        EventAction::UnlockRoom { room, owner } => {
            if owner.is_player_controlled() {
                state.player.unlock_room(room.clone());
            }
            if let Some(runtime) = &mut state.scenario_runtime {
                runtime.unlock_room(room);
            }
        }
        EventAction::UnlockSpell { spell, owner } => {
            if owner.is_player_controlled() {
                state.player.unlock_spell(spell.clone());
            }
            if let Some(runtime) = &mut state.scenario_runtime {
                runtime.unlock_spell(spell);
            }
        }
        EventAction::UnlockTrap { trap, owner } => {
            if owner.is_player_controlled() {
                state.player.unlock_trap(trap.clone());
            }
            if let Some(runtime) = &mut state.scenario_runtime {
                runtime.unlock_trap(trap);
            }
        }
        EventAction::SpawnCreature {
            creature,
            owner,
            x,
            y,
            level,
        } => {
            let Some(monster_data) = game_data.monsters.get(creature) else {
                return;
            };
            let visual_seed = macroquad_toolkit::rng::random_u64();
            let creature_state = CreatureState::new(
                creature.clone(),
                *level,
                monster_data.stats.health,
                monster_data.stats.mana,
                visual_seed,
            );
            state.entities.spawn_creature_for_owner(
                TilePos::new(*x, *y),
                creature_state,
                owner.clone(),
            );
        }
        EventAction::SpawnHeroParty { party, x, y } => {
            let Some(scenario_id) = &state.active_scenario_id else {
                return;
            };
            let Some(scenario) = game_data.scenarios.get(scenario_id) else {
                return;
            };
            let Some(party) = scenario.hero_parties.iter().find(|p| p.id == *party) else {
                return;
            };
            crate::engine::hero_parties::spawn_hero_party(
                state,
                party,
                TilePos::new(*x, *y),
                game_data,
            );
        }
        EventAction::SetRule { key, value } => apply_rule(state, game_data, key, value),
        EventAction::CompleteObjective { objective } => {
            if let Some(runtime) = &mut state.scenario_runtime {
                runtime.mark_objective_complete(objective);
            }
        }
    }
}

fn update_action_points(state: &mut GameState, scenario: &ScenarioDefinition) {
    if scenario.action_points.is_empty() {
        return;
    }

    let mut reached = Vec::new();
    for point in &scenario.action_points {
        let center = TilePos::new(point.x, point.y);
        for entity in state.entities.all() {
            if let Some(owner) = &point.owner {
                if &entity.owner != owner {
                    continue;
                }
            }
            if entity.pos.manhattan_distance(&center) <= point.radius.max(0) {
                reached.push((point.id.clone(), entity.owner.clone()));
            }
        }
    }

    if let Some(runtime) = &mut state.scenario_runtime {
        for (point_id, owner) in reached {
            runtime.mark_action_point_reached(&point_id, &owner);
        }
    }
}

fn apply_rule(state: &mut GameState, game_data: &GameData, key: &str, value: &serde_json::Value) {
    let Some(runtime) = &mut state.scenario_runtime else {
        return;
    };
    if !runtime.set_rule(key, value) {
        return;
    }

    match key {
        "fog_of_war" => {
            if value.as_bool() == Some(false) {
                state.cheat_fog_enabled = false;
                reveal_all_tiles(state);
            } else if value.as_bool() == Some(true) {
                state.cheat_fog_enabled = true;
            }
        }
        "starting_research" => {
            if let Some(research_ids) = value.as_array() {
                for tech_id in research_ids.iter().filter_map(|id| id.as_str()) {
                    if let Some(tech) = game_data.technologies.get(tech_id) {
                        state.player.complete_research(tech);
                    }
                }
            }
        }
        _ => {}
    }
}

fn reveal_all_tiles(state: &mut GameState) {
    for row in &mut state.dungeon.grid {
        for tile in row {
            tile.fog_state = FogState::Visible;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::scenario::{
        ActionPointDefinition, ScenarioEvent, ScenarioObjective, ScenarioRules,
    };
    use crate::state::OwnerId;

    #[test]
    fn timed_event_unlocks_trap_and_spawns_party_once() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_for_scenario(&game_data, "dark_beginnings");
        // first_raid now triggers at 300s to give new players a setup grace period
        state.time_elapsed = 301.0;

        update_scenario_events(&mut state, &game_data);
        let hero_count = state.entities.heroes().count();
        update_scenario_events(&mut state, &game_data);

        assert_eq!(hero_count, state.entities.heroes().count());
        let runtime = state.scenario_runtime.as_ref().unwrap();
        assert!(runtime.fired_events.contains("first_raid"));
        assert!(runtime.unlocked_traps.contains("spike_trap"));
        assert!(state.player.unlocked_traps.contains("spike_trap"));
    }

    #[test]
    fn action_point_event_fires_for_matching_owner() {
        let mut game_data = GameData::load().expect("game data should load");
        let mut scenario = game_data
            .scenarios
            .get("dark_beginnings")
            .expect("scenario should exist")
            .clone();
        scenario.meta.id = "action_point_test".to_string();
        scenario.action_points = vec![ActionPointDefinition {
            id: "frontier".to_string(),
            x: 12,
            y: 10,
            radius: 0,
            owner: Some(OwnerId::Player),
        }];
        scenario.events = vec![ScenarioEvent {
            id: "frontier_reached".to_string(),
            trigger: EventTrigger::ActionPointReached {
                id: "frontier".to_string(),
                owner: OwnerId::Player,
            },
            actions: vec![EventAction::CompleteObjective {
                objective: "frontier_done".to_string(),
            }],
            once: true,
        }];
        scenario.objectives = vec![ScenarioObjective::Custom {
            id: "frontier_done".to_string(),
            description: "Reach the frontier.".to_string(),
        }];
        game_data
            .scenarios
            .insert(scenario.meta.id.clone(), scenario);

        let mut state = GameState::new_for_scenario(&game_data, "action_point_test");
        update_scenario_events(&mut state, &game_data);

        let runtime = state.scenario_runtime.as_ref().unwrap();
        assert!(runtime.action_point_reached("frontier", &OwnerId::Player));
        assert!(runtime.fired_events.contains("frontier_reached"));
        assert!(runtime.completed_objectives.contains("frontier_done"));
    }

    #[test]
    fn set_rule_event_updates_runtime_and_applies_side_effects() {
        let mut game_data = GameData::load().expect("game data should load");
        let mut scenario = game_data
            .scenarios
            .get("dark_beginnings")
            .expect("scenario should exist")
            .clone();
        scenario.meta.id = "rule_event_test".to_string();
        scenario.rules = ScenarioRules::default();
        scenario.events = vec![ScenarioEvent {
            id: "disable_fog".to_string(),
            trigger: EventTrigger::TimeElapsed { seconds: 1.0 },
            actions: vec![EventAction::SetRule {
                key: "fog_of_war".to_string(),
                value: serde_json::json!(false),
            }],
            once: true,
        }];
        game_data
            .scenarios
            .insert(scenario.meta.id.clone(), scenario);

        let mut state = GameState::new_for_scenario(&game_data, "rule_event_test");
        state.time_elapsed = 1.0;
        update_scenario_events(&mut state, &game_data);

        let runtime = state.scenario_runtime.as_ref().unwrap();
        assert!(!runtime.active_rules.fog_of_war);
        assert!(!state.cheat_fog_enabled);
        assert!(state
            .dungeon
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .all(|tile| tile.fog_state == FogState::Visible));
    }
}
