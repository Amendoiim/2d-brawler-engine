//! Dynamic Music System
//! 
//! This module provides a world-class 4-stem adaptive music system with real-time analysis,
//! intelligent transitions, and dynamic music states for immersive gameplay experiences.

pub mod music_analyzer;
pub mod music_stems;
pub mod music_transitions;
pub mod music_states;
pub mod music_manager;
pub mod music_effects;
pub mod music_sequencer;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Re-export main components
pub use music_analyzer::MusicAnalyzer;
pub use music_stems::MusicStems;
pub use music_transitions::MusicTransitions;
pub use music_states::MusicStates;
pub use music_manager::DynamicMusicManager;
pub use music_effects::MusicEffects;
pub use music_sequencer::MusicSequencer;

/// Dynamic music configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicMusicConfig {
    /// Enable dynamic music system
    pub enabled: bool,
    /// Number of music stems (1-4)
    pub stem_count: u8,
    /// Default volume for all stems
    pub default_volume: f32,
    /// Transition time (seconds)
    pub transition_time: f32,
    /// Analysis window size (samples)
    pub analysis_window_size: usize,
    /// Analysis update rate (Hz)
    pub analysis_update_rate: f32,
    /// Enable real-time analysis
    pub enable_real_time_analysis: bool,
    /// Enable intelligent transitions
    pub enable_intelligent_transitions: bool,
    /// Enable dynamic music states
    pub enable_dynamic_states: bool,
    /// Enable music effects
    pub enable_music_effects: bool,
    /// Enable spatial audio
    pub enable_spatial_audio: bool,
    /// Music quality
    pub music_quality: MusicQuality,
    /// Sample rate
    pub sample_rate: u32,
    /// Buffer size
    pub buffer_size: u32,
    /// Enable crossfading
    pub enable_crossfading: bool,
    /// Crossfade time (seconds)
    pub crossfade_time: f32,
    /// Enable beat synchronization
    pub enable_beat_sync: bool,
    /// Enable phrase synchronization
    pub enable_phrase_sync: bool,
    /// Enable key detection
    pub enable_key_detection: bool,
    /// Enable mood detection
    pub enable_mood_detection: bool,
    /// Enable energy detection
    pub enable_energy_detection: bool,
}

/// Music quality levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MusicQuality {
    /// Low quality - maximum performance
    Low,
    /// Medium quality - balanced performance
    Medium,
    /// High quality - good performance
    High,
    /// Ultra quality - maximum quality
    Ultra,
}

/// Music stem types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MusicStemType {
    /// Bass and drums (foundation)
    Foundation,
    /// Rhythm and percussion
    Rhythm,
    /// Melody and lead instruments
    Melody,
    /// Harmony and backing instruments
    Harmony,
    /// Ambient and atmospheric
    Ambient,
    /// Effects and textures
    Effects,
    /// Custom stem
    Custom(String),
}

/// Music state types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MusicStateType {
    /// Calm and peaceful
    Calm,
    /// Building tension
    Tension,
    /// Active combat
    Combat,
    /// Intense action
    Intense,
    /// Boss battle
    Boss,
    /// Victory celebration
    Victory,
    /// Defeat/Game Over
    Defeat,
    /// Menu/UI
    Menu,
    /// Exploration
    Exploration,
    /// Puzzle solving
    Puzzle,
    /// Cutscene
    Cutscene,
    /// Custom state
    Custom(String),
}

/// Music transition types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MusicTransitionType {
    /// Fade in
    FadeIn,
    /// Fade out
    FadeOut,
    /// Crossfade
    Crossfade,
    /// Beat synchronized
    BeatSync,
    /// Phrase synchronized
    PhraseSync,
    /// Measure synchronized
    MeasureSync,
    /// Instant
    Instant,
    /// Custom transition
    Custom(String),
}

/// Music events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MusicEvent {
    /// Music state changed
    StateChanged { old_state: MusicStateType, new_state: MusicStateType },
    /// Stem activated
    StemActivated { stem_type: MusicStemType, volume: f32 },
    /// Stem deactivated
    StemDeactivated { stem_type: MusicStemType },
    /// Stem volume changed
    StemVolumeChanged { stem_type: MusicStemType, old_volume: f32, new_volume: f32 },
    /// Transition started
    TransitionStarted { transition_type: MusicTransitionType, duration: f32 },
    /// Transition completed
    TransitionCompleted { transition_type: MusicTransitionType },
    /// Beat detected
    BeatDetected { bpm: f32, confidence: f32 },
    /// Key detected
    KeyDetected { key: String, confidence: f32 },
    /// Mood detected
    MoodDetected { mood: String, confidence: f32 },
    /// Energy level changed
    EnergyLevelChanged { old_energy: f32, new_energy: f32 },
    /// Music started
    MusicStarted,
    /// Music stopped
    MusicStopped,
    /// Music paused
    MusicPaused,
    /// Music resumed
    MusicResumed,
}

/// Music result type
pub type MusicResult<T> = Result<T, MusicError>;

/// Music error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MusicError {
    /// Music system not initialized
    NotInitialized,
    /// Invalid stem type
    InvalidStemType(String),
    /// Invalid music state
    InvalidMusicState(String),
    /// Invalid transition type
    InvalidTransitionType(String),
    /// Stem not found
    StemNotFound(String),
    /// Music file not found
    MusicFileNotFound(String),
    /// Music file corrupted
    MusicFileCorrupted(String),
    /// Invalid configuration
    InvalidConfig(String),
    /// Analysis error
    AnalysisError(String),
    /// Transition error
    TransitionError(String),
    /// Playback error
    PlaybackError(String),
    /// Unknown error
    Unknown(String),
}

impl std::fmt::Display for MusicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MusicError::NotInitialized => write!(f, "Music system not initialized"),
            MusicError::InvalidStemType(stem) => write!(f, "Invalid stem type: {}", stem),
            MusicError::InvalidMusicState(state) => write!(f, "Invalid music state: {}", state),
            MusicError::InvalidTransitionType(transition) => write!(f, "Invalid transition type: {}", transition),
            MusicError::StemNotFound(stem) => write!(f, "Stem not found: {}", stem),
            MusicError::MusicFileNotFound(file) => write!(f, "Music file not found: {}", file),
            MusicError::MusicFileCorrupted(file) => write!(f, "Music file corrupted: {}", file),
            MusicError::InvalidConfig(config) => write!(f, "Invalid configuration: {}", config),
            MusicError::AnalysisError(error) => write!(f, "Analysis error: {}", error),
            MusicError::TransitionError(error) => write!(f, "Transition error: {}", error),
            MusicError::PlaybackError(error) => write!(f, "Playback error: {}", error),
            MusicError::Unknown(error) => write!(f, "Unknown error: {}", error),
        }
    }
}

impl std::error::Error for MusicError {}

impl Default for DynamicMusicConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            stem_count: 4,
            default_volume: 0.7,
            transition_time: 2.0,
            analysis_window_size: 1024,
            analysis_update_rate: 60.0,
            enable_real_time_analysis: true,
            enable_intelligent_transitions: true,
            enable_dynamic_states: true,
            enable_music_effects: true,
            enable_spatial_audio: true,
            music_quality: MusicQuality::High,
            sample_rate: 44100,
            buffer_size: 1024,
            enable_crossfading: true,
            crossfade_time: 1.0,
            enable_beat_sync: true,
            enable_phrase_sync: true,
            enable_key_detection: true,
            enable_mood_detection: true,
            enable_energy_detection: true,
        }
    }
}
