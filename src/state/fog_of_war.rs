use crate::data::GameData;
use crate::engine::tile_grid;
use crate::state::game_state::GameState;
use crate::state::tile_state::{Ownership, TilePos};
use crate::state::OwnerId;
use std::collections::HashSet;

impl GameState {
    pub(in crate::state) fn update_fog_of_war_system(&mut self, game_data: &GameData) {
        let mut claimed_tiles = HashSet::new();
        let (width, height) = tile_grid::get_grid_dimensions(&self.dungeon.grid);

        for y in 0..height {
            for x in 0..width {
                let pos = TilePos::new(x as i32, y as i32);
                if let Some(tile) = tile_grid::get_tile(&self.dungeon.grid, pos) {
                    if tile.ownership == Ownership::Player {
                        claimed_tiles.insert(pos);
                    }
                }
            }
        }

        // Only the player's own creatures grant vision — faction is not enough,
        // or rival keepers' dungeon creatures would reveal their lair to us
        let creature_positions: Vec<TilePos> = self
            .entities
            .creatures()
            .filter(|(_, creature)| creature.creature_id != "imp")
            .filter_map(|(id, _)| self.entities.get(id))
            .filter(|entity| entity.owner == OwnerId::Player)
            .map(|entity| entity.pos)
            .collect();

        self.dungeon
            .update_fog_of_war(&claimed_tiles, &creature_positions, game_data);
    }
}

#[cfg(test)]
mod tests {
    use crate::data::GameData;
    use crate::state::game_state::GameState;
    use crate::state::tile_state::{FogState, TilePos};

    #[test]
    fn rival_lair_stays_hidden_until_scouted() {
        let game_data = GameData::load().expect("game data should load");
        let mut state = GameState::new_for_scenario(&game_data, "dark_beginnings");
        state.update_fog_of_war_system(&game_data);

        // The rival keeper's heart and garrison must not be revealed by the
        // rival's own creatures
        let rival_heart = TilePos::new(6, 23);
        for dy in -1..=1 {
            for dx in -1..=1 {
                let pos = TilePos::new(rival_heart.x + dx, rival_heart.y + dy);
                let tile = state.get_tile(pos).expect("rival lair tile should exist");
                assert_eq!(
                    tile.fog_state,
                    FogState::Hidden,
                    "rival lair tile {pos:?} should be hidden"
                );
            }
        }

        let player_heart = TilePos::new(14, 9);
        let tile = state
            .get_tile(player_heart)
            .expect("player heart tile should exist");
        assert_eq!(tile.fog_state, FogState::Visible);
    }
}
