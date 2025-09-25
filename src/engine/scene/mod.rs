//! Scene management system

use anyhow::Result;
use crate::engine::ecs::{World, Entity};
use std::collections::HashMap;

/// Scene manager for handling level loading and scene transitions
pub struct SceneManager {
    current_scene: Option<String>,
    scenes: HashMap<String, Scene>,
    scene_transitions: HashMap<String, String>,
    transition_timer: f32,
    transition_duration: f32,
    is_transitioning: bool,
}

/// Scene data structure
pub struct Scene {
    pub name: String,
    pub entities: Vec<Entity>,
    pub spawn_points: Vec<SpawnPoint>,
    pub background_color: [f32; 4],
    pub gravity: f32,
    pub bounds: SceneBounds,
}

/// Spawn point for entities
pub struct SpawnPoint {
    pub x: f32,
    pub y: f32,
    pub spawn_type: SpawnType,
}

/// Type of entity to spawn
pub enum SpawnType {
    Player,
    Enemy,
    Item,
    Obstacle,
}

/// Scene boundaries
pub struct SceneBounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl SceneManager {
    /// Create a new scene manager
    pub fn new() -> Self {
        Self {
            current_scene: None,
            scenes: HashMap::new(),
            scene_transitions: HashMap::new(),
            transition_timer: 0.0,
            transition_duration: 1.0,
            is_transitioning: false,
        }
    }

    /// Load a scene
    pub fn load_scene(&mut self, name: &str, world: &mut World) -> Result<()> {
        log::info!("Loading scene: {}", name);
        
        // Unload current scene
        let current_scene_name = self.current_scene.clone();
        if let Some(scene_name) = current_scene_name {
            self.unload_scene(&scene_name, world)?;
        }

        // Load new scene
        if let Some(scene) = self.scenes.get(name) {
            self.spawn_scene_entities(scene, world)?;
            self.current_scene = Some(name.to_string());
            log::info!("Successfully loaded scene: {}", name);
        } else {
            return Err(anyhow::anyhow!("Scene not found: {}", name));
        }

        Ok(())
    }

    /// Unload a scene
    fn unload_scene(&mut self, scene_name: &str, world: &mut World) -> Result<()> {
        if let Some(scene) = self.scenes.get(scene_name) {
            log::info!("Unloading scene: {}", scene_name);
            
            // Remove all entities from the scene
            for entity in &scene.entities {
                // Note: In a real implementation, you'd need a way to remove entities
                // from the ECS world. This depends on your ECS implementation.
                log::debug!("Removing entity: {}", entity);
            }
        }
        Ok(())
    }

    /// Spawn entities for a scene
    fn spawn_scene_entities(&self, scene: &Scene, world: &mut World) -> Result<()> {
        log::info!("Spawning entities for scene: {}", scene.name);
        
        for spawn_point in &scene.spawn_points {
            let entity = world.create_entity();
            
            // Add position component
            world.add_component(entity, crate::game::Position {
                x: spawn_point.x,
                y: spawn_point.y,
            });
            
            // Add components based on spawn type
            match spawn_point.spawn_type {
                SpawnType::Player => {
                    world.add_component(entity, crate::game::Player {
                        player_id: 1,
                        input_device: crate::game::InputDevice::Keyboard,
                    });
                    world.add_component(entity, crate::game::Velocity { x: 0.0, y: 0.0 });
                    world.add_component(entity, crate::game::Sprite {
                        texture_id: 1,
                        width: 32.0,
                        height: 32.0,
                        color: [0.0, 1.0, 0.0, 1.0], // Green for player
                        visible: true,
                    });
                    world.add_component(entity, crate::game::Combat {
                        attack_damage: 25.0,
                        attack_range: 50.0,
                        attack_cooldown: 1.0,
                        current_cooldown: 0.0,
                        health: 100.0,
                        max_health: 100.0,
                        is_alive: true,
                    });
                }
                SpawnType::Enemy => {
                    world.add_component(entity, crate::game::Velocity { x: 0.0, y: 0.0 });
                    world.add_component(entity, crate::game::Sprite {
                        texture_id: 2,
                        width: 24.0,
                        height: 24.0,
                        color: [1.0, 0.0, 0.0, 1.0], // Red for enemy
                        visible: true,
                    });
                    world.add_component(entity, crate::game::Combat {
                        attack_damage: 15.0,
                        attack_range: 30.0,
                        attack_cooldown: 2.0,
                        current_cooldown: 0.0,
                        health: 50.0,
                        max_health: 50.0,
                        is_alive: true,
                    });
                }
                SpawnType::Item => {
                    world.add_component(entity, crate::game::Sprite {
                        texture_id: 3,
                        width: 16.0,
                        height: 16.0,
                        color: [1.0, 1.0, 0.0, 1.0], // Yellow for item
                        visible: true,
                    });
                }
                SpawnType::Obstacle => {
                    world.add_component(entity, crate::game::Sprite {
                        texture_id: 4,
                        width: 40.0,
                        height: 40.0,
                        color: [0.5, 0.5, 0.5, 1.0], // Gray for obstacle
                        visible: true,
                    });
                }
            }
            
            log::debug!("Spawned entity {} at ({}, {})", entity, spawn_point.x, spawn_point.y);
        }
        
        Ok(())
    }

    /// Create a test scene
    pub fn create_test_scene(&mut self, name: &str) {
        let scene = Scene {
            name: name.to_string(),
            entities: Vec::new(),
            spawn_points: vec![
                SpawnPoint {
                    x: 100.0,
                    y: 100.0,
                    spawn_type: SpawnType::Player,
                },
                SpawnPoint {
                    x: 300.0,
                    y: 200.0,
                    spawn_type: SpawnType::Enemy,
                },
                SpawnPoint {
                    x: 200.0,
                    y: 150.0,
                    spawn_type: SpawnType::Item,
                },
                SpawnPoint {
                    x: 400.0,
                    y: 300.0,
                    spawn_type: SpawnType::Obstacle,
                },
            ],
            background_color: [0.2, 0.3, 0.4, 1.0], // Dark blue background
            gravity: 980.0,
            bounds: SceneBounds {
                min_x: 0.0,
                max_x: 800.0,
                min_y: 0.0,
                max_y: 600.0,
            },
        };
        
        self.scenes.insert(name.to_string(), scene);
        log::info!("Created test scene: {}", name);
    }

    /// Start a scene transition
    pub fn start_transition(&mut self, target_scene: &str) {
        if let Some(current_scene) = &self.current_scene {
            self.scene_transitions.insert(current_scene.clone(), target_scene.to_string());
            self.is_transitioning = true;
            self.transition_timer = 0.0;
            log::info!("Starting transition from {} to {}", current_scene, target_scene);
        }
    }

    /// Update scene
    pub fn update(&mut self, dt: f32, world: &mut World) {
        // Handle scene transitions
        if self.is_transitioning {
            self.transition_timer += dt;
            
            if self.transition_timer >= self.transition_duration {
                // Complete the transition
                let current_scene = self.current_scene.clone();
                let target_scene = if let Some(scene_name) = current_scene {
                    self.scene_transitions.get(&scene_name).cloned()
                } else {
                    None
                };
                
                if let Some(target) = target_scene {
                    if let Err(e) = self.load_scene(&target, world) {
                        log::error!("Failed to load scene during transition: {}", e);
                    }
                }
                
                self.is_transitioning = false;
                self.transition_timer = 0.0;
                self.scene_transitions.clear();
            }
        }
        
        // Update scene-specific logic
        if let Some(current_scene) = &self.current_scene {
            if let Some(scene) = self.scenes.get(current_scene) {
                // Apply scene gravity
                let velocity_entities = world.query::<crate::game::Velocity>();
                for entity in velocity_entities {
                    if let Some(velocity) = world.get_component_mut::<crate::game::Velocity>(entity) {
                        velocity.y -= scene.gravity * dt;
                    }
                }
                
                // Check scene bounds
                let position_entities = world.query::<crate::game::Position>();
                for entity in position_entities {
                    if let Some(position) = world.get_component_mut::<crate::game::Position>(entity) {
                        // Keep entities within scene bounds
                        position.x = position.x.max(scene.bounds.min_x).min(scene.bounds.max_x);
                        position.y = position.y.max(scene.bounds.min_y).min(scene.bounds.max_y);
                    }
                }
            }
        }
    }

    /// Get current scene name
    pub fn current_scene(&self) -> Option<&String> {
        self.current_scene.as_ref()
    }

    /// Check if currently transitioning
    pub fn is_transitioning(&self) -> bool {
        self.is_transitioning
    }

    /// Get transition progress (0.0 to 1.0)
    pub fn transition_progress(&self) -> f32 {
        if self.is_transitioning {
            (self.transition_timer / self.transition_duration).min(1.0)
        } else {
            0.0
        }
    }
}
