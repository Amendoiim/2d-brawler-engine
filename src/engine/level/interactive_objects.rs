//! Interactive objects system for level generation

use crate::engine::level::{Level, Room, RoomType, Tile, SpawnPoint, SpawnType};
use crate::engine::level::biome::{Biome, BiomeManager};
use glam::Vec2;
use std::collections::HashMap;

/// An interactive object in the level
#[derive(Debug, Clone)]
pub struct InteractiveObject {
    /// Unique identifier
    pub id: String,
    /// Object type
    pub object_type: InteractiveObjectType,
    /// Position in the level
    pub x: f32,
    pub y: f32,
    /// Object state
    pub state: ObjectState,
    /// Health (for destructible objects)
    pub health: f32,
    /// Maximum health
    pub max_health: f32,
    /// Whether the object is active
    pub active: bool,
    /// Object properties
    pub properties: ObjectProperties,
}

/// Types of interactive objects
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InteractiveObjectType {
    /// Destructible wall or barrier
    DestructibleWall,
    /// Switch or lever
    Switch,
    /// Treasure chest
    TreasureChest,
    /// Environmental hazard
    Hazard,
    /// Door that can be opened/closed
    Door,
    /// Pressure plate
    PressurePlate,
    /// Teleporter
    Teleporter,
    /// Secret passage
    SecretPassage,
    /// Trap
    Trap,
    /// Interactive decoration
    Decoration,
}

/// State of an interactive object
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectState {
    /// Object is in its default state
    Default,
    /// Object is activated/triggered
    Activated,
    /// Object is destroyed
    Destroyed,
    /// Object is locked
    Locked,
    /// Object is unlocked
    Unlocked,
    /// Object is hidden
    Hidden,
    /// Object is revealed
    Revealed,
}

/// Properties of an interactive object
#[derive(Debug, Clone)]
pub struct ObjectProperties {
    /// Whether the object blocks movement
    pub blocks_movement: bool,
    /// Whether the object is visible
    pub visible: bool,
    /// Whether the object can be interacted with
    pub interactable: bool,
    /// Whether the object is destructible
    pub destructible: bool,
    /// Whether the object triggers events
    pub triggers_events: bool,
    /// Texture ID for rendering
    pub texture_id: u32,
    /// Color tint
    pub color: [f32; 4],
    /// Interaction range
    pub interaction_range: f32,
    /// Cooldown time between interactions
    pub interaction_cooldown: f32,
}

/// Interactive object manager
pub struct InteractiveObjectManager {
    /// Biome manager for biome-specific objects
    pub biome_manager: BiomeManager,
    /// Object templates
    pub object_templates: HashMap<InteractiveObjectType, ObjectTemplate>,
    /// Spawned objects
    pub objects: Vec<InteractiveObject>,
}

/// Template for creating interactive objects
#[derive(Debug, Clone)]
pub struct ObjectTemplate {
    /// Object type
    pub object_type: InteractiveObjectType,
    /// Default properties
    pub properties: ObjectProperties,
    /// Health settings
    pub health: f32,
    /// Spawn probability in different room types
    pub spawn_probabilities: HashMap<RoomType, f32>,
    /// Biome-specific spawn modifiers
    pub biome_modifiers: HashMap<String, f32>,
}

impl InteractiveObjectManager {
    /// Create a new interactive object manager
    pub fn new() -> Self {
        let mut manager = Self {
            biome_manager: BiomeManager::new(),
            object_templates: HashMap::new(),
            objects: Vec::new(),
        };
        manager.initialize_object_templates();
        manager
    }

    /// Initialize default object templates
    fn initialize_object_templates(&mut self) {
        // Destructible wall template
        let destructible_wall_template = ObjectTemplate {
            object_type: InteractiveObjectType::DestructibleWall,
            properties: ObjectProperties {
                blocks_movement: true,
                visible: true,
                interactable: true,
                destructible: true,
                triggers_events: false,
                texture_id: 2000,
                color: [0.8, 0.6, 0.4, 1.0],
                interaction_range: 1.0,
                interaction_cooldown: 0.0,
            },
            health: 100.0,
            spawn_probabilities: {
                let mut probs = HashMap::new();
                probs.insert(RoomType::Standard, 0.3);
                probs.insert(RoomType::Boss, 0.5);
                probs.insert(RoomType::Empty, 0.1);
                probs
            },
            biome_modifiers: {
                let mut modifiers = HashMap::new();
                modifiers.insert("industrial".to_string(), 1.5);
                modifiers.insert("cave".to_string(), 1.2);
                modifiers.insert("forest".to_string(), 0.8);
                modifiers
            },
        };
        self.object_templates.insert(InteractiveObjectType::DestructibleWall, destructible_wall_template);

        // Switch template
        let switch_template = ObjectTemplate {
            object_type: InteractiveObjectType::Switch,
            properties: ObjectProperties {
                blocks_movement: false,
                visible: true,
                interactable: true,
                destructible: false,
                triggers_events: true,
                texture_id: 2001,
                color: [0.9, 0.9, 0.1, 1.0],
                interaction_range: 1.5,
                interaction_cooldown: 1.0,
            },
            health: 0.0,
            spawn_probabilities: {
                let mut probs = HashMap::new();
                probs.insert(RoomType::Standard, 0.2);
                probs.insert(RoomType::Boss, 0.4);
                probs.insert(RoomType::Puzzle, 0.8);
                probs
            },
            biome_modifiers: {
                let mut modifiers = HashMap::new();
                modifiers.insert("industrial".to_string(), 1.8);
                modifiers.insert("cave".to_string(), 1.3);
                modifiers.insert("desert".to_string(), 0.5);
                modifiers
            },
        };
        self.object_templates.insert(InteractiveObjectType::Switch, switch_template);

        // Treasure chest template
        let treasure_chest_template = ObjectTemplate {
            object_type: InteractiveObjectType::TreasureChest,
            properties: ObjectProperties {
                blocks_movement: false,
                visible: true,
                interactable: true,
                destructible: true,
                triggers_events: true,
                texture_id: 2002,
                color: [0.8, 0.6, 0.2, 1.0],
                interaction_range: 1.0,
                interaction_cooldown: 0.0,
            },
            health: 50.0,
            spawn_probabilities: {
                let mut probs = HashMap::new();
                probs.insert(RoomType::Treasure, 1.0);
                probs.insert(RoomType::Standard, 0.1);
                probs.insert(RoomType::Boss, 0.3);
                probs
            },
            biome_modifiers: {
                let mut modifiers = HashMap::new();
                modifiers.insert("cave".to_string(), 1.5);
                modifiers.insert("desert".to_string(), 1.2);
                modifiers.insert("arctic".to_string(), 0.8);
                modifiers
            },
        };
        self.object_templates.insert(InteractiveObjectType::TreasureChest, treasure_chest_template);

        // Hazard template
        let hazard_template = ObjectTemplate {
            object_type: InteractiveObjectType::Hazard,
            properties: ObjectProperties {
                blocks_movement: false,
                visible: true,
                interactable: false,
                destructible: false,
                triggers_events: true,
                texture_id: 2003,
                color: [1.0, 0.2, 0.2, 1.0],
                interaction_range: 0.0,
                interaction_cooldown: 0.0,
            },
            health: 0.0,
            spawn_probabilities: {
                let mut probs = HashMap::new();
                probs.insert(RoomType::Standard, 0.2);
                probs.insert(RoomType::Boss, 0.4);
                probs.insert(RoomType::Empty, 0.1);
                probs
            },
            biome_modifiers: {
                let mut modifiers = HashMap::new();
                modifiers.insert("lava".to_string(), 2.0);
                modifiers.insert("arctic".to_string(), 1.5);
                modifiers.insert("desert".to_string(), 1.2);
                modifiers
            },
        };
        self.object_templates.insert(InteractiveObjectType::Hazard, hazard_template);

        // Door template
        let door_template = ObjectTemplate {
            object_type: InteractiveObjectType::Door,
            properties: ObjectProperties {
                blocks_movement: true,
                visible: true,
                interactable: true,
                destructible: true,
                triggers_events: true,
                texture_id: 2004,
                color: [0.6, 0.4, 0.2, 1.0],
                interaction_range: 1.0,
                interaction_cooldown: 0.5,
            },
            health: 75.0,
            spawn_probabilities: {
                let mut probs = HashMap::new();
                probs.insert(RoomType::Standard, 0.4);
                probs.insert(RoomType::Boss, 0.6);
                probs.insert(RoomType::Shop, 0.8);
                probs
            },
            biome_modifiers: {
                let mut modifiers = HashMap::new();
                modifiers.insert("industrial".to_string(), 1.3);
                modifiers.insert("cave".to_string(), 1.1);
                modifiers.insert("forest".to_string(), 0.9);
                modifiers
            },
        };
        self.object_templates.insert(InteractiveObjectType::Door, door_template);
    }

    /// Add interactive objects to a level
    pub fn add_objects_to_level(&mut self, level: &mut Level) -> Result<(), String> {
        self.objects.clear();

        for room in &level.rooms {
            self.add_objects_to_room(level, room)?;
        }

        Ok(())
    }

    /// Add interactive objects to a specific room
    fn add_objects_to_room(&mut self, level: &mut Level, room: &Room) -> Result<(), String> {
        let biome = self.biome_manager.get_biome(&level.biome);
        let biome_modifier = biome.map(|b| b.difficulty_modifier).unwrap_or(1.0);

        for (object_type, template) in &self.object_templates {
            // Check if this object type should spawn in this room type
            let base_probability = template.spawn_probabilities.get(&room.room_type).copied().unwrap_or(0.0);
            if base_probability <= 0.0 {
                continue;
            }

            // Apply biome modifier
            let biome_modifier_value = template.biome_modifiers.get(&level.biome).copied().unwrap_or(1.0);
            let final_probability = base_probability * biome_modifier_value * biome_modifier;

            // Determine number of objects to spawn
            let object_count = if final_probability >= 1.0 {
                1
            } else if final_probability > 0.5 {
                if fastrand::f32() < final_probability { 1 } else { 0 }
            } else {
                0
            };

            for _ in 0..object_count {
                let object = self.create_object_in_room(level, room, *object_type, template)?;
                self.objects.push(object);
            }
        }

        Ok(())
    }

    /// Create an interactive object in a room
    fn create_object_in_room(
        &self,
        level: &Level,
        room: &Room,
        object_type: InteractiveObjectType,
        template: &ObjectTemplate,
    ) -> Result<InteractiveObject, String> {
        // Find a suitable position in the room
        let mut attempts = 0;
        let max_attempts = 50;

        while attempts < max_attempts {
            let x = room.x as f32 + fastrand::f32() * (room.width - 2) as f32 + 1.0;
            let y = room.y as f32 + fastrand::f32() * (room.height - 2) as f32 + 1.0;

            // Check if position is valid
            if self.is_valid_object_position(level, x, y, object_type) {
                return Ok(InteractiveObject {
                    id: format!("{}_{}_{}", 
                               format!("{:?}", object_type).to_lowercase(),
                               room.id,
                               self.objects.len()),
                    object_type,
                    x,
                    y,
                    state: ObjectState::Default,
                    health: template.health,
                    max_health: template.health,
                    active: true,
                    properties: template.properties.clone(),
                });
            }

            attempts += 1;
        }

        Err(format!("Failed to place object {:?} in room {}", object_type, room.id))
    }

    /// Check if a position is valid for placing an object
    fn is_valid_object_position(&self, level: &Level, x: f32, y: f32, object_type: InteractiveObjectType) -> bool {
        let tile_x = x as u32;
        let tile_y = y as u32;

        // Check bounds
        if tile_x >= level.width || tile_y >= level.height {
            return false;
        }

        let tile = level.tiles[tile_x as usize][tile_y as usize];

        // Check if tile is walkable (for non-blocking objects) or solid (for blocking objects)
        match object_type {
            InteractiveObjectType::DestructibleWall | InteractiveObjectType::Door => {
                // These objects can be placed on walkable tiles
                tile.is_walkable()
            },
            InteractiveObjectType::Switch | InteractiveObjectType::TreasureChest | 
            InteractiveObjectType::Hazard | InteractiveObjectType::PressurePlate => {
                // These objects need walkable tiles
                tile.is_walkable()
            },
            _ => tile.is_walkable(),
        }
    }

    /// Get all objects in a level
    pub fn get_objects(&self) -> &[InteractiveObject] {
        &self.objects
    }

    /// Get objects of a specific type
    pub fn get_objects_by_type(&self, object_type: InteractiveObjectType) -> Vec<&InteractiveObject> {
        self.objects.iter().filter(|obj| obj.object_type == object_type).collect()
    }

    /// Get objects in a specific area
    pub fn get_objects_in_area(&self, x: f32, y: f32, radius: f32) -> Vec<&InteractiveObject> {
        self.objects.iter()
            .filter(|obj| {
                let distance = Vec2::new(obj.x - x, obj.y - y).length();
                distance <= radius
            })
            .collect()
    }

    /// Update an object's state
    pub fn update_object_state(&mut self, object_id: &str, new_state: ObjectState) -> Result<(), String> {
        if let Some(object) = self.objects.iter_mut().find(|obj| obj.id == object_id) {
            object.state = new_state;
            Ok(())
        } else {
            Err(format!("Object with ID {} not found", object_id))
        }
    }

    /// Damage an object
    pub fn damage_object(&mut self, object_id: &str, damage: f32) -> Result<bool, String> {
        if let Some(object) = self.objects.iter_mut().find(|obj| obj.id == object_id) {
            if object.properties.destructible {
                object.health -= damage;
                if object.health <= 0.0 {
                    object.state = ObjectState::Destroyed;
                    object.active = false;
                    return Ok(true); // Object destroyed
                }
            }
            Ok(false) // Object damaged but not destroyed
        } else {
            Err(format!("Object with ID {} not found", object_id))
        }
    }

    /// Get object templates
    pub fn get_object_templates(&self) -> &HashMap<InteractiveObjectType, ObjectTemplate> {
        &self.object_templates
    }

    /// Add a custom object template
    pub fn add_object_template(&mut self, template: ObjectTemplate) {
        self.object_templates.insert(template.object_type, template);
    }
}

impl Default for InteractiveObjectManager {
    fn default() -> Self {
        Self::new()
    }
}
