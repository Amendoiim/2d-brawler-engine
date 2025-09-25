//! Game-specific systems and logic

pub mod combat;
pub mod characters;
pub mod levels;
pub mod progression;

use crate::engine::ecs::{World, Component, System, Entity};

// Implement Component trait for all game components
impl Component for Position {}
impl Component for Velocity {}
impl Component for Sprite {}
impl Component for Health {}
impl Component for Player {}
impl Component for Combat {}
impl Component for Attack {}

/// Game state management
pub struct GameState {
    current_level: String,
    player_count: u32,
    score: u32,
    time: f32,
}

impl GameState {
    /// Create a new game state
    pub fn new() -> Self {
        Self {
            current_level: "level_1".to_string(),
            player_count: 1,
            score: 0,
            time: 0.0,
        }
    }

    /// Update game state
    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }
}

/// Position component for entities
#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Velocity component for entities
#[derive(Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// Sprite component for rendering
pub struct Sprite {
    pub texture_id: u32,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4], // RGBA
    pub visible: bool,
}

/// Health component for entities
pub struct Health {
    pub current: f32,
    pub maximum: f32,
}

/// Movement system for updating entity positions
pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // Get all entities with Position component
        let position_entities = world.query::<Position>();
        
        for entity in position_entities {
            // Get velocity and position separately to avoid borrowing conflicts
            let velocity = world.get_component::<Velocity>(entity).cloned();
            if let Some(vel) = velocity {
                if let Some(pos_mut) = world.get_component_mut::<Position>(entity) {
                    pos_mut.x += vel.x * dt;
                    pos_mut.y += vel.y * dt;
                }
            }
        }
    }
}

/// Input handling system for character movement
pub struct InputMovementSystem {
    move_speed: f32,
}

impl InputMovementSystem {
    pub fn new() -> Self {
        Self {
            move_speed: 200.0, // pixels per second
        }
    }
}

impl System for InputMovementSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // This system would need access to the input manager
        // For now, we'll implement a simple movement pattern
        let entities_with_velocity: Vec<Entity> = world.query::<Velocity>();
        
        for entity in entities_with_velocity {
            if let Some(velocity) = world.get_component_mut::<Velocity>(entity) {
                // Simple test movement - move in a circle
                let time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f32();
                
                velocity.x = (time * 0.5).cos() * self.move_speed;
                velocity.y = (time * 0.5).sin() * self.move_speed;
            }
        }
    }
}

/// Player input system that handles keyboard/gamepad input for players
pub struct PlayerInputSystem {
    move_speed: f32,
    jump_force: f32,
}

impl PlayerInputSystem {
    pub fn new() -> Self {
        Self {
            move_speed: 300.0,
            jump_force: 500.0,
        }
    }
}

impl System for PlayerInputSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // Get all player entities
        let player_entities = world.query::<Player>();
        
        for entity in player_entities {
            // Get player data first
            let player_data = world.get_component::<Player>(entity).cloned();
            
            if let Some(player) = player_data {
                if let Some(velocity) = world.get_component_mut::<Velocity>(entity) {
                    // For now, implement basic movement patterns
                    // In a real implementation, this would read from the input manager
                    match player.input_device {
                        InputDevice::Keyboard => {
                            // Keyboard player movement (WASD)
                            // This would be connected to the actual input manager
                            let time = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs_f32();
                            
                            // Simulate WASD input with sine waves
                            let move_x = (time * 2.0).sin() * self.move_speed;
                            let move_y = if (time * 3.0).sin() > 0.5 { 
                                self.jump_force 
                            } else { 
                                velocity.y 
                            };
                            
                            velocity.x = move_x;
                            velocity.y = move_y;
                        }
                        InputDevice::Gamepad(gamepad_id) => {
                            // Gamepad player movement
                            // This would read from the gamepad input
                            let time = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs_f32();
                            
                            // Simulate gamepad input with different patterns
                            let move_x = (time * 1.5 + gamepad_id as f32).cos() * self.move_speed;
                            let move_y = if (time * 2.5 + gamepad_id as f32).sin() > 0.3 { 
                                self.jump_force 
                            } else { 
                                velocity.y 
                            };
                            
                            velocity.x = move_x;
                            velocity.y = move_y;
                        }
                    }
                }
            }
        }
    }
}

/// Player component to identify player entities
#[derive(Clone)]
pub struct Player {
    pub player_id: u32,
    pub input_device: InputDevice,
}

/// Input device type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputDevice {
    Keyboard,
    Gamepad(u32),
}

/// Character controller system for player movement
pub struct CharacterControllerSystem {
    move_speed: f32,
    jump_force: f32,
    gravity: f32,
    max_fall_speed: f32,
}

impl CharacterControllerSystem {
    pub fn new() -> Self {
        Self {
            move_speed: 300.0,
            jump_force: 500.0,
            gravity: 980.0,
            max_fall_speed: 800.0,
        }
    }
}

impl System for CharacterControllerSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // Get all player entities
        let player_entities = world.query::<Player>();
        
        for entity in player_entities {
            if let Some(velocity) = world.get_component_mut::<Velocity>(entity) {
                // Apply gravity
                velocity.y -= self.gravity * dt;
                
                // Limit fall speed
                if velocity.y < -self.max_fall_speed {
                    velocity.y = -self.max_fall_speed;
                }
                
                // Apply friction to horizontal movement
                velocity.x *= 0.8;
            }
        }
    }
}

/// Combat component for entities that can fight
pub struct Combat {
    pub attack_damage: f32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub current_cooldown: f32,
    pub health: f32,
    pub max_health: f32,
    pub is_alive: bool,
}

/// Attack component for active attacks
#[derive(Clone)]
pub struct Attack {
    pub damage: f32,
    pub range: f32,
    pub duration: f32,
    pub remaining_time: f32,
    pub attacker: Entity,
}

/// Combat system for handling attacks and damage
pub struct CombatSystem;

impl System for CombatSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // Update attack cooldowns
        let combat_entities = world.query::<Combat>();
        for entity in combat_entities {
            if let Some(combat) = world.get_component_mut::<Combat>(entity) {
                if combat.current_cooldown > 0.0 {
                    combat.current_cooldown -= dt;
                }
            }
        }
        
        // Update active attacks
        let attack_entities = world.query::<Attack>();
        let mut attacks_to_remove = Vec::new();
        
        for entity in attack_entities {
            if let Some(attack) = world.get_component_mut::<Attack>(entity) {
                attack.remaining_time -= dt;
                if attack.remaining_time <= 0.0 {
                    attacks_to_remove.push(entity);
                }
            }
        }
        
        // Remove expired attacks
        for entity in attacks_to_remove {
            world.remove_component::<Attack>(entity);
        }
    }
}

/// Damage system for applying damage to entities
pub struct DamageSystem;

impl System for DamageSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // Check for attacks hitting targets
        let attack_entities = world.query::<Attack>();
        
        for attack_entity in attack_entities {
            // Get attack data first
            let attack_data = world.get_component::<Attack>(attack_entity).cloned();
            
            if let Some(attack) = attack_data {
                // Get attacker position
                let attacker_pos = world.get_component::<Position>(attack.attacker).cloned();
                
                if let Some(attacker_position) = attacker_pos {
                    // Find entities within attack range
                    let position_entities = world.query::<Position>();
                    
                    for target_entity in position_entities {
                        if target_entity == attack.attacker {
                            continue; // Don't attack self
                        }
                        
                        if let Some(target_pos) = world.get_component::<Position>(target_entity) {
                            let distance = ((target_pos.x - attacker_position.x).powi(2) + 
                                          (target_pos.y - attacker_position.y).powi(2)).sqrt();
                            
                            if distance <= attack.range {
                                // Apply damage - clone the attack data to avoid borrowing issues
                                let damage = attack.damage;
                                if let Some(combat) = world.get_component_mut::<Combat>(target_entity) {
                                    combat.health -= damage;
                                    if combat.health <= 0.0 {
                                        combat.is_alive = false;
                                        combat.health = 0.0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
