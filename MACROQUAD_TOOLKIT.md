# Macroquad Toolkit

A collection of common utilities for Macroquad game development, extracted from multiple games to reduce duplication and provide consistent patterns.

## Features

- **Input utilities**: Mouse hovering, clicking, rectangle collision detection
- **UI rendering**: Buttons, colored buttons, windows/modals, panels, progress bars
- **Asset management**: Texture loading and caching
- **Audio management**: Sound effect and music handling with volume control
- **Persistence**: Easy JSON save/load system
- **State management**: Game state trait and transition system
- **Camera2D**: Pan and zoom for 2D games
- **Event bus**: Generic event system for decoupled game logic
- **Color palettes**: Consistent dark theme colors
- **Sprite system**: Builder pattern for texture rendering with transformations

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
macroquad-toolkit = { path = "../macroquad-toolkit" }
```

### Quick Start

```rust
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    let mut assets = AssetManager::new();
    assets.load_texture("player", "assets/player.png").await.ok();

    loop {
        clear_background(dark::BACKGROUND);

        // Draw a button
        if button(10.0, 10.0, 100.0, 40.0, "Click Me") {
            println!("Button clicked!");
        }

        next_frame().await;
    }
}
```

## Modules

### Input (`input` module)

```rust
use macroquad_toolkit::input::*;

// Check if mouse is over a rectangle
if is_hovered(x, y, w, h) {
    // ...
}

// Check if rectangle was clicked (released)
if was_clicked(x, y, w, h) {
    // ...
}

// Check if rectangle was pressed (down)
if was_pressed(x, y, w, h) {
    // ...
}

// Capture input state
let input = InputState::capture();
if input.left_click {
    // ...
}
```

### UI (`ui` module)

```rust
use macroquad_toolkit::ui::*;

// Simple button (triggers on release)
if button(x, y, w, h, "Click") {
    // Button was clicked
}

// Button with custom style
let style = ButtonStyle::default_dark();
if button_styled(x, y, w, h, "Custom", &style) {
    // ...
}

// Button that triggers on press (instead of release)
if button_on_press(x, y, w, h, "Press", &style) {
    // Triggers immediately when mouse down
}

// Panel with title
panel(x, y, w, h, Some("Title"));

// Window (Modal-like) with close button
if window(x, y, w, h, Some("Title"), true) {
    // Window close button clicked
}

// Progress bar
progress_bar(x, y, w, h, current, max, dark::POSITIVE);

// Colored Button
if colored_button(x, y, w, h, "Action", RED) {
   // ...
}
```

### Assets (`assets` module)

```rust
use macroquad_toolkit::assets::AssetManager;

let mut assets = AssetManager::new();

// Load single texture
assets.load_texture("player", "assets/player.png").await.ok();

// Get texture
if let Some(tex) = assets.get_texture("player") {
    draw_texture(tex, x, y, WHITE);
}

// Data-driven loading (from JSON config)
// JSON: [{"key": "hero", "path": "assets/hero.png"}]
use macroquad_toolkit::assets::TextureConfig;
if let Ok(configs) = TextureConfig::load_from_file("assets/textures.json").await {
    for config in configs {
        assets.load_texture(&config.key, &config.path).await.ok();
    }
}
```

### Camera (`camera` module)

```rust
use macroquad_toolkit::camera::Camera2D;

let mut camera = Camera2D::new(vec2(0.0, 0.0), 1.0);

// In game loop
camera.update(get_frame_time(), false);

// Convert coordinates
let world_pos = camera.screen_to_world(mouse_position().into());
let screen_pos = camera.world_to_screen(world_pos);
```

### Events (`events` module)

```rust
use macroquad_toolkit::events::EventBus;

enum GameEvent {
    PlayerDied,
    EnemySpawned,
}

let mut events = EventBus::new();
events.push(GameEvent::PlayerDied);

// Process events
for event in events.drain() {
    match event {
        GameEvent::PlayerDied => { /* ... */ }
        GameEvent::EnemySpawned => { /* ... */ }
    }
}
```

### Colors (`colors` module)

```rust
use macroquad_toolkit::colors::dark;

clear_background(dark::BACKGROUND);
draw_rectangle(x, y, w, h, dark::PANEL);
draw_text("Hello", x, y, 20.0, dark::TEXT);
```

Available colors:
- `BACKGROUND`, `PANEL`, `PANEL_HEADER`
- `TEXT`, `TEXT_BRIGHT`, `TEXT_DIM`
- `ACCENT`, `POSITIVE`, `WARNING`, `NEGATIVE`
- `HOVERED`

Rarity colors (for RPG items):
```rust
use macroquad_toolkit::colors::rarity;

draw_rectangle(x, y, w, h, rarity::COMMON);     // Gray
draw_rectangle(x, y, w, h, rarity::UNCOMMON);   // Green
draw_rectangle(x, y, w, h, rarity::RARE);       // Blue
draw_rectangle(x, y, w, h, rarity::EPIC);       // Purple
draw_rectangle(x, y, w, h, rarity::LEGENDARY);  // Orange
```

### Additional UI Components

```rust
use macroquad_toolkit::ui::*;

// Section panel with title header
section_panel(x, y, w, h, "Section Title");

// Clickable card (returns true if clicked)
if card(x, y, w, h, is_selected) {
    // Card was clicked
}

// Full-screen overlay for modals
full_screen_overlay(0.7); // 70% opacity

// String helpers
let title = capitalize("warrior");           // "Warrior"
let name = display_name("health_potion");    // "Health Potion"

// Grid Layout Helper
let grid = GridLayout::new(x, y, width, padding, cols, card_height);
let (bx, by, bw, bh) = grid.get_item_rect(index, scroll_y);

// Scroll Helper
// Returns new scroll value clamped to content bounds
let new_scroll = handle_scroll(current_scroll, content_height, view_height, 30.0);

```

### RNG (`rng` module)

```rust
use macroquad_toolkit::rng::*;

// Seeded random number generator
let mut rng = GameRng::new(12345);
let value = rng.gen_range(1..100);
let choice = rng.choose(&["a", "b", "c"]);
```

### Sprite (`sprite` module)

```rust
use macroquad_toolkit::sprite::Sprite;

let sprite = Sprite::new()
    .with_texture(texture)
    .at(100.0, 100.0)
    .scaled(2.0, 2.0)
    .rotated(0.5)
    .colored(RED);

sprite.draw();
```

### Persistence (`persistence` module)

```rust
use macroquad_toolkit::persistence::*; // save_json, load_json, get_app_data_path

#[derive(Serialize, Deserialize)]
struct SaveData {
    score: u32,
    level: u32,
}

// Save to standard app data location
if let Some(path) = get_app_data_path("my_game", "save.json") {
    let data = SaveData { score: 100, level: 1 };
    save_json(&path, &data).ok();
}
```

### Audio (`audio` module)

```rust
use macroquad_toolkit::audio::{SoundManager, SoundId};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Sfx {
    Jump,
    Shoot,
}

let mut audio = SoundManager::new();
audio.load_sound(Sfx::Jump, "assets/jump.wav").await.ok();

audio.play_sfx(Sfx::Jump, 1.0); // Plays with 1.0 * global_sfx_volume
```

### States (`states` module)

```rust
use macroquad_toolkit::states::{GameState, Transition};
use std::any::Any;

struct MyGame;

struct MenuState;
impl GameState<MyGame> for MenuState {
    fn update(&mut self, _ctx: &mut MyGame) -> Option<Box<dyn Any>> {
        None
    }
    fn draw(&self, _ctx: &MyGame) {
        // Draw menu
    }
}
```

## Button Click Semantics

The toolkit provides two button variants to handle different click behaviors:

- **`button()` and `button_on_release()`**: Fire when mouse button is **released** over the button. This is the safer default as it prevents accidental double-clicks and allows users to move the mouse away to cancel.

- **`button_on_press()`**: Fires when mouse button is **pressed down** over the button. Use this for instant feedback scenarios.

## Database (`db` module)

Utilities for SQLite using `sqlx`. Enabled via the `db` feature.

**Cargo.toml**:
```toml
macroquad-toolkit = { version = "...", features = ["db"] }
```

**Usage**:
```rust
use macroquad_toolkit::db::Database;

// Initialize (auto-creates DB file if missing)
let db = Database::new("sqlite://game.db").await?;

// Run raw migrations/queries
db.run_raw_migrations(&[
    "CREATE TABLE IF NOT EXISTS players (id TEXT PRIMARY KEY)",
    "INSERT INTO players (id) VALUES ('p1')",
]).await?;

// Access the underlying sqlx::SqlitePool
let row: (i64,) = sqlx::query_as("SELECT count(*) FROM players")
    .fetch_one(&db.pool)
    .await?;
```

## License


This toolkit is extracted from game projects and shared for reuse across multiple games.
