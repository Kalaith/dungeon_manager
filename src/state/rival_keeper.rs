use crate::data::scenario::ScenarioDefinition;
use crate::state::OwnerId;
use serde::{Deserialize, Serialize};

pub const DEFAULT_MIN_GARRISON: usize = 3;
pub const DEFAULT_RAID_SIZE: usize = 3;
pub const DEFAULT_DIG_BATCH: usize = 4;
pub const DEFAULT_ROOM_SIZE: usize = 3;
pub const DEFAULT_ATTACK_COOLDOWN: f32 = 45.0;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RivalKeeperRuntime {
    #[serde(default)]
    pub keepers: Vec<RivalKeeperAiState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RivalKeeperAiState {
    pub owner: OwnerId,
    pub ai_profile: String,
    #[serde(default)]
    pub preferred_creatures: Vec<String>,
    #[serde(default)]
    pub desired_rooms: Vec<String>,
    #[serde(default)]
    pub next_decision_time: f32,
    #[serde(default = "default_min_garrison")]
    pub min_garrison: usize,
    #[serde(default = "default_raid_size")]
    pub raid_size: usize,
    #[serde(default = "default_dig_batch")]
    pub dig_batch: usize,
    #[serde(default = "default_room_size")]
    pub room_size: usize,
    #[serde(default = "default_attack_cooldown")]
    pub attack_cooldown: f32,
    #[serde(default)]
    pub next_attack_time: f32,
}

impl RivalKeeperRuntime {
    pub fn from_scenario(scenario: &ScenarioDefinition) -> Self {
        Self {
            keepers: scenario
                .rival_keepers
                .iter()
                .map(|keeper| RivalKeeperAiState {
                    owner: keeper.owner.clone(),
                    ai_profile: keeper.ai_profile.clone(),
                    preferred_creatures: keeper.preferred_creatures.clone(),
                    desired_rooms: keeper.starting_rooms.clone(),
                    next_decision_time: 1.0,
                    min_garrison: keeper.min_garrison,
                    raid_size: keeper.raid_size,
                    dig_batch: keeper.dig_batch,
                    room_size: keeper.room_size,
                    attack_cooldown: keeper.attack_cooldown,
                    next_attack_time: keeper.attack_cooldown,
                })
                .collect(),
        }
    }

    pub fn merge_from_scenario(&mut self, scenario: &ScenarioDefinition) {
        for keeper in &scenario.rival_keepers {
            self.upsert(RivalKeeperAiState {
                owner: keeper.owner.clone(),
                ai_profile: keeper.ai_profile.clone(),
                preferred_creatures: keeper.preferred_creatures.clone(),
                desired_rooms: keeper.starting_rooms.clone(),
                next_decision_time: 1.0,
                min_garrison: keeper.min_garrison,
                raid_size: keeper.raid_size,
                dig_batch: keeper.dig_batch,
                room_size: keeper.room_size,
                attack_cooldown: keeper.attack_cooldown,
                next_attack_time: keeper.attack_cooldown,
            });
        }
    }

    pub fn upsert(&mut self, keeper: RivalKeeperAiState) {
        if let Some(existing) = self
            .keepers
            .iter_mut()
            .find(|existing| existing.owner == keeper.owner)
        {
            *existing = keeper;
        } else {
            self.keepers.push(keeper);
        }
    }
}

pub fn default_min_garrison() -> usize {
    DEFAULT_MIN_GARRISON
}

pub fn default_raid_size() -> usize {
    DEFAULT_RAID_SIZE
}

pub fn default_dig_batch() -> usize {
    DEFAULT_DIG_BATCH
}

pub fn default_room_size() -> usize {
    DEFAULT_ROOM_SIZE
}

pub fn default_attack_cooldown() -> f32 {
    DEFAULT_ATTACK_COOLDOWN
}
