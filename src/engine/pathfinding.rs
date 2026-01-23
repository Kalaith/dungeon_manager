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

}

/// A pathfinding result with waypoints
#[derive(Debug, Clone)]
pub struct Path {
    pub waypoints: Vec<Pos>,
}

impl Path {
    /// Create a new path
    pub fn new(waypoints: Vec<Pos>) -> Self {
        Self { waypoints }
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
}

impl Heuristic {
    fn estimate(&self, from: Pos, to: Pos) -> f32 {
        match self {
            Heuristic::Manhattan => from.manhattan_distance(&to) as f32,
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
        return Some(Path::new(vec![start]));
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
            return Some(reconstruct_path(&came_from, current));
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
fn reconstruct_path(came_from: &HashMap<Pos, Pos>, mut current: Pos) -> Path {
    let mut path = vec![current];

    while let Some(&parent) = came_from.get(&current) {
        path.push(parent);
        current = parent;
    }

    path.reverse();
    Path::new(path)
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
        let grid = PathfindingGrid::new(10, 10);
        let start = Pos::new(0, 0);
        let goal = Pos::new(5, 0);

        let path = find_path(start, goal, &grid, Heuristic::Manhattan, false);
        assert!(path.is_some());

        let path = path.unwrap();
        assert_eq!(path.waypoints.len(), 6); // 0,1,2,3,4,5
        assert_eq!(path.waypoints.first().copied(), Some(start));
        assert_eq!(path.waypoints.last().copied(), Some(goal));
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
        let path_diagonal = find_path(start, goal, &grid, Heuristic::Manhattan, true);
        let path_4way = find_path(start, goal, &grid, Heuristic::Manhattan, false);

        assert!(path_diagonal.is_some());
        assert!(path_4way.is_some());

        // Diagonal path should be shorter
        assert!(path_diagonal.unwrap().waypoints.len() < path_4way.unwrap().waypoints.len());
    }
}
