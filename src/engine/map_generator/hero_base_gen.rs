use crate::engine::map_generator::config::{MapConfig, Grid};
use crate::state::tile_state::{TilePos, TileState, Ownership};
use rand::Rng;

pub fn place_hero_base(
    grid: &mut Grid, 
    start_pos: TilePos, 
    config: &MapConfig, 
    rng: &mut impl Rng
) {
    if !config.hero_base_enabled {
        return;
    }

    let (width, height) = (grid[0].len(), grid.len());

    // 1. Determine Base Location (Opposite to Start)
    let base_center = find_opposite_corner(start_pos, width, height);

    eprintln!("Placing Hero Base at {:?}", base_center);

    // 2. Clear Area (Flat ground)
    let base_radius = 6;
    for dy in -base_radius..=base_radius {
        for dx in -base_radius..=base_radius {
            let x = base_center.x + dx;
            let y = base_center.y + dy;

            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                if let Some(tile) = get_tile_mut(grid, x as usize, y as usize) {
                    // Make it claimed floor (enemy territory)
                    tile.tile_type = "claimed_floor".to_string(); 
                    tile.ownership = Ownership::Enemy; // Enemy floor 
                    // Then make it specific floor type if we want, or just "earth" for now
                    // Ideally "hero_ground" or similar, but "earth" works.
                    // We'll set the building tiles specifically.
                }
            }
        }
    }

    // 3. Place Buildings
    // Layout:
    // [W][W][W][G][W][W][W]
    // [W]                 [W]
    // [W]  [B] [T] [C]    [W]
    // [W]    [Square]     [W]
    // [W]  [A] [M] [S]    [W]
    // [W]                 [W]
    // [W][W][W][W][W][W][W]

    let buildings = vec![
        ("town_hall", 0, 0),         // Center
        ("barracks", -4, -2),        // Left Top
        ("church", 4, -2),           // Right Top
        ("mage_tower", 0, -4),       // Top Center (behind town hall relative to entrance?)
                                     // Let's orient based on where the dungeon is.
                                     // For simplicity, layout is fixed relative to map bounds?
                                     // Or assume standardized 7x7 layout.
        ("archery_range", -4, 2),    // Left Bottom
        ("stable", 4, 2),            // Right Bottom
        ("armory", 0, 4),            // Bottom Center
    ];

    for (id, dx, dy) in buildings {
        let bx = base_center.x + dx;
        let by = base_center.y + dy;
        place_building_tile(grid, bx, by, id, width, height);
    }
    
    // 4. Place Walls & Gates
    // Simple box around the radius 6
    let wall_radius = 6;
    for d in -wall_radius..=wall_radius {
        // Top & Bottom
        place_building_tile(grid, base_center.x + d, base_center.y - wall_radius, "hero_wall", width, height);
        place_building_tile(grid, base_center.x + d, base_center.y + wall_radius, "hero_wall", width, height);
        // Left & Right
        place_building_tile(grid, base_center.x - wall_radius, base_center.y + d, "hero_wall", width, height);
        place_building_tile(grid, base_center.x + wall_radius, base_center.y + d, "hero_wall", width, height);
    }

    // 5. Carve Gate and Path
    // Determine direction towards dungeon center
    let dx = start_pos.x - base_center.x;
    let dy = start_pos.y - base_center.y;

    let (gate_x, gate_y) = if dx.abs() > dy.abs() {
        // Horizontal connection
        let sign = dx.signum();
        (base_center.x + sign * wall_radius, base_center.y)
    } else {
        // Vertical connection
        let sign = dy.signum();
        (base_center.x, base_center.y + sign * wall_radius)
    };

    // Place Gate
    place_building_tile(grid, gate_x, gate_y, "hero_gate", width, height);

    // Carve Path towards start (simple Bresenham or just straight line for a bit)
    // Connectivity step usually handles main path, but we need to ensure the gate isn't blocked.
    // Let's carve a small clearing outside the gate.
    let gate_dir_x = (gate_x - base_center.x).signum();
    let gate_dir_y = (gate_y - base_center.y).signum();

    for i in 1..5 {
        let px = gate_x + gate_dir_x * i;
        let py = gate_y + gate_dir_y * i;
        if let Some(tile) = get_tile_mut(grid, px as usize, py as usize) {
            tile.tile_type = "earth".to_string(); // Diggable path
        }
    }
}

fn find_opposite_corner(start_pos: TilePos, width: usize, height: usize) -> TilePos {
    // If start is Top-Left (low x, low y), Base is Bottom-Right
    // If start is Center, Base is random corner.
    
    let w = width as i32;
    let h = height as i32;
    let margin = 10;

    if start_pos.x < w / 2 {
        // Start is Left
        if start_pos.y < h / 2 {
            // Start is Top-Left -> Base Bottom-Right
            TilePos::new(w - margin, h - margin)
        } else {
            // Start is Bottom-Left -> Base Top-Right
            TilePos::new(w - margin, margin)
        }
    } else {
        // Start is Right
        if start_pos.y < h / 2 {
            // Start is Top-Right -> Base Bottom-Left
            TilePos::new(margin, h - margin)
        } else {
            // Start is Bottom-Right -> Base Top-Left
            TilePos::new(margin, margin)
        }
    }
}

fn place_building_tile(grid: &mut Grid, x: i32, y: i32, id: &str, width: usize, height: usize) {
    if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
        if let Some(tile) = get_tile_mut(grid, x as usize, y as usize) {
            tile.tile_type = id.to_string();
            tile.ownership = Ownership::Enemy; // Mark as enemy owned? Or Neutral?
            // "Enemy" ownership might prevent player from digging?
            // Let's use ownership to indicate it's not player territory.
        }
    }
}

fn get_tile_mut(grid: &mut Grid, x: usize, y: usize) -> Option<&mut TileState> {
    if y < grid.len() && x < grid[y].len() {
        Some(&mut grid[y][x])
    } else {
        None
    }
}
