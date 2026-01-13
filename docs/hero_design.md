Perfect. Heroes are not just enemies, they are **walking audits of your dungeon design**. They should punish sloppy layouts, expose weak systems, and occasionally die in very embarrassing places ⚔️📜

Below is a **Hero JSON design schema** followed by a **modern hero class list** tailored for a DK-inspired clone.

---

# Hero Data Schema (JSON)

## Design Goals

* Heroes are **goal-driven**, not mindless attackers
* Strong interaction with rooms, traps, morale, and lighting
* Scales cleanly from fodder to raid-boss adventurers
* Mirrors monster schema, but with clearer intent logic

---

## High-Level Structure

```json
{
  "id": "knight",
  "name": "Knight",
  "description": "A disciplined warrior sworn to cleanse evil.",
  "tier": 2,
  "role": "frontliner",

  "stats": { },
  "ai": { },
  "combat": { },
  "abilities": [ ],
  "progression": { },
  "behavior": { },
  "visual": { }
}
```

---

## 1. Base Stats

Heroes tend to be more focused and less moody than monsters.

```json
"stats": {
  "health": 180,
  "mana": 20,
  "attack": 14,
  "defense": 8,
  "speed": 1.0,
  "sight_radius": 8,
  "bravery": 70
}
```

---

## 2. AI & Decision Making

Heroes operate on **mission priorities**.

```json
"ai": {
  "primary_goal": "destroy_heart",
  "secondary_goals": ["kill_creatures", "steal_gold"],
  "room_priorities": {
    "dungeon_heart": 3.0,
    "treasury": 1.5,
    "prison": 1.2
  },
  "threat_response": {
    "retreat_below_health": 0.25,
    "call_for_aid": true
  }
}
```

---

## 3. Combat Profile

```json
"combat": {
  "attack_type": "melee",
  "damage_range": [12, 18],
  "attack_speed": 1.0,
  "armor_type": "medium",
  "resistances": {
    "fire": 0.1,
    "dark": -0.2
  }
}
```

---

## 4. Abilities

Heroes are defined by *tools*, not raw stats.

```json
"abilities": [
  {
    "id": "shield_block",
    "cooldown": 10,
    "trigger": "on_hit",
    "effect": "reduce_damage"
  },
  {
    "id": "rally",
    "cooldown": 30,
    "trigger": "on_low_health",
    "effect": "buff_allies"
  }
]
```

---

## 5. Behavior Modifiers

How heroes interact with dungeon systems.

```json
"behavior": {
  "trap_awareness": 0.7,
  "door_break_chance": 0.4,
  "light_preference": "bright",
  "fear_resistance": 0.6,
  "will_fight_to_death": false
}
```

---

## 6. Progression & Scaling

```json
"progression": {
  "level_range": [3, 7],
  "stat_growth_per_level": {
    "health": 20,
    "attack": 3,
    "defense": 2
  },
  "elite_variants": ["knight_commander"]
}
```

---

## 7. Visual & Audio

```json
"visual": {
  "sprite": "heroes/knight.png",
  "scale": 1.1,
  "animations": ["idle", "walk", "attack", "block"],
  "voice_set": "knight"
}
```

---

## Complete Example: Knight

```json
{
  "id": "knight",
  "name": "Knight",
  "description": "A disciplined warrior sworn to cleanse evil.",
  "tier": 2,
  "role": "frontliner",

  "stats": {
    "health": 180,
    "attack": 14,
    "defense": 8,
    "speed": 1.0,
    "sight_radius": 8,
    "bravery": 70
  },

  "ai": {
    "primary_goal": "destroy_heart",
    "room_priorities": {
      "dungeon_heart": 3.0,
      "treasury": 1.5
    },
    "threat_response": {
      "retreat_below_health": 0.25
    }
  },

  "combat": {
    "attack_type": "melee",
    "damage_range": [12, 18],
    "attack_speed": 1.0
  },

  "abilities": [
    { "id": "shield_block", "cooldown": 10 }
  ],

  "behavior": {
    "trap_awareness": 0.7,
    "fear_resistance": 0.6
  },

  "progression": {
    "level_range": [3, 7]
  },

  "visual": {
    "sprite": "heroes/knight.png"
  }
}
```

---

# Modern Hero Roster (List)

## 🧍 Early-Game Adventurers (Tier 1)

* **Peasant Militia**
  Weak, numerous, panic easily.

* **Scout**
  Fast, high trap awareness, low combat power.

* **Acolyte**
  Weak caster, supports allies, dispels rituals.

---

## ⚔️ Mid-Game Heroes (Tier 2)

* **Knight**
  Balanced frontline threat.

* **Archer**
  Ranged damage, prioritizes traps and imps.

* **Battle Cleric**
  Heals allies, resists fear and corruption.

* **Rogue**
  High trap avoidance, attacks backline rooms.

---

## 🔮 Specialist Heroes (Tier 3)

* **Paladin**
  Anti-undead, aura-based buffs.

* **Wizard**
  High damage spells, fragile, targets lairs and libraries.

* **Inquisitor**
  Disables rooms, suppresses creature morale.

---

## 🛡 Elite & Boss Heroes (Tier 4–5)

* **Knight Commander**
  Buffs nearby heroes, organized assault behavior.

* **High Priest**
  Cleanses dungeon effects, disrupts rituals.

* **Archmage**
  Area devastation, dungeon geometry stress-test.

* **Champion of Light**
  Semi-boss. Ignores fear, hunts the Heart relentlessly.

---

## 🧠 Design Philosophy for Heroes

* Heroes test **specific dungeon failures**
* Good dungeons kill heroes indirectly
* Great dungeons make heroes regret entering
* Boss heroes force **rebuilds**, not just fights

---

## Want Next?

I can:

* ⚔️ Design **hero party composition rules**
* 🧠 Build **hero goal-state machines**
* 🧾 Provide **hero ability JSON schemas**
* 🧱 Create **hero vs room counter tables**
* 🔥 Design **final raid-style hero invasions**

Point at a hero gate and I’ll start drafting the obituary.
