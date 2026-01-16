# optimize pathfinding/room detection with caching
# add integration tests

Summary
The engine directory (15 modules, ~5,500 lines) follows modular design with stateless services but has significant duplicate code and quality issues. Key problems: movement logic duplication between creature and imp AI, file size violations, and functions exceeding length/parameter guidelines.

Duplicate Code Instances
Movement Processing Logic

Files: creature_ai.rs (lines 107-143) and imp_ai.rs (lines 338-374)
Issue: process_creature_movement and process_imp_movement contain nearly identical timer-based path movement logic.
Recommendation: Extract to a shared process_entity_movement function in a new movement.rs module.
Distance Calculations

Files: pathfinding.rs (line 305) and creature_ai.rs (line 332)
Issue: Manhattan distance functions with identical logic but different return types (i32 vs f32).
Recommendation: Unify into a single manhattan_distance function in a shared utilities module, with variants for different types.
Code Quality Issues
File Size Violations: creature_ai.rs (871 lines) exceeds the 800-line hard limit. Split into creature_movement.rs, creature_decision.rs, and creature_needs.rs.
Function Length: Several functions exceed 100 lines (e.g., update_creature_ai ~66 lines, decide_creature_task ~65 lines). Break into smaller, single-responsibility functions.
Parameter Count: Functions like update_creature_ai and process_creature_movement have 6+ parameters. Use context structs instead.
Other: Inconsistent naming, some unwrap() usage instead of proper error handling, mixed concerns in large functions.
Recommendations
High Priority: Extract shared movement and distance logic immediately.
Medium Priority: Reduce parameters with structs, improve error handling, decompose large functions.
Long-term: Implement strategy pattern for AI modules, add unit tests, optimize performance.
Standards Compliance: Address file sizes, function lengths, and parameter counts to align with CODE_STANDARDS.md.

Name              Lines
----              -----
combat.rs           372
creature_ai.rs      871
hero_ai.rs          376
imp_ai.rs           382
input.rs            524
map_generator.rs    324
mod.rs               18
pathfinding.rs      546
room_validator.rs   472
spawner.rs          145
spell_effects.rs    402
task_system.rs      240
tile_grid.rs        399
tile_types.rs       106
trap_system.rs       85