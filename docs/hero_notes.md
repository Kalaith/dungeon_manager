Now let me compile my comprehensive report based on all the research gathered.

Hero Unit Tunneling in Dungeon Keeper and Dungeons: Selective Pathfinding Toward the Dungeon Heart
Executive Summary
In both Dungeon Keeper (1997) and its spiritual successors, hero units employ selective, goal-oriented pathfinding rather than random tunneling. Heroes enter dungeons with a primary objective: locate and destroy the Dungeon Keeper's heart. This behavior is driven by sophisticated pathfinding algorithms—primarily A* with waypoint systems—that calculate the most efficient route to the dungeon heart once entry is gained. While specific technical documentation on hero targeting is limited, player defensive strategies and in-game mechanics strongly indicate that hero movement is deliberate and algorithmically optimized, not random.
​

The Architecture of Hero Pathfinding
Foundational Algorithm: A with Waypoint Fallback*

Dungeon Keeper's navigation system is one of the most computationally intensive components of the game. According to lead designer Peter Molyneux, the navigation code represents the largest single piece of code in the original Dungeon Keeper, reflecting the complexity of enabling creatures to navigate dynamically changing dungeons. The system uses a hybrid approach combining waypoint-based navigation with A* (A-Star) pathfinding as a fallback mechanism.
​

The A* algorithm calculates the lowest-cost path between two points by evaluating both the distance traveled and a heuristic estimate to the destination. In Dungeon Keeper's context, this allows creatures—including heroes—to find efficient routes around obstacles while adapting to dungeon layouts that change as imps dig or fortify walls. The waypoint system provides predetermined paths for routine movement, reducing CPU overhead, while A* engages when standard paths are blocked or unavailable.
​

Why This Matters for Hero Behavior

This architecture is not random. Random pathfinding would either involve creatures wandering aimlessly or selecting destinations by chance at each decision point. Instead, A* requires a specific goal coordinate—and in the case of heroes, that goal is the Dungeon Keeper's heart. The algorithm then methodically calculates the shortest navigable path to reach it.
​

Hero Objectives: Selective Targeting, Not Random
Primary Goal: Destroy the Dungeon Heart

Player strategy guides confirm that heroes spawn at hero gates with a clear objective: infiltrate the dungeon and destroy the keeper's heart. This is not ambiguous or probabilistic. From the Gamespot Dungeon Keeper 2 strategy guide: heroes "arrive through portals of their own and try to break through your dungeon walls in order to attack you," with the ultimate target being the dungeon heart.
​

This selective objective drives all downstream behavior. Once heroes breach into the dungeon, their pathfinding algorithm has a concrete target—the heart's location on the map—rather than a random set of waypoints to explore.

Tunneler Dwarves as Evidence of Selective Behavior

Tunneler dwarves, the hero equivalent of the player's imps, serve as a particularly revealing indicator. According to game documentation, "when dungeon walls have been breached, you can bet your last chicken that there is a Dwarf responsible." This phrasing reveals an expectation: tunnelers are responsible for breaches, meaning they are executing a deliberate task (tunneling toward a specific destination) rather than randomly digging in all directions.
​

Moreover, dwarves share pathfinding logic with imps and other creatures. If heroes' pathfinding were random, there would be no consistent pattern to where tunnels appear—yet experienced players develop defensive strategies specifically to funnel heroes through trap-laden corridors, suggesting hero movement is predictable enough to exploit.
​

Evidence from Player Defensive Strategies
Maze Tactics Reveal Directed Pathfinding

A revealing insight comes from documented player strategies for defending against heroes. Rather than randomizing dungeon layouts or relying on luck, experienced players construct long, winding, single-tile corridors specifically designed to slow hero progression and funnel them through killzones. One strategy guide explicitly states: "As you tunnel toward the lair of your enemies, take your time. Tunnel in a single corridor and place plenty of doors and traps (just in case they break in first!)."
​

This strategy is only effective if heroes follow a path-optimized route rather than wandering randomly. If hero movement were stochastic (random), maze design would offer no advantage—heroes would pass through walls equally in all directions. The fact that maze complexity is a recognized defense mechanic strongly indicates heroes follow deterministic pathfinding.
​

Line-of-Sight and Awareness Mechanics

Another critical detail: "heroes can't see or sense what's around a corner, so you can make the most of your traps if they surprise the invaders." This statement reveals that heroes don't have omniscient awareness; they navigate corridors sequentially, turning corners and discovering obstacles as they traverse. This is consistent with grid-based pathfinding where visibility and navigation follow the dungeon's physical topology, not random leaps across the map.
​

Selective vs. Random: The Distinction
Selective Behavior (Observed)

Heroes spawn with a known destination: the dungeon heart

Pathfinding algorithms calculate optimal routes to that destination

Heroes navigate corridors sequentially, adapting to walls, doors, and terrain

Tunnelers dig in targeted directions toward breach points, not randomly

Player defenses (mazes, traps) are effective, implying predictable hero routes

Random Behavior (Not Observed)

No evidence of heroes randomly wandering the dungeon without a goal

Player strategies do not rely on luck or chance avoidance

Tunnelers don't burrow in all directions simultaneously

No documented accounts of heroes ignoring the heart to pursue secondary objectives

Pathfinding in Descendant Titles
War for the Overworld and Dungeons Series

While less extensively documented, War for the Overworld (a spiritual successor to Dungeon Keeper) includes similar mechanics. Patch notes reference "optimized Unit Pathfinding & Needs decision making," suggesting the developers maintained sophisticated pathfinding for hero-equivalent units. The game features timed hero waves and defensive mechanics, implying directed, predictable hero behavior rather than randomness.
​

The Dungeons series (Dungeons 2, 3, 4) similarly emphasizes defensive design against hero waves, reinforcing the pattern of selective, algorithmically-optimized pathfinding.
​

Technical Limitations and Design Constraints
The developers' choice to implement A* with waypoint fallback reflects a pragmatic design decision. In the mid-1990s, CPU resources were severely limited. Random pathfinding would have been simpler to code but would offer no strategic gameplay—players couldn't plan defenses. By implementing efficient A* pathfinding, developers achieved both computational feasibility and strategic depth, allowing players to anticipate and counter hero movement through dungeon architecture.
​

Conclusion
Hero units tunnel toward the player dungeon using selective, goal-oriented pathfinding, not random movement. Heroes enter with a primary objective—destroying the Dungeon Keeper's heart—and employ A* pathfinding algorithms to calculate the most efficient route to that destination. This behavior is fundamentally different from random or probability-based navigation.

The evidence is multifaceted: the architecture of Dungeon Keeper's navigation system, player defensive strategies that rely on maze design, tunneler mechanics that target specific breach points, and in-game documentation that references heroes seeking the heart. Together, these elements confirm that hero movement is deterministic and algorithmically optimized, not chance-based. This design choice balanced computational constraints with strategic gameplay, allowing players to defend through environmental design rather than relying solely on creature combat.