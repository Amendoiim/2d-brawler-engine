//! Biome system for different level themes and environments

use std::collections::HashMap;
use crate::engine::level::tile::Tile;

/// A biome defines the visual and gameplay characteristics of a level
#[derive(Debug, Clone)]
pub struct Biome {
    /// Unique identifier for the biome
    pub id: String,
    /// Display name of the biome
    pub name: String,
    /// Description of the biome
    pub description: String,
    /// Primary floor tile for this biome
    pub floor_tile: Tile,
    /// Primary wall tile for this biome
    pub wall_tile: Tile,
    /// Decorative tiles that can appear
    pub decorative_tiles: Vec<Tile>,
    /// Color palette for the biome
    pub color_palette: ColorPalette,
    /// Environmental effects
    pub effects: Vec<EnvironmentalEffect>,
    /// Difficulty modifier
    pub difficulty_modifier: f32,
    /// Spawn rate modifiers for different entity types
    pub spawn_modifiers: HashMap<String, f32>,
}

/// Color palette for a biome
#[derive(Debug, Clone)]
pub struct ColorPalette {
    /// Primary color
    pub primary: [f32; 4],
    /// Secondary color
    pub secondary: [f32; 4],
    /// Accent color
    pub accent: [f32; 4],
    /// Background color
    pub background: [f32; 4],
    /// Ambient lighting color
    pub ambient: [f32; 4],
}

/// Environmental effects that can occur in a biome
#[derive(Debug, Clone)]
pub enum EnvironmentalEffect {
    /// Rain effect
    Rain { intensity: f32 },
    /// Snow effect
    Snow { intensity: f32 },
    /// Fog effect
    Fog { density: f32 },
    /// Wind effect
    Wind { strength: f32 },
    /// Dust effect
    Dust { density: f32 },
    /// Steam effect
    Steam { density: f32 },
    /// Glow effect
    Glow { intensity: f32 },
    /// Darkness effect
    Darkness { level: f32 },
}

impl Biome {
    /// Create a new biome
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            floor_tile: Tile::Floor,
            wall_tile: Tile::Wall,
            decorative_tiles: Vec::new(),
            color_palette: ColorPalette::default(),
            effects: Vec::new(),
            difficulty_modifier: 1.0,
            spawn_modifiers: HashMap::new(),
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Set the primary tiles
    pub fn with_tiles(mut self, floor_tile: Tile, wall_tile: Tile) -> Self {
        self.floor_tile = floor_tile;
        self.wall_tile = wall_tile;
        self
    }

    /// Add decorative tiles
    pub fn with_decorative_tiles(mut self, tiles: Vec<Tile>) -> Self {
        self.decorative_tiles = tiles;
        self
    }

    /// Set the color palette
    pub fn with_color_palette(mut self, palette: ColorPalette) -> Self {
        self.color_palette = palette;
        self
    }

    /// Add environmental effects
    pub fn with_effects(mut self, effects: Vec<EnvironmentalEffect>) -> Self {
        self.effects = effects;
        self
    }

    /// Set difficulty modifier
    pub fn with_difficulty_modifier(mut self, modifier: f32) -> Self {
        self.difficulty_modifier = modifier;
        self
    }

    /// Set spawn rate modifiers
    pub fn with_spawn_modifiers(mut self, modifiers: HashMap<String, f32>) -> Self {
        self.spawn_modifiers = modifiers;
        self
    }

    /// Get a random decorative tile
    pub fn random_decorative_tile(&self, rng: &mut fastrand::Rng) -> Option<Tile> {
        if self.decorative_tiles.is_empty() {
            None
        } else {
            let index = rng.usize(0..self.decorative_tiles.len());
            Some(self.decorative_tiles[index])
        }
    }

    /// Get spawn rate modifier for an entity type
    pub fn get_spawn_modifier(&self, entity_type: &str) -> f32 {
        self.spawn_modifiers.get(entity_type).copied().unwrap_or(1.0)
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: [1.0, 1.0, 1.0, 1.0],
            secondary: [0.8, 0.8, 0.8, 1.0],
            accent: [0.6, 0.6, 0.6, 1.0],
            background: [0.2, 0.2, 0.2, 1.0],
            ambient: [0.5, 0.5, 0.5, 1.0],
        }
    }
}

/// Biome manager for handling different biome types
pub struct BiomeManager {
    /// Available biomes
    pub biomes: HashMap<String, Biome>,
}

impl BiomeManager {
    /// Create a new biome manager
    pub fn new() -> Self {
        let mut manager = Self {
            biomes: HashMap::new(),
        };
        manager.initialize_default_biomes();
        manager
    }

    /// Initialize default biomes
    fn initialize_default_biomes(&mut self) {
        // Default biome
        let default_biome = Biome::new("default".to_string(), "Default".to_string())
            .with_description("A standard environment".to_string())
            .with_tiles(Tile::Floor, Tile::Wall)
            .with_decorative_tiles(vec![Tile::Grass, Tile::Stone])
            .with_color_palette(ColorPalette {
                primary: [0.8, 0.8, 0.8, 1.0],
                secondary: [0.6, 0.6, 0.6, 1.0],
                accent: [0.4, 0.4, 0.4, 1.0],
                background: [0.2, 0.2, 0.2, 1.0],
                ambient: [0.5, 0.5, 0.5, 1.0],
            })
            .with_difficulty_modifier(1.0);

        self.biomes.insert("default".to_string(), default_biome);

        // Forest biome
        let forest_biome = Biome::new("forest".to_string(), "Forest".to_string())
            .with_description("A lush forest environment".to_string())
            .with_tiles(Tile::Grass, Tile::Wood)
            .with_decorative_tiles(vec![Tile::Grass, Tile::Wood, Tile::Pillar])
            .with_color_palette(ColorPalette {
                primary: [0.2, 0.6, 0.2, 1.0],
                secondary: [0.1, 0.4, 0.1, 1.0],
                accent: [0.3, 0.7, 0.3, 1.0],
                background: [0.1, 0.2, 0.1, 1.0],
                ambient: [0.3, 0.5, 0.3, 1.0],
            })
            .with_effects(vec![
                EnvironmentalEffect::Wind { strength: 0.3 },
                EnvironmentalEffect::Fog { density: 0.2 },
            ])
            .with_difficulty_modifier(0.8);

        self.biomes.insert("forest".to_string(), forest_biome);

        // Desert biome
        let desert_biome = Biome::new("desert".to_string(), "Desert".to_string())
            .with_description("A harsh desert environment".to_string())
            .with_tiles(Tile::Sand, Tile::Stone)
            .with_decorative_tiles(vec![Tile::Sand, Tile::Stone, Tile::Pillar])
            .with_color_palette(ColorPalette {
                primary: [0.9, 0.8, 0.6, 1.0],
                secondary: [0.7, 0.6, 0.4, 1.0],
                accent: [1.0, 0.9, 0.7, 1.0],
                background: [0.4, 0.3, 0.2, 1.0],
                ambient: [0.8, 0.7, 0.5, 1.0],
            })
            .with_effects(vec![
                EnvironmentalEffect::Dust { density: 0.4 },
                EnvironmentalEffect::Wind { strength: 0.5 },
            ])
            .with_difficulty_modifier(1.2);

        self.biomes.insert("desert".to_string(), desert_biome);

        // Arctic biome
        let arctic_biome = Biome::new("arctic".to_string(), "Arctic".to_string())
            .with_description("A frozen arctic environment".to_string())
            .with_tiles(Tile::Snow, Tile::Ice)
            .with_decorative_tiles(vec![Tile::Snow, Tile::Ice, Tile::Pillar])
            .with_color_palette(ColorPalette {
                primary: [0.9, 0.9, 1.0, 1.0],
                secondary: [0.7, 0.7, 0.9, 1.0],
                accent: [1.0, 1.0, 1.0, 1.0],
                background: [0.1, 0.1, 0.2, 1.0],
                ambient: [0.6, 0.7, 0.9, 1.0],
            })
            .with_effects(vec![
                EnvironmentalEffect::Snow { intensity: 0.6 },
                EnvironmentalEffect::Wind { strength: 0.7 },
            ])
            .with_difficulty_modifier(1.1);

        self.biomes.insert("arctic".to_string(), arctic_biome);

        // Industrial biome
        let industrial_biome = Biome::new("industrial".to_string(), "Industrial".to_string())
            .with_description("A mechanical industrial environment".to_string())
            .with_tiles(Tile::Metal, Tile::Metal)
            .with_decorative_tiles(vec![Tile::Metal, Tile::Stone, Tile::Pillar])
            .with_color_palette(ColorPalette {
                primary: [0.6, 0.6, 0.7, 1.0],
                secondary: [0.4, 0.4, 0.5, 1.0],
                accent: [0.8, 0.8, 0.9, 1.0],
                background: [0.1, 0.1, 0.1, 1.0],
                ambient: [0.3, 0.3, 0.4, 1.0],
            })
            .with_effects(vec![
                EnvironmentalEffect::Steam { density: 0.3 },
                EnvironmentalEffect::Glow { intensity: 0.4 },
            ])
            .with_difficulty_modifier(1.3);

        self.biomes.insert("industrial".to_string(), industrial_biome);

        // Cave biome
        let cave_biome = Biome::new("cave".to_string(), "Cave".to_string())
            .with_description("A dark underground cave".to_string())
            .with_tiles(Tile::Stone, Tile::Stone)
            .with_decorative_tiles(vec![Tile::Stone, Tile::Pillar, Tile::Spikes])
            .with_color_palette(ColorPalette {
                primary: [0.4, 0.4, 0.4, 1.0],
                secondary: [0.2, 0.2, 0.2, 1.0],
                accent: [0.6, 0.6, 0.6, 1.0],
                background: [0.05, 0.05, 0.05, 1.0],
                ambient: [0.1, 0.1, 0.1, 1.0],
            })
            .with_effects(vec![
                EnvironmentalEffect::Darkness { level: 0.7 },
                EnvironmentalEffect::Fog { density: 0.3 },
            ])
            .with_difficulty_modifier(1.4);

        self.biomes.insert("cave".to_string(), cave_biome);

        // Lava biome
        let lava_biome = Biome::new("lava".to_string(), "Lava".to_string())
            .with_description("A dangerous lava-filled environment".to_string())
            .with_tiles(Tile::Stone, Tile::Stone)
            .with_decorative_tiles(vec![Tile::Lava, Tile::Stone, Tile::Spikes])
            .with_color_palette(ColorPalette {
                primary: [1.0, 0.3, 0.0, 1.0],
                secondary: [0.8, 0.2, 0.0, 1.0],
                accent: [1.0, 0.5, 0.0, 1.0],
                background: [0.2, 0.0, 0.0, 1.0],
                ambient: [0.6, 0.2, 0.0, 1.0],
            })
            .with_effects(vec![
                EnvironmentalEffect::Glow { intensity: 0.8 },
                EnvironmentalEffect::Steam { density: 0.5 },
            ])
            .with_difficulty_modifier(1.5);

        self.biomes.insert("lava".to_string(), lava_biome);
    }

    /// Get a biome by ID
    pub fn get_biome(&self, id: &str) -> Option<&Biome> {
        self.biomes.get(id)
    }

    /// Get all available biome IDs
    pub fn get_biome_ids(&self) -> Vec<String> {
        self.biomes.keys().cloned().collect()
    }

    /// Add a custom biome
    pub fn add_biome(&mut self, biome: Biome) {
        self.biomes.insert(biome.id.clone(), biome);
    }

    /// Get a random biome
    pub fn get_random_biome(&self, rng: &mut fastrand::Rng) -> Option<&Biome> {
        let ids: Vec<&String> = self.biomes.keys().collect();
        if ids.is_empty() {
            None
        } else {
            let index = rng.usize(0..ids.len());
            self.biomes.get(ids[index])
        }
    }
}

