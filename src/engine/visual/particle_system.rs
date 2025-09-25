//! Particle system for visual effects
//! 
//! This module provides a comprehensive particle system for various visual effects:
//! - Combat effects (hits, explosions, magic)
//! - Environmental effects (dust, smoke, fire)
//! - UI effects (sparks, glows, transitions)
//! - Character effects (trails, auras, buffs)

use std::collections::HashMap;

/// Particle system manager
#[derive(Default)]
pub struct ParticleManager {
    /// Active particle systems
    pub systems: HashMap<String, ParticleSystem>,
    /// Maximum number of particles
    pub max_particles: usize,
    /// Particle culling enabled
    pub culling_enabled: bool,
    /// Quality level
    pub quality: crate::engine::visual::QualityLevel,
    /// Performance metrics
    pub metrics: ParticleMetrics,
}

/// Individual particle system
#[derive(Clone)]
pub struct ParticleSystem {
    /// System ID
    pub id: String,
    /// System configuration
    pub config: ParticleSystemConfig,
    /// Active particles
    pub particles: Vec<Particle>,
    /// Emitter configuration
    pub emitter: ParticleEmitter,
    /// System state
    pub state: ParticleSystemState,
    /// Performance settings
    pub performance: ParticlePerformanceSettings,
}

/// Particle system configuration
#[derive(Clone)]
pub struct ParticleSystemConfig {
    /// Maximum number of particles for this system
    pub max_particles: usize,
    /// Particle lifetime in seconds
    pub lifetime: f32,
    /// Emission rate (particles per second)
    pub emission_rate: f32,
    /// Burst emission count
    pub burst_count: usize,
    /// Particle size range
    pub size_range: (f32, f32),
    /// Particle speed range
    pub speed_range: (f32, f32),
    /// Particle color range
    pub color_range: (Color, Color),
    /// Particle alpha range
    pub alpha_range: (f32, f32),
    /// Gravity effect
    pub gravity: Vec3,
    /// Wind effect
    pub wind: Vec3,
    /// Collision enabled
    pub collision_enabled: bool,
    /// Collision radius
    pub collision_radius: f32,
    /// Bounce factor
    pub bounce_factor: f32,
    /// Friction factor
    pub friction_factor: f32,
}

/// Particle emitter configuration
#[derive(Clone)]
pub struct ParticleEmitter {
    /// Emitter position
    pub position: Vec3,
    /// Emitter rotation
    pub rotation: Quat,
    /// Emitter scale
    pub scale: Vec3,
    /// Emitter shape
    pub shape: EmitterShape,
    /// Emitter direction
    pub direction: Vec3,
    /// Emitter spread angle
    pub spread_angle: f32,
    /// Emitter speed
    pub speed: f32,
    /// Emitter enabled
    pub enabled: bool,
    /// Emitter loop
    pub loop_emission: bool,
    /// Emitter duration
    pub duration: f32,
    /// Emitter delay
    pub delay: f32,
}

/// Emitter shapes
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EmitterShape {
    Point,
    Line,
    Circle,
    Sphere,
    Box,
    Cone,
    Mesh,
}

/// Particle system state
#[derive(Clone)]
pub struct ParticleSystemState {
    /// System active
    pub active: bool,
    /// System paused
    pub paused: bool,
    /// System time
    pub time: f32,
    /// System age
    pub age: f32,
    /// Particles emitted
    pub particles_emitted: usize,
    /// Particles alive
    pub particles_alive: usize,
    /// System completed
    pub completed: bool,
}

/// Individual particle
#[derive(Clone)]
pub struct Particle {
    /// Particle position
    pub position: Vec3,
    /// Particle velocity
    pub velocity: Vec3,
    /// Particle acceleration
    pub acceleration: Vec3,
    /// Particle size
    pub size: f32,
    /// Particle color
    pub color: Color,
    /// Particle alpha
    pub alpha: f32,
    /// Particle lifetime
    pub lifetime: f32,
    /// Particle age
    pub age: f32,
    /// Particle rotation
    pub rotation: f32,
    /// Particle angular velocity
    pub angular_velocity: f32,
    /// Particle alive
    pub alive: bool,
    /// Particle data
    pub data: ParticleData,
}

/// Particle data for custom effects
#[derive(Clone)]
pub struct ParticleData {
    /// Custom float values
    pub floats: Vec<f32>,
    /// Custom int values
    pub ints: Vec<i32>,
    /// Custom bool values
    pub bools: Vec<bool>,
    /// Custom string values
    pub strings: Vec<String>,
}

/// Particle performance settings
#[derive(Clone)]
pub struct ParticlePerformanceSettings {
    /// Enable particle culling
    pub culling_enabled: bool,
    /// Culling distance
    pub culling_distance: f32,
    /// LOD distance
    pub lod_distance: f32,
    /// LOD reduction factor
    pub lod_reduction: f32,
    /// Update frequency
    pub update_frequency: f32,
    /// Last update time
    pub last_update: f32,
}

/// Particle metrics
#[derive(Clone, Default)]
pub struct ParticleMetrics {
    /// Total particles
    pub total_particles: usize,
    /// Active particles
    pub active_particles: usize,
    /// Systems count
    pub systems_count: usize,
    /// Update time
    pub update_time: f32,
    /// Memory usage
    pub memory_usage: usize,
}

/// Particle effect types
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ParticleEffectType {
    // Combat effects
    Hit,
    Explosion,
    Magic,
    Fire,
    Ice,
    Lightning,
    Poison,
    Heal,
    
    // Environmental effects
    Dust,
    Smoke,
    Steam,
    Rain,
    Snow,
    Fireflies,
    Sparks,
    
    // UI effects
    Glow,
    Pulse,
    Fade,
    Slide,
    Scale,
    Rotate,
    
    // Character effects
    Trail,
    Aura,
    Buff,
    Debuff,
    Shield,
    Teleport,
}

/// Simple 3D vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
    
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }
}

/// Simple quaternion
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub const IDENTITY: Quat = Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
}

/// Simple color
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
}

impl ParticleManager {
    /// Create a new particle manager
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
            max_particles: 1000,
            culling_enabled: true,
            quality: crate::engine::visual::QualityLevel::High,
            metrics: ParticleMetrics::default(),
        }
    }

    /// Create a new particle system
    pub fn create_system(&mut self, id: String, config: ParticleSystemConfig) -> Result<(), String> {
        if self.systems.contains_key(&id) {
            return Err(format!("Particle system '{}' already exists", id));
        }

        let system = ParticleSystem {
            id: id.clone(),
            config,
            particles: Vec::new(),
            emitter: ParticleEmitter::default(),
            state: ParticleSystemState::default(),
            performance: ParticlePerformanceSettings::default(),
        };

        self.systems.insert(id, system);
        Ok(())
    }

    /// Remove a particle system
    pub fn remove_system(&mut self, id: &str) -> bool {
        self.systems.remove(id).is_some()
    }

    /// Get a particle system
    pub fn get_system(&self, id: &str) -> Option<&ParticleSystem> {
        self.systems.get(id)
    }

    /// Get a mutable particle system
    pub fn get_system_mut(&mut self, id: &str) -> Option<&mut ParticleSystem> {
        self.systems.get_mut(id)
    }

    /// Update all particle systems
    pub fn update(&mut self, delta_time: f32) {
        for system in self.systems.values_mut() {
            system.update(delta_time);
        }
        
        self.update_metrics();
    }

    /// Update performance metrics
    fn update_metrics(&mut self) {
        self.metrics.total_particles = 0;
        self.metrics.active_particles = 0;
        self.metrics.systems_count = self.systems.len();
        
        for system in self.systems.values() {
            self.metrics.total_particles += system.config.max_particles;
            self.metrics.active_particles += system.state.particles_alive;
        }
    }

    /// Set maximum particles
    pub fn set_max_particles(&mut self, max_particles: usize) {
        self.max_particles = max_particles;
    }

    /// Set culling enabled
    pub fn set_culling_enabled(&mut self, enabled: bool) {
        self.culling_enabled = enabled;
    }

    /// Set quality level
    pub fn set_quality(&mut self, quality: crate::engine::visual::QualityLevel) {
        self.quality = quality;
    }

    /// Get active particle count
    pub fn get_active_particle_count(&self) -> usize {
        self.metrics.active_particles
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> &ParticleMetrics {
        &self.metrics
    }
}

impl Default for ParticleSystemConfig {
    fn default() -> Self {
        Self {
            max_particles: 100,
            lifetime: 1.0,
            emission_rate: 10.0,
            burst_count: 0,
            size_range: (0.1, 0.5),
            speed_range: (1.0, 5.0),
            color_range: (Color::WHITE, Color::WHITE),
            alpha_range: (0.0, 1.0),
            gravity: Vec3::new(0.0, -9.81, 0.0),
            wind: Vec3::ZERO,
            collision_enabled: false,
            collision_radius: 0.1,
            bounce_factor: 0.5,
            friction_factor: 0.9,
        }
    }
}

impl Default for ParticleEmitter {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            shape: EmitterShape::Point,
            direction: Vec3::new(0.0, 1.0, 0.0),
            spread_angle: 45.0,
            speed: 1.0,
            enabled: true,
            loop_emission: true,
            duration: 0.0,
            delay: 0.0,
        }
    }
}

impl Default for ParticleSystemState {
    fn default() -> Self {
        Self {
            active: false,
            paused: false,
            time: 0.0,
            age: 0.0,
            particles_emitted: 0,
            particles_alive: 0,
            completed: false,
        }
    }
}

impl Default for ParticleData {
    fn default() -> Self {
        Self {
            floats: Vec::new(),
            ints: Vec::new(),
            bools: Vec::new(),
            strings: Vec::new(),
        }
    }
}

impl Default for ParticlePerformanceSettings {
    fn default() -> Self {
        Self {
            culling_enabled: true,
            culling_distance: 100.0,
            lod_distance: 50.0,
            lod_reduction: 0.5,
            update_frequency: 60.0,
            last_update: 0.0,
        }
    }
}

impl ParticleSystem {
    /// Update particle system
    pub fn update(&mut self, delta_time: f32) {
        if !self.state.active || self.state.paused {
            return;
        }

        self.state.time += delta_time;
        self.state.age += delta_time;

        // Update particles
        for particle in &mut self.particles {
            if particle.alive {
                particle.update(delta_time, &self.config);
            }
        }

        // Remove dead particles
        self.particles.retain(|p| p.alive);

        // Emit new particles
        if self.emitter.enabled && self.state.time >= self.emitter.delay {
            self.emit_particles(delta_time);
        }

        // Update state
        self.state.particles_alive = self.particles.iter().filter(|p| p.alive).count();
        
        if self.state.particles_alive == 0 && !self.emitter.loop_emission {
            self.state.completed = true;
            self.state.active = false;
        }
    }

    /// Emit particles
    fn emit_particles(&mut self, delta_time: f32) {
        let particles_to_emit = (self.config.emission_rate * delta_time) as usize;
        
        for _ in 0..particles_to_emit {
            if self.particles.len() >= self.config.max_particles {
                break;
            }

            let particle = self.create_particle();
            self.particles.push(particle);
            self.state.particles_emitted += 1;
        }
    }

    /// Create a new particle
    fn create_particle(&self) -> Particle {
        let mut particle = Particle {
            position: self.emitter.position,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            size: self.config.size_range.0 + (self.config.size_range.1 - self.config.size_range.0) * fastrand::f32(),
            color: self.config.color_range.0,
            alpha: self.config.alpha_range.0 + (self.config.alpha_range.1 - self.config.alpha_range.0) * fastrand::f32(),
            lifetime: self.config.lifetime,
            age: 0.0,
            rotation: 0.0,
            angular_velocity: 0.0,
            alive: true,
            data: ParticleData::default(),
        };

        // Set velocity based on emitter
        let direction = self.get_emission_direction();
        let speed = self.config.speed_range.0 + (self.config.speed_range.1 - self.config.speed_range.0) * fastrand::f32();
        particle.velocity = Vec3::new(direction.x * speed, direction.y * speed, direction.z * speed);

        particle
    }

    /// Get emission direction
    fn get_emission_direction(&self) -> Vec3 {
        match self.emitter.shape {
            EmitterShape::Point => {
                let angle = fastrand::f32() * self.emitter.spread_angle.to_radians();
                let direction = self.emitter.direction.normalize();
                // Rotate direction by random angle
                direction
            }
            EmitterShape::Line => {
                let t = fastrand::f32();
                let start = self.emitter.position;
                let end = Vec3::new(
                    self.emitter.position.x + self.emitter.direction.x,
                    self.emitter.position.y + self.emitter.direction.y,
                    self.emitter.position.z + self.emitter.direction.z,
                );
                let point = Vec3::new(
                    start.x + (end.x - start.x) * t,
                    start.y + (end.y - start.y) * t,
                    start.z + (end.z - start.z) * t,
                );
                Vec3::new(point.x - self.emitter.position.x, point.y - self.emitter.position.y, point.z - self.emitter.position.z).normalize()
            }
            EmitterShape::Circle => {
                let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
                let radius = fastrand::f32() * self.emitter.scale.x;
                let x = angle.cos() * radius;
                let z = angle.sin() * radius;
                Vec3::new(x, 0.0, z).normalize()
            }
            EmitterShape::Sphere => {
                let theta = fastrand::f32() * 2.0 * std::f32::consts::PI;
                let phi = fastrand::f32() * std::f32::consts::PI;
                let x = phi.sin() * theta.cos();
                let y = phi.sin() * theta.sin();
                let z = phi.cos();
                Vec3::new(x, y, z).normalize()
            }
            EmitterShape::Box => {
                let x = (fastrand::f32() - 0.5) * self.emitter.scale.x;
                let y = (fastrand::f32() - 0.5) * self.emitter.scale.y;
                let z = (fastrand::f32() - 0.5) * self.emitter.scale.z;
                Vec3::new(x, y, z).normalize()
            }
            EmitterShape::Cone => {
                let angle = fastrand::f32() * self.emitter.spread_angle.to_radians();
                let direction = self.emitter.direction.normalize();
                // Create cone direction
                direction
            }
            EmitterShape::Mesh => {
                // For mesh emission, we'd need mesh data
                self.emitter.direction.normalize()
            }
        }
    }
}

impl Particle {
    /// Update particle
    pub fn update(&mut self, delta_time: f32, config: &ParticleSystemConfig) {
        if !self.alive {
            return;
        }

        self.age += delta_time;
        
        if self.age >= self.lifetime {
            self.alive = false;
            return;
        }

        // Update physics
        self.velocity = Vec3::new(
            self.velocity.x + config.gravity.x * delta_time,
            self.velocity.y + config.gravity.y * delta_time,
            self.velocity.z + config.gravity.z * delta_time,
        );
        self.velocity = Vec3::new(
            self.velocity.x + config.wind.x * delta_time,
            self.velocity.y + config.wind.y * delta_time,
            self.velocity.z + config.wind.z * delta_time,
        );
        self.position = Vec3::new(
            self.position.x + self.velocity.x * delta_time,
            self.position.y + self.velocity.y * delta_time,
            self.position.z + self.velocity.z * delta_time,
        );
        
        // Update rotation
        self.rotation += self.angular_velocity * delta_time;
        
        // Update alpha based on lifetime
        let lifetime_ratio = self.age / self.lifetime;
        self.alpha = config.alpha_range.0 + (config.alpha_range.1 - config.alpha_range.0) * (1.0 - lifetime_ratio);
        
        // Update color based on lifetime
        let color_ratio = lifetime_ratio;
        self.color = config.color_range.0.lerp(config.color_range.1, color_ratio);
    }
}