//! Skirmish / sandbox setup.
//!
//! The procedural map generator (`engine::map_generator`) has always been
//! reachable in code but never from the UI — every launch forced
//! `MapType::Standard` at a fixed size. `SkirmishConfig` is the small,
//! testable model behind a skirmish setup screen: the player cycles a map
//! type and a size, and this maps those choices to the `(width, height,
//! MapType)` that `GameState::new_with_map_type` already understands.

use crate::state::game_state::MapType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkirmishConfig {
    /// Index into [`SkirmishConfig::MAP_TYPE_LABELS`].
    pub map_type: usize,
    /// Index into [`SkirmishConfig::SIZE_LABELS`].
    pub size: usize,
}

impl Default for SkirmishConfig {
    fn default() -> Self {
        // Standard terrain, medium map — a sensible neutral starting point.
        Self {
            map_type: 0,
            size: 1,
        }
    }
}

impl SkirmishConfig {
    pub const MAP_TYPE_LABELS: [&'static str; 3] = ["Standard", "Rich", "Hazardous"];
    pub const SIZE_LABELS: [&'static str; 3] = ["Small", "Medium", "Large"];
    /// Square edge length for each size index.
    pub const SIZE_DIMS: [usize; 3] = [24, 32, 48];

    fn map_type_idx(&self) -> usize {
        self.map_type % Self::MAP_TYPE_LABELS.len()
    }

    fn size_idx(&self) -> usize {
        self.size % Self::SIZE_LABELS.len()
    }

    /// The generator map type for the chosen terrain.
    pub fn map_type(&self) -> MapType {
        match self.map_type_idx() {
            1 => MapType::Rich,
            2 => MapType::Hazardous,
            _ => MapType::Standard,
        }
    }

    /// `(width, height)` for the chosen size — the generator uses square maps.
    pub fn dimensions(&self) -> (usize, usize) {
        let dim = Self::SIZE_DIMS[self.size_idx()];
        (dim, dim)
    }

    pub fn map_type_label(&self) -> &'static str {
        Self::MAP_TYPE_LABELS[self.map_type_idx()]
    }

    pub fn size_label(&self) -> &'static str {
        Self::SIZE_LABELS[self.size_idx()]
    }

    pub fn cycle_map_type(&mut self) {
        self.map_type = (self.map_type_idx() + 1) % Self::MAP_TYPE_LABELS.len();
    }

    pub fn cycle_size(&mut self) {
        self.size = (self.size_idx() + 1) % Self::SIZE_LABELS.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_standard_medium() {
        let cfg = SkirmishConfig::default();
        assert_eq!(cfg.map_type_label(), "Standard");
        assert_eq!(cfg.size_label(), "Medium");
        assert!(matches!(cfg.map_type(), MapType::Standard));
        assert_eq!(cfg.dimensions(), (32, 32));
    }

    #[test]
    fn every_skirmish_config_generates_a_playable_map() {
        // Drive the real path the setup screen uses: each map type + size must
        // produce a bootable game with a live player heart. This is what makes
        // the procedural generator reachable as a sandbox.
        let game_data = crate::data::GameData::load().expect("game data should load");
        let mut cfg = SkirmishConfig::default();
        for _type_step in 0..SkirmishConfig::MAP_TYPE_LABELS.len() {
            for _size_step in 0..SkirmishConfig::SIZE_LABELS.len() {
                let (w, h) = cfg.dimensions();
                let state = crate::state::game_state::GameState::new_with_map_type(
                    w,
                    h,
                    &game_data,
                    cfg.map_type(),
                );
                assert_eq!((state.dungeon.width, state.dungeon.height), (w, h));
                assert!(
                    state.find_dungeon_heart_position().is_some(),
                    "{} {} skirmish should have a player heart",
                    cfg.map_type_label(),
                    cfg.size_label()
                );
                cfg.cycle_size();
            }
            cfg.cycle_map_type();
        }
    }

    #[test]
    fn cycling_wraps_and_maps_to_generator_inputs() {
        let mut cfg = SkirmishConfig::default();
        cfg.cycle_map_type(); // Rich
        assert!(matches!(cfg.map_type(), MapType::Rich));
        cfg.cycle_map_type(); // Hazardous
        assert!(matches!(cfg.map_type(), MapType::Hazardous));
        cfg.cycle_map_type(); // wraps to Standard
        assert!(matches!(cfg.map_type(), MapType::Standard));

        cfg.cycle_size(); // Large
        assert_eq!(cfg.dimensions(), (48, 48));
        cfg.cycle_size(); // wraps to Small
        assert_eq!(cfg.dimensions(), (24, 24));
        assert_eq!(cfg.size_label(), "Small");
    }
}
