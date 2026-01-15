# Deep Dominion (Dungeon Manager)

A dungeon management simulation game built in Rust using the Macroquad engine. Become the Dungeon Overseer, build your layout, manage your minions, and defend against invading heroes!

## Features

### 🏰 Dungeon Building
- **Digging**: Carve out your dungeon from solid rock.
- **Rooms**: Construct various rooms to attract and support your minions:
  - **Lair**: Where your creatures sleep.
  - **Hatchery**: Provides food (chickens) for your creatures.
  - **Treasury**: Stores your mined gold.
  - **Training Room**: Allows creatures to gain experience.
  - **Library**: Research new spells.
  - **Workshop**: Manufacture traps and doors.
  - **Guard Post**: Station creatures to defend key areas.
  - **Prison**: Capture invading heroes.
- **Traps & Doors**: Fortify your dungeon with Doors and Spike Traps.

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
- ** Inspection**: View detailed stats of your units and rooms.

## Installation & Running

Ensure you have Rust and Cargo installed.

```bash
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
- `2`: **Lair** (Cost: Sleep)
- `3`: **Hatchery** (Cost: Food)
- `4`: **Treasury** (Cost: Gold Storage)
- `T`: **Training Room**
- `L`: **Library**
- `W`: **Workshop**
- `G`: **Guard Post**
- `P`: **Prison**
- `5`: **Monster Spawner**
- `X`: **Sell / Cancel**

### Trap Hotkeys
- `D`: **Door**
- `S`: **Spike Trap**

### Other
- `Tab` (Click): Switch Sidebar Tabs (Build, Magic, Minions, Traps).
