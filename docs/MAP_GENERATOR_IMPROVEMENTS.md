# Map Generator Improvements

## Current State Analysis

The existing map generator (324 lines) is functional but basic:
- **Strengths**: Clean API, configurable parameters, deterministic seeds
- **Weaknesses**: Uniform terrain, random resource placement, no strategic depth, limited variety

## Critical Issues

### 1. No Natural Terrain Variation
- Everything is flat earth with rock borders
- No caves, tunnels, caverns, or geological features
- No terrain height or topology

### 2. Poor Resource Distribution
- Resources randomly scattered via `rng.gen_range(-3..=3)`
- No strategic placement (risk/reward positioning)
- No mineral veins following realistic geological patterns

### 3. Predictable Starting Area
- Always centered at `(width/2, height/2)`
- Always same room layout (heart + 3 rooms in cross pattern)
- No variation in starting conditions

### 4. Trivial Hazards
- Water/lava are just random circular blobs
- No rivers, lakes, or lava flows
- No interaction with terrain topology

### 5. No Gameplay Considerations
- No hero entrance points
- No choke points or defensive positions
- No validation that resources are reachable
- No difficulty scaling

---

## Proposed Improvements

### Phase 1: Foundation Systems (High Priority)

#### 1.1 Noise-Based Terrain Generation
**Problem**: Flat, uniform terrain with no variation
**Solution**: Use Perlin/Simplex noise for organic terrain features

```rust
use noise::{NoiseFn, Perlin, Seedable};

struct TerrainGenerator {
    density_noise: Perlin,
    cave_noise: Perlin,
    biome_noise: Perlin,
}

impl TerrainGenerator {
    fn new(seed: u32) -> Self {
        Self {
            density_noise: Perlin::new().set_seed(seed),
            cave_noise: Perlin::new().set_seed(seed + 1),
            biome_noise: Perlin::new().set_seed(seed + 2),
        }
    }

    fn sample_density(&self, x: f64, y: f64) -> f64 {
        // Multi-octave sampling for natural variation
        let scale1 = 0.05;
        let scale2 = 0.1;
        let scale3 = 0.2;

        self.density_noise.get([x * scale1, y * scale1]) * 0.5 +
        self.density_noise.get([x * scale2, y * scale2]) * 0.3 +
        self.density_noise.get([x * scale3, y * scale3]) * 0.2
    }

    fn is_solid(&self, x: i32, y: i32) -> bool {
        let density = self.sample_density(x as f64, y as f64);
        density > 0.1 // Threshold determines cave size
    }
}
```

**Benefits**:
- Natural-looking cave systems
- Configurable cave density
- Deterministic from seed

#### 1.2 Cellular Automata Cave Refinement
**Problem**: Noise can create disconnected regions
**Solution**: Cellular automata smoothing pass

```rust
fn smooth_caves_cellular_automata(grid: &mut Grid, iterations: usize) {
    for _ in 0..iterations {
        let mut next_grid = grid.clone();

        for y in 1..grid.len()-1 {
            for x in 1..grid[0].len()-1 {
                let solid_neighbors = count_solid_neighbors(grid, x, y);

                // Birth rule: become solid if 5+ solid neighbors
                // Death rule: become open if <4 solid neighbors
                if solid_neighbors >= 5 {
                    next_grid[y][x].tile_type = "solid_rock".to_string();
                } else if solid_neighbors < 4 {
                    next_grid[y][x].tile_type = "earth".to_string();
                }
            }
        }

        *grid = next_grid;
    }
}

fn count_solid_neighbors(grid: &Grid, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if grid[ny][nx].tile_type == "solid_rock" {
                count += 1;
            }
        }
    }
    count
}
```

**Benefits**:
- Removes small disconnected rooms
- Creates more natural cave shapes
- Widens narrow passages

#### 1.3 Flood Fill Connectivity Analysis
**Problem**: No guarantee that map is playable
**Solution**: Ensure all regions are connected

```rust
fn ensure_connectivity(grid: &mut Grid) {
    let regions = find_disconnected_regions(grid);

    if regions.len() > 1 {
        // Connect largest regions with tunnels
        for i in 1..regions.len() {
            connect_regions(grid, &regions[0], &regions[i]);
        }
    }
}

fn find_disconnected_regions(grid: &Grid) -> Vec<Vec<TilePos>> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut regions = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !visited[y][x] && grid[y][x].tile_type != "solid_rock" {
                let region = flood_fill_region(grid, x, y, &mut visited);
                regions.push(region);
            }
        }
    }

    // Sort by size, largest first
    regions.sort_by_key(|r| std::cmp::Reverse(r.len()));
    regions
}

fn connect_regions(grid: &mut Grid, region_a: &[TilePos], region_b: &[TilePos]) {
    // Find closest points between regions
    let (start, end) = find_closest_points(region_a, region_b);

    // Carve straight tunnel (or use A* for natural path)
    carve_tunnel(grid, start, end);
}
```

**Benefits**:
- Guarantees map is playable
- No unreachable resource pockets
- Removes dead-end isolated areas

---

### Phase 2: Strategic Resource Placement

#### 2.1 Mineral Vein Systems
**Problem**: Resources are randomly scattered
**Solution**: Realistic vein generation using drunk walk

```rust
fn generate_mineral_vein(
    grid: &mut Grid,
    start: TilePos,
    tile_type: &str,
    length: usize,
    thickness: usize,
    rng: &mut impl Rng,
) {
    let mut current = start;
    let mut direction = (rng.gen_range(-1..=1), rng.gen_range(-1..=1));

    for _ in 0..length {
        // Place vein segment with thickness
        place_vein_segment(grid, current, tile_type, thickness);

        // Random walk with directional bias
        if rng.gen::<f32>() < 0.7 {
            // Continue in same direction (creates longer veins)
            current.x += direction.0;
            current.y += direction.1;
        } else {
            // Change direction
            direction = (rng.gen_range(-1..=1), rng.gen_range(-1..=1));
            current.x += direction.0;
            current.y += direction.1;
        }

        // Clamp to bounds
        current = clamp_position(current, grid);
    }
}

fn place_vein_segment(grid: &mut Grid, center: TilePos, tile_type: &str, thickness: usize) {
    for dy in -(thickness as i32)..=(thickness as i32) {
        for dx in -(thickness as i32)..=(thickness as i32) {
            if dx*dx + dy*dy <= (thickness*thickness) as i32 {
                let x = (center.x + dx).max(0).min(grid[0].len() as i32 - 1) as usize;
                let y = (center.y + dy).max(0).min(grid.len() as i32 - 1) as usize;

                if grid[y][x].tile_type == "solid_rock" {
                    grid[y][x].tile_type = tile_type.to_string();
                    set_resource_amount(grid, x, y, tile_type);
                }
            }
        }
    }
}
```

#### 2.2 Risk/Reward Resource Placement
**Problem**: No strategic depth in resource locations
**Solution**: Place valuable resources in dangerous areas

```rust
struct ResourcePlacementStrategy {
    min_distance_from_start: f32,
    prefer_near_hazards: bool,
    prefer_deep_areas: bool,
    require_digging: bool,
}

fn place_strategic_resources(
    grid: &mut Grid,
    start_pos: TilePos,
    config: &MapConfig,
    rng: &mut impl Rng,
) {
    // High-value resources (gems) far from start
    let gem_strategy = ResourcePlacementStrategy {
        min_distance_from_start: 20.0,
        prefer_near_hazards: true,
        prefer_deep_areas: true,
        require_digging: true,
    };
    place_resources_with_strategy(grid, start_pos, "gem_seam", &gem_strategy, rng);

    // Medium-value resources (gold) at moderate distance
    let gold_strategy = ResourcePlacementStrategy {
        min_distance_from_start: 10.0,
        prefer_near_hazards: false,
        prefer_deep_areas: true,
        require_digging: true,
    };
    place_resources_with_strategy(grid, start_pos, "gold_vein", &gold_strategy, rng);

    // Mana crystals near enemy portals (high risk/reward)
    place_mana_near_portals(grid, rng);
}

fn calculate_tile_danger(grid: &Grid, pos: TilePos) -> f32 {
    let mut danger = 0.0;

    // Danger from nearby hazards
    for hazard_pos in find_nearby_tiles(grid, pos, 5, &["lava", "water"]) {
        let dist = distance(pos, hazard_pos);
        danger += 1.0 / (dist + 1.0);
    }

    // Danger from depth (amount of digging required)
    let depth = calculate_digging_depth(grid, pos);
    danger += depth as f32 * 0.1;

    danger
}
```

---

### Phase 3: Advanced Features

#### 3.1 Biome System
**Problem**: Entire map has uniform appearance/mechanics
**Solution**: Distinct biome regions with unique properties

```rust
#[derive(Debug, Clone, Copy)]
enum Biome {
    Standard,       // Normal earth and rock
    Volcanic,       // Lava flows, obsidian, fire hazards
    Crystalline,    // Rich in mana, glowing crystals
    Flooded,        // Underground rivers, water everywhere
    Ancient,        // Ruins, ancient rune floors, special traps
    Corrupted,      // Purple corruption, spawns stronger monsters
}

fn generate_biome_map(width: usize, height: usize, rng: &mut impl Rng) -> Vec<Vec<Biome>> {
    let num_biome_centers = rng.gen_range(3..7);
    let mut biome_centers = Vec::new();

    // Place biome centers
    for _ in 0..num_biome_centers {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let biome = random_biome(rng);
        biome_centers.push((x, y, biome));
    }

    // Voronoi diagram: assign each tile to nearest biome center
    let mut biome_map = vec![vec![Biome::Standard; width]; height];
    for y in 0..height {
        for x in 0..width {
            let mut nearest_biome = Biome::Standard;
            let mut min_dist = f32::INFINITY;

            for (cx, cy, biome) in &biome_centers {
                let dist = ((x as f32 - *cx as f32).powi(2) +
                           (y as f32 - *cy as f32).powi(2)).sqrt();
                if dist < min_dist {
                    min_dist = dist;
                    nearest_biome = *biome;
                }
            }

            biome_map[y][x] = nearest_biome;
        }
    }

    // Smooth biome boundaries
    smooth_biome_boundaries(&mut biome_map, 2);

    biome_map
}

fn apply_biome_features(grid: &mut Grid, biome_map: &[Vec<Biome>], rng: &mut impl Rng) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            match biome_map[y][x] {
                Biome::Volcanic => {
                    if rng.gen::<f32>() < 0.3 && grid[y][x].tile_type == "earth" {
                        grid[y][x].tile_type = "lava".to_string();
                    }
                }
                Biome::Crystalline => {
                    if rng.gen::<f32>() < 0.2 && grid[y][x].tile_type == "solid_rock" {
                        grid[y][x].tile_type = "mana_crystal".to_string();
                    }
                }
                Biome::Ancient => {
                    if rng.gen::<f32>() < 0.15 && grid[y][x].tile_type == "earth" {
                        grid[y][x].tile_type = "ancient_rune_floor".to_string();
                    }
                }
                // ... other biomes
                _ => {}
            }
        }
    }
}
```

#### 3.2 Hero Portal Placement
**Problem**: No enemy spawn points
**Solution**: Strategic portal placement for wave-based invasions

```rust
fn place_hero_portals(
    grid: &mut Grid,
    start_pos: TilePos,
    num_portals: usize,
    rng: &mut impl Rng,
) -> Vec<TilePos> {
    let mut portal_positions = Vec::new();

    // Portals should be:
    // 1. Far from player start (minimum distance)
    // 2. In open areas (not surrounded by solid rock)
    // 3. Have clear path to player (via pathfinding check)
    // 4. Spread out from each other

    let candidates = find_portal_candidates(grid, start_pos);

    for _ in 0..num_portals {
        if let Some(pos) = select_best_portal_location(&candidates, &portal_positions, rng) {
            // Mark tile as hero portal spawn point
            grid[pos.y as usize][pos.x as usize].hero_portal = true;
            portal_positions.push(pos);
        }
    }

    portal_positions
}

fn find_portal_candidates(grid: &Grid, start_pos: TilePos) -> Vec<TilePos> {
    let mut candidates = Vec::new();
    let min_distance = 25.0;

    for y in 5..grid.len()-5 {
        for x in 5..grid[0].len()-5 {
            let pos = TilePos::new(x as i32, y as i32);

            // Must be far from start
            if distance_f32(pos, start_pos) < min_distance {
                continue;
            }

            // Must be in open area
            if !is_open_area(grid, pos, 3) {
                continue;
            }

            // Must have path to player start
            if !has_path_to(grid, pos, start_pos) {
                continue;
            }

            candidates.push(pos);
        }
    }

    candidates
}
```

#### 3.3 Natural Feature Generation
**Problem**: Maps lack interesting landmarks
**Solution**: Add distinctive geological features

```rust
fn add_natural_features(grid: &mut Grid, rng: &mut impl Rng) {
    // Stone pillars (provide cover in combat)
    add_stone_pillars(grid, rng.gen_range(5..15), rng);

    // Underground lakes (large water bodies)
    add_underground_lakes(grid, rng.gen_range(1..3), rng);

    // Collapsed chambers (large open caverns)
    add_collapsed_chambers(grid, rng.gen_range(2..5), rng);

    // Narrow chasms (defensive choke points)
    add_chasms(grid, rng.gen_range(1..4), rng);

    // Crystal formations (glowing landmarks)
    add_crystal_formations(grid, rng.gen_range(3..8), rng);
}

fn add_stone_pillars(grid: &mut Grid, count: usize, rng: &mut impl Rng) {
    for _ in 0..count {
        let x = rng.gen_range(5..grid[0].len()-5);
        let y = rng.gen_range(5..grid.len()-5);
        let radius = rng.gen_range(2..5);

        // Only place in open areas
        if is_open_area(grid, TilePos::new(x as i32, y as i32), radius * 2) {
            create_pillar(grid, x, y, radius);
        }
    }
}

fn create_pillar(grid: &mut Grid, cx: usize, cy: usize, radius: usize) {
    for dy in -(radius as i32)..=(radius as i32) {
        for dx in -(radius as i32)..=(radius as i32) {
            if dx*dx + dy*dy <= (radius*radius) as i32 {
                let x = (cx as i32 + dx).max(0).min(grid[0].len() as i32 - 1) as usize;
                let y = (cy as i32 + dy).max(0).min(grid.len() as i32 - 1) as usize;

                grid[y][x].tile_type = "solid_rock".to_string();
            }
        }
    }
}

fn add_underground_lakes(grid: &mut Grid, count: usize, rng: &mut impl Rng) {
    for _ in 0..count {
        let x = rng.gen_range(10..grid[0].len()-10);
        let y = rng.gen_range(10..grid.len()-10);
        let size = rng.gen_range(8..20);

        // Use organic shape (not just circle)
        create_organic_lake(grid, x, y, size, rng);
    }
}

fn create_organic_lake(grid: &mut Grid, cx: usize, cy: usize, size: usize, rng: &mut impl Rng) {
    // Use flood fill with random threshold for organic shape
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    queue.push_back((cx, cy));
    visited[cy][cx] = true;

    let mut tiles_placed = 0;
    let max_tiles = size * size;

    while let Some((x, y)) = queue.pop_front() {
        if tiles_placed >= max_tiles {
            break;
        }

        grid[y][x].tile_type = "water".to_string();
        tiles_placed += 1;

        // Spread to neighbors with probability
        for (dx, dy) in [(-1,0), (1,0), (0,-1), (0,1)] {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;

            if nx > 0 && nx < grid[0].len()-1 && ny > 0 && ny < grid.len()-1 {
                if !visited[ny][nx] && rng.gen::<f32>() < 0.7 {
                    visited[ny][nx] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }
}
```

---

### Phase 4: Starting Area Improvements

#### 4.1 Variable Starting Positions
**Problem**: Always centered, predictable
**Solution**: Multiple starting position strategies

```rust
enum StartingPositionStrategy {
    Center,
    Corner,
    Edge,
    Random,
    NearResources,
}

fn find_starting_position(
    grid: &Grid,
    strategy: StartingPositionStrategy,
    rng: &mut impl Rng,
) -> TilePos {
    match strategy {
        StartingPositionStrategy::Center => {
            TilePos::new(grid[0].len() as i32 / 2, grid.len() as i32 / 2)
        }
        StartingPositionStrategy::Corner => {
            let corners = vec![
                TilePos::new(10, 10),
                TilePos::new(grid[0].len() as i32 - 10, 10),
                TilePos::new(10, grid.len() as i32 - 10),
                TilePos::new(grid[0].len() as i32 - 10, grid.len() as i32 - 10),
            ];
            *corners.choose(rng).unwrap()
        }
        StartingPositionStrategy::Edge => {
            // Random position along edge
            let edge = rng.gen_range(0..4);
            match edge {
                0 => TilePos::new(rng.gen_range(10..grid[0].len() as i32 - 10), 10),
                1 => TilePos::new(grid[0].len() as i32 - 10, rng.gen_range(10..grid.len() as i32 - 10)),
                2 => TilePos::new(rng.gen_range(10..grid[0].len() as i32 - 10), grid.len() as i32 - 10),
                _ => TilePos::new(10, rng.gen_range(10..grid.len() as i32 - 10)),
            }
        }
        StartingPositionStrategy::Random => {
            find_valid_random_start(grid, rng)
        }
        StartingPositionStrategy::NearResources => {
            find_resource_rich_start(grid, rng)
        }
    }
}
```

#### 4.2 Procedural Starting Rooms
**Problem**: Always same 3 rooms
**Solution**: Vary starting rooms based on difficulty/strategy

```rust
struct StartingLayout {
    rooms: Vec<(String, i32, i32, usize)>, // (room_type, offset_x, offset_y, size)
}

fn generate_starting_layout(difficulty: f32, rng: &mut impl Rng) -> StartingLayout {
    let mut rooms = vec![
        ("dungeon_heart".to_string(), 0, 0, 1), // Always have heart
    ];

    if difficulty < 0.3 {
        // Easy: More starting rooms
        rooms.push(("lair".to_string(), -6, 0, 3));
        rooms.push(("hatchery".to_string(), 6, 0, 3));
        rooms.push(("treasury".to_string(), 0, -6, 3));
        rooms.push(("training_room".to_string(), 0, 6, 2));
    } else if difficulty < 0.7 {
        // Medium: Basic rooms only
        rooms.push(("lair".to_string(), -6, 0, 2));
        rooms.push(("hatchery".to_string(), 6, 0, 2));
        rooms.push(("treasury".to_string(), 0, -6, 2));
    } else {
        // Hard: Minimal start (heart + small lair)
        rooms.push(("lair".to_string(), -4, 0, 2));
    }

    StartingLayout { rooms }
}
```

---

### Phase 5: Performance & Quality

#### 5.1 Generation Metrics
```rust
struct MapQualityMetrics {
    resource_accessibility: f32,    // % of resources reachable
    defensive_positions: usize,     // Number of choke points
    average_room_size: f32,         // Size of open areas
    hazard_balance: f32,            // Ratio of hazards to open space
    strategic_depth: f32,           // Multiple paths to resources
}

fn evaluate_map_quality(grid: &Grid, start_pos: TilePos) -> MapQualityMetrics {
    // Analyze generated map for playability
    let reachable_resources = count_reachable_resources(grid, start_pos);
    let total_resources = count_total_resources(grid);

    MapQualityMetrics {
        resource_accessibility: reachable_resources as f32 / total_resources as f32,
        defensive_positions: count_choke_points(grid),
        average_room_size: calculate_average_open_area_size(grid),
        hazard_balance: calculate_hazard_ratio(grid),
        strategic_depth: calculate_path_diversity(grid, start_pos),
    }
}

// Regenerate map if quality is poor
pub fn generate_quality_map(config: &MapConfig, game_data: &GameData) -> Grid {
    let mut attempts = 0;
    let max_attempts = 10;

    loop {
        let grid = generate_map(config, game_data);
        let start_pos = find_dungeon_heart(&grid).unwrap();
        let metrics = evaluate_map_quality(&grid, start_pos);

        // Accept map if meets quality threshold
        if metrics.resource_accessibility > 0.8
            && metrics.defensive_positions >= 3
            && metrics.hazard_balance < 0.3 {
            return grid;
        }

        attempts += 1;
        if attempts >= max_attempts {
            eprintln!("Warning: Map quality below threshold after {} attempts", max_attempts);
            return grid; // Return best attempt
        }
    }
}
```

#### 5.2 Chunked Generation
```rust
// For large maps, generate in chunks to improve performance
fn generate_map_chunked(config: &MapConfig, game_data: &GameData) -> Grid {
    let chunk_size = 32;
    let num_chunks_x = (config.width + chunk_size - 1) / chunk_size;
    let num_chunks_y = (config.height + chunk_size - 1) / chunk_size;

    let mut grid = create_empty_grid(config.width, config.height);

    // Generate chunks in parallel (if using rayon)
    for cy in 0..num_chunks_y {
        for cx in 0..num_chunks_x {
            generate_chunk(&mut grid, cx * chunk_size, cy * chunk_size, chunk_size, config);
        }
    }

    // Stitch chunks together (smooth boundaries)
    smooth_chunk_boundaries(&mut grid, chunk_size);

    grid
}
```

---

## Implementation Priority

### Immediate (Next Sprint)
1. ✅ Noise-based terrain generation
2. ✅ Cellular automata smoothing
3. ✅ Connectivity validation
4. ✅ Mineral vein systems

### Short-term (2-3 Sprints)
5. ⏳ Strategic resource placement
6. ⏳ Hero portal placement
7. ⏳ Variable starting positions
8. ⏳ Natural feature generation

### Medium-term (Future)
9. 🔮 Biome system
10. 🔮 Map quality metrics
11. 🔮 Chunked generation for large maps

---

## Example Usage

```rust
// Rich, varied map with natural caves
let config = MapConfig {
    width: 80,
    height: 80,
    seed: Some(12345),
    gold_richness: 0.4,
    gem_richness: 0.2,
    mana_richness: 0.15,
    water_frequency: 0.15,
    lava_frequency: 0.08,
    starting_area_size: 7,

    // New parameters
    use_noise_terrain: true,
    cave_density: 0.3,
    num_biomes: 4,
    num_hero_portals: 3,
    natural_features: true,
    starting_position_strategy: StartingPositionStrategy::Random,
};

let grid = generate_quality_map(&config, game_data);
```

---

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_map_is_connected() {
        let config = MapConfig::default();
        let grid = generate_map(&config, &game_data);

        let regions = find_disconnected_regions(&grid);
        assert_eq!(regions.len(), 1, "Map should have exactly one connected region");
    }

    #[test]
    fn test_resources_reachable() {
        let config = MapConfig::default();
        let grid = generate_map(&config, &game_data);
        let start = find_dungeon_heart(&grid).unwrap();

        let metrics = evaluate_map_quality(&grid, start);
        assert!(metrics.resource_accessibility > 0.8,
                "At least 80% of resources should be reachable");
    }

    #[test]
    fn test_deterministic_generation() {
        let config = MapConfig { seed: Some(42), ..Default::default() };
        let grid1 = generate_map(&config, &game_data);
        let grid2 = generate_map(&config, &game_data);

        assert_eq!(grid1, grid2, "Same seed should produce identical maps");
    }
}
```

---

## References
- **Perlin Noise**: Classic gradient noise for terrain
- **Cellular Automata**: Cave generation technique (Rogue-like games)
- **Voronoi Diagrams**: Biome boundary generation
- **Flood Fill**: Connectivity analysis
- **Drunk Walk**: Natural vein/river generation
- **Dwarf Fortress**: Inspiration for geological realism
- **Dungeon Keeper**: Original game's map design philosophy
