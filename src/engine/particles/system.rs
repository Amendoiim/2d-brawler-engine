//! Particle system implementation

use crate::engine::particles::{ParticleSystem, ParticleSystemSettings};
use crate::engine::particles::emitter::ParticleEmitter;
use crate::engine::ecs::{Component, Entity, World};
use glam::Vec2;

/// Particle system implementation
impl ParticleSystem {
    /// Create a new particle system with custom settings
    pub fn with_settings(settings: ParticleSystemSettings) -> Self {
        Self {
            emitters: std::collections::HashMap::new(),
            particle_pool: Vec::new(),
            active_particles: Vec::new(),
            settings,
            gravity: Vec2::new(0.0, -980.0),
        }
    }

    /// Create a simple explosion effect
    pub fn create_explosion_effect(&mut self, entity: Entity, position: Vec2, intensity: f32) {
        let mut emitter = ParticleEmitter::new()
            .set_position(position)
            .set_emission_rate(0.0) // Burst only
            .set_max_particles(50)
            .set_lifetime(0.1) // Short burst
            .set_emission_shape(crate::engine::particles::emitter::EmissionShape::Circle { radius: 10.0 })
            .set_emission_direction(Vec2::new(0.0, 1.0))
            .set_emission_spread(2.0 * std::f32::consts::PI) // Full circle
            .set_speed_range((100.0 * intensity, 300.0 * intensity))
            .set_size_range((2.0, 5.0))
            .set_life_range((0.5, 1.5))
            .set_color_range(([1.0, 0.5, 0.0, 1.0], [1.0, 0.0, 0.0, 0.0]))
            .set_scale_range((1.0, 0.5))
            .set_mass_range((0.5, 1.5))
            .set_drag_range((0.1, 0.3));

        // Set burst settings
        emitter.set_burst_settings(crate::engine::particles::emitter::BurstSettings {
            particle_count: 50,
            burst_interval: 0.0,
            time_since_burst: 0.0,
            burst_count: 1,
            current_burst: 0,
        });

        self.add_emitter(entity, emitter);
    }

    /// Create a trail effect
    pub fn create_trail_effect(&mut self, entity: Entity, position: Vec2) {
        let emitter = ParticleEmitter::new()
            .set_position(position)
            .set_emission_rate(20.0)
            .set_max_particles(100)
            .set_lifetime(-1.0) // Infinite
            .set_emission_shape(crate::engine::particles::emitter::EmissionShape::Point)
            .set_emission_direction(Vec2::new(0.0, -1.0))
            .set_emission_spread(0.2)
            .set_speed_range((50.0, 100.0))
            .set_size_range((1.0, 2.0))
            .set_life_range((0.5, 1.0))
            .set_color_range(([0.5, 0.5, 1.0, 0.8], [0.0, 0.0, 1.0, 0.0]))
            .set_scale_range((1.0, 0.3))
            .set_mass_range((0.1, 0.5))
            .set_drag_range((0.2, 0.5));

        self.add_emitter(entity, emitter);
    }

    /// Create a spark effect
    pub fn create_spark_effect(&mut self, entity: Entity, position: Vec2, direction: Vec2) {
        let emitter = ParticleEmitter::new()
            .set_position(position)
            .set_emission_rate(0.0) // Burst only
            .set_max_particles(20)
            .set_lifetime(0.05) // Very short burst
            .set_emission_shape(crate::engine::particles::emitter::EmissionShape::Point)
            .set_emission_direction(direction)
            .set_emission_spread(0.5)
            .set_speed_range((200.0, 400.0))
            .set_size_range((0.5, 1.5))
            .set_life_range((0.2, 0.5))
            .set_color_range(([1.0, 1.0, 0.0, 1.0], [1.0, 0.5, 0.0, 0.0]))
            .set_scale_range((1.0, 0.2))
            .set_mass_range((0.1, 0.3))
            .set_drag_range((0.1, 0.2));

        // Set burst settings
        emitter.set_burst_settings(crate::engine::particles::emitter::BurstSettings {
            particle_count: 20,
            burst_interval: 0.0,
            time_since_burst: 0.0,
            burst_count: 1,
            current_burst: 0,
        });

        self.add_emitter(entity, emitter);
    }
}
