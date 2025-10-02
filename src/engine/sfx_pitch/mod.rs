//! SFX Pitch Variation System
//! 
//! This module provides advanced real-time SFX pitch variation system with
//! pitch shifting, bending, semitone control, and multiple variation algorithms.

pub mod pitch_processor;
pub mod pitch_variations;
pub mod pitch_effects;
pub mod pitch_manager;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Re-export main components
pub use pitch_processor::PitchProcessor;
pub use pitch_variations::PitchVariations;
pub use pitch_effects::PitchEffects;
pub use pitch_manager::SFXPitchManager;

/// SFX pitch configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFXPitchConfig {
    /// Enable SFX pitch system
    pub enabled: bool,
    /// Default pitch multiplier
    pub default_pitch_multiplier: f32,
    /// Pitch range minimum
    pub pitch_range_min: f32,
    /// Pitch range maximum
    pub pitch_range_max: f32,
    /// Enable real-time processing
    pub enable_real_time_processing: bool,
    /// Enable pitch effects
    pub enable_pitch_effects: bool,
    /// Enable pitch variations
    pub enable_pitch_variations: bool,
    /// Enable pitch bending
    pub enable_pitch_bending: bool,
    /// Enable semitone control
    pub enable_semitone_control: bool,
    /// Enable microtone control
    pub enable_microtone_control: bool,
    /// Enable random variations
    pub enable_random_variations: bool,
    /// Enable preset variations
    pub enable_preset_variations: bool,
    /// Enable custom variations
    pub enable_custom_variations: bool,
    /// Pitch quality
    pub pitch_quality: PitchQuality,
    /// Sample rate
    pub sample_rate: u32,
    /// Buffer size
    pub buffer_size: u32,
    /// Maximum simultaneous pitch shifts
    pub max_simultaneous_shifts: usize,
    /// Enable pitch analysis
    pub enable_pitch_analysis: bool,
    /// Enable pitch correction
    pub enable_pitch_correction: bool,
    /// Enable pitch smoothing
    pub enable_pitch_smoothing: bool,
}

/// Pitch quality levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PitchQuality {
    /// Low quality - maximum performance
    Low,
    /// Medium quality - balanced performance
    Medium,
    /// High quality - good performance
    High,
    /// Ultra quality - maximum quality
    Ultra,
}

/// Pitch variation types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PitchVariationType {
    /// Random variation
    Random,
    /// Preset variation
    Preset,
    /// Custom variation
    Custom,
    /// Semitone variation
    Semitone,
    /// Octave variation
    Octave,
    /// Microtone variation
    Microtone,
    /// Harmonic variation
    Harmonic,
    /// Subharmonic variation
    Subharmonic,
    /// Chromatic variation
    Chromatic,
    /// Pentatonic variation
    Pentatonic,
    /// Diatonic variation
    Diatonic,
    /// Blues variation
    Blues,
    /// Jazz variation
    Jazz,
    /// Classical variation
    Classical,
    /// Electronic variation
    Electronic,
    /// Acoustic variation
    Acoustic,
    /// Synthetic variation
    Synthetic,
    /// Organic variation
    Organic,
    /// Mechanical variation
    Mechanical,
    /// Natural variation
    Natural,
    /// Artificial variation
    Artificial,
}

/// Pitch effect types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PitchEffectType {
    /// Vibrato
    Vibrato,
    /// Tremolo
    Tremolo,
    /// Wah-wah
    WahWah,
    /// Chorus
    Chorus,
    /// Flanger
    Flanger,
    /// Phaser
    Phaser,
    /// Pitch shifter
    PitchShifter,
    /// Harmonizer
    Harmonizer,
    /// Pitch bender
    PitchBender,
    /// Pitch corrector
    PitchCorrector,
    /// Pitch smoother
    PitchSmoother,
    /// Pitch analyzer
    PitchAnalyzer,
    /// Pitch tracker
    PitchTracker,
    /// Pitch detector
    PitchDetector,
    /// Pitch quantizer
    PitchQuantizer,
    /// Pitch scaler
    PitchScaler,
    /// Pitch compressor
    PitchCompressor,
    /// Pitch expander
    PitchExpander,
    /// Pitch gate
    PitchGate,
    /// Pitch limiter
    PitchLimiter,
    /// Pitch filter
    PitchFilter,
    /// Pitch EQ
    PitchEQ,
    /// Pitch reverb
    PitchReverb,
    /// Pitch delay
    PitchDelay,
    /// Pitch echo
    PitchEcho,
    /// Pitch distortion
    PitchDistortion,
    /// Pitch overdrive
    PitchOverdrive,
    /// Pitch fuzz
    PitchFuzz,
    /// Pitch bit crusher
    PitchBitCrusher,
    /// Pitch sample rate reducer
    PitchSampleRateReducer,
    /// Pitch ring modulator
    PitchRingModulator,
    /// Pitch frequency shifter
    PitchFrequencyShifter,
    /// Pitch stereo widener
    PitchStereoWidener,
    /// Pitch mono maker
    PitchMonoMaker,
    /// Pitch stereo imager
    PitchStereoImager,
    /// Pitch spatial processor
    PitchSpatialProcessor,
    /// Pitch convolution
    PitchConvolution,
    /// Pitch impulse response
    PitchImpulseResponse,
    /// Custom pitch effect
    CustomPitchEffect(String),
}

/// SFX pitch events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SFXPitchEvent {
    /// Pitch changed
    PitchChanged { sound_id: String, old_pitch: f32, new_pitch: f32 },
    /// Pitch variation applied
    PitchVariationApplied { sound_id: String, variation_type: PitchVariationType, pitch: f32 },
    /// Pitch effect applied
    PitchEffectApplied { sound_id: String, effect_type: PitchEffectType, intensity: f32 },
    /// Pitch analysis completed
    PitchAnalysisCompleted { sound_id: String, detected_pitch: f32, confidence: f32 },
    /// Pitch correction applied
    PitchCorrectionApplied { sound_id: String, old_pitch: f32, corrected_pitch: f32 },
    /// Pitch smoothing applied
    PitchSmoothingApplied { sound_id: String, smoothed_pitch: f32 },
    /// Pitch variation preset loaded
    PitchVariationPresetLoaded { preset_name: String, variations: Vec<PitchVariation> },
    /// Pitch effect preset loaded
    PitchEffectPresetLoaded { preset_name: String, effects: Vec<PitchEffect> },
    /// Pitch system enabled
    PitchSystemEnabled,
    /// Pitch system disabled
    PitchSystemDisabled,
    /// Pitch system reset
    PitchSystemReset,
}

/// Pitch variation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PitchVariation {
    /// Variation ID
    pub id: String,
    /// Variation name
    pub name: String,
    /// Variation type
    pub variation_type: PitchVariationType,
    /// Pitch multiplier
    pub pitch_multiplier: f32,
    /// Pitch offset (semitones)
    pub pitch_offset: f32,
    /// Variation probability (0.0 to 1.0)
    pub probability: f32,
    /// Variation weight (0.0 to 1.0)
    pub weight: f32,
    /// Variation enabled
    pub enabled: bool,
    /// Variation parameters
    pub parameters: HashMap<String, f32>,
}

/// Pitch effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PitchEffect {
    /// Effect ID
    pub id: String,
    /// Effect name
    pub name: String,
    /// Effect type
    pub effect_type: PitchEffectType,
    /// Effect intensity (0.0 to 1.0)
    pub intensity: f32,
    /// Effect rate (Hz)
    pub rate: f32,
    /// Effect depth (0.0 to 1.0)
    pub depth: f32,
    /// Effect phase (0.0 to 1.0)
    pub phase: f32,
    /// Effect feedback (0.0 to 1.0)
    pub feedback: f32,
    /// Effect enabled
    pub enabled: bool,
    /// Effect bypass
    pub bypass: bool,
    /// Effect parameters
    pub parameters: HashMap<String, f32>,
}

/// SFX pitch result type
pub type SFXPitchResult<T> = Result<T, SFXPitchError>;

/// SFX pitch error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SFXPitchError {
    /// SFX pitch system not initialized
    NotInitialized,
    /// Invalid pitch value
    InvalidPitch(f32),
    /// Invalid pitch range
    InvalidPitchRange { min: f32, max: f32 },
    /// Invalid variation type
    InvalidVariationType(String),
    /// Invalid effect type
    InvalidEffectType(String),
    /// Sound not found
    SoundNotFound(String),
    /// Variation not found
    VariationNotFound(String),
    /// Effect not found
    EffectNotFound(String),
    /// Preset not found
    PresetNotFound(String),
    /// Invalid configuration
    InvalidConfig(String),
    /// Processing error
    ProcessingError(String),
    /// Analysis error
    AnalysisError(String),
    /// Correction error
    CorrectionError(String),
    /// Smoothing error
    SmoothingError(String),
    /// Unknown error
    Unknown(String),
}

impl std::fmt::Display for SFXPitchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SFXPitchError::NotInitialized => write!(f, "SFX pitch system not initialized"),
            SFXPitchError::InvalidPitch(pitch) => write!(f, "Invalid pitch value: {}", pitch),
            SFXPitchError::InvalidPitchRange { min, max } => write!(f, "Invalid pitch range: {} to {}", min, max),
            SFXPitchError::InvalidVariationType(variation) => write!(f, "Invalid variation type: {}", variation),
            SFXPitchError::InvalidEffectType(effect) => write!(f, "Invalid effect type: {}", effect),
            SFXPitchError::SoundNotFound(sound) => write!(f, "Sound not found: {}", sound),
            SFXPitchError::VariationNotFound(variation) => write!(f, "Variation not found: {}", variation),
            SFXPitchError::EffectNotFound(effect) => write!(f, "Effect not found: {}", effect),
            SFXPitchError::PresetNotFound(preset) => write!(f, "Preset not found: {}", preset),
            SFXPitchError::InvalidConfig(config) => write!(f, "Invalid configuration: {}", config),
            SFXPitchError::ProcessingError(error) => write!(f, "Processing error: {}", error),
            SFXPitchError::AnalysisError(error) => write!(f, "Analysis error: {}", error),
            SFXPitchError::CorrectionError(error) => write!(f, "Correction error: {}", error),
            SFXPitchError::SmoothingError(error) => write!(f, "Smoothing error: {}", error),
            SFXPitchError::Unknown(error) => write!(f, "Unknown error: {}", error),
        }
    }
}

impl std::error::Error for SFXPitchError {}

impl Default for SFXPitchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_pitch_multiplier: 1.0,
            pitch_range_min: 0.1,
            pitch_range_max: 4.0,
            enable_real_time_processing: true,
            enable_pitch_effects: true,
            enable_pitch_variations: true,
            enable_pitch_bending: true,
            enable_semitone_control: true,
            enable_microtone_control: true,
            enable_random_variations: true,
            enable_preset_variations: true,
            enable_custom_variations: true,
            pitch_quality: PitchQuality::High,
            sample_rate: 44100,
            buffer_size: 1024,
            max_simultaneous_shifts: 8,
            enable_pitch_analysis: true,
            enable_pitch_correction: true,
            enable_pitch_smoothing: true,
        }
    }
}
