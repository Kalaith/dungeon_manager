//! Saving and loading, and the slot identity that both hang off.
//!
//! # Two things were wrong here, and the slot literal was the visible one
//!
//! `"slot_1"` used to be a string literal at eleven call sites, so the game had
//! exactly one save and every new game silently overwrote it. Underneath that,
//! the native path wrote `save_slot_1.json` as a **relative** path — i.e. into
//! whatever directory the process happened to start in. Launch the game from a
//! shortcut, from Explorer, or from a store client and the save is somewhere
//! else, or the directory is not writable at all.
//!
//! Both are fixed by going through `macroquad_toolkit::persistence::slots`,
//! which this project had never adopted despite the toolkit carrying it: it
//! resolves `{app_data}/dungeon_manager/save_<slot>.json` on native and a
//! game-qualified key in the browser. See [`load_legacy`] for what happens to
//! saves written by the old build.

use crate::state::game_state::GameState;
// Imported by function rather than by module: the toolkit exports its own
// `SaveSlot` (a metadata header), and this module's `SaveSlot` is a slot
// *identity*. Naming both would be the drift hazard, not the convenience.
use macroquad_toolkit::persistence::{load_from_slot, save_to_slot_with_version, slot_exists};
use serde::{Deserialize, Serialize};

const GAME_NAME: &str = "dungeon_manager";
const SAVE_FORMAT_VERSION: &str = "0.1.0";

/// How many numbered slots the player gets.
pub const SLOT_COUNT: u8 = 3;

/// Which numbered slot a session reads and writes.
///
/// A newtype rather than a `u8` so an out-of-range slot cannot be constructed
/// and then quietly resolve to a filename nothing else knows about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SaveSlot(u8);

impl Default for SaveSlot {
    fn default() -> Self {
        Self(1)
    }
}

impl SaveSlot {
    /// `None` outside `1..=SLOT_COUNT`.
    pub fn new(number: u8) -> Option<Self> {
        (1..=SLOT_COUNT).contains(&number).then_some(Self(number))
    }

    pub fn number(self) -> u8 {
        self.0
    }

    pub fn all() -> impl Iterator<Item = Self> {
        (1..=SLOT_COUNT).map(Self)
    }

    /// The name the persistence layer files this slot under.
    fn key(self) -> String {
        format!("slot_{}", self.0)
    }
}

impl std::fmt::Display for SaveSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Slot {}", self.0)
    }
}

/// Written beside the game state so a slot can be *described* without
/// deserializing a whole dungeon — which is what a save picker needs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveMeta {
    /// Mission the save is in, or `"skirmish"` when there is no scenario.
    pub scenario_id: String,
    /// Hero wave reached.
    pub wave: u32,
    /// In-game seconds elapsed.
    pub in_game_seconds: f32,
    /// Wall-clock seconds since the Unix epoch, for ordering slots by recency.
    pub saved_at: f64,
    pub version: String,
}

impl SaveMeta {
    fn describe(state: &GameState) -> Self {
        Self {
            scenario_id: state
                .scenario_runtime
                .as_ref()
                .map(|runtime| runtime.scenario_id.clone())
                .unwrap_or_else(|| "skirmish".to_string()),
            wave: state.hero_base.current_wave_number,
            in_game_seconds: state.time_elapsed,
            // `date::now` is the one clock that exists on both native and wasm;
            // `std::time` does not compile for the web build.
            saved_at: macroquad::miniquad::date::now(),
            version: SAVE_FORMAT_VERSION.to_string(),
        }
    }
}

#[derive(Serialize)]
struct SaveFile<'a> {
    meta: SaveMeta,
    game_state: &'a GameState,
}

#[derive(Deserialize)]
struct LoadFile {
    game_state: GameState,
}

/// Just the header. Unknown fields are ignored, so this reads a full save
/// without paying to construct the `GameState` inside it.
#[derive(Deserialize)]
struct MetaOnly {
    meta: SaveMeta,
}

/// The shape the pre-slot build wrote: no `meta`, and on native at a relative
/// path. Kept only to be read.
#[derive(Deserialize)]
struct LegacySave {
    game_state: GameState,
}

/// Save to a slot.
pub fn save_game(game_state: &GameState, slot: SaveSlot) -> Result<(), String> {
    let file = SaveFile {
        meta: SaveMeta::describe(game_state),
        game_state,
    };
    save_to_slot_with_version(GAME_NAME, &slot.key(), &file, SAVE_FORMAT_VERSION)
}

/// Load from a slot, falling back to a save the old build wrote.
///
/// Stamps `active_slot` on the way out. `active_slot` is `#[serde(skip)]`, so a
/// freshly loaded state defaults to slot 1 no matter which slot it came from —
/// without this, loading slot 3 and then saving would write over slot 1. Doing
/// it here rather than at each call site means a future loader cannot forget.
pub fn load_game(slot: SaveSlot) -> Result<GameState, String> {
    let mut state = match load_from_slot::<LoadFile>(GAME_NAME, &slot.key()) {
        Ok(file) => file.game_state,
        Err(current) => load_legacy(slot)
            .map_err(|legacy| format!("No save in {slot} ({current}; legacy: {legacy})"))?,
    };
    state.active_slot = slot;
    Ok(state)
}

/// Is there anything in this slot?
///
/// Deliberately existence-only: the menus call this every frame while they are
/// on screen, so it must not parse the save. Use [`peek_slot`] when the
/// contents actually matter — from a click, not from a draw.
pub fn save_exists(slot: SaveSlot) -> bool {
    slot_exists(GAME_NAME, &slot.key()) || legacy_exists(slot)
}

/// Any slot at all — what a "LOAD GAME" button needs to know.
pub fn any_save_exists() -> bool {
    SaveSlot::all().any(save_exists)
}

/// Describe a slot for a picker. `None` if empty, or if it is a legacy save
/// (which carries no header — it still *loads*, it just cannot be summarised).
pub fn peek_slot(slot: SaveSlot) -> Option<SaveMeta> {
    load_from_slot::<MetaOnly>(GAME_NAME, &slot.key())
        .ok()
        .map(|peeked| peeked.meta)
}

/// The slot to offer when the player has not picked one — most recently saved,
/// falling back to the lowest occupied slot for legacy saves with no timestamp.
///
/// Parses every occupied slot, so call it from a click and not from a draw.
pub fn most_recent_slot() -> Option<SaveSlot> {
    let occupied: Vec<SaveSlot> = SaveSlot::all().filter(|slot| save_exists(*slot)).collect();
    occupied
        .iter()
        .filter_map(|slot| peek_slot(*slot).map(|meta| (*slot, meta.saved_at)))
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(slot, _)| slot)
        .or_else(|| occupied.first().copied())
}

/// Read a save written before slots went through the toolkit.
///
/// Native builds wrote a *relative* `save_slot_N.json`, so this looks where the
/// process is running rather than in app data — it finds the file only if the
/// game is launched from the same directory as before, which is the same
/// fragility that motivated the move. The browser key happens to be unchanged
/// (`keys::storage_key` and `slots::storage_key` both produce
/// `dungeon_manager_save_slot_N`), so only the wrapper shape differs there.
///
/// Nothing writes the legacy shape any more: the next save lands in the new
/// location, which is the migration.
fn load_legacy(slot: SaveSlot) -> Result<GameState, String> {
    let key = format!("save_{}", slot.key());

    #[cfg(target_arch = "wasm32")]
    {
        // Two browser shapes, both old. The game-qualified key is what the
        // previous build wrote; the bare key is what the build before *that*
        // wrote, back when localStorage was unqualified and every macroquad game
        // on the host shared a keyspace. The toolkit's slot loader already tries
        // both keys, but parses them as the current wrapper — so a save in
        // either place with the old shape reaches here, and dropping this branch
        // would quietly orphan a browser player's game.
        macroquad_toolkit::persistence::load_json_key::<LegacySave>(GAME_NAME, &key)
            .map(|legacy| legacy.game_state)
            .or_else(|qualified| {
                let raw = crate::state::wasm_storage::storage_get(&key)
                    .ok_or_else(|| format!("no unqualified browser save either ({qualified})"))?;
                serde_json::from_str::<LegacySave>(&raw)
                    .map(|legacy| legacy.game_state)
                    .map_err(|e| format!("unqualified browser save is unreadable: {e}"))
            })
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let legacy: LegacySave = macroquad_toolkit::persistence::load_json(format!("{key}.json"))
            .map_err(|e| e.to_string())?;
        Ok(legacy.game_state)
    }
}

fn legacy_exists(slot: SaveSlot) -> bool {
    let key = format!("save_{}", slot.key());

    #[cfg(target_arch = "wasm32")]
    {
        macroquad_toolkit::persistence::json_key_exists(GAME_NAME, &key)
            || crate::state::wasm_storage::storage_exists(&key)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        macroquad_toolkit::persistence::file_exists(format!("{key}.json"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::GameData;

    #[test]
    fn campaign_progress_survives_save_serialization() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_campaign_start(&game_data, "deep_dominion");
        let progress = state
            .campaign_progress
            .as_mut()
            .expect("campaign progress should exist");
        progress
            .completed_missions
            .insert("dark_beginnings".to_string());

        let file = SaveFile {
            meta: SaveMeta::describe(&state),
            game_state: &state,
        };
        let json = serde_json::to_string(&file).expect("save should serialize");
        let loaded: LoadFile = serde_json::from_str(&json).expect("save should deserialize");

        let loaded_progress = loaded
            .game_state
            .campaign_progress
            .expect("campaign progress should load");
        assert_eq!(loaded_progress.campaign_id, "deep_dominion");
        assert!(loaded_progress
            .completed_missions
            .contains("dark_beginnings"));
    }

    /// A slot outside the range cannot be built, so no call site can invent a
    /// filename the picker will never list.
    #[test]
    fn only_real_slots_exist() {
        assert!(SaveSlot::new(0).is_none());
        assert!(SaveSlot::new(SLOT_COUNT + 1).is_none());
        assert_eq!(SaveSlot::all().count(), SLOT_COUNT as usize);
        for slot in SaveSlot::all() {
            assert!(SaveSlot::new(slot.number()).is_some());
        }
    }

    /// Each slot must file under its own name. The whole point of the change is
    /// that a second save stops overwriting the first.
    #[test]
    fn every_slot_has_its_own_key() {
        let keys: std::collections::HashSet<String> =
            SaveSlot::all().map(|slot| slot.key()).collect();
        assert_eq!(
            keys.len(),
            SLOT_COUNT as usize,
            "slots share a key: {keys:?}"
        );
    }

    /// The default is slot 1 — the slot the old single-save build used — so a
    /// player upgrading lands on their existing save rather than an empty one.
    #[test]
    fn the_default_slot_is_the_one_the_old_build_wrote() {
        assert_eq!(SaveSlot::default().key(), "slot_1");
    }

    /// The header has to describe a save well enough to tell two apart without
    /// loading either, and it has to survive the round trip.
    #[test]
    fn the_header_describes_the_save_without_loading_it() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_for_scenario(&game_data, "the_iron_siege");
        state.time_elapsed = 421.5;
        state.hero_base.current_wave_number = 3;

        let json = serde_json::to_string(&SaveFile {
            meta: SaveMeta::describe(&state),
            game_state: &state,
        })
        .expect("save should serialize");

        let peeked: MetaOnly = serde_json::from_str(&json).expect("header should read alone");
        assert_eq!(peeked.meta.scenario_id, "the_iron_siege");
        assert_eq!(peeked.meta.wave, 3);
        assert!((peeked.meta.in_game_seconds - 421.5).abs() < 0.01);
        assert!(
            peeked.meta.saved_at > 0.0,
            "saved_at should be a real clock reading, got {}",
            peeked.meta.saved_at
        );
    }

    /// A save the old build wrote has no `meta`. It must still load — that is
    /// the migration — so the legacy shape is checked against the current one.
    #[test]
    fn a_headerless_save_still_loads() {
        let game_data = GameData::load().expect("game data should load");
        let state = GameState::new_for_scenario(&game_data, "the_iron_siege");

        // Exactly what the pre-slot build wrote: game_state, and no meta.
        let legacy_json = serde_json::json!({
            "game_state": &state,
            "save_date": "Unknown Date",
            "version": "0.1.0",
        })
        .to_string();

        let legacy: LegacySave =
            serde_json::from_str(&legacy_json).expect("legacy save should still deserialize");
        assert_eq!(legacy.game_state.time_elapsed, state.time_elapsed);
    }
}
