//! Resource generation (mineral veins, hazards)
//! Implements risk/reward strategic resource placement

use crate::state::tile_state::TilePos;
use rand::Rng;
use std::collections::VecDeque;

use super::config::{Grid, MapConfig};
use super::utils::distance_f32;

// ============================================================================
// STRATEGIC RESOURCE PLACEMENT
// ============================================================================

/// Configuration for how a resource type should be placed
struct ResourcePlacementStrategy {
    min_distance_from_start: f32,
    max_distance_from_start: f32,
    prefer_near_hazards: bool,
    hazard_bonus_radius: f32,
}

/// Add mineral veins strategically based on risk/reward placement
pub fn add_mineral_veins(grid: &mut Grid, config: &MapConfig, start_pos: TilePos, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();
    
    // Gold veins - moderate distance, basic resources
    let gold_strategy = ResourcePlacementStrategy {
        min_distance_from_start: 8.0,
        max_distance_from_start: 35.0,
        prefer_near_hazards: false,
        hazard_bonus_radius: 0.0,
    };
    let num_gold_veins = (config.gold_richness * 10.0) as usize + 2;
    place_strategic_veins(grid, start_pos, "gold_vein", num_gold_veins, 12..25, 2, &gold_strategy, rng);
    
    // Gem seams - RARE, single tiles only (4-6 on entire map)
    // Far from start, dangerous areas - these are finite high-value finds
    let num_gems = 4 + (config.gem_richness * 3.0) as usize; // 4-6 gems
    place_single_gems(grid, start_pos, num_gems, rng);
    
    // Mana crystals - scattered but bonus if near hero portals
    let mana_strategy = ResourcePlacementStrategy {
        min_distance_from_start: 12.0,
        max_distance_from_start: f32::INFINITY,
        prefer_near_hazards: false,
        hazard_bonus_radius: 0.0,
    };
    let num_mana_veins = (config.mana_richness * 10.0) as usize + 2;
    place_strategic_veins(grid, start_pos, "mana_crystal", num_mana_veins, 8..15, 1, &mana_strategy, rng);
    
    // Place bonus mana near hero portals
    place_mana_near_portals(grid, rng);

    // Scattered resources (single tiles spread everywhere including near base)
    // Scale count by map size (assuming 50x50 base)
    let map_area = (width * height) as f32;
    let density_scale = map_area / 2500.0;
    
    let scattered_gold = (180.0 * config.gold_richness * density_scale) as usize;
    place_scattered_resources(grid, "gold_vein", scattered_gold, rng);
    
    let scattered_mana = (90.0 * config.mana_richness * density_scale) as usize;
    place_scattered_resources(grid, "mana_crystal", scattered_mana, rng);
}

/// Place veins using strategic placement rules
fn place_strategic_veins(
    grid: &mut Grid,
    start_pos: TilePos,
    tile_type: &str,
    count: usize,
    length_range: std::ops::Range<usize>,
    thickness: usize,
    strategy: &ResourcePlacementStrategy,
    rng: &mut impl Rng,
) {
    let height = grid.len();
    let width = grid[0].len();
    
    // Find candidate positions scored by strategy
    let candidates = find_strategic_positions(grid, start_pos, strategy, rng);
    
    let placed = candidates.len().min(count);
    for i in 0..placed {
        let pos = candidates[i];
        let length = rng.gen_range(length_range.clone());
        generate_mineral_vein(grid, pos, tile_type, length, thickness, rng);
    }
    
    // Fill remaining with random positions if not enough strategic ones
    for _ in placed..count {
        let start_x = rng.gen_range(5..width - 5);
        let start_y = rng.gen_range(5..height - 5);
        let pos = TilePos::new(start_x as i32, start_y as i32);
        
        let dist = distance_f32(pos, start_pos);
        if dist >= strategy.min_distance_from_start {
            let length = rng.gen_range(length_range.clone());
            generate_mineral_vein(grid, pos, tile_type, length, thickness, rng);
        }
    }
}

/// Find positions that match the placement strategy, scored by desirability
fn find_strategic_positions(
    grid: &Grid,
    start_pos: TilePos,
    strategy: &ResourcePlacementStrategy,
    rng: &mut impl Rng,
) -> Vec<TilePos> {
    let height = grid.len();
    let width = grid[0].len();
    
    let mut scored_positions: Vec<(TilePos, f32)> = Vec::new();
    
    // Sample positions (not every tile for performance)
    for y in (5..height - 5).step_by(3) {
        for x in (5..width - 5).step_by(3) {
            let pos = TilePos::new(x as i32, y as i32);
            
            // Must be in diggable terrain (solid rock or earth)
            let tile_type = &grid[y][x].tile_type;
            if tile_type != "solid_rock" && tile_type != "earth" {
                continue;
            }
            
            let dist = distance_f32(pos, start_pos);
            
            // Check distance constraints
            if dist < strategy.min_distance_from_start || dist > strategy.max_distance_from_start {
                continue;
            }
            
            // Calculate score
            let mut score = dist; // Base score is distance from start
            
            // Bonus for being near hazards
            if strategy.prefer_near_hazards {
                let hazard_danger = calculate_tile_danger(grid, pos, strategy.hazard_bonus_radius);
                score += hazard_danger * 5.0; // Hazard proximity bonus
            }
            
            // Add some randomness
            score += rng.gen::<f32>() * 3.0;
            
            scored_positions.push((pos, score));
        }
    }
    
    // Sort by score (highest first for strategic positions)
    scored_positions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    // Return top positions
    scored_positions.into_iter().map(|(pos, _)| pos).collect()
}

/// Calculate danger score for a tile based on nearby hazards
fn calculate_tile_danger(grid: &Grid, pos: TilePos, radius: f32) -> f32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut danger = 0.0;
    let r = radius as i32;
    
    for dy in -r..=r {
        for dx in -r..=r {
            let nx = pos.x + dx;
            let ny = pos.y + dy;
            
            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let tile_type = &grid[ny as usize][nx as usize].tile_type;
                
                // Check for hazards
                if tile_type == "lava" {
                    let dist = ((dx * dx + dy * dy) as f32).sqrt();
                    danger += 3.0 / (dist + 1.0); // Lava is very dangerous
                } else if tile_type == "water" {
                    let dist = ((dx * dx + dy * dy) as f32).sqrt();
                    danger += 1.0 / (dist + 1.0); // Water is mildly dangerous
                } else if tile_type == "hero_portal" {
                    let dist = ((dx * dx + dy * dy) as f32).sqrt();
                    danger += 5.0 / (dist + 1.0); // Portals are very dangerous
                }
            }
        }
    }
    
    danger
}

/// Place mana crystals near hero portals (high risk/reward)
fn place_mana_near_portals(grid: &mut Grid, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();
    
    // Find all hero portals
    let portal_positions: Vec<TilePos> = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter(|&(x, y)| grid[y][x].tile_type == "hero_portal")
        .map(|(x, y)| TilePos::new(x as i32, y as i32))
        .collect();
    
    // Place small mana veins near each portal
    for portal_pos in portal_positions {
        // Find a nearby solid rock position
        for _ in 0..5 {
            let dx = rng.gen_range(-6..=6);
            let dy = rng.gen_range(-6..=6);
            let nx = (portal_pos.x + dx).max(2).min(width as i32 - 3) as usize;
            let ny = (portal_pos.y + dy).max(2).min(height as i32 - 3) as usize;
            
            if grid[ny][nx].tile_type == "solid_rock" {
                generate_mineral_vein(
                    grid,
                    TilePos::new(nx as i32, ny as i32),
                    "mana_crystal",
                    rng.gen_range(5..10),
                    1,
                    rng,
                );
                break;
            }
        }
    }
}

/// Place individual gem seam tiles (not veins) - rare, valuable finds
fn place_single_gems(grid: &mut Grid, start_pos: TilePos, count: usize, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();
    let min_distance = 18.0; // Far from start
    
    let mut placed = 0;
    let mut attempts = 0;
    
    while placed < count && attempts < 200 {
        attempts += 1;
        
        let x = rng.gen_range(5..width - 5);
        let y = rng.gen_range(5..height - 5);
        let pos = TilePos::new(x as i32, y as i32);
        
        // Must be far from start
        if distance_f32(pos, start_pos) < min_distance {
            continue;
        }
        
        // Must be in diggable terrain
        let tile_type = &grid[y][x].tile_type;
        if tile_type != "solid_rock" && tile_type != "earth" {
            continue;
        }
        
        // Prefer positions near hazards (lava = bonus)
        let danger = calculate_tile_danger(grid, pos, 6.0);
        if danger < 0.5 && rng.gen::<f32>() > 0.3 {
            continue; // Skip low-danger spots 70% of the time
        }
        
        // Place single gem tile
        grid[y][x].tile_type = "gem_seam".to_string();
        grid[y][x].resources_remaining = Some(rng.gen_range(200..400)); // Finite resources
        placed += 1;
    }
}

/// Place scattered single resource tiles across the map (no distance constraints)
fn place_scattered_resources(grid: &mut Grid, tile_type: &str, count: usize, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();
    
    let mut placed = 0;
    let mut attempts = 0;
    
    while placed < count && attempts < count * 10 {
        attempts += 1;
        
        let x = rng.gen_range(2..width - 2);
        let y = rng.gen_range(2..height - 2);
        
        // Must be in diggable terrain
        let current_type = &grid[y][x].tile_type;
        if current_type != "solid_rock" && current_type != "earth" {
            continue;
        }
        
        // Place resource
        grid[y][x].tile_type = tile_type.to_string();
        
        // Set resource amount based on type
        let amount = match tile_type {
            "gold_vein" => rng.gen_range(50..100), // Smaller amount for single tiles
            "mana_crystal" => rng.gen_range(100..200),
            _ => 100
        };
        
        grid[y][x].resources_remaining = Some(amount);
        placed += 1;
    }
}

// ============================================================================
// MINERAL VEIN GENERATION
// ============================================================================

/// Generate a single mineral vein using drunk walk algorithm
fn generate_mineral_vein(grid: &mut Grid, start: TilePos, tile_type: &str, length: usize, thickness: usize, rng: &mut impl Rng) {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut current = start;
    let mut direction = (rng.gen_range(-1..=1), rng.gen_range(-1..=1));
    if direction == (0, 0) { direction = (1, 0); }

    for _ in 0..length {
        place_vein_segment(grid, current, tile_type, thickness, rng);

        if rng.gen::<f32>() < 0.7 {
            current.x += direction.0;
            current.y += direction.1;
        } else {
            direction = (rng.gen_range(-1..=1), rng.gen_range(-1..=1));
            if direction == (0, 0) { direction = (1, 0); }
            current.x += direction.0;
            current.y += direction.1;
        }

        current.x = current.x.max(1).min(width - 2);
        current.y = current.y.max(1).min(height - 2);
    }
}

/// Place a circular segment of mineral vein
fn place_vein_segment(grid: &mut Grid, center: TilePos, tile_type: &str, thickness: usize, rng: &mut impl Rng) {
    let t = thickness as i32;
    for dy in -t..=t {
        for dx in -t..=t {
            if dx*dx + dy*dy <= t * t {
                let x = (center.x + dx).max(1).min(grid[0].len() as i32 - 2) as usize;
                let y = (center.y + dy).max(1).min(grid.len() as i32 - 2) as usize;

                // Allow veins in solid rock OR earth (both diggable terrains)
                let current_type = &grid[y][x].tile_type;
                if current_type == "solid_rock" || current_type == "earth" {
                    grid[y][x].tile_type = tile_type.to_string();
                    let resources = match tile_type {
                        "gold_vein" => rng.gen_range(80..150),
                        "gem_seam" => rng.gen_range(150..300),
                        "mana_crystal" => rng.gen_range(200..350),
                        _ => 100,
                    };
                    grid[y][x].resources_remaining = Some(resources);
                }
            }
        }
    }
}

// ============================================================================
// HAZARD GENERATION
// ============================================================================

/// Add hazard regions with organic shapes
pub fn add_enhanced_hazards(grid: &mut Grid, config: &MapConfig, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();

    if rng.gen::<f32>() < config.water_frequency {
        let num_water = rng.gen_range(1..3);
        for _ in 0..num_water {
            let cx = rng.gen_range(10..width - 10);
            let cy = rng.gen_range(10..height - 10);
            let size = rng.gen_range(5..12);
            create_organic_hazard_pool(grid, cx, cy, size, "water", rng);
        }
    }

    if rng.gen::<f32>() < config.lava_frequency {
        let num_lava = rng.gen_range(1..3);
        for _ in 0..num_lava {
            let cx = rng.gen_range(10..width - 10);
            let cy = rng.gen_range(10..height - 10);
            let size = rng.gen_range(4..9);
            create_organic_hazard_pool(grid, cx, cy, size, "lava", rng);
        }
    }
}

/// Create hazard pool with organic shape using flood fill
fn create_organic_hazard_pool(grid: &mut Grid, cx: usize, cy: usize, target_size: usize, tile_type: &str, rng: &mut impl Rng) {
    let height = grid.len();
    let width = grid[0].len();

    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    queue.push_back((cx, cy));
    visited[cy][cx] = true;

    let mut tiles_placed = 0;
    let max_tiles = target_size * target_size;

    while let Some((x, y)) = queue.pop_front() {
        if tiles_placed >= max_tiles { break; }

        if grid[y][x].tile_type == "earth" {
            grid[y][x].tile_type = tile_type.to_string();
            grid[y][x].resources_remaining = None;
            tiles_placed += 1;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;

            if nx > 5 && nx < width - 5 && ny > 5 && ny < height - 5 {
                if !visited[ny][nx] && rng.gen::<f32>() < 0.65 {
                    visited[ny][nx] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }
}
