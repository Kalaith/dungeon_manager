//! Content wiring tests
//!
//! Shipping a room is four separate edits — a `rooms.json` entry, art, a
//! `technologies.json` tech that unlocks it, and a slot in each mission's
//! `availability` — spread across files that never reference each other in
//! Rust. Nothing but these tests notices when one of the four is misspelled or
//! forgotten, and a room that no tech unlocks is a room the player can see in
//! the build bar and never build.
//!
//! Run with: cargo test --test content_wiring_tests

use serde_json::Value;
use std::collections::HashSet;
use std::path::PathBuf;

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn load(rel: &str) -> Vec<Value> {
    let path = project_root().join(rel);
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("failed to parse {} as a JSON array: {e}", path.display()))
}

fn ids(rel: &str) -> HashSet<String> {
    load(rel)
        .iter()
        .map(|entry| entry["id"].as_str().expect("id").to_string())
        .collect()
}

fn room_ids() -> HashSet<String> {
    let mut all = ids("assets/data/rooms.json");
    all.extend(ids("assets/data/special_rooms.json"));
    all
}

fn assert_empty(problems: Vec<String>, what: &str) {
    assert!(problems.is_empty(), "{what}:\n  {}", problems.join("\n  "));
}

#[test]
fn every_tech_unlock_names_real_content() {
    let rooms = room_ids();
    let spells = ids("assets/data/dungeon_spells.json");
    let creatures = ids("assets/data/monsters.json");
    let traps = ids("assets/data/traps.json");

    let mut problems = Vec::new();
    for tech in load("assets/data/technologies.json") {
        let tech_id = tech["id"].as_str().expect("tech id");
        for (kind, known) in [
            ("rooms", &rooms),
            ("spells", &spells),
            ("creatures", &creatures),
            ("traps", &traps),
        ] {
            let Some(list) = tech["unlocks"][kind].as_array() else {
                continue;
            };
            for unlocked in list {
                let id = unlocked.as_str().expect("unlock id");
                if !known.contains(id) {
                    problems.push(format!("tech `{tech_id}` unlocks unknown {kind} `{id}`"));
                }
            }
        }
    }
    assert_empty(
        problems,
        "technology unlocks reference content that does not exist",
    );
}

#[test]
fn every_tech_prerequisite_exists() {
    let techs = ids("assets/data/technologies.json");
    let mut problems = Vec::new();
    for tech in load("assets/data/technologies.json") {
        let tech_id = tech["id"].as_str().expect("tech id");
        for prereq in tech["prerequisites"].as_array().expect("prerequisites") {
            let id = prereq.as_str().expect("prerequisite id");
            if !techs.contains(id) {
                problems.push(format!("tech `{tech_id}` requires unknown tech `{id}`"));
            }
        }
    }
    assert_empty(
        problems,
        "technology prerequisites reference techs that do not exist",
    );
}

#[test]
fn every_room_not_unlocked_by_default_has_a_tech_that_grants_it() {
    // Rooms used to carry a `requirements.research` list beside the tech tree.
    // Nothing enforced it and the sidebar displayed it, so it drifted: six
    // rooms showed no requirement while being tech-locked, and the scavenger
    // room named `ritual_tech` when `logistics` was the real gate. The field
    // is gone; this is the check that replaces it — a room the player does not
    // start with must be reachable through research, or it can never be built.
    const UNLOCKED_AT_START: &[&str] = &[
        // Mirrors `PlayerState::new`.
        "lair",
        "hatchery",
        "treasury",
        "library",
        "dungeon_heart",
    ];

    let mut unlocked_by_tech: HashSet<String> = HashSet::new();
    for tech in load("assets/data/technologies.json") {
        for room in tech["unlocks"]["rooms"].as_array().expect("unlocks.rooms") {
            unlocked_by_tech.insert(room.as_str().expect("room id").to_string());
        }
    }

    let mut unreachable = Vec::new();
    for room in load("assets/data/rooms.json") {
        let id = room["id"].as_str().expect("room id");
        if UNLOCKED_AT_START.contains(&id) || unlocked_by_tech.contains(id) {
            continue;
        }
        unreachable.push(format!(
            "room `{id}` is neither unlocked at start nor granted by any technology"
        ));
    }

    assert_empty(unreachable, "rooms the player can never build");
}

#[test]
fn every_scenario_availability_entry_names_real_content() {
    let rooms = room_ids();
    let spells = ids("assets/data/dungeon_spells.json");
    let creatures = ids("assets/data/monsters.json");
    let traps = ids("assets/data/traps.json");

    let scenarios = project_root().join("assets").join("scenarios");
    let mut problems = Vec::new();
    for entry in std::fs::read_dir(&scenarios)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", scenarios.display()))
        .flatten()
    {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let raw = std::fs::read_to_string(&path).expect("read scenario");
        let parsed: Value = serde_json::from_str(&raw)
            .unwrap_or_else(|e| panic!("failed to parse {}: {e}", path.display()));
        // Scenario files are written both as a bare object and as a one-element
        // array; accept either rather than making authors care.
        let scenario = parsed.get(0).unwrap_or(&parsed);
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        for (kind, known) in [
            ("rooms", &rooms),
            ("spells", &spells),
            ("creatures", &creatures),
            ("traps", &traps),
        ] {
            let Some(entries) = scenario["availability"][kind].as_object() else {
                continue;
            };
            for id in entries.keys() {
                if !known.contains(id.as_str()) {
                    problems.push(format!("scenario `{name}` lists unknown {kind} `{id}`"));
                }
            }
        }
    }
    assert_empty(
        problems,
        "scenario availability references content that does not exist",
    );
}
