# Deep Dominion - Revised MVP Plan

## Current State (After 5 Phases)

**What Works:**
- ✅ All JSON data loads correctly (tiles, rooms, monsters, heroes, spells)
- ✅ 50x50 isometric grid renders with proper colors
- ✅ Mouse hover and tile selection
- ✅ Dig mode: Click earth tiles to claim them (turns green)
- ✅ Build mode: Place room tiles (Lair/Hatchery/Treasury)
- ✅ Camera controls (WASD)
- ✅ Room validation system exists (flood-fill, shape metrics, quality)
- ✅ A* pathfinding implemented and tested
- ✅ Entity system exists (CreatureState, HeroState, EntityManager)
- ✅ Creature AI exists (needs, mood, task selection)

**What's Missing:**
- ❌ **Nothing moves** - entities are static
- ❌ **Nothing happens** - no game loop simulation
- ❌ **Rooms don't work** - just colored tiles
- ❌ **No imps** - player can't actually dig, must click manually
- ❌ **No heroes** - no threat/challenge
- ❌ **No combat** - even though systems exist
- ❌ **No economy** - gold/mana don't matter

## Problem: Original Plan vs Reality

The original plan focused on **building systems** rather than **making a game**:
- Phases 1-5: Build all the infrastructure ✅
- Phases 6-8: Add more systems (combat, economy, UI)
- **No focus on making things actually work together**

## New Approach: Make It Playable First

Instead of completing phases 6-8, let's **wire up what we have** into a minimal gameplay loop.

---

## Revised MVP Roadmap

### MVP 1: "The Basics Work" (Next Priority)

**Goal:** Creatures move, imps dig, rooms function, basic game loop

**Tasks:**
1. **Spawn imps automatically** when player has claimed tiles
   - Add `spawn_imp()` in game_state.rs tick
   - Imps spawn at dungeon heart / claimed floor

2. **Make imps dig marked tiles**
   - Add `marked_for_dig` to TileState (already exists)
   - Digging mode marks tiles instead of instantly claiming
   - Imps pathfind to marked tiles and dig them over time

3. **Make creatures move and use pathfinding**
   - Creatures select task (already implemented in creature_ai)
   - Find path to task location using A*
   - Move along path each tick
   - Arrive at room and perform task

4. **Make rooms actually work**
   - Lair: Creatures sleep, restore sleep need
   - Hatchery: Creatures eat, restore food need
   - Treasury: Creatures deposit gold, restore gold need
   - Detect rooms automatically when tiles are built

5. **Add visible creature movement**
   - Render creatures as circles/sprites
   - Smooth interpolation between tiles
   - Show task icons above heads

**Result:** You can mark tiles for dig, imps spawn and dig them, creatures walk to rooms and use them.

---

### MVP 2: "There's Challenge" (Second Priority)

**Goal:** Heroes invade, combat happens, you can lose

**Tasks:**
1. **Spawn heroes periodically**
   - Timer-based spawning at map edges
   - Start easy (peasants), ramp up difficulty

2. **Make heroes pathfind to dungeon heart**
   - Use hero_ai.rs `decide_hero_goal()`
   - Pathfind toward heart
   - Attack anything in the way

3. **Implement combat**
   - Use combat.rs `resolve_combat_tick()`
   - Apply damage when entities are adjacent
   - Death removes entity, updates stats

4. **Add health bars**
   - Draw health bar above creatures/heroes
   - Visual feedback for combat

5. **Win/Lose conditions**
   - Lose: Dungeon heart destroyed
   - Win: Survive X waves of heroes

**Result:** Heroes invade, fight your creatures, destroy your heart if undefended. Actual challenge.

---

### MVP 3: "Strategic Choices Matter" (Third Priority)

**Goal:** Economy, meaningful decisions, progression

**Tasks:**
1. **Add gold mining**
   - Creatures mine gold veins when idle
   - Gold accumulates in treasury
   - Display gold count in HUD

2. **Add mana generation**
   - Dungeon heart generates mana over time
   - Display mana count in HUD

3. **Add creature wages**
   - Creatures cost gold per minute
   - If unpaid, they desert or steal

4. **Add room costs**
   - Building rooms costs gold
   - Can't build if can't afford

5. **Add basic spells**
   - Heal: Click creature, spend mana, heal them
   - Speed: Click creature, speed buff
   - Lightning: Click hero, damage them

**Result:** Managing gold/mana matters, strategic room placement, spells give player agency.

---

## Implementation Order (Wire Up What Exists)

### Step 1: Connect Entity Movement (1-2 hours)
**File:** `src/state/game_state.rs` - Update `tick()`

```rust
fn tick(&mut self, game_data: &GameData) {
    // For each creature with a task
    for entity in self.entities.all_mut() {
        if let Some(creature) = entity.as_creature_mut() {
            if let Some(task) = &creature.current_task {
                // Get target position for task
                let target = get_task_target_position(task, &self.rooms);

                // Pathfind to target
                if entity.pos != target {
                    // Move one step toward target
                    move_entity_toward(entity, target);
                }
            } else {
                // No task, decide one
                creature.current_task = creature_ai::decide_task(
                    creature, entity.pos, self, game_data
                );
            }
        }
    }
}
```

### Step 2: Imp Auto-Digging (1 hour)
**File:** `src/main.rs` - Change dig mode behavior

```rust
InteractionMode::Dig => {
    if let Some(tile) = state.get_tile_mut(hovered_tile) {
        if tile.tile_type == "earth" {
            tile.marked_for_dig = true; // Mark, don't instantly dig
        }
    }
}
```

**File:** `src/state/game_state.rs` - Spawn imps

```rust
fn tick(&mut self, game_data: &GameData) {
    // Spawn imp if needed and marked tiles exist
    if self.has_marked_tiles() && self.count_imps() < 5 {
        self.spawn_imp(game_data);
    }

    // Imps work on marked tiles
    for entity in self.entities.all_mut() {
        if let Some(creature) = entity.as_creature_mut() {
            if creature.creature_id == "imp" {
                if let Some(marked_pos) = find_nearest_marked_tile(entity.pos, &self.grid) {
                    // Pathfind and dig
                    if entity.pos == marked_pos {
                        // Dig!
                        if let Some(tile) = self.get_tile_mut(marked_pos) {
                            tile.tile_type = "claimed_floor".to_string();
                            tile.ownership = Ownership::Player;
                            tile.marked_for_dig = false;
                        }
                    } else {
                        // Move toward it
                        move_entity_toward(entity, marked_pos);
                    }
                }
            }
        }
    }
}
```

### Step 3: Room Detection & Function (1 hour)
**File:** `src/state/game_state.rs` - Detect rooms in tick

```rust
fn tick(&mut self, game_data: &GameData) {
    // Detect new rooms when tiles change
    self.detect_and_update_rooms(game_data);

    // Creatures in rooms satisfy needs
    for entity in self.entities.all() {
        if let Some(creature) = entity.as_creature() {
            if let Some(tile) = self.get_tile(entity.pos) {
                if let Some(room_id) = tile.room_id {
                    if let Some(room) = self.rooms.iter().find(|r| r.id == room_id) {
                        satisfy_creature_needs(creature, &room.room_type, dt);
                    }
                }
            }
        }
    }
}
```

### Step 4: Hero Spawning & Combat (2 hours)
**File:** `src/state/game_state.rs`

```rust
fn tick(&mut self, game_data: &GameData) {
    // Spawn heroes periodically
    self.hero_spawn_timer -= dt;
    if self.hero_spawn_timer <= 0.0 {
        self.spawn_hero_at_edge(game_data);
        self.hero_spawn_timer = 20.0; // Every 20 seconds
    }

    // Heroes pathfind to heart
    for entity in self.entities.all_mut() {
        if let Some(hero) = entity.as_hero_mut() {
            let heart_pos = find_dungeon_heart_position(&self.rooms);
            if entity.pos != heart_pos {
                move_entity_toward(entity, heart_pos);
            }
        }
    }

    // Resolve combat for adjacent enemies
    resolve_all_combat(&mut self.entities, game_data, dt);
}
```

---

## Minimal UI Requirements

**HUD (Already exists, just update):**
```
Gold: 1000 | Mana: 500 | Imps: 3/5 | Creatures: 8
[Health: ████████░░ 80%]
Mode: Dig | 1: Dig | 2: Lair | 3: Hatchery | 4: Treasury
```

**No need for:**
- ❌ Room menu panel
- ❌ Creature info panel
- ❌ Spell bar
- ❌ Minimap

Just update the existing HUD with live counts.

---

## Key Realizations

1. **We have all the systems** - they just aren't wired together
2. **Don't need more features** - need existing features to work
3. **Focus on game loop** - not more infrastructure
4. **Visible feedback** - creatures moving, combat happening, imps digging
5. **Playable in 30 minutes** - not "technically complete"

---

## New File Size Budget

**Don't create new files.** Modify existing ones:

| File | Current | Add | Total | Status |
|------|---------|-----|-------|--------|
| game_state.rs | 67 | +200 | ~270 | ✅ Under 400 |
| main.rs | 293 | +50 | ~340 | ✅ Under 400 |
| creature_ai.rs | 520 | +50 | ~570 | ✅ Under 600 |

**Files to create (only if needed):**
- `hero_ai.rs` (~300 lines) - Hero behavior
- `combat.rs` (~200 lines) - Combat resolution

**Total new code:** ~500 lines to make it playable

---

## Testing Checklist (MVP 1)

- [ ] Click dig mode, mark tiles
- [ ] Imps spawn automatically
- [ ] Imps pathfind to marked tiles
- [ ] Imps dig tiles (green floor appears)
- [ ] Build a lair (blue tiles)
- [ ] Room is detected automatically
- [ ] Spawn a creature manually (testing)
- [ ] Creature pathfinds to lair
- [ ] Creature arrives, sleep need increases
- [ ] Creatures are visible as colored circles
- [ ] Watch them move smoothly

**Success:** You can watch imps dig your dungeon and creatures use rooms. That's a **game**, not a tech demo.

---

## Priority: Make It Playable in Next 3 Hours

1. **Hour 1:** Entity movement + pathfinding integration
2. **Hour 2:** Imp spawning + auto-digging
3. **Hour 3:** Room detection + creature needs

Then we have something you can actually **play** and iterate on.

Stop building systems. Start making the game work.
