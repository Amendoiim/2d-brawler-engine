//! Core engine systems and modules

pub mod renderer;
pub mod physics;
pub mod audio;
pub mod input;
pub mod ecs;
pub mod scene;
pub mod asset;

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

        Ok(Self {
            platform,
            renderer,
            physics,
            audio,
            input,
            ecs,
            scene,
            asset_manager,
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
        self.update()?;
        
        // Render frame
        self.renderer.render(&self.ecs)?;
        
        Ok(())
    }

    /// Get mutable reference to ECS world
    pub fn ecs_mut(&mut self) -> &mut ecs::World {
        &mut self.ecs
    }

    /// Update all engine systems
    fn update(&mut self) -> Result<()> {
        let dt = self.platform.delta_time();
        
        // Update input
        self.input.update();
        
        // Update physics
        self.physics.step(dt);
        
        // Update audio
        self.audio.update(dt);
        
        // Update ECS systems
        self.ecs.update(dt);
        
        // Update scene
        self.scene.update(dt, &mut self.ecs);
        
        Ok(())
    }
}
