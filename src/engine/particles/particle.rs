//! Individual particle data structures and behavior

use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Individual particle in the system
#[derive(Debug, Clone)]
pub struct Particle {
    /// Particle position
    pub position: Vec2,
    /// Particle velocity
    pub velocity: Vec2,
    /// Particle acceleration
    pub acceleration: Vec2,
    /// Particle size
    pub size: f32,
    /// Particle color (RGBA)
    pub color: [f32; 4],
    /// Particle rotation
    pub rotation: f32,
    /// Particle angular velocity
    pub angular_velocity: f32,
    /// Particle life (0.0 to max_life)
    pub life: f32,
    /// Maximum particle life
    pub max_life: f32,
    /// Particle texture ID
    pub texture_id: Option<u32>,
    /// Particle scale
    pub scale: f32,
    /// Particle mass
    pub mass: f32,
    /// Particle drag coefficient
    pub drag: f32,
    /// Particle bounce coefficient
    pub bounce: f32,
    /// Particle friction
    pub friction: f32,
    /// Particle flags
    pub flags: ParticleFlags,
}

/// Particle behavior flags
#[derive(Debug, Clone)]
pub struct ParticleFlags {
    /// Whether particle is affected by gravity
    pub affected_by_gravity: bool,
    /// Whether particle bounces off surfaces
    pub bounces: bool,
    /// Whether particle fades out over time
    pub fades: bool,
    /// Whether particle scales over time
    pub scales: bool,
    /// Whether particle rotates over time
    pub rotates: bool,
    /// Whether particle is affected by wind
    pub affected_by_wind: bool,
}

impl Particle {
    /// Create a new particle
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            size: 1.0,
            color: [1.0, 1.0, 1.0, 1.0],
            rotation: 0.0,
            angular_velocity: 0.0,
            life: 1.0,
            max_life: 1.0,
            texture_id: None,
            scale: 1.0,
            mass: 1.0,
            drag: 0.0,
            bounce: 0.0,
            friction: 0.0,
            flags: ParticleFlags::default(),
        }
    }

    /// Update particle physics
    pub fn update(&mut self, dt: f32, gravity: Vec2) {
        // Apply gravity if enabled
        if self.flags.affected_by_gravity {
            self.acceleration += gravity;
        }

        // Apply drag
        if self.drag > 0.0 {
            let drag_force = -self.velocity * self.drag;
            self.acceleration += drag_force;
        }

        // Update velocity
        self.velocity += self.acceleration * dt;

        // Update position
        self.position += self.velocity * dt;

        // Update rotation
        if self.flags.rotates {
            self.rotation += self.angular_velocity * dt;
        }

        // Update life
        self.life -= dt;

        // Update visual properties based on life
        self.update_visual_properties();

        // Reset acceleration for next frame
        self.acceleration = Vec2::ZERO;
    }

    /// Update visual properties based on particle life
    fn update_visual_properties(&mut self) {
        let life_ratio = self.life / self.max_life;

        // Fade out over time
        if self.flags.fades {
            self.color[3] = life_ratio;
        }

        // Scale over time
        if self.flags.scales {
            self.scale = life_ratio;
        }
    }

    /// Check if particle is alive
    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    /// Get particle age (0.0 to 1.0)
    pub fn get_age(&self) -> f32 {
        1.0 - (self.life / self.max_life)
    }

    /// Set particle position
    pub fn set_position(&mut self, position: Vec2) -> &mut Self {
        self.position = position;
        self
    }

    /// Set particle velocity
    pub fn set_velocity(&mut self, velocity: Vec2) -> &mut Self {
        self.velocity = velocity;
        self
    }

    /// Set particle acceleration
    pub fn set_acceleration(&mut self, acceleration: Vec2) -> &mut Self {
        self.acceleration = acceleration;
        self
    }

    /// Set particle size
    pub fn set_size(&mut self, size: f32) -> &mut Self {
        self.size = size;
        self
    }

    /// Set particle color
    pub fn set_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = color;
        self
    }

    /// Set particle rotation
    pub fn set_rotation(&mut self, rotation: f32) -> &mut Self {
        self.rotation = rotation;
        self
    }

    /// Set particle angular velocity
    pub fn set_angular_velocity(&mut self, angular_velocity: f32) -> &mut Self {
        self.angular_velocity = angular_velocity;
        self
    }

    /// Set particle life
    pub fn set_life(&mut self, life: f32) -> &mut Self {
        self.life = life;
        self.max_life = life;
        self
    }

    /// Set particle texture
    pub fn set_texture(&mut self, texture_id: u32) -> &mut Self {
        self.texture_id = Some(texture_id);
        self
    }

    /// Set particle scale
    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        self.scale = scale;
        self
    }

    /// Set particle mass
    pub fn set_mass(&mut self, mass: f32) -> &mut Self {
        self.mass = mass;
        self
    }

    /// Set particle drag
    pub fn set_drag(&mut self, drag: f32) -> &mut Self {
        self.drag = drag;
        self
    }

    /// Set particle bounce
    pub fn set_bounce(&mut self, bounce: f32) -> &mut Self {
        self.bounce = bounce;
        self
    }

    /// Set particle friction
    pub fn set_friction(&mut self, friction: f32) -> &mut Self {
        self.friction = friction;
        self
    }

    /// Set particle flags
    pub fn set_flags(&mut self, flags: ParticleFlags) -> &mut Self {
        self.flags = flags;
        self
    }

    /// Apply force to particle
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }

    /// Apply impulse to particle
    pub fn apply_impulse(&mut self, impulse: Vec2) {
        self.velocity += impulse / self.mass;
    }

    /// Bounce particle off a surface
    pub fn bounce_off_surface(&mut self, normal: Vec2, restitution: f32) {
        if self.flags.bounces {
            let velocity_along_normal = self.velocity.dot(normal);
            if velocity_along_normal < 0.0 {
                self.velocity -= normal * velocity_along_normal * (1.0 + restitution);
            }
        }
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ParticleFlags {
    fn default() -> Self {
        Self {
            affected_by_gravity: true,
            bounces: false,
            fades: true,
            scales: false,
            rotates: false,
            affected_by_wind: false,
        }
    }
}

/// Particle builder for easy particle creation
pub struct ParticleBuilder {
    particle: Particle,
}

impl ParticleBuilder {
    /// Create a new particle builder
    pub fn new() -> Self {
        Self {
            particle: Particle::new(),
        }
    }

    /// Set position
    pub fn position(mut self, position: Vec2) -> Self {
        self.particle.position = position;
        self
    }

    /// Set velocity
    pub fn velocity(mut self, velocity: Vec2) -> Self {
        self.particle.velocity = velocity;
        self
    }

    /// Set acceleration
    pub fn acceleration(mut self, acceleration: Vec2) -> Self {
        self.particle.acceleration = acceleration;
        self
    }

    /// Set size
    pub fn size(mut self, size: f32) -> Self {
        self.particle.size = size;
        self
    }

    /// Set color
    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.particle.color = color;
        self
    }

    /// Set rotation
    pub fn rotation(mut self, rotation: f32) -> Self {
        self.particle.rotation = rotation;
        self
    }

    /// Set angular velocity
    pub fn angular_velocity(mut self, angular_velocity: f32) -> Self {
        self.particle.angular_velocity = angular_velocity;
        self
    }

    /// Set life
    pub fn life(mut self, life: f32) -> Self {
        self.particle.set_life(life);
        self
    }

    /// Set texture
    pub fn texture(mut self, texture_id: u32) -> Self {
        self.particle.set_texture(texture_id);
        self
    }

    /// Set scale
    pub fn scale(mut self, scale: f32) -> Self {
        self.particle.set_scale(scale);
        self
    }

    /// Set mass
    pub fn mass(mut self, mass: f32) -> Self {
        self.particle.set_mass(mass);
        self
    }

    /// Set drag
    pub fn drag(mut self, drag: f32) -> Self {
        self.particle.set_drag(drag);
        self
    }

    /// Set bounce
    pub fn bounce(mut self, bounce: f32) -> Self {
        self.particle.set_bounce(bounce);
        self
    }

    /// Set friction
    pub fn friction(mut self, friction: f32) -> Self {
        self.particle.set_friction(friction);
        self
    }

    /// Set flags
    pub fn flags(mut self, flags: ParticleFlags) -> Self {
        self.particle.set_flags(flags);
        self
    }

    /// Build the particle
    pub fn build(self) -> Particle {
        self.particle
    }
}

impl Default for ParticleBuilder {
    fn default() -> Self {
        Self::new()
    }
}
