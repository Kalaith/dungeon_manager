//! Asset manifest tests
//!
//! Every sprite path the content data declares must resolve to a file that the
//! graphics generator actually emits. This guards the failure mode that made
//! these tests worth writing: `rooms.json` pointed at `tiles/lair_floor.png`
//! and friends, none of which had ever existed, while the loader carried a
//! hand-maintained list that quietly ignored the data — so eleven rooms, every
//! trap but one and ten floor variants shipped with art on disk that never
//! reached the screen, and nothing failed.
//!
//! Run with: cargo test --test asset_manifest_tests

use serde_json::Value;
use std::path::{Path, PathBuf};

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

/// `assets/`-relative sprite reference -> absolute path on disk.
fn asset_path(sprite: &str) -> PathBuf {
    project_root().join("assets").join(sprite)
}

fn assert_all_exist(missing: Vec<String>, what: &str) {
    assert!(
        missing.is_empty(),
        "{what} reference art that does not exist. Either add the generator \
         (graphics_gen/) or fix the path:\n  {}",
        missing.join("\n  ")
    );
}

#[test]
fn every_tile_sprite_exists() {
    let mut missing = Vec::new();
    for tile in load("assets/data/tiles.json") {
        let id = tile["id"].as_str().expect("tile id");
        let sprite = tile["visual"]["sprite"].as_str().expect("tile sprite");
        if !asset_path(sprite).exists() {
            missing.push(format!("tile `{id}` -> {sprite}"));
        }
    }
    assert_all_exist(missing, "tiles.json entries");
}

#[test]
fn every_room_floor_sprite_exists() {
    let mut missing = Vec::new();
    for file in ["assets/data/rooms.json", "assets/data/special_rooms.json"] {
        for room in load(file) {
            let id = room["id"].as_str().expect("room id");
            let sprite = room["visual"]["floor_sprite"]
                .as_str()
                .expect("room floor_sprite");
            if !asset_path(sprite).exists() {
                missing.push(format!("room `{id}` -> {sprite}"));
            }
        }
    }
    assert_all_exist(missing, "room floor sprites");
}

#[test]
fn every_hero_building_sprite_exists() {
    let mut missing = Vec::new();
    for building in load("assets/data/hero_buildings.json") {
        let id = building["id"].as_str().expect("building id");
        let sprite = building["visual"]["tile"].as_str().expect("building tile");
        if !asset_path(sprite).exists() {
            missing.push(format!("hero building `{id}` -> {sprite}"));
        }
    }
    assert_all_exist(missing, "hero building tiles");
}

#[test]
fn every_trap_sprite_exists() {
    // Traps carry no `visual` block, so the loader falls back to the
    // `assets/tiles/<id>.png` convention the generator emits.
    let mut missing = Vec::new();
    for trap in load("assets/data/traps.json") {
        let id = trap["id"].as_str().expect("trap id");
        let sprite = format!("tiles/{id}.png");
        if !asset_path(&sprite).exists() {
            missing.push(format!("trap `{id}` -> {sprite}"));
        }
    }
    assert_all_exist(missing, "trap tiles");
}

#[test]
fn every_creature_and_hero_sprite_exists() {
    let mut missing = Vec::new();
    for (file, dir, what) in [
        ("assets/data/monsters.json", "sprites/monsters", "creature"),
        ("assets/data/heroes.json", "sprites/heroes", "hero"),
    ] {
        for entry in load(file) {
            let id = entry["id"].as_str().expect("roster entry id");
            let sprite = format!("{dir}/{id}.png");
            if !asset_path(&sprite).exists() {
                missing.push(format!("{what} `{id}` -> {sprite}"));
            }
        }
    }
    assert_all_exist(missing, "roster sprites");
}

#[test]
fn generated_tile_art_is_reachable_from_data() {
    // The other direction: art that no data entry points at is art that can
    // never be drawn. Orphans are how `dungeon_barracks.png` sat unused while
    // the barracks room borrowed the hero base's building art instead.
    let mut declared: Vec<String> = Vec::new();

    for tile in load("assets/data/tiles.json") {
        declared.push(tile["visual"]["sprite"].as_str().unwrap().to_string());
    }
    for file in ["assets/data/rooms.json", "assets/data/special_rooms.json"] {
        for room in load(file) {
            declared.push(room["visual"]["floor_sprite"].as_str().unwrap().to_string());
        }
    }
    for building in load("assets/data/hero_buildings.json") {
        declared.push(building["visual"]["tile"].as_str().unwrap().to_string());
    }
    for trap in load("assets/data/traps.json") {
        declared.push(format!("tiles/{}.png", trap["id"].as_str().unwrap()));
    }
    // Drawn by the renderer directly rather than via a data entry.
    declared.push("tiles/gold_pile.png".to_string());

    let orphans = orphaned_tiles(&declared);
    assert!(
        orphans.is_empty(),
        "generated tile art that no data entry references — either wire it up \
         or drop the generator:\n  {}",
        orphans.join("\n  ")
    );
}

/// Every `assets/tiles/**.png` that `declared` does not mention.
fn orphaned_tiles(declared: &[String]) -> Vec<String> {
    // Decorative floor variants a map author can select per-tile; they are
    // reachable through map JSON rather than a fixed data entry.
    const AUTHORABLE_FLOORS: &[&str] = &[
        "carpet",
        "grass",
        "marble_floor",
        "mosaic_floor",
        "sand",
        "snow",
    ];
    // Art that is finished and waiting on a data entry to become playable —
    // three traps needing balance numbers in `traps.json`, three hero
    // buildings needing spawn/destruction rules in `hero_buildings.json`, and
    // the casino, superseded by the Leisure Den in docs/ROOM_SET.md. Tracked
    // in TODO.md; shrink this list, don't grow it.
    const AWAITING_DATA: &[&str] = &[
        "blacksmith",
        "casino",
        "fire_trap",
        "gas_trap",
        "guard_tower",
        "lightning_trap",
        "tavern",
    ];

    let tiles_dir = project_root().join("assets").join("tiles");
    let mut orphans = Vec::new();
    collect_orphans(&tiles_dir, &tiles_dir, declared, &mut orphans);

    orphans.retain(|rel| {
        let stem = Path::new(rel)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        !AUTHORABLE_FLOORS.contains(&stem) && !AWAITING_DATA.contains(&stem)
    });
    orphans.sort();
    orphans
}

fn collect_orphans(root: &Path, dir: &Path, declared: &[String], out: &mut Vec<String>) {
    let entries =
        std::fs::read_dir(dir).unwrap_or_else(|e| panic!("failed to read {}: {e}", dir.display()));
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_orphans(root, &path, declared, out);
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("png") {
            continue;
        }
        let rel = path
            .strip_prefix(root)
            .expect("tile under assets/tiles")
            .to_string_lossy()
            .replace('\\', "/");
        let reference = format!("tiles/{rel}");
        if !declared.contains(&reference) {
            out.push(reference);
        }
    }
}
