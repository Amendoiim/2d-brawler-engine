//! Animation rendering system

use crate::engine::animation::{AnimationFrame, AnimationBlend};
use crate::engine::ecs::{Component, Entity, World};
use glam::Vec2;
use std::collections::HashMap;

/// Animation renderer for drawing animated sprites
pub struct AnimationRenderer {
    /// Texture cache for loaded textures
    pub texture_cache: HashMap<u32, TextureData>,
    /// Render batch for efficient drawing
    pub render_batch: Vec<RenderCommand>,
    /// Camera transform
    pub camera_transform: Transform,
    /// Render settings
    pub settings: RenderSettings,
}

/// Texture data for rendering
#[derive(Debug, Clone)]
pub struct TextureData {
    /// Texture ID
    pub id: u32,
    /// Texture width
    pub width: u32,
    /// Texture height: u32,
    /// Texture format
    pub format: TextureFormat,
    /// Texture data
    pub data: Vec<u8>,
}

/// Texture format
#[derive(Debug, Clone)]
pub enum TextureFormat {
    Rgba8,
    Rgb8,
    R8,
}

/// Transform for positioning and scaling
#[derive(Debug, Clone)]
pub struct Transform {
    /// Position
    pub position: Vec2,
    /// Scale
    pub scale: Vec2,
    /// Rotation in radians
    pub rotation: f32,
}

/// Render command for batching
#[derive(Debug, Clone)]
pub struct RenderCommand {
    /// Texture ID
    pub texture_id: u32,
    /// Source rectangle
    pub src_rect: [f32; 4], // x, y, width, height
    /// Destination rectangle
    pub dst_rect: [f32; 4], // x, y, width, height
    /// Color tint
    pub color: [f32; 4],
    /// Transform
    pub transform: Transform,
    /// Z-order for depth sorting
    pub z_order: f32,
}

/// Render settings
#[derive(Debug, Clone)]
pub struct RenderSettings {
    /// Enable depth sorting
    pub enable_depth_sort: bool,
    /// Enable texture filtering
    pub enable_filtering: bool,
    /// Maximum render commands per frame
    pub max_commands: usize,
    /// Enable debug rendering
    pub enable_debug: bool,
}

impl AnimationRenderer {
    /// Create a new animation renderer
    pub fn new() -> Self {
        Self {
            texture_cache: HashMap::new(),
            render_batch: Vec::new(),
            camera_transform: Transform {
                position: Vec2::ZERO,
                scale: Vec2::ONE,
                rotation: 0.0,
            },
            settings: RenderSettings {
                enable_depth_sort: true,
                enable_filtering: true,
                max_commands: 10000,
                enable_debug: false,
            },
        }
    }

    /// Load a texture into the cache
    pub fn load_texture(&mut self, id: u32, data: TextureData) {
        self.texture_cache.insert(id, data);
    }

    /// Unload a texture from the cache
    pub fn unload_texture(&mut self, id: u32) {
        self.texture_cache.remove(&id);
    }

    /// Set camera transform
    pub fn set_camera_transform(&mut self, transform: Transform) {
        self.camera_transform = transform;
    }

    /// Begin rendering frame
    pub fn begin_frame(&mut self) {
        self.render_batch.clear();
    }

    /// End rendering frame and submit commands
    pub fn end_frame(&mut self) {
        // Sort render commands by z-order if enabled
        if self.settings.enable_depth_sort {
            self.render_batch.sort_by(|a, b| a.z_order.partial_cmp(&b.z_order).unwrap());
        }

        // Submit render commands to GPU
        self.submit_render_commands();
    }

    /// Submit render commands to GPU
    fn submit_render_commands(&self) {
        // In a real implementation, this would submit commands to the GPU
        // For now, we'll just log the commands
        if self.settings.enable_debug {
            log::debug!("Submitting {} render commands", self.render_batch.len());
        }
    }

    /// Render an animation frame
    pub fn render_animation_frame(
        &mut self,
        entity: Entity,
        frame: &AnimationFrame,
        position: Vec2,
        scale: Vec2,
        rotation: f32,
        z_order: f32,
    ) {
        // Check if we have the texture
        if !self.texture_cache.contains_key(&frame.texture_id) {
            log::warn!("Texture {} not found in cache", frame.texture_id);
            return;
        }

        // Create render command
        let render_command = RenderCommand {
            texture_id: frame.texture_id,
            src_rect: [
                frame.offset_x,
                frame.offset_y,
                frame.width,
                frame.height,
            ],
            dst_rect: [
                position.x,
                position.y,
                frame.width * scale.x,
                frame.height * scale.y,
            ],
            color: frame.color,
            transform: Transform {
                position,
                scale,
                rotation,
            },
            z_order,
        };

        // Add to render batch
        if self.render_batch.len() < self.settings.max_commands {
            self.render_batch.push(render_command);
        } else {
            log::warn!("Render batch full, dropping command");
        }
    }

    /// Render a blended animation frame
    pub fn render_blended_animation_frame(
        &mut self,
        entity: Entity,
        from_frame: &AnimationFrame,
        to_frame: &AnimationFrame,
        blend_progress: f32,
        position: Vec2,
        scale: Vec2,
        rotation: f32,
        z_order: f32,
    ) {
        // Interpolate between frames
        let from_weight = 1.0 - blend_progress;
        let to_weight = blend_progress;

        // Blend colors
        let blended_color = [
            from_frame.color[0] * from_weight + to_frame.color[0] * to_weight,
            from_frame.color[1] * from_weight + to_frame.color[1] * to_weight,
            from_frame.color[2] * from_weight + to_frame.color[2] * to_weight,
            from_frame.color[3] * from_weight + to_frame.color[3] * to_weight,
        ];

        // Blend positions (if needed)
        let blended_position = position; // For now, use the same position

        // Blend scales
        let blended_scale = Vec2::new(
            scale.x * from_weight + scale.x * to_weight,
            scale.y * from_weight + scale.y * to_weight,
        );

        // Create blended render command
        let render_command = RenderCommand {
            texture_id: to_frame.texture_id, // Use target frame texture
            src_rect: [
                to_frame.offset_x,
                to_frame.offset_y,
                to_frame.width,
                to_frame.height,
            ],
            dst_rect: [
                blended_position.x,
                blended_position.y,
                to_frame.width * blended_scale.x,
                to_frame.height * blended_scale.y,
            ],
            color: blended_color,
            transform: Transform {
                position: blended_position,
                scale: blended_scale,
                rotation,
            },
            z_order,
        };

        // Add to render batch
        if self.render_batch.len() < self.settings.max_commands {
            self.render_batch.push(render_command);
        }
    }

    /// Render all animated entities
    pub fn render_animated_entities(&mut self, world: &World, animation_system: &crate::engine::animation::AnimationSystem) {
        // Get all entities with position and sprite components
        let entities = world.query::<crate::game::Position>();
        
        for entity in entities {
            // Get position
            if let Some(position) = world.get_component::<crate::game::Position>(entity) {
                // Get current animation frame
                if let Some(frame) = animation_system.get_current_frame(entity) {
                    // Get sprite component for scale (using default scale for now)
                    let scale = Vec2::ONE;

                    // Render the frame
                    self.render_animation_frame(
                        entity,
                        frame,
                        Vec2::new(position.x, position.y),
                        scale,
                        0.0, // rotation
                        0.0, // z_order
                    );
                }
            }
        }
    }

    /// Get render statistics
    pub fn get_render_stats(&self) -> RenderStats {
        RenderStats {
            commands_this_frame: self.render_batch.len(),
            textures_loaded: self.texture_cache.len(),
            memory_usage: self.calculate_memory_usage(),
        }
    }

    /// Calculate memory usage
    fn calculate_memory_usage(&self) -> usize {
        let mut total = 0;
        for texture in self.texture_cache.values() {
            total += texture.data.len();
        }
        total
    }
}

/// Render statistics
#[derive(Debug, Clone)]
pub struct RenderStats {
    /// Number of render commands this frame
    pub commands_this_frame: usize,
    /// Number of textures loaded
    pub textures_loaded: usize,
    /// Memory usage in bytes
    pub memory_usage: usize,
}

impl Default for AnimationRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            scale: Vec2::ONE,
            rotation: 0.0,
        }
    }
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            enable_depth_sort: true,
            enable_filtering: true,
            max_commands: 10000,
            enable_debug: false,
        }
    }
}
