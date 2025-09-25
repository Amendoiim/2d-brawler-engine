//! Particle emitter for creating and managing particle effects

use crate::engine::particles::particle::{Particle, ParticleBuilder, ParticleFlags};
use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Particle emitter for creating particle effects
#[derive(Debug, Clone)]
pub struct ParticleEmitter {
    /// Emitter position
    pub position: Vec2,
    /// Emitter rotation
    pub rotation: f32,
    /// Emitter scale
    pub scale: f32,
    /// Emission rate (particles per second)
    pub emission_rate: f32,
    /// Maximum number of particles to emit
    pub max_particles: usize,
    /// Current number of particles emitted
    pub particles_emitted: usize,
    /// Emitter lifetime (-1 for infinite)
    pub lifetime: f32,
    /// Current emitter age
    pub age: f32,
    /// Whether emitter is active
    pub is_active: bool,
    /// Whether emitter is emitting
    pub is_emitting: bool,
    /// Emission burst settings
    pub burst_settings: Option<BurstSettings>,
    /// Particle template
    pub particle_template: ParticleTemplate,
    /// Emission shape
    pub emission_shape: EmissionShape,
    /// Emission direction
    pub emission_direction: Vec2,
    /// Emission spread (in radians)
    pub emission_spread: f32,
    /// Emission speed range
    pub speed_range: (f32, f32),
    /// Emission size range
    pub size_range: (f32, f32),
    /// Emission life range
    pub life_range: (f32, f32),
    /// Emission color range
    pub color_range: ([f32; 4], [f32; 4]),
    /// Emission rotation range
    pub rotation_range: (f32, f32),
    /// Emission angular velocity range
    pub angular_velocity_range: (f32, f32),
    /// Emission scale range
    pub scale_range: (f32, f32),
    /// Emission mass range
    pub mass_range: (f32, f32),
    /// Emission drag range
    pub drag_range: (f32, f32),
    /// Emission bounce range
    pub bounce_range: (f32, f32),
    /// Emission friction range
    pub friction_range: (f32, f32),
    /// Emission flags
    pub emission_flags: ParticleFlags,
    /// Emission texture ID
    pub texture_id: Option<u32>,
    /// Emission parameters
    pub parameters: HashMap<String, f32>,
}

/// Burst emission settings
#[derive(Debug, Clone)]
pub struct BurstSettings {
    /// Number of particles to emit in burst
    pub particle_count: usize,
    /// Time between bursts
    pub burst_interval: f32,
    /// Time since last burst
    pub time_since_burst: f32,
    /// Number of bursts (-1 for infinite)
    pub burst_count: i32,
    /// Current burst number
    pub current_burst: i32,
}

/// Particle template for emission
#[derive(Debug, Clone)]
pub struct ParticleTemplate {
    /// Base particle properties
    pub base_particle: Particle,
    /// Property variations
    pub variations: ParticleVariations,
}

/// Property variations for particles
#[derive(Debug, Clone)]
pub struct ParticleVariations {
    /// Velocity variation
    pub velocity_variation: f32,
    /// Size variation
    pub size_variation: f32,
    /// Life variation
    pub life_variation: f32,
    /// Color variation
    pub color_variation: f32,
    /// Rotation variation
    pub rotation_variation: f32,
    /// Angular velocity variation
    pub angular_velocity_variation: f32,
    /// Scale variation
    pub scale_variation: f32,
    /// Mass variation
    pub mass_variation: f32,
    /// Drag variation
    pub drag_variation: f32,
    /// Bounce variation
    pub bounce_variation: f32,
    /// Friction variation
    pub friction_variation: f32,
}

/// Emission shape for particles
#[derive(Debug, Clone)]
pub enum EmissionShape {
    /// Point emission
    Point,
    /// Circle emission
    Circle { radius: f32 },
    /// Rectangle emission
    Rectangle { width: f32, height: f32 },
    /// Line emission
    Line { length: f32 },
    /// Custom shape
    Custom { points: Vec<Vec2> },
}

impl ParticleEmitter {
    /// Create a new particle emitter
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: 1.0,
            emission_rate: 10.0,
            max_particles: 100,
            particles_emitted: 0,
            lifetime: -1.0,
            age: 0.0,
            is_active: true,
            is_emitting: true,
            burst_settings: None,
            particle_template: ParticleTemplate::default(),
            emission_shape: EmissionShape::Point,
            emission_direction: Vec2::new(0.0, 1.0),
            emission_spread: 0.0,
            speed_range: (100.0, 200.0),
            size_range: (1.0, 2.0),
            life_range: (1.0, 3.0),
            color_range: ([1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]),
            rotation_range: (0.0, 0.0),
            angular_velocity_range: (0.0, 0.0),
            scale_range: (1.0, 1.0),
            mass_range: (1.0, 1.0),
            drag_range: (0.0, 0.0),
            bounce_range: (0.0, 0.0),
            friction_range: (0.0, 0.0),
            emission_flags: ParticleFlags::default(),
            texture_id: None,
            parameters: HashMap::new(),
        }
    }

    /// Update the emitter
    pub fn update(&mut self, dt: f32) {
        if !self.is_active {
            return;
        }

        self.age += dt;

        // Check if emitter should be destroyed
        if self.lifetime > 0.0 && self.age >= self.lifetime {
            self.is_active = false;
            self.is_emitting = false;
            return;
        }

        // Update burst settings
        if let Some(burst) = &mut self.burst_settings {
            burst.time_since_burst += dt;
        }
    }

    /// Check if emitter should emit particles
    pub fn is_emitting(&self) -> bool {
        self.is_active && self.is_emitting && 
        (self.max_particles == 0 || self.particles_emitted < self.max_particles)
    }

    /// Emit particles based on emission rate
    pub fn emit_particles(&mut self, dt: f32) -> Vec<Particle> {
        let mut particles = Vec::new();

        if !self.is_emitting() {
            return particles;
        }

        // Handle burst emission
        if let Some(burst) = &mut self.burst_settings {
            if burst.time_since_burst >= burst.burst_interval {
                if burst.burst_count == -1 || burst.current_burst < burst.burst_count {
                    let particle_count = burst.particle_count;
                    // Create particles without borrowing self
                    for _ in 0..particle_count {
                        if let Some(particle) = self.create_particle() {
                            particles.push(particle);
                        }
                    }
                    self.particles_emitted += particle_count;
                    burst.time_since_burst = 0.0;
                    burst.current_burst += 1;
                }
            }
        } else {
            // Handle continuous emission
            let particles_to_emit = (self.emission_rate * dt) as usize;
            for _ in 0..particles_to_emit {
                if let Some(particle) = self.create_particle() {
                    particles.push(particle);
                    self.particles_emitted += 1;
                }
            }
        }

        particles
    }

    /// Create a single particle
    fn create_particle(&mut self) -> Option<Particle> {
        if self.max_particles > 0 && self.particles_emitted >= self.max_particles {
            return None;
        }

        let mut particle = ParticleBuilder::new()
            .position(self.get_emission_position())
            .velocity(self.get_emission_velocity())
            .size(self.get_random_value(self.size_range))
            .color(self.get_random_color())
            .rotation(self.get_random_value(self.rotation_range))
            .angular_velocity(self.get_random_value(self.angular_velocity_range))
            .life(self.get_random_value(self.life_range))
            .scale(self.get_random_value(self.scale_range))
            .mass(self.get_random_value(self.mass_range))
            .drag(self.get_random_value(self.drag_range))
            .bounce(self.get_random_value(self.bounce_range))
            .friction(self.get_random_value(self.friction_range))
            .flags(self.emission_flags.clone())
            .build();

        if let Some(texture_id) = self.texture_id {
            particle.set_texture(texture_id);
        }

        Some(particle)
    }

    /// Get random emission position based on shape
    fn get_emission_position(&self) -> Vec2 {
        match &self.emission_shape {
            EmissionShape::Point => self.position,
            EmissionShape::Circle { radius } => {
                let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
                let distance = fastrand::f32() * radius;
                self.position + Vec2::new(angle.cos(), angle.sin()) * distance
            }
            EmissionShape::Rectangle { width, height } => {
                let x = (fastrand::f32() - 0.5) * width;
                let y = (fastrand::f32() - 0.5) * height;
                self.position + Vec2::new(x, y)
            }
            EmissionShape::Line { length } => {
                let t = fastrand::f32();
                let offset = (t - 0.5) * length;
                self.position + self.emission_direction * offset
            }
            EmissionShape::Custom { points } => {
                if points.is_empty() {
                    self.position
                } else {
                    let index = fastrand::usize(..points.len());
                    self.position + points[index]
                }
            }
        }
    }

    /// Get random emission velocity
    fn get_emission_velocity(&self) -> Vec2 {
        let speed = self.get_random_value(self.speed_range);
        let angle = self.rotation + (fastrand::f32() - 0.5) * self.emission_spread;
        Vec2::new(angle.cos(), angle.sin()) * speed
    }

    /// Get random value from range
    fn get_random_value(&self, range: (f32, f32)) -> f32 {
        fastrand::f32() * (range.1 - range.0) + range.0
    }

    /// Get random color from range
    fn get_random_color(&self) -> [f32; 4] {
        [
            self.get_random_value((self.color_range.0[0], self.color_range.1[0])),
            self.get_random_value((self.color_range.0[1], self.color_range.1[1])),
            self.get_random_value((self.color_range.0[2], self.color_range.1[2])),
            self.get_random_value((self.color_range.0[3], self.color_range.1[3])),
        ]
    }

    /// Set emitter position
    pub fn set_position(&mut self, position: Vec2) -> &mut Self {
        self.position = position;
        self
    }

    /// Set emitter rotation
    pub fn set_rotation(&mut self, rotation: f32) -> &mut Self {
        self.rotation = rotation;
        self
    }

    /// Set emitter scale
    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        self.scale = scale;
        self
    }

    /// Set emission rate
    pub fn set_emission_rate(&mut self, rate: f32) -> &mut Self {
        self.emission_rate = rate;
        self
    }

    /// Set maximum particles
    pub fn set_max_particles(&mut self, max: usize) -> &mut Self {
        self.max_particles = max;
        self
    }

    /// Set emitter lifetime
    pub fn set_lifetime(&mut self, lifetime: f32) -> &mut Self {
        self.lifetime = lifetime;
        self
    }

    /// Set emitter active state
    pub fn set_active(&mut self, active: bool) -> &mut Self {
        self.is_active = active;
        self
    }

    /// Set emitter emitting state
    pub fn set_emitting(&mut self, emitting: bool) -> &mut Self {
        self.is_emitting = emitting;
        self
    }

    /// Set burst settings
    pub fn set_burst_settings(&mut self, settings: BurstSettings) -> &mut Self {
        self.burst_settings = Some(settings);
        self
    }

    /// Set emission shape
    pub fn set_emission_shape(&mut self, shape: EmissionShape) -> &mut Self {
        self.emission_shape = shape;
        self
    }

    /// Set emission direction
    pub fn set_emission_direction(&mut self, direction: Vec2) -> &mut Self {
        self.emission_direction = direction;
        self
    }

    /// Set emission spread
    pub fn set_emission_spread(&mut self, spread: f32) -> &mut Self {
        self.emission_spread = spread;
        self
    }

    /// Set speed range
    pub fn set_speed_range(&mut self, range: (f32, f32)) -> &mut Self {
        self.speed_range = range;
        self
    }

    /// Set size range
    pub fn set_size_range(&mut self, range: (f32, f32)) -> &mut Self {
        self.size_range = range;
        self
    }

    /// Set life range
    pub fn set_life_range(&mut self, range: (f32, f32)) -> &mut Self {
        self.life_range = range;
        self
    }

    /// Set color range
    pub fn set_color_range(&mut self, range: ([f32; 4], [f32; 4])) -> &mut Self {
        self.color_range = range;
        self
    }

    /// Set scale range
    pub fn set_scale_range(&mut self, range: (f32, f32)) -> &mut Self {
        self.scale_range = range;
        self
    }

    /// Set texture ID
    pub fn set_texture_id(&mut self, texture_id: u32) -> &mut Self {
        self.texture_id = Some(texture_id);
        self
    }

    /// Set mass range
    pub fn set_mass_range(&mut self, range: (f32, f32)) -> &mut Self {
        self.mass_range = range;
        self
    }

    /// Set drag range
    pub fn set_drag_range(&mut self, range: (f32, f32)) -> &mut Self {
        self.drag_range = range;
        self
    }

    /// Set parameter
    pub fn set_parameter(&mut self, name: String, value: f32) -> &mut Self {
        self.parameters.insert(name, value);
        self
    }

    /// Get parameter
    pub fn get_parameter(&self, name: &str) -> Option<f32> {
        self.parameters.get(name).copied()
    }
}

impl Default for ParticleEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ParticleTemplate {
    fn default() -> Self {
        Self {
            base_particle: Particle::new(),
            variations: ParticleVariations::default(),
        }
    }
}

impl Default for ParticleVariations {
    fn default() -> Self {
        Self {
            velocity_variation: 0.0,
            size_variation: 0.0,
            life_variation: 0.0,
            color_variation: 0.0,
            rotation_variation: 0.0,
            angular_velocity_variation: 0.0,
            scale_variation: 0.0,
            mass_variation: 0.0,
            drag_variation: 0.0,
            bounce_variation: 0.0,
            friction_variation: 0.0,
        }
    }
}
