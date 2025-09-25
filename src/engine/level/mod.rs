//! Level generation system for procedural content creation

pub mod generator;
pub mod room;
pub mod tile;
pub mod biome;
pub mod biome_transition;
pub mod interactive_objects;
pub mod background_layers;
pub mod atmospheric_effects;
pub mod level_types;
pub mod level_progression;
pub mod pathfinding;

pub use generator::*;
pub use room::*;
pub use tile::*;
pub use biome::*;
pub use biome_transition::*;
pub use interactive_objects::*;
pub use background_layers::*;
pub use atmospheric_effects::*;
pub use level_types::{LevelType, LevelTypeConfig, LevelTypeParams, LevelTypeManager, LevelTypeTemplate, RoomPattern, PatternRequirement, TileRule, TileCondition, ObjectRule, ObjectCondition, SpawnRule, SpawnType as LevelSpawnType};
pub use level_progression::{
    LevelProgressionManager, LevelInfo, PlayerProgress, PlayerStats, CheckpointSystem, Checkpoint, 
    CheckpointType, CheckpointData, LevelState, PlayerState, RewardSystem, LevelReward, 
    RewardType, RewardRequirement, RequirementType, RewardTemplate, LevelSelection, 
    LevelCategory, LevelFilters, CompletionFilter
};
pub use pathfinding::*;

use crate::engine::ecs::{Component, Entity, World};
use glam::Vec2;
use std::collections::HashMap;

/// Level generation system for creating procedural levels
pub struct LevelGenerator {
    /// Current level being generated
    pub current_level: Option<Level>,
    /// Level generation parameters
    pub config: LevelConfig,
    /// Random number generator for procedural generation
    pub rng: fastrand::Rng,
    /// Biome types available for generation
    pub biomes: HashMap<String, Biome>,
    /// Advanced level generator for complex algorithms
    pub advanced_generator: generator::AdvancedLevelGenerator,
    /// Biome transition manager for handling biome changes
    pub biome_transition_manager: BiomeTransitionManager,
    /// Interactive object manager for placing interactive elements
    pub interactive_object_manager: InteractiveObjectManager,
    /// Background layer manager for parallax scrolling
    pub background_layer_manager: BackgroundLayerManager,
    /// Atmospheric effects manager for weather and lighting
    pub atmospheric_effects_manager: AtmosphericEffectsManager,
    /// Level type manager for different gameplay experiences
    pub level_type_manager: LevelTypeManager,
    /// Level progression manager for level selection, checkpoints, and rewards
    pub level_progression_manager: LevelProgressionManager,
}

/// A complete generated level
#[derive(Debug, Clone)]
pub struct Level {
    /// Unique identifier for the level
    pub id: String,
    /// Level dimensions in tiles
    pub width: u32,
    pub height: u32,
    /// Grid of tiles representing the level
    pub tiles: Vec<Vec<Tile>>,
    /// Rooms in the level
    pub rooms: Vec<Room>,
    /// Connections between rooms
    pub connections: Vec<Connection>,
    /// Spawn points for entities
    pub spawn_points: Vec<SpawnPoint>,
    /// Level theme/biome
    pub biome: String,
    /// Level difficulty
    pub difficulty: u32,
}

/// Level generation configuration
#[derive(Debug, Clone)]
pub struct LevelConfig {
    /// Minimum and maximum number of rooms
    pub room_count_range: (u32, u32),
    /// Minimum and maximum room dimensions
    pub room_size_range: (u32, u32),
    /// Level dimensions
    pub level_width: u32,
    pub level_height: u32,
    /// Difficulty level
    pub difficulty: u32,
    /// Biome type
    pub biome_type: String,
    /// Seed for random generation
    pub seed: Option<u64>,
}

impl LevelGenerator {
    /// Create a new level generator
    pub fn new() -> Self {
        Self {
            current_level: None,
            config: LevelConfig::default(),
            rng: fastrand::Rng::new(),
            biomes: HashMap::new(),
            advanced_generator: generator::AdvancedLevelGenerator::new(),
            biome_transition_manager: BiomeTransitionManager::new(),
            interactive_object_manager: InteractiveObjectManager::new(),
            background_layer_manager: BackgroundLayerManager::new(),
            atmospheric_effects_manager: AtmosphericEffectsManager::new(),
            level_type_manager: LevelTypeManager::new(),
            level_progression_manager: LevelProgressionManager::new(),
        }
    }

    /// Generate a new level
    pub fn generate_level(&mut self, config: LevelConfig) -> Result<Level, String> {
        // Set the seed if provided
        if let Some(seed) = config.seed {
            self.rng = fastrand::Rng::with_seed(seed);
        }

        self.config = config.clone();

        // Initialize empty level
        let mut level = Level {
            id: format!("level_{}", self.rng.u64(..)),
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
        self.generate_rooms(&mut level)?;

        // Connect rooms
        self.connect_rooms(&mut level)?;

        // Fill level with tiles
        self.fill_level_tiles(&mut level)?;

        // Add spawn points
        self.add_spawn_points(&mut level)?;

        self.current_level = Some(level.clone());
        Ok(level)
    }

    /// Generate rooms for the level
    fn generate_rooms(&mut self, level: &mut Level) -> Result<(), String> {
        let room_count = self.rng.u32(self.config.room_count_range.0..=self.config.room_count_range.1);
        
        for i in 0..room_count {
            let room = self.create_room(i, level)?;
            level.rooms.push(room);
        }

        Ok(())
    }

    /// Create a single room
    fn create_room(&mut self, index: u32, level: &Level) -> Result<Room, String> {
        let width = self.rng.u32(self.config.room_size_range.0..=self.config.room_size_range.1);
        let height = self.rng.u32(self.config.room_size_range.0..=self.config.room_size_range.1);
        
        // Try to find a valid position for the room
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
                room_type: RoomType::Standard,
                connections: Vec::new(),
            };
            
            // Check if room overlaps with existing rooms
            if !self.room_overlaps(&room, &level.rooms) {
                return Ok(room);
            }
            
            attempts += 1;
        }
        
        Err("Failed to place room after maximum attempts".to_string())
    }

    /// Check if a room overlaps with existing rooms
    fn room_overlaps(&self, room: &Room, existing_rooms: &[Room]) -> bool {
        for existing_room in existing_rooms {
            if room.x < existing_room.x + existing_room.width &&
               room.x + room.width > existing_room.x &&
               room.y < existing_room.y + existing_room.height &&
               room.y + room.height > existing_room.y {
                return true;
            }
        }
        false
    }

    /// Connect rooms with corridors
    fn connect_rooms(&mut self, level: &mut Level) -> Result<(), String> {
        if level.rooms.len() < 2 {
            return Ok(());
        }

        // Connect each room to at least one other room
        let mut connections = Vec::new();
        for i in 0..level.rooms.len() {
            if i == 0 {
                // Connect first room to second room
                if level.rooms.len() > 1 {
                    let (left, right) = level.rooms.split_at_mut(1);
                    let connection = self.connect_two_rooms_simple(&mut left[0], &mut right[0]);
                    connections.push(connection);
                }
            } else {
                // Connect to a random previous room
                let target_index = self.rng.usize(0..i);
                if target_index < i {
                    let (left, right) = level.rooms.split_at_mut(i);
                    let connection = self.connect_two_rooms_simple(&mut right[0], &mut left[target_index]);
                    connections.push(connection);
                }
            }
        }
        
        // Add all connections to the level
        level.connections.extend(connections);

        Ok(())
    }

    /// Connect two rooms with a corridor
    fn connect_two_rooms(&mut self, room1: &mut Room, room2: &mut Room, level: &mut Level) -> Result<(), String> {
        // Calculate room centers
        let center1 = Vec2::new(
            room1.x as f32 + room1.width as f32 / 2.0,
            room1.y as f32 + room1.height as f32 / 2.0,
        );
        let center2 = Vec2::new(
            room2.x as f32 + room2.width as f32 / 2.0,
            room2.y as f32 + room2.height as f32 / 2.0,
        );

        // Create connection
        let connection = Connection {
            from_room: room1.id.clone(),
            to_room: room2.id.clone(),
            path: self.create_corridor_path(center1, center2),
        };

        // Add connection to rooms
        room1.connections.push(room2.id.clone());
        room2.connections.push(room1.id.clone());

        level.connections.push(connection);
        Ok(())
    }

    /// Connect two rooms with a corridor (without level parameter to avoid borrowing conflicts)
    fn connect_two_rooms_simple(&mut self, room1: &mut Room, room2: &mut Room) -> Connection {
        // Calculate room centers
        let center1 = Vec2::new(
            room1.x as f32 + room1.width as f32 / 2.0,
            room1.y as f32 + room1.height as f32 / 2.0,
        );
        let center2 = Vec2::new(
            room2.x as f32 + room2.width as f32 / 2.0,
            room2.y as f32 + room2.height as f32 / 2.0,
        );

        // Create connection
        let connection = Connection {
            from_room: room1.id.clone(),
            to_room: room2.id.clone(),
            path: self.create_corridor_path(center1, center2),
        };

        // Add connection to rooms
        room1.connections.push(room2.id.clone());
        room2.connections.push(room1.id.clone());

        connection
    }

    /// Create a corridor path between two points
    fn create_corridor_path(&mut self, from: Vec2, to: Vec2) -> Vec<Vec2> {
        let mut path = Vec::new();
        
        // Simple L-shaped corridor
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
        
        path
    }

    /// Fill the level with tiles based on rooms and connections
    fn fill_level_tiles(&mut self, level: &mut Level) -> Result<(), String> {
        // Fill rooms with floor tiles
        for room in &level.rooms {
            for x in room.x..room.x + room.width {
                for y in room.y..room.y + room.height {
                    if x < level.width && y < level.height {
                        level.tiles[x as usize][y as usize] = Tile::Floor;
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
                    level.tiles[x as usize][y as usize] = Tile::Floor;
                }
            }
        }

        // Add walls around floors
        self.add_walls(level);

        Ok(())
    }

    /// Add walls around floor tiles
    fn add_walls(&mut self, level: &mut Level) {
        for x in 0..level.width as usize {
            for y in 0..level.height as usize {
                if level.tiles[x][y] == Tile::Empty {
                    // Check if adjacent to floor
                    let mut adjacent_to_floor = false;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if nx >= 0 && ny >= 0 && 
                               nx < level.width as i32 && ny < level.height as i32 &&
                               level.tiles[nx as usize][ny as usize] == Tile::Floor {
                                adjacent_to_floor = true;
                                break;
                            }
                        }
                        if adjacent_to_floor { break; }
                    }
                    
                    if adjacent_to_floor {
                        level.tiles[x][y] = Tile::Wall;
                    }
                }
            }
        }
    }

    /// Add spawn points to the level
    fn add_spawn_points(&mut self, level: &mut Level) -> Result<(), String> {
        for room in &level.rooms {
            // Add player spawn point (first room only)
            if room.id == "room_0" {
                let center_x = room.x + room.width / 2;
                let center_y = room.y + room.height / 2;
                level.spawn_points.push(SpawnPoint {
                    x: center_x as f32,
                    y: center_y as f32,
                    spawn_type: SpawnType::Player,
                });
            }

            // Add enemy spawn points
            let enemy_count = self.rng.u32(1..=3);
            for _ in 0..enemy_count {
                let x = room.x + self.rng.u32(1..room.width - 1);
                let y = room.y + self.rng.u32(1..room.height - 1);
                level.spawn_points.push(SpawnPoint {
                    x: x as f32,
                    y: y as f32,
                    spawn_type: SpawnType::Enemy,
                });
            }

            // Add item spawn points
            if self.rng.bool() {
                let x = room.x + self.rng.u32(1..room.width - 1);
                let y = room.y + self.rng.u32(1..room.height - 1);
                level.spawn_points.push(SpawnPoint {
                    x: x as f32,
                    y: y as f32,
                    spawn_type: SpawnType::Item,
                });
            }
        }

        Ok(())
    }

    /// Get the current level
    pub fn get_current_level(&self) -> Option<&Level> {
        self.current_level.as_ref()
    }

    /// Clear the current level
    pub fn clear_level(&mut self) {
        self.current_level = None;
    }

    /// Generate a level using advanced algorithms
    pub fn generate_level_advanced(&mut self, config: LevelConfig, algorithm: generator::GenerationAlgorithm) -> Result<Level, String> {
        let level = self.advanced_generator.generate_level(config, algorithm, None)?;
        self.current_level = Some(level.clone());
        Ok(level)
    }

    /// Generate a multi-biome level with transitions
    pub fn generate_multi_biome_level(&mut self, config: LevelConfig, biomes: Vec<String>) -> Result<Level, String> {
        if biomes.is_empty() {
            return Err("At least one biome must be specified".to_string());
        }

        // Generate base level with first biome
        let mut base_config = config.clone();
        base_config.biome_type = biomes[0].clone();
        
        let mut level = self.advanced_generator.generate_level(base_config, generator::GenerationAlgorithm::RoomBased, None)?;
        
        // Add biome transitions if multiple biomes
        if biomes.len() > 1 {
            self.add_biome_transitions(&mut level, &biomes)?;
        }
        
        self.current_level = Some(level.clone());
        Ok(level)
    }

    /// Add biome transitions to a level
    fn add_biome_transitions(&mut self, level: &mut Level, biomes: &[String]) -> Result<(), String> {
        if biomes.len() < 2 {
            return Ok(());
        }

        // Calculate transition areas
        let transition_width = level.width / (biomes.len() as u32 - 1);
        
        for i in 1..biomes.len() {
            let from_biome = &biomes[i - 1];
            let to_biome = &biomes[i];
            
            let transition_x = (i - 1) as u32 * transition_width;
            let transition_width_actual = if i == biomes.len() - 1 {
                level.width - transition_x
            } else {
                transition_width
            };
            
            // Create transition
            let transition = self.biome_transition_manager.create_transition(
                from_biome,
                to_biome,
                transition_x,
                0,
                transition_width_actual,
                level.height,
                biome_transition::TransitionType::Gradual,
            )?;
            
            // Apply transition
            self.biome_transition_manager.apply_transition(level, &transition)?;
        }
        
        Ok(())
    }

    /// Generate a level with interactive objects
    pub fn generate_level_with_objects(&mut self, config: LevelConfig, algorithm: generator::GenerationAlgorithm) -> Result<Level, String> {
        // Generate base level
        let mut level = self.advanced_generator.generate_level(config, algorithm, None)?;
        
        // Add interactive objects
        self.interactive_object_manager.add_objects_to_level(&mut level)?;
        
        self.current_level = Some(level.clone());
        Ok(level)
    }

    /// Get interactive objects from the current level
    pub fn get_interactive_objects(&self) -> &[InteractiveObject] {
        self.interactive_object_manager.get_objects()
    }

    /// Update an interactive object's state
    pub fn update_interactive_object(&mut self, object_id: &str, new_state: ObjectState) -> Result<(), String> {
        self.interactive_object_manager.update_object_state(object_id, new_state)
    }

    /// Damage an interactive object
    pub fn damage_interactive_object(&mut self, object_id: &str, damage: f32) -> Result<bool, String> {
        self.interactive_object_manager.damage_object(object_id, damage)
    }

    /// Create background layers for a biome
    pub fn create_background_layers(&mut self, biome: &str, screen_width: f32, screen_height: f32) -> Result<(), String> {
        self.background_layer_manager.create_biome_background(biome, screen_width, screen_height)
    }

    /// Update background layers
    pub fn update_background_layers(&mut self, dt: f32) {
        self.background_layer_manager.update_layers(dt);
    }

    /// Set camera position for parallax calculations
    pub fn set_camera_position(&mut self, position: Vec2) {
        self.background_layer_manager.set_camera_position(position);
    }

    /// Set camera velocity for smooth scrolling
    pub fn set_camera_velocity(&mut self, velocity: Vec2) {
        self.background_layer_manager.set_camera_velocity(velocity);
    }

    /// Get background layers
    pub fn get_background_layers(&self) -> &[BackgroundLayer] {
        self.background_layer_manager.get_layers()
    }

    /// Get visible background layers
    pub fn get_visible_background_layers(&self) -> Vec<&BackgroundLayer> {
        self.background_layer_manager.get_visible_layers()
    }

    /// Create atmospheric effects for a biome
    pub fn create_atmospheric_effects(&mut self, biome: &str, level_width: f32, level_height: f32) -> Result<(), String> {
        self.atmospheric_effects_manager.create_biome_effects(biome, level_width, level_height)
    }

    /// Update atmospheric effects
    pub fn update_atmospheric_effects(&mut self, dt: f32) {
        self.atmospheric_effects_manager.update_effects(dt);
    }

    /// Get atmospheric effects
    pub fn get_atmospheric_effects(&self) -> &[AtmosphericEffect] {
        self.atmospheric_effects_manager.get_effects()
    }

    /// Get active atmospheric effects
    pub fn get_active_atmospheric_effects(&self) -> Vec<&AtmosphericEffect> {
        self.atmospheric_effects_manager.get_active_effects()
    }

    /// Get global lighting settings
    pub fn get_global_lighting(&self) -> &GlobalLighting {
        self.atmospheric_effects_manager.get_global_lighting()
    }

    /// Get weather system
    pub fn get_weather_system(&self) -> &WeatherSystem {
        self.atmospheric_effects_manager.get_weather_system()
    }

    /// Set atmospheric effect intensity
    pub fn set_atmospheric_effect_intensity(&mut self, effect_id: &str, intensity: f32) -> Result<(), String> {
        self.atmospheric_effects_manager.set_effect_intensity(effect_id, intensity)
    }

    /// Set atmospheric effect opacity
    pub fn set_atmospheric_effect_opacity(&mut self, effect_id: &str, opacity: f32) -> Result<(), String> {
        self.atmospheric_effects_manager.set_effect_opacity(effect_id, opacity)
    }

    /// Generate a combat arena level
    pub fn generate_combat_arena(&mut self, config: LevelConfig, enemy_spawn_count: u32, cover_count: u32) -> Result<Level, String> {
        let level_type_config = LevelTypeConfig {
            level_type: LevelType::CombatArena,
            base_config: config,
            type_params: LevelTypeParams::CombatArena {
                enemy_spawn_count,
                cover_count,
                size_multiplier: 1.5,
                include_hazards: true,
            },
        };
        let level = self.level_type_manager.generate_level_type(level_type_config)?;
        self.current_level = Some(level.clone());
        Ok(level)
    }

    /// Generate a platforming level
    pub fn generate_platforming_level(&mut self, config: LevelConfig, moving_platforms: u32, static_platforms: u32, hazard_count: u32) -> Result<Level, String> {
        let level_type_config = LevelTypeConfig {
            level_type: LevelType::Platforming,
            base_config: config,
            type_params: LevelTypeParams::Platforming {
                moving_platforms,
                static_platforms,
                hazard_count,
                vertical_challenge: 1.5,
            },
        };
        let level = self.level_type_manager.generate_level_type(level_type_config)?;
        self.current_level = Some(level.clone());
        Ok(level)
    }

    /// Generate a puzzle level
    pub fn generate_puzzle_level(&mut self, config: LevelConfig, switch_count: u32, pressure_plate_count: u32, door_count: u32, complexity: u32) -> Result<Level, String> {
        let level_type_config = LevelTypeConfig {
            level_type: LevelType::Puzzle,
            base_config: config,
            type_params: LevelTypeParams::Puzzle {
                switch_count,
                pressure_plate_count,
                door_count,
                complexity,
            },
        };
        let level = self.level_type_manager.generate_level_type(level_type_config)?;
        self.current_level = Some(level.clone());
        Ok(level)
    }

    /// Generate a boss arena level
    pub fn generate_boss_arena(&mut self, config: LevelConfig, boss_type: String, arena_size: (u32, u32), phase_count: u32) -> Result<Level, String> {
        let level_type_config = LevelTypeConfig {
            level_type: LevelType::BossArena,
            base_config: config,
            type_params: LevelTypeParams::BossArena {
                boss_type,
                arena_size,
                phase_count,
                include_storytelling: true,
            },
        };
        let level = self.level_type_manager.generate_level_type(level_type_config)?;
        self.current_level = Some(level.clone());
        Ok(level)
    }

    /// Generate a standard level
    pub fn generate_standard_level(&mut self, config: LevelConfig, combat_ratio: f32, exploration_focus: f32) -> Result<Level, String> {
        let level_type_config = LevelTypeConfig {
            level_type: LevelType::Standard,
            base_config: config,
            type_params: LevelTypeParams::Standard {
                combat_ratio,
                exploration_focus,
            },
        };
        let level = self.level_type_manager.generate_level_type(level_type_config)?;
        self.current_level = Some(level.clone());
        Ok(level)
    }

    /// Get available level types
    pub fn get_available_level_types(&self) -> Vec<LevelType> {
        self.level_type_manager.get_available_level_types()
    }

    /// Get level type template
    pub fn get_level_type_template(&self, level_type: &LevelType) -> Option<&LevelTypeTemplate> {
        self.level_type_manager.get_template(level_type)
    }

    /// Get level progression manager
    pub fn get_level_progression_manager(&self) -> &LevelProgressionManager {
        &self.level_progression_manager
    }

    /// Get level progression manager (mutable)
    pub fn get_level_progression_manager_mut(&mut self) -> &mut LevelProgressionManager {
        &mut self.level_progression_manager
    }

    /// Complete a level and get rewards
    pub fn complete_level(&mut self, level_id: &str, completion_time: f32, star_rating: u32) -> Result<Vec<LevelReward>, String> {
        self.level_progression_manager.complete_level(level_id, completion_time, star_rating)
    }

    /// Create a checkpoint
    pub fn create_checkpoint(&mut self, checkpoint: Checkpoint) {
        self.level_progression_manager.create_checkpoint(checkpoint);
    }

    /// Reach a checkpoint
    pub fn reach_checkpoint(&mut self, checkpoint_id: &str, player_state: PlayerState) -> Result<(), String> {
        self.level_progression_manager.reach_checkpoint(checkpoint_id, player_state)
    }

    /// Get checkpoint data
    pub fn get_checkpoint_data(&self, checkpoint_id: &str) -> Option<&CheckpointData> {
        self.level_progression_manager.get_checkpoint_data(checkpoint_id)
    }

    /// Get last checkpoint
    pub fn get_last_checkpoint(&self) -> Option<&str> {
        self.level_progression_manager.get_last_checkpoint()
    }

    /// Get player progress
    pub fn get_player_progress(&self) -> &PlayerProgress {
        self.level_progression_manager.get_player_progress()
    }

    /// Add experience to player
    pub fn add_experience(&mut self, amount: u32) {
        self.level_progression_manager.add_experience(amount);
    }

    /// Get filtered levels
    pub fn get_filtered_levels(&self) -> Vec<&LevelInfo> {
        self.level_progression_manager.get_filtered_levels()
    }

    /// Set level filters
    pub fn set_level_filters(&mut self, filters: LevelFilters) {
        self.level_progression_manager.set_level_filters(filters);
    }

    /// Select a level
    pub fn select_level(&mut self, level_id: &str) -> Result<(), String> {
        self.level_progression_manager.select_level(level_id)
    }

    /// Get selected level
    pub fn get_selected_level(&self) -> Option<&str> {
        self.level_progression_manager.get_selected_level()
    }

    /// Claim a reward
    pub fn claim_reward(&mut self, level_id: &str, reward_id: &str) -> Result<(), String> {
        self.level_progression_manager.claim_reward(level_id, reward_id)
    }
}

impl Default for LevelConfig {
    fn default() -> Self {
        Self {
            room_count_range: (3, 8),
            room_size_range: (5, 15),
            level_width: 50,
            level_height: 50,
            difficulty: 1,
            biome_type: "default".to_string(),
            seed: None,
        }
    }
}
