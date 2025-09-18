//! 2D rendering system

use anyhow::Result;
use winit::dpi::PhysicalSize;
use glam::Mat4;

use crate::engine::ecs::World;
use crate::platform::Platform;

pub mod sprite;

/// 2D renderer (simplified for Phase 1)
pub struct Renderer {
    size: PhysicalSize<u32>,
    camera: Camera,
}

/// Simple 2D camera
pub struct Camera {
    pub position: glam::Vec2,
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: glam::Vec2::ZERO,
            zoom: 1.0,
        }
    }

    pub fn view_projection(&self, screen_width: f32, screen_height: f32) -> Mat4 {
        let scale = 2.0 / self.zoom;
        let translation = glam::Vec3::new(-self.position.x, -self.position.y, 0.0);
        
        Mat4::from_scale_rotation_translation(
            glam::Vec3::new(scale / screen_width, scale / screen_height, 1.0),
            glam::Quat::IDENTITY,
            translation,
        )
    }
}

impl Renderer {
    /// Create a new renderer
    pub fn new(platform: &Platform) -> Result<Self> {
        let window = platform.window();
        let size = window.inner_size();

        // For Phase 1, we'll create a simplified renderer
        // TODO: Implement proper WGPU integration in Phase 2
        let camera = Camera::new();

        Ok(Self {
            size,
            camera,
        })
    }

    /// Resize the rendering surface
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.size = size;
        }
    }

    /// Render a frame
    pub fn render(&mut self, world: &World) -> Result<()> {
        // For Phase 1, we'll just log that rendering would happen
        // TODO: Implement actual rendering in Phase 2
        log::debug!("Rendering frame");
        Ok(())
    }
}
