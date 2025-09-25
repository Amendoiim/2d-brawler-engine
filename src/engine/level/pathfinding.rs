//! Pathfinding system for level navigation

use glam::Vec2;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

/// A simple position type that implements Hash and Eq for pathfinding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_vec2(v: Vec2) -> Self {
        Self {
            x: v.x as i32,
            y: v.y as i32,
        }
    }

    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

/// A pathfinding node for A* algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct PathNode {
    /// Position of the node
    pub position: Position,
    /// Cost to reach this node from start
    pub g_cost: f32,
    /// Heuristic cost to reach goal from this node
    pub h_cost: f32,
    /// Total cost (g + h)
    pub f_cost: f32,
    /// Parent node for path reconstruction
    pub parent: Option<Position>,
}

impl PathNode {
    /// Create a new pathfinding node
    pub fn new(position: Vec2, g_cost: f32, h_cost: f32) -> Self {
        Self {
            position: Position::from_vec2(position),
            g_cost,
            h_cost,
            f_cost: g_cost + h_cost,
            parent: None,
        }
    }

    /// Update the node with new costs
    pub fn update(&mut self, g_cost: f32, h_cost: f32, parent: Option<Vec2>) {
        self.g_cost = g_cost;
        self.h_cost = h_cost;
        self.f_cost = g_cost + h_cost;
        self.parent = parent.map(Position::from_vec2);
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Reverse order for min-heap (lowest f_cost first)
        other.f_cost.partial_cmp(&self.f_cost)
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Eq for PathNode {}

/// Pathfinding system for navigating through levels
pub struct Pathfinder {
    /// Width of the level grid
    pub width: u32,
    /// Height of the level grid
    pub height: u32,
    /// Walkable tiles
    pub walkable_tiles: HashSet<TileType>,
}

/// Types of tiles for pathfinding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    /// Empty space
    Empty,
    /// Floor tile
    Floor,
    /// Wall tile
    Wall,
    /// Door tile
    Door,
    /// Water tile
    Water,
    /// Other tile types
    Other,
}

impl Pathfinder {
    /// Create a new pathfinder
    pub fn new(width: u32, height: u32) -> Self {
        let mut walkable_tiles = HashSet::new();
        walkable_tiles.insert(TileType::Empty);
        walkable_tiles.insert(TileType::Floor);
        walkable_tiles.insert(TileType::Water);

        Self {
            width,
            height,
            walkable_tiles,
        }
    }

    /// Set which tile types are walkable
    pub fn set_walkable_tiles(&mut self, tiles: HashSet<TileType>) {
        self.walkable_tiles = tiles;
    }

    /// Find a path from start to goal using A* algorithm
    pub fn find_path(&self, start: Vec2, goal: Vec2, is_walkable: impl Fn(u32, u32) -> bool) -> Option<Vec<Vec2>> {
        if !self.is_valid_position(start) || !self.is_valid_position(goal) {
            return None;
        }

        if start == goal {
            return Some(vec![start]);
        }

        let start_pos = Position::from_vec2(start);
        let goal_pos = Position::from_vec2(goal);

        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        let mut came_from = HashMap::new();

        let start_node = PathNode::new(start, 0.0, self.heuristic(start, goal));
        open_set.push(start_node);

        while let Some(current) = open_set.pop() {
            if current.position == goal_pos {
                return Some(self.reconstruct_path(came_from, current.position));
            }

            closed_set.insert(current.position);

            for neighbor in self.get_neighbors(current.position.to_vec2()) {
                let neighbor_pos = Position::from_vec2(neighbor);
                if closed_set.contains(&neighbor_pos) {
                    continue;
                }

                let neighbor_x = neighbor_pos.x as u32;
                let neighbor_y = neighbor_pos.y as u32;

                if !is_walkable(neighbor_x, neighbor_y) {
                    continue;
                }

                let tentative_g_cost = current.g_cost + current.position.to_vec2().distance(neighbor);

                if let Some(existing_node) = open_set.iter().find(|n| n.position == neighbor_pos) {
                    if tentative_g_cost >= existing_node.g_cost {
                        continue;
                    }
                }

                let h_cost = self.heuristic(neighbor, goal);
                let mut neighbor_node = PathNode::new(neighbor, tentative_g_cost, h_cost);
                neighbor_node.parent = Some(current.position);

                came_from.insert(neighbor_pos, current.position);

                // Remove existing node if it exists
                open_set.retain(|n| n.position != neighbor_pos);
                open_set.push(neighbor_node);
            }
        }

        None
    }

    /// Find a path using breadth-first search (simpler but less optimal)
    pub fn find_path_bfs(&self, start: Vec2, goal: Vec2, is_walkable: impl Fn(u32, u32) -> bool) -> Option<Vec<Vec2>> {
        if !self.is_valid_position(start) || !self.is_valid_position(goal) {
            return None;
        }

        if start == goal {
            return Some(vec![start]);
        }

        let start_pos = Position::from_vec2(start);
        let goal_pos = Position::from_vec2(goal);

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();

        queue.push_back(start_pos);
        visited.insert(start_pos);

        while let Some(current) = queue.pop_front() {
            if current == goal_pos {
                return Some(self.reconstruct_path(parent, current));
            }

            for neighbor in self.get_neighbors(current.to_vec2()) {
                let neighbor_pos = Position::from_vec2(neighbor);
                if visited.contains(&neighbor_pos) {
                    continue;
                }

                let neighbor_x = neighbor_pos.x as u32;
                let neighbor_y = neighbor_pos.y as u32;

                if !is_walkable(neighbor_x, neighbor_y) {
                    continue;
                }

                visited.insert(neighbor_pos);
                parent.insert(neighbor_pos, current);
                queue.push_back(neighbor_pos);
            }
        }

        None
    }

    /// Check if two points are connected (can reach each other)
    pub fn is_connected(&self, start: Vec2, goal: Vec2, is_walkable: impl Fn(u32, u32) -> bool) -> bool {
        self.find_path(start, goal, is_walkable).is_some()
    }

    /// Get the distance between two points (if path exists)
    pub fn get_distance(&self, start: Vec2, goal: Vec2, is_walkable: impl Fn(u32, u32) -> bool) -> Option<f32> {
        if let Some(path) = self.find_path(start, goal, is_walkable) {
            let mut distance = 0.0;
            for i in 1..path.len() {
                distance += path[i - 1].distance(path[i]);
            }
            Some(distance)
        } else {
            None
        }
    }

    /// Get all reachable positions from a starting point
    pub fn get_reachable_positions(&self, start: Vec2, max_distance: f32, is_walkable: impl Fn(u32, u32) -> bool) -> Vec<Vec2> {
        let mut reachable = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut distances = HashMap::new();

        let start_pos = Position::from_vec2(start);
        queue.push_back(start_pos);
        visited.insert(start_pos);
        distances.insert(start_pos, 0.0);

        while let Some(current) = queue.pop_front() {
            let current_distance = distances[&current];
            if current_distance > max_distance {
                continue;
            }

            reachable.push(current.to_vec2());

            for neighbor in self.get_neighbors(current.to_vec2()) {
                let neighbor_pos = Position::from_vec2(neighbor);
                if visited.contains(&neighbor_pos) {
                    continue;
                }

                let neighbor_x = neighbor_pos.x as u32;
                let neighbor_y = neighbor_pos.y as u32;

                if !is_walkable(neighbor_x, neighbor_y) {
                    continue;
                }

                let distance = current_distance + current.to_vec2().distance(neighbor);
                if distance <= max_distance {
                    visited.insert(neighbor_pos);
                    distances.insert(neighbor_pos, distance);
                    queue.push_back(neighbor_pos);
                }
            }
        }

        reachable
    }

    /// Check if a position is valid within the grid
    fn is_valid_position(&self, pos: Vec2) -> bool {
        pos.x >= 0.0 && pos.x < self.width as f32 && pos.y >= 0.0 && pos.y < self.height as f32
    }

    /// Get neighboring positions (8-directional)
    fn get_neighbors(&self, pos: Vec2) -> Vec<Vec2> {
        let mut neighbors = Vec::new();
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for (dx, dy) in directions.iter() {
            let new_x = pos.x + *dx as f32;
            let new_y = pos.y + *dy as f32;
            let new_pos = Vec2::new(new_x, new_y);
            
            if self.is_valid_position(new_pos) {
                neighbors.push(new_pos);
            }
        }

        neighbors
    }

    /// Calculate heuristic distance (Euclidean distance)
    fn heuristic(&self, a: Vec2, b: Vec2) -> f32 {
        a.distance(b)
    }

    /// Reconstruct path from came_from map
    fn reconstruct_path(&self, came_from: HashMap<Position, Position>, mut current: Position) -> Vec<Vec2> {
        let mut path = vec![current.to_vec2()];
        
        while let Some(&parent) = came_from.get(&current) {
            path.push(parent.to_vec2());
            current = parent;
        }
        
        path.reverse();
        path
    }
}

/// Simple pathfinding utilities
pub struct PathfindingUtils;

impl PathfindingUtils {
    /// Smooth a path by removing unnecessary waypoints
    pub fn smooth_path(path: &[Vec2], is_walkable: impl Fn(u32, u32) -> bool) -> Vec<Vec2> {
        if path.len() <= 2 {
            return path.to_vec();
        }

        let mut smoothed = vec![path[0]];
        let mut i = 0;

        while i < path.len() - 1 {
            let mut j = path.len() - 1;
            let mut found_direct_path = false;

            while j > i + 1 {
                if Self::has_direct_line_of_sight(&path[i], &path[j], &is_walkable) {
                    smoothed.push(path[j]);
                    i = j;
                    found_direct_path = true;
                    break;
                }
                j -= 1;
            }

            if !found_direct_path {
                smoothed.push(path[i + 1]);
                i += 1;
            }
        }

        smoothed
    }

    /// Check if there's a direct line of sight between two points
    fn has_direct_line_of_sight(start: &Vec2, end: &Vec2, is_walkable: &impl Fn(u32, u32) -> bool) -> bool {
        let steps = ((end.x - start.x).abs().max((end.y - start.y).abs()) as u32).max(1);
        let step_x = (end.x - start.x) / steps as f32;
        let step_y = (end.y - start.y) / steps as f32;

        for i in 0..=steps {
            let x = (start.x + step_x * i as f32) as u32;
            let y = (start.y + step_y * i as f32) as u32;
            
            if !is_walkable(x, y) {
                return false;
            }
        }

        true
    }

    /// Calculate path length
    pub fn calculate_path_length(path: &[Vec2]) -> f32 {
        let mut length = 0.0;
        for i in 1..path.len() {
            length += path[i - 1].distance(path[i]);
        }
        length
    }

    /// Find the closest walkable position to a target
    pub fn find_closest_walkable(target: Vec2, max_radius: f32, is_walkable: impl Fn(u32, u32) -> bool) -> Option<Vec2> {
        let mut radius = 1.0;
        
        while radius <= max_radius {
            let steps = (radius * 8.0) as u32;
            let angle_step = 2.0 * std::f32::consts::PI / steps as f32;
            
            for i in 0..steps {
                let angle = i as f32 * angle_step;
                let x = (target.x + radius * angle.cos()) as u32;
                let y = (target.y + radius * angle.sin()) as u32;
                
                if is_walkable(x, y) {
                    return Some(Vec2::new(x as f32, y as f32));
                }
            }
            
            radius += 1.0;
        }
        
        None
    }
}
