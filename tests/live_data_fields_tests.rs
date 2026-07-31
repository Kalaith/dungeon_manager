//! Guards against room-effect fields that are parsed and then never read.
//!
//! Six `EffectsData` fields have shipped dead so far, and every one was found
//! by accident while building something else: `happiness_modifier` (twelve
//! rooms carried tuned values that did nothing), `research_rate` (declared
//! under a name no JSON used, so serde silently dropped the authored value),
//! `xp_per_minute`, `creature_defense_modifier`, `sleep_recovery_rate` and
//! `grouping_point`. The failure mode is always silence: the data looks
//! authored, the room looks configured, and nothing reports that the number
//! goes nowhere.
//!
//! This reads the struct definition and checks each field is mentioned
//! somewhere outside it. That is a coarse signal — it proves a field is
//! referenced, not that the reference is correct — but it is exactly the
//! signal that was missing, and it costs nothing.
//!
//! Run with: cargo test --test live_data_fields_tests

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Fields knowingly parsed but not yet consumed, with the reason. Shrink this
/// list; do not grow it. A field belongs here only when the subsystem that
/// would read it does not exist yet.
const KNOWN_UNCONSUMED: &[(&str, &str)] = &[(
    "hero_conversion_rate",
    "prison hero->creature conversion is not built; see TODO.md",
)];

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Field names declared in `struct <name>` in `rel`.
fn struct_fields(rel: &str, struct_name: &str) -> Vec<String> {
    let source = std::fs::read_to_string(project_root().join(rel))
        .unwrap_or_else(|e| panic!("failed to read {rel}: {e}"));

    let start = source
        .find(&format!("struct {struct_name} {{"))
        .unwrap_or_else(|| panic!("{struct_name} not found in {rel}"));
    let body_start = start + source[start..].find('{').expect("opening brace");
    let body_end = body_start
        + source[body_start..]
            .find("\n}")
            .expect("closing brace on its own line");
    let body = &source[body_start..body_end];

    body.lines()
        .filter_map(|line| {
            let line = line.trim();
            let rest = line.strip_prefix("pub ")?;
            let name = rest.split(':').next()?.trim();
            (!name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase() || c == '_'))
                .then(|| name.to_string())
        })
        .collect()
}

/// Every `.rs` file under `src/`, excluding the one declaring the struct.
fn source_files_excluding(declaring: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_rs(&project_root().join("src"), &mut files);
    let declaring = project_root().join(declaring);
    files.retain(|path| path != &declaring);
    files
}

fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries =
        std::fs::read_dir(dir).unwrap_or_else(|e| panic!("failed to read {}: {e}", dir.display()));
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

#[test]
fn every_room_effect_field_is_read_somewhere() {
    const DECL: &str = "src/data/rooms.rs";
    let fields = struct_fields(DECL, "EffectsData");
    assert!(
        fields.len() > 10,
        "expected to parse the EffectsData fields, got {fields:?}"
    );

    let sources: Vec<String> = source_files_excluding(DECL)
        .iter()
        .map(|path| std::fs::read_to_string(path).expect("read source"))
        .collect();

    let allowed: BTreeSet<&str> = KNOWN_UNCONSUMED.iter().map(|(name, _)| *name).collect();

    let mut dead = Vec::new();
    let mut needlessly_allowed = Vec::new();

    for field in &fields {
        let read = sources.iter().any(|source| source.contains(field.as_str()));
        match (read, allowed.contains(field.as_str())) {
            (false, false) => dead.push(field.clone()),
            // An allowlisted field that is now read: the note is stale.
            (true, true) => needlessly_allowed.push(field.clone()),
            _ => {}
        }
    }

    assert!(
        dead.is_empty(),
        "room effects parsed from rooms.json but read nowhere — either wire \
         them up or delete the field (see CODE_STANDARDS.md on unused code):\n  {}",
        dead.join("\n  ")
    );
    assert!(
        needlessly_allowed.is_empty(),
        "these are listed in KNOWN_UNCONSUMED but are now read — drop them \
         from the list:\n  {}",
        needlessly_allowed.join("\n  ")
    );
}

#[test]
fn known_unconsumed_fields_still_exist() {
    // Keeps the allowlist from outliving the fields it excuses.
    let fields: BTreeSet<String> = struct_fields("src/data/rooms.rs", "EffectsData")
        .into_iter()
        .collect();

    let stale: Vec<&str> = KNOWN_UNCONSUMED
        .iter()
        .map(|(name, _)| *name)
        .filter(|name| !fields.contains(*name))
        .collect();

    assert!(
        stale.is_empty(),
        "KNOWN_UNCONSUMED names fields that no longer exist: {stale:?}"
    );
}
