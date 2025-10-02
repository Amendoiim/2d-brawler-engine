//! Audio Polish System
//! 
//! This module provides comprehensive audio polish, spatial audio, reverb, and dynamic mixing
//! for the 2D brawler game.

pub mod spatial_audio;
pub mod reverb;
pub mod dynamic_mixing;
pub mod audio_manager;
pub mod audio_effects;
pub mod audio_processing;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Re-export main components
pub use spatial_audio::SpatialAudioManager;
pub use reverb::ReverbManager;
pub use dynamic_mixing::DynamicMixingManager;
pub use audio_manager::AudioManager;
pub use audio_effects::AudioEffectsManager;
pub use audio_processing::AudioProcessingManager;

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Enable audio system
    pub audio_enabled: bool,
    /// Master volume (0.0 to 1.0)
    pub master_volume: f32,
    /// SFX volume (0.0 to 1.0)
    pub sfx_volume: f32,
    /// Music volume (0.0 to 1.0)
    pub music_volume: f32,
    /// Voice volume (0.0 to 1.0)
    pub voice_volume: f32,
    /// Ambient volume (0.0 to 1.0)
    pub ambient_volume: f32,
    /// Enable spatial audio
    pub spatial_audio_enabled: bool,
    /// Enable reverb
    pub reverb_enabled: bool,
    /// Enable dynamic mixing
    pub dynamic_mixing_enabled: bool,
    /// Audio quality
    pub audio_quality: AudioQuality,
    /// Sample rate
    pub sample_rate: u32,
    /// Buffer size
    pub buffer_size: u32,
    /// Number of channels
    pub channels: u32,
    /// Enable 3D audio
    pub enable_3d_audio: bool,
    /// Audio device
    pub audio_device: String,
    /// Enable audio effects
    pub audio_effects_enabled: bool,
    /// Enable audio processing
    pub audio_processing_enabled: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            audio_enabled: true,
            master_volume: 1.0,
            sfx_volume: 0.8,
            music_volume: 0.7,
            voice_volume: 0.9,
            ambient_volume: 0.6,
            spatial_audio_enabled: true,
            reverb_enabled: true,
            dynamic_mixing_enabled: true,
            audio_quality: AudioQuality::High,
            sample_rate: 44100,
            buffer_size: 1024,
            channels: 2,
            enable_3d_audio: true,
            audio_device: "default".to_string(),
            audio_effects_enabled: true,
            audio_processing_enabled: true,
        }
    }
}

/// Audio quality levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioQuality {
    /// Low quality - maximum performance
    Low,
    /// Medium quality - balanced performance
    Medium,
    /// High quality - good performance
    High,
    /// Ultra quality - maximum quality
    Ultra,
}

/// Audio source types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioSourceType {
    /// Sound effect
    SFX,
    /// Background music
    Music,
    /// Voice/voiceover
    Voice,
    /// Ambient sound
    Ambient,
    /// UI sound
    UI,
    /// Footstep sound
    Footstep,
    /// Weapon sound
    Weapon,
    /// Environmental sound
    Environmental,
}

/// Audio source states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioSourceState {
    /// Stopped
    Stopped,
    /// Playing
    Playing,
    /// Paused
    Paused,
    /// Fading in
    FadingIn,
    /// Fading out
    FadingOut,
    /// Looping
    Looping,
}

/// Audio events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioEvent {
    /// Audio source started playing
    SourceStarted { source_id: String, source_type: AudioSourceType },
    /// Audio source stopped playing
    SourceStopped { source_id: String, source_type: AudioSourceType },
    /// Audio source paused
    SourcePaused { source_id: String, source_type: AudioSourceType },
    /// Audio source resumed
    SourceResumed { source_id: String, source_type: AudioSourceType },
    /// Audio source finished playing
    SourceFinished { source_id: String, source_type: AudioSourceType },
    /// Audio source looped
    SourceLooped { source_id: String, source_type: AudioSourceType },
    /// Volume changed
    VolumeChanged { channel: String, old_volume: f32, new_volume: f32 },
    /// Audio quality changed
    QualityChanged { old_quality: AudioQuality, new_quality: AudioQuality },
    /// Audio device changed
    DeviceChanged { old_device: String, new_device: String },
    /// Spatial audio enabled/disabled
    SpatialAudioToggled { enabled: bool },
    /// Reverb enabled/disabled
    ReverbToggled { enabled: bool },
    /// Dynamic mixing enabled/disabled
    DynamicMixingToggled { enabled: bool },
}

/// Audio result type
pub type AudioResult<T> = Result<T, AudioError>;

/// Audio error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioError {
    /// Audio system not initialized
    NotInitialized,
    /// Audio device not found
    DeviceNotFound(String),
    /// Audio source not found
    SourceNotFound(String),
    /// Invalid audio format
    InvalidFormat(String),
    /// Audio file not found
    FileNotFound(String),
    /// Audio file corrupted
    FileCorrupted(String),
    /// Audio buffer overflow
    BufferOverflow,
    /// Audio buffer underflow
    BufferUnderflow,
    /// Audio device error
    DeviceError(String),
    /// Audio processing error
    ProcessingError(String),
    /// Invalid configuration
    InvalidConfig(String),
    /// Unknown error
    Unknown(String),
}

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioError::NotInitialized => write!(f, "Audio system not initialized"),
            AudioError::DeviceNotFound(device) => write!(f, "Audio device not found: {}", device),
            AudioError::SourceNotFound(source) => write!(f, "Audio source not found: {}", source),
            AudioError::InvalidFormat(format) => write!(f, "Invalid audio format: {}", format),
            AudioError::FileNotFound(file) => write!(f, "Audio file not found: {}", file),
            AudioError::FileCorrupted(file) => write!(f, "Audio file corrupted: {}", file),
            AudioError::BufferOverflow => write!(f, "Audio buffer overflow"),
            AudioError::BufferUnderflow => write!(f, "Audio buffer underflow"),
            AudioError::DeviceError(error) => write!(f, "Audio device error: {}", error),
            AudioError::ProcessingError(error) => write!(f, "Audio processing error: {}", error),
            AudioError::InvalidConfig(config) => write!(f, "Invalid audio configuration: {}", config),
            AudioError::Unknown(error) => write!(f, "Unknown audio error: {}", error),
        }
    }
}

impl std::error::Error for AudioError {}