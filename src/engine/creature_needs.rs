use crate::data::GameData;
use crate::state::dungeon::Dungeon;
use crate::state::entities::EntityManager;
use crate::state::room_manager::RoomManager;

/// Check for creatures with critical needs and make them desert
pub fn handle_creature_desertion(
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    dungeon: &mut Dungeon,
    game_data: &GameData,
) {
    use crate::engine::creature_task_logic;

    let mut deserting_ids = Vec::new();

    // Check all creatures for desertion
    for (creature_id, creature) in entities.creatures() {
        // Check ALL critical needs, not just food
        let food_need = creature.get_need("food");
        let sleep_need = creature.get_need("sleep");
        let gold_need = creature.get_need("gold");

        // Creature is in critical condition if any need is below threshold
        let desert_threshold = game_data.config.creature_ai.need_desert_threshold;
        let is_critical = food_need < desert_threshold
            || sleep_need < desert_threshold
            || gold_need < desert_threshold;

        if is_critical {
            if let Some(monster_data) = game_data.monsters.get(&creature.creature_id) {
                // Check if they should desert using AI logic (mood-based)
                if creature_task_logic::should_desert(creature, monster_data) {
                    deserting_ids.push(creature_id);
                    trace_log!("creatures", "Creature {} is deserting! (mood: {:.1}, food: {:.1}, sleep: {:.1}, gold: {:.1})",
                        creature.creature_id, creature.mood, food_need, sleep_need, gold_need);
                }
            }
        }
    }

    // Remove deserting creatures
    for creature_id in deserting_ids {
        room_manager.release_lair_tile(dungeon, creature_id);
        entities.remove(creature_id);
    }
}
