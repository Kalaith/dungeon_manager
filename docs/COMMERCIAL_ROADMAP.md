# Deep Dominion — Commercial Readiness Roadmap

*Generated 2026-07-13 from a full code + docs + assets audit. Supersedes the status claims in
`ROADMAP.md` and `docs/FEATURE_GAP_ANALYSIS.md`, which contradict each other and the code
(both are stale — e.g. the gap analysis says Boulder/Alarm/Lightning traps and the research UI
are missing; the code has 7 traps and prison/torture/temple/graveyard all fully implemented).*

## Where the project actually stands

~26.8k LOC. The **simulation engine is a mature vertical slice**: creature/imp AI (needs, moods,
desertion, slap discipline, task scoring), hero AI with goals/threat/retreat/parties/waves/digging,
a real rival-keeper opponent AI, prison→skeleton conversion, torture→defection, temple sacrifice,
graveyard→vampires, scavenger, barracks, projectile combat with line-of-sight and resistances,
scenario event/trigger system, campaign framework with persistent unlocks, save/load (native+WASM),
tutorial framework, minimap, tooltips, fog of war. 68 unit tests.

What makes it the **longest-runway project in the workspace** (standing.md: 10–14 months) is
everything *around* that engine:

- **Content: one campaign, one mission, one 32×32 map.** Loaders `include_str!` single files.
- **Audio: literally zero** — no code, no assets, no dependency.
- **Presentation: static single-frame 64×64 billboards** — no animation, particles, shake, lighting.
- **Authored-but-inert data**: monster/hero abilities, traits, and combat status effects are parsed
  but never executed.
- **UI breadth**: 2-option settings menu, single hardcoded save slot, no keybind remap, no i18n,
  no level select, 6-step tutorial covering only the opening loop.
- **Zero release engineering**: v0.1.0, path dependency, cheats (incl. God Mode) reachable via F1
  in release builds, no store integration, no icon, no installer.

---

## 1. Content production (largest single cost)

### Campaign & missions
- [ ] Design a full campaign arc (industry norm for the genre: 15–20 missions; minimum viable ~10)
      with a difficulty curve, mechanic-introduction order, and narrative framing/briefings.
- [ ] Author the missions: one map + one scenario JSON + event scripting each. Only
      `dark_beginnings` exists. The event system (triggers: TimeElapsed, ObjectiveComplete,
      RoomClaimed, ActionPointReached, DungeonBreached; actions: unlocks, spawns, rules) is done —
      this is pure content work.
- [ ] **Un-hardcode content loading**: `data/campaign.rs:128` and `data/scenario.rs:350`
      `include_str!` exactly one campaign/scenario file. Build a manifest-driven loader so missions
      can be added without code changes (native dir scan + WASM embedded manifest).
- [ ] Exercise the unlock graph: `required_completed` / `unlocks_after` logic exists but has never
      branched (single linear mission). Add branching or hub structure and test it.
- [ ] Between-mission screens: mission briefing/debriefing, campaign map / mission-select UI
      (currently START GAME jumps straight into the one mission; victory shows "NEXT MISSION" that
      never fires because there is no next mission).
- [ ] Boss heroes / final-mission climax encounters (designed in hero_notes/ROADMAP, not built).
- [ ] Skirmish/sandbox mode: `MapType::Rich/Hazardous/Test` and the whole procedural map generator
      exist but are **unreachable from the UI** (`input.rs:93` forces Standard). Add a skirmish
      setup screen (map size/type/seed, rival count, difficulty) — cheap win, the generator with
      biomes/connectivity/resource placement is already built and this is the genre's replayability
      pillar.
- [ ] Difficulty levels (none exist — no easy/normal/hard anywhere).

### Roster & data content
- [ ] More monsters: 13 shipped. `docs/monsters.md` designs ~18 more (Lich, Balor, Ogre, Shadow
      Stalker, etc.). Four finished sprites are already orphaned with no data entry:
      **zombie, ghost, bat_swarm, dark_elf** — wiring these is near-free.
- [ ] More heroes: 17 shipped; orphan sprites **champion, dragon_knight, peasant** ready to wire.
- [ ] Tech tree: only **4 technologies** exist. A commercial progression system needs a real tree
      (rooms/spells/traps/creatures as unlocks across a campaign). Also fix persistence:
      `player_state.rs:240` — unlocked techs are inferred, not stored.
- [ ] Rooms: 15 built; `special_rooms.json` has a single entry. `docs/rooms.md` designs a large
      future set (Gatehouse, Soul Furnace, Summoning Vault, Legacy Vault…). Pick the commercial set.
- [ ] Spells: 11 built; design docs want more plus miscasts, hero counter-spells,
      research-unlocked modifiers.
- [ ] Mod/content-pack system exists (`mods/load_order.json`) but ships **empty** — either make
      modding a marketed feature (docs, examples, validation) or cut it.

## 2. Half-built gameplay systems (engine work)

### Inert authored data — the biggest correctness gap
- [ ] **Status effects**: `combat.rs:222` `generate_status_effects()` returns an empty Vec by
      design ("no status effects for now"). The struct + duration ticking exist; melee/ranged
      combat never applies poison/burn/freeze/stun. Wire abilities → effects.
- [ ] **Monster abilities**: `monsters.rs:59` deserializes `combat.abilities` as raw
      `serde_json::Value`, never read by the engine. Only 6/13 monsters even have abilities
      authored. Define the ability schema, execute in combat, author for the full roster.
- [ ] **Hero abilities**: `HeroAbilityData` parsed (`heroes.rs:15,57`), zero engine references.
- [ ] **Traits**: parsed `Vec<String>`, no trait logic anywhere.

### Known-broken / dead branches
- [ ] `hero_ai.rs:43` — `threat_level` hardcoded to `Moderate` in `decide_hero_goal`
      ("Placeholder"); the real `evaluate_threat` is never consulted for goal selection.
- [ ] `spell_effects.rs:269` — heal spell **logs but never heals heroes** (creatures/structures OK).
- [ ] `spell_effects.rs:398` — `spawn_entity_effect` silently ignores every entity id except "imp".
- [ ] `spell_effects.rs:288` — stat modifiers support only "speed" and are **permanent** (no
      duration/revert).
- [ ] `task_system.rs:125` — wage-collection branch is an empty `if` (no theft/unrest consequence
      for unpaid creatures, which the design docs treat as core).
- [x] Mana economy config bug: `game_config.json` max mana capacity 0 vs starting mana 10,000
      (flagged in BALANCE_TESTING.md). Already fixed in code (100/500) and covered by
      `test_mana_capacity_not_zero`.
- [ ] Gem seams give flat 25 gold; GDD + feedback both specify gems as the
      infinite-but-slower gold source.
- [x] Fog/visibility leak default: `cheat_fog_enabled` **defaulted to true** (`game_state.rs:218`),
      disabling fog-of-war for every new game. Now defaults to `false`. (Player feedback about the
      enemy keeper's lair being visible may still need separate investigation.)

### Depth features (designed, not started)
- [ ] Creature social dynamics: anger currently only penalizes mood — no infighting
      (`combat.rs:619` same-faction hostility hard-off), no species rivalries, no brawl-breaking
      gameplay. A signature genre feature.
- [ ] Room efficiency mechanics: adjacency bonuses, door placement, shape penalties (top item in
      docs/feedback.md).
- [ ] Rival keeper economy: currently digs instantly, places rooms instantly, spawns reinforcements
      for free; no traps/spells/research use. Fine for one mission, too shallow for a campaign
      antagonist. Also: support multiple simultaneous rivals.
- [ ] Trap ammunition/reload supply chain (imps rearm traps); magical door locking;
      alarm traps summoning defenders rather than just alerting.
- [ ] Reinforced wall construction (in GDD, not in tiles/data).
- [ ] Environmental hazards as gameplay: lava/water damage & movement effects.
- [ ] Fog-of-war scouting gameplay (field exists; no scouting mechanics).
- [ ] Hand interactions for objects/gold, not just creatures.
- [ ] Formation system / ranged combat improvements (roadmap).
- [ ] **Scope decisions needed** (design fork, then build or cut): possession mode; overworld/surface
      raiding (Dungeons-3 style — huge); meta-progression (faction-restricted runs, cross-mission
      unlocks, Legacy Vault); creature mutations/evolutions; trade/hiring economy.

## 3. Audio (0% — full greenfield)

- [ ] Audio engine layer (macroquad audio or kira — per workspace rules, build it into
      `macroquad-toolkit` so all games benefit), with WASM support.
- [ ] SFX set (~60–100 sounds for this genre): digging/claiming, room build, gold pickup/deposit,
      combat hits per attack type, projectiles, each spell, each trap, creature vocals
      (attract/slap/pickup/drop/death per species is genre-standard), hero alerts, heart heartbeat
      + damage, UI clicks, notification stingers, victory/defeat.
- [ ] Music: main theme, ambient dungeon layers, raid/combat transitions, victory/defeat cues.
      The GDD calls for "industrial sound design" — sound is a stated pillar, currently absent.
- [ ] Mixing, ducking, positional/pan by camera, volume settings (master/music/SFX) + persistence.
- [ ] Sourcing: commission/license/produce all of the above — budget and pipeline decision.

## 4. Visual polish & game feel

- [ ] **Animation**: every sprite is one static 64×64 frame. Minimum commercial bar: walk/attack/
      death frames or procedural motion (bob/squash/flip). `docs/3d_update.md` sketches the
      ambitious path (articulated limbs, per-face lit isometric blocks, AO, rim light) — pick a
      tier and budget it; this plus audio is what makes the game "stream-worthy" per feedback.md.
- [ ] Particles & feedback: hit flashes, damage numbers, death poofs, spell VFX, trap triggers,
      dig debris, gold sparkle, heart damage screen shake. Zero exist today (verified by grep).
- [ ] Lighting/atmosphere pass; "warped evil" theming from the GDD.
- [ ] UI art overhaul — player feedback explicitly calls the UI "dated / Rust-default".
      Includes proper entity render (renderer.rs:503 "full square for now"), sidebar animation
      (sidebar.rs:100 "snap for now"), minimap viewport (minimap.rs:118).
- [ ] Window icon; title-screen/menu art beyond the single `main_menu_bg.png`.
- [ ] 7 orphan sprites wired (see content), missing-texture placeholder handling (resources.rs:127).

## 5. UI / UX breadth

- [ ] **Settings menu** is fullscreen + 3-step UI scale only. Needs: resolution & window modes,
      volume sliders (once audio exists), keybind remapping (all keys hardcoded in
      `engine/input.rs`), camera/scroll options, autosave toggle.
- [ ] **Save system**: backend supports named slots but every call site hardcodes `"slot_1"`
      (`action_processor.rs:67`, `input.rs:227/240`, `menus.rs:76/152`). Needs multiple slots UI,
      autosave, quicksave/quickload, save metadata (mission, playtime, timestamp, thumbnail), and
      **save-format versioning/migration** so post-release patches don't corrupt saves.
- [ ] **Tutorial**: 6 steps covering only dig→rooms→recruit. Must cover combat, traps, spells,
      research, prison/torture, temple, wages/moods/slapping, hero waves, minimap — plus
      contextual hint system for later mechanics. Player feedback asked for intro/story.
- [ ] Hotkey overlay / help screen (roadmap item, unchecked; F1 currently toggles cheats).
- [ ] **Localization**: no i18n layer; all strings hardcoded English. Retro-fitting later is
      expensive — externalize strings early. Commercial norm: EFIGS + zh-CN minimum for this genre.
- [ ] Accessibility: colorblind-safe palettes (faction colors!), scalable text beyond 3 steps,
      hold-vs-toggle options, screen-shake toggle.
- [ ] Gamepad/controller: none. Decide (PC-only mouse genre — probably cut, but Steam Deck
      verification wants at least basic navigation).
- [x] **Gate the cheat menu**: God Mode/gold/spawner reachable via F1 in release builds
      (`sidebar.rs:73` — no `cfg(debug_assertions)` anywhere). F1 toggle now compiled out via
      `#[cfg(debug_assertions)]`; `cheats_visible` already defaulted false so the tab/menu stay
      unreachable in release builds.

## 6. Balance & playtesting

Documented, unresolved (BALANCE_TESTING.md + feedback.md):
- [ ] Hero vs creature level asymmetry (heroes: +15%/lvl to cap 10; creatures: +10%/lvl to cap 5)
      — late game mathematically favors heroes.
- [ ] Wave pacing: players report being killed before setup; balance doc claims a ~33-min first
      wave. Reconcile config vs shipped build, add difficulty-scaled pacing.
- [ ] Starting gold scarcity — feedback calls more gold near the start "the single most important
      milestone".
- [ ] Room sell refund 5% ("extremely punishing"; suggest 25–50%).
- [ ] Creature value outliers: Bile Demon overpriced, Hellhound free, Succubus mood loop,
      treasury desirability clustering.
- [ ] Structured playtest program: wave-1 survivability, sustainable army size, wave-10+ viability,
      per-mission tuning once the campaign exists.
- [ ] Fold the `balance_calculator` simulations into `cargo test`/CI (its assertions are
      hand-rolled `passed: bool`, zero `#[test]`s) so balance regressions are caught automatically.

## 7. Technical debt & quality

- [ ] File-size hard-limit violations (>800 lines): `sidebar_renderer.rs` (~1218),
      `game_state.rs` (~1113), `balance_calculator/analysis.rs` (~1211), `creature_ai.rs` (~939),
      `renderer.rs` (~846). Workspace rules require restructuring these when touched.
- [ ] Strip debug output: `eprintln!("[DEBUG] …")` throughout hero AI, combat, imp AI, spells,
      traps — hot-path console spam in release.
- [ ] Error-handling hardening: ~25 `unwrap()`, 9 `expect()`, 4 `panic!` (per code review) —
      crash-proof asset loading and save handling especially.
- [ ] Deduplicate movement/distance logic (creature_ai vs imp_ai; two `manhattan_distance` impls)
      per `notes.md` review.
- [ ] Test coverage gaps: combat math has **one** test; save/load robustness one; UI/actions none
      (README lists these explicitly). The 68 existing tests cluster on AI/scenario/state.
- [ ] Performance for large maps: O(n) entity-position scans (`game_state.rs:387` "slow but safe
      for now" — needs a spatial index), pathfinding/room-detection caching (code review),
      profiling of sidebar/renderer per README. Only a 32×32 map exists today; campaign maps will
      be larger.
- [ ] Data-driven stragglers: hardcoded imp claim delay (`imp_ai.rs:692`), loose spawn placement
      validation (`input.rs:702`).
- [ ] Decide on the GDD's determinism/replay aspiration (fixed timestep exists; RNG discipline
      partially via toolkit) — cheap now, impossible later.

## 8. Release engineering & commercialization

- [ ] Versioning & builds: v0.1.0; `macroquad-toolkit` is a local path dependency (fine for the
      monorepo, but pin/vendor for reproducible release builds); add a tuned `[profile.release]`
      (workspace default is z/lto for WASM — a native Steam build likely wants opt-level 3).
- [ ] **Storefront**: no Steam/itch integration of any kind. Steam release needs: steamworks
      bindings (achievements, cloud saves, rich presence), depot/build scripts, store page,
      capsule art set, trailer, screenshots/GIFs (blocked on the polish work above), tags/deck
      verification pass. itch.io/web demo (WASM path already works) as funnel.
- [ ] Demo build (Steam Next Fest is the genre's proven funnel) — needs 2–3 polished missions.
- [ ] Windows installer/zip, Linux build (GDD targets Windows+Linux — Linux is untested),
      crash reporting (opt-in), update/patch strategy compatible with save migration.
- [ ] Legal/business: trademark check on "Deep Dominion", dependency license audit,
      asset provenance (procedural + commissioned audio/art contracts), EULA/privacy if telemetry.
- [ ] Marketing runway: press kit, devlog cadence, wishlist campaign — typically started 6+ months
      pre-launch.
- [ ] Doc hygiene: retire/merge `ROADMAP.md`, `FEATURE_GAP_ANALYSIS.md`, `REVISED_PLAN.md`
      (mutually contradictory snapshots); fix `gdd.md`'s stale "Bevy ECS" claim; CHANGELOG.md is
      empty — start keeping it.

---

## Suggested sequencing

1. **Fix the inert/broken engine pieces** (§2: status effects, abilities, hero-goal threat branch,
   heal/spawn/stat-modifier spells, wage consequences, mana cap, fog leak) — everything after
   builds on a trustworthy sim.
2. **Skirmish mode + save slots + settings + cheat gating** — cheap, uses systems that already
   exist, makes the game testable and demoable.
3. **Audio pass 1 + animation/juice pass 1** — the two zero-percent pillars; do early because they
   change how everything feels and what playtests report.
4. **Campaign production** (missions 1–10, tutorial expansion, mission select, difficulty) — the
   long-pole content grind, interleaved with balance playtesting.
5. **Localization/accessibility/UX breadth**, string externalization early in phase 4.
6. **Release engineering** (Steam, demo, marketing) — start store page + wishlists as soon as
   phase 3 makes it screenshot-worthy.

Consistent with `standing.md`'s 10–14 month estimate: roughly 2–3 months on §2+§5 engineering,
3–4 months on content production, 2–3 months on audio/visual polish (plus commissioning lead
time), 1–2 months on balance/QA, 1–2 months on release engineering — with content and art/audio
dominating.
