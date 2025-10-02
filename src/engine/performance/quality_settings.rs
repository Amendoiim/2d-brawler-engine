//! Quality Settings
//! 
//! This module manages quality settings and their impact on performance.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{QualityLevel, PerformanceResult, PerformanceError};

/// Quality settings manager
#[derive(Debug, Clone)]
pub struct QualitySettings {
    /// Current quality level
    pub current_quality: QualityLevel,
    /// Quality presets
    pub presets: HashMap<QualityLevel, QualityPreset>,
    /// Custom settings
    pub custom_settings: CustomQualitySettings,
    /// Auto-adjustment enabled
    pub auto_adjust: bool,
    /// Performance impact tracking
    pub performance_impact: HashMap<String, f32>,
}

/// Quality preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityPreset {
    /// Quality level
    pub level: QualityLevel,
    /// Rendering settings
    pub rendering: RenderingSettings,
    /// Audio settings
    pub audio: AudioSettings,
    /// Physics settings
    pub physics: PhysicsSettings,
    /// Visual effects settings
    pub visual_effects: VisualEffectsSettings,
    /// Performance impact score
    pub performance_impact: f32,
}

/// Custom quality settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomQualitySettings {
    /// Rendering settings
    pub rendering: RenderingSettings,
    /// Audio settings
    pub audio: AudioSettings,
    /// Physics settings
    pub physics: PhysicsSettings,
    /// Visual effects settings
    pub visual_effects: VisualEffectsSettings,
}

/// Rendering settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingSettings {
    /// Resolution scale (0.1 to 1.0)
    pub resolution_scale: f32,
    /// Anti-aliasing level
    pub anti_aliasing: AntiAliasingLevel,
    /// Anisotropic filtering level
    pub anisotropic_filtering: u32,
    /// Shadow quality
    pub shadow_quality: ShadowQuality,
    /// Texture quality
    pub texture_quality: TextureQuality,
    /// LOD bias
    pub lod_bias: f32,
    /// Max draw distance
    pub max_draw_distance: f32,
    /// Occlusion culling enabled
    pub occlusion_culling: bool,
    /// Frustum culling enabled
    pub frustum_culling: bool,
    /// V-Sync enabled
    pub vsync: bool,
    /// Triple buffering enabled
    pub triple_buffering: bool,
}

/// Audio settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    /// Audio quality
    pub quality: AudioQuality,
    /// Sample rate
    pub sample_rate: u32,
    /// Bit depth
    pub bit_depth: u32,
    /// Channels
    pub channels: u32,
    /// 3D audio enabled
    pub spatial_audio: bool,
    /// Reverb quality
    pub reverb_quality: ReverbQuality,
    /// Compression enabled
    pub compression: bool,
    /// Dynamic range
    pub dynamic_range: DynamicRange,
}

/// Physics settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsSettings {
    /// Physics quality
    pub quality: PhysicsQuality,
    /// Update frequency
    pub update_frequency: u32,
    /// Collision detection quality
    pub collision_quality: CollisionQuality,
    /// Gravity strength
    pub gravity_strength: f32,
    /// Air resistance
    pub air_resistance: f32,
    /// Friction coefficient
    pub friction_coefficient: f32,
    /// Restitution coefficient
    pub restitution_coefficient: f32,
    /// Sleep threshold
    pub sleep_threshold: f32,
    /// Max velocity
    pub max_velocity: f32,
}

/// Visual effects settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualEffectsSettings {
    /// Particle quality
    pub particle_quality: ParticleQuality,
    /// Lighting quality
    pub lighting_quality: LightingQuality,
    /// Post-processing quality
    pub post_processing_quality: PostProcessingQuality,
    /// Bloom enabled
    pub bloom: bool,
    /// SSAO enabled
    pub ssao: bool,
    /// Motion blur enabled
    pub motion_blur: bool,
    /// Depth of field enabled
    pub depth_of_field: bool,
    /// Screen space reflections enabled
    pub screen_space_reflections: bool,
    /// Volumetric lighting enabled
    pub volumetric_lighting: bool,
    /// Global illumination quality
    pub global_illumination: GlobalIlluminationQuality,
}

/// Anti-aliasing level
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AntiAliasingLevel {
    /// No anti-aliasing
    None,
    /// 2x MSAA
    MSAA2x,
    /// 4x MSAA
    MSAA4x,
    /// 8x MSAA
    MSAA8x,
    /// FXAA
    FXAA,
    /// TAA
    TAA,
}

/// Shadow quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShadowQuality {
    /// No shadows
    None,
    /// Low quality shadows
    Low,
    /// Medium quality shadows
    Medium,
    /// High quality shadows
    High,
    /// Ultra quality shadows
    Ultra,
}

/// Texture quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextureQuality {
    /// Low quality textures
    Low,
    /// Medium quality textures
    Medium,
    /// High quality textures
    High,
    /// Ultra quality textures
    Ultra,
}

/// Audio quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioQuality {
    /// Low quality audio
    Low,
    /// Medium quality audio
    Medium,
    /// High quality audio
    High,
    /// Ultra quality audio
    Ultra,
}

/// Reverb quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReverbQuality {
    /// No reverb
    None,
    /// Low quality reverb
    Low,
    /// Medium quality reverb
    Medium,
    /// High quality reverb
    High,
    /// Ultra quality reverb
    Ultra,
}

/// Dynamic range
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DynamicRange {
    /// Low dynamic range
    Low,
    /// Medium dynamic range
    Medium,
    /// High dynamic range
    High,
    /// Ultra dynamic range
    Ultra,
}

/// Physics quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhysicsQuality {
    /// Low quality physics
    Low,
    /// Medium quality physics
    Medium,
    /// High quality physics
    High,
    /// Ultra quality physics
    Ultra,
}

/// Collision quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollisionQuality {
    /// Low quality collision
    Low,
    /// Medium quality collision
    Medium,
    /// High quality collision
    High,
    /// Ultra quality collision
    Ultra,
}

/// Particle quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParticleQuality {
    /// Low quality particles
    Low,
    /// Medium quality particles
    Medium,
    /// High quality particles
    High,
    /// Ultra quality particles
    Ultra,
}

/// Lighting quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LightingQuality {
    /// Low quality lighting
    Low,
    /// Medium quality lighting
    Medium,
    /// High quality lighting
    High,
    /// Ultra quality lighting
    Ultra,
}

/// Post-processing quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PostProcessingQuality {
    /// Low quality post-processing
    Low,
    /// Medium quality post-processing
    Medium,
    /// High quality post-processing
    High,
    /// Ultra quality post-processing
    Ultra,
}

/// Global illumination quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GlobalIlluminationQuality {
    /// No global illumination
    None,
    /// Low quality global illumination
    Low,
    /// Medium quality global illumination
    Medium,
    /// High quality global illumination
    High,
    /// Ultra quality global illumination
    Ultra,
}

impl QualitySettings {
    /// Create a new quality settings manager
    pub fn new() -> Self {
        let mut settings = Self {
            current_quality: QualityLevel::High,
            presets: HashMap::new(),
            custom_settings: CustomQualitySettings::default(),
            auto_adjust: true,
            performance_impact: HashMap::new(),
        };

        // Initialize default presets
        settings.initialize_presets();
        settings
    }

    /// Set quality level
    pub fn set_quality_level(&mut self, level: QualityLevel) -> PerformanceResult<()> {
        if let Some(preset) = self.presets.get(&level) {
            self.current_quality = level;
            self.apply_preset(preset)?;
            Ok(())
        } else {
            Err(PerformanceError::UnsupportedQualityLevel(level))
        }
    }

    /// Get current quality level
    pub fn get_quality_level(&self) -> QualityLevel {
        self.current_quality.clone()
    }

    /// Apply quality preset
    pub fn apply_preset(&mut self, preset: &QualityPreset) -> PerformanceResult<()> {
        // Apply rendering settings
        self.custom_settings.rendering = preset.rendering.clone();
        
        // Apply audio settings
        self.custom_settings.audio = preset.audio.clone();
        
        // Apply physics settings
        self.custom_settings.physics = preset.physics.clone();
        
        // Apply visual effects settings
        self.custom_settings.visual_effects = preset.visual_effects.clone();

        // Update performance impact
        self.performance_impact.insert("quality_level".to_string(), preset.performance_impact);

        Ok(())
    }

    /// Get performance impact of current settings
    pub fn get_performance_impact(&self) -> f32 {
        self.performance_impact.values().sum()
    }

    /// Enable/disable auto-adjustment
    pub fn set_auto_adjust(&mut self, enabled: bool) {
        self.auto_adjust = enabled;
    }

    /// Get custom settings
    pub fn get_custom_settings(&self) -> &CustomQualitySettings {
        &self.custom_settings
    }

    /// Update custom settings
    pub fn update_custom_settings(&mut self, settings: CustomQualitySettings) {
        self.custom_settings = settings;
        self.current_quality = QualityLevel::Custom;
    }

    /// Initialize default presets
    fn initialize_presets(&mut self) {
        // Low quality preset
        self.presets.insert(QualityLevel::Low, QualityPreset {
            level: QualityLevel::Low,
            rendering: RenderingSettings {
                resolution_scale: 0.5,
                anti_aliasing: AntiAliasingLevel::None,
                anisotropic_filtering: 1,
                shadow_quality: ShadowQuality::None,
                texture_quality: TextureQuality::Low,
                lod_bias: 1.0,
                max_draw_distance: 100.0,
                occlusion_culling: false,
                frustum_culling: true,
                vsync: false,
                triple_buffering: false,
            },
            audio: AudioSettings {
                quality: AudioQuality::Low,
                sample_rate: 22050,
                bit_depth: 16,
                channels: 2,
                spatial_audio: false,
                reverb_quality: ReverbQuality::None,
                compression: true,
                dynamic_range: DynamicRange::Low,
            },
            physics: PhysicsSettings {
                quality: PhysicsQuality::Low,
                update_frequency: 30,
                collision_quality: CollisionQuality::Low,
                gravity_strength: 9.81,
                air_resistance: 0.1,
                friction_coefficient: 0.5,
                restitution_coefficient: 0.3,
                sleep_threshold: 0.1,
                max_velocity: 100.0,
            },
            visual_effects: VisualEffectsSettings {
                particle_quality: ParticleQuality::Low,
                lighting_quality: LightingQuality::Low,
                post_processing_quality: PostProcessingQuality::Low,
                bloom: false,
                ssao: false,
                motion_blur: false,
                depth_of_field: false,
                screen_space_reflections: false,
                volumetric_lighting: false,
                global_illumination: GlobalIlluminationQuality::None,
            },
            performance_impact: 0.1,
        });

        // Medium quality preset
        self.presets.insert(QualityLevel::Medium, QualityPreset {
            level: QualityLevel::Medium,
            rendering: RenderingSettings {
                resolution_scale: 0.75,
                anti_aliasing: AntiAliasingLevel::FXAA,
                anisotropic_filtering: 4,
                shadow_quality: ShadowQuality::Low,
                texture_quality: TextureQuality::Medium,
                lod_bias: 0.5,
                max_draw_distance: 200.0,
                occlusion_culling: true,
                frustum_culling: true,
                vsync: true,
                triple_buffering: false,
            },
            audio: AudioSettings {
                quality: AudioQuality::Medium,
                sample_rate: 44100,
                bit_depth: 16,
                channels: 2,
                spatial_audio: true,
                reverb_quality: ReverbQuality::Low,
                compression: true,
                dynamic_range: DynamicRange::Medium,
            },
            physics: PhysicsSettings {
                quality: PhysicsQuality::Medium,
                update_frequency: 60,
                collision_quality: CollisionQuality::Medium,
                gravity_strength: 9.81,
                air_resistance: 0.1,
                friction_coefficient: 0.5,
                restitution_coefficient: 0.3,
                sleep_threshold: 0.1,
                max_velocity: 100.0,
            },
            visual_effects: VisualEffectsSettings {
                particle_quality: ParticleQuality::Medium,
                lighting_quality: LightingQuality::Medium,
                post_processing_quality: PostProcessingQuality::Medium,
                bloom: true,
                ssao: false,
                motion_blur: false,
                depth_of_field: false,
                screen_space_reflections: false,
                volumetric_lighting: false,
                global_illumination: GlobalIlluminationQuality::Low,
            },
            performance_impact: 0.5,
        });

        // High quality preset
        self.presets.insert(QualityLevel::High, QualityPreset {
            level: QualityLevel::High,
            rendering: RenderingSettings {
                resolution_scale: 1.0,
                anti_aliasing: AntiAliasingLevel::MSAA4x,
                anisotropic_filtering: 8,
                shadow_quality: ShadowQuality::High,
                texture_quality: TextureQuality::High,
                lod_bias: 0.0,
                max_draw_distance: 500.0,
                occlusion_culling: true,
                frustum_culling: true,
                vsync: true,
                triple_buffering: true,
            },
            audio: AudioSettings {
                quality: AudioQuality::High,
                sample_rate: 48000,
                bit_depth: 24,
                channels: 2,
                spatial_audio: true,
                reverb_quality: ReverbQuality::High,
                compression: false,
                dynamic_range: DynamicRange::High,
            },
            physics: PhysicsSettings {
                quality: PhysicsQuality::High,
                update_frequency: 120,
                collision_quality: CollisionQuality::High,
                gravity_strength: 9.81,
                air_resistance: 0.1,
                friction_coefficient: 0.5,
                restitution_coefficient: 0.3,
                sleep_threshold: 0.1,
                max_velocity: 100.0,
            },
            visual_effects: VisualEffectsSettings {
                particle_quality: ParticleQuality::High,
                lighting_quality: LightingQuality::High,
                post_processing_quality: PostProcessingQuality::High,
                bloom: true,
                ssao: true,
                motion_blur: true,
                depth_of_field: true,
                screen_space_reflections: true,
                volumetric_lighting: true,
                global_illumination: GlobalIlluminationQuality::High,
            },
            performance_impact: 0.8,
        });

        // Ultra quality preset
        self.presets.insert(QualityLevel::Ultra, QualityPreset {
            level: QualityLevel::Ultra,
            rendering: RenderingSettings {
                resolution_scale: 1.0,
                anti_aliasing: AntiAliasingLevel::MSAA8x,
                anisotropic_filtering: 16,
                shadow_quality: ShadowQuality::Ultra,
                texture_quality: TextureQuality::Ultra,
                lod_bias: -0.5,
                max_draw_distance: 1000.0,
                occlusion_culling: true,
                frustum_culling: true,
                vsync: true,
                triple_buffering: true,
            },
            audio: AudioSettings {
                quality: AudioQuality::Ultra,
                sample_rate: 96000,
                bit_depth: 32,
                channels: 2,
                spatial_audio: true,
                reverb_quality: ReverbQuality::Ultra,
                compression: false,
                dynamic_range: DynamicRange::Ultra,
            },
            physics: PhysicsSettings {
                quality: PhysicsQuality::Ultra,
                update_frequency: 240,
                collision_quality: CollisionQuality::Ultra,
                gravity_strength: 9.81,
                air_resistance: 0.1,
                friction_coefficient: 0.5,
                restitution_coefficient: 0.3,
                sleep_threshold: 0.1,
                max_velocity: 100.0,
            },
            visual_effects: VisualEffectsSettings {
                particle_quality: ParticleQuality::Ultra,
                lighting_quality: LightingQuality::Ultra,
                post_processing_quality: PostProcessingQuality::Ultra,
                bloom: true,
                ssao: true,
                motion_blur: true,
                depth_of_field: true,
                screen_space_reflections: true,
                volumetric_lighting: true,
                global_illumination: GlobalIlluminationQuality::Ultra,
            },
            performance_impact: 1.0,
        });
    }
}

impl Default for CustomQualitySettings {
    fn default() -> Self {
        Self {
            rendering: RenderingSettings::default(),
            audio: AudioSettings::default(),
            physics: PhysicsSettings::default(),
            visual_effects: VisualEffectsSettings::default(),
        }
    }
}

impl Default for RenderingSettings {
    fn default() -> Self {
        Self {
            resolution_scale: 1.0,
            anti_aliasing: AntiAliasingLevel::MSAA4x,
            anisotropic_filtering: 8,
            shadow_quality: ShadowQuality::High,
            texture_quality: TextureQuality::High,
            lod_bias: 0.0,
            max_draw_distance: 500.0,
            occlusion_culling: true,
            frustum_culling: true,
            vsync: true,
            triple_buffering: true,
        }
    }
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            quality: AudioQuality::High,
            sample_rate: 48000,
            bit_depth: 24,
            channels: 2,
            spatial_audio: true,
            reverb_quality: ReverbQuality::High,
            compression: false,
            dynamic_range: DynamicRange::High,
        }
    }
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            quality: PhysicsQuality::High,
            update_frequency: 120,
            collision_quality: CollisionQuality::High,
            gravity_strength: 9.81,
            air_resistance: 0.1,
            friction_coefficient: 0.5,
            restitution_coefficient: 0.3,
            sleep_threshold: 0.1,
            max_velocity: 100.0,
        }
    }
}

impl Default for VisualEffectsSettings {
    fn default() -> Self {
        Self {
            particle_quality: ParticleQuality::High,
            lighting_quality: LightingQuality::High,
            post_processing_quality: PostProcessingQuality::High,
            bloom: true,
            ssao: true,
            motion_blur: true,
            depth_of_field: true,
            screen_space_reflections: true,
            volumetric_lighting: true,
            global_illumination: GlobalIlluminationQuality::High,
        }
    }
}

impl Default for QualitySettings {
    fn default() -> Self {
        Self::new()
    }
}
