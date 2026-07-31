# TODO — Deep Dominion

## Engine & simulation

- Wire the 4 non-proc monster abilities (`charge`, `smash`, `berserk`, `charm`) — they need bonus-damage / self-buff / morale hooks the engine lacks.
- Wire the `tile_transform` and `polymorph` spell effect types; `corrupt_land`, `make_earth` and `chickenify` currently fall through the dispatcher and do nothing.
- 5 hero abilities (dispel, purify, backstab, teleport, mass_cleanse) stay inert until ritual-detection, stealth and trap-state subsystems exist.
- Gem seams pay a flat 25 gold; the GDD specifies gems as the infinite-but-slower gold source.
- Creature social dynamics: same-faction hostility is hard-off, so there is no infighting, species rivalry or brawl-breaking.
- Room efficiency mechanics: adjacency bonuses, door placement, shape penalties.
- Rival keeper economy: digs and builds instantly and spawns free reinforcements; no traps, spells or research. Also support multiple simultaneous rivals.
- Trap ammunition/reload supply chain (imps rearm traps), magical door locking, alarm traps that summon defenders.
- Reinforced wall construction (in the GDD, absent from tiles/data).
- Environmental hazards as gameplay: lava/water damage and movement effects.
- Fog-of-war scouting gameplay; the tile field exists but no mechanics use it.
- Investigate the residual visibility leak that let a player see the rival keeper's lair.
- Hand interactions for gold and objects, not only creatures.
- Formation system and ranged combat improvements.
- A conversion-count trigger, so "convert N heroes" style `custom` objectives become winnable win conditions.
- Generalize the inert `happiness_modifier` room effect — one hook unlocks the whole amenity room tier.
- Map generator: quality metrics with regenerate-on-poor-quality, and chunked generation for large maps.
- Scope decisions to take (build or cut): possession mode, overworld/surface raiding, meta-progression (faction runs, cross-mission unlocks), creature mutations, trade/hiring economy.

## Content

- Build the 5 remaining 1.0 rooms from `docs/ROOM_SET.md`. Vault and Mana Well have shipped (art + `rooms.json` + gating tech + mission availability, no Rust changes). The rest each need an engine hook as well as art: **Combat Pit** (a second XP-producing `task_type`), **Arcane Archive** (generalize the library's `room_type == "library"` research key into a task family), **Soul Furnace** (widen `mana_generation_per_second` past its temple/ritual hardcode, plus a death hook), **Gatehouse** (room-scale door/wall with HP), **Leisure Den** (blocked on the inert `happiness_modifier` below).
- Author the remaining `docs/monsters.md` roster (Lich, Balor, Ogre, Shadow Stalker, …). No longer hard-blocked on sprites — un-arted entries render as a placeholder checker, so stats can be authored and playtested first — but each still wants a `graphics_gen/monsters/` generator before it ships.
- Wire the finished-but-unreachable art: three traps (`fire_trap`, `gas_trap`, `lightning_trap`) need balance entries in `traps.json`, and three hero buildings (`blacksmith`, `guard_tower`, `tavern`) need spawn/destruction rules in `hero_buildings.json`. The allowlist in `tests/asset_manifest_tests.rs` names them.
- Rooms declare a `visual.wall_sprite` (`tiles/lair_wall.png`, …) that no generator emits and nothing reads. Either generate per-room wall art and render it, or drop the field.
- Spell depth that needs engine hooks: miscasts, hero counter-spells, research-unlocked spell modifiers.
- Decide the fate of the mod/content-pack system: `mods/load_order.json` ships empty — market it (docs, examples, validation) or cut it.
- Wire campaign missions to grant *technologies* rather than raw unlocks, so research gates progression.

## Audio

- Audio engine layer with WASM support, built into `macroquad-toolkit` so every game benefits.
- SFX set (~60–100 sounds): digging, claiming, room build, gold pickup/deposit, combat, spells, traps, creature moods, UI, alerts.
- Music: main theme, ambient dungeon layers, raid/combat transitions, victory and defeat cues.
- Mixing, ducking, camera-relative panning, master/music/SFX volume settings with persistence.
- Sourcing decision: commission, license or produce — budget and pipeline.

## Visuals & game feel

- Animation: every sprite is a single static 64×64 frame. Pick a tier (walk/attack/death frames, or procedural bob/squash/flip) and budget it.
- Particles and feedback: hit flashes, damage numbers, death poofs, spell VFX, trap triggers, dig debris, gold sparkle, heart-damage screen shake — none exist.
- Lighting and atmosphere pass; the GDD's "warped evil" theming.
- UI art overhaul — playtest feedback calls the UI dated and Rust-default. Includes real entity rendering, sidebar animation and the minimap viewport.
- Window icon and title/menu art beyond the single `main_menu_bg.png`.

## UI & UX

- Settings menu breadth: resolution and window modes, volume sliders, keybind remapping (keys are hardcoded in `engine/input.rs`), camera/scroll options, autosave toggle.
- Save system: every call site hardcodes `"slot_1"`. Needs a multi-slot UI, autosave, quicksave/quickload, save metadata, and format versioning/migration.
- Tutorial: 6 steps cover only dig→rooms→recruit. Extend to combat, traps, spells, research, prison/torture, temple, wages/moods/slapping, hero waves and the minimap, plus contextual hints and an intro.
- Hotkey overlay / help screen.
- Localization: no i18n layer, all strings hardcoded English — externalize strings before it gets expensive.
- Accessibility: colorblind-safe faction palettes, text scaling beyond 3 steps, hold-vs-toggle options, screen-shake toggle.
- Decide on gamepad support (probably cut, but Steam Deck verification wants basic navigation).

## Balance

- Hero/creature level asymmetry: heroes gain +15%/level to cap 10, creatures +10% to cap 5 — the late game mathematically favours heroes.
- Wave pacing: players are killed before they can set up, while the balance numbers claim a ~33-minute first wave. Reconcile config against the shipped build and scale pacing by difficulty.
- Starting gold scarcity near the dungeon core — the loudest playtest complaint.
- Room sell refund is 5%; 25–50% would make experimentation viable.
- Creature value outliers: Bile Demon overpriced, Hellhound free, Succubus mood loop, treasury desirability clustering.
- Creature wage and need decay rates still untuned.
- Structured playtest program: wave-1 survivability, sustainable army size, wave-10+ viability, per-mission tuning.
- Fold the `balance_calculator` simulations into `cargo test`/CI — its assertions are hand-rolled bools with no `#[test]`s.

## Code quality

- Strip debug output: ~98 untagged `eprintln!` calls remain across combat, imp AI, spells, traps and save/load.
- Error-handling hardening: ~25 `unwrap()`, 9 `expect()`, 4 `panic!` — asset loading and save handling especially.
- Deduplicate movement and distance logic between `creature_ai` and `imp_ai` (two `manhattan_distance` impls).
- Test coverage: combat math has one test, save/load one, UI/actions none. Add tests for menu action handlers, sidebar selection, tooltip state and dungeon command dispatch.
- Scenario fixtures for room placement, resource flow, encounters and progression milestones.
- Separate model mutation from the renderer and sidebar modules so UI actions call explicit domain commands.
- Performance on large maps: O(n) entity-position scans need a spatial index; cache pathfinding and room detection; profile the sidebar and renderer paths that recompute layout every frame.
- Data-driven stragglers: hardcoded imp claim delay, loose spawn-placement validation.
- Decide on the GDD's determinism/replay aspiration while it is still cheap.

## Release

- Versioning and builds: still v0.1.0; pin or vendor `macroquad-toolkit` for reproducible releases; add a native `[profile.release]` (the workspace default is tuned for WASM size).
- Storefront: no Steam or itch integration. Needs steamworks bindings (achievements, cloud saves, rich presence), depot/build scripts, store page, capsule art, trailer, screenshots.
- Demo build for Steam Next Fest — needs 2–3 polished missions.
- Windows installer/zip, a tested Linux build, opt-in crash reporting, and a patch strategy compatible with save migration.
- Legal: trademark check on "Deep Dominion", dependency license audit, asset provenance, EULA/privacy if telemetry ships.
- Marketing runway: press kit, devlog cadence, wishlist campaign.
- Fix `docs/gdd.md`'s stale "Bevy ECS" claim.
