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
fn every_room_research_requirement_names_a_real_tech() {
    let techs = ids("assets/data/technologies.json");
    let mut problems = Vec::new();
    for room in load("assets/data/rooms.json") {
        let room_id = room["id"].as_str().expect("room id");
        for required in room["requirements"]["research"]
            .as_array()
            .expect("requirements.research")
        {
            let id = required.as_str().expect("research id");
            if !techs.contains(id) {
                problems.push(format!("room `{room_id}` requires unknown tech `{id}`"));
            }
        }
    }
    assert_empty(
        problems,
        "room research requirements reference techs that do not exist",
    );
}

#[test]
fn every_research_gated_room_has_a_tech_that_unlocks_it() {
    // The inverse of the check above, and the one that actually bites: a room
    // gated on research that no tech grants can never be built.
    let mut unlocked_by_tech: HashSet<String> = HashSet::new();
    for tech in load("assets/data/technologies.json") {
        for room in tech["unlocks"]["rooms"].as_array().expect("unlocks.rooms") {
            unlocked_by_tech.insert(room.as_str().expect("room id").to_string());
        }
    }

    let mut problems = Vec::new();
    for room in load("assets/data/rooms.json") {
        let room_id = room["id"].as_str().expect("room id");
        let gated = !room["requirements"]["research"]
            .as_array()
            .expect("requirements.research")
            .is_empty();
        if gated && !unlocked_by_tech.contains(room_id) {
            problems.push(format!(
                "room `{room_id}` is research-gated but no technology unlocks it"
            ));
        }
    }
    assert_empty(problems, "research-gated rooms that nothing can unlock");
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
