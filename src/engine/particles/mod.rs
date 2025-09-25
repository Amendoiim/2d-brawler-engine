//! Particle system for visual effects

pub mod particle;
pub mod emitter;
pub mod system;

pub use particle::*;
pub use emitter::*;
pub use system::*;

use crate::engine::ecs::{Component, Entity, World};
use glam::Vec2;
use std::collections::HashMap;

/// Particle system for managing visual effects
pub struct ParticleSystem {
    /// Active particle emitters
    pub emitters: HashMap<Entity, ParticleEmitter>,
    /// Particle pool for efficient allocation
    pub particle_pool: Vec<Particle>,
    /// Active particles
    pub active_particles: Vec<Particle>,
    /// System settings
    pub settings: ParticleSystemSettings,
    /// Global gravity
    pub gravity: Vec2,
}

/// Particle system settings
#[derive(Debug, Clone)]
pub struct ParticleSystemSettings {
    /// Maximum number of particles
    pub max_particles: usize,
    /// Enable particle pooling
    pub enable_pooling: bool,
    /// Enable particle sorting
    pub enable_sorting: bool,
    /// Global particle scale
    pub global_scale: f32,
    /// Enable debug rendering
    pub enable_debug: bool,
}

impl ParticleSystem {
    /// Create a new particle system
    pub fn new() -> Self {
        Self {
            emitters: HashMap::new(),
            particle_pool: Vec::new(),
            active_particles: Vec::new(),
            settings: ParticleSystemSettings {
                max_particles: 10000,
                enable_pooling: true,
                enable_sorting: true,
                global_scale: 1.0,
                enable_debug: false,
            },
            gravity: Vec2::new(0.0, -980.0), // Earth gravity
        }
    }

    /// Add a particle emitter to an entity
    pub fn add_emitter(&mut self, entity: Entity, mut emitter: ParticleEmitter) {
        self.emitters.insert(entity, emitter);
    }

    /// Remove a particle emitter from an entity
    pub fn remove_emitter(&mut self, entity: Entity) {
        self.emitters.remove(&entity);
    }

    /// Update the particle system
    pub fn update(&mut self, world: &World, dt: f32) {
        // Collect new particles from all emitters
        let mut all_new_particles = Vec::new();
        
        // Update all emitters
        for (entity, emitter) in self.emitters.iter_mut() {
            emitter.update(dt);
            
            // Emit new particles if needed
            if emitter.is_emitting() {
                let new_particles = emitter.emit_particles(dt);
                all_new_particles.extend(new_particles);
            }
        }
        
        // Spawn all new particles
        for particle in all_new_particles {
            self.spawn_particle(particle);
        }

        // Update all active particles
        for particle in &mut self.active_particles {
            particle.update(dt, self.gravity);
        }

        // Remove dead particles
        self.active_particles.retain(|particle| particle.is_alive());

        // Sort particles if enabled
        if self.settings.enable_sorting {
            self.active_particles.sort_by(|a, b| {
                b.position.y.partial_cmp(&a.position.y).unwrap()
            });
        }
    }

    /// Spawn a new particle
    fn spawn_particle(&mut self, mut particle: Particle) {
        if self.active_particles.len() < self.settings.max_particles {
            particle.life = particle.max_life; // Reset life
            self.active_particles.push(particle);
        }
    }

    /// Get all active particles
    pub fn get_active_particles(&self) -> &Vec<Particle> {
        &self.active_particles
    }

    /// Get particle count
    pub fn get_particle_count(&self) -> usize {
        self.active_particles.len()
    }

    /// Clear all particles
    pub fn clear_particles(&mut self) {
        self.active_particles.clear();
    }

    /// Set global gravity
    pub fn set_gravity(&mut self, gravity: Vec2) {
        self.gravity = gravity;
    }

    /// Get system statistics
    pub fn get_stats(&self) -> ParticleSystemStats {
        ParticleSystemStats {
            active_particles: self.active_particles.len(),
            max_particles: self.settings.max_particles,
            active_emitters: self.emitters.len(),
            memory_usage: self.calculate_memory_usage(),
        }
    }

    /// Calculate memory usage
    fn calculate_memory_usage(&self) -> usize {
        self.active_particles.len() * std::mem::size_of::<Particle>()
    }
}

/// Particle system statistics
#[derive(Debug, Clone)]
pub struct ParticleSystemStats {
    /// Number of active particles
    pub active_particles: usize,
    /// Maximum number of particles
    pub max_particles: usize,
    /// Number of active emitters
    pub active_emitters: usize,
    /// Memory usage in bytes
    pub memory_usage: usize,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ParticleSystemSettings {
    fn default() -> Self {
        Self {
            max_particles: 10000,
            enable_pooling: true,
            enable_sorting: true,
            global_scale: 1.0,
            enable_debug: false,
        }
    }
}
