use macroquad::prelude::*;
use std::collections::HashMap;

pub struct GraphicsCache {
    pub tile_textures: HashMap<String, Texture2D>,
    pub monster_textures: HashMap<String, Texture2D>,
    pub hero_textures: HashMap<String, Texture2D>,
    pub ui_textures: HashMap<String, Texture2D>,
    pub projectile_textures: HashMap<String, Texture2D>,
}

impl GraphicsCache {
    pub fn new() -> Self {
        Self {
            tile_textures: HashMap::new(),
            monster_textures: HashMap::new(),
            hero_textures: HashMap::new(),
            ui_textures: HashMap::new(),
            projectile_textures: HashMap::new(),
        }
    }

    pub async fn load_all(game_data: Option<&crate::data::GameData>) -> Result<Self, String> {
        let mut cache = Self::new();

        // Load tile textures
        let tile_types = vec![
            "solid_rock", "earth", "claimed_floor", "reinforced_wall",
            "gold_vein", "gem_seam", "mana_crystal",
            "lava", "water", "bridge",
            "corrupted_floor", "ancient_rune_floor",
            "dungeon_heart", "lair", "hatchery", "treasury", "workshop",
            "training_room", "library", "prison", "guard_post", "ritual_circle", "monster_spawner",
            "spike_trap",
        ];
        
        let hero_building_tiles = vec![
            "town_hall", "barracks", "archery_range", "church", "mage_tower", 
            "stable", "armory", "hero_wall", "hero_gate"
        ];

        for tile_type in tile_types {
            let path = format!("assets/tiles/{}.png", tile_type);
            // In a real scenario we'd use robust error handling.
            // For macroquad web, loading is async.
            match load_texture(&path).await {
                Ok(tex) => {
                    tex.set_filter(FilterMode::Nearest);
                    cache.tile_textures.insert(tile_type.to_string(), tex);
                }
                Err(e) => {
                    println!("Failed to load texture {}: {}", path, e);
                    // generate a placeholder?
                }
            }
        }

        for tile_type in hero_building_tiles {
            let path = format!("assets/tiles/hero_buildings/{}.png", tile_type);
            match load_texture(&path).await {
                Ok(tex) => {
                    tex.set_filter(FilterMode::Nearest);
                    cache.tile_textures.insert(tile_type.to_string(), tex);
                }
                Err(e) => {
                     println!("Failed to load hero building texture {}: {}", path, e);
                }
            }
        }

        // Load unit textures (creatures)
        let creatures = vec![
            "imp", "goblin", "orc", "warlock", "troll", "skeleton", "demon_spawn",
            "spider", "lizard"  // Wild/neutral monsters
        ];
        for creature in creatures {
            let path = format!("assets/sprites/monsters/{}.png", creature);
            match load_texture(&path).await {
                Ok(tex) => {
                    tex.set_filter(FilterMode::Nearest);
                    cache.monster_textures.insert(creature.to_string(), tex);
                }
                Err(e) => println!("Failed to load texture {}: {}", path, e),
            }
        }
        
        // Load hero textures
        let heroes = vec![
            "peasant_militia", "scout", "acolyte", "knight", "archer", "battle_cleric", 
            "rogue", "paladin", "wizard", "inquisitor", "knight_commander", 
            "high_priest", "archmage", "champion_of_light"
        ];
        for hero in heroes {
            let path = format!("assets/sprites/heroes/{}.png", hero);
            match load_texture(&path).await {
                Ok(tex) => {
                    tex.set_filter(FilterMode::Nearest);
                    cache.hero_textures.insert(hero.to_string(), tex);
                }
                Err(e) => println!("Failed to load texture {}: {}", path, e),
            }
        }

        // Load UI textures
        let ui_textures = vec!["main_menu_bg"];
        for tex_name in ui_textures {
            let path = format!("assets/ui/{}.png", tex_name);
             match load_texture(&path).await {
                Ok(tex) => {
                    tex.set_filter(FilterMode::Linear); // UI usually looks better with Linear
                    cache.ui_textures.insert(tex_name.to_string(), tex);
                }
                Err(e) => println!("Failed to load texture {}: {}", path, e),
            }
        }

        // Load Spell Icons dynamically from GameData
        // Only load icons that are known to exist to avoid panics from missing files
        let known_spell_icons = ["icons/spells/heal.png", "icons/spells/lightning.png",
                                  "icons/spells/speed.png", "icons/spells/summon.png"];
        if let Some(data) = game_data {
            for spell in data.spells.values() {
                let icon_path = &spell.visual.icon;
                if !icon_path.is_empty() && known_spell_icons.contains(&icon_path.as_str()) {
                    let path = format!("assets/{}", icon_path);
                    match load_texture(&path).await {
                        Ok(tex) => {
                            tex.set_filter(FilterMode::Nearest);
                            println!("Loaded spell icon: {} -> {}", icon_path, path);
                            cache.ui_textures.insert(icon_path.clone(), tex);
                        }
                        Err(e) => println!("Failed to load spell icon {}: {}", path, e),
                    }
                }
            }
        }

        // Load projectile textures
        let projectiles = vec!["projectile_melee", "projectile_arrow", "projectile_magic"];
        for projectile in projectiles {
            let path = format!("assets/sprites/projectiles/{}.png", projectile);
            match load_texture(&path).await {
                Ok(tex) => {
                    tex.set_filter(FilterMode::Nearest);
                    cache.projectile_textures.insert(projectile.to_string(), tex);
                }
                Err(e) => println!("Failed to load projectile texture {}: {}", path, e),
            }
        }

        Ok(cache)
    }
}
