Excellent. Monsters are not units, they’re **walking opinions about your dungeon**. The data should let them disagree with you loudly, occasionally violently 🦴😈

Below is a **modern, systemic Monster (Creature) JSON schema**, designed to mirror the **room schema style**, slot cleanly into **Rust + ECS**, and support emergent behavior without per-monster code.

---

# Monster / Creature Data Schema (JSON)

## Design Goals

* No hardcoded creature logic
* Behavior emerges from needs, traits, and preferences
* Rooms, morale, and economy all influence decisions
* Supports variants, mutations, and hero conversions
* Serializable and deterministic

---

## High-Level Structure

```json
{
  "id": "goblin",
  "name": "Goblin",
  "description": "A weak but eager creature, easily satisfied and easily frightened.",
  "faction": "dungeon",
  "role": "fighter",

  "stats": { },
  "needs": { },
  "traits": [ ],
  "ai": { },
  "combat": { },
  "progression": { },
  "visual": { }
}
```

---

## 1. Base Stats

Raw numbers. Everything else modifies these.

```json
"stats": {
  "health": 120,
  "mana": 0,
  "attack": 8,
  "defense": 3,
  "speed": 1.0,
  "carry_capacity": 25,
  "sight_radius": 6
}
```

---

## 2. Needs System

Needs tick down over time. Unmet needs cause mood decay.

```json
"needs": {
  "sleep": {
    "decay_per_minute": 1.0,
    "satisfied_by": ["lair"]
  },
  "food": {
    "decay_per_minute": 1.5,
    "satisfied_by": ["hatchery"]
  },
  "gold": {
    "decay_per_minute": 0.5,
    "satisfied_by": ["treasury"],
    "stash_amount": 50
  },
  "training": {
    "decay_per_minute": 0.8,
    "satisfied_by": ["training_hall", "combat_pit"]
  }
}
```

---

## 3. Traits

Traits are **tags with systemic meaning**. They modify AI weights and reactions.

```json
"traits": [
  "cowardly",
  "greedy",
  "pack_fighter"
]
```

Trait effects live in a separate trait definition table, keeping creatures clean.

---

## 4. AI Behavior

Defines how the creature makes decisions, not what decisions exist.

```json
"ai": {
  "base_mood": 60,
  "anger_threshold": 25,
  "desertion_threshold": 15,

  "task_preferences": {
    "guard": 1.2,
    "train": 0.8,
    "wander": 0.5
  },

  "room_desires": {
    "training_hall": 1.1,
    "leisure_den": 0.9
  },

  "discipline_response": {
    "slap": -5,
    "torture": -25,
    "reward": 10
  }
}
```

---

## 5. Combat Profile

What happens when talking stops.

```json
"combat": {
  "attack_type": "melee",
  "damage_range": [6, 10],
  "attack_speed": 1.2,
  "armor_type": "light",
  "resistances": {
    "fire": -0.2,
    "poison": 0.3
  },
  "abilities": ["pack_strike"]
}
```

---

## 6. Progression & Growth

Defines how the creature evolves.

```json
"progression": {
  "xp_to_level": [0, 100, 250, 500, 900],
  "stat_growth_per_level": {
    "health": 12,
    "attack": 2,
    "defense": 1
  },
  "max_level": 10,
  "mutations": [
    {
      "id": "hobgoblin",
      "conditions": {
        "level_at_least": 5,
        "training_hall_tiles": 20
      }
    }
  ]
}
```

---

## 7. Economy Interaction

How this creature touches your gold pile.

```json
"economy": {
  "wage_per_minute": 2,
  "steals_if_unpaid": true,
  "drops_gold_on_death": [10, 25]
}
```

---

## 8. Spawn & Attraction Rules

How they enter your dungeon.

```json
"spawn": {
  "source": "portal",
  "min_dungeon_reputation": 10,
  "preferred_rooms": ["training_hall"],
  "spawn_weight": 1.5,
  "max_population": 20
}
```

---

## 9. Visual & Audio

Pure flavor, no balance impact.

```json
"visual": {
  "sprite": "creatures/goblin.png",
  "scale": 1.0,
  "animations": ["idle", "walk", "attack", "sleep"],
  "voice_set": "goblin"
}
```

---

## 10. Complete Example: Goblin

```json
{
  "id": "goblin",
  "name": "Goblin",
  "description": "A weak but eager creature, easily satisfied and easily frightened.",
  "role": "fighter",

  "stats": {
    "health": 120,
    "attack": 8,
    "defense": 3,
    "speed": 1.0,
    "sight_radius": 6
  },

  "needs": {
    "sleep": { "decay_per_minute": 1.0, "satisfied_by": ["lair"] },
    "food": { "decay_per_minute": 1.5, "satisfied_by": ["hatchery"] },
    "gold": { "decay_per_minute": 0.5, "satisfied_by": ["treasury"] }
  },

  "traits": ["cowardly", "greedy"],

  "ai": {
    "base_mood": 60,
    "desertion_threshold": 15,
    "task_preferences": {
      "guard": 1.2,
      "train": 0.8
    }
  },

  "combat": {
    "attack_type": "melee",
    "damage_range": [6, 10],
    "attack_speed": 1.2
  },

  "progression": {
    "max_level": 10,
    "stat_growth_per_level": {
      "health": 12,
      "attack": 2
    }
  },

  "economy": {
    "wage_per_minute": 2
  },

  "spawn": {
    "source": "portal",
    "spawn_weight": 1.5
  },

  "visual": {
    "sprite": "creatures/goblin.png"
  }
}
```

---

## Rust ECS Mapping (Conceptual)

* `CreatureDefinition` → loaded from JSON
* `CreatureInstance` → entity with:

  * Current needs
  * Mood
  * Active traits
  * Task state
* Systems:

  * `need_decay_system`
  * `mood_evaluation_system`
  * `task_selection_system`
  * `combat_resolution_system`

No behavior branches. Only numbers arguing with each other ⚖️
