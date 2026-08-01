# TODO — Deep Dominion

## In scope

Everything below this section is a backlog. **These three items are the committed
work**, and the target that binds them is one player finishing one mission
start-to-finish without a developer sitting next to them. Each is scoped to the
smallest version that clears that bar — not to the full item the backlog
describes.

### 1. Save slots

`"slot_1"` is a string literal at **eleven call sites across five files**
(`engine/action_processor.rs`, `engine/input/menus.rs`, `engine/input/playing.rs`,
`ui/menus.rs`, `ui/sidebar_renderer.rs`), so there is exactly one save and every
new game silently overwrites it.

Done means: a slot is chosen once and threaded through, a save/load UI that lists
slots with enough metadata to tell them apart (mission, in-game time, wave), and
autosave. Out of scope for now: quicksave/quickload keys, and format
versioning/migration — the second matters but nothing ships that needs migrating
yet, so it stays in the backlog until the save format is otherwise stable.

The first move is to stop the literal spreading, not to build the UI: replace the
eleven sites with a single accessor so the slot has one owner. Note that
`TutorialState` is serialized into the save, so slot work and tutorial work touch
the same format.

### 2. Tutorial coverage

`STEPS` in `engine/tutorial_system.rs` is six steps — dig, claim, Lair, Hatchery,
Treasury, recruit — and stops there. A player who finishes it has never met
combat, traps, spells, research, wages, moods, the minimap, or a hero wave, which
is everything that decides whether they survive the mission they are in.

Done means: coverage through the first hero wave, so the tutorial hands off at the
point the player has something to defend and knows how to defend it. Priority
order is defence-critical first — hero waves and combat, then traps, then
wages/moods (creatures desert and the player never learns why), then research.
Spells, prison/torture and the temple can wait; they are not on the path to
surviving.

**Note before starting:** `STEPS` is a hardcoded Rust `const`, which contradicts
this project's data-driven rule. Roughly doubling its length is the wrong moment
to keep it in Rust — move it to `assets/` first, then author. That also makes step
text reachable by the localization item later.

### 3. Wave 1

The loudest playtest complaint, and the one with a concrete finding behind it:
`hero_waves.initial_delay` is **600s (10 minutes)**, and `HeroBase::new` takes it
raw. Threat is applied *only* to later waves (`hero_spawner.rs:40`) and to
garrison replenishment (`hero_spawner.rs:114`) — so **wave 1 lands at exactly
10:00 on every mission at every difficulty**, identically for the tutorial map on
Easy and the hardest mission on Hard. It is the one wave with no scaling at all.

Done means: wave 1 scales by mission and difficulty like every other wave, and its
timing is reconciled against what a player can actually build in that window.

Two corrections to what this file used to say: the old entry claimed the balance
numbers assert a **~33-minute** first wave — config says 600s, and the only
assertion in `balance_calculator` is `initial_delay >= 30.0` *seconds*, so that
figure came from somewhere else and should not be trusted. And this is not purely
a pacing dial: the starting-gold item under Balance is the same complaint from the
other side, since more setup time and faster setup are interchangeable fixes. Take
the measurement before turning either.

---

## Engine & simulation

- Wire the 4 non-proc monster abilities (`charge`, `smash`, `berserk`, `charm`) — they need bonus-damage / self-buff / morale hooks the engine lacks.
- Wire the `tile_transform` and `polymorph` spell effect types; `corrupt_land`, `make_earth` and `chickenify` currently fall through the dispatcher and do nothing.
- 5 hero abilities (dispel, purify, backstab, teleport, mass_cleanse) stay inert until ritual-detection, stealth and trap-state subsystems exist.
- Gem seam tuning: a seam pays its authored 25 per dig against a vein's 100 and is never consumed. The plumbing is done and pinned by a test; the 4x gap may make seams not worth an imp's time. Wants the playtest program.
- Creature social dynamics: same-faction hostility is hard-off, so there is no infighting, species rivalry or brawl-breaking.
- Room efficiency mechanics: adjacency bonuses, door placement, shape penalties. The Cultist's "generates power through sacrifice synergy" is the same shape — `generate_room_mana` is `tiles × efficiency × rate` and does not know which creatures are standing in the room, so no creature can contribute to a ritual circle's output.
- Rival keeper economy: digs and builds instantly and spawns free reinforcements; no traps, spells or research. Also support multiple simultaneous rivals.
- Trap ammunition/reload supply chain (imps rearm traps), magical door locking, alarm traps that summon defenders.
- Player-directed wall reinforcement. The Stone Warden's `stonebinding` produces `reinforced_wall`, but only near where you station it — there is still no way to point at a specific wall and order it reinforced.
- No creature can damage terrain, so the Balor's authored "breaks walls" has no mechanic.
- No mana upkeep anywhere: `economy.wage_per_minute` is gold-only, and the engine's `needs` keys are sleep/food/gold/training. Both the Ironbound's "drains mana instead" and the Balor's "burns mana" want this same missing feature.
- Environmental hazards as gameplay: lava/water damage and movement effects.
- Fog-of-war scouting gameplay; the tile field exists but no mechanics use it.
- Investigate the residual visibility leak that let a player see the rival keeper's lair.
- Hand interactions for gold and objects, not only creatures.
- Formation system and ranged combat improvements.
- A conversion-count trigger, so "convert N heroes" style `custom` objectives become winnable win conditions.
- Map generator: quality metrics with regenerate-on-poor-quality, and chunked generation for large maps.
- Mutation presentation: `engine/mutation.rs` works, but a mutation announces itself with a notification and nothing else — no transformation effect, no entry in any creature list, and no way for the player to see *which* rooms a creature is close to evolving through.
- `apply_combat_result` multiplies `movement_speed` only for `freeze`, while `expired_speed_multipliers` divides back out for `freeze` *and* `speed_modifier` — so a creature ability authored as `speed_modifier` would leave its victim permanently faster. A test rejects that authoring outright; the cleaner fix is to make application symmetric.
- `SpecialData::triggers_event` is inert: there is no event system to fire `ancient_awakening`.
- Scope decisions to take (build or cut): possession mode, overworld/surface raiding, meta-progression (faction runs, cross-mission unlocks), trade/hiring economy.

## Content

- The Gatehouse ships as a defensive *bonus* room, not `docs/ROOM_SET.md`'s "gates auto-close under threat" version — that wants a threat-detection subsystem it shares with alarm traps and the fog-of-war item. Decide whether to build it or restate the room as it now is.
- `UNCONSUMED` in `tests/live_data_fields_tests.rs` is the honest inventory of authored-but-inert data fields. The biggest clusters:
  - **Room build time** — `construction_time` is ignored, so rooms appear the instant they are paid for rather than being raised. Note before building this: the authored values are 0.4–0.9s, so wiring them as written buys a sub-second delay in exchange for a save-format change (per-tile progress) and renderer work to show a part-built room. Either re-scope the numbers to something a player would notice, or drop the field.
  - **The rest of the hero behaviour model** — `door_break_chance` (heroes never attack doors — they path around them, so there is nothing to roll against) and `call_for_aid` (needs a hero rally mechanic; the natural shape is a threatened hero pulling nearby allies onto their attacker).
- The field guard matches field *names* as text across `src/`, so a field sharing a name with a live one (`sprite`, `name`, `id`, `cost`, `icon`) reads as live even when it is dead — that is how `visual.sprite` stayed invisible in both rosters. Making the sweep struct-aware needs real parsing rather than text matching; until then, distrust it for common names.
- Hero-building destruction penalties **stack globally**: two razed barracks each contribute their full 50%, reaching the 90% cap. Whether that is right is a design question (it does reward levelling more of the base) rather than a bug.
- The hero base sits behind solid rock and creatures do not dig, so the keeper cannot assault it unaided at all — the raiders have to follow the corridor the heroes tunnel in through. That is a genuine asymmetry (heroes come to you) and may be worth an explicit design decision rather than an accident of the map.
- **9 of 20 heroes** are authored `will_fight_to_death`, including the `battle_cleric` that spawns in wave 1, so nearly half the roster never retreats regardless of the nerve model. Faithful to the data, but a much broader balance property than one late-game outlier — wants a real playtest.
- The Soul Furnace burns *hero* corpses only, matching the graveyard. Dead creatures still vanish with nothing to show for them — decide whether your own fallen should be renderable, which is a tone question as much as a balance one.
- The temple is available in only 1 of 13 missions (`pacts_and_sacrifice`), so its `mana_generation_per_second` and the whole prayer loop are nearly unreachable in the campaign. Either widen its availability or accept that the ritual circle is the real mana room.
- `execute_sleep`, `execute_eat` and `execute_deposit_gold` still test `room_type ==` a specific room. These are *not* the same edit `research` and `train` were — each needs a data decision first, so don't generalize them mechanically:
  - **sleep** — the `sleep` family is `lair` *and* `kennel`, so widening it lets creatures sleep in kennels. Probably right (a kennel is where beasts sleep), but it interacts with `count_available_lair_tiles`, which sets the creature cap. Decide the cap question first. Until then the kennel's authored `sleep_recovery_rate: 1.2` stays unreachable — the lair's 1.0 is the only one the engine can see.
  - **eat** — the hatchery is `task_type: "work"`, so there is no `eat` family to match on. Needs a `task_type` reassignment, which changes what `execute_work` sees.
  - **deposit** — the treasury is `task_type: "none"`, shared with graveyard, vault, mana_well and leisure_den. Keying on it would make the graveyard a treasury. Needs its own task type.
- The rest of the amenity tier (Mentor's Den, Doctrine Chamber) is pure data now that `happiness_modifier` is live — but `docs/rooms.md`, which `docs/ROOM_SET.md` cites as their design source, **does not exist**. Either write the designs or drop the rooms; there is nothing to build from.
- Creature AI has no *need* that an amenity room satisfies, so creatures only reach the Leisure Den by wandering into it. A `comfort` need in `monsters.json` with `satisfied_by: ["leisure_den"]` would let them seek it out deliberately, the way they already seek food and sleep.
- **Only the Assassin Wisp remains** unauthored from `docs/monsters.md`, and it is still blocked: its signature "bypasses doors and traps" is *already true of every creature* — `process_trap_triggers` only iterates `entities.heroes()`, so no creature can trigger a trap. Door bypass would need a per-creature pathfinding flag.
- No scenario lists **any** undead in `availability.creatures` — not skeleton, zombie, ghost or vampire, nor Lich or Grave Hulk. That is consistent rather than an omission (the tier is reached through `necromancy` research, and `availability` only seeds the starting set), but worth confirming that is the intent before someone "fixes" it.
- Rooms declare a `visual.wall_sprite` (`tiles/lair_wall.png`, …) that no generator emits and nothing reads. Deciding this needs a renderer answer, not a data one: walls are `solid_rock`/`earth` tiles, so drawing a room's own wall art means a per-wall-tile adjacency lookup every frame — which collides with the O(n) scan problem already flagged under Code quality. Treat it as part of the lighting/atmosphere pass, or drop the field.
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
- Remaining atmosphere-pass work on the lighting system:
  - **`flicker` is still unread** — it needs a time-varying term, which means the light map stops being a pure function of the dungeon. Cheap to add once there is a frame clock to hand; deliberately not to be faked with a per-frame random, which would strobe rather than flicker.
  - The light map is rebuilt once per tick on `GameState`. It splats outward from sources rather than asking each tile what lights it, so it is a few thousand operations — but it is still redundant work between dungeon changes, and belongs with the caching item under Code quality.
- Every existing sprite was authored while looking at inverted key lighting, so some base colours may have been chosen to compensate for the darkness. A post-fix sweep showed no blowouts, but a colour pass is worth doing when the palette is next revisited.
- UI art overhaul — playtest feedback calls the UI dated and Rust-default. Includes real entity rendering, sidebar animation and the minimap viewport.
- Title/menu art beyond the single `main_menu_bg.png`.

## UI & UX

- Settings menu breadth: resolution and window modes, volume sliders, keybind remapping (keys are hardcoded in `engine/input.rs`), camera/scroll options, autosave toggle.
- Save system, beyond the slots in scope above: quicksave/quickload, and save format versioning/migration.
- Tutorial, beyond the coverage in scope above: spells, prison/torture, the temple, contextual hints and an intro.
- Hotkey overlay / help screen.
- Localization: no i18n layer, all strings hardcoded English — externalize strings before it gets expensive.
- Accessibility: colorblind-safe faction palettes, text scaling beyond 3 steps, hold-vs-toggle options, screen-shake toggle.
- Decide on gamepad support (probably cut, but Steam Deck verification wants basic navigation).

## Balance

- Hero/creature level asymmetry: heroes gain +15%/level to cap 10, creatures +10% to cap 5 — the late game mathematically favours heroes.
- Wave pacing beyond wave 1 (in scope above): reconcile the shipped `wave_interval` and scaling curve against the balance numbers across a full mission.
- Starting gold scarcity near the dungeon core. Couples to the wave-1 item in scope above — more setup time and faster setup are interchangeable fixes, so measure before turning either.
- Room sell refund is 5%; 25–50% would make experimentation viable.
- Creature value outliers: Bile Demon overpriced, Hellhound free, Succubus mood loop, treasury desirability clustering.
- Creature wage and need decay rates still untuned.
- Structured playtest program: wave-1 survivability, sustainable army size, wave-10+ viability, per-mission tuning. The `wave` and `raid` capture scenes make the first two observable without a ten-minute sit — a first run showed wave 1 (2 heroes) reaching the dungeon heart and taking it to 988/1000 within ~15s of launching. Treat that as a starting point rather than a verdict: the scene seeds dig orders, which opens paths a real wave-1 dungeon would not have.
- Fold the `balance_calculator` simulations into `cargo test`/CI — its assertions are hand-rolled bools with no `#[test]`s.

## Code quality

- Error-handling hardening: ~25 `unwrap()`, 9 `expect()`, 4 `panic!` — asset loading and save handling especially.
- Deduplicate movement and distance logic between `creature_ai` and `imp_ai` (two `manhattan_distance` impls).
- Test coverage: combat math has one test, save/load one, UI/actions none. Add tests for menu action handlers, sidebar selection, tooltip state and dungeon command dispatch.
- Scenario fixtures for room placement, resource flow, encounters and progression milestones.
- Separate model mutation from the renderer and sidebar modules so UI actions call explicit domain commands.
- Performance on large maps: O(n) entity-position scans need a spatial index; cache pathfinding and room detection; profile the sidebar and renderer paths that recompute layout every frame. `effective_threat_multiplier` belongs on this list too — it is called twice per tick in `hero_spawner` and each call scans the creature list.
- Data-driven stragglers: hardcoded imp claim delay, loose spawn-placement validation.
- Decide on the GDD's determinism/replay aspiration while it is still cheap.
- Fix `docs/gdd.md`'s stale "Bevy ECS" claim.
