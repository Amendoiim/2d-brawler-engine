//! Atmospheric effects system for weather, lighting, and environmental ambiance

use crate::engine::level::biome::{Biome, BiomeManager, EnvironmentalEffect};
use glam::Vec2;
use std::collections::HashMap;

/// An atmospheric effect in the level
#[derive(Debug, Clone)]
pub struct AtmosphericEffect {
    /// Effect identifier
    pub id: String,
    /// Effect type
    pub effect_type: AtmosphericEffectType,
    /// Effect position
    pub position: Vec2,
    /// Effect intensity (0.0 to 1.0)
    pub intensity: f32,
    /// Effect duration (0.0 = infinite)
    pub duration: f32,
    /// Effect age
    pub age: f32,
    /// Effect size/radius
    pub size: f32,
    /// Effect color
    pub color: [f32; 4],
    /// Effect opacity
    pub opacity: f32,
    /// Whether the effect is active
    pub active: bool,
    /// Effect properties
    pub properties: EffectProperties,
}

/// Types of atmospheric effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtmosphericEffectType {
    /// Rain effect
    Rain,
    /// Snow effect
    Snow,
    /// Fog effect
    Fog,
    /// Wind effect
    Wind,
    /// Dust effect
    Dust,
    /// Steam effect
    Steam,
    /// Glow effect
    Glow,
    /// Darkness effect
    Darkness,
    /// Lightning effect
    Lightning,
    /// Fire effect
    Fire,
    /// Smoke effect
    Smoke,
    /// Mist effect
    Mist,
    /// Aurora effect
    Aurora,
}

/// Properties of an atmospheric effect
#[derive(Debug, Clone)]
pub struct EffectProperties {
    /// Whether the effect moves
    pub moving: bool,
    /// Movement velocity
    pub velocity: Vec2,
    /// Whether the effect rotates
    pub rotating: bool,
    /// Rotation speed
    pub rotation_speed: f32,
    /// Whether the effect scales
    pub scaling: bool,
    /// Scale animation speed
    pub scale_speed: f32,
    /// Whether the effect fades
    pub fading: bool,
    /// Fade speed
    pub fade_speed: f32,
    /// Whether the effect pulses
    pub pulsing: bool,
    /// Pulse speed
    pub pulse_speed: f32,
    /// Effect texture ID
    pub texture_id: u32,
    /// Effect particle count
    pub particle_count: u32,
    /// Effect update frequency
    pub update_frequency: f32,
}

/// Atmospheric effects manager
pub struct AtmosphericEffectsManager {
    /// Biome manager for biome-specific effects
    pub biome_manager: BiomeManager,
    /// Active effects
    pub effects: Vec<AtmosphericEffect>,
    /// Effect templates
    pub effect_templates: HashMap<AtmosphericEffectType, EffectTemplate>,
    /// Global lighting settings
    pub global_lighting: GlobalLighting,
    /// Weather system
    pub weather_system: WeatherSystem,
}

/// Template for creating atmospheric effects
#[derive(Debug, Clone)]
pub struct EffectTemplate {
    /// Effect type
    pub effect_type: AtmosphericEffectType,
    /// Default properties
    pub properties: EffectProperties,
    /// Default intensity
    pub intensity: f32,
    /// Default duration
    pub duration: f32,
    /// Default size
    pub size: f32,
    /// Default color
    pub color: [f32; 4],
    /// Default opacity
    pub opacity: f32,
    /// Biome-specific settings
    pub biome_settings: HashMap<String, BiomeEffectSettings>,
}

/// Biome-specific settings for atmospheric effects
#[derive(Debug, Clone)]
pub struct BiomeEffectSettings {
    /// Intensity modifier
    pub intensity_modifier: f32,
    /// Color tint
    pub color_tint: [f32; 4],
    /// Opacity modifier
    pub opacity_modifier: f32,
    /// Size modifier
    pub size_modifier: f32,
    /// Duration modifier
    pub duration_modifier: f32,
}

/// Global lighting settings
#[derive(Debug, Clone)]
pub struct GlobalLighting {
    /// Ambient light color
    pub ambient_color: [f32; 4],
    /// Ambient light intensity
    pub ambient_intensity: f32,
    /// Directional light color
    pub directional_color: [f32; 4],
    /// Directional light intensity
    pub directional_intensity: f32,
    /// Light direction
    pub light_direction: Vec2,
    /// Shadow intensity
    pub shadow_intensity: f32,
    /// Fog color
    pub fog_color: [f32; 4],
    /// Fog density
    pub fog_density: f32,
    /// Fog start distance
    pub fog_start: f32,
    /// Fog end distance
    pub fog_end: f32,
}

/// Weather system for managing weather effects
#[derive(Debug, Clone)]
pub struct WeatherSystem {
    /// Current weather type
    pub current_weather: WeatherType,
    /// Weather intensity
    pub weather_intensity: f32,
    /// Weather transition time
    pub transition_time: f32,
    /// Weather duration
    pub weather_duration: f32,
    /// Weather age
    pub weather_age: f32,
    /// Weather probability
    pub weather_probability: f32,
}

/// Types of weather
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeatherType {
    /// Clear weather
    Clear,
    /// Rainy weather
    Rainy,
    /// Snowy weather
    Snowy,
    /// Foggy weather
    Foggy,
    /// Stormy weather
    Stormy,
    /// Windy weather
    Windy,
    /// Dusty weather
    Dusty,
    /// Steam weather
    Steam,
}

impl AtmosphericEffectsManager {
    /// Create a new atmospheric effects manager
    pub fn new() -> Self {
        let mut manager = Self {
            biome_manager: BiomeManager::new(),
            effects: Vec::new(),
            effect_templates: HashMap::new(),
            global_lighting: GlobalLighting::default(),
            weather_system: WeatherSystem::default(),
        };
        manager.initialize_effect_templates();
        manager
    }

    /// Initialize default effect templates
    fn initialize_effect_templates(&mut self) {
        // Rain effect template
        let rain_template = EffectTemplate {
            effect_type: AtmosphericEffectType::Rain,
            properties: EffectProperties {
                moving: true,
                velocity: Vec2::new(0.0, -100.0),
                rotating: false,
                rotation_speed: 0.0,
                scaling: false,
                scale_speed: 0.0,
                fading: false,
                fade_speed: 0.0,
                pulsing: false,
                pulse_speed: 0.0,
                texture_id: 4000,
                particle_count: 1000,
                update_frequency: 60.0,
            },
            intensity: 0.5,
            duration: 0.0, // Infinite
            size: 1.0,
            color: [0.7, 0.8, 1.0, 0.8],
            opacity: 0.8,
            biome_settings: {
                let mut settings = HashMap::new();
                settings.insert("forest".to_string(), BiomeEffectSettings {
                    intensity_modifier: 1.2,
                    color_tint: [0.3, 0.6, 0.9, 1.0],
                    opacity_modifier: 1.0,
                    size_modifier: 1.0,
                    duration_modifier: 1.0,
                });
                settings.insert("desert".to_string(), BiomeEffectSettings {
                    intensity_modifier: 0.3,
                    color_tint: [0.8, 0.6, 0.4, 1.0],
                    opacity_modifier: 0.5,
                    size_modifier: 0.8,
                    duration_modifier: 0.5,
                });
                settings.insert("arctic".to_string(), BiomeEffectSettings {
                    intensity_modifier: 0.8,
                    color_tint: [0.6, 0.7, 0.9, 1.0],
                    opacity_modifier: 0.9,
                    size_modifier: 1.1,
                    duration_modifier: 1.5,
                });
                settings
            },
        };
        self.effect_templates.insert(AtmosphericEffectType::Rain, rain_template);

        // Snow effect template
        let snow_template = EffectTemplate {
            effect_type: AtmosphericEffectType::Snow,
            properties: EffectProperties {
                moving: true,
                velocity: Vec2::new(0.0, -20.0),
                rotating: true,
                rotation_speed: 1.0,
                scaling: false,
                scale_speed: 0.0,
                fading: false,
                fade_speed: 0.0,
                pulsing: false,
                pulse_speed: 0.0,
                texture_id: 4001,
                particle_count: 500,
                update_frequency: 30.0,
            },
            intensity: 0.6,
            duration: 0.0,
            size: 1.0,
            color: [1.0, 1.0, 1.0, 0.9],
            opacity: 0.9,
            biome_settings: {
                let mut settings = HashMap::new();
                settings.insert("arctic".to_string(), BiomeEffectSettings {
                    intensity_modifier: 1.5,
                    color_tint: [0.9, 0.9, 1.0, 1.0],
                    opacity_modifier: 1.0,
                    size_modifier: 1.2,
                    duration_modifier: 2.0,
                });
                settings.insert("forest".to_string(), BiomeEffectSettings {
                    intensity_modifier: 0.8,
                    color_tint: [0.8, 0.9, 1.0, 1.0],
                    opacity_modifier: 0.8,
                    size_modifier: 1.0,
                    duration_modifier: 1.0,
                });
                settings.insert("desert".to_string(), BiomeEffectSettings {
                    intensity_modifier: 0.1,
                    color_tint: [1.0, 0.9, 0.8, 1.0],
                    opacity_modifier: 0.3,
                    size_modifier: 0.5,
                    duration_modifier: 0.2,
                });
                settings
            },
        };
        self.effect_templates.insert(AtmosphericEffectType::Snow, snow_template);

        // Fog effect template
        let fog_template = EffectTemplate {
            effect_type: AtmosphericEffectType::Fog,
            properties: EffectProperties {
                moving: true,
                velocity: Vec2::new(10.0, 0.0),
                rotating: false,
                rotation_speed: 0.0,
                scaling: true,
                scale_speed: 0.1,
                fading: true,
                fade_speed: 0.05,
                pulsing: true,
                pulse_speed: 0.2,
                texture_id: 4002,
                particle_count: 200,
                update_frequency: 20.0,
            },
            intensity: 0.4,
            duration: 0.0,
            size: 2.0,
            color: [0.8, 0.8, 0.8, 0.6],
            opacity: 0.6,
            biome_settings: {
                let mut settings = HashMap::new();
                settings.insert("forest".to_string(), BiomeEffectSettings {
                    intensity_modifier: 1.0,
                    color_tint: [0.3, 0.5, 0.3, 1.0],
                    opacity_modifier: 0.8,
                    size_modifier: 1.0,
                    duration_modifier: 1.0,
                });
                settings.insert("cave".to_string(), BiomeEffectSettings {
                    intensity_modifier: 1.5,
                    color_tint: [0.2, 0.2, 0.2, 1.0],
                    opacity_modifier: 1.2,
                    size_modifier: 1.5,
                    duration_modifier: 1.5,
                });
                settings.insert("arctic".to_string(), BiomeEffectSettings {
                    intensity_modifier: 0.8,
                    color_tint: [0.6, 0.7, 0.8, 1.0],
                    opacity_modifier: 0.7,
                    size_modifier: 1.2,
                    duration_modifier: 1.2,
                });
                settings
            },
        };
        self.effect_templates.insert(AtmosphericEffectType::Fog, fog_template);

        // Steam effect template
        let steam_template = EffectTemplate {
            effect_type: AtmosphericEffectType::Steam,
            properties: EffectProperties {
                moving: true,
                velocity: Vec2::new(0.0, -50.0),
                rotating: false,
                rotation_speed: 0.0,
                scaling: true,
                scale_speed: 0.2,
                fading: true,
                fade_speed: 0.1,
                pulsing: false,
                pulse_speed: 0.0,
                texture_id: 4003,
                particle_count: 300,
                update_frequency: 40.0,
            },
            intensity: 0.7,
            duration: 5.0,
            size: 1.5,
            color: [0.9, 0.9, 0.9, 0.7],
            opacity: 0.7,
            biome_settings: {
                let mut settings = HashMap::new();
                settings.insert("lava".to_string(), BiomeEffectSettings {
                    intensity_modifier: 2.0,
                    color_tint: [1.0, 0.5, 0.2, 1.0],
                    opacity_modifier: 1.0,
                    size_modifier: 1.5,
                    duration_modifier: 1.0,
                });
                settings.insert("industrial".to_string(), BiomeEffectSettings {
                    intensity_modifier: 1.5,
                    color_tint: [0.7, 0.7, 0.8, 1.0],
                    opacity_modifier: 0.8,
                    size_modifier: 1.2,
                    duration_modifier: 1.2,
                });
                settings.insert("cave".to_string(), BiomeEffectSettings {
                    intensity_modifier: 1.0,
                    color_tint: [0.6, 0.6, 0.6, 1.0],
                    opacity_modifier: 0.6,
                    size_modifier: 1.0,
                    duration_modifier: 1.0,
                });
                settings
            },
        };
        self.effect_templates.insert(AtmosphericEffectType::Steam, steam_template);

        // Glow effect template
        let glow_template = EffectTemplate {
            effect_type: AtmosphericEffectType::Glow,
            properties: EffectProperties {
                moving: false,
                velocity: Vec2::ZERO,
                rotating: false,
                rotation_speed: 0.0,
                scaling: true,
                scale_speed: 0.1,
                fading: true,
                fade_speed: 0.05,
                pulsing: true,
                pulse_speed: 0.3,
                texture_id: 4004,
                particle_count: 100,
                update_frequency: 60.0,
            },
            intensity: 0.8,
            duration: 0.0,
            size: 1.0,
            color: [1.0, 0.8, 0.2, 0.8],
            opacity: 0.8,
            biome_settings: {
                let mut settings = HashMap::new();
                settings.insert("lava".to_string(), BiomeEffectSettings {
                    intensity_modifier: 1.5,
                    color_tint: [1.0, 0.3, 0.0, 1.0],
                    opacity_modifier: 1.0,
                    size_modifier: 1.2,
                    duration_modifier: 1.0,
                });
                settings.insert("industrial".to_string(), BiomeEffectSettings {
                    intensity_modifier: 1.2,
                    color_tint: [0.8, 0.8, 1.0, 1.0],
                    opacity_modifier: 0.9,
                    size_modifier: 1.0,
                    duration_modifier: 1.0,
                });
                settings.insert("cave".to_string(), BiomeEffectSettings {
                    intensity_modifier: 1.0,
                    color_tint: [0.6, 0.6, 0.8, 1.0],
                    opacity_modifier: 0.8,
                    size_modifier: 0.8,
                    duration_modifier: 1.0,
                });
                settings
            },
        };
        self.effect_templates.insert(AtmosphericEffectType::Glow, glow_template);
    }

    /// Create atmospheric effects for a biome
    pub fn create_biome_effects(&mut self, biome: &str, level_width: f32, level_height: f32) -> Result<(), String> {
        self.effects.clear();

        // Get biome information and clone it to avoid borrowing issues
        let biome_info = self.biome_manager.get_biome(biome)
            .ok_or_else(|| format!("Unknown biome: {}", biome))?
            .clone();

        // Clone the effects to avoid borrowing issues
        let effects = biome_info.effects.clone();

        // Create effects based on biome's environmental effects
        for environmental_effect in &effects {
            match environmental_effect {
                EnvironmentalEffect::Rain { intensity } => {
                    self.create_effect(AtmosphericEffectType::Rain, *intensity, level_width, level_height, biome)?;
                },
                EnvironmentalEffect::Snow { intensity } => {
                    self.create_effect(AtmosphericEffectType::Snow, *intensity, level_width, level_height, biome)?;
                },
                EnvironmentalEffect::Fog { density } => {
                    self.create_effect(AtmosphericEffectType::Fog, *density, level_width, level_height, biome)?;
                },
                EnvironmentalEffect::Wind { strength } => {
                    self.create_effect(AtmosphericEffectType::Wind, *strength, level_width, level_height, biome)?;
                },
                EnvironmentalEffect::Dust { density } => {
                    self.create_effect(AtmosphericEffectType::Dust, *density, level_width, level_height, biome)?;
                },
                EnvironmentalEffect::Steam { density } => {
                    self.create_effect(AtmosphericEffectType::Steam, *density, level_width, level_height, biome)?;
                },
                EnvironmentalEffect::Glow { intensity } => {
                    self.create_effect(AtmosphericEffectType::Glow, *intensity, level_width, level_height, biome)?;
                },
                EnvironmentalEffect::Darkness { level } => {
                    self.create_effect(AtmosphericEffectType::Darkness, *level, level_width, level_height, biome)?;
                },
            }
        }

        // Update global lighting based on biome
        self.update_global_lighting_for_biome(&biome_info);

        Ok(())
    }

    /// Create a specific atmospheric effect
    fn create_effect(
        &mut self,
        effect_type: AtmosphericEffectType,
        intensity: f32,
        level_width: f32,
        level_height: f32,
        biome: &str,
    ) -> Result<(), String> {
        let template = self.effect_templates.get(&effect_type)
            .ok_or_else(|| format!("No template found for effect type: {:?}", effect_type))?;

        // Get biome-specific settings
        let biome_settings = template.biome_settings.get(biome)
            .or_else(|| template.biome_settings.get("default"))
            .ok_or_else(|| format!("No biome settings found for biome: {}", biome))?;

        let effect = AtmosphericEffect {
            id: format!("{}_{}_{}", 
                       format!("{:?}", effect_type).to_lowercase(),
                       biome,
                       self.effects.len()),
            effect_type,
            position: Vec2::new(level_width / 2.0, level_height / 2.0),
            intensity: intensity * biome_settings.intensity_modifier,
            duration: template.duration * biome_settings.duration_modifier,
            age: 0.0,
            size: template.size * biome_settings.size_modifier,
            color: [
                template.color[0] * biome_settings.color_tint[0],
                template.color[1] * biome_settings.color_tint[1],
                template.color[2] * biome_settings.color_tint[2],
                template.color[3] * biome_settings.color_tint[3],
            ],
            opacity: template.opacity * biome_settings.opacity_modifier,
            active: true,
            properties: template.properties.clone(),
        };

        self.effects.push(effect);
        Ok(())
    }

    /// Update atmospheric effects
    pub fn update_effects(&mut self, dt: f32) {
        for effect in &mut self.effects {
            if !effect.active {
                continue;
            }

            effect.age += dt;

            // Check if effect should expire
            if effect.duration > 0.0 && effect.age >= effect.duration {
                effect.active = false;
                continue;
            }

            // Update effect properties
            if effect.properties.moving {
                effect.position += effect.properties.velocity * dt;
            }

            if effect.properties.fading {
                effect.opacity -= effect.properties.fade_speed * dt;
                effect.opacity = effect.opacity.max(0.0);
            }

            if effect.properties.pulsing {
                let pulse = (effect.age * effect.properties.pulse_speed).sin();
                effect.opacity = (effect.opacity + pulse * 0.1).clamp(0.0, 1.0);
            }
        }

        // Remove inactive effects
        self.effects.retain(|effect| effect.active);

        // Update weather system
        self.update_weather_system(dt);
    }

    /// Update weather system
    fn update_weather_system(&mut self, dt: f32) {
        self.weather_system.weather_age += dt;

        // Check for weather transitions
        if self.weather_system.weather_age >= self.weather_system.weather_duration {
            self.weather_system.weather_age = 0.0;
            self.weather_system.weather_duration = fastrand::f32() * 60.0 + 30.0; // 30-90 seconds

            // Randomly change weather
            if fastrand::f32() < self.weather_system.weather_probability {
                self.weather_system.current_weather = match fastrand::u32(0..7) {
                    0 => WeatherType::Clear,
                    1 => WeatherType::Rainy,
                    2 => WeatherType::Snowy,
                    3 => WeatherType::Foggy,
                    4 => WeatherType::Stormy,
                    5 => WeatherType::Windy,
                    6 => WeatherType::Dusty,
                    _ => WeatherType::Steam,
                };
            }
        }
    }

    /// Update global lighting for a biome
    fn update_global_lighting_for_biome(&mut self, biome: &Biome) {
        self.global_lighting.ambient_color = biome.color_palette.ambient;
        self.global_lighting.ambient_intensity = 0.3;
        self.global_lighting.directional_color = biome.color_palette.primary;
        self.global_lighting.directional_intensity = 0.7;
        self.global_lighting.fog_color = biome.color_palette.background;
        self.global_lighting.fog_density = 0.1;
    }

    /// Get all atmospheric effects
    pub fn get_effects(&self) -> &[AtmosphericEffect] {
        &self.effects
    }

    /// Get effects by type
    pub fn get_effects_by_type(&self, effect_type: AtmosphericEffectType) -> Vec<&AtmosphericEffect> {
        self.effects.iter().filter(|effect| effect.effect_type == effect_type).collect()
    }

    /// Get active effects
    pub fn get_active_effects(&self) -> Vec<&AtmosphericEffect> {
        self.effects.iter().filter(|effect| effect.active).collect()
    }

    /// Get global lighting settings
    pub fn get_global_lighting(&self) -> &GlobalLighting {
        &self.global_lighting
    }

    /// Get weather system
    pub fn get_weather_system(&self) -> &WeatherSystem {
        &self.weather_system
    }

    /// Set effect intensity
    pub fn set_effect_intensity(&mut self, effect_id: &str, intensity: f32) -> Result<(), String> {
        if let Some(effect) = self.effects.iter_mut().find(|e| e.id == effect_id) {
            effect.intensity = intensity.clamp(0.0, 1.0);
            Ok(())
        } else {
            Err(format!("Effect with ID {} not found", effect_id))
        }
    }

    /// Set effect opacity
    pub fn set_effect_opacity(&mut self, effect_id: &str, opacity: f32) -> Result<(), String> {
        if let Some(effect) = self.effects.iter_mut().find(|e| e.id == effect_id) {
            effect.opacity = opacity.clamp(0.0, 1.0);
            Ok(())
        } else {
            Err(format!("Effect with ID {} not found", effect_id))
        }
    }

    /// Add a custom effect template
    pub fn add_effect_template(&mut self, template: EffectTemplate) {
        self.effect_templates.insert(template.effect_type, template);
    }
}

impl Default for GlobalLighting {
    fn default() -> Self {
        Self {
            ambient_color: [0.3, 0.3, 0.3, 1.0],
            ambient_intensity: 0.3,
            directional_color: [1.0, 1.0, 1.0, 1.0],
            directional_intensity: 0.7,
            light_direction: Vec2::new(0.5, -0.5),
            shadow_intensity: 0.5,
            fog_color: [0.5, 0.5, 0.5, 1.0],
            fog_density: 0.1,
            fog_start: 10.0,
            fog_end: 100.0,
        }
    }
}

impl Default for WeatherSystem {
    fn default() -> Self {
        Self {
            current_weather: WeatherType::Clear,
            weather_intensity: 0.5,
            transition_time: 2.0,
            weather_duration: 60.0,
            weather_age: 0.0,
            weather_probability: 0.3,
        }
    }
}

impl Default for AtmosphericEffectsManager {
    fn default() -> Self {
        Self::new()
    }
}
