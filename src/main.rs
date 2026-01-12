use macroquad::prelude::*;
use std::collections::HashMap;

mod data;
mod engine;
mod state;
mod ui;

use data::GameData;
use state::game_state::GameState;
use state::MapType;
use state::Ownership;

pub enum GamePhase {
    Loading,
    MainMenu,
    Playing(GameState),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InteractionMode {
    None,
    Dig,
    BuildRoom(String), // room_type_id
    PlaceSpawner,
}

pub struct GraphicsCache {
    tile_textures: HashMap<String, Texture2D>,
    monster_textures: HashMap<String, Texture2D>,
    hero_textures: HashMap<String, Texture2D>,
}

impl GraphicsCache {
    fn new() -> Self {
        Self {
            tile_textures: HashMap::new(),
            monster_textures: HashMap::new(),
            hero_textures: HashMap::new(),
        }
    }

    async fn load_all() -> Result<Self, String> {
        let mut cache = Self::new();

        // List of all tile types
        let tile_types = vec![
            "solid_rock", "earth", "claimed_floor", "reinforced_wall",
            "gold_vein", "gem_seam", "lava", "water", "bridge",
            "corrupted_floor", "ancient_rune_floor", "dungeon_heart",
            "lair", "hatchery", "treasury", "workshop", "training_room",
            "prison", "guard_post", "ritual_circle", "monster_spawner"
        ];

        // Load tile textures
        for tile_type in tile_types {
            let path = format!("assets/tiles/{}.png", tile_type);
            match load_texture(&path).await {
                Ok(texture) => {
                    texture.set_filter(FilterMode::Nearest);
                    cache.tile_textures.insert(tile_type.to_string(), texture);
                    eprintln!("Loaded tile texture: {}", tile_type);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load tile texture {}: {:?}", tile_type, e);
                }
            }
        }

        // List of all monster types
        let monster_types = vec![
            "imp", "goblin", "orc", "warlock", "troll", "skeleton", "demon_spawn"
        ];

        // Load monster textures
        for monster_type in monster_types {
            let path = format!("assets/sprites/monsters/{}.png", monster_type);
            match load_texture(&path).await {
                Ok(texture) => {
                    texture.set_filter(FilterMode::Nearest);
                    cache.monster_textures.insert(monster_type.to_string(), texture);
                    eprintln!("Loaded monster texture: {}", monster_type);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load monster texture {}: {:?}", monster_type, e);
                }
            }
        }

        // List of all hero types
        let hero_types = vec![
            "peasant_militia", "scout", "acolyte", "knight", "archer",
            "battle_cleric", "rogue", "paladin", "wizard", "inquisitor",
            "knight_commander", "high_priest", "archmage", "champion_of_light"
        ];

        // Load hero textures
        for hero_type in hero_types {
            let path = format!("assets/sprites/heroes/{}.png", hero_type);
            match load_texture(&path).await {
                Ok(texture) => {
                    texture.set_filter(FilterMode::Nearest);
                    cache.hero_textures.insert(hero_type.to_string(), texture);
                    eprintln!("Loaded hero texture: {}", hero_type);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load hero texture {}: {:?}", hero_type, e);
                }
            }
        }

        Ok(cache)
    }
}

pub struct Game {
    phase: GamePhase,
    game_data: Option<GameData>,
    graphics_cache: Option<GraphicsCache>,
    interaction_mode: InteractionMode,
    hovered_tile: Option<state::tile_state::TilePos>,
    build_menu: ui::build_menu::BuildMenu,
    spell_bar: ui::spell_bar::SpellBar,
    selected_map_type: MapType,
    selected_entity: Option<state::entities::EntityId>,
    spell_shop_open: bool,
}

impl Game {
    fn new() -> Self {
        let mut spell_bar = ui::spell_bar::SpellBar::new();
        spell_bar.update_position();

        Self {
            phase: GamePhase::Loading,
            game_data: None,
            graphics_cache: None,
            interaction_mode: InteractionMode::None,
            hovered_tile: None,
            build_menu: ui::build_menu::BuildMenu::new(),
            spell_bar,
            selected_map_type: MapType::Standard,
            selected_entity: None,
            spell_shop_open: false,
        }
    }

    async fn load_resources(&mut self) {
        // Load game data
        if self.game_data.is_none() {
            match GameData::load() {
                Ok(data) => {
                    eprintln!("Successfully loaded game data!");
                    eprintln!("  - {} tiles", data.tiles.len());
                    eprintln!("  - {} rooms", data.rooms.len());
                    eprintln!("  - {} monsters", data.monsters.len());
                    eprintln!("  - {} heroes", data.heroes.len());
                    eprintln!("  - {} spells", data.spells.len());
                    self.game_data = Some(data);
                }
                Err(e) => {
                    eprintln!("Failed to load game data: {}", e);
                    return;
                }
            }
        }

        // Load graphics
        if self.graphics_cache.is_none() {
            eprintln!("Loading graphics...");
            match GraphicsCache::load_all().await {
                Ok(cache) => {
                    eprintln!("Successfully loaded {} tile textures", cache.tile_textures.len());
                    eprintln!("Successfully loaded {} monster textures", cache.monster_textures.len());
                    eprintln!("Successfully loaded {} hero textures", cache.hero_textures.len());
                    self.graphics_cache = Some(cache);
                    self.phase = GamePhase::MainMenu;
                }
                Err(e) => {
                    eprintln!("Failed to load graphics: {}", e);
                }
            }
        }
    }

    fn update(&mut self, dt: f32) {
        match &mut self.phase {
            GamePhase::Loading => {
                // Loading is handled asynchronously in main loop
            }
            GamePhase::MainMenu => {
                let mouse_pos = mouse_position();

                // Map type selection buttons
                let map_button_y = screen_height() / 2.0 - 50.0;
                let map_button_width = 160.0;
                let map_button_height = 40.0;
                let map_button_spacing = 10.0;
                let total_width = (map_button_width + map_button_spacing) * 4.0 - map_button_spacing;
                let map_button_start_x = screen_width() / 2.0 - total_width / 2.0;

                // Check each map type button
                let map_types = [
                    (MapType::Standard, "Standard", 0),
                    (MapType::Rich, "Rich", 1),
                    (MapType::Hazardous, "Hazardous", 2),
                    (MapType::Test, "Test", 3),
                ];

                if is_mouse_button_pressed(MouseButton::Left) {
                    for (map_type, _label, index) in &map_types {
                        let btn_x = map_button_start_x + (map_button_width + map_button_spacing) * (*index as f32);
                        if mouse_pos.0 >= btn_x
                            && mouse_pos.0 <= btn_x + map_button_width
                            && mouse_pos.1 >= map_button_y
                            && mouse_pos.1 <= map_button_y + map_button_height
                        {
                            self.selected_map_type = *map_type;
                        }
                    }
                }

                // Check for SPACE key or mouse click on start button
                let button_x = screen_width() / 2.0 - 100.0;
                let button_y = screen_height() / 2.0 + 60.0;
                let button_width = 200.0;
                let button_height = 50.0;

                let should_start = is_key_pressed(KeyCode::Space)
                    || (is_mouse_button_pressed(MouseButton::Left)
                        && mouse_pos.0 >= button_x
                        && mouse_pos.0 <= button_x + button_width
                        && mouse_pos.1 >= button_y
                        && mouse_pos.1 <= button_y + button_height);

                if should_start {
                    if let Some(ref game_data) = self.game_data {
                        let game_state = GameState::new_with_map_type(
                            50,
                            50,
                            game_data,
                            self.selected_map_type,
                        );
                        self.phase = GamePhase::Playing(game_state);
                    }
                }
            }
            GamePhase::Playing(state) => {
                if let Some(ref game_data) = self.game_data {
                    state.update(dt, game_data);
                }

                // Camera controls (WASD)
                let camera_speed = 300.0 * dt;
                if is_key_down(KeyCode::W) {
                    state.camera_y += camera_speed;
                }
                if is_key_down(KeyCode::S) {
                    state.camera_y -= camera_speed;
                }
                if is_key_down(KeyCode::A) {
                    state.camera_x += camera_speed;
                }
                if is_key_down(KeyCode::D) {
                    state.camera_x -= camera_speed;
                }

                // Mode switching (keyboard shortcuts still work)
                if is_key_pressed(KeyCode::Key1) {
                    self.interaction_mode = InteractionMode::Dig;
                }
                if is_key_pressed(KeyCode::Key2) {
                    self.interaction_mode = InteractionMode::BuildRoom("lair".to_string());
                }
                if is_key_pressed(KeyCode::Key3) {
                    self.interaction_mode = InteractionMode::BuildRoom("hatchery".to_string());
                }
                if is_key_pressed(KeyCode::Key4) {
                    self.interaction_mode = InteractionMode::BuildRoom("treasury".to_string());
                }
                if is_key_pressed(KeyCode::Key5) {
                    self.interaction_mode = InteractionMode::PlaceSpawner;
                }
                if is_key_pressed(KeyCode::T) {
                    self.interaction_mode = InteractionMode::BuildRoom("training_room".to_string());
                }
                if is_key_pressed(KeyCode::L) {
                    self.interaction_mode = InteractionMode::BuildRoom("library".to_string());
                }
                if is_key_pressed(KeyCode::Escape) {
                    self.interaction_mode = InteractionMode::None;
                    // Also clear any selected spell
                    self.spell_bar.clear_selection();
                }

                // Mouse input - get hovered tile
                let mouse_pos = mouse_position();
                let hovered_tile = engine::tile_grid::screen_to_tile(
                    mouse_pos.0,
                    mouse_pos.1,
                    state.camera_x,
                    state.camera_y,
                    ui::core::TILE_WIDTH,
                    ui::core::TILE_HEIGHT,
                );
                self.hovered_tile = Some(hovered_tile);

                // Handle spell bar input
                if let Some(ref game_data) = self.game_data {
                    if let Some(spell_action) = self.spell_bar.handle_input(&state.player, &game_data.spells, Some(hovered_tile)) {
                        match spell_action {
                            ui::spell_bar::SpellAction::SelectSpell(spell_id) => {
                                eprintln!("Selected spell: {}", spell_id);
                            }
                            ui::spell_bar::SpellAction::CastSpell(spell_id, target) => {
                                let (target_pos, target_entity) = match target {
                                    ui::spell_bar::SpellTarget::Tile(pos) => (Some(pos), None),
                                    ui::spell_bar::SpellTarget::Entity(entity_id) => (None, Some(entity_id)),
                                    ui::spell_bar::SpellTarget::None => (None, None),
                                };

                                let cast_result = engine::spell_effects::cast_spell(
                                    &spell_id,
                                    state,
                                    game_data,
                                    target_pos,
                                    target_entity,
                                );

                                match cast_result {
                                    engine::spell_effects::CastResult::Success => {
                                        eprintln!("Spell cast successfully: {}", spell_id);
                                        state.player.record_spell_cast(spell_id);
                                    }
                                    engine::spell_effects::CastResult::InsufficientMana => {
                                        eprintln!("Not enough mana to cast {}", spell_id);
                                    }
                                    engine::spell_effects::CastResult::OnCooldown => {
                                        eprintln!("Spell {} is on cooldown", spell_id);
                                    }
                                    engine::spell_effects::CastResult::MaxCapReached => {
                                        eprintln!("Cannot cast {}: maximum capacity reached", spell_id);
                                    }
                                    _ => {
                                        eprintln!("Failed to cast spell: {:?}", cast_result);
                                    }
                                }
                            }
                            ui::spell_bar::SpellAction::CancelCast => {
                                eprintln!("Spell cast cancelled");
                            }
                        }
                    }
                }

                // Handle build menu clicks
                if let Some(new_mode) = self.build_menu.handle_input(&state.player) {
                    self.interaction_mode = new_mode;
                }

                // Check if mouse is over UI (build menu or spell bar)
                let mouse_over_ui = self.build_menu.is_mouse_over_panel() || self.spell_bar.is_mouse_over_panel();

                // Handle right-click to cancel dig marks
                if is_mouse_button_pressed(MouseButton::Right) && !mouse_over_ui {
                    if self.interaction_mode == InteractionMode::Dig {
                        if let Some(tile) = state.get_tile_mut(hovered_tile) {
                            if tile.marked_for_dig {
                                tile.marked_for_dig = false;
                            }
                        }
                    }
                }

                // Handle mouse click - only if not clicking on UI
                if is_mouse_button_pressed(MouseButton::Left) && !mouse_over_ui {
                    use state::tile_state::Ownership;

                    match &self.interaction_mode {
                        InteractionMode::Dig => {
                            // Mark tile for digging (imps will dig it) - FREE
                            if let Some(tile) = state.get_tile_mut(hovered_tile) {
                                if (tile.tile_type == "earth" || tile.tile_type == "gold_vein" || tile.tile_type == "gem_seam")
                                   && tile.ownership == Ownership::Unclaimed {
                                    tile.marked_for_dig = true;
                                }
                            }
                        }
                        InteractionMode::BuildRoom(room_type) => {
                            // Build room on claimed tile - COSTS GOLD
                            let cost = match room_type.as_str() {
                                "lair" => 10,
                                "hatchery" => 15,
                                "treasury" => 20,
                                "training_room" => 25,
                                "library" => 30,
                                _ => 10,
                            };

                            // Check if player can afford and tile is valid
                            let can_build = state.player.gold >= cost;
                            let is_valid_tile = if let Some(tile) = state.get_tile(hovered_tile) {
                                tile.ownership == Ownership::Player && tile.room_id.is_none()
                            } else {
                                false
                            };

                            if can_build && is_valid_tile {
                                // Deduct cost first
                                state.player.gold -= cost;

                                // Build room
                                if let Some(tile) = state.get_tile_mut(hovered_tile) {
                                    tile.tile_type = room_type.clone();
                                }

                                // Trigger room detection
                                if let Some(ref game_data) = self.game_data {
                                    state.detect_and_update_rooms(game_data);
                                }
                            } else if !can_build {
                                eprintln!("Not enough gold! Need {} gold to build {}", cost, room_type);
                            }
                        }
                        InteractionMode::PlaceSpawner => {
                            // Place monster spawner on claimed tile - COSTS 50 GOLD
                            const SPAWNER_COST: i32 = 50;

                            // Check if player can afford and tile is valid
                            let can_afford = state.player.gold >= SPAWNER_COST;
                            let is_valid_tile = if let Some(tile) = state.get_tile(hovered_tile) {
                                tile.ownership == Ownership::Player && tile.tile_type == "claimed_floor"
                            } else {
                                false
                            };

                            if can_afford && is_valid_tile {
                                // Deduct cost first
                                state.player.gold -= SPAWNER_COST;

                                // Place spawner
                                if let Some(tile) = state.get_tile_mut(hovered_tile) {
                                    tile.tile_type = "monster_spawner".to_string();
                                }
                                eprintln!("Placed monster spawner at {:?} for {} gold", hovered_tile, SPAWNER_COST);
                            } else if !can_afford {
                                eprintln!("Not enough gold! Need {} gold to place spawner", SPAWNER_COST);
                            }
                        }
                        InteractionMode::None => {
                            // Select entity or open library shop
                            let mut entity_found = false;
                            
                            // Check for entities at the hovered position
                            if let Some(entity) = state.entities.at_position(hovered_tile).next() {
                                self.selected_entity = Some(entity.id);
                                eprintln!("Selected entity: {}", entity.id);
                                entity_found = true;
                            } else {
                                self.selected_entity = None;
                            }

                            // If no entity selected, check if we clicked a library
                            if !entity_found {
                                if let Some(tile) = state.get_tile(hovered_tile) {
                                    if tile.tile_type == "library" {
                                        self.spell_shop_open = !self.spell_shop_open;
                                        eprintln!("Toggled spell shop: {}", self.spell_shop_open);
                                    } else {
                                        self.spell_shop_open = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw(&self) {
        clear_background(ui::core::colors::BACKGROUND);

        match &self.phase {
            GamePhase::Loading => {
                draw_text(
                    "Loading Deep Dominion...",
                    screen_width() / 2.0 - 150.0,
                    screen_height() / 2.0,
                    32.0,
                    WHITE,
                );
            }
            GamePhase::MainMenu => {
                let mouse_pos = mouse_position();

                // Draw title
                draw_text(
                    "Deep Dominion",
                    screen_width() / 2.0 - 150.0,
                    screen_height() / 2.0 - 180.0,
                    48.0,
                    WHITE,
                );

                // Draw subtitle
                draw_text(
                    "Select Map Type:",
                    screen_width() / 2.0 - 80.0,
                    screen_height() / 2.0 - 90.0,
                    20.0,
                    Color::new(0.9, 0.9, 0.9, 1.0),
                );

                // Draw map type selection buttons
                let map_button_y = screen_height() / 2.0 - 50.0;
                let map_button_width = 160.0;
                let map_button_height = 40.0;
                let map_button_spacing = 10.0;
                let total_width = (map_button_width + map_button_spacing) * 4.0 - map_button_spacing;
                let map_button_start_x = screen_width() / 2.0 - total_width / 2.0;

                let map_types = [
                    (MapType::Standard, "Standard", "Balanced", 0),
                    (MapType::Rich, "Rich", "Lots of gold", 1),
                    (MapType::Hazardous, "Hazardous", "Many dangers", 2),
                    (MapType::Test, "Test", "Fixed seed", 3),
                ];

                for (map_type, label, desc, index) in &map_types {
                    let btn_x = map_button_start_x + (map_button_width + map_button_spacing) * (*index as f32);

                    let is_hovered = mouse_pos.0 >= btn_x
                        && mouse_pos.0 <= btn_x + map_button_width
                        && mouse_pos.1 >= map_button_y
                        && mouse_pos.1 <= map_button_y + map_button_height;

                    let is_selected = self.selected_map_type == *map_type;

                    let button_color = if is_selected {
                        Color::new(0.2, 0.7, 0.3, 1.0) // Green for selected
                    } else if is_hovered {
                        Color::new(0.4, 0.6, 0.9, 1.0) // Blue for hover
                    } else {
                        Color::new(0.3, 0.3, 0.4, 1.0) // Gray default
                    };

                    draw_rectangle(btn_x, map_button_y, map_button_width, map_button_height, button_color);
                    draw_rectangle_lines(btn_x, map_button_y, map_button_width, map_button_height, 2.0, WHITE);

                    draw_text(
                        label,
                        btn_x + 10.0,
                        map_button_y + 20.0,
                        18.0,
                        WHITE,
                    );

                    draw_text(
                        desc,
                        btn_x + 10.0,
                        map_button_y + 35.0,
                        12.0,
                        Color::new(0.8, 0.8, 0.8, 1.0),
                    );
                }

                // Draw start button
                let button_x = screen_width() / 2.0 - 100.0;
                let button_y = screen_height() / 2.0 + 60.0;
                let button_width = 200.0;
                let button_height = 50.0;

                let is_hovered = mouse_pos.0 >= button_x
                    && mouse_pos.0 <= button_x + button_width
                    && mouse_pos.1 >= button_y
                    && mouse_pos.1 <= button_y + button_height;

                let button_color = if is_hovered {
                    Color::new(0.4, 0.6, 0.9, 1.0)
                } else {
                    Color::new(0.3, 0.5, 0.8, 1.0)
                };

                draw_rectangle(button_x, button_y, button_width, button_height, button_color);
                draw_rectangle_lines(button_x, button_y, button_width, button_height, 3.0, WHITE);

                draw_text(
                    "START GAME",
                    button_x + 35.0,
                    button_y + 32.0,
                    24.0,
                    WHITE,
                );

                // Draw hint
                draw_text(
                    "Click to start or press SPACE",
                    screen_width() / 2.0 - 120.0,
                    screen_height() / 2.0 + 140.0,
                    16.0,
                    Color::new(0.7, 0.7, 0.7, 1.0),
                );
            }
            GamePhase::Playing(state) => {
                self.draw_game(state);
            }
        }
    }

    fn draw_game(&self, state: &GameState) {
        use engine::tile_grid;
        use state::tile_state::FogState;

        let graphics = if let Some(ref cache) = self.graphics_cache {
            cache
        } else {
            return; // Can't render without graphics
        };

        // Draw grid
        for row in &state.grid {
            for tile in row {
                let (iso_x, iso_y) = tile_grid::world_to_iso(
                    tile.pos.x as f32,
                    tile.pos.y as f32,
                    ui::core::TILE_WIDTH,
                    ui::core::TILE_HEIGHT,
                );

                let screen_x = iso_x + state.camera_x;
                let screen_y = iso_y + state.camera_y;

                // Skip tiles outside screen
                if screen_x < -100.0 || screen_x > screen_width() + 100.0
                    || screen_y < -100.0 || screen_y > screen_height() + 100.0
                {
                    continue;
                }

                // Get texture for this tile type
                if let Some(texture) = graphics.tile_textures.get(&tile.tile_type) {
                    let mut color = WHITE;

                    // Apply fog of war
                    match tile.fog_state {
                        FogState::Hidden => color = ui::core::colors::FOG_HIDDEN,
                        FogState::Revealed => color = ui::core::colors::FOG_REVEALED,
                        FogState::Visible => color = WHITE, // Full color for visible
                    }

                    // Show marked tiles with yellow tint
                    if tile.marked_for_dig {
                        color = Color::new(1.0, 1.0, 0.3, 1.0); // Yellow tint for marked tiles
                    }



                    // Draw the tile texture
                    draw_texture_ex(
                        texture,
                        screen_x - ui::core::TILE_WIDTH / 2.0,
                        screen_y - ui::core::TILE_HEIGHT / 2.0,
                        color,
                        DrawTextureParams {
                            dest_size: Some(vec2(ui::core::TILE_WIDTH, ui::core::TILE_HEIGHT)),
                            ..Default::default()
                        },
                    );
                } else {
                    // Fallback to colored diamond if texture not found
                    let mut color = ui::core::get_tile_color(&tile.tile_type);

                    match tile.fog_state {
                        FogState::Hidden => color = ui::core::colors::FOG_HIDDEN,
                        FogState::Revealed => {
                            color.r *= 0.5;
                            color.g *= 0.5;
                            color.b *= 0.5;
                        }
                        FogState::Visible => {}
                    }

                    if tile.marked_for_dig {
                        color = Color::new(1.0, 0.8, 0.0, 1.0);
                    }



                    ui::core::draw_iso_tile(
                        screen_x,
                        screen_y,
                        ui::core::TILE_WIDTH,
                        ui::core::TILE_HEIGHT,
                        color,
                    );
                }

                // Draw selection outline if this is the hovered tile AND it's a valid target
                if let Some(hovered_pos) = self.hovered_tile {
                    if tile.pos == hovered_pos {
                        let mut outline_color = None;

                        match &self.interaction_mode {
                            InteractionMode::None => {
                                // Generic selection
                                outline_color = Some(Color::new(1.0, 1.0, 1.0, 0.5));
                            }
                            InteractionMode::Dig => {
                                // Valid dig targets: earth, gold, gems
                                // Also allow marked tiles to be selected (to unmark)
                                if tile.tile_type == "earth" || tile.tile_type == "gold_vein" || tile.tile_type == "gem_seam" {
                                    outline_color = Some(ui::core::colors::ACCENT);
                                }
                            }
                            InteractionMode::BuildRoom(_) => {
                                // Valid build targets: claimed floors owned by player
                                if tile.ownership == Ownership::Player && tile.tile_type == "claimed_floor" {
                                    outline_color = Some(ui::core::colors::POSITIVE);
                                }
                            }
                            InteractionMode::PlaceSpawner => {
                                // Valid spawner location: claimed floors
                                if tile.ownership == Ownership::Player && tile.tile_type == "claimed_floor" {
                                    outline_color = Some(ui::core::colors::POSITIVE);
                                }
                            }
                        }

                        if let Some(color) = outline_color {
                            ui::core::draw_iso_tile_outline(
                                screen_x,
                                screen_y,
                                ui::core::TILE_WIDTH,
                                ui::core::TILE_HEIGHT,
                                color,
                                2.0, // Thickness
                            );
                        }
                    }
                }
            }
        }

        // Draw entities (heroes and creatures)
        for entity in state.entities.all() {
            let (iso_x, iso_y) = tile_grid::world_to_iso(
                entity.pos.x as f32,
                entity.pos.y as f32,
                ui::core::TILE_WIDTH,
                ui::core::TILE_HEIGHT,
            );

            let screen_x = iso_x + state.camera_x;
            let screen_y = iso_y + state.camera_y;

            // Skip entities outside screen
            if screen_x < -50.0 || screen_x > screen_width() + 50.0
                || screen_y < -50.0 || screen_y > screen_height() + 50.0
            {
                continue;
            }

            // Get the appropriate sprite texture
            let texture_opt = match &entity.entity_type {
                state::entities::EntityType::Hero(hero_state) => {
                    graphics.hero_textures.get(&hero_state.hero_id)
                }
                state::entities::EntityType::Creature(creature_state) => {
                    graphics.monster_textures.get(&creature_state.creature_id)
                }
            };

            if let Some(texture) = texture_opt {
                // Draw sprite centered on tile position
                let sprite_size = 48.0; // Size to draw the sprite
                draw_texture_ex(
                    texture,
                    screen_x - sprite_size / 2.0,
                    screen_y - sprite_size / 2.0 - 10.0, // Offset up slightly
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(sprite_size, sprite_size)),
                        ..Default::default()
                    },
                );

                // Draw a small shadow/base
                draw_circle(screen_x, screen_y - 2.0, 6.0, Color::new(0.0, 0.0, 0.0, 0.3));
            } else {
                // Fallback to colored circles if texture not found
                let (color, size) = match &entity.entity_type {
                    state::entities::EntityType::Hero(_) => (Color::new(0.2, 0.8, 0.2, 1.0), 8.0),
                    state::entities::EntityType::Creature(_) => (Color::new(0.8, 0.2, 0.2, 1.0), 8.0),
                };

                draw_circle(screen_x, screen_y - 10.0, size, color);
                draw_circle(screen_x, screen_y - 2.0, size * 0.7, Color::new(0.0, 0.0, 0.0, 0.3));
            }
        }

        // Draw HUD
        draw_rectangle(0.0, 0.0, screen_width(), ui::core::HUD_HEIGHT, ui::core::colors::PANEL);

        let mode_text = match &self.interaction_mode {
            InteractionMode::None => "Mode: None (Press 1-5 to select)".to_string(),
            InteractionMode::Dig => "Mode: Dig (FREE - Click to mark tiles)".to_string(),
            InteractionMode::BuildRoom(room_type) => {
                let cost = match room_type.as_str() {
                    "lair" => 10,
                    "hatchery" => 15,
                    "treasury" => 20,
                    "training_room" => 25,
                    "library" => 30,
                    _ => 10,
                };
                format!("Mode: Build {} (Cost: {}g per tile)", room_type, cost)
            }
            InteractionMode::PlaceSpawner => "Mode: Place Spawner (Cost: 50g)".to_string(),
        };

        draw_text(
            &format!("Gold: {}/{} | Mana: {}/{} | Food: {}/{} | Imps: {}/{} | Monsters: {}/{}",
                state.player.gold, state.player.max_gold,
                state.player.mana, state.player.max_mana,
                state.player.food, state.player.max_food,
                state.count_imps(), GameState::MAX_IMPS,
                state.count_monsters(), state.player.max_creatures),
            10.0,
            25.0,
            18.0,
            ui::core::colors::TEXT,
        );
        // Draw mode text - use ACCENT for mode, NEGATIVE when can't afford
        let mode_color = match &self.interaction_mode {
            InteractionMode::None => ui::core::colors::TEXT_DIM,
            InteractionMode::Dig => ui::core::colors::ACCENT,
            InteractionMode::BuildRoom(room_type) => {
                let cost = match room_type.as_str() {
                    "lair" => 10,
                    "hatchery" => 15,
                    "treasury" => 20,
                    "training_room" => 25,
                    "library" => 30,
                    _ => 10,
                };
                if state.player.gold >= cost {
                    ui::core::colors::ACCENT
                } else {
                    ui::core::colors::NEGATIVE
                }
            }
            InteractionMode::PlaceSpawner => {
                if state.player.gold >= 50 {
                    ui::core::colors::ACCENT
                } else {
                    ui::core::colors::NEGATIVE
                }
            }
        };
        draw_text(
            &mode_text,
            10.0,
            45.0,
            16.0,
            mode_color,
        );
        
        // Show selected spell indicator using get_selected_spell
        if let Some(selected_spell) = self.spell_bar.get_selected_spell() {
            draw_text(
                &format!("Casting: {} (Click target)", selected_spell),
                300.0,
                45.0,
                16.0,
                ui::core::colors::ACCENT,
            );
        }
        draw_text(
            "1: Dig (Free) | 2: Lair (10g) | 3: Hatchery (15g) | 4: Treasury (20g) | 5: Spawner (50g)",
            10.0,
            screen_height() - 25.0,
            14.0,
            ui::core::colors::TEXT_DIM,
        );
        draw_text(
            "WASD: Camera | ESC: Cancel Mode | Right-click: Cancel Dig",
            10.0,
            screen_height() - 10.0,
            14.0,
            ui::core::colors::TEXT_DIM,
        );

        // Draw resource warnings using WARNING, NEGATIVE, and POSITIVE colors
        // Position relative to SIDEBAR_WIDTH
        let status_x = screen_width() - ui::core::SIDEBAR_WIDTH + 150.0;
        let mut warning_y = screen_height() - 40.0;
        
        // Show positive status when resources are good
        if state.player.gold >= 100 {
            draw_text(
                "Food OK",
                status_x,
                warning_y,
                14.0,
                ui::core::colors::POSITIVE,
            );
        } else if state.player.food <= 0 {
            draw_text(
                "No Food!",
                status_x,
                warning_y,
                14.0,
                ui::core::colors::NEGATIVE,
            );
        } else if state.player.food < state.player.max_food / 4 {
            draw_text(
                "Low Food!",
                status_x,
                warning_y,
                14.0,
                ui::core::colors::WARNING,
            );
        }

        // Draw build menu
        self.build_menu.draw(&self.interaction_mode, &state.player);

        // Draw spell bar
        if let Some(ref game_data) = self.game_data {
            self.spell_bar.draw(&state.player, &game_data.spells);
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Deep Dominion".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    let mut loading_started = false;

    loop {
        // Handle loading phase
        if matches!(game.phase, GamePhase::Loading) && !loading_started {
            loading_started = true;
            game.load_resources().await;
        }

        let dt = get_frame_time();
        game.update(dt);
        game.draw();

        next_frame().await;
    }
}
