# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Deep Dominion** is a Dungeon Keeper-style god game/dungeon management simulator in development. This is currently a design-phase repository containing comprehensive game design documentation, JSON data schemas, and configuration files. No source code has been implemented yet.

**Tech Stack (Planned):**
- Language: Rust (Edition 2021)
- Rendering: Macroquad
- Architecture: ECS-based (likely using a lightweight ECS or custom implementation)
- Deployment: Native Windows + WebGL/WASM
- Data Format: JSON for all game content (rooms, monsters, heroes, spells)

## Repository Structure

### Design Documentation
- `gdd.md` - Core Game Design Document defining mechanics, features, and vision
- `CODE_STANDARDS.md` - Rust coding standards for Macroquad games (module structure, naming, patterns)
- `GAME_DEVELOPMENT_GUIDE.md` - Technical guide for Rust game development workflow
- `MACROQUAD_TOOLKIT.md` - Documentation for the `macroquad-toolkit` library used across projects

### Game Design Schemas
- `room_design.md` - JSON schema and design philosophy for room definitions
- `monster_design.md` / `monsters.md` - Monster/creature design schemas
- `hero_design.md` - Hero/adventurer design schemas and behaviors
- `dungeon_spell_design.md` - Spell system design

### Data Files (JSON)
- `rooms.json` - Room definitions (Lair, Hatchery, Treasury, Training Room, Library, etc.)
- `monsters.json` - Creature definitions (Imp, Goblin, Troll, Warlock, Demon, etc.)
- `heroes.json` - Hero/adventurer definitions (Knights, Archers, Wizards, etc.)
- `dungeon_spells.json` - Spell definitions (Speed, Heal, Lightning, etc.)
- `tiles.json` - Tile type definitions

### Build/Deploy
- `publish.ps1` - PowerShell script for building and deploying to Windows and WebGL
- `index.html` - WebGL host page template

## Core Game Architecture (Planned)

### ECS Systems (from GDD)
The game will use an ECS architecture with these planned systems:
- AI Decision System
- Pathfinding System (A* on tile grid)
- Room Validation System
- Economy Tick System
- Combat Resolution System

### Key Game Concepts

**Tile-Based Isometric Grid:**
- Each tile has terrain type, ownership, room association, and fog-of-war state
- Tile types: Solid Rock, Earth, Gold Vein, Gems, Reinforced Wall, Claimed Floor

**Room System:**
- Rooms are contiguous claimed tiles of the same type
- Room quality based on size, shape, and placed objects
- Rooms are entirely data-driven from JSON (no hardcoded room logic)

**Creature AI:**
- Creatures have Needs (Sleep, Food, Gold, Training)
- Creatures have Traits (Greedy, Lazy, Aggressive, Loyal)
- Creatures choose tasks autonomously based on mood and needs
- Player influences creatures indirectly, never controls them directly

**Hero AI:**
- Goal-driven behavior (destroy_heart, steal_gold, kill_creatures)
- Room priorities and threat response logic
- Heroes test dungeon design weaknesses

## Development Commands (When Source Code Exists)

### Building
```powershell
# Windows native build
cargo build --release

# WebGL/WASM build
cargo build --release --target wasm32-unknown-unknown

# Using publish script (builds both + deploys)
.\publish.ps1                    # Preview deployment
.\publish.ps1 -Production        # Production deployment
.\publish.ps1 -WebGLOnly         # Skip Windows build
.\publish.ps1 -WindowsOnly       # Skip WebGL build
.\publish.ps1 -SkipBuild         # Deploy existing builds
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests for specific module
cargo test module_name::
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check without building
cargo check
```

## Coding Standards (Critical Rules)

These standards from `CODE_STANDARDS.md` are strictly enforced:

### Project Structure
- `main.rs` - Entry point, game loop, phase transitions
- `data/` - Data structures and JSON loading (no game logic)
- `engine/` - Stateless game logic services
- `state/` - Game state management
- `ui/` - UI components (macroquad-toolkit based)
- `screens/` - Screen-specific rendering (optional)

### Data-Driven Design
- ALL game content defined in JSON under `assets/`
- Load data at startup; data is immutable after loading
- Never hardcode magic numbers or balance values
- Use structs that mirror JSON structure for type safety

### No Unused Code
- Remove unused variables, fields, and functions immediately
- Never use `_` prefix on struct fields to suppress warnings
- If a field is unused, delete it completely
- No backwards-compatibility hacks (renamed `_vars`, re-exports, `// removed` comments)

### Module Boundaries
- UI must never mutate game state directly (returns `Option<UiAction>` instead)
- Engine services are stateless - receive state, return results
- Data module has no knowledge of engine or UI
- State mutations happen only in main.rs via clearly defined actions

### File Size Limits
- Target: 200-400 lines per file
- Soft limit: 600 lines
- Hard limit: 800 lines (except main.rs)

## Data Schema Patterns

All game entities follow consistent JSON schema patterns:

### Common Structure
```json
{
  "id": "unique_identifier",
  "name": "Display Name",
  "description": "Flavor text",
  "stats": { },
  "ai": { },
  "visual": { }
}
```

### Room Schema Sections
- `build` - Construction costs and requirements
- `requirements` - Research and unlock conditions
- `effects` - Passive gameplay effects
- `scaling` - Size and shape bonuses/penalties
- `ai` - How creatures perceive and use the room
- `visual` - Rendering data (zero gameplay impact)

### Creature/Hero Schema Sections
- `stats` - Health, attack, defense, speed, etc.
- `ai` - Decision-making, desirability, task preferences
- `combat` - Attack type, damage ranges, resistances
- `abilities` - Active and passive abilities
- `behavior` - Environmental interactions
- `visual` - Sprites, animations, voice sets

## Important Design Principles

### Indirect Control Philosophy
The player never directly commands units. Instead:
- Shape space (dig, build rooms)
- Set priorities (room placement, gold allocation)
- Exploit creature psychology (slapping, dropping, room desirability)

### Emergent Behavior
- No special-case creature or room logic
- AI systems read data, don't hardcode behaviors
- Room effects are passive, evaluated during simulation ticks
- Keep individual values small, let size and scaling do the work

### Simulation Architecture
- Fixed timestep (10-20 ticks/second planned)
- Deterministic for replay/debugging
- Cached pathfinding per room

## When Implementing Source Code

### Initial Setup Checklist
1. Create standard Rust/Macroquad folder structure (src/data/, src/engine/, src/state/, src/ui/)
2. Implement JSON loaders for all data files first
3. Set up GameState and StateTransition enums
4. Create Game struct with update/draw loop
5. Implement basic tile rendering (isometric projection)
6. Build room validation system
7. Implement creature AI decision system
8. Add pathfinding (A*)

### Common Patterns to Use

**State Machine:**
```rust
pub enum GamePhase {
    Loading,
    MainMenu,
    Playing,
    Paused,
    GameOver,
}
```

**UI Action Pattern:**
```rust
pub enum UiAction {
    DigTiles(Vec<TilePos>),
    BuildRoom(RoomType, Vec<TilePos>),
    SlapCreature(EntityId),
    CastSpell(SpellId, Target),
}
```

**Service Pattern:**
```rust
// Engine services are stateless
pub fn calculate_room_efficiency(room: &Room, tiles: &[Tile]) -> f32 {
    // Pure function, no hidden state
}
```

### What NOT to Do
- Don't create ECS overengineering before core features work
- Don't create custom editor tooling initially
- Don't implement procedural generation before core gameplay is stable
- Don't add features beyond what's defined in the GDD
- Don't hardcode creature behaviors or room effects

## Testing Focus Areas

When writing tests, prioritize:
- JSON data loading and validation
- Room shape detection and efficiency calculations
- Creature AI decision trees
- Pathfinding correctness
- Combat damage calculations
- State machine transitions

UI and rendering generally do not need unit tests.

## MVP Scope (from GDD)

The Minimum Viable Product should include:
- Digging and claiming tiles
- 4 core rooms (Lair, Hatchery, Treasury, Heart)
- 3 creature types (Imp, basic fighter, basic researcher)
- Basic hero invasion AI
- One playable map

Features explicitly out of scope for MVP:
- Full possession mode
- Advanced fluid simulation
- Modding support
- Multiplayer

## Notes for AI Assistants

- This project is in design phase - expect to be creating initial implementations from scratch
- All game data is already defined in JSON files - use those as the source of truth
- Follow the coding standards strictly - they exist for consistency across multiple Macroquad games
- The GDD defines the complete feature set - don't add features beyond what's documented
- When in doubt about architecture, refer to `GAME_DEVELOPMENT_GUIDE.md` for patterns
- The project uses `macroquad-toolkit` from a sibling directory - see `MACROQUAD_TOOLKIT.md` for available utilities
