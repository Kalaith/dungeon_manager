# Deep Dominion Implementation Plan

## Overview

Implement Deep Dominion, a Dungeon Keeper-style dungeon management game in Rust + Macroquad. The game is fully data-driven from JSON files (tiles, rooms, monsters, heroes, spells) with strict SOLID architecture.

**Tech Stack**: Rust + Macroquad + macroquad-toolkit
**Architecture**: ECS-inspired with stateless engine services
**Target**: 26 files, average ~320 lines/file (200-400 target, 800 hard limit)

## Core Architectural Principles

1. **Data-Driven**: All game content in 5 JSON files (tiles, rooms, monsters, heroes, spells)
2. **Stateless Services**: All `engine/` functions are pure - receive state, return results
3. **UI Action Pattern**: UI returns `Option<UiAction>`, never mutates state
4. **State Mutations**: Only in main.rs via `handle_ui_action()`
5. **File Size Discipline**: 200-400 lines target, 600 soft limit, 800 hard limit
6. **A* Modularity**: Pathfinding designed for eventual extraction to macroquad-toolkit

## Module Structure

```
src/
├── main.rs                    # Game loop, state transitions
├── data/                      # Data structures + JSON loading
│   ├── mod.rs                # GameData container
│   ├── tiles.rs              # TileData + loader
│   ├── rooms.rs              # RoomData + loader
│   ├── monsters.rs           # MonsterData + loader
│   ├── heroes.rs             # HeroData + loader
│   └── spells.rs             # SpellData + loader
├── engine/                    # Stateless game logic services
│   ├── mod.rs
│   ├── tile_grid.rs          # Grid ops, isometric math
│   ├── room_validator.rs     # Shape detection, quality calc
│   ├── pathfinding.rs        # A* (modular for toolkit)
│   ├── creature_ai.rs        # Creature decision-making
│   ├── hero_ai.rs            # Hero goal-driven behavior
│   ├── combat.rs             # Combat resolution
│   ├── economy.rs            # Gold/mana/wages tick
│   └── spell_effects.rs      # Spell effect application
├── state/                     # Game state management
│   ├── mod.rs
│   ├── game_state.rs         # Main state container
│   ├── tile_state.rs         # Runtime tile data
│   ├── entities.rs           # Entity management
│   └── player_state.rs       # Player resources, unlocks
└── ui/                        # UI components
    ├── mod.rs
    ├── core.rs               # Colors, fonts, layouts
    ├── hud.rs                # Resource display, minimap
    ├── room_menu.rs          # Room selection panel
    ├── creature_panel.rs     # Creature info display
    └── spells_bar.rs         # Spell quick-access
```

## Implementation Phases

### Phase 1: Project Setup & Data Loading (Foundation)
**Goal**: Load all JSON data into type-safe structs

**Files to Create** (9 files, ~1750 lines total):
1. `Cargo.toml` - Dependencies: macroquad, macroquad-toolkit, serde, serde_json, rand
2. `src/main.rs` (~150 lines) - Window config, game loop skeleton, GamePhase enum
3. `src/data/mod.rs` (~50 lines) - GameData container, re-exports
4. `src/data/tiles.rs` (~150 lines) - TileData struct, loader
5. `src/data/rooms.rs` (~300 lines) - RoomData struct, loader
6. `src/data/monsters.rs` (~350 lines) - MonsterData struct, loader
7. `src/data/heroes.rs` (~350 lines) - HeroData struct, loader
8. `src/data/spells.rs` (~200 lines) - SpellData struct, loader
9. `publish.ps1` + `index.html` - Build/deploy scripts

**Key Types**:
```rust
pub struct GameData {
    pub tiles: HashMap<String, TileData>,
    pub rooms: HashMap<String, RoomData>,
    pub monsters: HashMap<String, MonsterData>,
    pub heroes: HashMap<String, HeroData>,
    pub spells: HashMap<String, SpellData>,
}

pub enum GamePhase {
    Loading,
    MainMenu,
    Playing(GameState),
}
```

**Testing**: Verify all 5 JSON files load successfully

---

### Phase 2: Tile Grid System (Core Rendering)
**Goal**: Tile-based grid with isometric rendering, fog-of-war, ownership

**Files to Create** (4 files, ~1100 lines total):
1. `src/state/tile_state.rs` (~250 lines) - TileState runtime data
2. `src/engine/tile_grid.rs` (~400 lines) - Grid creation, isometric projection
3. `src/state/game_state.rs` (~300 lines) - Main GameState container
4. `src/ui/core.rs` (~150 lines) - Color schemes, layout constants

**Key Types**:
```rust
pub struct TileState {
    pub tile_type: String,
    pub pos: TilePos,
    pub ownership: Ownership,  // Unclaimed/Player/Enemy
    pub room_id: Option<usize>,
    pub fog_state: FogState,   // Hidden/Revealed/Visible
    pub resources_remaining: Option<u32>,
}

pub struct GameState {
    pub grid: Vec<Vec<TileState>>,
    pub entities: HashMap<EntityId, Entity>,
    pub rooms: Vec<Room>,
    pub player: PlayerState,
}
```

**Key Functions**:
- `world_to_iso(Vec2) -> Vec2` - Isometric projection
- `screen_to_tile(Vec2, &Camera) -> Option<TilePos>` - Mouse picking
- `get_neighbors(&Grid, TilePos) -> Vec<TilePos>` - 8-way connectivity

**Testing**: Render checkerboard grid, verify coordinate conversion roundtrip

---

### Phase 3: Room System (Validation & Quality)
**Goal**: Detect room shapes, calculate quality, validate placement

**Files to Create** (2 files, ~700 lines total):
1. `src/engine/room_validator.rs` (~500 lines) - Shape detection, quality calc
2. `src/state/player_state.rs` (~200 lines) - Player resources, research, unlocks

**Key Functions**:
- `detect_rooms(&Grid, room_type) -> Vec<HashSet<TilePos>>` - Flood-fill contiguous tiles
- `validate_room_shape(&HashSet<TilePos>, &RoomData) -> Result<(), String>` - Check size/shape
- `calculate_room_quality(&Room, &RoomData) -> f32` - Size thresholds + shape penalties
- `calculate_shape_metrics(&HashSet<TilePos>) -> ShapeMetrics` - Aspect ratio, compactness

**Key Types**:
```rust
pub struct Room {
    pub id: usize,
    pub room_type: String,
    pub tiles: HashSet<TilePos>,
    pub quality: f32,
    pub active: bool,
}

pub struct ShapeMetrics {
    pub aspect_ratio: f32,
    pub compactness: f32,
    pub is_fragmented: bool,
    pub is_thin_corridor: bool,
}
```

**Testing**: Validate L-shapes, squares, corridors; verify quality calculation

---

### Phase 4: Pathfinding System (A* with Caching)
**Goal**: A* pathfinding designed for eventual toolkit extraction

**Files to Create** (1 file, ~600 lines):
1. `src/engine/pathfinding.rs` (~600 lines) - Self-contained A* implementation

**Design Note**: Keep completely modular with zero game-specific dependencies for easy migration to macroquad-toolkit later.

**Key Types**:
```rust
pub struct PathfindingGrid {
    pub width: usize,
    pub height: usize,
    pub walkable: Vec<Vec<bool>>,
    pub cost: Vec<Vec<f32>>,
}

pub struct Path {
    pub waypoints: Vec<TilePos>,
    pub total_cost: f32,
}

pub struct PathCache {
    cache: HashMap<(TilePos, TilePos), Path>,
    invalidation_tracker: HashSet<TilePos>,
}
```

**Key Functions**:
- `find_path(start, goal, &PathfindingGrid) -> Option<Path>` - Classic A*
- `find_path_cached(start, goal, &mut PathCache, &PathfindingGrid) -> Option<Path>` - With cache
- `invalidate_paths(&mut PathCache, &[TilePos])` - Clear affected paths

**Testing**: Straight lines, obstacles, unreachable goals, cache hit/miss rates

---

### Phase 5: Entity System & Creature AI
**Goal**: Creatures with needs-based AI and task selection

**Files to Create** (2 files, ~1000 lines total):
1. `src/state/entities.rs` (~400 lines) - Entity management, creature/hero state
2. `src/engine/creature_ai.rs` (~600 lines) - Creature decision-making

**Key Types**:
```rust
pub struct Entity {
    pub id: EntityId,
    pub entity_type: EntityType,
    pub pos: TilePos,
}

pub enum EntityType {
    Creature(CreatureState),
    Hero(HeroState),
}

pub struct CreatureState {
    pub creature_id: String,
    pub level: u32,
    pub health: f32,
    pub needs: HashMap<String, f32>,  // 0-100 per need
    pub mood: f32,                     // 0-100
    pub current_task: Option<Task>,
}

pub enum Task {
    Dig(TilePos),
    Work(RoomId),
    Sleep(RoomId),
    Eat(RoomId),
    Train(RoomId),
}
```

**Key Functions**:
- `decide_task(&CreatureState, &GameState, &GameData) -> Option<Task>` - Evaluate needs → task
- `calculate_task_desirability(&Task, &CreatureState, &RoomData) -> f32` - Priority scoring
- `find_best_room(room_type, pos, &GameState) -> Option<RoomId>` - Distance + quality
- `update_needs(&mut CreatureState, dt, &MonsterData)` - Decay over time
- `should_desert(&CreatureState, &MonsterData) -> bool` - Mood check

**Testing**: Need decay, mood calculation, task prioritization, desertion triggers

---

### Phase 6: Hero AI & Combat
**Goal**: Goal-driven hero AI and combat resolution

**Files to Create** (2 files, ~900 lines total):
1. `src/engine/hero_ai.rs` (~500 lines) - Hero goal selection, threat response
2. `src/engine/combat.rs` (~400 lines) - Combat resolution

**Key Functions**:
- `decide_hero_goal(&HeroState, &GameState, &GameData) -> HeroGoal` - Primary/secondary goals
- `find_target_room(&HeroState, &HeroGoal, &GameState) -> Option<RoomId>` - Room priorities
- `evaluate_threat(&HeroState, &GameState) -> ThreatLevel` - Nearby creature count
- `resolve_combat_tick(&Entity, &Entity, dt, &GameData) -> CombatResult` - Damage calc
- `calculate_damage(&CombatStats, &CombatStats) -> f32` - Attack/defense/resistances

**Key Types**:
```rust
pub enum HeroGoal {
    DestroyHeart,
    StealGold(i32),
    KillCreatures(u32),
    SabotageRoom(RoomId),
}

pub struct CombatResult {
    pub damage_dealt: f32,
    pub status_applied: Vec<StatusEffect>,
    pub defender_died: bool,
}
```

**Testing**: Goal selection, retreat logic, damage formulas, death handling

---

### Phase 7: Economy & Spell Systems
**Goal**: Resource generation and spell casting

**Files to Create** (2 files, ~700 lines total):
1. `src/engine/economy.rs` (~300 lines) - Gold/mana generation, wages
2. `src/engine/spell_effects.rs` (~400 lines) - Spell effect application

**Key Functions**:
- `tick_economy(&mut GameState, dt, &GameData)` - Mana, food, gold mining, wages
- `calculate_mana_generation(&[Room], &GameData) -> f32` - Room effects
- `pay_creature_wages(&[CreatureState], &mut PlayerState) -> Vec<EntityId>` - Unpaid list
- `cast_spell(spell_id, target, &mut GameState, &GameData) -> Result<(), String>` - Validate & apply
- `apply_damage_effect(EntityId, f32, &str, &mut GameState)` - Damage spell
- `apply_tile_transform(&[TilePos], from, to, &mut GameState)` - Corrupt Land

**Testing**: Mana accumulation, wage payment, spell targeting, effect application

---

### Phase 8: Player Interaction & UI
**Goal**: Complete UI and player controls

**Files to Create** (5 files, ~1600 lines total):
1. `src/ui/hud.rs` (~400 lines) - Resource display, minimap
2. `src/ui/room_menu.rs` (~350 lines) - Room selection panel
3. `src/ui/spells_bar.rs` (~250 lines) - Spell quick-access
4. `src/ui/creature_panel.rs` (~300 lines) - Creature info display
5. Update `src/main.rs` (+450 lines) - UI integration, action handling, Hand of Evil

**Key Types**:
```rust
pub enum UiAction {
    SelectRoom(String),
    MarkTileForDig(TilePos),
    BuildRoom(String, Vec<TilePos>),
    PickUpCreature(EntityId),
    DropCreature(EntityId, TilePos),
    SlapCreature(EntityId),
    CastSpell(String, Target),
}
```

**Key Functions in main.rs**:
- `handle_ui_action(UiAction, &mut GameState, &GameData)` - Dispatch to state mutations
- `handle_hand_of_evil(&mut GameState, &InputState) -> Option<UiAction>` - Pick/drop/slap
- `handle_digging_mode(&mut GameState, &InputState) -> Vec<UiAction>` - Click-drag tiles
- `handle_building_mode(room_id, &mut GameState, &InputState) -> Option<UiAction>` - Validate placement

**Testing**: UI actions → state mutations, room building validation, digging marks, spell casting

---

## MVP Scope (Prioritized Features)

### MVP Phase 1: Basic Dungeon Functionality
- Digging and claiming tiles (Imp task system)
- 4 core rooms: Heart, Lair, Hatchery, Treasury
- 3 creatures: Imp, Goblin, Orc
- Basic needs: sleep, food, gold
- Room quality calculation
- Simple pathfinding

### MVP Phase 2: Combat & Heroes
- 3 basic heroes: Peasant, Knight, Archer
- Combat resolution
- Hero AI: destroy heart goal
- Creature combat response

### MVP Phase 3: Economy & Polish
- Gold mining from veins
- Wage payment system
- Mana generation
- 2 spells: Lightning, Heal
- Hand of Evil: pickup/drop/slap
- Complete HUD

---

## Critical Files for Implementation

These 5 files are the architectural cornerstones:

1. **H:\RustGames\dungeon_manager\src\data\mod.rs**
   Central data loader, orchestrates all JSON loading

2. **H:\RustGames\dungeon_manager\src\engine\tile_grid.rs**
   Core grid operations, isometric math, used by every system

3. **H:\RustGames\dungeon_manager\src\engine\pathfinding.rs**
   Critical for movement, designed for toolkit extraction, complex algorithm

4. **H:\RustGames\dungeon_manager\src\engine\creature_ai.rs**
   Core gameplay loop (needs → decisions → tasks), most complex AI

5. **H:\RustGames\dungeon_manager\src\state\game_state.rs**
   Central state container, simulation tick orchestration

---

## File Size Compliance

**Target**: 200-400 lines/file
**Soft Limit**: 600 lines
**Hard Limit**: 800 lines (main.rs excepted)

**Breakdown**:
- 24/26 files under soft limit (600 lines)
- 2/26 files at hard limit (pathfinding.rs, creature_ai.rs - both justified)
- 0 files over hard limit
- **Average**: ~320 lines/file

Files at hard limit are intentionally designed as cohesive modules:
- `pathfinding.rs` (~600 lines): A* algorithm with caching, designed for toolkit extraction
- `creature_ai.rs` (~600 lines): Complete decision tree for creature behavior

---

## Testing Strategy

### Unit Tests (Per-Phase)
- **Phase 1**: JSON loading, deserialization errors
- **Phase 2**: Isometric coordinate conversion, neighbor detection
- **Phase 3**: Room shape detection, quality formulas
- **Phase 4**: A* correctness, cache hit rates
- **Phase 5**: Need decay, mood calculation, task selection
- **Phase 6**: Damage formulas, retreat logic
- **Phase 7**: Resource accumulation, spell validation
- **Phase 8**: UI action dispatch

### Integration Tests
- Full game loop runs without crashes
- Entity spawning → pathfinding → room arrival
- Combat tick → damage → death
- Economy tick → wage payment → stealing behavior

### Visual Tests
- Render checkerboard grid
- Creature movement along paths
- Room highlighting
- Fog-of-war updates

---

## Critical Design Patterns

### 1. Stateless Engine Services
```rust
// WRONG: Stateful service
struct CombatEngine {
    results: Vec<CombatResult>,
}

// RIGHT: Stateless service
fn resolve_combat(attacker: &Entity, defender: &Entity) -> CombatResult {
    // Pure function
}
```

### 2. UI Action Pattern
```rust
// UI returns actions, never mutates state
fn draw_room_menu(player: &PlayerState) -> Option<UiAction> {
    if button_clicked {
        return Some(UiAction::SelectRoom("lair".to_string()));
    }
    None
}

// main.rs handles mutations
fn handle_ui_action(action: UiAction, state: &mut GameState) {
    match action {
        UiAction::SelectRoom(id) => {
            state.selected_room = Some(id);
        }
    }
}
```

### 3. Data-Driven Design
```rust
// WRONG: Hardcoded behavior
if room.room_type == "hatchery" {
    food_rate = 0.5;
}

// RIGHT: Data-driven
let room_data = game_data.rooms.get(&room.room_type)?;
let food_rate = room_data.effects.food_generation_per_second;
```

### 4. A* Modularity (for toolkit extraction)
```rust
// Zero game-specific dependencies
pub struct PathfindingGrid {
    pub width: usize,
    pub height: usize,
    pub walkable: Vec<Vec<bool>>,  // Generic grid
}

// TilePos is just (x, y), no game logic
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}
```

---

## Verification & Testing

### Phase Completion Criteria

Each phase is complete when:
1. All files created and under size limits
2. Code compiles without warnings
3. Unit tests pass
4. Integration test for phase works
5. Visual verification (where applicable)

### End-to-End Testing

After all 8 phases:
1. **Load Test**: All 5 JSON files load successfully
2. **Render Test**: Grid renders with isometric projection
3. **Interaction Test**: Click tiles, select rooms, build
4. **AI Test**: Spawn creatures, verify they move to rooms
5. **Combat Test**: Spawn hero, verify combat resolves
6. **Economy Test**: Verify gold/mana accumulate over time
7. **Performance Test**: 100 entities at 60 FPS

### Build Verification

```powershell
# Windows build
cargo build --release

# WebGL build
cargo build --release --target wasm32-unknown-unknown

# Run tests
cargo test

# Check code quality
cargo fmt
cargo clippy
```

---

## Risk Mitigation

### Risk 1: File Size Creep
**Mitigation**: Track line counts per commit, split files proactively at 500 lines

### Risk 2: GameState God Object
**Mitigation**: Keep GameState as container only, all logic in engine/ services

### Risk 3: A* Coupling to Game
**Mitigation**: Design pathfinding.rs with zero game imports from day 1, verify it compiles standalone

### Risk 4: Data Loading Complexity
**Mitigation**: Each JSON file gets own loader module with clear error messages

### Risk 5: ECS Overengineering
**Mitigation**: Use simple HashMap<EntityId, Entity> initially, migrate to ECS only if needed

---

## Dependencies

- macroquad = "0.4"
- macroquad-toolkit (path = "../macroquad-toolkit")
- serde = { version = "1.0", features = ["derive"] }
- serde_json = "1.0"
- rand = "0.8"

---

## Next Steps

1. Create Cargo.toml with dependencies
2. Implement Phase 1 (Data Loading)
3. Verify all JSON files load
4. Implement Phase 2 (Tile Grid)
5. Visual test: render grid
6. Continue phases 3-8 sequentially
7. MVP testing and polish
8. Post-MVP features
