# Deep Dominion — Commercial Room Set (scoping decision)

*Satisfies COMMERCIAL_ROADMAP.md §1 "Rooms … Pick the commercial set." This is the **design/scoping
decision**, grounded in the engine as it actually is — building the picked rooms (art + per-room
mechanics) is the downstream work each entry sizes. Mirrors the `docs/CAMPAIGN_ARC.md` deliverable:
decide first, build against the decision.*

## Engine reality this decision is grounded in

Verified against the code (not the design docs), because it determines what a new room *costs*:

- **Room data is rich and data-driven** (`assets/data/rooms.json` / `special_rooms.json`:
  `build`/`requirements`/`effects`/`scaling`/`ai`/`visual`), but **most `effects` are consumed by
  room-type-keyed engine code, not generically.** Concretely:
  - **Generic (data-only) today:** `gold_storage_capacity` and `mana_storage_capacity` — applied to
    *any* room with the effect (`state/rooms.rs:25-29`). A pure storage room needs **no new code**.
  - **Room-type-keyed (needs engine code):** research (`task_system` checks `room_type == "library"`),
    `mana_generation_per_second` (only `temple`/`ritual_circle`, `special_rooms.rs:117`), corpse
    storage / `spawns_vampires` (`graveyard`), torture, prison, scavenger, hatchery food, training.
    A new room that does any of these needs a new `room_type` branch in the relevant system.
  - **Inert:** `happiness_modifier` is parsed but **has no consumer** (grep-verified) — a latent gap;
    wiring it generically (mood bonus for creatures in/near the room) would make a whole *class* of
    "amenity" rooms cheap. Tracked as a prerequisite for the amenity tier below.
- **Every room needs tile art.** Visuals reference `tiles/<room>_floor.png` + `_wall.png`; there is no
  `assets/sprites/rooms/` and no orphan room art (unlike the monster/hero sprites). So **every new
  room is art-blocked** until the §4 art pass, regardless of mechanics — this, not data, is the real
  gate. (Missing-texture placeholder handling, §4, would at least let un-arted rooms be playtested.)

**Cost tiers** used below: **D** = data-only (works today, art-blocked only); **H** = small engine
*hook* on an existing system (+ art); **S** = new *subsystem* (+ art); **X** = cut.

## The commercial set

**Baseline (shipped, 15):** lair, hatchery, treasury, library, workshop, training_hall, prison,
guard_post, ritual_circle, torture_chamber, graveyard, kennel, barracks, temple, scavenger — plus the
`dungeon_heart` special room. This is already a competitive core for the genre; **the commercial set =
these 15 + the "Commit" rooms below.** Everything else defers or cuts.

### Commit — build for 1.0 (7 new; ordered by value ÷ cost)

Grounded in `docs/rooms.md`'s designs, retitled where the doc's name is awkward:

| Room | From docs | Cost | Mechanic (engine hook) | Why it's in |
|------|-----------|------|------------------------|-------------|
| **Vault** | (Treasury successor / Legacy Vault's storage half) | **D** | pure `gold_storage_capacity` — bigger, denser gold cap than treasury | Zero new code; the cleanest first new room. Solves the "treasury desirability clustering" balance note by splitting *storage* from *creature gold-need*. |
| **Mana Well** | Power Conduit | **D** | pure `mana_storage_capacity` (+ later a mana-gen hook) | Data-only storage now; a natural home for the mana-gen hook once generalized past temple/ritual. |
| **Combat Pit** | Combat Pit | **H** | reuse the training loop (a second `task_type` producing XP/combat-skill) keyed on a new room_type | Depth for the army tier; small hook on an existing system. |
| **Soul Furnace** | Soul Furnace | **H** | generalize `mana_generation_per_second` past its temple/ritual hardcode → mana from nearby deaths/corpses | The undead/occult tier's signature; one filter-widen + a death hook. |
| **Gatehouse** | Gatehouse | **H** | a fortified chokepoint: room-scale door/wall with HP, gates auto-close under threat (reuse door/trap tile logic) | The defense tier's headline; leans on existing door/trap systems. |
| **Arcane Archive** | Arcane Archive | **H** | research-rate room like library but for *spell* research specifically (generalize the library key → a `research` task_type family) | Ties into the now-expanded tech tree; makes research a room *choice*, not just "build a library". |
| **Leisure Den** | Leisure Den (Casino successor) | **S**\* | amenity: mood/loyalty regen for idle creatures — *blocked on wiring the inert `happiness_modifier`* | Directly addresses the wage/desertion loop; **do the `happiness_modifier` generalization first**, then this + any amenity room is **D**. |

\* Leisure Den is **S** only because it needs the `happiness_modifier` consumer built; that one hook
turns the entire **amenity tier** (Leisure Den, Mentor's Den, Doctrine Chamber) into data-only rooms.

### Defer — post-1.0 depth (need real subsystems)

- **Summoning Vault** (S) — timed off-map creature summons; needs a summon-queue subsystem.
- **Fate Loom** (S) — high-risk ritual gambles; needs a risk/miscast subsystem (overlaps the §1 spell
  "miscasts" item).
- **Cryptorium** (S) — undead upgrade/mutation; needs the creature-mutation subsystem (a §2 "scope
  decision" fork).
- **Reeducation Chamber** (S) — deeper torture→conversion; extends the prison/torture subsystem.
- **Watch Nexus** (H) — vision/scouting hub; blocked on the "fog-of-war scouting gameplay" §2 item.
- **Supply Depot** (H) — trap-ammunition storage; blocked on the "trap ammunition/reload" §2 item.
- **Trap Corridor** (H) — roomified traps; nice, but overlaps existing trap placement — low marginal value.

### Cut — meta rooms that fight the current design

- **Legacy Vault / Council Chamber / Obelisk of Memory** — all serve *meta-progression* (cross-mission
  unlocks, faction runs), which the roadmap lists under **"Scope decisions needed"** (§2) as an
  explicit build-or-cut fork not yet taken. Cutting them from the room set until that meta-progression
  decision is made avoids building rooms with nothing to persist into. (The `Legacy Vault`'s *storage*
  function is absorbed by **Vault** above.)

## Build order (once art is scheduled)

1. **Vault** + **Mana Well** — data-only, prove the "new room" content pipeline end-to-end with two
   zero-code rooms (only art + a `rooms.json` entry + a tech-tree unlock).
2. **Generalize `happiness_modifier`** (one engine hook) → unlocks the amenity tier; ship **Leisure
   Den**.
3. **Combat Pit**, **Arcane Archive** — reuse the training/research task families.
4. **Soul Furnace**, **Gatehouse** — the occult and defense headline rooms.

Each Commit room also wants a `technologies.json` tech to gate it (the tree is now real — §1 "Tech
tree") and a slot in a mission's `availability`, so a new room is: **art + `rooms.json` entry + tech +
(engine hook if not storage)**.
