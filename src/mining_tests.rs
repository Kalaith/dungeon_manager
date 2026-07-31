//! Tests for what a dig pays out.
//!
//! The gem seam authored `mine_value: 25` in `tiles.json` while the config
//! authored `gem_seam_reward: 25` separately — the same number in two files,
//! and only the config one was read. These pin the tile's own number as the
//! source, and pin the design property the duplication was obscuring: gems are
//! the slow-but-endless gold source.

use crate::data::GameData;
use crate::state::player_state::PlayerState;
use crate::state::tile_state::TilePos;

/// Gold in the player's purse after one imp finishes digging `tile_type`.
fn dig_once(game_data: &GameData, tile_type: &str) -> (i32, String, bool) {
    let mut state =
        crate::state::game_state::GameState::new_for_scenario(game_data, "dark_beginnings");
    let pos = TilePos::new(5, 5);
    {
        let tile = state.dungeon.get_tile_mut(pos).expect("tile in bounds");
        tile.tile_type = tile_type.to_string();
        tile.marked_for_dig = true;
        tile.resources_remaining = game_data
            .tiles
            .get(tile_type)
            .and_then(|t| t.resources.as_ref())
            .and_then(|r| (r.amount > 0).then_some(r.amount as u32));
    }

    // The scenario starts the purse full, so a dig would spill into a pile
    // rather than pay out. Empty it first — this measures the yield, not the
    // overflow behaviour.
    state.player.gold = 0;
    state.player.mana = 0;
    let before = state.player.gold;
    crate::engine::imp_ai::complete_dig(
        &mut state.dungeon,
        None,
        &mut state.player,
        pos,
        game_data,
    );
    let after = state.player.gold;

    let tile = state.dungeon.get_tile(pos).expect("tile still in bounds");
    (after - before, tile.tile_type.clone(), tile.marked_for_dig)
}

#[test]
fn a_dig_pays_the_tiles_authored_mine_value() {
    let game_data = GameData::load().expect("game data should load");

    for tile_id in ["gold_vein", "gem_seam"] {
        let authored = game_data.tiles[tile_id]
            .resources
            .as_ref()
            .and_then(|r| r.mine_value)
            .unwrap_or_else(|| panic!("`{tile_id}` should author a mine_value"));

        let (gained, _, _) = dig_once(&game_data, tile_id);
        assert_eq!(
            gained, authored,
            "`{tile_id}` paid {gained}, but its data says {authored}"
        );
    }
}

#[test]
fn gems_are_the_slow_but_endless_source() {
    // The property the GDD asks for, and the one the hardcoded constant made
    // hard to see: a seam pays less per dig than a vein, and unlike a vein it
    // is never consumed.
    let game_data = GameData::load().expect("game data should load");

    let (vein_gold, vein_after, vein_marked) = dig_once(&game_data, "gold_vein");
    let (gem_gold, gem_after, gem_marked) = dig_once(&game_data, "gem_seam");

    assert!(
        gem_gold < vein_gold,
        "a gem seam should pay less per dig than a vein ({gem_gold} vs {vein_gold})"
    );

    assert_ne!(vein_after, "gold_vein", "a mined vein should be consumed");
    assert!(
        !vein_marked,
        "a consumed vein should stop being a dig target"
    );

    assert_eq!(
        gem_after, "gem_seam",
        "a gem seam should survive being mined"
    );
    assert!(
        gem_marked,
        "a gem seam should stay marked so imps keep working it"
    );
}

#[test]
fn every_resource_tile_authors_its_own_yield() {
    // Otherwise the value silently falls back to a global constant, which is
    // how the gem seam's 25 came to exist in two places.
    let game_data = GameData::load().expect("game data should load");

    let mut missing = Vec::new();
    for (id, tile) in &game_data.tiles {
        let Some(resources) = tile.resources.as_ref() else {
            continue;
        };
        if resources.mine_value.is_none() {
            missing.push(id.clone());
        }
    }

    assert!(
        missing.is_empty(),
        "resource tiles with no authored mine_value: {missing:?}"
    );
    let _ = PlayerState::new(&game_data);
}
