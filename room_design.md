Excellent choice. Rooms are the beating organs of a Dungeon Keeper–style game, so the schema should feel **structural, extensible, and very moddable**. Below is a **production-ready JSON room data schema**, designed to map cleanly onto a Rust ECS and stay friendly to tools and hot-reloading 🧱📜

---

# Room Data Schema (JSON)

## Design Goals

- Data-driven, no hardcoded room logic
- Supports room scaling and shape bonuses
- Clean hooks for AI desirability
- Works with tile-based isometric ECS
- Easy to extend later (rituals, corruption, factions)

---

## High-Level Structure

Each room is a **definition**, not an instance.

```json
{
  "id": "training_room",
  "name": "Training Room",
  "description": "Creatures hone their combat skills here.",
  "category": "training",
  "icon": "icons/rooms/training.png",

  "build": {},
  "requirements": {},
  "effects": {},
  "ai": {},
  "visual": {}
}
```

---

## 1. Build Section

Controls cost, placement, and construction rules.

```json
"build": {
  "cost_per_tile": 150,
  "mana_cost": 0,
  "min_tiles": 9,
  "max_tiles": 81,
  "dig_required": true,
  "requires_claimed": true,
  "can_overlap": false,
  "allowed_terrain": ["claimed_floor"],
  "construction_time": 0.5
}
```

### Notes

- `construction_time` is **seconds per tile**
- `allowed_terrain` lets you do lava-only or water rooms later
- `can_overlap` allows special rooms (e.g. heart aura rooms)

---

## 2. Requirements Section

What must exist before this room is buildable.

```json
"requirements": {
  "research": ["training_tech_1"],
  "global_rooms_required": [],
  "max_instances": 1,
  "forbidden_if": ["library"]
}
```

### Notes

- `forbidden_if` allows mutually exclusive designs
- `max_instances = 0` can mean infinite

---

## 3. Effects Section

What the room _does_ while active.

```json
"effects": {
  "creature_xp_per_second": 0.8,
  "mana_drain_per_tile": 0.2,
  "happiness_modifier": -2,
  "stat_bonuses": {
    "attack": 1.0,
    "defense": 0.5
  },
  "aura": {
    "radius": 6,
    "effects": {
      "combat_speed": 0.1
    }
  }
}
```

### Notes

- Effects are **passive**, evaluated during simulation ticks
- Auras allow spatial influence without explicit room use
- Keep values small, let size do the work

---

## 4. Scaling Section

Controls how room size and shape affect output.

```json
"scaling": {
  "per_tile_multiplier": 1.0,
  "size_thresholds": [
    { "tiles": 9, "multiplier": 1.0 },
    { "tiles": 16, "multiplier": 1.25 },
    { "tiles": 25, "multiplier": 1.5 }
  ],
  "shape_penalties": {
    "thin_corridor": 0.75,
    "fragmented": 0.5
  }
}
```

### Notes

- Shape penalties discourage spaghetti dungeons
- Shape classification handled by room validation system

---

## 5. AI Section

Defines how creatures perceive and use the room.

```json
"ai": {
  "task_type": "train",
  "desirability": 1.2,
  "max_creatures": 6,
  "preferred_creatures": ["goblin", "orc"],
  "forbidden_creatures": ["imp"],
  "entry_conditions": {
    "min_level": 2,
    "max_level": 8,
    "mood_above": 40
  }
}
```

### Notes

- `desirability` feeds into task selection weighting
- AI systems only read this, never special-case rooms
- Keeps behavior emergent, not scripted

---

## 6. Visual Section

Purely presentation, zero gameplay impact.

```json
"visual": {
  "floor_sprite": "tiles/training_floor.png",
  "wall_sprite": "tiles/training_wall.png",
  "object_spawn": [
    {
      "object": "training_dummy",
      "density": 0.15
    }
  ],
  "light": {
    "color": [255, 80, 80],
    "intensity": 0.6,
    "flicker": true
  }
}
```

### Notes

- `density` = objects per tile
- Visuals can be swapped without breaking saves
- Lighting is critical in isometric readability

---

## 7. Complete Example: Training Room

```json
{
  "id": "training_room",
  "name": "Training Room",
  "description": "Creatures train relentlessly, trading comfort for strength.",
  "category": "training",

  "build": {
    "cost_per_tile": 150,
    "min_tiles": 9,
    "dig_required": true,
    "requires_claimed": true,
    "allowed_terrain": ["claimed_floor"],
    "construction_time": 0.5
  },

  "requirements": {
    "research": ["training_tech_1"],
    "max_instances": 0
  },

  "effects": {
    "creature_xp_per_second": 0.8,
    "happiness_modifier": -2
  },

  "scaling": {
    "size_thresholds": [
      { "tiles": 9, "multiplier": 1.0 },
      { "tiles": 16, "multiplier": 1.25 },
      { "tiles": 25, "multiplier": 1.5 }
    ]
  },

  "ai": {
    "task_type": "train",
    "desirability": 1.2,
    "max_creatures": 6,
    "forbidden_creatures": ["imp"]
  },

  "visual": {
    "floor_sprite": "tiles/training_floor.png",
    "object_spawn": [{ "object": "training_dummy", "density": 0.15 }],
    "light": {
      "color": [255, 80, 80],
      "intensity": 0.6
    }
  }
}
```

---

## Rust ECS Mapping (Conceptual)

- `RoomDefinition` → loaded from JSON
- `RoomInstance` → entity with:
  - Tile list
  - Cached size & shape
  - Active effect values

- Systems:
  - `room_validation_system`
  - `room_effect_tick_system`
  - `room_ai_offer_system`

No room ever needs custom code. That’s the real spell circle 🜏
