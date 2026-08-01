# TODO — Deep Dominion

## Engine & simulation

- Wire the 4 non-proc monster abilities (`charge`, `smash`, `berserk`, `charm`) — they need bonus-damage / self-buff / morale hooks the engine lacks.
- Wire the `tile_transform` and `polymorph` spell effect types; `corrupt_land`, `make_earth` and `chickenify` currently fall through the dispatcher and do nothing.
- 5 hero abilities (dispel, purify, backstab, teleport, mass_cleanse) stay inert until ritual-detection, stealth and trap-state subsystems exist.
- Gem seams now pay their authored `resources.mine_value`, and a test pins the GDD property: a seam pays less per dig than a vein and is never consumed. The remaining question is **tuning**, not plumbing — 25 vs a vein's 100 is a 4x gap, which may make seams not worth an imp's time. Wants the playtest program.
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
- Map generator: quality metrics with regenerate-on-poor-quality, and chunked generation for large maps.
- Scope decisions to take (build or cut): possession mode, overworld/surface raiding, meta-progression (faction runs, cross-mission unlocks), creature mutations, trade/hiring economy.

## Content

- `docs/ROOM_SET.md`'s commit set is **built** — all 7 (Vault, Mana Well, Leisure Den, Arcane Archive, Combat Pit, Soul Furnace, Gatehouse) have art, a `rooms.json` entry, a gating tech and mission availability. Remaining from that doc: the Gatehouse ships as a defensive *bonus* room, not the "gates auto-close under threat" version — that wants a threat-detection subsystem it shares with alarm traps and the fog-of-war item. Decide whether to build it or restate the room as it now is.
- `tests/live_data_fields_tests.rs` sweeps **every** `pub struct` under `src/data/` for fields that are parsed and read nowhere, so a new struct cannot arrive with dead fields unnoticed. The first full sweep found **48**, now listed in `UNCONSUMED` with reasons. That list is the honest inventory of authored-but-inert content data; the biggest clusters are worth their own items:
  - **Room build time** — `construction_time` is still ignored, so rooms appear the instant they are paid for rather than being raised. Note before building this: the authored values are 0.4–0.9s, so wiring them as written buys a sub-second delay in exchange for a save-format change (per-tile progress) and renderer work to show a part-built room. Either re-scope the numbers to something a player would notice, or drop the field. The rest of the build rules (`allowed_terrain`, `requires_claimed`, `can_overlap`, `dig_required`, `max_instances`, `forbidden_if`) now go through `room_validator::tile_permits_room` / `dungeon_permits_room`.
  - **The rest of the hero behaviour model** — `trap_awareness`, `fear_resistance`, `will_fight_to_death` and `bravery` are wired. Still inert: `door_break_chance` (heroes never attack doors — they path around them, so there is nothing to roll against), `light_preference` (needs the lighting pass to exist), `call_for_aid` (needs a hero rally mechanic; the natural shape is a threatened hero pulling nearby allies onto their attacker).
- Per-dig mining yields are now read from each tile's `resources.mine_value` in `tiles.json`, with the `imp_behavior.*_reward` config constants as fallback for tiles that declare none. All three resource tiles author their own, and a test requires that — otherwise a value silently reverts to the global constant, which is how the gem seam's 25 came to exist in two files at once.
- **A bug class the field guard cannot see** — fields that are *displayed* but not *enforced* look alive to any reference-counting check. Two found and fixed: `global_rooms_required` (the build button promised "Requires: Library" while the engine let you build without one) and `requirements.research` (a parallel tech list that drifted from the real gate in 7 of 18 rooms, and is now deleted in favour of deriving from `technologies.json`). The UI renders no other unverified promise — `LOCKED` reads `is_room_unlocked`, which is the real gate. The general lesson worth carrying: **any datum duplicated between two files will drift, and the copy the engine does not read is the one that lies.**
- Hero-building destruction effects are live: razing spawn buildings permanently slows the garrison, the stables cut hero speed, the armory and forge blunt every hero, and the town hall's `win_game` ends the map. Two follow-ups: the penalties are **global**, not per-building-instance, so two barracks each grant the full 50% when razed; and nothing surfaces them in the UI, so the player has no way to know the armory mattered.
- Hero nerve is live: `effective_retreat_threshold` folds `bravery` and `fear_resistance` into the authored `retreat_below_health`, and `will_fight_to_death` removes it. Worth a playtest pass — the champion of light is authored with both `will_fight_to_death` and a 0.0 baseline, so late heroes now genuinely never break, which may be too strong against a player with no crowd control.
- The Soul Furnace burns *hero* corpses only, matching the graveyard. Dead creatures still vanish with nothing to show for them — decide whether your own fallen should be renderable, which is a tone question as much as a balance one.
- The temple is available in only 1 of 13 missions (`pacts_and_sacrifice`), so its `mana_generation_per_second` and the whole prayer loop are nearly unreachable in the campaign. Either widen its availability or accept that the ritual circle is the real mana room.
- `execute_sleep`, `execute_eat` and `execute_deposit_gold` still test `room_type ==` a specific room. These are *not* the same edit `research` and `train` were — each needs a data decision first, so don't generalize them mechanically:
  - **sleep** — the `sleep` family is `lair` *and* `kennel`, so widening it lets creatures sleep in kennels. Probably right (a kennel is where beasts sleep), but it interacts with `count_available_lair_tiles`, which sets the creature cap. Decide the cap question first. Until then the kennel's authored `sleep_recovery_rate: 1.2` stays unreachable — the lair's 1.0 is the only one the engine can see.
  - **eat** — the hatchery is `task_type: "work"`, so there is no `eat` family to match on. Needs a `task_type` reassignment, which changes what `execute_work` sees.
  - **deposit** — the treasury is `task_type: "none"`, shared with graveyard, vault, mana_well and leisure_den. Keying on it would make the graveyard a treasury. Needs its own task type.
- The rest of the amenity tier (Mentor's Den, Doctrine Chamber) is pure data now that `happiness_modifier` is live — but `docs/rooms.md`, which `docs/ROOM_SET.md` cites as their design source, **does not exist**. Either write the designs or drop the rooms; there is nothing to build from.
- Creature AI has no *need* that an amenity room satisfies, so creatures only reach the Leisure Den by wandering into it. A `comfort` need in `monsters.json` with `satisfied_by: ["leisure_den"]` would let them seek it out deliberately, the way they already seek food and sleep.
- Author the remaining `docs/monsters.md` roster (Lich, Balor, Ogre, Shadow Stalker, …). No longer hard-blocked on sprites — un-arted entries render as a placeholder checker, so stats can be authored and playtested first — but each still wants a `graphics_gen/monsters/` generator before it ships.
- All generated art is now reachable from data except the casino tile, kept only until the Leisure Den's supersession of it (docs/ROOM_SET.md) is confirmed — at which point delete `create_casino` too. `AWAITING_DATA` in `tests/asset_manifest_tests.rs` is down to that one name.
- Rooms declare a `visual.wall_sprite` (`tiles/lair_wall.png`, …) that no generator emits and nothing reads. Deciding this needs a renderer answer, not a data one: walls are `solid_rock`/`earth` tiles, so drawing a room's own wall art means a per-wall-tile adjacency lookup every frame — which collides with the O(n) scan problem already flagged under Code quality. Treat it as part of the lighting/atmosphere pass, or drop the field.
- Spell targeting is enforced: `valid_targets` refuses a creature of the wrong allegiance (you could heal an invading knight), `requires_visibility` refuses a fogged target, and every refusal now reaches the player as a notification instead of an `eprintln!`. Two deliberate limits — `valid_targets` is enforced only for `creature`-targeted spells, since refusing to place a fireball on an empty tile would stop you catching someone standing beside it; and the fog check mirrors the renderer's condition, so it does nothing while fog is disabled.
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
- Title/menu art beyond the single `main_menu_bg.png`. (The window icon ships — `graphics_gen/icon.rs` draws the dungeon heart at 16/32/64 and `main.rs` embeds it.)

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
- Structured playtest program: wave-1 survivability, sustainable army size, wave-10+ viability, per-mission tuning. The `wave` capture scene makes the first two observable without a ten-minute sit — a first run showed wave 1 (2 heroes) reaching the dungeon heart and taking it to 988/1000 within ~15s of launching. Treat that as a starting point rather than a verdict: the scene seeds dig orders, which opens paths a real wave-1 dungeon would not have.
- Fold the `balance_calculator` simulations into `cargo test`/CI — its assertions are hand-rolled bools with no `#[test]`s.

## Code quality

- `PlayerState::warn_once(key, message)` is the pattern for any condition checked every tick that the player still needs telling about once — used by the treasury overflow and all four spawn-blocked reasons. Reach for it rather than a bespoke bool.
- Strip debug output. The swallowed-player-feedback subset is done: spell-cast failures, room build refusals (unresearched / not enough gold / not enough mana), trap refusals (unresearched / no Workshop / no crates) and treasury overflow all reach the player now. Save/load failures turned out to already notify — the `eprintln!` beside them is a genuine diagnostic. The spawner's four blocked reasons are done too. The per-tick tracing — 31 calls across combat, traps, imps, tasks, creatures, spells and hero waves — is now behind `trace_log!(tag, ..)`, silent unless `DUNGEON_MANAGER_LOG` names the tag. What remains unconditional is one-shot startup diagnostics (asset/font/data loading, map load, save/load) and four genuine warnings, which is roughly what should stay.
- The screenshot harness has `simulation` (intro dismissed, dig orders seeded — the game actually running) and `wave` (the same with the first hero wave pulled forward from 600s to 2s, so combat is reachable at all). `gameplay` still captures the briefing overlay, which freezes the simulation by design. Note the shared toolkit capture script does not forward extra env vars, so `DUNGEON_MANAGER_LOG` tracing needs the exe driven directly rather than through `scripts/capture_ui.ps1`.
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
