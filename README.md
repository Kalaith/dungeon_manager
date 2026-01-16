# Deep Dominion (Dungeon Manager)

A dungeon management simulation game built in Rust using the Macroquad engine. Become the Dungeon Overseer, build your layout, manage your minions, and defend against invading heroes!

> **Project Status**: 🚧 *Early Development*  
> Core systems implemented: procedural map generation, room building, creature AI, needs/mood system, combat, spells, traps.  
> Actively evolving: UI polish, balance tuning, content expansion, win conditions.  
> 📋 See **[ROADMAP.md](ROADMAP.md)** for detailed progress and planned features.  
> Contributions and feedback welcome!

## Core Gameplay Loop

1. **Dig & Claim**: Expand your territory by ordering Imps to dig through earth and claim floors.
2. **Build Rooms**: Construct essential rooms (Lair, Hatchery, Treasury) to keep your creatures alive and happy.
3. **Grow Your Army**: Attract creatures via spawners; keep them fed, rested, and paid to prevent desertion.
4. **Expand Economy**: Mine Gold Veins and Gem Seams; harvest Mana Crystals for spellcasting.
5. **Research & Upgrade**: Use the Library to unlock advanced rooms, spells, and traps.
6. **Fortify Defenses**: Place Doors and Traps at chokepoints to slow and damage invading heroes.
7. **Survive Invasions**: Heroes spawn from portals in escalating waves. Repel them or lose your Dungeon Heart.

**Victory**: Survive and dominate. *(Currently endless/sandbox mode—scenario objectives planned for future)*

**Defeat** occurs when:
- **Dungeon Heart Destroyed**: The Heart has 100 HP. If heroes reach it and deal enough damage, you lose instantly.
- **Total Collapse**: All your creatures are dead or have deserted due to low morale.

### 💀 The Dungeon Heart
Your Dungeon Heart is the core of your domain:
- **HP**: 100 (cannot be healed)
- **Location**: Placed at starting position (center or corner depending on map); cannot be moved
- **Function**: Generates +1 Mana/sec, provides initial Gold/Mana storage
- **Vulnerability**: Heroes prioritize attacking it. If HP reaches 0, **game over**.

> **Note**: Morale collapse (all creatures deserting) also ends the run, even if the Heart survives. Keep your minions happy!

## Features

### 🏰 Dungeon Building
- **Digging**: Carve out your dungeon from solid rock.
- **Rooms**: Construct various rooms to attract and support your minions:
  - **Lair**: Where your creatures sleep.
  - **Hatchery**: Provides food (chickens) for your creatures.
  - **Treasury**: Stores your mined gold.
  - **Training Hall**: Allows creatures to gain experience.
  - **Library**: Research new spells.
  - **Workshop**: Manufacture traps and doors.
  - **Guard Post**: Station creatures to defend key areas.
  - **Prison**: Capture invading heroes.
  - **Ritual Circle**: Perform dark rituals to summon creatures or boost spells.
- **Traps & Doors**: Fortify your dungeon with Doors, Spike Traps, Boulder Traps, and Alarm Traps.

### 😈 Minion Management
- **Imps**: Your loyal workers who dig, mine, and claim territory.
- **Creatures**: Various monsters with individual needs (Sleep, Food, Mood) and jobs.
- **Heroes**: Invaders who seek to destroy your Dungeon Heart.
- **Spawner**: Place monster spawners to grow your army.

### ✨ Magic System
- **Spells**: Cast powerful spells like "Summon Imps".
- **Economy**: Manage **Gold** for building and **Mana** for spellcasting.
- **Materials**: Harvest resources for constructing traps.

### 🖥️ UI
- **Sidebar Interface**: Unified control panel for Building, Magic, Minion management, and Traps.
- **Inspection**: View detailed stats of your units and rooms.

---

## Technical System Deep Dive

### 🎨 Procedural Image Generation
The game utilizes a custom-built procedural graphics engine (`build_graphics.rs`) to generate all game assets (tiles, sprites) at compile time, requiring no external art assets. Key features:

- **Software Rendering Pipeline**: A from-scratch rendering engine implementing:
  - **Depth Buffering**: Z-sorting for 3D occlusion in sprites.
  - **Primitive Rendering**: Draws Spheres, Ellipsoids, Cylinders, and Cones using ray-geometry intersection.
  - **Blinn-Phong Shading**: Real-time lighting with Ambient, Diffuse, and Specular components.
  - **Material System**: Material types include `Matte`, `Metallic`, `Leather`, `Glowing`, and `Bone`, each with unique light properties (`shininess`, `specular`).
- **Procedural Texturing**: Algorithms for generating patterns (noise, grids, veins) for tiles.
- **Asset Categories**:
  - **Tiles (64x64)**: Environment tiles (Rock, Earth, Lava, Water, Gem Seam, Gold Vein, etc.)
  - **Room Tiles**: Dungeon Heart, Lair, Hatchery, Treasury, Workshop, Library, Prison, etc.
  - **Trap Tiles**: Door, Spike Trap, Boulder Trap, Alarm Trap.
  - **Monster Sprites (64x64)**: Imp, Goblin, Orc, Warlock, Troll, Skeleton, Demon Spawn, etc.
  - **Hero Sprites (64x64)**: Peasant, Scout, Acolyte, Knight, Archer, Paladin, Wizard, Archmage, etc.

---

### 🗺️ Modular Map Generation
Dungeons are created using a multi-stage procedural pipeline (`src/engine/map_generator/`):

1. **Base Terrain** (`terrain.rs`): Generates layout using simplex noise or cellular automata.
2. **Smoothing** (`terrain.rs`): Cellular automata passes for organic cave structures.
3. **Connectivity** (`connectivity.rs`): Flood-fill ensures all areas are reachable; culls disconnected regions.
4. **Hazards** (`resources.rs`): Places Lava pools and Water bodies.
5. **Resources** (`resources.rs`): Strategic placement of Gold Veins, Gem Seams, and Mana Crystals using risk/reward clustering (valuable resources near hazards).
6. **Features** (`features.rs`): Places Hero Portals, Monster Lairs, and natural features.
7. **Biomes** (`biomes.rs`): Optional biome layering affecting terrain and resources.
8. **Starting Area** (`starting_area.rs`): Clears a safe zone with the Dungeon Heart and initial rooms (Treasury, Lair).

**Map Presets**: `generate_test_map`, `generate_rich_map`, `generate_hazardous_map`, `generate_classic_map`, `generate_corner_start_map`.

---

### 🧱 Tile Types
All tiles are data-driven via `assets/data/tiles.json`:

| Tile | Category | Diggable | Blocks Movement | Notes |
|---|---|---|---|---|
| Solid Rock | `rock` | No | Yes | Impenetrable |
| Earth | `rock` | Yes | Yes | Standard digging target |
| Claimed Floor | `floor` | No | No | Supports room construction |
| Reinforced Wall | `wall` | No | Yes | High durability |
| Gold Vein | `resource` | Yes | Yes | Yields 750 Gold (finite) |
| Gem Seam | `resource` | Yes | Yes | Infinite Gold source |
| Mana Crystal | `resource` | Yes | Yes | Yields 250 Mana |
| Lava | `hazard` | No | Yes | 25 damage/sec |
| Water | `hazard` | No | Yes | Impassable |
| Bridge | `structure` | No | No | Built over water/lava |
| Corrupted Floor | `floor` | No | No | Aura: +fear to heroes, +morale to creatures |
| Ancient Rune Floor | `special` | No | No | Triggers special events |

---

### 🏠 Room System
Rooms are data-driven via `assets/data/rooms.json`.

| Room | Gold Cost/Tile | Min Tiles | Effects | Research Required |
|---|---|---|---|---|
| **Dungeon Heart** | 0 | 1 | Mana gen +1/s, Gold storage +500, Mana storage +100 | - |
| **Lair** | 50 | 4 | Sleep recovery rate, +10 happiness | - |
| **Hatchery** | 75 | 9 | Food gen +0.5/s, +2 happiness | - |
| **Treasury** | 100 | 1 | Gold storage +50/tile | - |
| **Library** | 150 | 9 | Research rate +1.0 | - |
| **Workshop** | 120 | 9 | Trap production rate, -1 happiness | - |
| **Training Hall** | 150 | 9 | Creature XP +0.8/s, attack/defense bonuses | `training_tech` |
| **Prison** | 80 | 4 | Hero conversion rate +0.1 | `prison_tech` |
| **Guard Post** | 90 | 4 | Defense bonus +1.5, combat speed aura | - |
| **Ritual Circle** | 200 (+50 Mana) | 9 | Mana gen +0.5/s, spell power +1.2 | `ritual_tech` |

---

### ⚔️ Trap System
Traps are data-driven via `assets/data/traps.json`. Traps require a **Workshop** room and **Materials** to construct.

| Trap | Gold Cost | Build Time | Effect |
|---|---|---|---|
| **Door** | 50 | 5s | Blocks movement, lockable |
| **Spike Trap** | 100 | 10s | 25 damage, pressure trigger |
| **Boulder Trap** | 150 | 15s | 50 damage, area effect, pressure trigger |
| **Alarm Trap** | 30 | 3s | Alerts creatures in 10 tile radius |

---

### ✨ Spell System
Spells are data-driven via `assets/data/dungeon_spells.json`.

| Spell | Mana Cost | Category | Effect | Cooldown |
|---|---|---|---|---|
| **Lightning Strike** | 35 | Combat | 40 lightning damage (AoE 1) | 2s |
| **Heal Creature** | 25 | Utility | +50 HP to friendly | 3s |
| **Possession** | 60 (+10 HP) | Control | Directly control a creature for 30s | 30s |
| **Call to Arms** | 45 | Control | Force all creatures to rally point for 20s | 20s |
| **Corrupt Land** | 80 | Ritual | Transform claimed floor to corrupted floor (AoE 4) | 60s |
| **Dark Revelation** | 40 | Utility | Reveal map (AoE 8) for 20s | 25s |
| **Summon Imps** | 50 | Utility | Spawn 1 Imp | 15s |
| **Make Earth** | 40 | Construction | Create earth tile | 1s |

---

### 👹 Creature System
Creatures are data-driven via `assets/data/monsters.json`. Each creature has:

- **Stats**: Health, Mana, Attack, Defense, Speed, Carry Capacity, Sight Radius.
- **Needs**: Sleep, Food, Gold (with decay rates and rooms that satisfy them).
- **Traits**: e.g., `loyal`, `greedy`, `cowardly`, `undead`.
- **AI**: Base mood, anger/desertion thresholds, task preferences, room desires.
- **Combat**: Attack type (melee/magic/ranged), damage range, armor type, resistances.
- **Progression**: XP thresholds, stat growth per level, max level, possible mutations.
- **Economy**: Wage per minute, steals if unpaid flag, gold dropped on death.

**Example Creatures:**

| Creature | Role | HP | ATK | DEF | Speed | Max Level | Notes |
|---|---|---|---|---|---|---|---|
| **Imp** | Worker | 80 | 4 | 2 | 1.2 | 5 | Digging expert, low wage |
| **Goblin** | Fighter | 120 | 8 | 3 | 1.0 | 10 | Cowardly, may steal if unpaid |
| **Orc** | Fighter | 200 | 15 | 8 | 0.9 | 10 | Aggressive, high wage |
| **Warlock** | Mage | 100 | 5 | 2 | 0.8 | 10 | Magic attacks, prefers Library |
| **Troll** | Worker | 300 | 12 | 10 | 0.6 | 8 | Strong crafter, slow |
| **Skeleton** | Fighter | 90 | 7 | 4 | 1.1 | 8 | Undead: no sleep/food needs |
| **Demon Spawn** | Elite | 400 | 25 | 15 | 1.0 | 10 | Powerful but demanding |

**Needs-Based AI:**
Creatures prioritize tasks based on their needs (Sleep, Food, Gold) and personality. If a need drops below 30%, they seek rooms to satisfy it. Mood affects work efficiency and combat performance. Critically low mood leads to anger (slower work) or desertion (leaving dungeon).

---

### ⚔️ Combat System
Combat is resolved in real-time ticks in `src/engine/combat.rs`:

1. **Attack Calculation**: `attack_chance = attacks_per_second * delta_time`
2. **Damage Calculation**: `base_damage + (attack * 0.5) - (defense * 0.3)`
3. **Resistance Application**: Elemental resistances reduce damage (e.g., 50% magic resistance halves magic damage).
4. **Status Effects**: Abilities can apply statuses (e.g., `possessed`, `forced_move`).
5. **Death & XP**: Killing enemies grants XP. Creatures level up, gaining stats.

**Factions**: Dungeon creatures vs. Heroes. `wild` faction attacks everyone.

---

### 🦸 Hero System
Heroes invade from **Hero Portals** placed during map generation. Defined in `assets/data/heroes.json`.

**Hero Tiers:**
- **Tier 1 (Weak)**: Peasant Militia, Scout, Acolyte
- **Tier 2 (Standard)**: Knight, Archer, Battle Cleric, Rogue
- **Tier 3 (Strong)**: Paladin, Wizard, Inquisitor
- **Tier 4 (Elite)**: Knight Commander, High Priest, Archmage
- **Tier 5 (Boss)**: Champion of Light

---

## Installation & Running

Ensure you have Rust and Cargo installed.

```bash
# Generate graphics assets (first time only)
cargo run --bin build_graphics

# Run the game
cargo run --release
```

## Controls

### General
- **Left Click**: Select / Place / Interact.
- **Right Click / Escape**: Cancel action / Deselect.
- **Mouse Hover**: View information in the sidebar.

### Build Hotkeys
- `1`: **Dig**
- `2`: **Lair** (50 Gold/tile)
- `3`: **Hatchery** (75 Gold/tile)
- `4`: **Treasury** (100 Gold/tile)
- `T`: **Training Hall** (150 Gold/tile, requires research)
- `L`: **Library** (150 Gold/tile)
- `W`: **Workshop** (120 Gold/tile)
- `G`: **Guard Post** (90 Gold/tile)
- `P`: **Prison** (80 Gold/tile, requires research)
- `5`: **Monster Spawner**
- `X`: **Sell / Cancel**

### Trap Hotkeys
- `D`: **Door** (50 Gold)
- `S`: **Spike Trap** (100 Gold)

### Other
- `Tab` (Click): Switch Sidebar Tabs (Build, Magic, Minions, Traps).
