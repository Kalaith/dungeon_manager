use crate::state::entities::{EntityId, EntityManager};
use crate::state::tile_state::TilePos;

/// Shared movement logic for entities following a path/timer system
pub fn process_entity_movement(
    entities: &mut EntityManager,
    entity_id: EntityId,
    dt: f32,
) -> Option<TilePos> {
    let movement_data = if let Some(entity) = entities.get(entity_id) {
        match &entity.entity_type {
            crate::state::entities::EntityType::Creature(c) => {
                Some((c.current_path.clone(), c.move_timer, c.movement_speed))
            }
            crate::state::entities::EntityType::Hero(h) => {
                Some((h.current_path.clone(), h.move_timer, h.movement_speed))
            }
            _ => None,
        }
    } else {
        None
    };

    let (current_path, move_timer, movement_speed) = movement_data?;

    let mut should_move = false;
    let mut next_waypoint = None;
    let mut new_move_timer = move_timer;

    if let Some(path) = current_path {
        if !path.is_empty() {
            new_move_timer += dt;
            let move_interval = 1.0 / movement_speed;

            if new_move_timer >= move_interval {
                new_move_timer = 0.0;
                should_move = true;
                next_waypoint = path.first().copied();
            }
        }
    }

    if should_move {
        if let Some(next_pos) = next_waypoint {
            if let Some(entity) = entities.get_mut(entity_id) {
                // Update position and state
                entity.pos = next_pos;

                match &mut entity.entity_type {
                    crate::state::entities::EntityType::Creature(c) => {
                        c.move_timer = 0.0;
                        if let Some(ref mut path) = c.current_path {
                            if !path.is_empty() {
                                path.remove(0);
                            }
                            if path.is_empty() {
                                c.current_path = None;
                            }
                        }
                    }
                    crate::state::entities::EntityType::Hero(h) => {
                        h.move_timer = 0.0;
                        if let Some(ref mut path) = h.current_path {
                            if !path.is_empty() {
                                path.remove(0);
                            }
                            if path.is_empty() {
                                h.current_path = None;
                            }
                        }
                    }
                    _ => {}
                }
                return Some(next_pos);
            }
        }
    } else {
        // Update timer only
        if let Some(entity) = entities.get_mut(entity_id) {
            match &mut entity.entity_type {
                crate::state::entities::EntityType::Creature(c) => c.move_timer = new_move_timer, // Use calculated new timer
                crate::state::entities::EntityType::Hero(h) => h.move_timer = new_move_timer,
                _ => {}
            }
        }
    }

    None
}
