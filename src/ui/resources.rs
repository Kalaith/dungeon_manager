use crate::sprite_variation::SpriteVariationCache;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use std::cell::RefCell;
use std::collections::HashMap;

const ASSET_PACKS: [&str; 4] = [
    "assets/tiles.zip",
    "assets/sprites.zip",
    "assets/icons.zip",
    "assets/ui.zip",
];

pub struct GraphicsCache {
    pub tile_textures: HashMap<String, Texture2D>,
    pub monster_textures: HashMap<String, Texture2D>,
    pub hero_textures: HashMap<String, Texture2D>,
    pub ui_textures: HashMap<String, Texture2D>,
    pub projectile_textures: HashMap<String, Texture2D>,
    /// Cache for runtime-generated sprite variations (uses RefCell for interior mutability)
    variation_cache: RefCell<SpriteVariationCache>,
}

impl GraphicsCache {
    pub fn new() -> Self {
        Self {
            tile_textures: HashMap::new(),
            monster_textures: HashMap::new(),
            hero_textures: HashMap::new(),
            ui_textures: HashMap::new(),
            projectile_textures: HashMap::new(),
            variation_cache: RefCell::new(SpriteVariationCache::new()),
        }
    }

    /// Get a varied texture for a creature, generating it if needed
    /// Uses interior mutability (RefCell) so this can be called with &self
    pub fn get_creature_texture(&self, creature_id: &str, visual_seed: u64) -> Option<Texture2D> {
        // If seed is 0, return base texture (no variation)
        if visual_seed == 0 {
            return self.monster_textures.get(creature_id).cloned();
        }

        // Get base texture
        let base_texture = self.monster_textures.get(creature_id)?;

        // Get or create varied texture (borrow RefCell mutably)
        Some(self.variation_cache.borrow_mut().get_or_create(
            creature_id,
            visual_seed,
            base_texture,
        ))
    }

    /// Get a varied texture for a hero, generating it if needed
    /// Uses interior mutability (RefCell) so this can be called with &self
    pub fn get_hero_texture(&self, hero_id: &str, visual_seed: u64) -> Option<Texture2D> {
        // If seed is 0, return base texture (no variation)
        if visual_seed == 0 {
            return self.hero_textures.get(hero_id).cloned();
        }

        // Get base texture
        let base_texture = self.hero_textures.get(hero_id)?;

        // Get or create varied texture (borrow RefCell mutably)
        Some(
            self.variation_cache
                .borrow_mut()
                .get_or_create(hero_id, visual_seed, base_texture),
        )
    }

    pub async fn load_all(game_data: Option<&crate::data::GameData>) -> Result<Self, String> {
        let mut cache = Self::new();
        let mut loader = PackedTextureLoader::new().await;

        // Load tile textures
        let tile_types = vec![
            "solid_rock",
            "earth",
            "claimed_floor",
            "reinforced_wall",
            "gold_vein",
            "gem_seam",
            "mana_crystal",
            "lava",
            "water",
            "bridge",
            "corrupted_floor",
            "ancient_rune_floor",
            "dungeon_heart",
            "lair",
            "hatchery",
            "treasury",
            "workshop",
            "training_room",
            "library",
            "prison",
            "guard_post",
            "ritual_circle",
            "monster_spawner",
            "spike_trap",
        ];

        let hero_building_tiles = vec![
            "town_hall",
            "barracks",
            "archery_range",
            "church",
            "mage_tower",
            "stable",
            "armory",
            "hero_wall",
            "hero_gate",
        ];

        for tile_type in tile_types {
            let path = format!("assets/tiles/{}.png", tile_type);
            let load_path = resolve_asset_path(game_data, &path);
            match loader.load(&load_path, FilterMode::Nearest).await {
                Ok(tex) => {
                    cache.tile_textures.insert(tile_type.to_string(), tex);
                }
                Err(e) => {
                    println!("Failed to load texture {}: {}", load_path, e);
                    // generate a placeholder?
                }
            }
        }

        for tile_type in hero_building_tiles {
            let path = format!("assets/tiles/hero_buildings/{}.png", tile_type);
            let load_path = resolve_asset_path(game_data, &path);
            match loader.load(&load_path, FilterMode::Nearest).await {
                Ok(tex) => {
                    cache.tile_textures.insert(tile_type.to_string(), tex);
                }
                Err(e) => {
                    println!("Failed to load hero building texture {}: {}", load_path, e);
                }
            }
        }

        // Load unit textures (creatures). Data-driven off the monster roster so
        // every monster in `monsters.json` gets its sprite — adding a monster is
        // then enough to wire it (no hardcoded list to keep in sync). Falls back
        // to the core set only when game data isn't available.
        let creatures: Vec<String> = match game_data {
            Some(data) => data.monsters.keys().cloned().collect(),
            None => [
                "imp",
                "goblin",
                "orc",
                "warlock",
                "troll",
                "skeleton",
                "demon_spawn",
                "spider",
                "lizard",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        };
        for creature in &creatures {
            let path = format!("assets/sprites/monsters/{}.png", creature);
            let load_path = resolve_asset_path(game_data, &path);
            match loader.load(&load_path, FilterMode::Nearest).await {
                Ok(tex) => {
                    cache.monster_textures.insert(creature.clone(), tex);
                }
                Err(e) => println!("Failed to load texture {}: {}", load_path, e),
            }
        }

        // Load hero textures
        let heroes = vec![
            "peasant_militia",
            "scout",
            "acolyte",
            "knight",
            "archer",
            "battle_cleric",
            "rogue",
            "paladin",
            "wizard",
            "inquisitor",
            "knight_commander",
            "high_priest",
            "archmage",
            "champion_of_light",
        ];
        for hero in heroes {
            let path = format!("assets/sprites/heroes/{}.png", hero);
            let load_path = resolve_asset_path(game_data, &path);
            match loader.load(&load_path, FilterMode::Nearest).await {
                Ok(tex) => {
                    cache.hero_textures.insert(hero.to_string(), tex);
                }
                Err(e) => println!("Failed to load texture {}: {}", load_path, e),
            }
        }

        // Load UI textures
        let ui_textures = vec!["main_menu_bg"];
        for tex_name in ui_textures {
            let path = format!("assets/ui/{}.png", tex_name);
            let load_path = resolve_asset_path(game_data, &path);
            match loader.load(&load_path, FilterMode::Linear).await {
                Ok(tex) => {
                    cache.ui_textures.insert(tex_name.to_string(), tex);
                }
                Err(e) => println!("Failed to load texture {}: {}", load_path, e),
            }
        }

        // Load Spell Icons dynamically from GameData
        // Only load icons that are known to exist to avoid panics from missing files
        let known_spell_icons = [
            "icons/spells/heal.png",
            "icons/spells/lightning.png",
            "icons/spells/speed.png",
            "icons/spells/summon.png",
        ];
        if let Some(data) = game_data {
            for spell in data.spells.values() {
                let icon_path = &spell.visual.icon;
                if !icon_path.is_empty() && known_spell_icons.contains(&icon_path.as_str()) {
                    let path = format!("assets/{}", icon_path);
                    let load_path = resolve_asset_path(game_data, &path);
                    match loader.load(&load_path, FilterMode::Nearest).await {
                        Ok(tex) => {
                            println!("Loaded spell icon: {} -> {}", icon_path, load_path);
                            cache.ui_textures.insert(icon_path.clone(), tex);
                        }
                        Err(e) => println!("Failed to load spell icon {}: {}", load_path, e),
                    }
                }
            }
        }

        // Load projectile textures
        let projectiles = vec!["projectile_melee", "projectile_arrow", "projectile_magic"];
        for projectile in projectiles {
            let path = format!("assets/sprites/projectiles/{}.png", projectile);
            let load_path = resolve_asset_path(game_data, &path);
            match loader.load(&load_path, FilterMode::Nearest).await {
                Ok(tex) => {
                    cache
                        .projectile_textures
                        .insert(projectile.to_string(), tex);
                }
                Err(e) => println!("Failed to load projectile texture {}: {}", load_path, e),
            }
        }

        Ok(cache)
    }
}

fn resolve_asset_path(game_data: Option<&crate::data::GameData>, path: &str) -> String {
    game_data
        .map(|data| data.resolve_asset_path(path))
        .unwrap_or_else(|| path.to_string())
}

struct PackedTextureLoader {
    assets: AssetManager,
}

impl PackedTextureLoader {
    async fn new() -> Self {
        let mut assets = AssetManager::new();
        for path in ASSET_PACKS {
            assets.load_asset_pack(path).await.ok();
        }

        Self { assets }
    }

    async fn load(&mut self, path: &str, filter: FilterMode) -> Result<Texture2D, String> {
        self.assets
            .load_texture_with_filter(path, path, filter)
            .await?;
        self.assets
            .get_texture(path)
            .cloned()
            .ok_or_else(|| format!("Texture was not cached after loading: {}", path))
    }
}
