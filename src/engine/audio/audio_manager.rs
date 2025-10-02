//! Audio Manager
//! 
//! This module provides the main audio management system that coordinates all audio subsystems.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::{
    AudioConfig, AudioSourceType, AudioSourceState, AudioEvent, AudioResult, AudioError, AudioQuality
};

use crate::engine::audio::{
    spatial_audio::SpatialAudioManager,
    reverb::ReverbManager,
    dynamic_mixing::DynamicMixingManager,
    audio_effects::AudioEffectsManager,
    audio_processing::AudioProcessingManager,
};

/// Audio manager
pub struct AudioManager {
    /// Audio configuration
    pub config: AudioConfig,
    /// Spatial audio manager
    pub spatial_audio: SpatialAudioManager,
    /// Reverb manager
    pub reverb: ReverbManager,
    /// Dynamic mixing manager
    pub dynamic_mixing: DynamicMixingManager,
    /// Audio effects manager
    pub audio_effects: AudioEffectsManager,
    /// Audio processing manager
    pub audio_processing: AudioProcessingManager,
    /// Audio sources
    pub audio_sources: HashMap<String, AudioSource>,
    /// Audio buffers
    pub audio_buffers: HashMap<String, AudioBuffer>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&AudioEvent) + Send + Sync>>,
}

/// Audio source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSource {
    /// Source ID
    pub id: String,
    /// Source type
    pub source_type: AudioSourceType,
    /// Source state
    pub state: AudioSourceState,
    /// Volume (0.0 to 1.0)
    pub volume: f32,
    /// Pitch (0.1 to 10.0)
    pub pitch: f32,
    /// Pan (-1.0 to 1.0)
    pub pan: f32,
    /// Loop
    pub loop_audio: bool,
    /// Position (seconds)
    pub position: f32,
    /// Duration (seconds)
    pub duration: f32,
    /// Buffer ID
    pub buffer_id: String,
    /// Position (3D)
    pub position_3d: (f32, f32, f32),
    /// Velocity (3D)
    pub velocity_3d: (f32, f32, f32),
    /// Direction (3D)
    pub direction_3d: (f32, f32, f32),
    /// Rolloff factor
    pub rolloff_factor: f32,
    /// Reference distance
    pub reference_distance: f32,
    /// Maximum distance
    pub max_distance: f32,
    /// Cone inner angle
    pub cone_inner_angle: f32,
    /// Cone outer angle
    pub cone_outer_angle: f32,
    /// Cone outer volume
    pub cone_outer_volume: f32,
    /// Relative position
    pub relative: bool,
    /// Mute
    pub mute: bool,
    /// Pause
    pub pause: bool,
    /// Priority
    pub priority: AudioPriority,
}

/// Audio buffer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBuffer {
    /// Buffer ID
    pub id: String,
    /// Buffer data
    pub data: Vec<u8>,
    /// Sample rate
    pub sample_rate: u32,
    /// Bit depth
    pub bit_depth: u32,
    /// Channels
    pub channels: u32,
    /// Duration (seconds)
    pub duration: f32,
    /// Size (bytes)
    pub size: usize,
    /// Format
    pub format: String,
    /// Quality
    pub quality: AudioQuality,
}

/// Audio priority
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioPriority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

impl AudioManager {
    /// Create a new audio manager
    pub fn new(config: AudioConfig) -> Self {
        Self {
            config,
            spatial_audio: SpatialAudioManager::new(),
            reverb: ReverbManager::new(),
            dynamic_mixing: DynamicMixingManager::new(),
            audio_effects: AudioEffectsManager::new(),
            audio_processing: AudioProcessingManager::new(),
            audio_sources: HashMap::new(),
            audio_buffers: HashMap::new(),
            event_handlers: Vec::new(),
        }
    }

    /// Update audio manager
    pub fn update(&mut self, delta_time: f32) -> AudioResult<()> {
        // Update subsystems
        self.spatial_audio.update(delta_time)?;
        self.reverb.update(delta_time)?;
        self.dynamic_mixing.update(delta_time)?;
        self.audio_effects.update(delta_time)?;
        self.audio_processing.update(delta_time)?;

        // Update audio sources
        self.update_audio_sources(delta_time)?;

        // Process audio
        self.process_audio()?;

        Ok(())
    }

    /// Create audio source
    pub fn create_audio_source(&mut self, id: &str, source_type: AudioSourceType, buffer_id: &str) -> AudioResult<()> {
        if self.audio_sources.contains_key(id) {
            return Err(AudioError::InvalidConfig(format!("Audio source '{}' already exists", id)));
        }

        if !self.audio_buffers.contains_key(buffer_id) {
            return Err(AudioError::SourceNotFound(buffer_id.to_string()));
        }

        let source = AudioSource {
            id: id.to_string(),
            source_type,
            state: AudioSourceState::Stopped,
            volume: 1.0,
            pitch: 1.0,
            pan: 0.0,
            loop_audio: false,
            position: 0.0,
            duration: 0.0,
            buffer_id: buffer_id.to_string(),
            position_3d: (0.0, 0.0, 0.0),
            velocity_3d: (0.0, 0.0, 0.0),
            direction_3d: (0.0, 0.0, -1.0),
            rolloff_factor: 1.0,
            reference_distance: 1.0,
            max_distance: 100.0,
            cone_inner_angle: 360.0,
            cone_outer_angle: 360.0,
            cone_outer_volume: 0.0,
            relative: false,
            mute: false,
            pause: false,
            priority: AudioPriority::Medium,
        };

        self.audio_sources.insert(id.to_string(), source);
        Ok(())
    }

    /// Remove audio source
    pub fn remove_audio_source(&mut self, id: &str) -> AudioResult<()> {
        if !self.audio_sources.contains_key(id) {
            return Err(AudioError::SourceNotFound(id.to_string()));
        }

        self.audio_sources.remove(id);
        Ok(())
    }

    /// Get audio source
    pub fn get_audio_source(&self, id: &str) -> Option<&AudioSource> {
        self.audio_sources.get(id)
    }

    /// Get audio source mutably
    pub fn get_audio_source_mut(&mut self, id: &str) -> Option<&mut AudioSource> {
        self.audio_sources.get_mut(id)
    }

    /// Play audio source
    pub fn play_audio_source(&mut self, id: &str) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.state = AudioSourceState::Playing;
        source.position = 0.0;

        // Emit event
        self.emit_event(AudioEvent::SourceStarted {
            source_id: id.to_string(),
            source_type: source.source_type.clone(),
        });

        Ok(())
    }

    /// Stop audio source
    pub fn stop_audio_source(&mut self, id: &str) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.state = AudioSourceState::Stopped;
        source.position = 0.0;

        // Emit event
        self.emit_event(AudioEvent::SourceStopped {
            source_id: id.to_string(),
            source_type: source.source_type.clone(),
        });

        Ok(())
    }

    /// Pause audio source
    pub fn pause_audio_source(&mut self, id: &str) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.state = AudioSourceState::Paused;
        source.pause = true;

        // Emit event
        self.emit_event(AudioEvent::SourcePaused {
            source_id: id.to_string(),
            source_type: source.source_type.clone(),
        });

        Ok(())
    }

    /// Resume audio source
    pub fn resume_audio_source(&mut self, id: &str) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.state = AudioSourceState::Playing;
        source.pause = false;

        // Emit event
        self.emit_event(AudioEvent::SourceResumed {
            source_id: id.to_string(),
            source_type: source.source_type.clone(),
        });

        Ok(())
    }

    /// Set audio source volume
    pub fn set_audio_source_volume(&mut self, id: &str, volume: f32) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.volume = volume.max(0.0).min(1.0);
        Ok(())
    }

    /// Set audio source pitch
    pub fn set_audio_source_pitch(&mut self, id: &str, pitch: f32) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.pitch = pitch.max(0.1).min(10.0);
        Ok(())
    }

    /// Set audio source pan
    pub fn set_audio_source_pan(&mut self, id: &str, pan: f32) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.pan = pan.max(-1.0).min(1.0);
        Ok(())
    }

    /// Set audio source position
    pub fn set_audio_source_position(&mut self, id: &str, position: f32) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.position = position.max(0.0);
        Ok(())
    }

    /// Set audio source 3D position
    pub fn set_audio_source_3d_position(&mut self, id: &str, position: (f32, f32, f32)) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.position_3d = position;
        Ok(())
    }

    /// Set audio source 3D velocity
    pub fn set_audio_source_3d_velocity(&mut self, id: &str, velocity: (f32, f32, f32)) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.velocity_3d = velocity;
        Ok(())
    }

    /// Set audio source 3D direction
    pub fn set_audio_source_3d_direction(&mut self, id: &str, direction: (f32, f32, f32)) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.direction_3d = direction;
        Ok(())
    }

    /// Set audio source loop
    pub fn set_audio_source_loop(&mut self, id: &str, loop_audio: bool) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.loop_audio = loop_audio;
        Ok(())
    }

    /// Set audio source mute
    pub fn set_audio_source_mute(&mut self, id: &str, mute: bool) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.mute = mute;
        Ok(())
    }

    /// Set audio source priority
    pub fn set_audio_source_priority(&mut self, id: &str, priority: AudioPriority) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(id)
            .ok_or_else(|| AudioError::SourceNotFound(id.to_string()))?;

        source.priority = priority;
        Ok(())
    }

    /// Load audio buffer
    pub fn load_audio_buffer(&mut self, id: &str, data: Vec<u8>, sample_rate: u32, bit_depth: u32, channels: u32) -> AudioResult<()> {
        if self.audio_buffers.contains_key(id) {
            return Err(AudioError::InvalidConfig(format!("Audio buffer '{}' already exists", id)));
        }

        let duration = (data.len() as f32) / (sample_rate as f32 * (bit_depth as f32 / 8.0) * channels as f32);
        let buffer = AudioBuffer {
            id: id.to_string(),
            data,
            sample_rate,
            bit_depth,
            channels,
            duration,
            size: 0, // Will be set below
            format: "unknown".to_string(),
            quality: AudioQuality::High,
        };

        self.audio_buffers.insert(id.to_string(), buffer);
        Ok(())
    }

    /// Unload audio buffer
    pub fn unload_audio_buffer(&mut self, id: &str) -> AudioResult<()> {
        if !self.audio_buffers.contains_key(id) {
            return Err(AudioError::SourceNotFound(id.to_string()));
        }

        self.audio_buffers.remove(id);
        Ok(())
    }

    /// Get audio buffer
    pub fn get_audio_buffer(&self, id: &str) -> Option<&AudioBuffer> {
        self.audio_buffers.get(id)
    }

    /// Set master volume
    pub fn set_master_volume(&mut self, volume: f32) {
        let old_volume = self.config.master_volume;
        self.config.master_volume = volume.max(0.0).min(1.0);

        // Emit event
        self.emit_event(AudioEvent::VolumeChanged {
            channel: "master".to_string(),
            old_volume,
            new_volume: self.config.master_volume,
        });
    }

    /// Set SFX volume
    pub fn set_sfx_volume(&mut self, volume: f32) {
        let old_volume = self.config.sfx_volume;
        self.config.sfx_volume = volume.max(0.0).min(1.0);

        // Emit event
        self.emit_event(AudioEvent::VolumeChanged {
            channel: "sfx".to_string(),
            old_volume,
            new_volume: self.config.sfx_volume,
        });
    }

    /// Set music volume
    pub fn set_music_volume(&mut self, volume: f32) {
        let old_volume = self.config.music_volume;
        self.config.music_volume = volume.max(0.0).min(1.0);

        // Emit event
        self.emit_event(AudioEvent::VolumeChanged {
            channel: "music".to_string(),
            old_volume,
            new_volume: self.config.music_volume,
        });
    }

    /// Set voice volume
    pub fn set_voice_volume(&mut self, volume: f32) {
        let old_volume = self.config.voice_volume;
        self.config.voice_volume = volume.max(0.0).min(1.0);

        // Emit event
        self.emit_event(AudioEvent::VolumeChanged {
            channel: "voice".to_string(),
            old_volume,
            new_volume: self.config.voice_volume,
        });
    }

    /// Set ambient volume
    pub fn set_ambient_volume(&mut self, volume: f32) {
        let old_volume = self.config.ambient_volume;
        self.config.ambient_volume = volume.max(0.0).min(1.0);

        // Emit event
        self.emit_event(AudioEvent::VolumeChanged {
            channel: "ambient".to_string(),
            old_volume,
            new_volume: self.config.ambient_volume,
        });
    }

    /// Set audio quality
    pub fn set_audio_quality(&mut self, quality: AudioQuality) {
        let old_quality = self.config.audio_quality.clone();
        self.config.audio_quality = quality;

        // Emit event
        self.emit_event(AudioEvent::QualityChanged {
            old_quality,
            new_quality: quality,
        });
    }

    /// Set audio device
    pub fn set_audio_device(&mut self, device: &str) -> AudioResult<()> {
        let old_device = self.config.audio_device.clone();
        self.config.audio_device = device.to_string();

        // Emit event
        self.emit_event(AudioEvent::DeviceChanged {
            old_device,
            new_device: device.to_string(),
        });

        Ok(())
    }

    /// Enable/disable spatial audio
    pub fn set_spatial_audio_enabled(&mut self, enabled: bool) {
        self.config.spatial_audio_enabled = enabled;

        // Emit event
        self.emit_event(AudioEvent::SpatialAudioToggled { enabled });
    }

    /// Enable/disable reverb
    pub fn set_reverb_enabled(&mut self, enabled: bool) {
        self.config.reverb_enabled = enabled;

        // Emit event
        self.emit_event(AudioEvent::ReverbToggled { enabled });
    }

    /// Enable/disable dynamic mixing
    pub fn set_dynamic_mixing_enabled(&mut self, enabled: bool) {
        self.config.dynamic_mixing_enabled = enabled;

        // Emit event
        self.emit_event(AudioEvent::DynamicMixingToggled { enabled });
    }

    /// Get audio configuration
    pub fn get_config(&self) -> &AudioConfig {
        &self.config
    }

    /// Set audio configuration
    pub fn set_config(&mut self, config: AudioConfig) {
        self.config = config;
    }

    /// Update audio sources
    fn update_audio_sources(&mut self, delta_time: f32) -> AudioResult<()> {
        for (id, source) in self.audio_sources.iter_mut() {
            if source.state == AudioSourceState::Playing && !source.pause {
                // Update position
                source.position += delta_time * source.pitch;

                // Check if source finished
                if source.position >= source.duration {
                    if source.loop_audio {
                        source.position = 0.0;
                        
                        // Emit loop event
                        self.emit_event(AudioEvent::SourceLooped {
                            source_id: id.clone(),
                            source_type: source.source_type.clone(),
                        });
                    } else {
                        source.state = AudioSourceState::Stopped;
                        source.position = 0.0;
                        
                        // Emit finished event
                        self.emit_event(AudioEvent::SourceFinished {
                            source_id: id.clone(),
                            source_type: source.source_type.clone(),
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Process audio
    fn process_audio(&mut self) -> AudioResult<()> {
        // In a real implementation, this would process the audio through the audio pipeline
        // For now, we'll just simulate the processing
        
        // Process spatial audio
        if self.config.spatial_audio_enabled {
            // Process 3D audio
        }

        // Process reverb
        if self.config.reverb_enabled {
            // Process reverb effects
        }

        // Process dynamic mixing
        if self.config.dynamic_mixing_enabled {
            // Process dynamic mixing
        }

        // Process audio effects
        if self.config.audio_effects_enabled {
            // Process audio effects
        }

        // Process audio processing
        if self.config.audio_processing_enabled {
            // Process audio processing
        }

        Ok(())
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&AudioEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Emit audio event
    fn emit_event(&self, event: AudioEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new(AudioConfig::default())
    }
}
