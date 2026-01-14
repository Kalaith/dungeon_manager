use crate::engine::pathfinding::Pos;
use crate::state::entities::{EntityId, EntityManager};
use crate::state::tile_state::TilePos;

/// Shared movement logic for entities following a path/timer system
pub fn process_entity_movement(
    entities: &mut EntityManager,
    entity_id: EntityId,
    dt: f32,
) -> Option<TilePos> {
    let (should_move, next_waypoint) = {
        let entity = match entities.get_mut(entity_id) {
            Some(e) => e,
            None => return None,
        };
        let creature = match entity.as_creature_mut() {
            Some(c) => c,
            None => return None,
        };

        let mut should_move = false;
        let mut next_waypoint = None;

        if let Some(ref mut path) = creature.current_path {
            if !path.is_empty() {
                creature.move_timer += dt;
                let move_interval = 1.0 / creature.movement_speed;

                if creature.move_timer >= move_interval {
                    creature.move_timer = 0.0;
                    should_move = true;
                    next_waypoint = path.first().copied();
                }
            }
        }

        (should_move, next_waypoint)
    };

    if should_move {
        if let Some(next_pos) = next_waypoint {
            if let Some(entity) = entities.get_mut(entity_id) {
                entity.pos = next_pos;

                if let Some(creature) = entity.as_creature_mut() {
                    if let Some(ref mut path) = creature.current_path {
                        path.remove(0);
                        if path.is_empty() {
                            creature.current_path = None;
                        }
                    }
                }
                return Some(next_pos);
            }
        }
    }
    
    None
}
