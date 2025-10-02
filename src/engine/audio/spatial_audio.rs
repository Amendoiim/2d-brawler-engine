//! Spatial Audio System
//! 
//! This module provides comprehensive spatial audio processing for 3D positioning and directional audio.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::{AudioResult, AudioError, AudioEvent, AudioSourceType};

/// Spatial audio manager
pub struct SpatialAudioManager {
    /// Listener position
    pub listener_position: (f32, f32, f32),
    /// Listener orientation
    pub listener_orientation: (f32, f32, f32),
    /// Listener up vector
    pub listener_up: (f32, f32, f32),
    /// Audio sources
    pub audio_sources: HashMap<String, SpatialAudioSource>,
    /// Spatial audio settings
    pub settings: SpatialAudioSettings,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&AudioEvent) + Send + Sync>>,
}

/// Spatial audio source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialAudioSource {
    /// Source ID
    pub id: String,
    /// Source type
    pub source_type: AudioSourceType,
    /// Position
    pub position: (f32, f32, f32),
    /// Velocity
    pub velocity: (f32, f32, f32),
    /// Direction
    pub direction: (f32, f32, f32),
    /// Volume
    pub volume: f32,
    /// Pitch
    pub pitch: f32,
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
    /// Loop
    pub loop_audio: bool,
    /// Playing
    pub playing: bool,
    /// Paused
    pub paused: bool,
}

/// Spatial audio settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialAudioSettings {
    /// Doppler factor
    pub doppler_factor: f32,
    /// Speed of sound
    pub speed_of_sound: f32,
    /// Distance model
    pub distance_model: DistanceModel,
    /// Rolloff factor
    pub rolloff_factor: f32,
    /// Reference distance
    pub reference_distance: f32,
    /// Maximum distance
    pub max_distance: f32,
    /// Enable doppler effect
    pub enable_doppler: bool,
    /// Enable distance attenuation
    pub enable_distance_attenuation: bool,
    /// Enable cone attenuation
    pub enable_cone_attenuation: bool,
    /// Enable occlusion
    pub enable_occlusion: bool,
    /// Occlusion factor
    pub occlusion_factor: f32,
    /// Enable obstruction
    pub enable_obstruction: bool,
    /// Obstruction factor
    pub obstruction_factor: f32,
}

/// Distance model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DistanceModel {
    /// No distance attenuation
    None,
    /// Inverse distance
    Inverse,
    /// Inverse clamped distance
    InverseClamped,
    /// Linear distance
    Linear,
    /// Linear clamped distance
    LinearClamped,
    /// Exponential distance
    Exponential,
    /// Exponential clamped distance
    ExponentialClamped,
}

impl SpatialAudioManager {
    /// Create a new spatial audio manager
    pub fn new() -> Self {
        Self {
            listener_position: (0.0, 0.0, 0.0),
            listener_orientation: (0.0, 0.0, -1.0),
            listener_up: (0.0, 1.0, 0.0),
            audio_sources: HashMap::new(),
            settings: SpatialAudioSettings::default(),
            event_handlers: Vec::new(),
        }
    }

    /// Update spatial audio manager
    pub fn update(&mut self, delta_time: f32) -> AudioResult<()> {
        // Update audio sources
        for (id, source) in self.audio_sources.iter_mut() {
            if source.playing && !source.paused {
                // Update source position based on velocity
                source.position.0 += source.velocity.0 * delta_time;
                source.position.1 += source.velocity.1 * delta_time;
                source.position.2 += source.velocity.2 * delta_time;

                // Calculate spatial audio parameters
                self.calculate_spatial_parameters(source)?;
            }
        }

        Ok(())
    }

    /// Set listener position
    pub fn set_listener_position(&mut self, position: (f32, f32, f32)) {
        self.listener_position = position;
    }

    /// Set listener orientation
    pub fn set_listener_orientation(&mut self, orientation: (f32, f32, f32)) {
        self.listener_orientation = orientation;
    }

    /// Set listener up vector
    pub fn set_listener_up(&mut self, up: (f32, f32, f32)) {
        self.listener_up = up;
    }

    /// Add audio source
    pub fn add_audio_source(&mut self, source: SpatialAudioSource) -> AudioResult<()> {
        if self.audio_sources.contains_key(&source.id) {
            return Err(AudioError::InvalidConfig(format!("Audio source '{}' already exists", source.id)));
        }

        self.audio_sources.insert(source.id.clone(), source);
        Ok(())
    }

    /// Remove audio source
    pub fn remove_audio_source(&mut self, source_id: &str) -> AudioResult<()> {
        if !self.audio_sources.contains_key(source_id) {
            return Err(AudioError::SourceNotFound(source_id.to_string()));
        }

        self.audio_sources.remove(source_id);
        Ok(())
    }

    /// Get audio source
    pub fn get_audio_source(&self, source_id: &str) -> Option<&SpatialAudioSource> {
        self.audio_sources.get(source_id)
    }

    /// Get audio source mutably
    pub fn get_audio_source_mut(&mut self, source_id: &str) -> Option<&mut SpatialAudioSource> {
        self.audio_sources.get_mut(source_id)
    }

    /// Set audio source position
    pub fn set_audio_source_position(&mut self, source_id: &str, position: (f32, f32, f32)) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(source_id)
            .ok_or_else(|| AudioError::SourceNotFound(source_id.to_string()))?;

        source.position = position;
        Ok(())
    }

    /// Set audio source velocity
    pub fn set_audio_source_velocity(&mut self, source_id: &str, velocity: (f32, f32, f32)) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(source_id)
            .ok_or_else(|| AudioError::SourceNotFound(source_id.to_string()))?;

        source.velocity = velocity;
        Ok(())
    }

    /// Set audio source direction
    pub fn set_audio_source_direction(&mut self, source_id: &str, direction: (f32, f32, f32)) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(source_id)
            .ok_or_else(|| AudioError::SourceNotFound(source_id.to_string()))?;

        source.direction = direction;
        Ok(())
    }

    /// Set audio source volume
    pub fn set_audio_source_volume(&mut self, source_id: &str, volume: f32) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(source_id)
            .ok_or_else(|| AudioError::SourceNotFound(source_id.to_string()))?;

        source.volume = volume.max(0.0).min(1.0);
        Ok(())
    }

    /// Set audio source pitch
    pub fn set_audio_source_pitch(&mut self, source_id: &str, pitch: f32) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(source_id)
            .ok_or_else(|| AudioError::SourceNotFound(source_id.to_string()))?;

        source.pitch = pitch.max(0.1).min(10.0);
        Ok(())
    }

    /// Play audio source
    pub fn play_audio_source(&mut self, source_id: &str) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(source_id)
            .ok_or_else(|| AudioError::SourceNotFound(source_id.to_string()))?;

        source.playing = true;
        source.paused = false;

        // Emit event
        self.emit_event(AudioEvent::SourceStarted {
            source_id: source_id.to_string(),
            source_type: source.source_type.clone(),
        });

        Ok(())
    }

    /// Stop audio source
    pub fn stop_audio_source(&mut self, source_id: &str) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(source_id)
            .ok_or_else(|| AudioError::SourceNotFound(source_id.to_string()))?;

        source.playing = false;
        source.paused = false;

        // Emit event
        self.emit_event(AudioEvent::SourceStopped {
            source_id: source_id.to_string(),
            source_type: source.source_type.clone(),
        });

        Ok(())
    }

    /// Pause audio source
    pub fn pause_audio_source(&mut self, source_id: &str) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(source_id)
            .ok_or_else(|| AudioError::SourceNotFound(source_id.to_string()))?;

        source.paused = true;

        // Emit event
        self.emit_event(AudioEvent::SourcePaused {
            source_id: source_id.to_string(),
            source_type: source.source_type.clone(),
        });

        Ok(())
    }

    /// Resume audio source
    pub fn resume_audio_source(&mut self, source_id: &str) -> AudioResult<()> {
        let source = self.audio_sources.get_mut(source_id)
            .ok_or_else(|| AudioError::SourceNotFound(source_id.to_string()))?;

        source.paused = false;

        // Emit event
        self.emit_event(AudioEvent::SourceResumed {
            source_id: source_id.to_string(),
            source_type: source.source_type.clone(),
        });

        Ok(())
    }

    /// Set spatial audio settings
    pub fn set_settings(&mut self, settings: SpatialAudioSettings) {
        self.settings = settings;
    }

    /// Get spatial audio settings
    pub fn get_settings(&self) -> &SpatialAudioSettings {
        &self.settings
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&AudioEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Calculate spatial audio parameters
    fn calculate_spatial_parameters(&self, source: &mut SpatialAudioSource) -> AudioResult<()> {
        if source.relative {
            return Ok(());
        }

        // Calculate distance from listener
        let distance = self.calculate_distance(source.position, self.listener_position);

        // Calculate distance attenuation
        let distance_attenuation = if self.settings.enable_distance_attenuation {
            self.calculate_distance_attenuation(distance, source)
        } else {
            1.0
        };

        // Calculate cone attenuation
        let cone_attenuation = if self.settings.enable_cone_attenuation {
            self.calculate_cone_attenuation(source)
        } else {
            1.0
        };

        // Calculate doppler effect
        let doppler_pitch = if self.settings.enable_doppler {
            self.calculate_doppler_effect(source)
        } else {
            1.0
        };

        // Apply spatial audio parameters
        source.volume *= distance_attenuation * cone_attenuation;
        source.pitch *= doppler_pitch;

        Ok(())
    }

    /// Calculate distance between two points
    fn calculate_distance(&self, pos1: (f32, f32, f32), pos2: (f32, f32, f32)) -> f32 {
        let dx = pos1.0 - pos2.0;
        let dy = pos1.1 - pos2.1;
        let dz = pos1.2 - pos2.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Calculate distance attenuation
    fn calculate_distance_attenuation(&self, distance: f32, source: &SpatialAudioSource) -> f32 {
        match self.settings.distance_model {
            DistanceModel::None => 1.0,
            DistanceModel::Inverse => {
                if distance <= 0.0 {
                    1.0
                } else {
                    source.reference_distance / (source.reference_distance + source.rolloff_factor * (distance - source.reference_distance))
                }
            },
            DistanceModel::InverseClamped => {
                if distance <= 0.0 {
                    1.0
                } else {
                    let attenuation = source.reference_distance / (source.reference_distance + source.rolloff_factor * (distance - source.reference_distance));
                    attenuation.max(0.0).min(1.0)
                }
            },
            DistanceModel::Linear => {
                if distance <= 0.0 {
                    1.0
                } else {
                    let attenuation = 1.0 - source.rolloff_factor * (distance - source.reference_distance) / (source.max_distance - source.reference_distance);
                    attenuation.max(0.0)
                }
            },
            DistanceModel::LinearClamped => {
                if distance <= 0.0 {
                    1.0
                } else {
                    let attenuation = 1.0 - source.rolloff_factor * (distance - source.reference_distance) / (source.max_distance - source.reference_distance);
                    attenuation.max(0.0).min(1.0)
                }
            },
            DistanceModel::Exponential => {
                if distance <= 0.0 {
                    1.0
                } else {
                    (distance / source.reference_distance).powf(-source.rolloff_factor)
                }
            },
            DistanceModel::ExponentialClamped => {
                if distance <= 0.0 {
                    1.0
                } else {
                    let attenuation = (distance / source.reference_distance).powf(-source.rolloff_factor);
                    attenuation.max(0.0).min(1.0)
                }
            },
        }
    }

    /// Calculate cone attenuation
    fn calculate_cone_attenuation(&self, source: &SpatialAudioSource) -> f32 {
        // Calculate angle between source direction and listener direction
        let source_to_listener = (
            self.listener_position.0 - source.position.0,
            self.listener_position.1 - source.position.1,
            self.listener_position.2 - source.position.2,
        );

        let distance = (source_to_listener.0 * source_to_listener.0 + 
                       source_to_listener.1 * source_to_listener.1 + 
                       source_to_listener.2 * source_to_listener.2).sqrt();

        if distance <= 0.0 {
            return 1.0;
        }

        let normalized_direction = (
            source_to_listener.0 / distance,
            source_to_listener.1 / distance,
            source_to_listener.2 / distance,
        );

        let dot_product = normalized_direction.0 * source.direction.0 +
                         normalized_direction.1 * source.direction.1 +
                         normalized_direction.2 * source.direction.2;

        let angle = dot_product.acos().to_degrees();

        if angle <= source.cone_inner_angle {
            1.0
        } else if angle >= source.cone_outer_angle {
            source.cone_outer_volume
        } else {
            let factor = (angle - source.cone_inner_angle) / (source.cone_outer_angle - source.cone_inner_angle);
            source.cone_outer_volume + (1.0 - source.cone_outer_volume) * (1.0 - factor)
        }
    }

    /// Calculate doppler effect
    fn calculate_doppler_effect(&self, source: &SpatialAudioSource) -> f32 {
        // Calculate relative velocity
        let relative_velocity = (
            source.velocity.0 - 0.0, // Listener velocity (assumed to be 0)
            source.velocity.1 - 0.0,
            source.velocity.2 - 0.0,
        );

        // Calculate distance from listener
        let distance = self.calculate_distance(source.position, self.listener_position);

        if distance <= 0.0 {
            return 1.0;
        }

        // Calculate direction from listener to source
        let direction = (
            (source.position.0 - self.listener_position.0) / distance,
            (source.position.1 - self.listener_position.1) / distance,
            (source.position.2 - self.listener_position.2) / distance,
        );

        // Calculate relative velocity in direction of listener
        let relative_speed = relative_velocity.0 * direction.0 +
                            relative_velocity.1 * direction.1 +
                            relative_velocity.2 * direction.2;

        // Calculate doppler shift
        let doppler_shift = (self.settings.speed_of_sound + relative_speed * self.settings.doppler_factor) / 
                           (self.settings.speed_of_sound - relative_speed * self.settings.doppler_factor);

        doppler_shift.max(0.1).min(10.0)
    }

    /// Emit audio event
    fn emit_event(&self, event: AudioEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for SpatialAudioSettings {
    fn default() -> Self {
        Self {
            doppler_factor: 1.0,
            speed_of_sound: 343.0, // m/s
            distance_model: DistanceModel::InverseClamped,
            rolloff_factor: 1.0,
            reference_distance: 1.0,
            max_distance: 100.0,
            enable_doppler: true,
            enable_distance_attenuation: true,
            enable_cone_attenuation: true,
            enable_occlusion: false,
            occlusion_factor: 0.5,
            enable_obstruction: false,
            obstruction_factor: 0.3,
        }
    }
}

impl Default for SpatialAudioManager {
    fn default() -> Self {
        Self::new()
    }
}
