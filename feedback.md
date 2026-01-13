  Major Missing Features

  1. Room Efficiency Mechanics

  - What's missing: Door placement affecting productivity, adjacency bonuses between rooms, shape optimization penalties       
  - From research: Dungeons 4 introduced meaningful room placement where configuration impacts efficiency
  - Current state: You have basic room quality (size/shape) but no adjacency bonuses or door optimization

  2. Faction-Specific Playstyles

  - What's missing: Pure faction runs (Undead-only, Demon-only, Horde-only) with unique passive bonuses
  - From research: Dungeons 4's faction system adds strategic variety and replayability
  - Current state: You have creature types but no faction grouping or specialized playstyles

  3. Meta-Progression System

  - What's missing: Persistent buffs/unlocks across missions, achievement-based bonuses
  - From research: Complete missions with constraints (e.g., "Undead only") to unlock permanent 15% cost reduction
  - Current state: Research tree exists but no cross-mission progression

  4. Hero Conversion Mechanics

  - What's missing: Torture chamber converting captured heroes into your creatures
  - From research: Core feature of Dungeon Keeper - extract information, break morale, turn enemies into allies
  - Current state: Prison room exists in design but conversion mechanics not detailed

  5. Economic Threats

  - What's missing: Dwarven miners/enemy workers establishing camps IN your dungeon
  - From research: Dungeons 4's internal economic threats that steal resources
  - Current state: Heroes attack but don't establish persistent resource theft

  6. Temple/Sacrifice System

  - What's missing: Religious mechanics where you sacrifice creatures to dark gods for benefits
  - From research: Dungeon Keeper's temple allowed sacrifices for spells, creature happiness
  - Current state: No religious/ritual gameplay elements

  7. Creature Social Dynamics

  - What's missing: Inter-creature conflicts based on personality, species rivalries, emergent fights
  - From research: "Creatures possess complex AI... creating unique stories through opposing system management"
  - Current state: You have mood/needs but no creature-to-creature relationship mechanics

  8. Physical Hand Interactions (Extended)

  - What's missing: Picking up gold, room objects, or other items (not just creatures)
  - From research: Hand cursor serves multiple tactile functions beyond slapping
  - Current state: You have slap/pickup for creatures but not for objects/gold

  Medium Priority Gaps

  9. Alarm/Detection Traps

  - What's missing: Traps that alert you or summon creatures (not just damage)
  - From research: Alarm traps mentioned as core trap variant
  - Current state: Only damage-dealing traps planned

  10. Automatic Room Object Placement

  - What's missing: Furniture, equipment, decorations auto-placed in rooms
  - From research: "Room objects placed automatically" enhances visual feedback
  - Current state: Mentioned in GDD but not detailed in implementation

  11. Fog of War Progression

  - What's missing: Gameplay around revealing the map, scouting mechanics
  - From research: "Fog-of-war state" as core tile property
  - Current state: Tile has fog-of-war field but no exploration mechanics detailed

  12. Creature Fleeing/Self-Preservation

  - What's missing: Automatic retreat when wounded without player input
  - From research: "Flee when wounded without player input" creates emergent stories
  - Current state: Basic combat but no cowardice/retreat AI mentioned

  13. Room Object Interaction

  - What's missing: Creatures interacting with specific furniture (training dummies, bookshelves, beds)
  - From research: Visual storytelling through object usage
  - Current state: Rooms exist but object-level interactions not specified

  14. Reinforced Wall Construction

  - What's missing: Player-built defensive walls (beyond natural rock)
  - From research: "Mark walls to reinforce" mentioned in GDD
  - Current state: In GDD but not in JSON data or implementation priority

  15. Alternative Resource Nodes

  - What's missing: Gems as infinite (but slower) gold sources
  - From research: Strategic choice between finite high-yield gold vs infinite gems
  - Current state: Gold veins only

  Minor/Polish Gaps

  16. Environmental Hazards

  - What's missing: Lava/water tiles with gameplay impact
  - From research: Environmental damage, movement restrictions
  - Current state: Listed as "later" in GDD, no mechanics designed

  17. Trap Ammunition/Reloading

  - What's missing: Imps need to refill traps after X triggers
  - From research: "Traps require imps to transport and install" - creates supply chain
  - Current state: Traps have limited charges but reload mechanics unclear

  18. Creature Information Extraction

  - What's missing: Torturing prisoners reveals enemy dungeon layout, plans
  - From research: Strategic intelligence gathering through torture
  - Current state: Torture chamber exists but only for conversion, not intel

  19. Magical Door Locking

  - What's missing: Spell-locked doors requiring magic to bypass
  - From research: "Can be locked magically" adds tactical depth
  - Current state: Basic doors only

  20. Overworld/Surface Gameplay

  - What's missing: RTS-style surface world to raid/conquer
  - From research: Dungeons 3-4's major expansion of surface gameplay
  - Current state: Explicitly out of MVP scope, but worth noting for future

  Design Philosophy Gaps

  21. Emergent Narrative Focus

  - What's research emphasizes: "Creating unique stories through opposing system management"
  - Current state: Systems exist but no explicit design for memorable moments/stories

  22. Dark Humor/Personality

  - What's missing: Cartoonish malevolence, creature voice lines, exaggerated evil
  - From research: "Dungeon Keeper's success stemmed from its cartoonish malevolence"
  - Current state: Flavor text exists but no personality/humor system

  Summary: Top 5 Priorities to Consider

  If you want to match the genre's most successful titles:

  1. Room efficiency mechanics (adjacency, door placement) - Makes layout strategic
  2. Hero conversion system - Core fantasy of turning enemies
  3. Creature social dynamics - Emergent storytelling through conflicts
  4. Meta-progression across missions - Long-term engagement
  5. Temple/sacrifice mechanics - Reinforces evil theme through gameplay

  Your current design is solid for an MVP, but these research-backed features separate "functional dungeon keeper clone" from "compelling dungeon keeper experience."