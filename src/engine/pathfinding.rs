//! A* Pathfinding module
//! Designed to be extracted to macroquad-toolkit
//! Zero game-specific dependencies - completely self-contained

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Generic 2D position for pathfinding
/// No game-specific logic - just coordinates
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Calculate Manhattan distance to another position
    pub fn manhattan_distance(&self, other: &Pos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Calculate Euclidean distance to another position
    pub fn euclidean_distance(&self, other: &Pos) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate squared Euclidean distance (faster than euclidean_distance)
    pub fn euclidean_distance_squared(&self, other: &Pos) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        dx * dx + dy * dy
    }

    /// Get 4-way neighbors (N, S, E, W)
    pub fn neighbors_4way(&self) -> [Pos; 4] {
        [
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x, self.y - 1),
        ]
    }

    /// Get 8-way neighbors (includes diagonals)
    pub fn neighbors_8way(&self) -> [Pos; 8] {
        [
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x, self.y - 1),
            Pos::new(self.x + 1, self.y + 1),
            Pos::new(self.x + 1, self.y - 1),
            Pos::new(self.x - 1, self.y + 1),
            Pos::new(self.x - 1, self.y - 1),
        ]
    }
}

/// A pathfinding grid with walkability and cost information
#[derive(Debug, Clone)]
pub struct PathfindingGrid {
    pub width: usize,
    pub height: usize,
    walkable: Vec<Vec<bool>>,
    cost: Vec<Vec<f32>>,
}

impl PathfindingGrid {
    /// Create a new pathfinding grid
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            walkable: vec![vec![true; width]; height],
            cost: vec![vec![1.0; width]; height],
        }
    }

    /// Check if a position is within bounds
    pub fn is_valid(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.y >= 0 && (pos.x as usize) < self.width && (pos.y as usize) < self.height
    }

    /// Check if a position is walkable
    pub fn is_walkable(&self, pos: Pos) -> bool {
        if !self.is_valid(pos) {
            return false;
        }
        self.walkable[pos.y as usize][pos.x as usize]
    }

    /// Get the cost of moving to a position
    pub fn get_cost(&self, pos: Pos) -> f32 {
        if !self.is_valid(pos) {
            return f32::INFINITY;
        }
        self.cost[pos.y as usize][pos.x as usize]
    }

    /// Set whether a position is walkable
    pub fn set_walkable(&mut self, pos: Pos, walkable: bool) {
        if self.is_valid(pos) {
            self.walkable[pos.y as usize][pos.x as usize] = walkable;
        }
    }

    /// Set the movement cost for a position
    pub fn set_cost(&mut self, pos: Pos, cost: f32) {
        if self.is_valid(pos) {
            self.cost[pos.y as usize][pos.x as usize] = cost;
        }
    }

    /// Set a rectangular region as walkable/unwalkable
    pub fn set_region_walkable(&mut self, min: Pos, max: Pos, walkable: bool) {
        let min_x = min.x.max(0) as usize;
        let min_y = min.y.max(0) as usize;
        let max_x = (max.x as usize).min(self.width);
        let max_y = (max.y as usize).min(self.height);

        for y in min_y..max_y {
            for x in min_x..max_x {
                self.walkable[y][x] = walkable;
            }
        }
    }
}

/// A pathfinding result with waypoints and cost
#[derive(Debug, Clone)]
pub struct Path {
    pub waypoints: Vec<Pos>,
    pub total_cost: f32,
}

impl Path {
    /// Create a new path
    pub fn new(waypoints: Vec<Pos>, total_cost: f32) -> Self {
        Self {
            waypoints,
            total_cost,
        }
    }

    /// Get the length of the path (number of waypoints)
    pub fn len(&self) -> usize {
        self.waypoints.len()
    }

    /// Check if path is empty
    pub fn is_empty(&self) -> bool {
        self.waypoints.is_empty()
    }

    /// Get the first waypoint (start position)
    pub fn start(&self) -> Option<Pos> {
        self.waypoints.first().copied()
    }

    /// Get the last waypoint (goal position)
    pub fn goal(&self) -> Option<Pos> {
        self.waypoints.last().copied()
    }
}

/// Node in the A* priority queue
#[derive(Debug, Clone)]
struct AStarNode {
    pos: Pos,
    f_score: f32, // g_score + heuristic
    g_score: f32, // Actual cost from start
}

impl Eq for AStarNode {}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap (lower f_score = higher priority)
        other
            .f_score
            .partial_cmp(&self.f_score)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Heuristic function type
pub enum Heuristic {
    Manhattan,
    Euclidean,
}

impl Heuristic {
    fn estimate(&self, from: Pos, to: Pos) -> f32 {
        match self {
            Heuristic::Manhattan => from.manhattan_distance(&to) as f32,
            Heuristic::Euclidean => from.euclidean_distance(&to),
        }
    }
}

/// Find a path from start to goal using A* algorithm
pub fn find_path(
    start: Pos,
    goal: Pos,
    grid: &PathfindingGrid,
    heuristic: Heuristic,
    allow_diagonals: bool,
) -> Option<Path> {
    // Early exit checks
    if !grid.is_walkable(start) || !grid.is_walkable(goal) {
        return None;
    }

    if start == goal {
        return Some(Path::new(vec![start], 0.0));
    }

    // A* data structures
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Pos, Pos> = HashMap::new();
    let mut g_scores: HashMap<Pos, f32> = HashMap::new();
    let mut closed_set: HashSet<Pos> = HashSet::new();

    // Initialize start node
    g_scores.insert(start, 0.0);
    open_set.push(AStarNode {
        pos: start,
        f_score: heuristic.estimate(start, goal),
        g_score: 0.0,
    });

    while let Some(current_node) = open_set.pop() {
        let current = current_node.pos;

        // Goal reached
        if current == goal {
            return Some(reconstruct_path(&came_from, current, current_node.g_score));
        }

        // Skip if already processed
        if closed_set.contains(&current) {
            continue;
        }

        closed_set.insert(current);

        // Get neighbors based on movement mode
        let neighbors: Vec<Pos> = if allow_diagonals {
            current.neighbors_8way().to_vec()
        } else {
            current.neighbors_4way().to_vec()
        };

        for neighbor in neighbors {
            // Skip invalid or unwalkable positions
            if !grid.is_walkable(neighbor) || closed_set.contains(&neighbor) {
                continue;
            }

            // Calculate movement cost
            let move_cost = if allow_diagonals {
                // Diagonal movement costs sqrt(2) ≈ 1.414
                let is_diagonal = (current.x - neighbor.x).abs() + (current.y - neighbor.y).abs() == 2;
                if is_diagonal {
                    1.414 * grid.get_cost(neighbor)
                } else {
                    grid.get_cost(neighbor)
                }
            } else {
                grid.get_cost(neighbor)
            };

            let tentative_g_score = current_node.g_score + move_cost;

            // Check if this path to neighbor is better than previous
            let neighbor_g_score = g_scores.get(&neighbor).copied().unwrap_or(f32::INFINITY);

            if tentative_g_score < neighbor_g_score {
                // This path is better, record it
                came_from.insert(neighbor, current);
                g_scores.insert(neighbor, tentative_g_score);

                let f_score = tentative_g_score + heuristic.estimate(neighbor, goal);

                open_set.push(AStarNode {
                    pos: neighbor,
                    f_score,
                    g_score: tentative_g_score,
                });
            }
        }
    }

    // No path found
    None
}

/// Reconstruct path from came_from map
fn reconstruct_path(came_from: &HashMap<Pos, Pos>, mut current: Pos, total_cost: f32) -> Path {
    let mut path = vec![current];

    while let Some(&parent) = came_from.get(&current) {
        path.push(parent);
        current = parent;
    }

    path.reverse();
    Path::new(path, total_cost)
}

/// Path cache for avoiding repeated pathfinding calculations
pub struct PathCache {
    cache: HashMap<(Pos, Pos), Path>,
    invalidated_positions: HashSet<Pos>,
    max_cache_size: usize,
}

impl PathCache {
    /// Create a new path cache
    pub fn new(max_cache_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            invalidated_positions: HashSet::new(),
            max_cache_size,
        }
    }

    /// Find a path with caching
    pub fn find_path_cached(
        &mut self,
        start: Pos,
        goal: Pos,
        grid: &PathfindingGrid,
        heuristic: Heuristic,
        allow_diagonals: bool,
    ) -> Option<Path> {
        let key = (start, goal);

        // Check if we need to invalidate this cached path
        if let Some(cached_path) = self.cache.get(&key) {
            // Check if any waypoint has been invalidated
            let is_invalid = cached_path
                .waypoints
                .iter()
                .any(|pos| self.invalidated_positions.contains(pos));

            if !is_invalid {
                // Cache hit - return cloned path
                return Some(cached_path.clone());
            } else {
                // Path invalidated, remove from cache
                self.cache.remove(&key);
            }
        }

        // Cache miss - compute new path
        if let Some(path) = find_path(start, goal, grid, heuristic, allow_diagonals) {
            // Add to cache if we have room
            if self.cache.len() < self.max_cache_size {
                self.cache.insert(key, path.clone());
            } else {
                // Cache full - could implement LRU here, for now just skip caching
            }
            Some(path)
        } else {
            None
        }
    }

    /// Invalidate paths that pass through specific positions
    pub fn invalidate_positions(&mut self, positions: &[Pos]) {
        for pos in positions {
            self.invalidated_positions.insert(*pos);
        }

        // Remove paths that use invalidated positions
        self.cache.retain(|_, path| {
            !path
                .waypoints
                .iter()
                .any(|pos| self.invalidated_positions.contains(pos))
        });
    }

    /// Clear all invalidated positions tracking
    pub fn clear_invalidations(&mut self) {
        self.invalidated_positions.clear();
    }

    /// Clear entire cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.invalidated_positions.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            cached_paths: self.cache.len(),
            invalidated_positions: self.invalidated_positions.len(),
            max_size: self.max_cache_size,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub cached_paths: usize,
    pub invalidated_positions: usize,
    pub max_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let a = Pos::new(0, 0);
        let b = Pos::new(3, 4);
        assert_eq!(a.manhattan_distance(&b), 7);
    }

    #[test]
    fn test_straight_line_path() {
        let mut grid = PathfindingGrid::new(10, 10);
        let start = Pos::new(0, 0);
        let goal = Pos::new(5, 0);

        let path = find_path(start, goal, &grid, Heuristic::Manhattan, false);
        assert!(path.is_some());

        let path = path.unwrap();
        assert_eq!(path.len(), 6); // 0,1,2,3,4,5
        assert_eq!(path.start(), Some(start));
        assert_eq!(path.goal(), Some(goal));
    }

    #[test]
    fn test_path_around_obstacle() {
        let mut grid = PathfindingGrid::new(5, 5);

        // Create a wall at x=2 from y=0 to y=3
        for y in 0..4 {
            grid.set_walkable(Pos::new(2, y), false);
        }

        let start = Pos::new(0, 2);
        let goal = Pos::new(4, 2);

        let path = find_path(start, goal, &grid, Heuristic::Manhattan, false);
        assert!(path.is_some());

        let path = path.unwrap();
        // Path must go around the wall
        assert!(!path.waypoints.contains(&Pos::new(2, 0)));
        assert!(!path.waypoints.contains(&Pos::new(2, 1)));
        assert!(!path.waypoints.contains(&Pos::new(2, 2)));
    }

    #[test]
    fn test_no_path() {
        let mut grid = PathfindingGrid::new(5, 5);

        // Create a complete wall blocking the path
        for y in 0..5 {
            grid.set_walkable(Pos::new(2, y), false);
        }

        let start = Pos::new(0, 2);
        let goal = Pos::new(4, 2);

        let path = find_path(start, goal, &grid, Heuristic::Manhattan, false);
        assert!(path.is_none());
    }

    #[test]
    fn test_diagonal_movement() {
        let grid = PathfindingGrid::new(10, 10);
        let start = Pos::new(0, 0);
        let goal = Pos::new(5, 5);

        // With diagonals, should find shorter path
        let path_diagonal = find_path(start, goal, &grid, Heuristic::Euclidean, true);
        let path_4way = find_path(start, goal, &grid, Heuristic::Manhattan, false);

        assert!(path_diagonal.is_some());
        assert!(path_4way.is_some());

        // Diagonal path should be shorter
        assert!(path_diagonal.unwrap().len() < path_4way.unwrap().len());
    }

    #[test]
    fn test_path_cache() {
        let grid = PathfindingGrid::new(10, 10);
        let mut cache = PathCache::new(100);

        let start = Pos::new(0, 0);
        let goal = Pos::new(5, 5);

        // First call - cache miss
        let path1 = cache.find_path_cached(start, goal, &grid, Heuristic::Manhattan, false);
        assert!(path1.is_some());

        // Second call - cache hit (should return same path)
        let path2 = cache.find_path_cached(start, goal, &grid, Heuristic::Manhattan, false);
        assert!(path2.is_some());

        assert_eq!(path1.unwrap().len(), path2.unwrap().len());
    }

    #[test]
    fn test_cache_invalidation() {
        let grid = PathfindingGrid::new(10, 10);
        let mut cache = PathCache::new(100);

        let start = Pos::new(0, 0);
        let goal = Pos::new(5, 0);

        // Cache a path
        let path1 = cache.find_path_cached(start, goal, &grid, Heuristic::Manhattan, false);
        assert!(path1.is_some());

        let stats = cache.stats();
        assert_eq!(stats.cached_paths, 1);

        // Invalidate a position on the path
        cache.invalidate_positions(&[Pos::new(2, 0)]);

        let stats = cache.stats();
        assert_eq!(stats.cached_paths, 0); // Path should be removed
    }
}
