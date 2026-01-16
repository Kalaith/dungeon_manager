# Feature Gap Analysis (vs. Game Design Notes)

Based on a review of `game_design_notes.md` and the current codebase (`src/data`, `src/engine`, `assets/data`), here is the status of planned features.

## ✅ Implemented Features
- **Core Loop**: Digging, Room Construction, Resource Gathering (Gold), Creature Attraction (Portal/Spawner).
- **Room Typology**:
  - **Lair** (Sleep)
  - **Hatchery** (Food)
  - **Treasury** (Gold Storage)
  - **Training Hall** (XP)
  - **Library** (Research rate - *see Gaps*)
  - **Workshop** (Trap production)
  - **Prison** (Hero conversion rate)
  - **Guard Post** (Defense bonus)
  - **Ritual Circle** (Mana generation)
- **Creature AI**:
  - **Needs**: Sleep, Food, Gold, Happiness (Mood).
  - **Behaviors**: Work, Train, Research, Guard, Flee, Desert.
  - **Interactions**: "Slap" discipline mechanic (right-click).
- **Control Interface**:
  - Hand cursor logic for **Pickup/Drop** and **Slap**.
  - Unified Sidebar for building and management.

## ⚠️ Missing / Incomplete Features

### 1. Research & Tech Tree UI
- **Status**: Backend support exists (rooms have `research` requirements, Library generates `research_points`), but there is **no UI** to view or select technologies to research.
- **Gap**: Players cannot unlock new rooms (like Training Hall) that require tech (`training_tech`) because there is no interface to spend research points.

### 2. Temple & Sacrifice Mechanics
- **Status**: `Ritual Circle` exists but functions as a passive mana generator.
- **Gap**: The notes mention "Sacrifice mechanics" (dropping creatures to kill them for bonuses). This specific interaction is not implemented.

### 3. Torture Mechanics
- **Status**: `Prison` exists with a passive `hero_conversion_rate`.
- **Gap**: Notes mention "information extraction" and active torture. There is no interactive "Torture" task or "Torture Chamber" room distinct from the Prison (or interactive torture within the Prison).

### 4. Advanced Traps
- **Status**: Basic `Door` and `Spike Trap` are implemented.
- **Gap**: Notes mention "Lightning", "Boulder", and "Alarm" traps. These are missing from `traps.json` and `sidebar.rs`.

### 5. Overworld / Raiding
- **Status**: Non-existent.
- **Gap**: Notes mention active RTS-style overworld management (Dungeons 3/4 style). This is a major scope item currently completely missing.

### 6. Visual Polishing (Theming)
- **Status**: Functional.
- **Gap**: "Warped geometry" and "Industrial sound design" mentioned in notes are likely not fully realized in the current Macroquad rendering, though basic lighting exists.

## Recommendations
Prioritize **#1 (Research UI)** as it blocks progression for advanced rooms. Then consider **#2 (Sacrifice)** or **#4 (More Traps)** to flesh out the "Evil Dungeon" fantasy.
