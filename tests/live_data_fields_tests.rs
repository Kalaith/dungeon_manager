//! Guards against content-data fields that are parsed and then never read.
//!
//! Ten such fields have been found so far, and every one of them turned up by
//! accident while building something else: six on `EffectsData`
//! (`happiness_modifier` — twelve rooms carried tuned values that did nothing;
//! `research_rate`, declared under a name no JSON used, so serde silently
//! dropped the authored value; `xp_per_minute`; `creature_defense_modifier`;
//! `sleep_recovery_rate`; `grouping_point`), plus `lockable`, `trigger_type`,
//! `durability` and `damage_per_second`. The failure mode is always silence:
//! the data looks authored, the entity looks configured, and nothing reports
//! that the number goes nowhere.
//!
//! Each guarded struct is checked field by field against the rest of `src/`.
//! That is a coarse signal — it proves a field is referenced, not that the
//! reference is correct — but it is precisely the signal that was missing, and
//! it costs nothing.
//!
//! Run with: cargo test --test live_data_fields_tests

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// A content struct whose fields must all be read somewhere.
struct Guarded {
    decl: &'static str,
    struct_name: &'static str,
    /// Fields knowingly parsed but not yet consumed, each with the reason.
    /// Shrink these lists; do not grow them. A field belongs here only when the
    /// subsystem that would read it does not exist yet — every entry below
    /// corresponds to a named item in TODO.md.
    unconsumed: &'static [(&'static str, &'static str)],
}

const GUARDED: &[Guarded] = &[
    Guarded {
        decl: "src/data/rooms.rs",
        struct_name: "EffectsData",
        unconsumed: &[(
            "hero_conversion_rate",
            "prison hero->creature conversion is not built",
        )],
    },
    Guarded {
        decl: "src/data/traps.rs",
        struct_name: "TrapEffects",
        unconsumed: &[
            (
                "lockable",
                "all three doors author it; magical door locking is not built",
            ),
            (
                "trigger_type",
                "every trap is \"pressure\"; decorative until a second trigger exists",
            ),
        ],
    },
    Guarded {
        decl: "src/data/tiles.rs",
        struct_name: "TileData",
        unconsumed: &[
            (
                "durability",
                "dig cost comes from config; per-tile durability is a balance change",
            ),
            (
                "damage_per_second",
                "lava authors 25 but also blocks movement, so nothing can stand in it",
            ),
        ],
    },
    Guarded {
        decl: "src/data/monsters.rs",
        struct_name: "MonsterData",
        unconsumed: &[],
    },
];

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
fn every_guarded_data_field_is_read_somewhere() {
    let mut failures = Vec::new();

    for guarded in GUARDED {
        let fields = struct_fields(guarded.decl, guarded.struct_name);
        assert!(
            !fields.is_empty(),
            "parsed no fields from {}; the struct layout must have changed",
            guarded.struct_name
        );

        let sources: Vec<String> = source_files_excluding(guarded.decl)
            .iter()
            .map(|path| std::fs::read_to_string(path).expect("read source"))
            .collect();

        let allowed: BTreeSet<&str> = guarded.unconsumed.iter().map(|(name, _)| *name).collect();

        for field in &fields {
            let read = sources.iter().any(|source| source.contains(field.as_str()));
            match (read, allowed.contains(field.as_str())) {
                (false, false) => failures.push(format!(
                    "{}::{field} is parsed but read nowhere. Wire it up, delete it \
(CODE_STANDARDS.md on unused code), or add it to `unconsumed` with a reason.",
                    guarded.struct_name
                )),
                (true, true) => failures.push(format!(
                    "{}::{field} is listed as unconsumed but is now read. Drop it from the list.",
                    guarded.struct_name
                )),
                _ => {}
            }
        }
    }

    assert!(failures.is_empty(), "\n  {}", failures.join("\n  "));
}

#[test]
fn unconsumed_lists_name_fields_that_exist() {
    // Keeps each allowlist from outliving the fields it excuses.
    let mut stale = Vec::new();

    for guarded in GUARDED {
        let fields: BTreeSet<String> = struct_fields(guarded.decl, guarded.struct_name)
            .into_iter()
            .collect();
        for (name, _) in guarded.unconsumed {
            if !fields.contains(*name) {
                stale.push(format!("{}::{name}", guarded.struct_name));
            }
        }
    }

    assert!(
        stale.is_empty(),
        "`unconsumed` names fields that no longer exist: {stale:?}"
    );
}
