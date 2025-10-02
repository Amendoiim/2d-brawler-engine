//! Sound Test System
//! 
//! This module provides a classic 16-bit brawler style sound test system with SFX, BGM, Voice, and Ambient categories.

pub mod sound_test_manager;
pub mod sound_categories;
pub mod playback_controls;
pub mod waveform_display;
pub mod sound_library;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Re-export main components
pub use sound_test_manager::SoundTestManager;
pub use sound_categories::SoundCategory;
pub use playback_controls::PlaybackControls;
pub use waveform_display::WaveformDisplay;
pub use sound_library::SoundLibrary;

/// Sound test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundTestConfig {
    /// Enable sound test system
    pub enabled: bool,
    /// Default volume for sound test
    pub default_volume: f32,
    /// Default playback speed
    pub default_playback_speed: f32,
    /// Enable waveform display
    pub enable_waveform_display: bool,
    /// Enable playback controls
    pub enable_playback_controls: bool,
    /// Enable sound library
    pub enable_sound_library: bool,
    /// Maximum simultaneous sounds
    pub max_simultaneous_sounds: usize,
    /// Auto-stop after duration
    pub auto_stop_duration: Option<f32>,
    /// Enable looping
    pub enable_looping: bool,
    /// Enable pitch control
    pub enable_pitch_control: bool,
    /// Enable volume control
    pub enable_volume_control: bool,
    /// Enable pan control
    pub enable_pan_control: bool,
    /// Enable effects
    pub enable_effects: bool,
    /// Enable spatial audio
    pub enable_spatial_audio: bool,
    /// UI scale
    pub ui_scale: f32,
    /// Theme
    pub theme: String,
}

impl Default for SoundTestConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_volume: 0.8,
            default_playback_speed: 1.0,
            enable_waveform_display: true,
            enable_playback_controls: true,
            enable_sound_library: true,
            max_simultaneous_sounds: 8,
            auto_stop_duration: Some(30.0),
            enable_looping: true,
            enable_pitch_control: true,
            enable_volume_control: true,
            enable_pan_control: true,
            enable_effects: true,
            enable_spatial_audio: true,
            ui_scale: 1.0,
            theme: "classic".to_string(),
        }
    }
}

/// Sound test events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SoundTestEvent {
    /// Sound started playing
    SoundStarted { sound_id: String, category: SoundCategory },
    /// Sound stopped playing
    SoundStopped { sound_id: String, category: SoundCategory },
    /// Sound paused
    SoundPaused { sound_id: String, category: SoundCategory },
    /// Sound resumed
    SoundResumed { sound_id: String, category: SoundCategory },
    /// Sound looped
    SoundLooped { sound_id: String, category: SoundCategory },
    /// Volume changed
    VolumeChanged { sound_id: String, old_volume: f32, new_volume: f32 },
    /// Pitch changed
    PitchChanged { sound_id: String, old_pitch: f32, new_pitch: f32 },
    /// Pan changed
    PanChanged { sound_id: String, old_pan: f32, new_pan: f32 },
    /// Category changed
    CategoryChanged { old_category: SoundCategory, new_category: SoundCategory },
    /// Playback speed changed
    PlaybackSpeedChanged { old_speed: f32, new_speed: f32 },
    /// Sound test started
    SoundTestStarted,
    /// Sound test stopped
    SoundTestStopped,
    /// Sound test paused
    SoundTestPaused,
    /// Sound test resumed
    SoundTestResumed,
    /// Sound test reset
    SoundTestReset,
    /// Sound test exported
    SoundTestExported { file_path: String },
    /// Sound test imported
    SoundTestImported { file_path: String },
}

/// Sound test result type
pub type SoundTestResult<T> = Result<T, SoundTestError>;

/// Sound test error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SoundTestError {
    /// Sound test system not initialized
    NotInitialized,
    /// Sound not found
    SoundNotFound(String),
    /// Category not found
    CategoryNotFound(String),
    /// Invalid sound format
    InvalidSoundFormat(String),
    /// Sound file not found
    SoundFileNotFound(String),
    /// Sound file corrupted
    SoundFileCorrupted(String),
    /// Maximum sounds exceeded
    MaximumSoundsExceeded,
    /// Invalid configuration
    InvalidConfig(String),
    /// Playback error
    PlaybackError(String),
    /// Export error
    ExportError(String),
    /// Import error
    ImportError(String),
    /// Unknown error
    Unknown(String),
}

impl std::fmt::Display for SoundTestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoundTestError::NotInitialized => write!(f, "Sound test system not initialized"),
            SoundTestError::SoundNotFound(id) => write!(f, "Sound not found: {}", id),
            SoundTestError::CategoryNotFound(category) => write!(f, "Category not found: {}", category),
            SoundTestError::InvalidSoundFormat(format) => write!(f, "Invalid sound format: {}", format),
            SoundTestError::SoundFileNotFound(file) => write!(f, "Sound file not found: {}", file),
            SoundTestError::SoundFileCorrupted(file) => write!(f, "Sound file corrupted: {}", file),
            SoundTestError::MaximumSoundsExceeded => write!(f, "Maximum sounds exceeded"),
            SoundTestError::InvalidConfig(config) => write!(f, "Invalid configuration: {}", config),
            SoundTestError::PlaybackError(error) => write!(f, "Playback error: {}", error),
            SoundTestError::ExportError(error) => write!(f, "Export error: {}", error),
            SoundTestError::ImportError(error) => write!(f, "Import error: {}", error),
            SoundTestError::Unknown(error) => write!(f, "Unknown error: {}", error),
        }
    }
}

impl std::error::Error for SoundTestError {}
