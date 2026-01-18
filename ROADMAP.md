# Deep Dominion Roadmap

## Current Status: Early Development 🚧

This document tracks what's currently being worked on and planned future features.

---

## ✅ Implemented (Core Systems)

### Map Generation
- [x] Procedural terrain generation (noise-based and cellular automata)
- [x] Connectivity validation (flood-fill)
- [x] Resource placement (Gold Veins, Gem Seams, Mana Crystals)
- [x] Hazard generation (Lava, Water)
- [x] Hero portal placement
- [x] Biome system (optional)
- [x] Multiple starting positions (center, corner)

### Rooms
- [x] 10 room types with data-driven definitions
- [x] Room validation and quality scoring
- [x] Room effects (mana gen, food gen, research rate, etc.)

### Creatures & AI
- [x] Needs-based AI (Sleep, Food, Gold)
- [x] Mood and desertion system
- [x] Task scoring and assignment
- [x] Pathfinding (A*)
- [x] 7+ creature types with unique stats/behaviors

### Combat
- [x] Real-time combat resolution
- [x] Stat-based damage calculation
- [x] Elemental resistances
- [x] XP and leveling system

### Spells
- [x] 8 spells with varied effects
- [x] Mana economy
- [x] Cooldown system
- [x] Targeting system (tile, creature, area)

### Traps
- [x] 4 trap types (Door, Spike, Boulder, Alarm)
- [x] Workshop-based construction
- [x] Material costs

### Heroes
- [x] 14 hero types across 5 tiers
- [x] Hero AI and pathfinding
- [x] Portal spawning

### UI
- [x] Sidebar with tabs (Build, Magic, Minions, Traps)
- [x] Creature inspection panel
- [x] Resource display

### Graphics
- [x] Procedural tile generation
- [x] Procedural sprite generation (3D-shaded)
- [x] Fog of war

### Research & Progression
- [x] Tech tree system (technologies.json)
- [x] Library room logic
- [x] Research tasks and UI

---

## 🔨 In Progress

### Balance & Tuning
- [ ] Creature wage/need decay rates
- [x] Hero wave scaling
- [x] Room cost balancing
- [x] Spell cooldown/power tuning

### UI Polish
- [x] Tooltips for all UI elements
- [x] Better feedback for failed actions (Notifications)
- [x] Minimap
- [ ] Hotkey display overlay

### Content
- [x] More creature types (Spider, Vampire, Dark Elf)
- [x] More trap types (Gas, Lightning, Pit)
- [x] More spells (Earthquake, Speed Boost, Invisibility)

---

## 🚀 Planned Features

### Gameplay
- [ ] Win conditions (scenario objectives)
- [ ] Campaign/scenario mode
- [ ] Creature mutations/evolutions
- [ ] Hero party composition AI
- [ ] Boss heroes
- [ ] Dungeon reputation system

### Rooms
- [ ] Torture Chamber
- [ ] Graveyard (undead spawning)
- [ ] Barracks
- [ ] Temple

### Economy
- [ ] Trade system
- [ ] Creature hiring (vs. attraction)
- [ ] Resource trading

### Combat
- [ ] Creature abilities (active skills)
- [ ] Formation system
- [ ] Ranged combat improvements
- [ ] Status effects (poison, burn, freeze)

### Technical
- [ ] Save/Load system
- [ ] Settings menu
- [ ] Sound system
- [ ] Music


---

## 💡 Ideas Under Consideration

- Procedural quests/objectives
- Creature breeding
- Artifact/relic system
- Underground ecosystems
- Day/night cycle affecting hero invasions
- Multiple dungeon hearts (expansion gameplay)

---

## Contributing

Contributions are welcome! If you'd like to work on any of these features:

1. Check if there's an existing issue
2. Open a discussion for large features
3. Submit PRs against the `main` branch

---

*Last updated: January 2026*
