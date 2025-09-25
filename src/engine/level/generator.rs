//! Advanced level generation algorithms and patterns

use crate::engine::level::{Level, LevelConfig, Room, RoomType, Connection, SpawnPoint, SpawnType};
use crate::engine::level::tile::Tile;
use crate::engine::level::biome::{Biome, BiomeManager};
use glam::Vec2;
use std::collections::HashMap;
use std::collections::HashSet;

/// Advanced level generator with multiple generation algorithms
pub struct AdvancedLevelGenerator {
    /// Random number generator
    pub rng: fastrand::Rng,
    /// Biome manager for handling different biome types
    pub biome_manager: BiomeManager,
    /// Generation algorithms
    pub algorithms: Vec<GenerationAlgorithm>,
}

/// Different level generation algorithms
#[derive(Debug, Clone, PartialEq)]
pub enum GenerationAlgorithm {
    /// Simple room-based generation
    RoomBased,
    /// Cellular automata for cave-like levels
    CellularAutomata,
    /// BSP (Binary Space Partitioning) for structured levels
    BSP,
    /// Maze generation for complex layouts
    Maze,
    /// Hybrid approach combining multiple methods
    Hybrid,
}

/// Parameters for specific generation algorithms
#[derive(Debug, Clone)]
pub struct AlgorithmParams {
    /// For cellular automata
    pub birth_limit: u32,
    pub death_limit: u32,
    pub iterations: u32,
    /// For BSP
    pub min_room_size: u32,
    pub max_room_size: u32,
    /// For maze
    pub maze_width: u32,
    pub maze_height: u32,
    /// For hybrid
    pub primary_algorithm: GenerationAlgorithm,
    pub secondary_algorithm: GenerationAlgorithm,
    pub blend_ratio: f32,
}

impl AdvancedLevelGenerator {
    /// Create a new advanced level generator
    pub fn new() -> Self {
        Self {
            rng: fastrand::Rng::new(),
            biome_manager: BiomeManager::new(),
            algorithms: vec![
                GenerationAlgorithm::RoomBased,
                GenerationAlgorithm::CellularAutomata,
                GenerationAlgorithm::BSP,
                GenerationAlgorithm::Maze,
                GenerationAlgorithm::Hybrid,
            ],
        }
    }

    /// Generate a level using a specific algorithm
    pub fn generate_level(&mut self, config: LevelConfig, algorithm: GenerationAlgorithm, params: Option<AlgorithmParams>) -> Result<Level, String> {
        // Set seed if provided
        if let Some(seed) = config.seed {
            self.rng = fastrand::Rng::with_seed(seed);
        }

        match algorithm {
            GenerationAlgorithm::RoomBased => self.generate_room_based(config),
            GenerationAlgorithm::CellularAutomata => self.generate_cellular_automata(config, params),
            GenerationAlgorithm::BSP => self.generate_bsp(config, params),
            GenerationAlgorithm::Maze => self.generate_maze(config, params),
            GenerationAlgorithm::Hybrid => self.generate_hybrid(config, params),
        }
    }

    /// Generate a level using room-based algorithm
    fn generate_room_based(&mut self, config: LevelConfig) -> Result<Level, String> {
        let mut level = Level {
            id: format!("room_level_{}", self.rng.u64(..)),
            width: config.level_width,
            height: config.level_height,
            tiles: vec![vec![Tile::Empty; config.level_height as usize]; config.level_width as usize],
            rooms: Vec::new(),
            connections: Vec::new(),
            spawn_points: Vec::new(),
            biome: config.biome_type.clone(),
            difficulty: config.difficulty,
        };

        // Generate rooms
        self.generate_rooms_advanced(&mut level, &config)?;
        
        // Connect rooms
        self.connect_rooms_advanced(&mut level)?;
        
        // Fill with tiles
        self.fill_level_advanced(&mut level)?;
        
        // Add spawn points
        self.add_spawn_points_advanced(&mut level)?;

        Ok(level)
    }

    /// Generate a level using cellular automata
    fn generate_cellular_automata(&mut self, config: LevelConfig, params: Option<AlgorithmParams>) -> Result<Level, String> {
        let params = params.unwrap_or_else(|| AlgorithmParams {
            birth_limit: 4,
            death_limit: 3,
            iterations: 5,
            min_room_size: 5,
            max_room_size: 15,
            maze_width: 20,
            maze_height: 20,
            primary_algorithm: GenerationAlgorithm::CellularAutomata,
            secondary_algorithm: GenerationAlgorithm::RoomBased,
            blend_ratio: 0.5,
        });

        let mut level = Level {
            id: format!("ca_level_{}", self.rng.u64(..)),
            width: config.level_width,
            height: config.level_height,
            tiles: vec![vec![Tile::Empty; config.level_height as usize]; config.level_width as usize],
            rooms: Vec::new(),
            connections: Vec::new(),
            spawn_points: Vec::new(),
            biome: config.biome_type.clone(),
            difficulty: config.difficulty,
        };

        // Initialize with random noise
        self.initialize_cellular_automata(&mut level)?;
        
        // Apply cellular automata rules
        for _ in 0..params.iterations {
            self.apply_cellular_automata_rules(&mut level, params.birth_limit, params.death_limit)?;
        }
        
        // Clean up isolated areas
        self.cleanup_cellular_automata(&mut level)?;
        
        // Add spawn points
        self.add_spawn_points_advanced(&mut level)?;

        Ok(level)
    }

    /// Generate a level using BSP (Binary Space Partitioning)
    fn generate_bsp(&mut self, config: LevelConfig, params: Option<AlgorithmParams>) -> Result<Level, String> {
        let params = params.unwrap_or_else(|| AlgorithmParams {
            birth_limit: 4,
            death_limit: 3,
            iterations: 5,
            min_room_size: 5,
            max_room_size: 15,
            maze_width: 20,
            maze_height: 20,
            primary_algorithm: GenerationAlgorithm::BSP,
            secondary_algorithm: GenerationAlgorithm::RoomBased,
            blend_ratio: 0.5,
        });

        let mut level = Level {
            id: format!("bsp_level_{}", self.rng.u64(..)),
            width: config.level_width,
            height: config.level_height,
            tiles: vec![vec![Tile::Empty; config.level_height as usize]; config.level_width as usize],
            rooms: Vec::new(),
            connections: Vec::new(),
            spawn_points: Vec::new(),
            biome: config.biome_type.clone(),
            difficulty: config.difficulty,
        };

        // Create BSP tree
        let bsp_tree = self.create_bsp_tree(0, 0, config.level_width, config.level_height, params.min_room_size, params.max_room_size)?;
        
        // Extract rooms from BSP tree
        self.extract_rooms_from_bsp(&mut level, &bsp_tree)?;
        
        // Connect rooms
        self.connect_rooms_advanced(&mut level)?;
        
        // Fill with tiles
        self.fill_level_advanced(&mut level)?;
        
        // Add spawn points
        self.add_spawn_points_advanced(&mut level)?;

        Ok(level)
    }

    /// Generate a level using maze algorithm
    fn generate_maze(&mut self, config: LevelConfig, params: Option<AlgorithmParams>) -> Result<Level, String> {
        let params = params.unwrap_or_else(|| AlgorithmParams {
            birth_limit: 4,
            death_limit: 3,
            iterations: 5,
            min_room_size: 5,
            max_room_size: 15,
            maze_width: 20,
            maze_height: 20,
            primary_algorithm: GenerationAlgorithm::Maze,
            secondary_algorithm: GenerationAlgorithm::RoomBased,
            blend_ratio: 0.5,
        });

        let mut level = Level {
            id: format!("maze_level_{}", self.rng.u64(..)),
            width: config.level_width,
            height: config.level_height,
            tiles: vec![vec![Tile::Empty; config.level_height as usize]; config.level_width as usize],
            rooms: Vec::new(),
            connections: Vec::new(),
            spawn_points: Vec::new(),
            biome: config.biome_type.clone(),
            difficulty: config.difficulty,
        };

        // Generate maze
        self.generate_maze_recursive_backtrack(&mut level, params.maze_width, params.maze_height)?;
        
        // Add some rooms to the maze
        self.add_rooms_to_maze(&mut level)?;
        
        // Add spawn points
        self.add_spawn_points_advanced(&mut level)?;

        Ok(level)
    }

    /// Generate a level using hybrid approach
    fn generate_hybrid(&mut self, config: LevelConfig, params: Option<AlgorithmParams>) -> Result<Level, String> {
        let params = params.unwrap_or_else(|| AlgorithmParams {
            birth_limit: 4,
            death_limit: 3,
            iterations: 5,
            min_room_size: 5,
            max_room_size: 15,
            maze_width: 20,
            maze_height: 20,
            primary_algorithm: GenerationAlgorithm::RoomBased,
            secondary_algorithm: GenerationAlgorithm::CellularAutomata,
            blend_ratio: 0.5,
        });

        // Generate two levels with different algorithms
        let level1 = self.generate_level(config.clone(), params.primary_algorithm.clone(), Some(params.clone()))?;
        let level2 = self.generate_level(config.clone(), params.secondary_algorithm.clone(), Some(params.clone()))?;

        // Blend the two levels
        self.blend_levels(level1, level2, params.blend_ratio)
    }

    /// Advanced room generation
    fn generate_rooms_advanced(&mut self, level: &mut Level, config: &LevelConfig) -> Result<(), String> {
        let room_count = self.rng.u32(config.room_count_range.0..=config.room_count_range.1);
        
        for i in 0..room_count {
            let room = self.create_room_advanced(i, level, config)?;
            level.rooms.push(room);
        }

        Ok(())
    }

    /// Create an advanced room with special types
    fn create_room_advanced(&mut self, index: u32, level: &Level, config: &LevelConfig) -> Result<Room, String> {
        let width = self.rng.u32(config.room_size_range.0..=config.room_size_range.1);
        let height = self.rng.u32(config.room_size_range.0..=config.room_size_range.1);
        
        // Determine room type
        let room_type = if index == 0 {
            RoomType::Start
        } else if index == level.rooms.len() as u32 {
            RoomType::Boss
        } else {
            match self.rng.u32(0..100) {
                0..=5 => RoomType::Treasure,
                6..=10 => RoomType::Shop,
                11..=15 => RoomType::Empty,
                _ => RoomType::Standard,
            }
        };
        
        // Try to place room
        let mut attempts = 0;
        let max_attempts = 100;
        
        while attempts < max_attempts {
            let x = self.rng.u32(1..level.width - width - 1);
            let y = self.rng.u32(1..level.height - height - 1);
            
            let room = Room {
                id: format!("room_{}", index),
                x,
                y,
                width,
                height,
                room_type: room_type,
                connections: Vec::new(),
            };
            
            if !self.room_overlaps_advanced(&room, &level.rooms) {
                return Ok(room);
            }
            
            attempts += 1;
        }
        
        Err("Failed to place room after maximum attempts".to_string())
    }

    /// Check room overlaps with advanced logic
    fn room_overlaps_advanced(&self, room: &Room, existing_rooms: &[Room]) -> bool {
        for existing_room in existing_rooms {
            // Add some padding between rooms
            let padding = 2;
            if room.x < existing_room.x + existing_room.width + padding &&
               room.x + room.width + padding > existing_room.x &&
               room.y < existing_room.y + existing_room.height + padding &&
               room.y + room.height + padding > existing_room.y {
                return true;
            }
        }
        false
    }

    /// Advanced room connection
    fn connect_rooms_advanced(&mut self, level: &mut Level) -> Result<(), String> {
        if level.rooms.len() < 2 {
            return Ok(());
        }

        // Create minimum spanning tree for connections
        let mut connections: Vec<Connection> = Vec::new();
        let mut connected_rooms = HashSet::new();
        connected_rooms.insert(0); // Start with first room

        while connected_rooms.len() < level.rooms.len() {
            let mut best_connection = None;
            let mut best_distance = f32::INFINITY;

            for &room1 in &connected_rooms {
                for (room2_idx, room2) in level.rooms.iter().enumerate() {
                    if connected_rooms.contains(&room2_idx) {
                        continue;
                    }

                    let distance = level.rooms[room1].distance_to(room2);
                    if distance < best_distance {
                        best_distance = distance;
                        best_connection = Some((room1, room2_idx));
                    }
                }
            }

            if let Some((room1_idx, room2_idx)) = best_connection {
                let connection = if room1_idx < room2_idx {
                    let (left, right) = level.rooms.split_at_mut(room1_idx + 1);
                    self.connect_two_rooms_advanced_simple(&mut left[room1_idx], &mut right[room2_idx - room1_idx - 1])
                } else {
                    let (left, right) = level.rooms.split_at_mut(room2_idx + 1);
                    self.connect_two_rooms_advanced_simple(&mut right[room1_idx - room2_idx - 1], &mut left[room2_idx])
                };
                level.connections.push(connection);
                connected_rooms.insert(room2_idx);
            }
        }

        // Add some extra connections for variety
        let extra_connections = self.rng.u32(0..level.rooms.len() as u32 / 2);
        for _ in 0..extra_connections {
            let room1_idx = self.rng.usize(0..level.rooms.len());
            let room2_idx = self.rng.usize(0..level.rooms.len());
            
            if room1_idx != room2_idx && !level.rooms[room1_idx].is_connected_to(&level.rooms[room2_idx].id) {
                let connection = if room1_idx < room2_idx {
                    let (left, right) = level.rooms.split_at_mut(room1_idx + 1);
                    self.connect_two_rooms_advanced_simple(&mut left[room1_idx], &mut right[room2_idx - room1_idx - 1])
                } else {
                    let (left, right) = level.rooms.split_at_mut(room2_idx + 1);
                    self.connect_two_rooms_advanced_simple(&mut right[room1_idx - room2_idx - 1], &mut left[room2_idx])
                };
                level.connections.push(connection);
            }
        }

        Ok(())
    }

    /// Connect two rooms with advanced corridor generation
    fn connect_two_rooms_advanced(&mut self, room1: &mut Room, room2: &mut Room, level: &mut Level) -> Result<(), String> {
        let center1 = room1.center();
        let center2 = room2.center();

        // Create L-shaped corridor
        let path = self.create_corridor_path_advanced(center1, center2);
        
        let connection = Connection {
            from_room: room1.id.clone(),
            to_room: room2.id.clone(),
            path,
        };

        room1.add_connection(room2.id.clone());
        room2.add_connection(room1.id.clone());
        level.connections.push(connection);

        Ok(())
    }

    /// Connect two rooms with advanced corridor generation (without level parameter to avoid borrowing conflicts)
    fn connect_two_rooms_advanced_simple(&mut self, room1: &mut Room, room2: &mut Room) -> Connection {
        let center1 = room1.center();
        let center2 = room2.center();

        // Create L-shaped corridor
        let path = self.create_corridor_path_advanced(center1, center2);
        
        let connection = Connection {
            from_room: room1.id.clone(),
            to_room: room2.id.clone(),
            path,
        };

        room1.add_connection(room2.id.clone());
        room2.add_connection(room1.id.clone());

        connection
    }

    /// Create advanced corridor path
    fn create_corridor_path_advanced(&mut self, from: Vec2, to: Vec2) -> Vec<Vec2> {
        let mut path = Vec::new();
        
        // Choose between L-shaped and direct path
        if self.rng.bool() {
            // L-shaped path
            let mid_x = from.x;
            let mid_y = to.y;
            
            // Horizontal segment
            let start_x = from.x.min(mid_x);
            let end_x = from.x.max(mid_x);
            let mut x = start_x;
            while x <= end_x {
                path.push(Vec2::new(x, from.y));
                x += 1.0;
            }
            
            // Vertical segment
            let start_y = from.y.min(mid_y);
            let end_y = from.y.max(mid_y);
            let mut y = start_y;
            while y <= end_y {
                path.push(Vec2::new(mid_x, y));
                y += 1.0;
            }
        } else {
            // Direct path with some randomness
            let steps = ((to.x - from.x).abs().max((to.y - from.y).abs()) as u32).max(1);
            let step_x = (to.x - from.x) / steps as f32;
            let step_y = (to.y - from.y) / steps as f32;
            
            for i in 0..=steps {
                let x = from.x + step_x * i as f32;
                let y = from.y + step_y * i as f32;
                path.push(Vec2::new(x, y));
            }
        }
        
        path
    }

    /// Fill level with advanced tile placement
    fn fill_level_advanced(&mut self, level: &mut Level) -> Result<(), String> {
        // Get biome information
        let biome = self.biome_manager.get_biome(&level.biome)
            .ok_or_else(|| format!("Unknown biome: {}", level.biome))?;

        // Fill rooms with biome-appropriate tiles
        for room in &level.rooms {
            let floor_tile = match room.room_type {
                RoomType::Start => biome.floor_tile,
                RoomType::Boss => {
                    // Boss rooms get special treatment based on biome
                    match level.biome.as_str() {
                        "lava" => Tile::Lava,
                        "arctic" => Tile::Ice,
                        "desert" => Tile::Sand,
                        _ => biome.floor_tile,
                    }
                },
                RoomType::Treasure => {
                    // Treasure rooms get special materials
                    match level.biome.as_str() {
                        "industrial" => Tile::Metal,
                        "cave" => Tile::Stone,
                        _ => biome.floor_tile,
                    }
                },
                RoomType::Shop => {
                    // Shop rooms get comfortable materials
                    match level.biome.as_str() {
                        "forest" => Tile::Wood,
                        "desert" => Tile::Sand,
                        _ => biome.floor_tile,
                    }
                },
                RoomType::Empty => biome.floor_tile,
                RoomType::Standard => biome.floor_tile,
            };

            for x in room.x..room.x + room.width {
                for y in room.y..room.y + room.height {
                    if x < level.width && y < level.height {
                        level.tiles[x as usize][y as usize] = floor_tile;
                    }
                }
            }
        }

        // Fill connections with floor tiles
        for connection in &level.connections {
            for point in &connection.path {
                let x = point.x as u32;
                let y = point.y as u32;
                if x < level.width && y < level.height {
                    level.tiles[x as usize][y as usize] = biome.floor_tile;
                }
            }
        }

        // Add walls
        self.add_walls_advanced(level);

        // Add decorative elements
        self.add_decorative_elements(level);

        Ok(())
    }

    /// Add walls with advanced logic
    fn add_walls_advanced(&mut self, level: &mut Level) {
        // Get biome information for wall tiles
        let biome = self.biome_manager.get_biome(&level.biome);
        let wall_tile = biome.map(|b| b.wall_tile).unwrap_or(Tile::Wall);

        for x in 0..level.width as usize {
            for y in 0..level.height as usize {
                if level.tiles[x][y] == Tile::Empty {
                    let mut adjacent_to_floor = false;
                    let mut wall_count = 0;

                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 { continue; }
                            
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            
                            if nx >= 0 && ny >= 0 && 
                               nx < level.width as i32 && ny < level.height as i32 {
                                let tile = level.tiles[nx as usize][ny as usize];
                                if tile.is_walkable() {
                                    adjacent_to_floor = true;
                                } else if tile == Tile::Wall {
                                    wall_count += 1;
                                }
                            }
                        }
                    }
                    
                    if adjacent_to_floor {
                        level.tiles[x][y] = wall_tile;
                    }
                }
            }
        }
    }

    /// Add decorative elements to the level
    fn add_decorative_elements(&mut self, level: &mut Level) {
        // Get biome information for decorative tiles
        let biome = self.biome_manager.get_biome(&level.biome);
        
        for room in &level.rooms {
            if room.room_type == RoomType::Standard || room.room_type == RoomType::Empty {
                // Add some decorative tiles based on biome
                let decoration_count = self.rng.u32(0..3);
                for _ in 0..decoration_count {
                    let x = room.x + self.rng.u32(1..room.width - 1);
                    let y = room.y + self.rng.u32(1..room.height - 1);
                    
                    if x < level.width && y < level.height {
                        let decorative_tile = if let Some(biome) = biome {
                            biome.random_decorative_tile(&mut self.rng).unwrap_or(biome.floor_tile)
                        } else {
                            Tile::Pillar
                        };
                        level.tiles[x as usize][y as usize] = decorative_tile;
                    }
                }
            }
        }
    }

    /// Add spawn points with advanced logic and difficulty scaling
    fn add_spawn_points_advanced(&mut self, level: &mut Level) -> Result<(), String> {
        // Get biome information for spawn modifiers
        let biome = self.biome_manager.get_biome(&level.biome);
        let difficulty_modifier = biome.map(|b| b.difficulty_modifier).unwrap_or(1.0);
        
        for (i, room) in level.rooms.iter().enumerate() {
            match room.room_type {
                RoomType::Start => {
                    // Player spawn point
                    let center_x = room.x + room.width / 2;
                    let center_y = room.y + room.height / 2;
                    level.spawn_points.push(SpawnPoint::new(
                        center_x as f32,
                        center_y as f32,
                        SpawnType::Player,
                    ));
                },
                RoomType::Boss => {
                    // Boss spawn point
                    let center_x = room.x + room.width / 2;
                    let center_y = room.y + room.height / 2;
                    level.spawn_points.push(SpawnPoint::new(
                        center_x as f32,
                        center_y as f32,
                        SpawnType::Boss,
                    ));
                },
                RoomType::Shop => {
                    // NPC spawn point
                    let center_x = room.x + room.width / 2;
                    let center_y = room.y + room.height / 2;
                    level.spawn_points.push(SpawnPoint::new(
                        center_x as f32,
                        center_y as f32,
                        SpawnType::NPC,
                    ));
                },
                _ => {
                    // Enemy and item spawn points with difficulty scaling
                    let base_enemy_count = (level.difficulty as f32 * difficulty_modifier) as u32;
                    let enemy_count = self.rng.u32(1..=base_enemy_count.max(1).min(6));
                    
                    for _ in 0..enemy_count {
                        let x = room.x + self.rng.u32(1..room.width - 1);
                        let y = room.y + self.rng.u32(1..room.height - 1);
                        level.spawn_points.push(SpawnPoint::new(
                            x as f32,
                            y as f32,
                            SpawnType::Enemy,
                        ));
                    }

                    // Item spawn probability increases with difficulty
                    let item_spawn_chance = 0.3 + (level.difficulty as f32 * 0.1).min(0.7);
                    if self.rng.f32() < item_spawn_chance {
                        let x = room.x + self.rng.u32(1..room.width - 1);
                        let y = room.y + self.rng.u32(1..room.height - 1);
                        level.spawn_points.push(SpawnPoint::new(
                            x as f32,
                            y as f32,
                            SpawnType::Item,
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    // Placeholder implementations for other algorithms
    fn initialize_cellular_automata(&mut self, _level: &mut Level) -> Result<(), String> {
        // TODO: Implement cellular automata initialization
        Ok(())
    }

    fn apply_cellular_automata_rules(&mut self, _level: &mut Level, _birth_limit: u32, _death_limit: u32) -> Result<(), String> {
        // TODO: Implement cellular automata rules
        Ok(())
    }

    fn cleanup_cellular_automata(&mut self, _level: &mut Level) -> Result<(), String> {
        // TODO: Implement cellular automata cleanup
        Ok(())
    }

    fn create_bsp_tree(&mut self, _x: u32, _y: u32, _width: u32, _height: u32, _min_size: u32, _max_size: u32) -> Result<BSPNode, String> {
        // TODO: Implement BSP tree creation
        Err("BSP not implemented yet".to_string())
    }

    fn extract_rooms_from_bsp(&mut self, _level: &mut Level, _bsp_tree: &BSPNode) -> Result<(), String> {
        // TODO: Implement room extraction from BSP
        Ok(())
    }

    fn generate_maze_recursive_backtrack(&mut self, _level: &mut Level, _width: u32, _height: u32) -> Result<(), String> {
        // TODO: Implement maze generation
        Ok(())
    }

    fn add_rooms_to_maze(&mut self, _level: &mut Level) -> Result<(), String> {
        // TODO: Implement room addition to maze
        Ok(())
    }

    fn blend_levels(&mut self, _level1: Level, _level2: Level, _ratio: f32) -> Result<Level, String> {
        // TODO: Implement level blending
        Err("Level blending not implemented yet".to_string())
    }
}

/// BSP tree node for binary space partitioning
#[derive(Debug, Clone)]
pub struct BSPNode {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub left: Option<Box<BSPNode>>,
    pub right: Option<Box<BSPNode>>,
    pub room: Option<Room>,
}

impl BSPNode {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            left: None,
            right: None,
            room: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

use std::collections::HashSet;
