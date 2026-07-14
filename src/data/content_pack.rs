use crate::data::{
    CampaignDefinition, GameData, HeroBuildingData, HeroData, MonsterData, RoomData,
    ScenarioDefinition, SpellData, TechData, TileData, TraitData, TrapData,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub trait Identified {
    fn id(&self) -> &str;
}

impl Identified for TileData {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identified for RoomData {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identified for MonsterData {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identified for HeroData {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identified for SpellData {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identified for TrapData {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identified for TraitData {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identified for TechData {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identified for HeroBuildingData {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Identified for ScenarioDefinition {
    fn id(&self) -> &str {
        &self.meta.id
    }
}

impl Identified for CampaignDefinition {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentPackManifest {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub data: ContentPackDataPaths,
    #[serde(default)]
    pub assets: Vec<String>,
    #[serde(default)]
    pub maps: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentPackDataPaths {
    #[serde(default)]
    pub tiles: Vec<String>,
    #[serde(default)]
    pub rooms: Vec<String>,
    #[serde(default)]
    pub monsters: Vec<String>,
    #[serde(default)]
    pub heroes: Vec<String>,
    #[serde(default)]
    pub spells: Vec<String>,
    #[serde(default)]
    pub traps: Vec<String>,
    #[serde(default)]
    pub technologies: Vec<String>,
    #[serde(default)]
    pub traits: Vec<String>,
    #[serde(default)]
    pub hero_buildings: Vec<String>,
    #[serde(default)]
    pub scenarios: Vec<String>,
    #[serde(default)]
    pub campaigns: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentPackLoadOrder {
    #[serde(default)]
    pub packs: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContentPackReport {
    pub pack_id: String,
    pub replaced_ids: Vec<String>,
    pub added_ids: Vec<String>,
}

pub fn load_manifest(path: &Path) -> Result<ContentPackManifest, Box<dyn Error>> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub fn load_order(path: &Path) -> Result<ContentPackLoadOrder, Box<dyn Error>> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub fn load_manifests_from_order(
    mods_root: &Path,
    load_order: &ContentPackLoadOrder,
) -> Result<Vec<(PathBuf, ContentPackManifest)>, Box<dyn Error>> {
    let mut manifests = Vec::new();

    for pack_dir in &load_order.packs {
        let root = mods_root.join(pack_dir);
        let manifest = load_manifest(&root.join("pack.json"))?;
        manifests.push((root, manifest));
    }

    Ok(manifests)
}

pub fn merge_items<T: Identified>(
    target: &mut HashMap<String, T>,
    items: Vec<T>,
) -> ContentPackReport {
    let mut report = ContentPackReport::default();
    for item in items {
        let id = item.id().to_string();
        if target.insert(id.clone(), item).is_some() {
            report.replaced_ids.push(id);
        } else {
            report.added_ids.push(id);
        }
    }
    report
}

pub fn apply_content_pack(
    game_data: &mut GameData,
    manifest: &ContentPackManifest,
    root: &Path,
) -> Result<ContentPackReport, Box<dyn Error>> {
    let mut report = ContentPackReport {
        pack_id: manifest.id.clone(),
        ..Default::default()
    };

    merge_file_set(
        root,
        &manifest.data.tiles,
        &mut game_data.tiles,
        &mut report,
    )?;
    merge_file_set(
        root,
        &manifest.data.rooms,
        &mut game_data.rooms,
        &mut report,
    )?;
    merge_file_set(
        root,
        &manifest.data.monsters,
        &mut game_data.monsters,
        &mut report,
    )?;
    merge_file_set(
        root,
        &manifest.data.heroes,
        &mut game_data.heroes,
        &mut report,
    )?;
    merge_file_set(
        root,
        &manifest.data.spells,
        &mut game_data.spells,
        &mut report,
    )?;
    merge_file_set(
        root,
        &manifest.data.traps,
        &mut game_data.traps,
        &mut report,
    )?;
    merge_file_set(
        root,
        &manifest.data.technologies,
        &mut game_data.technologies,
        &mut report,
    )?;
    merge_file_set(
        root,
        &manifest.data.traits,
        &mut game_data.traits,
        &mut report,
    )?;
    merge_file_set(
        root,
        &manifest.data.hero_buildings,
        &mut game_data.hero_buildings,
        &mut report,
    )?;
    merge_file_set(
        root,
        &manifest.data.scenarios,
        &mut game_data.scenarios,
        &mut report,
    )?;
    merge_file_set(
        root,
        &manifest.data.campaigns,
        &mut game_data.campaigns,
        &mut report,
    )?;
    game_data.register_content_pack_roots(manifest, root);

    Ok(report)
}

pub fn apply_ordered_content_packs(
    game_data: &mut GameData,
    manifests: &[(PathBuf, ContentPackManifest)],
) -> Result<Vec<ContentPackReport>, Box<dyn Error>> {
    let mut reports = Vec::new();
    for (root, manifest) in manifests {
        reports.push(apply_content_pack(game_data, manifest, root)?);
    }
    game_data.content_pack_reports = reports.clone();

    Ok(reports)
}

fn merge_file_set<T>(
    root: &Path,
    files: &[String],
    target: &mut HashMap<String, T>,
    report: &mut ContentPackReport,
) -> Result<(), Box<dyn Error>>
where
    T: DeserializeOwned + Identified,
{
    for file in files {
        let path = root.join(file);
        let items: Vec<T> = read_json_array(&path)?;
        let item_report = merge_items(target, items);
        report.replaced_ids.extend(item_report.replaced_ids);
        report.added_ids.extend(item_report.added_ids);
    }

    Ok(())
}

fn read_json_array<T: DeserializeOwned>(path: &Path) -> Result<Vec<T>, Box<dyn Error>> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Deserialize)]
    struct TestItem {
        id: String,
        value: i32,
    }

    impl Identified for TestItem {
        fn id(&self) -> &str {
            &self.id
        }
    }

    #[test]
    fn merge_reports_added_and_replaced_ids() {
        let mut target = HashMap::new();
        target.insert(
            "room_a".to_string(),
            TestItem {
                id: "room_a".to_string(),
                value: 1,
            },
        );

        let report = merge_items(
            &mut target,
            vec![
                TestItem {
                    id: "room_a".to_string(),
                    value: 2,
                },
                TestItem {
                    id: "room_b".to_string(),
                    value: 3,
                },
            ],
        );

        assert_eq!(target["room_a"].value, 2);
        assert_eq!(target["room_b"].value, 3);
        assert_eq!(report.replaced_ids, vec!["room_a"]);
        assert_eq!(report.added_ids, vec!["room_b"]);
    }

    #[test]
    fn load_order_resolves_pack_manifests() {
        let root =
            std::env::temp_dir().join(format!("dungeon_manager_mod_test_{}", std::process::id()));
        let pack_root = root.join("test_pack");
        std::fs::create_dir_all(&pack_root).unwrap();
        std::fs::write(
            pack_root.join("pack.json"),
            r#"{
              "id": "test_pack",
              "name": "Test Pack",
              "priority": 10
            }"#,
        )
        .unwrap();

        let order = ContentPackLoadOrder {
            packs: vec!["test_pack".to_string()],
        };

        let manifests = load_manifests_from_order(&root, &order).unwrap();
        assert_eq!(manifests[0].1.id, "test_pack");

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn load_order_later_pack_overrides_earlier_data_and_assets() {
        let root = std::env::temp_dir().join(format!(
            "dungeon_manager_mod_override_test_{}",
            std::process::id()
        ));
        let first_root = root.join("first");
        let second_root = root.join("second");
        std::fs::create_dir_all(first_root.join("data")).unwrap();
        std::fs::create_dir_all(second_root.join("data")).unwrap();
        std::fs::create_dir_all(second_root.join("maps")).unwrap();
        std::fs::create_dir_all(second_root.join("assets/tiles")).unwrap();

        std::fs::write(
            first_root.join("pack.json"),
            r#"{
              "id": "first",
              "name": "First",
              "priority": 100,
              "data": { "campaigns": ["data/campaigns.json"] }
            }"#,
        )
        .unwrap();
        std::fs::write(
            first_root.join("data/campaigns.json"),
            r#"[{ "id": "override_campaign", "name": "First Campaign" }]"#,
        )
        .unwrap();

        std::fs::write(
            second_root.join("pack.json"),
            r#"{
              "id": "second",
              "name": "Second",
              "priority": 0,
              "assets": ["assets"],
              "maps": ["maps"],
              "data": { "campaigns": ["data/campaigns.json"] }
            }"#,
        )
        .unwrap();
        std::fs::write(
            second_root.join("data/campaigns.json"),
            r#"[{ "id": "override_campaign", "name": "Second Campaign" }]"#,
        )
        .unwrap();
        std::fs::write(second_root.join("maps/level_1.json"), "{}").unwrap();
        std::fs::write(second_root.join("assets/tiles/lair.png"), b"").unwrap();

        let order = ContentPackLoadOrder {
            packs: vec!["first".to_string(), "second".to_string()],
        };
        let manifests = load_manifests_from_order(&root, &order).unwrap();
        let mut data = GameData::load().unwrap();

        let reports = apply_ordered_content_packs(&mut data, &manifests).unwrap();

        assert_eq!(
            reports
                .iter()
                .map(|r| r.pack_id.as_str())
                .collect::<Vec<_>>(),
            vec!["first", "second"]
        );
        assert_eq!(data.campaigns["override_campaign"].name, "Second Campaign");
        assert_eq!(
            data.resolve_map_path("assets/maps/level_1.json"),
            second_root.join("maps/level_1.json")
        );
        assert!(data
            .resolve_asset_path("assets/tiles/lair.png")
            .ends_with("second/assets/tiles/lair.png"));

        let _ = std::fs::remove_dir_all(root);
    }
}
