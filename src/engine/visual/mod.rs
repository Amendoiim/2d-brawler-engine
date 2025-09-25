//! Visual effects and polish system
//! 
//! This module provides comprehensive visual effects including:
//! - Particle systems for various effects
//! - Lighting and shadows
//! - Post-processing effects
//! - Screen effects and transitions
//! - Visual feedback systems

pub mod particle_system;
pub mod lighting;
pub mod post_processing;
pub mod screen_effects;
pub mod visual_feedback;

use std::collections::HashMap;

/// Visual effects manager that coordinates all visual polish systems
#[derive(Default)]
pub struct VisualEffectsManager {
    /// Particle system manager
    pub particle_manager: particle_system::ParticleManager,
    /// Lighting system manager
    pub lighting_manager: lighting::LightingManager,
    /// Post-processing manager
    pub post_processing_manager: post_processing::PostProcessingManager,
    /// Screen effects manager
    pub screen_effects_manager: screen_effects::ScreenEffectsManager,
    /// Visual feedback manager
    pub visual_feedback_manager: visual_feedback::VisualFeedbackManager,
    /// Performance settings
    pub performance_settings: VisualPerformanceSettings,
    /// Quality settings
    pub quality_settings: VisualQualitySettings,
}

/// Performance settings for visual effects
#[derive(Clone)]
pub struct VisualPerformanceSettings {
    /// Maximum number of particles
    pub max_particles: usize,
    /// Maximum number of lights
    pub max_lights: usize,
    /// Enable particle culling
    pub enable_particle_culling: bool,
    /// Enable light culling
    pub enable_light_culling: bool,
    /// Target frame rate
    pub target_fps: u32,
    /// Enable vsync
    pub enable_vsync: bool,
    /// Enable frame rate limiting
    pub enable_frame_rate_limiting: bool,
}

/// Quality settings for visual effects
#[derive(Clone)]
pub struct VisualQualitySettings {
    /// Particle quality level
    pub particle_quality: QualityLevel,
    /// Lighting quality level
    pub lighting_quality: QualityLevel,
    /// Post-processing quality level
    pub post_processing_quality: QualityLevel,
    /// Shadow quality level
    pub shadow_quality: QualityLevel,
    /// Anti-aliasing level
    pub anti_aliasing: AntiAliasingLevel,
    /// Texture filtering
    pub texture_filtering: TextureFiltering,
}

/// Quality levels for visual effects
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum QualityLevel {
    Low,
    Medium,
    High,
    Ultra,
}

impl Default for QualityLevel {
    fn default() -> Self {
        QualityLevel::High
    }
}

/// Anti-aliasing levels
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AntiAliasingLevel {
    None,
    FXAA,
    MSAA2x,
    MSAA4x,
    MSAA8x,
    TAA,
}

/// Texture filtering options
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextureFiltering {
    Nearest,
    Linear,
    Anisotropic2x,
    Anisotropic4x,
    Anisotropic8x,
    Anisotropic16x,
}

impl Default for VisualPerformanceSettings {
    fn default() -> Self {
        Self {
            max_particles: 1000,
            max_lights: 50,
            enable_particle_culling: true,
            enable_light_culling: true,
            target_fps: 60,
            enable_vsync: true,
            enable_frame_rate_limiting: true,
        }
    }
}

impl Default for VisualQualitySettings {
    fn default() -> Self {
        Self {
            particle_quality: QualityLevel::High,
            lighting_quality: QualityLevel::High,
            post_processing_quality: QualityLevel::High,
            shadow_quality: QualityLevel::High,
            anti_aliasing: AntiAliasingLevel::FXAA,
            texture_filtering: TextureFiltering::Anisotropic4x,
        }
    }
}

impl VisualEffectsManager {
    /// Create a new visual effects manager
    pub fn new() -> Self {
        Self {
            particle_manager: particle_system::ParticleManager::new(),
            lighting_manager: lighting::LightingManager::new(),
            post_processing_manager: post_processing::PostProcessingManager::new(),
            screen_effects_manager: screen_effects::ScreenEffectsManager::new(),
            visual_feedback_manager: visual_feedback::VisualFeedbackManager::new(),
            performance_settings: VisualPerformanceSettings::default(),
            quality_settings: VisualQualitySettings::default(),
        }
    }

    /// Update visual effects
    pub fn update(&mut self, delta_time: f32) {
        self.particle_manager.update(delta_time);
        self.lighting_manager.update(delta_time);
        self.post_processing_manager.update(delta_time);
        self.screen_effects_manager.update(delta_time);
        self.visual_feedback_manager.update(delta_time);
    }

    /// Set performance settings
    pub fn set_performance_settings(&mut self, settings: VisualPerformanceSettings) {
        self.performance_settings = settings;
        self.apply_performance_settings();
    }

    /// Set quality settings
    pub fn set_quality_settings(&mut self, settings: VisualQualitySettings) {
        self.quality_settings = settings;
        self.apply_quality_settings();
    }

    /// Apply performance settings
    fn apply_performance_settings(&mut self) {
        self.particle_manager.set_max_particles(self.performance_settings.max_particles);
        self.particle_manager.set_culling_enabled(self.performance_settings.enable_particle_culling);
        self.lighting_manager.set_max_lights(self.performance_settings.max_lights);
        self.lighting_manager.set_culling_enabled(self.performance_settings.enable_light_culling);
    }

    /// Apply quality settings
    fn apply_quality_settings(&mut self) {
        self.particle_manager.set_quality(self.quality_settings.particle_quality);
        self.lighting_manager.set_quality(self.quality_settings.lighting_quality);
        self.post_processing_manager.set_quality(self.quality_settings.post_processing_quality);
        self.lighting_manager.set_shadow_quality(self.quality_settings.shadow_quality);
    }

    /// Get current performance metrics
    pub fn get_performance_metrics(&self) -> VisualPerformanceMetrics {
        VisualPerformanceMetrics {
            active_particles: self.particle_manager.get_active_particle_count(),
            active_lights: self.lighting_manager.get_active_light_count(),
            frame_time: 0.0, // This would be calculated from actual frame timing
            memory_usage: 0, // This would be calculated from actual memory usage
        }
    }
}

/// Performance metrics for visual effects
#[derive(Debug, Clone)]
pub struct VisualPerformanceMetrics {
    /// Number of active particles
    pub active_particles: usize,
    /// Number of active lights
    pub active_lights: usize,
    /// Frame time in milliseconds
    pub frame_time: f32,
    /// Memory usage in bytes
    pub memory_usage: usize,
}