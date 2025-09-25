//! Biome transition system for seamless level generation

use crate::engine::level::{Level, Room, RoomType, Tile};
use crate::engine::level::biome::{Biome, BiomeManager};
use glam::Vec2;
use std::collections::HashMap;

/// A biome transition area between two different biomes
#[derive(Debug, Clone)]
pub struct BiomeTransition {
    /// Starting biome
    pub from_biome: String,
    /// Ending biome
    pub to_biome: String,
    /// Transition area bounds
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    /// Transition type
    pub transition_type: TransitionType,
    /// Blend factor (0.0 = from_biome, 1.0 = to_biome)
    pub blend_factor: f32,
}

/// Types of biome transitions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransitionType {
    /// Gradual transition with mixed tiles
    Gradual,
    /// Sharp transition with a clear boundary
    Sharp,
    /// Transition through a corridor or tunnel
    Corridor,
    /// Transition through a special room
    SpecialRoom,
}

/// Biome transition manager
pub struct BiomeTransitionManager {
    /// Biome manager for accessing biome data
    pub biome_manager: BiomeManager,
    /// Transition templates
    pub transition_templates: HashMap<String, TransitionTemplate>,
}

/// Template for creating biome transitions
#[derive(Debug, Clone)]
pub struct TransitionTemplate {
    /// Template name
    pub name: String,
    /// Transition type
    pub transition_type: TransitionType,
    /// Minimum width for this transition
    pub min_width: u32,
    /// Minimum height for this transition
    pub min_height: u32,
    /// Tile blending pattern
    pub blending_pattern: BlendingPattern,
}

/// Pattern for blending tiles between biomes
#[derive(Debug, Clone)]
pub enum BlendingPattern {
    /// Linear blend from one biome to another
    Linear,
    /// Checkerboard pattern
    Checkerboard,
    /// Random scattered tiles
    Random,
    /// Gradient pattern
    Gradient,
    /// Custom pattern with specific tile placements
    Custom(Vec<Vec<f32>>),
}

impl BiomeTransitionManager {
    /// Create a new biome transition manager
    pub fn new() -> Self {
        let mut manager = Self {
            biome_manager: BiomeManager::new(),
            transition_templates: HashMap::new(),
        };
        manager.initialize_transition_templates();
        manager
    }

    /// Initialize default transition templates
    fn initialize_transition_templates(&mut self) {
        // Gradual transition template
        let gradual_template = TransitionTemplate {
            name: "gradual".to_string(),
            transition_type: TransitionType::Gradual,
            min_width: 3,
            min_height: 3,
            blending_pattern: BlendingPattern::Linear,
        };
        self.transition_templates.insert("gradual".to_string(), gradual_template);

        // Sharp transition template
        let sharp_template = TransitionTemplate {
            name: "sharp".to_string(),
            transition_type: TransitionType::Sharp,
            min_width: 1,
            min_height: 1,
            blending_pattern: BlendingPattern::Linear,
        };
        self.transition_templates.insert("sharp".to_string(), sharp_template);

        // Corridor transition template
        let corridor_template = TransitionTemplate {
            name: "corridor".to_string(),
            transition_type: TransitionType::Corridor,
            min_width: 2,
            min_height: 5,
            blending_pattern: BlendingPattern::Gradient,
        };
        self.transition_templates.insert("corridor".to_string(), corridor_template);

        // Special room transition template
        let special_room_template = TransitionTemplate {
            name: "special_room".to_string(),
            transition_type: TransitionType::SpecialRoom,
            min_width: 5,
            min_height: 5,
            blending_pattern: BlendingPattern::Random,
        };
        self.transition_templates.insert("special_room".to_string(), special_room_template);
    }

    /// Create a biome transition between two biomes
    pub fn create_transition(
        &self,
        from_biome: &str,
        to_biome: &str,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        transition_type: TransitionType,
    ) -> Result<BiomeTransition, String> {
        // Validate biomes exist
        if self.biome_manager.get_biome(from_biome).is_none() {
            return Err(format!("Unknown from_biome: {}", from_biome));
        }
        if self.biome_manager.get_biome(to_biome).is_none() {
            return Err(format!("Unknown to_biome: {}", to_biome));
        }

        // Get appropriate template
        let template_name = match transition_type {
            TransitionType::Gradual => "gradual",
            TransitionType::Sharp => "sharp",
            TransitionType::Corridor => "corridor",
            TransitionType::SpecialRoom => "special_room",
        };

        let template = self.transition_templates.get(template_name)
            .ok_or_else(|| format!("No template found for transition type: {:?}", transition_type))?;

        // Validate dimensions
        if width < template.min_width || height < template.min_height {
            return Err(format!(
                "Transition dimensions {}x{} are too small for template {} (min: {}x{})",
                width, height, template.name, template.min_width, template.min_height
            ));
        }

        Ok(BiomeTransition {
            from_biome: from_biome.to_string(),
            to_biome: to_biome.to_string(),
            x,
            y,
            width,
            height,
            transition_type,
            blend_factor: 0.5, // Default to middle blend
        })
    }

    /// Apply a biome transition to a level
    pub fn apply_transition(&self, level: &mut Level, transition: &BiomeTransition) -> Result<(), String> {
        let from_biome = self.biome_manager.get_biome(&transition.from_biome)
            .ok_or_else(|| format!("Unknown from_biome: {}", transition.from_biome))?;
        let to_biome = self.biome_manager.get_biome(&transition.to_biome)
            .ok_or_else(|| format!("Unknown to_biome: {}", transition.to_biome))?;

        match transition.transition_type {
            TransitionType::Gradual => self.apply_gradual_transition(level, transition, from_biome, to_biome),
            TransitionType::Sharp => self.apply_sharp_transition(level, transition, from_biome, to_biome),
            TransitionType::Corridor => self.apply_corridor_transition(level, transition, from_biome, to_biome),
            TransitionType::SpecialRoom => self.apply_special_room_transition(level, transition, from_biome, to_biome),
        }
    }

    /// Apply a gradual transition
    fn apply_gradual_transition(
        &self,
        level: &mut Level,
        transition: &BiomeTransition,
        from_biome: &Biome,
        to_biome: &Biome,
    ) -> Result<(), String> {
        for x in transition.x..transition.x + transition.width {
            for y in transition.y..transition.y + transition.height {
                if x < level.width && y < level.height {
                    // Calculate blend factor based on position
                    let relative_x = (x - transition.x) as f32 / (transition.width - 1) as f32;
                    let relative_y = (y - transition.y) as f32 / (transition.height - 1) as f32;
                    let blend_factor = (relative_x + relative_y) / 2.0;

                    // Choose tile based on blend factor
                    let tile = if blend_factor < 0.5 {
                        from_biome.floor_tile
                    } else {
                        to_biome.floor_tile
                    };

                    level.tiles[x as usize][y as usize] = tile;
                }
            }
        }
        Ok(())
    }

    /// Apply a sharp transition
    fn apply_sharp_transition(
        &self,
        level: &mut Level,
        transition: &BiomeTransition,
        from_biome: &Biome,
        to_biome: &Biome,
    ) -> Result<(), String> {
        let mid_x = transition.x + transition.width / 2;
        
        for x in transition.x..transition.x + transition.width {
            for y in transition.y..transition.y + transition.height {
                if x < level.width && y < level.height {
                    let tile = if x < mid_x {
                        from_biome.floor_tile
                    } else {
                        to_biome.floor_tile
                    };

                    level.tiles[x as usize][y as usize] = tile;
                }
            }
        }
        Ok(())
    }

    /// Apply a corridor transition
    fn apply_corridor_transition(
        &self,
        level: &mut Level,
        transition: &BiomeTransition,
        from_biome: &Biome,
        to_biome: &Biome,
    ) -> Result<(), String> {
        // Create a corridor with gradient from one biome to another
        for x in transition.x..transition.x + transition.width {
            for y in transition.y..transition.y + transition.height {
                if x < level.width && y < level.height {
                    let relative_x = (x - transition.x) as f32 / (transition.width - 1) as f32;
                    
                    let tile = if relative_x < 0.3 {
                        from_biome.floor_tile
                    } else if relative_x > 0.7 {
                        to_biome.floor_tile
                    } else {
                        // Middle section uses a mix
                        if (x + y) % 2 == 0 {
                            from_biome.floor_tile
                        } else {
                            to_biome.floor_tile
                        }
                    };

                    level.tiles[x as usize][y as usize] = tile;
                }
            }
        }
        Ok(())
    }

    /// Apply a special room transition
    fn apply_special_room_transition(
        &self,
        level: &mut Level,
        transition: &BiomeTransition,
        from_biome: &Biome,
        to_biome: &Biome,
    ) -> Result<(), String> {
        // Create a special transition room with mixed elements
        for x in transition.x..transition.x + transition.width {
            for y in transition.y..transition.y + transition.height {
                if x < level.width && y < level.height {
                    // Use a checkerboard pattern for special rooms
                    let tile = if (x + y) % 2 == 0 {
                        from_biome.floor_tile
                    } else {
                        to_biome.floor_tile
                    };

                    level.tiles[x as usize][y as usize] = tile;
                }
            }
        }
        Ok(())
    }

    /// Get available transition templates
    pub fn get_transition_templates(&self) -> Vec<&TransitionTemplate> {
        self.transition_templates.values().collect()
    }

    /// Add a custom transition template
    pub fn add_transition_template(&mut self, template: TransitionTemplate) {
        self.transition_templates.insert(template.name.clone(), template);
    }
}

impl Default for BiomeTransitionManager {
    fn default() -> Self {
        Self::new()
    }
}
