# Deep Dominion

Deep Dominion is a dungeon overseer game about carving out an underground domain, attracting creatures, and defending the dungeon heart from invading heroes.

You do not directly control every minion. You shape the dungeon, provide what creatures need, and create conditions where your forces can survive.

## Gameplay

- Dig and claim space for your dungeon.
- Build rooms that support creatures, economy, and defense.
- Attract minions and keep them fed, rested, and paid.
- Mine gold and mana to fund growth.
- Place traps and doors around chokepoints.
- Survive hero invasions before they reach the heart.

## Goal

Keep the dungeon heart alive while growing a self-sustaining underground empire. The current focus is sandbox survival and dungeon growth.

## Controls

- WASD: pan the camera.
- Mouse: select and build.
- Scroll: zoom.
- Space: pause.

## Current Scope

In-development dungeon management with digging, rooms, creatures, needs, resources, combat pressure, and defense planning.
# Practical Future Improvements

- Add tests for menu action handlers, sidebar selection, tooltip state, and dungeon command dispatch.
- Separate model mutation from renderer/sidebar modules so UI actions call explicit domain commands.
- Add scenario fixtures for room placement, resource flow, encounters, and dungeon progression milestones.
- Profile large-dungeon sidebar and renderer paths, then cache derived layout data that is recalculated every frame.

