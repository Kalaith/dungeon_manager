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
- [x] Design a full campaign arc (industry norm for the genre: 15–20 missions; minimum viable ~10)
      with a difficulty curve, mechanic-introduction order, and narrative framing/briefings.
      Done: `docs/CAMPAIGN_ARC.md` — a grounded 12-mission arc (three acts, one branch at M6→M7a/M7b
      re-merging at M8 via an OR `required_completed` gate, boss climax at M11–M12). Every mechanic,
      creature, room, spell, trap, hero, objective type, trigger, and event action it references is
      one that already exists in the codebase (verified against `data/scenario.rs` and the content
      JSON). Includes an authored difficulty curve driven by the five existing scenario dials
      (`threat_multiplier`, `start_gold`, `max_creatures`, hero-party comp, rival aggression), a
      per-mission mechanic-introduction table (all rooms/traps/techs/spells and all 13 creatures
      introduced by M8; Act III is pure mastery), narrative framing + per-mission briefings, and the
      unlock graph that exercises the never-branched `unlocks_after`/`required_completed` logic. This
      is the design deliverable; *authoring* each mission's map + scenario JSON + event scripting is
      the next item below.
- [x] Author the missions: one map + one scenario JSON + event scripting each. The event system
      (triggers: TimeElapsed, ObjectiveComplete, RoomClaimed, ActionPointReached, DungeonBreached;
      actions: unlocks, spawns, rules) is done — this was pure content work. **DONE: the full
      12-slot arc from `docs/CAMPAIGN_ARC.md` is authored (13 mission *entries* — the M7 slot is a
      two-way branch)**, each a hand-authored 32×32 map + scenario JSON + event scripting wired into
      `deep_dominion.json`. The chain is linear M1→…→M6, then **M6 forks to both M7a and M7b**, and
      **both branches re-merge into M8**, then linear M8→M9→M10→M11→M12 (the fork/re-merge are the
      first real use of the never-branched unlock graph; see the "Exercise the unlock graph" item
      below):
      - **M1 `dark_beginnings`** (pre-existing): dig→build→recruit→survive.
      - **M2 `blood_and_iron`**: survive-720s→raze-outpost; a walled hero outpost w/ single gate;
        training_hall + guard_post + braced_door/blowgun_trap intro; two scripted hero waves
        (t+180/t+420); a `room_claimed` event unlocking braced_door; a more-aggressive rival.
      - **M3 `the_long_dark`**: an economy race — `gather_resource` 6000 gold + survive-600s, **no
        hero base** (heroes only raid via `steal_gold` parties at t+240/t+480); resource-rich map
        (24 gold/gem seams) shared with an economically-competing rival; `room_claimed` events on
        library→unlock lightning_strike and workshop→unlock boulder_trap.
      - **M4 `no_prisoners`** (Act I finale): survive-780s→raze a fortified 4-building outpost
        (town_hall/barracks/church/armory); introduces prison + torture_chamber + prison_tech as the
        mission's power tools with capturable named hero parties (knight, battle_cleric); a
        `room_claimed` guard_post→boulder_trap unlock. **Note:** the arc's "convert N heroes" *custom*
        objective was deferred — victory requires *all* objectives complete and a `custom` objective
        only completes via a `complete_objective` event, but no trigger counts hero conversions, so
        such an objective is currently unsatisfiable (would make the mission unwinnable). M4 ships
        with engine-supported objectives; a conversion-count trigger is a future §2 engine feature
        that would let this (and similar capture/harvest objectives) become hard win conditions.
      - **M5 `whispers_in_the_circle`** (Act II opener): survive-720s→raze a mage-tower outpost
        (town_hall/mage_tower/barracks/church); introduces ritual_circle + ritual_tech + the warlock
        caster + utility spells (reveal_map, speed_boost); a `room_claimed` ritual_circle→unlock
        lightning_strike event (bind the circle to gain offensive magic); wizard-led hero waves
        (t+180/t+450) that punish clumped defense; a mana-crystal-rich map for the mana economy.
      - **M6 `the_kennels`** (army-scaling): survive-840s→raze a ranged garrison
        (town_hall/barracks/archery_range/church); introduces kennel + barracks + scavenger rooms and
        the hellhound/spider/lizard beast roster, with a bumped creature cap (22 vs Act I's 15–18); a
        `room_claimed` kennel→unlock call_to_arms event; **sustained** pressure via three hero waves
        (t+150/t+400/t+650). Its `unlocks_after` names **both** branch missions below.
      - **M7a `restless_dead`** (branch — graveyard path): survive-900s→raze a holy garrison; the
        graveyard/vampire/troll undead path against a **paladin + high_priest (turn-undead)** order; a
        `room_claimed` graveyard→`spawn_creature` vampire event (the fallen rise).
      - **M7b `pacts_and_sacrifice`** (branch — temple path): survive-900s→raze an inquisition
        garrison; the temple/succubus/demon_spawn path against an **inquisitor + high_priest (banish)**
        order; a `room_claimed` temple→unlock corrupt_land + `spawn_creature` succubus event.
      - **M8 `the_iron_siege`** (Act II climax + branch re-merge): the difficulty knee — survive-960s
        behind layered defenses (introduces magic_door + alarm_trap, the bile_demon anchor tank, and
        the iron_skin spell; a map with a bedrock-spur choke corridor) against a **four-wave
        knight_commander gauntlet** (t+150/380/610/840 with paladin/high_priest/wizard support),
        *then* raze the garrison. `room_claimed` events unlock magic_door (workshop) and
        alarm_trap+iron_skin (guard_post). **Both M7 branches `unlocks_after` M8** — this is the
        re-merge (see below).
      - **M9 `corruption_rising`** (Act III opener — offense): the script flips — the player is the
        aggressor razing a full hero **town** (a 9×9 walled enclave with *six* halls:
        town_hall/mage_tower/barracks/archery_range/church/armory, two gates). Sole objective is
        `destroy_all_hero_buildings` (no survive floor); the town counter-sorties at the player's rear
        heart (t+200/450/700) so the risk is overextension, not defense. Adds the offense toolkit
        (corrupt_land, possess, and chickenify via a `room_claimed` library→research event); a mastery
        mission with no new rooms. (The arc's "raze within a time limit" fail-on-timeout isn't
        expressible — there's no lose-on-time objective — so M9 uses raze-the-town as the pure win.)
      - **M10 `two_kings`** (rival-keeper duel): a genuine **two-rival** mission — a three-heart map
        (player + RivalKeeper(1) + RivalKeeper(2)) with no hero base; win = survive-900s +
        `gather_resource` 8000 gold (out-expand and outlast both). **Multi-rival turned out to be
        already supported** — I'd flagged it as blocked, but investigation showed the sim handles it:
        `OwnerId::RivalKeeper(u8)` (any index), the faction table already makes two rivals *mutually*
        hostile (`RivalKeeper(a),RivalKeeper(b) => a!=b`), `owner_from_label` parses `keeper2`, the
        rival AI iterates *all* keepers, and `find_dungeon_heart_position` disambiguates the player
        heart by `Ownership::Player` even among three hearts. The only `RivalKeeper(1)` hardcodes are
        in `#[cfg(test)]` blocks. So M10 ships as a real duel (win adapted from the arc's unsatisfiable
        `destroy_heart(rival)` to survive + out-earn). NB: the §2 "support multiple simultaneous
        rivals" item is about rival-AI *economy depth*, which is still shallow — the *count* works.
      - **M11 `heavens_reach`** (Act III — capital assault): breach a large double-walled capital
        (8 halls, two gates) defended by a **knight_commander boss** (a lvl-3 hero entity in the
        garrison) plus paladin/high_priest/archmage relief columns; survive-900s + raze the capital;
        threat 1.5.
      - **M12 `deep_dominion_finale`** (finale): endure the full host (high_priest/archmage/
        knight_commander waves), then break the capital and its **Champion of Light boss** (lvl-3
        champion_of_light + high_priest + knight_commander in the last wave); survive-1020s + raze;
        threat 1.6, cap-30 army. The campaign ends here (`unlocks_after: []`).
      **Note (boss objectives):** the arc gave M11/M12 a "defeat the named boss" *custom* objective.
      Like M4/M7's deferrals, there's no "party/entity defeated" trigger to fire a `complete_objective`
      event, so the boss can't be a *separate* hard objective. Instead each boss is a high-level named
      hero placed in the capital garrison (+ in the final wave) that must be beaten *in practice* to
      raze the halls behind it; the win stays survive + `destroy_all_hero_buildings`. A
      party-defeated trigger is a future §2 engine feature that would promote "defeat the boss" to a
      first-class objective.
      **Note (deferred objectives):** the arc gave M7a a `destroy_heart(rival)` win and M7b a
      "N sacrifices" custom win. Both are currently *unsatisfiable* — a rival keeper's heart is a map
      *tile*, not the `Structure` entity `destroy_heart` scans for (only the player heart has tracked
      health), and no trigger counts sacrifices (same class of gap as M4's capture objective). Those
      two branch missions therefore ship with engine-supported objectives (survive + raze the themed
      outpost); a rival-heart entity + conversion/sacrifice-count triggers are future §2 engine
      features. **Branch re-merge — solved without an engine change:** the arc rejoins at M8 via an
      *OR* gate, and `required_completed` is AND-only, so instead of listing both branch ids on M8 the
      OR-join is routed through the *additive* `unlocks_after` edge: **both** M7a and M7b list
      `unlocks_after: ["the_iron_siege"]`, and M8 gates only on `required_completed: ["the_kennels"]`
      (already satisfied on either path). Completing either branch therefore unlocks M8. Verified
      end-to-end: every mission boots via `GameState::new_for_scenario` (live heart; hero bases
      active), all references validate (the guard test auto-covers new heroes/creatures incl.
      knight_commander), the branch is tested (M6 unlocks *both* paths), and **the re-merge is tested
      from *both* branches** (`either_m7_branch_re_merges_into_the_iron_siege` + the graveyard-path
      assertion), the Act III opener M9 boots as an offense mission (single raze objective + a
      six-hall town), **M10 loads two mutually-hostile rivals with the player heart still correctly
      identified among three hearts**
      (`two_kings_boots_with_two_hostile_rivals_and_a_clear_player_heart`), and **M11/M12 boot with a
      live capital and their named boss hero present**
      (`climax_missions_boot_with_capital_and_boss_heroes`). A completeness test confirms all 13
      mission entries load, the finale is terminal, and the whole chain is playable end-to-end
      (`deep_dominion_campaign_is_complete_thirteen_entries`). `cargo test` (**100 unit** incl. 21 new
      + 28 balance) + `clippy -D warnings` pass. **The campaign is content-complete.** Follow-on
      polish (not blocking): the deferred objective types above (capture/conversion/sacrifice/rival-
      heart/boss-defeated/fail-on-timeout) would each become a first-class win condition given a small
      §2 engine addition; and balance/pacing wants the §6 playtest program now that all 12 slots
      exist.
- [x] **Un-hardcode content loading**: `data/campaign.rs:128` and `data/scenario.rs:350` used to
      `include_str!` exactly one campaign/scenario file. Now manifest-driven: a new `build.rs` scans
      `assets/campaigns/` and `assets/scenarios/` at build time and generates
      `EMBEDDED_CAMPAIGNS`/`EMBEDDED_SCENARIOS` (`include_str!`-backed `&[&str]`, the "WASM embedded
      manifest"), pulled in via `data/embedded.rs`. On **native** builds `data/content_source.rs`
      overlays a runtime directory scan (`overlay_from_disk`) on top, so a mission JSON dropped into
      either directory loads *without recompiling*; on **wasm32** the embedded set is authoritative
      (drop a file + rebuild — no source edit either way). `load_scenarios()`/`load_campaigns()` were
      rewritten to use `content_source::from_embedded` + the native overlay. Verified: `cargo build`
      (build.rs runs), `cargo clippy --all-targets -- -D warnings`, `cargo test` (84 unit + 28
      balance, all pass) incl. 2 new `from_embedded` tests; and an end-to-end check that dropping a
      new `assets/scenarios/*.json` auto-appears in the generated manifest with zero code changes.
      Adding a mission is now pure content work. (Maps are still path-referenced and resolved via the
      content-pack roots — untouched here, as this item is specifically the campaign/scenario
      loaders.)
- [x] Exercise the unlock graph: `required_completed` / `unlocks_after` logic exists but had never
      branched (single linear mission). Done as part of campaign authoring above: `the_kennels` (M6)
      now `unlocks_after` **both** `restless_dead` (M7a) and `pacts_and_sacrifice` (M7b), each gated
      `required_completed: ["the_kennels"]` — a real design fork. Tested in
      `deep_dominion_unlocks_blood_and_iron_after_dark_beginnings`: completing M6 puts *both* branch
      missions in `unlocked_missions`. **The re-merge is also done and tested:** the arc rejoins the
      branch at M8 via an *OR* gate, and although `CampaignProgress` treats `required_completed` as
      **AND** (`campaign.rs`: `.all(...)`), the OR-join is expressed through the *additive*
      `unlocks_after` edge instead — both M7a and M7b `unlocks_after` M8, which gates only on
      `required_completed: ["the_kennels"]`. So completing *either* path unlocks M8, verified from
      both branches (`either_m7_branch_re_merges_into_the_iron_siege`). No engine change was needed —
      fork *and* re-merge both work on the existing graph.
- [x] Between-mission screens: mission briefing/debriefing, campaign map / mission-select UI.
      Done: a new **`GamePhase::MissionSelect`** — START GAME now opens a mission-select screen (it no
      longer jumps straight into the one mission), and the previously-dead "NEXT MISSION" victory
      button now fires because the campaign has 12+ missions. Pieces:
      - **Model** (`data/campaign.rs`, fully tested): `MissionStatus` (Completed/Available/Locked) +
        `CampaignProgress::mission_menu()` (every mission in authored order tagged by per-player
        status) + `select_mission()` (guards against launching a Locked mission). Test
        `mission_menu_tags_status_and_exposes_the_branch_choice` proves that at the M7 fork *both*
        branch paths show as Available — so the player can finally **choose** their branch (before,
        auto-advance always forced `restless_dead`).
      - **Screen** (`ui/menus.rs::draw_mission_select` + `ui/menu_layout.rs::mission_select_rows`,
        which auto-sizes rows so all 13 mission entries fit any window): renders the list with
        status colours + the hovered/active mission's briefing; `GamePhase::MissionSelect` dispatched
        in `ui/renderer.rs`.
      - **Input** (`engine/input.rs::handle_mission_select`): click an unlocked row → launch it via
        `GameState::new_for_campaign_progress`; Back/Esc → main menu.
      Verified: `cargo test` (101 unit incl. 1 new + 28 balance) + `clippy -D warnings` pass, **and a
      headless capture (`missionselect` scene) confirms the screen renders correctly** — 13 missions,
      the opener Available and the rest locked, the briefing line, and a Back button. Briefing/
      debriefing on victory is the existing game-over screen (it already shows the win + next
      mission's briefing). **Polish left (not blocking):** a graphical node-graph "campaign map" (this
      ships the functional list form), a dedicated pre-mission briefing modal, and carrying campaign
      progress back to the select screen after a mid-campaign win (today NEXT MISSION auto-continues
      linearly; the select screen is the fresh-start entry point).
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
- [x] **Status effects**: `combat.rs:222` `generate_status_effects()` returned an empty Vec by
      design ("no status effects for now"). The struct + duration ticking existed but nothing
      ever applied poison/burn/freeze/stun. Now fully wired: `combat::update_status_effects`
      applies poison/burn as real damage-over-time (ticks `strength` damage/sec off health),
      applies freeze as an immediate movement-speed multiplier that reverts exactly on expiry,
      and `combat::resolve_combat_tick` checks for an active "stun" effect and skips the
      attacker's attack (and any projectile spawn) entirely while stunned. Covered by 3 new
      tests in `combat_tests.rs` (`poison_and_burn_status_effects_deal_damage_over_time`,
      `freeze_status_effect_slows_movement_and_reverts_on_expiry`,
      `stunned_attacker_cannot_land_an_attack`).
- [ ] **Monster abilities** (partially done): `monsters.rs:59` deserialized `combat.abilities` as raw
      `serde_json::Value`, never read by the engine — but every authored entry was actually a
      bare ability-name string (`"charge"`, `"fireball"`, etc.), so the schema is now simply
      `Vec<String>`, matching the real data exactly. Execution is wired via a new data-driven
      `game_config.json` → `status_effects.ability_effects` table (ability id → status type/
      duration/strength/proc chance) that `combat::generate_status_effects` rolls against on
      every landed hit (melee instant, ranged/magic on the cast that spawns the projectile).
      Of the 8 distinct abilities currently authored across the 13-monster roster, the 4 that
      are naturally a status-effect proc are wired: `poison_bite` → poison, `fireball` → burn,
      `fire_breath` → burn, `lightning` → stun. The other 4 (`charge`, `smash`, `berserk`,
      `charm`) aren't status procs — they need bonus-damage/self-buff/morale mechanics the
      engine doesn't have hooks for yet, so they're still inert; adding more authored monsters
      (`docs/monsters.md`'s ~18) is separately tracked under §1 roster content.
- [x] **Hero abilities**: `HeroAbilityData` was parsed (`heroes.rs:15,57`) but `effect` was a bare
      label string (`"restore_health"`, `"area_damage"`, …) with zero engine references. Now
      fully data-driven, reusing spells' own schema and dispatcher rather than adding a parallel
      one: `HeroAbilityData.effects` is `Vec<data::spells::SpellEffect>` — the exact same struct
      and `spell_effects::apply_spell_effect` (now `pub(crate)`) spells already use. A new
      `engine::hero_abilities` module evaluates each ability's `trigger` against a small, fixed
      vocabulary matched by trigger *type* (never by ability id): "passive", "on_low_health" /
      "on_hit" (self), "on_ally_low_health" (nearest low-health same-owner hero),
      "on_target" / "on_undead_nearby" (nearest hostile entity, the latter checking the target's
      trait list — reuses the trait system below), "on_multiple_targets" (nearest hostile's
      position once ≥2 are in range), "in_room" / "in_room:<type>" (the hero's own tile/room).
      Several other authored trigger strings are recognized as aliases of these (`on_damaged` →
      `on_hit`, `on_group`/`on_large_group` → the multi-target family, `on_target_isolated` →
      exactly one hostile in range, etc.) — see the module doc comment for the full mapping.
      Rewrote all 28 authored abilities across 17 heroes in `heroes.json` with real `effects`
      arrays (damage/heal/status_apply/stat_modifier), and fixed `apply_stat_modifier` to also
      handle `Hero` entities (it only handled `Creature` before, so any hero speed buff would
      have silently no-op'd). `HeroState` gained `ability_cooldowns: HashMap<String, f32>`.
      5 of 28 abilities (dispel, purify, backstab, teleport, mass_cleanse) still can't fire —
      their triggers (`on_ritual_detected`, `on_corruption`, `on_sneak_attack`, `on_trapped`,
      `on_corruption_detected`) need ritual-detection/stealth/trap-state subsystems that don't
      exist yet; they're valid data, just inert until those systems exist. Adding a new ability
      with an already-recognized trigger — or changing an existing one's numbers — is now a pure
      `heroes.json` edit. Covered by 4 new tests in `hero_abilities_tests.rs`.
- [x] **Traits**: parsed `Vec<String>`, no trait logic anywhere. Now data-driven via a new
      `assets/data/traits.json` (array of `{id, ...}` objects, same convention as monsters/
      spells/etc., with full content-pack/mod merge support) and `TraitData`: a fixed set of
      generic numeric knobs (`mood_modifier`, `anger_threshold_modifier`,
      `desertion_threshold_modifier`, `need_decay_multipliers`, `task_preference_multipliers`,
      `attack_multiplier`, `defense_multiplier`, `discipline_response_multiplier`) that the
      engine sums/multiplies in wherever it already computes mood
      (`creature_ai::needs::calculate_mood`/`update_mood`), need decay (`update_needs`), task
      desirability (`creature_task_logic::calculate_task_desirability`), combat stats
      (`combat::extract_combat_stats`), and discipline response (`apply_slap`) — the engine never
      branches on a trait's name. All 17 currently-authored traits (loyal, cowardly, greedy,
      aggressive, intelligent, strong, slow, undead, mindless, fearless, demonic, wild, sadistic,
      arrogant, beast, glutton, hard_worker) got real values; adding a new trait, or changing what
      an existing one does, is now a `traits.json` edit. Covered by 3 new tests in
      `traits_tests.rs` (attack multiplier, need-decay zeroing, discipline-response damping).

### Known-broken / dead branches
- [x] `hero_ai.rs:43` — `threat_level` hardcoded to `Moderate` in `decide_hero_goal`
      ("Placeholder"); the real `evaluate_threat` is never consulted for goal selection.
      `decide_hero_goal` now takes the hero's position and calls `evaluate_threat` for real,
      matching the two other call sites in the same file that already did this correctly.
- [x] `spell_effects.rs:269` — heal spell **logs but never heals heroes** (creatures/structures OK).
      Fixed; `apply_heal_effect` now applies `heal_amount` to `hero.health` like it already did
      for creatures/structures. Covered by a new test (`heal_effect_heals_heroes`).
- [x] `spell_effects.rs:398` — `spawn_entity_effect` silently ignores every entity id except "imp".
      Generalized to look up any id in `game_data.monsters`; the imp-specific population cap still
      applies only to imps. Covered by a new test (`spawn_entity_effect_supports_non_imp_creatures`).
- [x] `spell_effects.rs:288` — stat modifiers support only "speed" and are **permanent** (no
      duration/revert). "speed" remains the only supported stat (it's the only mutable runtime
      stat `CreatureState` tracks — other stats are computed from base data + level at combat time
      and have no field to modify without a data-model change). Duration/revert is now wired: a
      timed speed buff pushes a `speed_modifier` status effect, and `combat::update_status_effects`
      divides the speed back out when it expires. Covered by a new test
      (`stat_modifier_speed_buff_reverts_after_duration`).
- [x] `task_system.rs:125` — wage-collection branch is an empty `if` (no theft/unrest consequence
      for unpaid creatures, which the design docs treat as core). The authored-but-inert
      `economy.steals_if_unpaid` monster field is now read: when the treasury has no gold to pay a
      creature, its "gold" need decays extra hard if it's theft-prone (2x vs. 1x for a docile
      creature), pushing it toward the desertion threshold faster — there's nothing literal to
      steal from an empty coffer, so this is the unrest consequence rather than a gold transfer.
      Covered by a new test (`unpaid_theft_prone_creature_loses_gold_satisfaction_faster_than_docile_one`).
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

- [x] File-size hard-limit violations (>800 lines). The `src/` files this item originally named
      (`sidebar_renderer.rs`, `game_state.rs`, `balance_calculator/analysis.rs`, `creature_ai.rs`,
      `renderer.rs`) were already back under 800 lines by the time this was picked up (this
      roadmap's own line counts were stale — see the disclaimer at the top). A repo-wide scan found
      the actual violators were all in the `graphics_gen/` dev-tool asset generator plus one test
      file: `graphics_gen/monsters.rs` (1053), `graphics_gen/core.rs` (956), `graphics_gen/tiles.rs`
      (882), `graphics_gen/heroes.rs` (839), `tests/balance_tests.rs` (856). All five were split into
      thin parent files + sibling submodules (`foo.rs` + `foo/child.rs`, no `mod.rs`), verified with
      `cargo check --all-targets`, `cargo clippy --all-targets`, and `cargo test` (68 unit + 28
      balance tests, all passing, no behavior change). No file in the repo now exceeds 800 lines.
      Later, adding hero ability cooldowns pushed `state/entities.rs` to exactly 800; split it the
      same way into `entities.rs` + `entities/{creature,hero}.rs` (CreatureState/HeroState) as
      part of that change, per the "restructure when a touched file approaches the limit" rule.
- [ ] Strip debug output: `eprintln!("[DEBUG] …")` throughout hero AI, combat, imp AI, spells,
      traps — hot-path console spam in release. Literal `[DEBUG]`-tagged prints removed from
      `hero_ai.rs` (8 call sites, per-frame per-hero pathfinding spam); ~98 other un-tagged
      `eprintln!` calls remain across combat/imp AI/spells/traps/save-load and still need a pass.
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
