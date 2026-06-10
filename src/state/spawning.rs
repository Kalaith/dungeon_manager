use crate::data::GameData;
use crate::engine::{spawner, spawner_logic::MonsterSpawner, tile_grid};
use crate::state::dungeon::Dungeon;
use crate::state::entities::CreatureState;
use crate::state::game_state::GameState;
use crate::state::tile_state::{Ownership, TilePos};
use macroquad_toolkit::rng;

pub(in crate::state) fn detect_monster_spawners(
    dungeon: &Dungeon,
    game_data: &GameData,
) -> Vec<MonsterSpawner> {
    let mut spawners = Vec::new();
    let (w, h) = tile_grid::get_grid_dimensions(&dungeon.grid);

    for y in 0..h {
        for x in 0..w {
            let pos = TilePos::new(x as i32, y as i32);
            if let Some(tile) = tile_grid::get_tile(&dungeon.grid, pos) {
                if tile.tile_type == "monster_spawner" {
                    let monster_id = if rng::chance(0.5) { "spider" } else { "lizard" };
                    spawners.push(MonsterSpawner::new(
                        pos,
                        monster_id.to_string(),
                        game_data.config.spawning.max_monsters_per_spawner,
                    ));
                }
            }
        }
    }

    spawners
}

impl GameState {
    pub(in crate::state) fn spawn_random_creature(&mut self, game_data: &GameData) {
        spawner::SpawnSystem::spawn_random_creature(
            &mut self.dungeon,
            &self.room_manager,
            &mut self.entities,
            game_data,
        );
    }

    pub fn spawn_starting_imps(&mut self, game_data: &GameData, count: usize) {
        for _ in 0..count {
            self.spawn_imp(game_data);
        }
        eprintln!("Spawned {} starting imps", count);
    }

    fn spawn_imp(&mut self, game_data: &GameData) {
        let mut spawn_positions = Vec::new();

        for row in &self.dungeon.grid {
            for tile in row {
                if tile.ownership == Ownership::Player
                    && (tile.tile_type == "claimed_floor" || tile.tile_type == "dungeon_heart")
                {
                    spawn_positions.push(tile.pos);
                }
            }
        }

        let Some(pos) = rng::choose(&spawn_positions).copied() else {
            return;
        };

        if let Some(monster_data) = game_data.monsters.get("imp") {
            let creature_state = CreatureState::new(
                "imp".to_string(),
                1,
                monster_data.stats.health,
                monster_data.stats.mana,
                rng::random_u64(),
            );
            self.entities.spawn_creature(pos, creature_state);
            eprintln!("Spawned imp at {:?}", pos);
        }
    }

    pub fn trigger_pay_day(&mut self) {
        eprintln!("PAY DAY! All creatures are demanding wages!");
        for entity in self.entities.all_mut() {
            if let Some(creature) = entity.as_creature_mut() {
                creature.set_need("gold".to_string(), 0.0);
            }
        }
        self.notifications
            .warning("Pay Day! Creatures demand wages.");
    }
}
