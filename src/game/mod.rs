//! Game-specific systems and logic

pub mod combat;
pub mod characters;
pub mod levels;
pub mod progression;

use crate::engine::ecs::{World, Component, System};

// Implement Component trait for all game components
impl Component for Position {}
impl Component for Velocity {}
impl Component for Sprite {}
impl Component for Health {}

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
