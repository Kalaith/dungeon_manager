Here is a **data-driven `spells.json` design** for **Dungeon Core–cast spells**.
These are *keeper spells*, not creature abilities. They bend the dungeon itself, tilt systems, and create expensive regrets 🔮🕯️

The schema mirrors your **rooms, monsters, heroes, and tiles** so everything plugs into the same ECS logic.

---

# Dungeon Core Spells (`spells.json`)

## Design Goals

* Spells affect **systems**, not just HP bars
* Clear costs and risks
* Scales with dungeon size and corruption
* No spell requires custom code paths
* Supports instant, duration, and aura spells

---

## High-Level Structure

```json
{
  "id": "spell_id",
  "name": "Spell Name",
  "description": "What the spell does and why it is tempting.",
  "category": "utility | combat | control | ritual",

  "cost": { },
  "targeting": { },
  "effects": [ ],
  "scaling": { },
  "cooldown": 0,
  "visual": { }
}
```

---

## 1. Cost Definition

```json
"cost": {
  "mana": 50,
  "gold": 0,
  "souls": 0,
  "health": 0
}
```

Costs can stack. Health cost means **Dungeon Heart damage**.

---

## 2. Targeting Rules

```json
"targeting": {
  "type": "tile | creature | room | area | global",
  "range": 10,
  "area_radius": 3,
  "requires_visibility": true,
  "valid_targets": ["enemy", "friendly", "neutral"]
}
```

---

## 3. Effects

Each spell can have **multiple effects**, evaluated in order.

```json
"effects": [
  {
    "type": "damage",
    "amount": 30,
    "damage_type": "lightning"
  }
]
```

Supported effect types (extensible):

* `damage`
* `heal`
* `status_apply`
* `stat_modifier`
* `tile_transform`
* `spawn_entity`
* `fear`
* `resource_change`
* `room_disable`
* `reveal_map`

---

## 4. Scaling

```json
"scaling": {
  "per_dungeon_tile": 0.02,
  "per_research_level": 0.1,
  "max_multiplier": 2.0
}
```

---

## 5. Visuals

```json
"visual": {
  "icon": "icons/spells/lightning.png",
  "cast_effect": "effects/lightning_cast",
  "impact_effect": "effects/lightning_hit",
  "screen_shake": 0.3
}
```

---

# Complete `spells.json` Example List

```json
[
  {
    "id": "lightning_strike",
    "name": "Lightning Strike",
    "description": "Calls down raw punishment from the Dungeon Heart.",
    "category": "combat",

    "cost": { "mana": 35 },

    "targeting": {
      "type": "tile",
      "range": 12,
      "area_radius": 1,
      "requires_visibility": true,
      "valid_targets": ["enemy"]
    },

    "effects": [
      {
        "type": "damage",
        "amount": 40,
        "damage_type": "lightning"
      }
    ],

    "cooldown": 2,

    "visual": {
      "icon": "icons/spells/lightning.png"
    }
  },

  {
    "id": "speed",
    "name": "Speed",
    "description": "Forces creatures into frantic efficiency.",
    "category": "utility",

    "cost": { "mana": 20 },

    "targeting": {
      "type": "creature",
      "range": 8,
      "valid_targets": ["friendly"]
    },

    "effects": [
      {
        "type": "stat_modifier",
        "stat": "speed",
        "multiplier": 1.5,
        "duration": 15
      },
      {
        "type": "status_apply",
        "status": "exhausted",
        "delay": 15
      }
    ],

    "cooldown": 5,

    "visual": {
      "icon": "icons/spells/speed.png"
    }
  },

  {
    "id": "heal",
    "name": "Heal Creature",
    "description": "Mends flesh, not morale.",
    "category": "utility",

    "cost": { "mana": 25 },

    "targeting": {
      "type": "creature",
      "range": 8,
      "valid_targets": ["friendly"]
    },

    "effects": [
      {
        "type": "heal",
        "amount": 50
      }
    ],

    "cooldown": 3,

    "visual": {
      "icon": "icons/spells/heal.png"
    }
  },

  {
    "id": "possess",
    "name": "Possession",
    "description": "Directly control a creature at great risk.",
    "category": "control",

    "cost": { "mana": 60, "health": 10 },

    "targeting": {
      "type": "creature",
      "range": 6,
      "valid_targets": ["friendly"]
    },

    "effects": [
      {
        "type": "status_apply",
        "status": "possessed",
        "duration": 30
      }
    ],

    "cooldown": 30,

    "visual": {
      "icon": "icons/spells/possess.png"
    }
  },

  {
    "id": "call_to_arms",
    "name": "Call to Arms",
    "description": "Drags all creatures toward a chosen point.",
    "category": "control",

    "cost": { "mana": 45 },

    "targeting": {
      "type": "tile",
      "range": 20,
      "area_radius": 6,
      "valid_targets": ["friendly"]
    },

    "effects": [
      {
        "type": "status_apply",
        "status": "forced_move",
        "duration": 20
      }
    ],

    "cooldown": 20,

    "visual": {
      "icon": "icons/spells/call_to_arms.png"
    }
  },

  {
    "id": "corrupt_land",
    "name": "Corrupt Land",
    "description": "Twists the dungeon floor to your influence.",
    "category": "ritual",

    "cost": { "mana": 80 },

    "targeting": {
      "type": "area",
      "range": 10,
      "area_radius": 4
    },

    "effects": [
      {
        "type": "tile_transform",
        "from": "claimed_floor",
        "to": "corrupted_floor"
      }
    ],

    "cooldown": 60,

    "visual": {
      "icon": "icons/spells/corruption.png"
    }
  },

  {
    "id": "reveal_map",
    "name": "Dark Revelation",
    "description": "Temporarily exposes hidden dungeon areas.",
    "category": "utility",

    "cost": { "mana": 40 },

    "targeting": {
      "type": "area",
      "range": 999,
      "area_radius": 8
    },

    "effects": [
      {
        "type": "reveal_map",
        "duration": 20
      }
    ],

    "cooldown": 25,

    "visual": {
      "icon": "icons/spells/reveal.png"
    }
  },

  {
    "id": "summon_imps",
    "name": "Summon Imps",
    "description": "Creates new servants from raw mana.",
    "category": "utility",

    "cost": { "mana": 50 },

    "targeting": {
      "type": "tile",
      "range": 5
    },

    "effects": [
      {
        "type": "spawn_entity",
        "entity": "imp",
        "count": 2
      }
    ],

    "cooldown": 15,

    "visual": {
      "icon": "icons/spells/summon.png"
    }
  }
]
```

---

## Design Notes (Important)

* **Dungeon Core spells should feel expensive**, even when cheap
* Cooldowns matter more than raw mana
* Most spells should create *secondary problems*
* Ritual spells reshape the map, not combat
* Possession is optional and always dangerous

---

## Easy Future Extensions

* Spell corruption levels
* Spell miscasts
* Hero counter-spells
* Research-unlocked modifiers
* Per-biome spell variants
