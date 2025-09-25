//! Background layers system for parallax scrolling and atmospheric depth

use crate::engine::level::biome::{Biome, BiomeManager};
use glam::Vec2;
use std::collections::HashMap;

/// A background layer for parallax scrolling
#[derive(Debug, Clone)]
pub struct BackgroundLayer {
    /// Layer identifier
    pub id: String,
    /// Layer depth (0.0 = closest to camera, 1.0 = farthest)
    pub depth: f32,
    /// Layer position
    pub position: Vec2,
    /// Layer velocity for scrolling
    pub velocity: Vec2,
    /// Layer scale
    pub scale: Vec2,
    /// Layer opacity
    pub opacity: f32,
    /// Layer texture ID
    pub texture_id: u32,
    /// Layer color tint
    pub color: [f32; 4],
    /// Whether the layer is visible
    pub visible: bool,
    /// Whether the layer repeats horizontally
    pub repeat_x: bool,
    /// Whether the layer repeats vertically
    pub repeat_y: bool,
    /// Layer dimensions
    pub width: f32,
    pub height: f32,
    /// Layer type
    pub layer_type: BackgroundLayerType,
}

/// Types of background layers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackgroundLayerType {
    /// Static background layer
    Static,
    /// Scrolling background layer
    Scrolling,
    /// Animated background layer
    Animated,
    /// Parallax background layer
    Parallax,
    /// Atmospheric layer (fog, clouds, etc.)
    Atmospheric,
    /// Decorative layer
    Decorative,
}

/// Background layer manager
pub struct BackgroundLayerManager {
    /// Biome manager for biome-specific backgrounds
    pub biome_manager: BiomeManager,
    /// Background layers
    pub layers: Vec<BackgroundLayer>,
    /// Layer templates
    pub layer_templates: HashMap<String, BackgroundLayerTemplate>,
    /// Camera position for parallax calculations
    pub camera_position: Vec2,
    /// Camera velocity for smooth scrolling
    pub camera_velocity: Vec2,
}

/// Template for creating background layers
#[derive(Debug, Clone)]
pub struct BackgroundLayerTemplate {
    /// Template name
    pub name: String,
    /// Layer type
    pub layer_type: BackgroundLayerType,
    /// Default depth
    pub depth: f32,
    /// Default scale
    pub scale: Vec2,
    /// Default opacity
    pub opacity: f32,
    /// Default color
    pub color: [f32; 4],
    /// Texture ID
    pub texture_id: u32,
    /// Scroll speed multiplier
    pub scroll_speed: f32,
    /// Whether layer repeats
    pub repeat_x: bool,
    pub repeat_y: bool,
    /// Biome-specific settings
    pub biome_settings: HashMap<String, BiomeLayerSettings>,
}

/// Biome-specific settings for background layers
#[derive(Debug, Clone)]
pub struct BiomeLayerSettings {
    /// Texture ID for this biome
    pub texture_id: u32,
    /// Color tint for this biome
    pub color: [f32; 4],
    /// Opacity for this biome
    pub opacity: f32,
    /// Scroll speed for this biome
    pub scroll_speed: f32,
}

impl BackgroundLayerManager {
    /// Create a new background layer manager
    pub fn new() -> Self {
        let mut manager = Self {
            biome_manager: BiomeManager::new(),
            layers: Vec::new(),
            layer_templates: HashMap::new(),
            camera_position: Vec2::ZERO,
            camera_velocity: Vec2::ZERO,
        };
        manager.initialize_layer_templates();
        manager
    }

    /// Initialize default layer templates
    fn initialize_layer_templates(&mut self) {
        // Sky layer template
        let sky_template = BackgroundLayerTemplate {
            name: "sky".to_string(),
            layer_type: BackgroundLayerType::Static,
            depth: 1.0,
            scale: Vec2::new(1.0, 1.0),
            opacity: 1.0,
            color: [0.5, 0.7, 1.0, 1.0],
            texture_id: 3000,
            scroll_speed: 0.0,
            repeat_x: true,
            repeat_y: false,
            biome_settings: {
                let mut settings = HashMap::new();
                settings.insert("forest".to_string(), BiomeLayerSettings {
                    texture_id: 3000,
                    color: [0.3, 0.6, 0.9, 1.0],
                    opacity: 1.0,
                    scroll_speed: 0.0,
                });
                settings.insert("desert".to_string(), BiomeLayerSettings {
                    texture_id: 3001,
                    color: [1.0, 0.8, 0.6, 1.0],
                    opacity: 1.0,
                    scroll_speed: 0.0,
                });
                settings.insert("arctic".to_string(), BiomeLayerSettings {
                    texture_id: 3002,
                    color: [0.8, 0.9, 1.0, 1.0],
                    opacity: 1.0,
                    scroll_speed: 0.0,
                });
                settings.insert("cave".to_string(), BiomeLayerSettings {
                    texture_id: 3003,
                    color: [0.1, 0.1, 0.1, 1.0],
                    opacity: 1.0,
                    scroll_speed: 0.0,
                });
                settings.insert("lava".to_string(), BiomeLayerSettings {
                    texture_id: 3004,
                    color: [0.8, 0.2, 0.0, 1.0],
                    opacity: 1.0,
                    scroll_speed: 0.0,
                });
                settings
            },
        };
        self.layer_templates.insert("sky".to_string(), sky_template);

        // Mountains layer template
        let mountains_template = BackgroundLayerTemplate {
            name: "mountains".to_string(),
            layer_type: BackgroundLayerType::Parallax,
            depth: 0.8,
            scale: Vec2::new(1.0, 1.0),
            opacity: 0.9,
            color: [0.6, 0.6, 0.6, 1.0],
            texture_id: 3010,
            scroll_speed: 0.3,
            repeat_x: true,
            repeat_y: false,
            biome_settings: {
                let mut settings = HashMap::new();
                settings.insert("forest".to_string(), BiomeLayerSettings {
                    texture_id: 3010,
                    color: [0.4, 0.5, 0.3, 1.0],
                    opacity: 0.9,
                    scroll_speed: 0.3,
                });
                settings.insert("desert".to_string(), BiomeLayerSettings {
                    texture_id: 3011,
                    color: [0.8, 0.7, 0.5, 1.0],
                    opacity: 0.9,
                    scroll_speed: 0.3,
                });
                settings.insert("arctic".to_string(), BiomeLayerSettings {
                    texture_id: 3012,
                    color: [0.7, 0.8, 0.9, 1.0],
                    opacity: 0.9,
                    scroll_speed: 0.3,
                });
                settings.insert("cave".to_string(), BiomeLayerSettings {
                    texture_id: 3013,
                    color: [0.2, 0.2, 0.2, 1.0],
                    opacity: 0.9,
                    scroll_speed: 0.3,
                });
                settings.insert("lava".to_string(), BiomeLayerSettings {
                    texture_id: 3014,
                    color: [0.6, 0.3, 0.1, 1.0],
                    opacity: 0.9,
                    scroll_speed: 0.3,
                });
                settings
            },
        };
        self.layer_templates.insert("mountains".to_string(), mountains_template);

        // Clouds layer template
        let clouds_template = BackgroundLayerTemplate {
            name: "clouds".to_string(),
            layer_type: BackgroundLayerType::Scrolling,
            depth: 0.6,
            scale: Vec2::new(1.0, 1.0),
            opacity: 0.7,
            color: [1.0, 1.0, 1.0, 1.0],
            texture_id: 3020,
            scroll_speed: 0.1,
            repeat_x: true,
            repeat_y: false,
            biome_settings: {
                let mut settings = HashMap::new();
                settings.insert("forest".to_string(), BiomeLayerSettings {
                    texture_id: 3020,
                    color: [0.9, 0.9, 0.9, 0.7],
                    opacity: 0.7,
                    scroll_speed: 0.1,
                });
                settings.insert("desert".to_string(), BiomeLayerSettings {
                    texture_id: 3021,
                    color: [1.0, 0.9, 0.8, 0.5],
                    opacity: 0.5,
                    scroll_speed: 0.2,
                });
                settings.insert("arctic".to_string(), BiomeLayerSettings {
                    texture_id: 3022,
                    color: [0.8, 0.9, 1.0, 0.8],
                    opacity: 0.8,
                    scroll_speed: 0.05,
                });
                settings.insert("cave".to_string(), BiomeLayerSettings {
                    texture_id: 3023,
                    color: [0.3, 0.3, 0.3, 0.3],
                    opacity: 0.3,
                    scroll_speed: 0.05,
                });
                settings.insert("lava".to_string(), BiomeLayerSettings {
                    texture_id: 3024,
                    color: [0.8, 0.4, 0.2, 0.6],
                    opacity: 0.6,
                    scroll_speed: 0.15,
                });
                settings
            },
        };
        self.layer_templates.insert("clouds".to_string(), clouds_template);

        // Atmospheric layer template
        let atmospheric_template = BackgroundLayerTemplate {
            name: "atmospheric".to_string(),
            layer_type: BackgroundLayerType::Atmospheric,
            depth: 0.4,
            scale: Vec2::new(1.0, 1.0),
            opacity: 0.3,
            color: [0.5, 0.5, 0.5, 1.0],
            texture_id: 3030,
            scroll_speed: 0.05,
            repeat_x: true,
            repeat_y: true,
            biome_settings: {
                let mut settings = HashMap::new();
                settings.insert("forest".to_string(), BiomeLayerSettings {
                    texture_id: 3030,
                    color: [0.2, 0.4, 0.2, 0.3],
                    opacity: 0.3,
                    scroll_speed: 0.05,
                });
                settings.insert("desert".to_string(), BiomeLayerSettings {
                    texture_id: 3031,
                    color: [0.8, 0.6, 0.4, 0.4],
                    opacity: 0.4,
                    scroll_speed: 0.1,
                });
                settings.insert("arctic".to_string(), BiomeLayerSettings {
                    texture_id: 3032,
                    color: [0.6, 0.7, 0.8, 0.2],
                    opacity: 0.2,
                    scroll_speed: 0.02,
                });
                settings.insert("cave".to_string(), BiomeLayerSettings {
                    texture_id: 3033,
                    color: [0.1, 0.1, 0.1, 0.5],
                    opacity: 0.5,
                    scroll_speed: 0.01,
                });
                settings.insert("lava".to_string(), BiomeLayerSettings {
                    texture_id: 3034,
                    color: [0.6, 0.2, 0.0, 0.4],
                    opacity: 0.4,
                    scroll_speed: 0.08,
                });
                settings
            },
        };
        self.layer_templates.insert("atmospheric".to_string(), atmospheric_template);
    }

    /// Create background layers for a biome
    pub fn create_biome_background(&mut self, biome: &str, screen_width: f32, screen_height: f32) -> Result<(), String> {
        self.layers.clear();

        // Create layers based on templates
        for (template_name, template) in &self.layer_templates {
            let layer = self.create_layer_from_template(template, biome, screen_width, screen_height)?;
            self.layers.push(layer);
        }

        // Sort layers by depth (farthest first)
        self.layers.sort_by(|a, b| b.depth.partial_cmp(&a.depth).unwrap_or(std::cmp::Ordering::Equal));

        Ok(())
    }

    /// Create a layer from a template
    fn create_layer_from_template(
        &self,
        template: &BackgroundLayerTemplate,
        biome: &str,
        screen_width: f32,
        screen_height: f32,
    ) -> Result<BackgroundLayer, String> {
        // Get biome-specific settings
        let biome_settings = template.biome_settings.get(biome)
            .or_else(|| template.biome_settings.get("default"))
            .ok_or_else(|| format!("No biome settings found for biome: {}", biome))?;

        Ok(BackgroundLayer {
            id: format!("{}_{}", template.name, biome),
            depth: template.depth,
            position: Vec2::ZERO,
            velocity: Vec2::new(biome_settings.scroll_speed, 0.0),
            scale: template.scale,
            opacity: biome_settings.opacity,
            texture_id: biome_settings.texture_id,
            color: biome_settings.color,
            visible: true,
            repeat_x: template.repeat_x,
            repeat_y: template.repeat_y,
            width: screen_width,
            height: screen_height,
            layer_type: template.layer_type,
        })
    }

    /// Update background layers
    pub fn update_layers(&mut self, dt: f32) {
        for layer in &mut self.layers {
            match layer.layer_type {
                BackgroundLayerType::Scrolling | BackgroundLayerType::Parallax => {
                    // Update position based on velocity and camera movement
                    let parallax_factor = 1.0 - layer.depth;
                    layer.position += layer.velocity * dt;
                    layer.position += self.camera_velocity * parallax_factor * dt;
                },
                BackgroundLayerType::Animated => {
                    // Update animated layers (placeholder for future animation system)
                    layer.position += layer.velocity * dt;
                },
                _ => {
                    // Static layers don't move
                }
            }

            // Handle layer repetition
            if layer.repeat_x {
                if layer.position.x > layer.width {
                    layer.position.x -= layer.width;
                } else if layer.position.x < -layer.width {
                    layer.position.x += layer.width;
                }
            }

            if layer.repeat_y {
                if layer.position.y > layer.height {
                    layer.position.y -= layer.height;
                } else if layer.position.y < -layer.height {
                    layer.position.y += layer.height;
                }
            }
        }
    }

    /// Set camera position for parallax calculations
    pub fn set_camera_position(&mut self, position: Vec2) {
        self.camera_position = position;
    }

    /// Set camera velocity for smooth scrolling
    pub fn set_camera_velocity(&mut self, velocity: Vec2) {
        self.camera_velocity = velocity;
    }

    /// Get all background layers
    pub fn get_layers(&self) -> &[BackgroundLayer] {
        &self.layers
    }

    /// Get visible layers
    pub fn get_visible_layers(&self) -> Vec<&BackgroundLayer> {
        self.layers.iter().filter(|layer| layer.visible).collect()
    }

    /// Get layers by type
    pub fn get_layers_by_type(&self, layer_type: BackgroundLayerType) -> Vec<&BackgroundLayer> {
        self.layers.iter().filter(|layer| layer.layer_type == layer_type).collect()
    }

    /// Set layer visibility
    pub fn set_layer_visibility(&mut self, layer_id: &str, visible: bool) -> Result<(), String> {
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == layer_id) {
            layer.visible = visible;
            Ok(())
        } else {
            Err(format!("Layer with ID {} not found", layer_id))
        }
    }

    /// Set layer opacity
    pub fn set_layer_opacity(&mut self, layer_id: &str, opacity: f32) -> Result<(), String> {
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == layer_id) {
            layer.opacity = opacity.clamp(0.0, 1.0);
            Ok(())
        } else {
            Err(format!("Layer with ID {} not found", layer_id))
        }
    }

    /// Add a custom layer template
    pub fn add_layer_template(&mut self, template: BackgroundLayerTemplate) {
        self.layer_templates.insert(template.name.clone(), template);
    }

    /// Get layer templates
    pub fn get_layer_templates(&self) -> &HashMap<String, BackgroundLayerTemplate> {
        &self.layer_templates
    }
}

impl Default for BackgroundLayerManager {
    fn default() -> Self {
        Self::new()
    }
}
