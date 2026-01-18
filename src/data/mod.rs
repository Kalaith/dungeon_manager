pub mod tiles;
pub mod rooms;
pub mod monsters;
pub mod heroes;
pub mod spells;
pub mod traps;
pub mod technologies;
pub mod hero_buildings;
pub mod game_config;

use std::collections::HashMap;
use std::error::Error;

pub use tiles::TileData;
pub use rooms::RoomData;
pub use monsters::MonsterData;
pub use heroes::HeroData;
pub use spells::SpellData;
pub use traps::TrapData;
pub use technologies::TechData;
pub use hero_buildings::HeroBuildingData;
pub use game_config::GameConfig;

pub struct GameData {
    pub tiles: HashMap<String, TileData>,
    pub rooms: HashMap<String, RoomData>,
    pub monsters: HashMap<String, MonsterData>,
    pub heroes: HashMap<String, HeroData>,
    pub spells: HashMap<String, SpellData>,
    pub traps: HashMap<String, TrapData>,
    pub technologies: HashMap<String, TechData>,
    pub hero_buildings: HashMap<String, HeroBuildingData>,
    pub config: GameConfig,
}

impl GameData {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let tiles = tiles::load_tiles()?;
        let rooms = rooms::load_rooms()?;
        let monsters = monsters::load_monsters()?;
        let heroes = heroes::load_heroes()?;
        let spells = spells::load_spells()?;
        let traps = traps::load_traps()?;
        let technologies = technologies::load_technologies()?;
        let hero_buildings = hero_buildings::load_hero_buildings()?;
        let config = game_config::load_game_config()?;

        Ok(Self {
            tiles,
            rooms,
            monsters,
            heroes,
            spells,
            traps,
            technologies,
            hero_buildings,
            config,
        })
    }
}
