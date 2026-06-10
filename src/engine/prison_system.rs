use crate::data::GameData;
use crate::state::entities::{EntityId, EntityManager};
use crate::state::notifications::NotificationManager;
use crate::state::room_manager::RoomManager;
use crate::state::tile_state::TilePos;

/// Handle capturing dead heroes - teleport to available prison
pub fn handle_prison_captures(
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    notifications: &mut NotificationManager,
) {
    // Find heroes that just died (health <= 0) and aren't already captured
    let mut heroes_to_capture: Vec<EntityId> = Vec::new();

    for (hero_id, hero) in entities.heroes() {
        if hero.health <= 0.0 && !hero.is_captured && !hero.is_converted {
            heroes_to_capture.push(hero_id);
        }
    }

    if heroes_to_capture.is_empty() {
        return;
    }

    // Find available prison tiles
    // We'll just look for ANY prison tile for now, ideally one that is empty
    let mut available_prison_tiles = Vec::new();
    for room in &room_manager.rooms {
        if room.room_type == "prison" {
            for &tile_pos in &room.tiles {
                available_prison_tiles.push(tile_pos);
            }
        }
    }

    if available_prison_tiles.is_empty() {
        return; // No prison, they just die
    }

    // Shuffle tiles to avoid stacking all in one spot (if possible)
    // For deterministic behavior in this tool we might skip shuffle or use a simple index

    for hero_id in heroes_to_capture {
        // Just pick a random one for now
        let random_idx = macroquad_toolkit::rng::gen_range(0, available_prison_tiles.len());
        let target_pos = available_prison_tiles[random_idx];

        // Capture the hero
        if let Some(entity) = entities.get_mut(hero_id) {
            // Move entity first to avoid borrow issues
            entity.pos = target_pos;
            entity.visual_pos = (target_pos.x as f32, target_pos.y as f32);

            if let Some(hero) = entity.as_hero_mut() {
                hero.is_captured = true;
                hero.conversion_progress = 0.0;
                // Revive slightly so they don't get cleaned up as "dead" immediately
                hero.health = 10.0;

                eprintln!(
                    "Hero {} captured and teleported to prison at {:?}!",
                    hero.hero_id, target_pos
                );
                notifications.success(format!("Hero captured!"));
            }
        }
    }
}

/// Progress prison (Skeleton) and torture (Conversion) logic
pub fn progress_prison_conversions(
    entities: &mut EntityManager,
    room_manager: &RoomManager,
    notifications: &mut NotificationManager,
    game_data: &GameData,
    dt: f32,
) {
    // Identify active Torture Chambers with working Succubi
    let mut active_torture_rooms = std::collections::HashSet::new();
    for (_, creature) in entities.creatures() {
        if creature.creature_id == "succubus" {
            if let Some(crate::state::entities::Task::Work(room_id, _)) = creature.current_task {
                active_torture_rooms.insert(room_id);
            }
        }
    }

    // Get conversion rates from config
    let skeleton_rate = game_data.config.conversion.skeleton_rate;
    let torture_base_rate = game_data.config.conversion.torture_rate;

    let mut conversions_to_process: Vec<(EntityId, bool)> = Vec::new(); // (Id, IsTorture)

    for (hero_id, hero) in entities.heroes() {
        if hero.is_captured && !hero.is_converted {
            if let Some(entity) = entities.get(hero_id) {
                // Check which room they are in
                if let Some(room) = room_manager.get_room_at(entity.pos) {
                    if room.room_type == "prison" {
                        conversions_to_process.push((hero_id, false));
                    } else if room.room_type == "torture_chamber" {
                        if active_torture_rooms.contains(&room.id) {
                            conversions_to_process.push((hero_id, true));
                        }
                    }
                }
            }
        }
    }

    // Apply progress
    for (hero_id, is_torture) in conversions_to_process {
        let mut completed = false;
        let mut hero_name = "".to_string();

        if let Some(entity) = entities.get_mut(hero_id) {
            if let Some(hero) = entity.as_hero_mut() {
                hero_name = hero.hero_id.clone();
                let rate = if is_torture {
                    torture_base_rate
                } else {
                    skeleton_rate
                };
                hero.conversion_progress += rate * dt;

                if hero.conversion_progress >= 1.0 {
                    completed = true;
                }
            }
        }

        if completed {
            if is_torture {
                // Retrieve Hero and convert
                if let Some(entity) = entities.get_mut(hero_id) {
                    let pos = entity.pos;
                    if let Some(hero) = entity.as_hero_mut() {
                        hero.is_converted = true;
                        hero.is_captured = false;
                        hero.health = hero.max_health; // Heal them up
                                                       // Reset goal to something safe
                        hero.current_goal = crate::state::entities::HeroGoal::RestAtSpawn(pos);
                        notifications.success(format!("{} converted to your side!", hero_name));
                    }
                }
            } else {
                // Skeleton time
                // Remove hero, spawn skeleton
                let pos = if let Some(e) = entities.get(hero_id) {
                    e.pos
                } else {
                    TilePos::new(0, 0)
                }; // Should be valid
                entities.remove(hero_id);

                if let Some(monster_data) = game_data.monsters.get("skeleton") {
                    let visual_seed = macroquad_toolkit::rng::random_u64();
                    let creature_state = crate::state::entities::CreatureState::new(
                        "skeleton".to_string(),
                        1,
                        monster_data.stats.health,
                        monster_data.stats.mana,
                        visual_seed,
                    );
                    entities.spawn_creature(pos, creature_state);
                    notifications.success("Captured hero rotted into a Skeleton!");
                }
            }
        }
    }
}
