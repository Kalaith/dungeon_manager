use crate::data::scenario::ScenarioObjective;
use crate::data::GameData;
use crate::state::entities::EntityType;
use crate::state::faction::OwnerId;
use crate::state::game_state::GameState;

pub fn update_victory_and_defeat(state: &mut GameState, game_data: &GameData) {
    if state.game_over {
        return;
    }

    if state.dungeon_heart_health <= 0.0 {
        set_defeat(state, "DEFEAT! Your Dungeon Heart was destroyed!");
        return;
    }

    if update_scenario_objectives(state, game_data) {
        set_victory(state, "VICTORY! Scenario objectives completed!");
        complete_active_campaign_mission(state, game_data);
        return;
    }

    if state.hero_base.enabled && state.hero_base.is_defeated(&state.entities) {
        set_victory(state, "VICTORY! All hero buildings have been destroyed!");
        complete_active_campaign_mission(state, game_data);
    }
}

fn update_scenario_objectives(state: &mut GameState, game_data: &GameData) -> bool {
    let Some(scenario_id) = &state.active_scenario_id else {
        return false;
    };
    let Some(scenario) = game_data.scenarios.get(scenario_id) else {
        return false;
    };
    if scenario.objectives.is_empty() {
        return false;
    }

    let completed_ids: Vec<String> = scenario
        .objectives
        .iter()
        .filter(|objective| objective_is_complete(objective, state, game_data))
        .map(ScenarioObjective::objective_id)
        .collect();

    if let Some(runtime) = &mut state.scenario_runtime {
        for id in completed_ids {
            runtime.mark_objective_complete(&id);
        }

        scenario.objectives.iter().all(|objective| {
            runtime
                .completed_objectives
                .contains(&objective.objective_id())
        })
    } else {
        false
    }
}

fn objective_is_complete(
    objective: &ScenarioObjective,
    state: &GameState,
    game_data: &GameData,
) -> bool {
    match objective {
        ScenarioObjective::DestroyHeart { owner } => owner_heart_destroyed(owner, state),
        ScenarioObjective::SurviveTime { seconds } => state.time_elapsed >= *seconds,
        ScenarioObjective::DestroyAllHeroBuildings => hero_buildings_destroyed(state, game_data),
        ScenarioObjective::GatherResource { resource, amount } => {
            current_resource_amount(state, resource) >= *amount
        }
        ScenarioObjective::Custom { id, .. } => state
            .scenario_runtime
            .as_ref()
            .map(|runtime| runtime.completed_objectives.contains(id))
            .unwrap_or(false),
    }
}

fn owner_heart_destroyed(owner: &OwnerId, state: &GameState) -> bool {
    if owner.is_player_controlled() {
        return state.dungeon_heart_health <= 0.0;
    }

    let matching_hearts = state.entities.all().filter(|entity| {
        entity.owner == *owner
            && matches!(
                &entity.entity_type,
                EntityType::Structure(structure) if structure.building_id == "dungeon_heart"
            )
    });

    let mut saw_heart = false;
    for entity in matching_hearts {
        saw_heart = true;
        if entity.is_alive() {
            return false;
        }
    }

    saw_heart
}

fn hero_buildings_destroyed(state: &GameState, game_data: &GameData) -> bool {
    if state.hero_base.enabled {
        return state.hero_base.is_defeated(&state.entities);
    }

    let live_hero_buildings = state.entities.all().any(|entity| {
        entity.owner == OwnerId::Heroes
            && matches!(
                &entity.entity_type,
                EntityType::Structure(structure)
                    if game_data.hero_buildings.contains_key(&structure.building_id)
                        && structure.health > 0.0
            )
    });

    !live_hero_buildings && has_authored_hero_building_tiles(state, game_data)
}

fn has_authored_hero_building_tiles(state: &GameState, game_data: &GameData) -> bool {
    state.dungeon.grid.iter().flatten().any(|tile| {
        game_data.hero_buildings.contains_key(&tile.tile_type) && tile.owner == OwnerId::Heroes
    })
}

fn current_resource_amount(state: &GameState, resource: &str) -> i32 {
    match resource {
        "gold" => state.player.gold,
        "mana" => state.player.mana,
        "food" => state.player.food,
        "materials" => state.player.materials,
        _ => 0,
    }
}

fn set_victory(state: &mut GameState, message: &str) {
    state.game_over = true;
    state.victory = true;
    state.notifications.success(message);
}

fn set_defeat(state: &mut GameState, message: &str) {
    state.game_over = true;
    state.victory = false;
    state.notifications.danger(message);
}

fn complete_active_campaign_mission(state: &mut GameState, game_data: &GameData) {
    let Some(progress) = &mut state.campaign_progress else {
        return;
    };
    let Some(campaign) = game_data.campaigns.get(&progress.campaign_id) else {
        return;
    };

    let mission_id = progress.active_mission.clone();
    progress.complete_mission(campaign, &mission_id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::campaign::{
        CampaignDefinition, CampaignMission, CampaignProgress, CampaignUnlocks,
    };
    use crate::data::scenario::ScenarioObjective;

    #[test]
    fn timed_scenario_objective_drives_victory() {
        let mut game_data = GameData::load().expect("game data should load");
        add_test_scenario(
            &mut game_data,
            "timed_win",
            vec![ScenarioObjective::SurviveTime { seconds: 10.0 }],
        );
        let mut state = GameState::new_for_scenario(&game_data, "timed_win");

        state.time_elapsed = 10.1;
        update_victory_and_defeat(&mut state, &game_data);

        assert!(state.game_over);
        assert!(state.victory);
        assert!(state
            .scenario_runtime
            .as_ref()
            .unwrap()
            .completed_objectives
            .contains("survive_time"));
    }

    #[test]
    fn scenario_victory_completes_campaign_mission() {
        let mut game_data = GameData::load().expect("game data should load");
        add_test_scenario(
            &mut game_data,
            "campaign_win",
            vec![ScenarioObjective::SurviveTime { seconds: 1.0 }],
        );
        let campaign = CampaignDefinition {
            id: "test_campaign".to_string(),
            name: "Test Campaign".to_string(),
            description: String::new(),
            starting_mission: "m1".to_string(),
            persistent_unlocks: CampaignUnlocks::default(),
            missions: vec![
                CampaignMission {
                    id: "m1".to_string(),
                    scenario_id: "campaign_win".to_string(),
                    name: "One".to_string(),
                    briefing: "Briefing".to_string(),
                    unlocks_after: vec!["m2".to_string()],
                    required_completed: Vec::new(),
                },
                CampaignMission {
                    id: "m2".to_string(),
                    scenario_id: "campaign_win".to_string(),
                    name: "Two".to_string(),
                    briefing: String::new(),
                    unlocks_after: Vec::new(),
                    required_completed: vec!["m1".to_string()],
                },
            ],
        };
        game_data
            .campaigns
            .insert(campaign.id.clone(), campaign.clone());

        let mut state = GameState::new_for_scenario(&game_data, "campaign_win");
        state.campaign_progress = Some(CampaignProgress::new(&campaign));
        state.time_elapsed = 1.1;

        update_victory_and_defeat(&mut state, &game_data);

        let progress = state.campaign_progress.as_ref().unwrap();
        assert!(progress.completed_missions.contains("m1"));
        assert_eq!(progress.active_mission, "m2");
    }

    #[test]
    fn campaign_start_applies_persistent_content_unlocks() {
        let mut game_data = GameData::load().expect("game data should load");
        add_test_scenario(
            &mut game_data,
            "unlock_campaign",
            vec![ScenarioObjective::SurviveTime { seconds: 1.0 }],
        );
        let campaign = CampaignDefinition {
            id: "unlock_campaign".to_string(),
            name: "Unlock Campaign".to_string(),
            description: String::new(),
            starting_mission: "m1".to_string(),
            persistent_unlocks: CampaignUnlocks {
                rooms: vec!["workshop".to_string()],
                spells: vec!["lightning_strike".to_string()],
                traps: vec!["spike_trap".to_string()],
                creatures: vec!["orc".to_string()],
            },
            missions: vec![CampaignMission {
                id: "m1".to_string(),
                scenario_id: "unlock_campaign".to_string(),
                name: "One".to_string(),
                briefing: String::new(),
                unlocks_after: Vec::new(),
                required_completed: Vec::new(),
            }],
        };
        game_data
            .campaigns
            .insert(campaign.id.clone(), campaign.clone());

        let state = GameState::new_campaign_start(&game_data, "unlock_campaign");

        assert!(state.player.unlocked_rooms.contains("workshop"));
        assert!(state.player.unlocked_spells.contains("lightning_strike"));
        assert!(state.player.unlocked_traps.contains("spike_trap"));
        assert!(state.player.unlocked_creatures.contains("orc"));
    }

    #[test]
    fn dungeon_heart_state_health_drives_defeat() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_for_scenario(&game_data, "dark_beginnings");
        state.dungeon_heart_health = 0.0;

        update_victory_and_defeat(&mut state, &game_data);

        assert!(state.game_over);
        assert!(!state.victory);
    }

    fn add_test_scenario(
        game_data: &mut GameData,
        scenario_id: &str,
        objectives: Vec<ScenarioObjective>,
    ) {
        let mut scenario = game_data
            .scenarios
            .get("dark_beginnings")
            .expect("base scenario should exist")
            .clone();
        scenario.meta.id = scenario_id.to_string();
        scenario.objectives = objectives;
        game_data
            .scenarios
            .insert(scenario_id.to_string(), scenario);
    }
}
