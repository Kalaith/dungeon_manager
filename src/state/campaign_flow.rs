use crate::data::campaign::{CampaignMission, CampaignProgress};
use crate::data::GameData;
use crate::state::game_state::GameState;

impl GameState {
    pub fn active_campaign_mission<'a>(
        &self,
        game_data: &'a GameData,
    ) -> Option<&'a CampaignMission> {
        let progress = self.campaign_progress.as_ref()?;
        let campaign = game_data.campaigns.get(&progress.campaign_id)?;
        progress.active_mission(campaign)
    }

    pub fn active_campaign_briefing<'a>(&self, game_data: &'a GameData) -> Option<&'a str> {
        self.active_campaign_mission(game_data)
            .map(|mission| mission.briefing.as_str())
            .filter(|briefing| !briefing.is_empty())
    }

    pub fn has_pending_campaign_mission(&self, game_data: &GameData) -> bool {
        let Some(progress) = &self.campaign_progress else {
            return false;
        };
        let Some(campaign) = game_data.campaigns.get(&progress.campaign_id) else {
            return false;
        };
        let Some(mission) = progress.active_mission(campaign) else {
            return false;
        };

        progress.unlocked_missions.contains(&mission.id)
            && !progress.completed_missions.contains(&mission.id)
            && mission
                .required_completed
                .iter()
                .all(|id| progress.completed_missions.contains(id))
    }

    pub fn new_for_campaign_progress(
        game_data: &GameData,
        progress: CampaignProgress,
    ) -> Option<Self> {
        let campaign = game_data.campaigns.get(&progress.campaign_id)?;
        let mission = progress.active_mission(campaign)?;

        let mut state = Self::new_for_scenario(game_data, &mission.scenario_id);
        state.apply_campaign_unlocks(&progress.persistent_unlocks);
        state.campaign_progress = Some(progress);
        Some(state)
    }

    pub fn start_pending_campaign_mission(&self, game_data: &GameData) -> Option<Self> {
        if !self.has_pending_campaign_mission(game_data) {
            return None;
        }

        let mut next =
            Self::new_for_campaign_progress(game_data, self.campaign_progress.as_ref()?.clone())?;
        // Keep the player's chosen difficulty across the whole campaign.
        next.difficulty = self.difficulty;
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::campaign::{CampaignDefinition, CampaignMission, CampaignUnlocks};
    use crate::data::scenario::ScenarioObjective;
    use crate::engine::objectives::update_victory_and_defeat;

    #[test]
    fn victory_can_start_next_campaign_mission_with_progress() {
        let mut game_data = GameData::load().expect("game data should load");
        add_test_scenario(
            &mut game_data,
            "campaign_m1",
            vec![ScenarioObjective::SurviveTime { seconds: 1.0 }],
        );
        add_test_scenario(
            &mut game_data,
            "campaign_m2",
            vec![ScenarioObjective::SurviveTime { seconds: 99.0 }],
        );
        let campaign = CampaignDefinition {
            id: "two_step".to_string(),
            name: "Two Step".to_string(),
            description: String::new(),
            starting_mission: "m1".to_string(),
            persistent_unlocks: CampaignUnlocks {
                rooms: vec!["workshop".to_string()],
                ..CampaignUnlocks::default()
            },
            missions: vec![
                CampaignMission {
                    id: "m1".to_string(),
                    scenario_id: "campaign_m1".to_string(),
                    name: "One".to_string(),
                    briefing: "First briefing".to_string(),
                    unlocks_after: vec!["m2".to_string()],
                    required_completed: Vec::new(),
                },
                CampaignMission {
                    id: "m2".to_string(),
                    scenario_id: "campaign_m2".to_string(),
                    name: "Two".to_string(),
                    briefing: "Second briefing".to_string(),
                    unlocks_after: Vec::new(),
                    required_completed: vec!["m1".to_string()],
                },
            ],
        };
        game_data.campaigns.insert(campaign.id.clone(), campaign);

        let mut state = GameState::new_campaign_start(&game_data, "two_step");
        state.time_elapsed = 1.1;
        update_victory_and_defeat(&mut state, &game_data);

        assert!(state.game_over);
        assert!(state.has_pending_campaign_mission(&game_data));

        let next_state = state
            .start_pending_campaign_mission(&game_data)
            .expect("next mission should start");
        let progress = next_state.campaign_progress.as_ref().unwrap();
        assert_eq!(
            next_state.active_scenario_id.as_deref(),
            Some("campaign_m2")
        );
        assert_eq!(progress.active_mission, "m2");
        assert!(progress.completed_missions.contains("m1"));
        assert!(next_state.player.unlocked_rooms.contains("workshop"));
        assert_eq!(
            next_state.active_campaign_briefing(&game_data),
            Some("Second briefing")
        );
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
