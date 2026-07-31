use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub cost: SpellCost,
    pub targeting: TargetingData,
    pub effects: Vec<SpellEffect>,
    pub cooldown: f32,
    pub visual: SpellVisualData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellCost {
    #[serde(default)]
    pub mana: i32,
    #[serde(default)]
    pub gold: i32,
    #[serde(default)]
    pub health: i32,
    // `souls` used to sit here: no spell in dungeon_spells.json ever authored
    // one, and the game has no soul resource for it to spend. Deleted rather
    // than left as an aspiration (CODE_STANDARDS.md on unused code).
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingData {
    #[serde(rename = "type")]
    pub target_type: String,
    #[serde(default)]
    pub range: u32,
    #[serde(default)]
    pub area_radius: u32,
    #[serde(default)]
    pub requires_visibility: bool,
    #[serde(default)]
    pub valid_targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellEffect {
    #[serde(rename = "type")]
    pub effect_type: String,
    #[serde(default)]
    pub amount: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_tile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_tile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellVisualData {
    pub icon: String,
}

pub fn load_spells() -> Result<HashMap<String, SpellData>, Box<dyn Error>> {
    let json_content = include_str!("../../assets/data/dungeon_spells.json");
    let spells_vec: Vec<SpellData> = serde_json::from_str(json_content)?;

    let mut spells_map = HashMap::new();
    for spell in spells_vec {
        spells_map.insert(spell.id.clone(), spell);
    }

    Ok(spells_map)
}

#[cfg(test)]
mod tests {
    use crate::data::GameData;

    #[test]
    fn new_battle_spells_are_wired_with_working_effects() {
        let game_data = GameData::load().expect("game data should load");
        // Effect types the spell dispatcher (`spell_effects::apply_spell_effect`)
        // actually executes — new spells must stick to these so they aren't inert.
        const SUPPORTED: [&str; 6] = [
            "damage",
            "heal",
            "stat_modifier",
            "status_apply",
            "spawn_entity",
            "reveal_map",
        ];
        for id in [
            "fireball",
            "frost_nova",
            "chain_lightning",
            "venom_spray",
            "raise_the_dead",
            "summon_skeletons",
        ] {
            let spell = game_data
                .spells
                .get(id)
                .unwrap_or_else(|| panic!("{id} should be in the spellbook"));
            assert!(!spell.effects.is_empty(), "{id} has no effects");
            for effect in &spell.effects {
                assert!(
                    SUPPORTED.contains(&effect.effect_type.as_str()),
                    "spell '{}' uses unsupported (inert) effect type '{}'",
                    id,
                    effect.effect_type
                );
            }
        }
        // The summon spells reference real creatures.
        for (spell_id, creature) in [
            ("raise_the_dead", "zombie"),
            ("summon_skeletons", "skeleton"),
        ] {
            assert!(game_data.monsters.contains_key(creature));
            let spell = &game_data.spells[spell_id];
            assert!(spell
                .effects
                .iter()
                .any(|e| e.entity.as_deref() == Some(creature)));
        }
    }
}
