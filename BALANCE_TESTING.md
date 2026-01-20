# Dungeon Manager - Balance Testing Guide

This document tracks all systems requiring balance testing and tuning before release.

---

## Table of Contents
1. [Resource Economy](#1-resource-economy)
2. [Digging & Mining Rewards](#2-digging--mining-rewards)
3. [Hero Wave System](#3-hero-wave-system)
4. [Creature Stats & Costs](#4-creature-stats--costs)
5. [Hero Stats](#5-hero-stats)
6. [Room Costs & Benefits](#6-room-costs--benefits)
7. [Combat Mechanics](#7-combat-mechanics)
8. [Traps](#8-traps)
9. [Spells](#9-spells)
10. [Creature AI & Mood](#10-creature-ai--mood)
11. [Progression & Leveling](#11-progression--leveling)
12. [Known Issues & Bugs](#12-known-issues--bugs)

---

## 1. Resource Economy

### Starting Resources
| Resource | Current Value | Max Capacity | Notes |
|----------|---------------|--------------|-------|
| Gold | 20,000 | 20,000 | |
| Mana | 10,000 | 0 (BUG?) | Max capacity is 0 in config |
| Food | 100 | 500 | |
| Materials | 100 | 100 | |

### Testing Checklist
- [ ] Is 20,000 starting gold appropriate for early game?
- [ ] Can player survive first wave with default resources?
- [ ] Is mana max capacity intentionally 0? (appears to be a bug)
- [ ] How long until gold runs out with typical army composition?
- [ ] Is food generation sufficient for creature needs?

### Questions to Answer
- How many rooms can be built before first wave arrives?
- What's the minimum viable dungeon setup cost?
- At what point does the economy become self-sustaining?

---

## 2. Digging & Mining Rewards

### Current Values
| Tile Type | Reward | Notes |
|-----------|--------|-------|
| Gold Vein | 50 gold | Per vein mined |
| Gem Seam | 25 gold | Should this be gems instead? |
| Mana Crystal | 20 mana | Per crystal |

### Imp Behavior
| Setting | Value |
|---------|-------|
| Gold Carrying Threshold | 20 gold (seeks treasury when reached) |
| Dig Completion Delay | 2.0 seconds |
| Gem Priority Bonus | 10,000 (AI priority modifier) |

### Testing Checklist
- [ ] Is 50 gold per vein enough to sustain economy?
- [ ] How many gold veins exist per map?
- [ ] Is gem seam reward (25 gold) intentional or should it grant gems?
- [ ] Does 20 gold carrying threshold cause too many treasury trips?
- [ ] Is digging speed appropriate for map size?

### Questions to Answer
- Average gold earned from fully mining a map?
- Gold income rate per imp per minute?
- Optimal number of imps for economy?

---

## 3. Hero Wave System

### Wave Configuration
| Setting | Current Value | Notes |
|---------|---------------|-------|
| Initial Wave Delay | 2,000 seconds (~33 min) | Time before first wave |
| Wave Interval | 180 seconds (3 min) | Time between waves |
| Wave Scaling Multiplier | 1.5x | Hero count increase per wave |
| Spawn Rate Decay | 0.9x | Spawning gets 10% faster each wave |
| Min Spawn Rate | 5.0 seconds | Fastest possible spawn interval |
| Defender Ratio | 0.5 | 50% defend buildings, 50% attack |

### Wave Scaling Formula
```
Max Heroes Per Type = base_count + (wave_number × 1.5)
Spawn Rate = base_rate × 0.9^wave_number
```

### Example Wave Progression
| Wave | Knights (base 2) | Spawn Rate (base 60s) |
|------|------------------|----------------------|
| 1 | 3.5 | 54s |
| 2 | 5 | 48.6s |
| 3 | 6.5 | 43.7s |
| 5 | 9.5 | 35.4s |
| 10 | 17 | 20.9s |

### Hero Building Spawns
| Building | HP | Spawns | Rate | Max Count |
|----------|----|----|------|-----------|
| Barracks | 200 | Knight | 120s | 2 |
| | | Peasant | 60s | 5 |
| Archery Range | 150 | Archer | 90s | 2 |
| | | Scout | 80s | 3 |
| Church | 200 | Acolyte | 100s | 2 |
| | | Battle Cleric | 180s | 1 |
| Mage Tower | 250 | Wizard | 240s | 1 |
| | | Archmage | 600s | 1 |
| Town Hall | 500 | - | - | Victory target |

### Testing Checklist
- [ ] Is 33 minutes enough time to prepare for wave 1?
- [ ] Is wave 1 beatable with starting resources only?
- [ ] At what wave does difficulty become overwhelming?
- [ ] Does 1.5x scaling create a fair difficulty curve?
- [ ] Is 3 minutes between waves enough recovery time?
- [ ] Does destroying buildings meaningfully reduce hero spawns?

### Questions to Answer
- What's the expected wave player should reach on first playthrough?
- Is there a "winning" state or endless waves?
- Should early waves have fewer hero types?

---

## 4. Creature Stats & Costs

### Creature Wages (Gold/Minute)
| Creature | Wage | Role | Value Assessment |
|----------|------|------|------------------|
| Imp | 1 | Worker | |
| Skeleton | 1 | Undead Fighter | |
| Goblin | 2 | Fighter | |
| Warlock | 3 | Mage | |
| Orc | 4 | Heavy Fighter | |
| Troll | 5 | Heavy Worker | |
| Succubus | 6 | Torturer | |
| Demon Spawn | 8 | Elite Fighter | |
| Vampire | 15 | Elite Caster | |
| Bile Demon | 20 | Worker Tank | |
| Hellhound | 0 | Beast Scout | Free! |

### Creature Combat Stats (Level 1)
| Creature | HP | Attack | Defense | Speed | Notes |
|----------|-----|--------|---------|-------|-------|
| Imp | 80 | 4 | 2 | 1.2 | Fastest worker |
| Goblin | 120 | 8 | 3 | 1.0 | |
| Skeleton | 90 | 7 | 4 | 1.1 | |
| Warlock | 100 | 5 | 2 | 0.8 | Magic user |
| Orc | 200 | 15 | 8 | 0.9 | |
| Succubus | 150 | 10 | 5 | 1.1 | |
| Troll | 300 | 12 | 10 | 0.6 | Slow tank |
| Vampire | 150 | 12 | 5 | 1.1 | |
| Demon Spawn | 400 | 25 | 15 | 1.0 | Best fighter |
| Bile Demon | 300 | 15 | 8 | 0.6 | |
| Hellhound | 90 | 10 | 3 | 1.6 | Fastest combat |

### Testing Checklist
- [ ] Are wages proportional to creature effectiveness?
- [ ] Is Bile Demon worth 20 gold/min vs Demon Spawn at 8 gold/min?
- [ ] Are Hellhounds overpowered being free?
- [ ] Is Skeleton cost-effective at 1 gold/min?
- [ ] Do creature stats justify their wage costs?

### Questions to Answer
- Gold-per-HP efficiency for each creature?
- Optimal army composition for cost vs power?
- Which creatures are never worth recruiting?

---

## 5. Hero Stats

### Hero Combat Stats (Level 1)
| Hero | HP | Attack | Defense | Speed | Tier |
|------|-----|--------|---------|-------|------|
| Peasant Militia | 80 | 5 | 2 | 0.9 | 1 |
| Scout | 60 | 4 | 1 | 1.4 | 1 |
| Acolyte | 70 | 3 | 2 | 0.8 | 1 (Healer) |
| Knight | 180 | 14 | 8 | 1.0 | 2 |
| Archer | 100 | 10 | 3 | 1.1 | 2 |
| Battle Cleric | 150 | 8 | 6 | 0.9 | 2 |
| Rogue | 90 | 12 | 4 | 1.3 | 2 |
| Alchemist | 90 | 8 | 3 | 1.0 | 2 |
| Paladin | 220 | 16 | 12 | 0.95 | 3 |
| Wizard | 80 | 6 | 2 | 0.8 | 3 |
| Inquisitor | 120 | 9 | 5 | 1.0 | 3 |
| Barbarian | 200 | 18 | 4 | 1.2 | 3 |
| Geomancer | 140 | 10 | 8 | 0.7 | 4 |
| Knight Commander | 250 | 18 | 10 | 1.0 | 4 |
| High Priest | 160 | 10 | 7 | 0.85 | 4 |
| Archmage | 100 | 8 | 3 | 0.75 | 4 |
| Champion of Light | 400 | 25 | 15 | 1.1 | 5 (BOSS) |

### Hero vs Creature Comparison
| Matchup | Hero | Creature | Assessment |
|---------|------|----------|------------|
| Equal HP | Peasant (80) | Imp (80) | Peasant slightly stronger |
| Tanks | Knight (180) | Orc (200) | Orc tankier, Knight hits harder |
| Elite | Champion (400) | Demon Spawn (400) | Equal HP, similar stats |

### Testing Checklist
- [ ] Can tier 1 heroes be defeated by basic creatures?
- [ ] Is Knight vs Orc balanced?
- [ ] Is Champion of Light beatable without elite creatures?
- [ ] Do hero healers (Acolyte/Cleric) make fights too long?
- [ ] Are Rogues too fast at infiltrating?

### Questions to Answer
- Minimum army to defeat each hero tier?
- Which heroes require special tactics?
- At what wave do tier 4-5 heroes appear?

---

## 6. Room Costs & Benefits

### Room Build Costs
| Room | Gold/Tile | Min Size | Max Size | Special |
|------|-----------|----------|----------|---------|
| Lair | 50 | 4 | 49 | Creature housing |
| Kennel | 75 | 4 | 64 | Beast housing |
| Hatchery | 75 | 9 | 64 | +0.5 food/s |
| Prison | 80 | 4 | 36 | Hold captured heroes |
| Guard Post | 90 | 4 | 25 | +1.5 defense |
| Barracks | 100 | 9 | 100 | Creature organizing |
| Treasury | 100 | 1 | 81 | +50 gold capacity/tile |
| Workshop | 120 | 9 | 49 | Trap building |
| Library | 150 | 9 | 64 | +1.0 research/s |
| Training Hall | 150 | 9 | 81 | +0.8 XP/s |
| Graveyard | 150 | 9 | 100 | Skeleton spawning |
| Ritual Circle | 200 | 9 | 49 | +0.5 mana/s, 50 mana cost |
| Torture Chamber | 250 | 9 | 36 | Convert heroes |

### Room Sell Refund
- **Refund Rate**: 5% of build cost (very punishing!)

### Testing Checklist
- [ ] Is 5% sell refund too harsh?
- [ ] Is Torture Chamber worth 5x Lair cost?
- [ ] Does Treasury capacity (50/tile) match gold generation?
- [ ] Is Hatchery food generation (0.5/s) enough?
- [ ] Are minimum room sizes appropriate?

### Questions to Answer
- Cost to build a functional dungeon?
- Most gold-efficient room sizes?
- Which rooms are essential vs luxury?

---

## 7. Combat Mechanics

### Damage Formula
```
Base Damage = Random(damage_range)
Attack Damage = Base + (Attacker.Attack × 0.5)
Defense Reduction = Defender.Defense × 0.3
Pre-Resist Damage = max(0, Attack Damage - Defense Reduction)
Final Damage = Pre-Resist Damage × (1 - Resistance%)
```

### Combat Ranges
| Type | Range (tiles) |
|------|---------------|
| Melee | 1 |
| Ranged | 5 |
| Magic | 8 |

### Level Bonuses
| Stat | Creature | Hero |
|------|----------|------|
| Attack Multiplier | 10%/level | 15%/level |
| Health Per Level | +10 | +15 |
| Max Level | 5 | 10 |

### Testing Checklist
- [ ] Is 0.5 attack multiplier appropriate?
- [ ] Is 0.3 defense reduction too weak/strong?
- [ ] Does ranged (5 tiles) vs magic (8 tiles) feel right?
- [ ] Is hero 15% attack bonus vs creature 10% balanced?
- [ ] Does max level 5 vs 10 create late-game imbalance?

### Questions to Answer
- Average time to kill for each matchup?
- Does high defense make units unkillable?
- Is magic range too advantageous?

---

## 8. Traps

### Trap Stats
| Trap | Cost | Build Time | Damage | Cooldown | Special |
|------|------|------------|--------|----------|---------|
| Door | 50 | 5s | - | - | Blocks movement |
| Alarm | 30 | 3s | - | 10s | Alerts in 10 tile radius |
| Spike | 100 | 10s | 25 | 5s | Area trigger |
| Boulder | 150 | 15s | 50 | 5s | 1.5 tile area |

### Trap vs Hero Analysis
| Trap | Damage | Knight HP (180) | Hits to Kill |
|------|--------|-----------------|--------------|
| Spike | 25 | 14% | 7-8 |
| Boulder | 50 | 28% | 4 |

### Testing Checklist
- [ ] Are traps cost-effective vs just hiring creatures?
- [ ] Is 25 spike damage enough to matter?
- [ ] Does 5s cooldown allow trap chaining?
- [ ] Is alarm 10-tile radius sufficient?
- [ ] Can traps alone stop early waves?

### Questions to Answer
- Gold cost to kill a Knight with traps only?
- Optimal trap corridor design?
- Should trap damage scale with waves?

---

## 9. Spells

### Spell Stats
| Spell | Mana | Cooldown | Effect | Range |
|-------|------|----------|--------|-------|
| Lightning Strike | 35 | 2s | 40 damage | 12 tiles |
| Heal | 25 | 3s | 50 HP | 8 tiles |
| Speed Boost | 15 | 5s | +speed | 10 tiles |
| Iron Skin | 25 | 10s | +defense | 8 tiles |
| Dark Revelation | 40 | 25s | Reveal map | All |
| Summon Imps | 50 | 15s | Spawn 1 Imp | 5 tiles |
| Make Earth | 40 | 1s | Create tile | 10 tiles |
| Call to Arms | 45 | 20s | Rally creatures | 20 tiles |
| Possess | 60+10HP | 30s | Control creature | 6 tiles |
| Corrupt Land | 80 | 60s | Transform floor | 10 tiles |
| Chickenify | 100 | 60s | Polymorph | 6 tiles |

### Testing Checklist
- [ ] Is Lightning (40 dmg) worth 35 mana?
- [ ] Is Heal (50 HP) cost-effective vs creature cost?
- [ ] Does Chickenify (100 mana) justify its cost?
- [ ] Is Summon Imps efficient vs recruiting?
- [ ] Can spells trivialize combat?

### Questions to Answer
- Mana cost per damage dealt efficiency?
- Most essential spells for survival?
- Spell spam viability with mana regen?

---

## 10. Creature AI & Mood

### Mood Thresholds
| Threshold | Value | Effect |
|-----------|-------|--------|
| Critical Need | 30% | Creature becomes desperate |
| Desert | 10% | Creature leaves dungeon |
| Mood Attention | 40% | Needs monitoring |
| Need Attention | 20% | Urgent needs |
| Training Mood | 50% | Won't train below this |
| Satisfaction | 60% | Happy and productive |

### Need Decay Rates (Per Minute)
| Creature | Sleep | Food | Gold | Training |
|----------|-------|------|------|----------|
| Imp | 15 | 20 | 5 | - |
| Goblin | 12 | 22 | 8 | 5 |
| Orc | 10 | 18 | 6 | 4 |
| Warlock | 8 | 15 | 10 | 3 |
| Troll | 15 | 25 | 12 | - |
| Skeleton | 0 | 0 | 0.2 | - |
| Demon Spawn | 10 | 30 | 15 | 6 |
| Succubus | 10 | 15 | 8 | 0 |
| Vampire | 8 | 20 | 25 | - |
| Hellhound | 15 | 30 | 0 | - |
| Bile Demon | 20 | 60 | 15 | - |

### Discipline Effects (Mood Change)
| Creature | Slap | Torture | Reward |
|----------|------|---------|--------|
| Imp | -3 | -15 | +8 |
| Goblin | -5 | -25 | +10 |
| Orc | -8 | -30 | +15 |
| Skeleton | 0 | 0 | 0 |
| Succubus | +5 | +10 | +5 |
| Demon Spawn | -10 | -40 | +20 |

### Testing Checklist
- [ ] Do creatures desert too quickly?
- [ ] Is 10% desert threshold appropriate?
- [ ] Can Bile Demon's 60 food/min be sustained?
- [ ] Does Succubus enjoying torture make her too easy?
- [ ] Is Skeleton's 0 needs balanced by weak stats?

### Questions to Answer
- Time until creature deserts with no care?
- Optimal creature mix for low maintenance?
- Does mood micromanagement feel tedious?

---

## 11. Progression & Leveling

### XP Requirements
| Level | XP Needed | Formula |
|-------|-----------|---------|
| 1→2 | 100 | Base |
| 2→3 | 200 | 100 × 2 |
| 3→4 | 400 | 100 × 4 |
| 4→5 | 800 | 100 × 8 |

### XP Sources
- **Combat Kill**: 10 XP per victim level
- **Training Hall**: 0.8 XP per second

### Level Up Bonuses
- **Health**: +10 HP + (current HP × 0.2)
- **Attack**: Varies by creature type
- **Defense**: Varies by creature type

### Research Technology Tree
| Tech | Cost | Requires | Unlocks |
|------|------|----------|---------|
| Training | 10 | - | Training Hall |
| Prison | 15 | Training | Prison room |
| Ritual | 30 | Prison | Ritual Circle |
| Trap | 20 | Training | Workshop traps |

### Testing Checklist
- [ ] Is XP doubling per level too steep?
- [ ] Is Training Hall 0.8 XP/s enough?
- [ ] Do creatures reach max level before late waves?
- [ ] Is 10 XP per kill appropriate?
- [ ] Is research progression pacing good?

### Questions to Answer
- Time to max level a creature via training?
- Time to max level via combat?
- Are high-level creatures worth the investment?

---

## 12. Known Issues & Bugs

### Confirmed Issues
1. **Mana Max Capacity = 0**: In game_config.json, mana max is set to 0 while starting mana is 10,000. Needs investigation.
2. **Gem Seam Rewards Gold**: Currently gives 25 gold instead of gems. Intentional?

### Balance Concerns
1. **Hero Level Advantage**: Heroes gain 15% attack/level vs creature 10%. With hero max level 10 vs creature max 5, late-game heavily favors heroes.
2. **Wage Collection Priority**: At 2.0 desirability (highest), creatures may cluster at treasury too often.
3. **Room Sell Penalty**: 5% refund is extremely punishing for experimentation.
4. **Bile Demon Cost**: At 20 gold/min with 300 HP, less efficient than Demon Spawn (8 gold/min, 400 HP).

### Needs Playtesting
- Wave 1 difficulty with starting resources only
- Sustainable army size with gold generation
- Late wave (10+) survivability
- Optimal dungeon layout costs

---

## Balance Testing Procedure

### Phase 1: Early Game (Waves 1-3)
1. Start new game, build only essential rooms
2. Note: Can you survive wave 1 without mining?
3. Note: Time to prepare adequate defenses
4. Note: First creature deaths and causes

### Phase 2: Mid Game (Waves 4-7)
1. Note: Is economy sustainable?
2. Note: Are all room types useful?
3. Note: Creature mood issues?
4. Note: Hero types that cause problems

### Phase 3: Late Game (Waves 8+)
1. Note: Does scaling become impossible?
2. Note: Which creatures remain relevant?
3. Note: Are spells necessary for survival?
4. Note: Victory conditions achievable?

---

## Suggested Starting Adjustments

Based on initial analysis, consider testing these changes:

| Setting | Current | Suggested | Reason |
|---------|---------|-----------|--------|
| Room Sell Refund | 5% | 25-50% | Less punishing experimentation |
| Mana Max Capacity | 0 | 10,000+ | Bug fix |
| Wave Scaling | 1.5x | 1.25x | Gentler difficulty curve |
| Bile Demon Wage | 20/min | 12/min | Better value proposition |
| Training XP | 0.8/s | 1.5/s | Faster progression |
| Spike Trap Damage | 25 | 35 | More trap viability |

---

*Last Updated: 2026-01-19*
*Version: Pre-Release Balance Pass*
