pub mod tiles;
pub mod rooms;
pub mod monsters;
pub mod heroes;
pub mod spells;
pub mod traps;
pub mod technologies;

use std::collections::HashMap;
use std::error::Error;

pub use tiles::TileData;
pub use rooms::RoomData;
pub use monsters::MonsterData;
pub use heroes::HeroData;
pub use spells::SpellData;
pub use traps::TrapData;
pub use technologies::TechData;

pub struct GameData {
    pub tiles: HashMap<String, TileData>,
    pub rooms: HashMap<String, RoomData>,
    pub monsters: HashMap<String, MonsterData>,
    pub heroes: HashMap<String, HeroData>,
    pub spells: HashMap<String, SpellData>,
    pub traps: HashMap<String, TrapData>,
    pub technologies: HashMap<String, TechData>,
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

        Ok(Self {
            tiles,
            rooms,
            monsters,
            heroes,
            spells,
            traps,
            technologies
        })
    }
}
