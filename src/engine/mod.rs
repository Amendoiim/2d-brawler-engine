//! Core engine systems and modules

pub mod renderer;
pub mod physics;
pub mod audio;
pub mod input;
pub mod ecs;
pub mod scene;
pub mod asset;
pub mod animation;
pub mod particles;
pub mod level;
pub mod localization;
pub mod visual;

use anyhow::Result;
use winit::{
    event::WindowEvent,
    window::{Window, WindowId},
};

use crate::platform::Platform;

/// Main engine struct that coordinates all systems
pub struct Engine {
    platform: Platform,
    renderer: renderer::Renderer,
    physics: physics::PhysicsWorld,
    audio: audio::AudioEngine,
    input: input::InputManager,
    ecs: ecs::World,
    scene: scene::SceneManager,
    asset_manager: asset::AssetManager,
    animation: animation::AnimationSystem,
    particles: particles::ParticleSystem,
    level_generator: level::LevelGenerator,
    localization: localization::manager::GameLocalizationManager,
    visual_effects: visual::VisualEffectsManager,
}

impl Engine {
    /// Create a new engine instance
    pub fn new(window: Window) -> Result<Self> {
        let window_id = window.id();
        
        // Initialize platform abstraction
        let platform = Platform::new(window)?;
        
        // Initialize core systems
        let renderer = renderer::Renderer::new(&platform)?;
        let physics = physics::PhysicsWorld::new()?;
        let audio = audio::AudioEngine::new()?;
        let input = input::InputManager::new(window_id)?;
        let ecs = ecs::World::new();
        let scene = scene::SceneManager::new();
        let asset_manager = asset::AssetManager::new()?;
        let animation = animation::AnimationSystem::new();
        let particles = particles::ParticleSystem::new();
        let level_generator = level::LevelGenerator::new();
        let localization = localization::manager::GameLocalizationManager::new();
        let visual_effects = visual::VisualEffectsManager::new();

        Ok(Self {
            platform,
            renderer,
            physics,
            audio,
            input,
            ecs,
            scene,
            asset_manager,
            animation,
            particles,
            level_generator,
            localization,
            visual_effects,
        })
    }

    /// Get reference to the window
    pub fn window(&self) -> &Window {
        self.platform.window()
    }

    /// Handle window events
    pub fn handle_window_event(&mut self, event: WindowEvent) {
        self.input.handle_window_event(event);
    }

    /// Resize the rendering surface
    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(size);
    }

    /// Main render function
    pub fn render(&mut self) -> Result<()> {
        // Update timing
        self.platform.update_timing();
        
        // Update systems
        self.update_internal()?;
        
        // Render frame
        self.renderer.render(&self.ecs)?;
        
        Ok(())
    }

    /// Get mutable reference to ECS world
    pub fn ecs_mut(&mut self) -> &mut ecs::World {
        &mut self.ecs
    }

    /// Get mutable reference to audio engine
    pub fn audio_mut(&mut self) -> &mut audio::AudioEngine {
        &mut self.audio
    }

    /// Get mutable reference to asset manager
    pub fn asset_manager_mut(&mut self) -> &mut asset::AssetManager {
        &mut self.asset_manager
    }

    /// Get mutable reference to scene manager
    pub fn scene_mut(&mut self) -> &mut scene::SceneManager {
        &mut self.scene
    }

    /// Get mutable reference to animation system
    pub fn animation_mut(&mut self) -> &mut animation::AnimationSystem {
        &mut self.animation
    }

    /// Get mutable reference to particle system
    pub fn particles_mut(&mut self) -> &mut particles::ParticleSystem {
        &mut self.particles
    }

    /// Get mutable reference to level generator
    pub fn level_generator_mut(&mut self) -> &mut level::LevelGenerator {
        &mut self.level_generator
    }

    /// Get mutable reference to localization manager
    pub fn localization_mut(&mut self) -> &mut localization::manager::GameLocalizationManager {
        &mut self.localization
    }

    /// Get reference to localization manager
    pub fn localization(&self) -> &localization::manager::GameLocalizationManager {
        &self.localization
    }

    /// Get mutable reference to visual effects manager
    pub fn visual_effects_mut(&mut self) -> &mut visual::VisualEffectsManager {
        &mut self.visual_effects
    }

    /// Get reference to visual effects manager
    pub fn visual_effects(&self) -> &visual::VisualEffectsManager {
        &self.visual_effects
    }

    /// Load a scene
    pub fn load_scene(&mut self, name: &str) -> Result<()> {
        self.scene.load_scene(name, &mut self.ecs)
    }

    /// Update all engine systems with given delta time
    pub fn update(&mut self, dt: f32) -> Result<()> {
        // Update input
        self.input.update();
        
        // Update physics
        self.physics.step(dt);
        
        // Update audio
        self.audio.update(dt);
        
        // Update asset manager
        self.asset_manager.update(dt)?;
        
        // Update ECS systems
        self.ecs.update(dt);
        
        // Update animation system
        self.animation.update(&mut self.ecs, dt);
        
        // Update particle system (commented out for now)
        // self.particles.update(&self.ecs, dt);
        
        // Update visual effects
        self.visual_effects.update(0.016); // 60 FPS delta time
        
        // Update scene
        self.scene.update(dt, &mut self.ecs);
        
        Ok(())
    }

    /// Update all engine systems (internal method)
    fn update_internal(&mut self) -> Result<()> {
        let dt = self.platform.delta_time();
        self.update(dt)
    }
}
