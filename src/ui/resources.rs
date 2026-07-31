use crate::sprite_variation::build_variation_cache;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::sprite::SpriteVariationCache;
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
    /// Hero-faction building art, keyed by `building_id`.
    ///
    /// Separate from `tile_textures` because the two id spaces collide: the
    /// hero base's `barracks` and the player's `barracks` room are different
    /// buildings with different art, and merging both into one map meant one
    /// silently won. Grid tiles disambiguate on ownership (see the renderer).
    pub building_textures: HashMap<String, Texture2D>,
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
            building_textures: HashMap::new(),
            monster_textures: HashMap::new(),
            hero_textures: HashMap::new(),
            ui_textures: HashMap::new(),
            projectile_textures: HashMap::new(),
            variation_cache: RefCell::new(build_variation_cache()),
        }
    }

    /// Art for a hero-faction building, whether it is drawn as a structure
    /// entity or as a grid tile inside the hero base.
    pub fn building_texture(&self, building_id: &str) -> Option<&Texture2D> {
        self.building_textures.get(building_id)
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

        let placeholder = missing_texture();

        // Load tile textures. The manifest is derived from the content data
        // rather than listed here, so shipping a tile, room or trap is enough
        // to put its art on screen.
        for (key, path) in tile_texture_manifest(game_data) {
            let load_path = resolve_asset_path(game_data, &path);
            match loader.load(&load_path, FilterMode::Nearest).await {
                Ok(tex) => {
                    cache.tile_textures.insert(key, tex);
                }
                Err(e) => {
                    println!(
                        "Missing tile texture {} ({}) — using placeholder",
                        load_path, e
                    );
                    cache.tile_textures.insert(key, placeholder.clone());
                }
            }
        }

        for (key, path) in building_texture_manifest(game_data) {
            let load_path = resolve_asset_path(game_data, &path);
            match loader.load(&load_path, FilterMode::Nearest).await {
                Ok(tex) => {
                    cache.building_textures.insert(key, tex);
                }
                Err(e) => {
                    println!(
                        "Missing hero building texture {} ({}) — using placeholder",
                        load_path, e
                    );
                    cache.building_textures.insert(key, placeholder.clone());
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
                Err(e) => {
                    // A placeholder rather than nothing: an un-arted creature
                    // is invisible otherwise, so new roster entries can't be
                    // playtested until their sprite lands.
                    println!(
                        "Missing creature sprite {} ({}) — using placeholder",
                        load_path, e
                    );
                    cache
                        .monster_textures
                        .insert(creature.clone(), placeholder.clone());
                }
            }
        }

        // Load hero textures
        // Data-driven off the hero roster (same rationale as creatures above):
        // adding a hero to `heroes.json` wires its sprite with no list to update.
        let heroes: Vec<String> = match game_data {
            Some(data) => data.heroes.keys().cloned().collect(),
            None => [
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
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        };
        for hero in &heroes {
            let path = format!("assets/sprites/heroes/{}.png", hero);
            let load_path = resolve_asset_path(game_data, &path);
            match loader.load(&load_path, FilterMode::Nearest).await {
                Ok(tex) => {
                    cache.hero_textures.insert(hero.clone(), tex);
                }
                Err(e) => {
                    println!(
                        "Missing hero sprite {} ({}) — using placeholder",
                        load_path, e
                    );
                    cache
                        .hero_textures
                        .insert(hero.clone(), placeholder.clone());
                }
            }
        }

        // Load UI textures
        let ui_textures = vec!["main_menu_bg"];
        for tex_name in ui_textures {
            // JPEG: stretched full-screen behind the menu, where lossless PNG
            // cost 0.8 MB for no visible benefit.
            let path = format!("assets/ui/{}.jpg", tex_name);
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

/// Every `(lookup key, asset path)` the tile layer can be asked to draw.
///
/// The renderer looks a floor or wall up by `tile.tile_type` and a trap by
/// `trap.trap_type`, so in every case the key is the content id and the
/// manifest is simply "every id the data declares". This used to be a
/// hand-maintained 24-entry list, which quietly stranded generated art —
/// temple, graveyard, kennel, torture chamber, every trap but the spike trap,
/// all ten floor variants — with nothing to flag the drift. Deriving it from
/// the data means adding a room or trap to JSON is enough to render it.
///
/// Paths come from each entry's `visual` block, falling back to the
/// `assets/tiles/<id>.png` convention that the graphics generator emits.
fn tile_texture_manifest(game_data: Option<&crate::data::GameData>) -> Vec<(String, String)> {
    let Some(data) = game_data else {
        // No content loaded (the menu before a scenario starts): the terrain
        // the map generator can emit unaided is enough to draw a preview.
        return [
            "solid_rock",
            "earth",
            "claimed_floor",
            "gold_vein",
            "dungeon_heart",
        ]
        .iter()
        .map(|id| (id.to_string(), format!("assets/tiles/{}.png", id)))
        .collect();
    };

    let mut manifest: Vec<(String, String)> = Vec::new();

    for (id, tile) in &data.tiles {
        manifest.push((id.clone(), format!("assets/{}", tile.visual.sprite)));
    }

    for (id, room) in &data.rooms {
        // Keyed by the string the grid actually stores, which differs from the
        // data id for the training hall.
        let key = crate::data::rooms::room_tile_type(id).to_string();
        manifest.push((key, format!("assets/{}", room.visual.floor_sprite)));
    }

    // Traps carry no `visual` block, so they rely on the filename convention.
    for id in data.traps.keys() {
        manifest.push((id.clone(), format!("assets/tiles/{}.png", id)));
    }

    // A resource pile on the floor is drawn from the tile atlas but is not a
    // tile type, so nothing above covers it.
    manifest.push((
        "gold_pile".to_string(),
        "assets/tiles/gold_pile.png".to_string(),
    ));

    manifest
}

/// Every `(building_id, asset path)` for hero-faction buildings.
fn building_texture_manifest(game_data: Option<&crate::data::GameData>) -> Vec<(String, String)> {
    let Some(data) = game_data else {
        return Vec::new();
    };

    data.hero_buildings
        .iter()
        .map(|(id, building)| (id.clone(), format!("assets/{}", building.visual.tile)))
        .collect()
}

/// A magenta/black checker stand-in for art that failed to load.
///
/// Without it a missing texture drew *nothing* — an invisible floor, an
/// invisible creature — which reads as a rendering bug rather than as missing
/// art, and blocks playtesting content whose sprite hasn't been drawn yet.
fn missing_texture() -> Texture2D {
    const SIZE: u16 = 64;
    const CELL: u32 = 8;

    let mut image = Image::gen_image_color(SIZE, SIZE, MAGENTA);
    for y in 0..SIZE as u32 {
        for x in 0..SIZE as u32 {
            if (x / CELL + y / CELL).is_multiple_of(2) {
                image.set_pixel(x, y, BLACK);
            }
        }
    }

    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
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
