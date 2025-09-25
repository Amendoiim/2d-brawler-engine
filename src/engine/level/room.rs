//! Room-based level generation components

use glam::Vec2;
use std::collections::HashSet;

/// A room in the generated level
#[derive(Debug, Clone)]
pub struct Room {
    /// Unique identifier for the room
    pub id: String,
    /// Room position (top-left corner)
    pub x: u32,
    pub y: u32,
    /// Room dimensions
    pub width: u32,
    pub height: u32,
    /// Type of room
    pub room_type: RoomType,
    /// Connected room IDs
    pub connections: Vec<String>,
}

/// Types of rooms that can be generated
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoomType {
    /// Standard room with enemies and items
    Standard,
    /// Starting room for the player
    Start,
    /// Boss room with powerful enemies
    Boss,
    /// Treasure room with valuable items
    Treasure,
    /// Empty room (rare)
    Empty,
    /// Shop room with merchants
    Shop,
}

/// Connection between two rooms
#[derive(Debug, Clone)]
pub struct Connection {
    /// Source room ID
    pub from_room: String,
    /// Destination room ID
    pub to_room: String,
    /// Path points for the corridor
    pub path: Vec<Vec2>,
}

/// Spawn point for entities in the level
#[derive(Debug, Clone)]
pub struct SpawnPoint {
    /// World position
    pub x: f32,
    pub y: f32,
    /// Type of entity to spawn
    pub spawn_type: SpawnType,
}

/// Types of entities that can be spawned
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpawnType {
    /// Player character
    Player,
    /// Enemy entity
    Enemy,
    /// Item or pickup
    Item,
    /// Obstacle or decoration
    Obstacle,
    /// Boss enemy
    Boss,
    /// NPC or merchant
    NPC,
}

impl Room {
    /// Create a new room
    pub fn new(id: String, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            id,
            x,
            y,
            width,
            height,
            room_type: RoomType::Standard,
            connections: Vec::new(),
        }
    }

    /// Get the center point of the room
    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.x as f32 + self.width as f32 / 2.0,
            self.y as f32 + self.height as f32 / 2.0,
        )
    }

    /// Get the area of the room
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Check if a point is inside the room
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x as f32 && x < (self.x + self.width) as f32 &&
        y >= self.y as f32 && y < (self.y + self.height) as f32
    }

    /// Check if this room overlaps with another room
    pub fn overlaps_with(&self, other: &Room) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }

    /// Get the distance to another room
    pub fn distance_to(&self, other: &Room) -> f32 {
        let center1 = self.center();
        let center2 = other.center();
        center1.distance(center2)
    }

    /// Add a connection to another room
    pub fn add_connection(&mut self, room_id: String) {
        if !self.connections.contains(&room_id) {
            self.connections.push(room_id);
        }
    }

    /// Remove a connection to another room
    pub fn remove_connection(&mut self, room_id: &str) {
        self.connections.retain(|id| id != room_id);
    }

    /// Get all connected room IDs
    pub fn get_connections(&self) -> &[String] {
        &self.connections
    }

    /// Check if this room is connected to another room
    pub fn is_connected_to(&self, room_id: &str) -> bool {
        self.connections.contains(&room_id.to_string())
    }

    /// Get the room bounds as a rectangle
    pub fn bounds(&self) -> (u32, u32, u32, u32) {
        (self.x, self.y, self.width, self.height)
    }

    /// Get random position within the room
    pub fn random_position(&self, rng: &mut fastrand::Rng) -> Vec2 {
        let x = self.x + rng.u32(1..self.width - 1);
        let y = self.y + rng.u32(1..self.height - 1);
        Vec2::new(x as f32, y as f32)
    }

    /// Get all positions along the room's perimeter
    pub fn perimeter_positions(&self) -> Vec<Vec2> {
        let mut positions = Vec::new();
        
        // Top and bottom edges
        for x in self.x..self.x + self.width {
            positions.push(Vec2::new(x as f32, self.y as f32));
            positions.push(Vec2::new(x as f32, (self.y + self.height - 1) as f32));
        }
        
        // Left and right edges
        for y in self.y + 1..self.y + self.height - 1 {
            positions.push(Vec2::new(self.x as f32, y as f32));
            positions.push(Vec2::new((self.x + self.width - 1) as f32, y as f32));
        }
        
        positions
    }

    /// Get positions for doors (where corridors can connect)
    pub fn door_positions(&self) -> Vec<Vec2> {
        let mut positions = Vec::new();
        let center_x = self.x + self.width / 2;
        let center_y = self.y + self.height / 2;
        
        // Center of each wall
        positions.push(Vec2::new(center_x as f32, self.y as f32)); // Top
        positions.push(Vec2::new(center_x as f32, (self.y + self.height - 1) as f32)); // Bottom
        positions.push(Vec2::new(self.x as f32, center_y as f32)); // Left
        positions.push(Vec2::new((self.x + self.width - 1) as f32, center_y as f32)); // Right
        
        positions
    }
}

impl Connection {
    /// Create a new connection between two rooms
    pub fn new(from_room: String, to_room: String, path: Vec<Vec2>) -> Self {
        Self {
            from_room,
            to_room,
            path,
        }
    }

    /// Get the length of the connection path
    pub fn length(&self) -> f32 {
        let mut total_length = 0.0;
        for i in 1..self.path.len() {
            total_length += self.path[i - 1].distance(self.path[i]);
        }
        total_length
    }

    /// Check if the connection contains a specific point
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        let point = Vec2::new(x, y);
        for i in 1..self.path.len() {
            let start = self.path[i - 1];
            let end = self.path[i];
            
            // Check if point is on the line segment
            let distance_to_start = point.distance(start);
            let distance_to_end = point.distance(end);
            let segment_length = start.distance(end);
            
            if (distance_to_start + distance_to_end - segment_length).abs() < 0.1 {
                return true;
            }
        }
        false
    }

    /// Get all points along the connection path
    pub fn get_path_points(&self) -> &[Vec2] {
        &self.path
    }
}

impl SpawnPoint {
    /// Create a new spawn point
    pub fn new(x: f32, y: f32, spawn_type: SpawnType) -> Self {
        Self { x, y, spawn_type }
    }

    /// Get the position as a Vec2
    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// Set the position
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    /// Get the spawn type
    pub fn spawn_type(&self) -> &SpawnType {
        &self.spawn_type
    }

    /// Set the spawn type
    pub fn set_spawn_type(&mut self, spawn_type: SpawnType) {
        self.spawn_type = spawn_type;
    }
}
