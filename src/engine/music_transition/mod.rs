//! Music Transition System
//! 
//! This module provides professional music transition system with fade in/out,
//! crossfade, beat-synchronized transitions, and advanced transition effects.

pub mod transition_types;
pub mod transition_curves;
pub mod transition_effects;
pub mod transition_manager;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Re-export main components
pub use transition_types::TransitionType;
pub use transition_curves::TransitionCurve;
pub use transition_effects::TransitionEffect;
pub use transition_manager::MusicTransitionManager;

/// Music transition configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicTransitionConfig {
    /// Enable music transition system
    pub enabled: bool,
    /// Default transition duration
    pub default_transition_duration: f32,
    /// Minimum transition duration
    pub min_transition_duration: f32,
    /// Maximum transition duration
    pub max_transition_duration: f32,
    /// Enable fade in transitions
    pub enable_fade_in: bool,
    /// Enable fade out transitions
    pub enable_fade_out: bool,
    /// Enable crossfade transitions
    pub enable_crossfade: bool,
    /// Enable beat-synchronized transitions
    pub enable_beat_sync: bool,
    /// Enable phrase-synchronized transitions
    pub enable_phrase_sync: bool,
    /// Enable measure-synchronized transitions
    pub enable_measure_sync: bool,
    /// Enable transition effects
    pub enable_transition_effects: bool,
    /// Enable transition curves
    pub enable_transition_curves: bool,
    /// Enable transition automation
    pub enable_transition_automation: bool,
    /// Enable transition presets
    pub enable_transition_presets: bool,
    /// Transition quality
    pub transition_quality: TransitionQuality,
    /// Sample rate
    pub sample_rate: u32,
    /// Buffer size
    pub buffer_size: u32,
    /// Maximum simultaneous transitions
    pub max_simultaneous_transitions: usize,
    /// Enable transition analysis
    pub enable_transition_analysis: bool,
    /// Enable transition optimization
    pub enable_transition_optimization: bool,
}

/// Transition quality levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransitionQuality {
    /// Low quality - maximum performance
    Low,
    /// Medium quality - balanced performance
    Medium,
    /// High quality - good performance
    High,
    /// Ultra quality - maximum quality
    Ultra,
}

/// Music transition events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MusicTransitionEvent {
    /// Transition started
    TransitionStarted { 
        transition_id: String, 
        from_track: Option<String>, 
        to_track: String, 
        transition_type: TransitionType 
    },
    /// Transition progress updated
    TransitionProgressUpdated { 
        transition_id: String, 
        progress: f32, 
        time_remaining: f32 
    },
    /// Transition completed
    TransitionCompleted { 
        transition_id: String, 
        success: bool, 
        duration: f32 
    },
    /// Transition cancelled
    TransitionCancelled { 
        transition_id: String, 
        reason: String 
    },
    /// Transition effect applied
    TransitionEffectApplied { 
        transition_id: String, 
        effect_type: String, 
        intensity: f32 
    },
    /// Transition curve applied
    TransitionCurveApplied { 
        transition_id: String, 
        curve_type: String, 
        curve_value: f32 
    },
    /// Beat synchronization applied
    BeatSyncApplied { 
        transition_id: String, 
        beat_position: f32, 
        sync_accuracy: f32 
    },
    /// Phrase synchronization applied
    PhraseSyncApplied { 
        transition_id: String, 
        phrase_position: f32, 
        sync_accuracy: f32 
    },
    /// Measure synchronization applied
    MeasureSyncApplied { 
        transition_id: String, 
        measure_position: f32, 
        sync_accuracy: f32 
    },
    /// Transition preset loaded
    TransitionPresetLoaded { 
        preset_name: String, 
        transitions: Vec<TransitionPreset> 
    },
    /// Transition system enabled
    TransitionSystemEnabled,
    /// Transition system disabled
    TransitionSystemDisabled,
    /// Transition system reset
    TransitionSystemReset,
}

/// Transition preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionPreset {
    /// Preset ID
    pub id: String,
    /// Preset name
    pub name: String,
    /// Transition type
    pub transition_type: TransitionType,
    /// Transition duration
    pub duration: f32,
    /// Transition curve
    pub curve: TransitionCurve,
    /// Transition effects
    pub effects: Vec<TransitionEffect>,
    /// Beat synchronization
    pub beat_sync: bool,
    /// Phrase synchronization
    pub phrase_sync: bool,
    /// Measure synchronization
    pub measure_sync: bool,
    /// Preset enabled
    pub enabled: bool,
    /// Preset parameters
    pub parameters: HashMap<String, f32>,
}

/// Music transition result type
pub type MusicTransitionResult<T> = Result<T, MusicTransitionError>;

/// Music transition error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MusicTransitionError {
    /// Music transition system not initialized
    NotInitialized,
    /// Invalid transition duration
    InvalidTransitionDuration(f32),
    /// Invalid transition type
    InvalidTransitionType(String),
    /// Invalid transition curve
    InvalidTransitionCurve(String),
    /// Invalid transition effect
    InvalidTransitionEffect(String),
    /// Transition not found
    TransitionNotFound(String),
    /// Preset not found
    PresetNotFound(String),
    /// Track not found
    TrackNotFound(String),
    /// Invalid configuration
    InvalidConfig(String),
    /// Transition processing error
    ProcessingError(String),
    /// Transition analysis error
    AnalysisError(String),
    /// Transition optimization error
    OptimizationError(String),
    /// Beat synchronization error
    BeatSyncError(String),
    /// Phrase synchronization error
    PhraseSyncError(String),
    /// Measure synchronization error
    MeasureSyncError(String),
    /// Unknown error
    Unknown(String),
}

impl std::fmt::Display for MusicTransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MusicTransitionError::NotInitialized => write!(f, "Music transition system not initialized"),
            MusicTransitionError::InvalidTransitionDuration(duration) => write!(f, "Invalid transition duration: {}", duration),
            MusicTransitionError::InvalidTransitionType(transition_type) => write!(f, "Invalid transition type: {}", transition_type),
            MusicTransitionError::InvalidTransitionCurve(curve) => write!(f, "Invalid transition curve: {}", curve),
            MusicTransitionError::InvalidTransitionEffect(effect) => write!(f, "Invalid transition effect: {}", effect),
            MusicTransitionError::TransitionNotFound(transition) => write!(f, "Transition not found: {}", transition),
            MusicTransitionError::PresetNotFound(preset) => write!(f, "Preset not found: {}", preset),
            MusicTransitionError::TrackNotFound(track) => write!(f, "Track not found: {}", track),
            MusicTransitionError::InvalidConfig(config) => write!(f, "Invalid configuration: {}", config),
            MusicTransitionError::ProcessingError(error) => write!(f, "Processing error: {}", error),
            MusicTransitionError::AnalysisError(error) => write!(f, "Analysis error: {}", error),
            MusicTransitionError::OptimizationError(error) => write!(f, "Optimization error: {}", error),
            MusicTransitionError::BeatSyncError(error) => write!(f, "Beat synchronization error: {}", error),
            MusicTransitionError::PhraseSyncError(error) => write!(f, "Phrase synchronization error: {}", error),
            MusicTransitionError::MeasureSyncError(error) => write!(f, "Measure synchronization error: {}", error),
            MusicTransitionError::Unknown(error) => write!(f, "Unknown error: {}", error),
        }
    }
}

impl std::error::Error for MusicTransitionError {}

impl Default for MusicTransitionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_transition_duration: 2.0,
            min_transition_duration: 0.1,
            max_transition_duration: 10.0,
            enable_fade_in: true,
            enable_fade_out: true,
            enable_crossfade: true,
            enable_beat_sync: true,
            enable_phrase_sync: true,
            enable_measure_sync: true,
            enable_transition_effects: true,
            enable_transition_curves: true,
            enable_transition_automation: true,
            enable_transition_presets: true,
            transition_quality: TransitionQuality::High,
            sample_rate: 44100,
            buffer_size: 1024,
            max_simultaneous_transitions: 4,
            enable_transition_analysis: true,
            enable_transition_optimization: true,
        }
    }
}
