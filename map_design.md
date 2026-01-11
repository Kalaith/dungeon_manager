# Dungeon Keeper Level Design Format

Perfect reference. Dungeon Keeper's level scripts are basically a **domain-specific language pretending not to be one**. For a modern Rust-based clone, you want something that is:

- Data-driven
- Diffable
- Toolable (editor-friendly)
- Still expressive enough for triggers, AI, parties, win conditions

Below is a **clean, modern JSON level design format** inspired by DK, but not shackled to its syntax.

I'll give you:

1. Overall level file structure
2. Concrete `level.json` example
3. Trigger & condition system
4. Hero party definition
5. Notes on how this maps cleanly into Rust ECS / systems

---

## 1️⃣ High-Level Level JSON Structure

```json
{
  "meta": {},
  "rules": {},
  "players": {},
  "creatures": {},
  "rooms": {},
  "magic": {},
  "traps": {},
  "map": {},
  "events": [],
  "hero_parties": {},
  "win_conditions": [],
  "lose_conditions": []
}
```

Each section is **pure data**. Logic lives in systems.

---

## 2️⃣ Example `level_vulkaan.json`

### Meta & Global Rules

```json
{
  "meta": {
    "id": "vulkaan",
    "name": "Vulkaan",
    "version": 1,
    "author": "Yani",
    "description": "A volcanic standoff between keepers and heroes"
  },

  "rules": {
    "generate_speed": 600,
    "torture_convert_chance": 0.6
  },
```

---

### Players

```json
  "players": {
    "player0": {
      "type": "human",
      "start_money": 2500,
      "max_creatures": 15
    },
    "player3": {
      "type": "ai",
      "start_money": 0,
      "max_creatures": 25,
      "ai_profile": "aggressive_keeper"
    },
    "heroes": {
      "type": "hero_faction"
    }
  },
```

---

### Creature Pool & Availability

```json
  "creatures": {
    "pool": {
      "fly": 5,
      "bug": 10,
      "demonspawn": 10,
      "troll": 20,
      "spider": 20,
      "orc": 20,
      "sorcerer": 20,
      "dragon": 10,
      "dark_mistress": 50
    },

    "availability": [
      { "player": "all", "creature": "demonspawn", "level": 1 },
      { "player": "player0", "creature": "bug", "level": 1 },
      { "player": "player3", "creature": "dark_mistress", "level": 1 }
    ]
  },
```

---

### Rooms

```json
  "rooms": {
    "starting": [
      { "player": "player0", "room": "treasury", "min_size": 1 },
      { "player": "player0", "room": "lair", "min_size": 1 },
      { "player": "player0", "room": "hatchery", "min_size": 1 },
      { "player": "player0", "room": "training", "min_size": 1 }
    ],

    "unlock_on_claim": [
      { "player": "player0", "room": "research", "min_size": 3 },
      { "player": "player0", "room": "workshop", "min_size": 3 },
      { "player": "player0", "room": "torture", "min_size": 3 }
    ]
  },
```

---

### Magic / Keeper Spells

```json
  "magic": {
    "player0": [
      { "spell": "hand", "enabled": true },
      { "spell": "slap", "enabled": true },
      { "spell": "imp", "enabled": true },
      { "spell": "possess", "enabled": false }
    ],
    "player3": [
      { "spell": "hand", "enabled": true },
      { "spell": "heal_creature", "enabled": true }
    ]
  },
```

---

### Traps & Doors

```json
  "traps": {
    "available": [
      { "type": "poison_gas", "player": "all" },
      { "type": "lightning", "player": "all" }
    ],
    "doors": [
      { "type": "braced", "player": "all" },
      { "type": "steel", "player": "all" }
    ]
  },
```

---

### Map & Fog

```json
  "map": {
    "concealed_areas": [
      { "player": "player0", "x": 241, "y": 220, "w": 12, "h": 9 }
    ]
  },
```

---

## 3️⃣ Events, Triggers & Conditions

This replaces DK's `IF_ACTION_POINT`, `IF_DUNGEON_DESTROYED`, etc.

```json
  "events": [
    {
      "id": "heroes_heart_destroyed",
      "when": {
        "type": "dungeon_destroyed",
        "player": "heroes"
      },
      "then": [
        { "action": "unlock_room", "player": "player0", "room": "bridge" },
        { "action": "spawn_creature", "player": "player3", "creature": "dark_mistress", "x": 7, "y": 10 },
        { "action": "enable_spell", "player": "player3", "spell": "call_to_arms" }
      ]
    }
  ],
```

---

## 4️⃣ Hero Parties

```json
  "hero_parties": {
    "cheeky": {
      "members": [
        { "unit": "dwarf", "level": 1, "count": 2 }
      ],
      "behavior": "defend_location"
    },

    "knight_fight": {
      "members": [
        { "unit": "knight", "level": 10, "count": 1 },
        { "unit": "samurai", "level": 8, "count": 3 },
        { "unit": "giant", "level": 7, "count": 2 }
      ],
      "behavior": "attack_dungeon_heart"
    }
  },
```

Spawn via events:

```json
{
  "when": { "type": "action_point", "id": 4 },
  "then": [
    { "action": "spawn_party", "party": "knight_fight", "player": "heroes" }
  ]
}
```

---

## 5️⃣ Win / Lose Conditions

```json
  "win_conditions": [
    {
      "type": "and",
      "conditions": [
        { "type": "dungeon_destroyed", "player": "player3" },
        { "type": "dungeon_destroyed", "player": "heroes" }
      ]
    }
  ],

  "lose_conditions": [
    { "type": "dungeon_destroyed", "player": "player0" }
  ]
}
```

---

## 6️⃣ Why This Works Beautifully in Rust 🦀

- Deserialize with `serde`
- Events become queued ECS commands
- Conditions are pure data evaluators
- Easy modding
- Easy editor tooling
- Deterministic replays possible

Think of this as **Dungeon Keeper rewritten as a declarative spellbook**, not a pile of arcane macros.
