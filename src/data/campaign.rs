use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignDefinition {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub starting_mission: String,
    #[serde(default)]
    pub missions: Vec<CampaignMission>,
    #[serde(default)]
    pub persistent_unlocks: CampaignUnlocks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignMission {
    pub id: String,
    pub scenario_id: String,
    pub name: String,
    #[serde(default)]
    pub briefing: String,
    #[serde(default)]
    pub unlocks_after: Vec<String>,
    #[serde(default)]
    pub required_completed: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CampaignUnlocks {
    #[serde(default)]
    pub rooms: Vec<String>,
    #[serde(default)]
    pub spells: Vec<String>,
    #[serde(default)]
    pub traps: Vec<String>,
    #[serde(default)]
    pub creatures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignProgress {
    pub campaign_id: String,
    pub active_mission: String,
    pub completed_missions: HashSet<String>,
    pub unlocked_missions: HashSet<String>,
    pub persistent_unlocks: CampaignUnlocks,
}

impl CampaignProgress {
    pub fn new(campaign: &CampaignDefinition) -> Self {
        let active_mission = if campaign.starting_mission.is_empty() {
            campaign
                .missions
                .first()
                .map(|mission| mission.id.clone())
                .unwrap_or_default()
        } else {
            campaign.starting_mission.clone()
        };

        let mut unlocked_missions = HashSet::new();
        if !active_mission.is_empty() {
            unlocked_missions.insert(active_mission.clone());
        }

        Self {
            campaign_id: campaign.id.clone(),
            active_mission,
            completed_missions: HashSet::new(),
            unlocked_missions,
            persistent_unlocks: campaign.persistent_unlocks.clone(),
        }
    }

    pub fn complete_mission(&mut self, campaign: &CampaignDefinition, mission_id: &str) {
        self.completed_missions.insert(mission_id.to_string());

        if let Some(mission) = campaign.missions.iter().find(|m| m.id == mission_id) {
            for unlock in &mission.unlocks_after {
                self.unlocked_missions.insert(unlock.clone());
            }
        }

        if let Some(next) = campaign
            .missions
            .iter()
            .find(|mission| {
                self.unlocked_missions.contains(&mission.id)
                    && !self.completed_missions.contains(&mission.id)
                    && mission
                        .required_completed
                        .iter()
                        .all(|id| self.completed_missions.contains(id))
            })
            .map(|mission| mission.id.clone())
        {
            self.active_mission = next;
        }
    }

    pub fn active_mission<'a>(
        &self,
        campaign: &'a CampaignDefinition,
    ) -> Option<&'a CampaignMission> {
        campaign
            .missions
            .iter()
            .find(|mission| mission.id == self.active_mission)
    }

    pub fn unlocked_missions<'a>(
        &self,
        campaign: &'a CampaignDefinition,
    ) -> Vec<&'a CampaignMission> {
        campaign
            .missions
            .iter()
            .filter(|mission| self.unlocked_missions.contains(&mission.id))
            .collect()
    }
}

pub fn load_campaigns() -> Result<HashMap<String, CampaignDefinition>, Box<dyn Error>> {
    let json_content = include_str!("../../assets/campaigns/deep_dominion.json");
    let campaigns_vec: Vec<CampaignDefinition> = serde_json::from_str(json_content)?;
    Ok(campaigns_vec
        .into_iter()
        .map(|campaign| (campaign.id.clone(), campaign))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_campaign_parses() {
        let campaigns = load_campaigns().expect("campaign json should parse");
        let campaign = campaigns
            .get("deep_dominion")
            .expect("default campaign missing");
        assert_eq!(campaign.starting_mission, "dark_beginnings");
        assert!(!campaign.missions.is_empty());
    }

    #[test]
    fn progress_unlocks_next_mission() {
        let campaign: CampaignDefinition = serde_json::from_str(
            r#"{
              "id": "test",
              "name": "Test",
              "starting_mission": "m1",
              "missions": [
                { "id": "m1", "scenario_id": "s1", "name": "One", "unlocks_after": ["m2"] },
                { "id": "m2", "scenario_id": "s2", "name": "Two", "required_completed": ["m1"] }
              ]
            }"#,
        )
        .unwrap();

        let mut progress = CampaignProgress::new(&campaign);
        progress.complete_mission(&campaign, "m1");

        assert!(progress.completed_missions.contains("m1"));
        assert_eq!(progress.active_mission, "m2");
    }

    #[test]
    fn progress_exposes_active_briefing_and_unlocked_missions() {
        let campaign: CampaignDefinition = serde_json::from_str(
            r#"{
              "id": "test",
              "name": "Test",
              "starting_mission": "m1",
              "missions": [
                { "id": "m1", "scenario_id": "s1", "name": "One", "briefing": "First briefing" },
                { "id": "m2", "scenario_id": "s2", "name": "Two" }
              ]
            }"#,
        )
        .unwrap();

        let progress = CampaignProgress::new(&campaign);

        assert_eq!(
            progress.active_mission(&campaign).unwrap().briefing,
            "First briefing"
        );
        assert_eq!(progress.unlocked_missions(&campaign).len(), 1);
    }
}
