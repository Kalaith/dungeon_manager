use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub faction: String,
    pub role: String,
    pub stats: StatsData,
    pub needs: HashMap<String, NeedData>,
    pub traits: Vec<String>,
    pub ai: MonsterAIData,
    pub combat: CombatData,
    pub progression: ProgressionData,
    pub economy: EconomyData,
    pub spawn: SpawnData,
    pub visual: MonsterVisualData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsData {
    pub health: f32,
    pub mana: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub carry_capacity: i32,
    pub sight_radius: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeedData {
    pub decay_per_minute: f32,
    pub satisfied_by: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stash_amount: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterAIData {
    pub base_mood: f32,
    pub anger_threshold: f32,
    pub desertion_threshold: f32,
    pub task_preferences: HashMap<String, f32>,
    pub room_desires: HashMap<String, f32>,
    pub discipline_response: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatData {
    pub attack_type: String,
    pub damage_range: [f32; 2],
    pub attack_speed: f32,
    pub armor_type: String,
    pub resistances: HashMap<String, f32>,
    pub abilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionData {
    pub xp_to_level: Vec<u32>,
    pub stat_growth_per_level: HashMap<String, f32>,
    pub max_level: u32,
    pub mutations: Vec<MutationData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationData {
    pub id: String,
    pub conditions: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomyData {
    pub wage_per_minute: i32,
    pub steals_if_unpaid: bool,
    pub drops_gold_on_death: [i32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnData {
    pub source: String,
    pub min_dungeon_reputation: i32,
    pub preferred_rooms: Vec<String>,
    pub spawn_weight: f32,
    pub max_population: u32,
    #[serde(default)]
    pub summon_base_cost: Option<i32>,
    #[serde(default)]
    pub summon_cost_per_existing: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterVisualData {
    pub sprite: String,
    pub scale: f32,
    pub animations: Vec<String>,
    pub voice_set: String,
}

pub fn load_monsters() -> Result<HashMap<String, MonsterData>, Box<dyn Error>> {
    let json_content = include_str!("../../assets/data/monsters.json");
    let monsters_vec: Vec<MonsterData> = serde_json::from_str(json_content)?;

    let mut monsters_map = HashMap::new();
    for monster in monsters_vec {
        monsters_map.insert(monster.id.clone(), monster);
    }

    Ok(monsters_map)
}

#[cfg(test)]
mod tests {
    use crate::data::GameData;

    #[test]
    fn orphan_sprite_monsters_are_wired() {
        let game_data = GameData::load().expect("game data should load");
        // The four previously-orphaned sprites now have data entries.
        for id in ["zombie", "ghost", "bat_swarm", "dark_elf"] {
            let monster = game_data
                .monsters
                .get(id)
                .unwrap_or_else(|| panic!("{id} should be in the roster"));
            assert!(!monster.name.is_empty());
            assert!(monster.stats.health > 0.0);
        }
    }

    #[test]
    fn every_monster_trait_resolves_against_the_trait_system() {
        // Traits are data-driven; a typo'd trait id would silently do nothing.
        // Guard the whole roster (incl. the new monsters) against dangling ids.
        let game_data = GameData::load().expect("game data should load");
        for monster in game_data.monsters.values() {
            for trait_id in &monster.traits {
                assert!(
                    game_data.traits.contains_key(trait_id),
                    "monster '{}' references unknown trait '{}'",
                    monster.id,
                    trait_id
                );
            }
        }
    }
}
