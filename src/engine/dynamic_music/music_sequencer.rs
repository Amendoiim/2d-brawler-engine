//! Music Sequencer
//! 
//! This module provides music sequencing capabilities including pattern-based
//! composition, real-time sequencing, and adaptive music generation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{MusicResult, MusicError, MusicEvent, MusicStemType, MusicStateType};

/// Music sequencer
pub struct MusicSequencer {
    /// Sequencer configuration
    pub config: SequencerConfig,
    /// Active sequences
    pub active_sequences: HashMap<String, MusicSequence>,
    /// Sequence patterns
    pub sequence_patterns: HashMap<String, SequencePattern>,
    /// Sequence tracks
    pub sequence_tracks: HashMap<String, SequenceTrack>,
    /// Sequence events
    pub sequence_events: Vec<SequenceEvent>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&MusicEvent) + Send + Sync>>,
}

/// Sequencer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencerConfig {
    /// Enable sequencer
    pub enable_sequencer: bool,
    /// Default tempo (BPM)
    pub default_tempo: f32,
    /// Default time signature
    pub default_time_signature: (u32, u32),
    /// Default key
    pub default_key: String,
    /// Default scale
    pub default_scale: String,
    /// Maximum sequences
    pub max_sequences: usize,
    /// Maximum patterns per sequence
    pub max_patterns_per_sequence: usize,
    /// Maximum tracks per pattern
    pub max_tracks_per_pattern: usize,
    /// Enable real-time sequencing
    pub enable_real_time_sequencing: bool,
    /// Enable adaptive sequencing
    pub enable_adaptive_sequencing: bool,
    /// Enable pattern variation
    pub enable_pattern_variation: bool,
    /// Enable automatic composition
    pub enable_automatic_composition: bool,
}

/// Music sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicSequence {
    /// Sequence ID
    pub id: String,
    /// Sequence name
    pub name: String,
    /// Sequence description
    pub description: String,
    /// Sequence tempo (BPM)
    pub tempo: f32,
    /// Sequence time signature
    pub time_signature: (u32, u32),
    /// Sequence key
    pub key: String,
    /// Sequence scale
    pub scale: String,
    /// Sequence patterns
    pub patterns: Vec<String>,
    /// Current pattern
    pub current_pattern: Option<String>,
    /// Sequence position (beats)
    pub position: f32,
    /// Sequence length (beats)
    pub length: f32,
    /// Sequence loop
    pub loop_sequence: bool,
    /// Sequence enabled
    pub enabled: bool,
    /// Sequence playing
    pub playing: bool,
    /// Sequence paused
    pub paused: bool,
    /// Sequence mute
    pub muted: bool,
    /// Sequence solo
    pub solo: bool,
    /// Sequence volume (0.0 to 1.0)
    pub volume: f32,
    /// Sequence pan (-1.0 to 1.0)
    pub pan: f32,
    /// Sequence priority (0-100)
    pub priority: u8,
    /// Sequence layer (0-10)
    pub layer: u8,
}

/// Sequence pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencePattern {
    /// Pattern ID
    pub id: String,
    /// Pattern name
    pub name: String,
    /// Pattern description
    pub description: String,
    /// Pattern length (beats)
    pub length: f32,
    /// Pattern tracks
    pub tracks: Vec<String>,
    /// Pattern tempo (BPM)
    pub tempo: f32,
    /// Pattern time signature
    pub time_signature: (u32, u32),
    /// Pattern key
    pub key: String,
    /// Pattern scale
    pub scale: String,
    /// Pattern loop
    pub loop_pattern: bool,
    /// Pattern enabled
    pub enabled: bool,
    /// Pattern mute
    pub muted: bool,
    /// Pattern solo
    pub solo: bool,
    /// Pattern volume (0.0 to 1.0)
    pub volume: f32,
    /// Pattern pan (-1.0 to 1.0)
    pub pan: f32,
    /// Pattern priority (0-100)
    pub priority: u8,
    /// Pattern layer (0-10)
    pub layer: u8,
}

/// Sequence track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceTrack {
    /// Track ID
    pub id: String,
    /// Track name
    pub name: String,
    /// Track description
    pub description: String,
    /// Track stem type
    pub stem_type: MusicStemType,
    /// Track events
    pub events: Vec<TrackEvent>,
    /// Track length (beats)
    pub length: f32,
    /// Track loop
    pub loop_track: bool,
    /// Track enabled
    pub enabled: bool,
    /// Track mute
    pub muted: bool,
    /// Track solo
    pub solo: bool,
    /// Track volume (0.0 to 1.0)
    pub volume: f32,
    /// Track pan (-1.0 to 1.0)
    pub pan: f32,
    /// Track priority (0-100)
    pub priority: u8,
    /// Track layer (0-10)
    pub layer: u8,
}

/// Track event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackEvent {
    /// Event ID
    pub id: String,
    /// Event type
    pub event_type: TrackEventType,
    /// Event start time (beats)
    pub start_time: f32,
    /// Event end time (beats)
    pub end_time: f32,
    /// Event duration (beats)
    pub duration: f32,
    /// Event velocity (0.0 to 1.0)
    pub velocity: f32,
    /// Event pitch (MIDI note number)
    pub pitch: u8,
    /// Event parameters
    pub parameters: HashMap<String, f32>,
    /// Event enabled
    pub enabled: bool,
    /// Event muted
    pub muted: bool,
}

/// Track event type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrackEventType {
    /// Note on
    NoteOn,
    /// Note off
    NoteOff,
    /// Note hold
    NoteHold,
    /// Control change
    ControlChange,
    /// Program change
    ProgramChange,
    /// Pitch bend
    PitchBend,
    /// Aftertouch
    Aftertouch,
    /// System exclusive
    SystemExclusive,
    /// Custom event
    Custom(String),
}

/// Sequence event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceEvent {
    /// Event ID
    pub id: String,
    /// Event type
    pub event_type: SequenceEventType,
    /// Event time (beats)
    pub time: f32,
    /// Event parameters
    pub parameters: HashMap<String, f32>,
    /// Event enabled
    pub enabled: bool,
}

/// Sequence event type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SequenceEventType {
    /// Pattern change
    PatternChange,
    /// Tempo change
    TempoChange,
    /// Time signature change
    TimeSignatureChange,
    /// Key change
    KeyChange,
    /// Scale change
    ScaleChange,
    /// Volume change
    VolumeChange,
    /// Pan change
    PanChange,
    /// Mute change
    MuteChange,
    /// Solo change
    SoloChange,
    /// Loop change
    LoopChange,
    /// Custom event
    Custom(String),
}

impl MusicSequencer {
    /// Create new music sequencer
    pub fn new(config: SequencerConfig) -> Self {
        Self {
            config,
            active_sequences: HashMap::new(),
            sequence_patterns: HashMap::new(),
            sequence_tracks: HashMap::new(),
            sequence_events: Vec::new(),
            event_handlers: Vec::new(),
        }
    }

    /// Update music sequencer
    pub fn update(&mut self, delta_time: f32) -> MusicResult<()> {
        if !self.config.enable_sequencer {
            return Ok(());
        }

        // Update active sequences
        self.update_active_sequences(delta_time)?;

        // Process sequence events
        self.process_sequence_events(delta_time)?;

        // Update adaptive sequencing
        if self.config.enable_adaptive_sequencing {
            self.update_adaptive_sequencing(delta_time)?;
        }

        // Update automatic composition
        if self.config.enable_automatic_composition {
            self.update_automatic_composition(delta_time)?;
        }

        Ok(())
    }

    /// Add sequence
    pub fn add_sequence(&mut self, sequence: MusicSequence) -> MusicResult<()> {
        if self.active_sequences.len() >= self.config.max_sequences {
            return Err(MusicError::InvalidConfig("Maximum sequences exceeded".to_string()));
        }

        let id = sequence.id.clone();
        self.active_sequences.insert(id, sequence);
        Ok(())
    }

    /// Remove sequence
    pub fn remove_sequence(&mut self, id: &str) -> MusicResult<()> {
        if !self.active_sequences.contains_key(id) {
            return Err(MusicError::Unknown(format!("Sequence '{}' not found", id)));
        }

        self.active_sequences.remove(id);
        Ok(())
    }

    /// Get sequence
    pub fn get_sequence(&self, id: &str) -> Option<&MusicSequence> {
        self.active_sequences.get(id)
    }

    /// Get sequence mutably
    pub fn get_sequence_mut(&mut self, id: &str) -> Option<&mut MusicSequence> {
        self.active_sequences.get_mut(id)
    }

    /// Play sequence
    pub fn play_sequence(&mut self, id: &str) -> MusicResult<()> {
        if let Some(sequence) = self.active_sequences.get_mut(id) {
            sequence.playing = true;
            sequence.paused = false;
        }
        Ok(())
    }

    /// Stop sequence
    pub fn stop_sequence(&mut self, id: &str) -> MusicResult<()> {
        if let Some(sequence) = self.active_sequences.get_mut(id) {
            sequence.playing = false;
            sequence.paused = false;
            sequence.position = 0.0;
        }
        Ok(())
    }

    /// Pause sequence
    pub fn pause_sequence(&mut self, id: &str) -> MusicResult<()> {
        if let Some(sequence) = self.active_sequences.get_mut(id) {
            sequence.paused = true;
        }
        Ok(())
    }

    /// Resume sequence
    pub fn resume_sequence(&mut self, id: &str) -> MusicResult<()> {
        if let Some(sequence) = self.active_sequences.get_mut(id) {
            sequence.paused = false;
        }
        Ok(())
    }

    /// Set sequence tempo
    pub fn set_sequence_tempo(&mut self, id: &str, tempo: f32) -> MusicResult<()> {
        if let Some(sequence) = self.active_sequences.get_mut(id) {
            sequence.tempo = tempo.max(60.0).min(200.0);
        }
        Ok(())
    }

    /// Set sequence time signature
    pub fn set_sequence_time_signature(&mut self, id: &str, time_signature: (u32, u32)) -> MusicResult<()> {
        if let Some(sequence) = self.active_sequences.get_mut(id) {
            sequence.time_signature = time_signature;
        }
        Ok(())
    }

    /// Set sequence key
    pub fn set_sequence_key(&mut self, id: &str, key: &str) -> MusicResult<()> {
        if let Some(sequence) = self.active_sequences.get_mut(id) {
            sequence.key = key.to_string();
        }
        Ok(())
    }

    /// Set sequence scale
    pub fn set_sequence_scale(&mut self, id: &str, scale: &str) -> MusicResult<()> {
        if let Some(sequence) = self.active_sequences.get_mut(id) {
            sequence.scale = scale.to_string();
        }
        Ok(())
    }

    /// Add pattern to sequence
    pub fn add_pattern_to_sequence(&mut self, sequence_id: &str, pattern_id: &str) -> MusicResult<()> {
        if let Some(sequence) = self.active_sequences.get_mut(sequence_id) {
            if sequence.patterns.len() >= self.config.max_patterns_per_sequence {
                return Err(MusicError::InvalidConfig("Maximum patterns per sequence exceeded".to_string()));
            }

            sequence.patterns.push(pattern_id.to_string());
        }
        Ok(())
    }

    /// Remove pattern from sequence
    pub fn remove_pattern_from_sequence(&mut self, sequence_id: &str, pattern_id: &str) -> MusicResult<()> {
        if let Some(sequence) = self.active_sequences.get_mut(sequence_id) {
            sequence.patterns.retain(|id| id != pattern_id);
        }
        Ok(())
    }

    /// Set current pattern
    pub fn set_current_pattern(&mut self, sequence_id: &str, pattern_id: &str) -> MusicResult<()> {
        if let Some(sequence) = self.active_sequences.get_mut(sequence_id) {
            if sequence.patterns.contains(&pattern_id.to_string()) {
                sequence.current_pattern = Some(pattern_id.to_string());
            } else {
                return Err(MusicError::InvalidConfig("Pattern not found in sequence".to_string()));
            }
        }
        Ok(())
    }

    /// Add pattern
    pub fn add_pattern(&mut self, pattern: SequencePattern) -> MusicResult<()> {
        let id = pattern.id.clone();
        self.sequence_patterns.insert(id, pattern);
        Ok(())
    }

    /// Remove pattern
    pub fn remove_pattern(&mut self, id: &str) -> MusicResult<()> {
        if !self.sequence_patterns.contains_key(id) {
            return Err(MusicError::Unknown(format!("Pattern '{}' not found", id)));
        }

        self.sequence_patterns.remove(id);
        Ok(())
    }

    /// Get pattern
    pub fn get_pattern(&self, id: &str) -> Option<&SequencePattern> {
        self.sequence_patterns.get(id)
    }

    /// Get pattern mutably
    pub fn get_pattern_mut(&mut self, id: &str) -> Option<&mut SequencePattern> {
        self.sequence_patterns.get_mut(id)
    }

    /// Add track to pattern
    pub fn add_track_to_pattern(&mut self, pattern_id: &str, track_id: &str) -> MusicResult<()> {
        if let Some(pattern) = self.sequence_patterns.get_mut(pattern_id) {
            if pattern.tracks.len() >= self.config.max_tracks_per_pattern {
                return Err(MusicError::InvalidConfig("Maximum tracks per pattern exceeded".to_string()));
            }

            pattern.tracks.push(track_id.to_string());
        }
        Ok(())
    }

    /// Remove track from pattern
    pub fn remove_track_from_pattern(&mut self, pattern_id: &str, track_id: &str) -> MusicResult<()> {
        if let Some(pattern) = self.sequence_patterns.get_mut(pattern_id) {
            pattern.tracks.retain(|id| id != track_id);
        }
        Ok(())
    }

    /// Add track
    pub fn add_track(&mut self, track: SequenceTrack) -> MusicResult<()> {
        let id = track.id.clone();
        self.sequence_tracks.insert(id, track);
        Ok(())
    }

    /// Remove track
    pub fn remove_track(&mut self, id: &str) -> MusicResult<()> {
        if !self.sequence_tracks.contains_key(id) {
            return Err(MusicError::Unknown(format!("Track '{}' not found", id)));
        }

        self.sequence_tracks.remove(id);
        Ok(())
    }

    /// Get track
    pub fn get_track(&self, id: &str) -> Option<&SequenceTrack> {
        self.sequence_tracks.get(id)
    }

    /// Get track mutably
    pub fn get_track_mut(&mut self, id: &str) -> Option<&mut SequenceTrack> {
        self.sequence_tracks.get_mut(id)
    }

    /// Add track event
    pub fn add_track_event(&mut self, track_id: &str, event: TrackEvent) -> MusicResult<()> {
        if let Some(track) = self.sequence_tracks.get_mut(track_id) {
            track.events.push(event);
        }
        Ok(())
    }

    /// Remove track event
    pub fn remove_track_event(&mut self, track_id: &str, event_id: &str) -> MusicResult<()> {
        if let Some(track) = self.sequence_tracks.get_mut(track_id) {
            track.events.retain(|event| event.id != event_id);
        }
        Ok(())
    }

    /// Add sequence event
    pub fn add_sequence_event(&mut self, event: SequenceEvent) -> MusicResult<()> {
        self.sequence_events.push(event);
        Ok(())
    }

    /// Remove sequence event
    pub fn remove_sequence_event(&mut self, event_id: &str) -> MusicResult<()> {
        self.sequence_events.retain(|event| event.id != event_id);
        Ok(())
    }

    /// Update active sequences
    fn update_active_sequences(&mut self, delta_time: f32) -> MusicResult<()> {
        for sequence in self.active_sequences.values_mut() {
            if sequence.playing && !sequence.paused {
                // Update sequence position
                let beats_per_second = sequence.tempo / 60.0;
                sequence.position += delta_time * beats_per_second;

                // Check if sequence should loop
                if sequence.position >= sequence.length {
                    if sequence.loop_sequence {
                        sequence.position = 0.0;
                    } else {
                        sequence.playing = false;
                    }
                }

                // Update current pattern
                if let Some(pattern_id) = &sequence.current_pattern {
                    if let Some(pattern) = self.sequence_patterns.get_mut(pattern_id) {
                        // Update pattern position
                        let pattern_position = sequence.position % pattern.length;
                        
                        // Process pattern events
                        self.process_pattern_events(pattern, pattern_position)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Process sequence events
    fn process_sequence_events(&mut self, delta_time: f32) -> MusicResult<()> {
        // Process sequence events based on current time
        for event in &self.sequence_events {
            if event.enabled {
                // Process event based on type
                self.process_sequence_event(event)?;
            }
        }
        Ok(())
    }

    /// Process pattern events
    fn process_pattern_events(&self, pattern: &SequencePattern, position: f32) -> MusicResult<()> {
        // Process pattern tracks
        for track_id in &pattern.tracks {
            if let Some(track) = self.sequence_tracks.get(track_id) {
                // Process track events
                for event in &track.events {
                    if event.enabled && !event.muted {
                        // Check if event should be triggered
                        if position >= event.start_time && position < event.end_time {
                            self.process_track_event(event)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Process sequence event
    fn process_sequence_event(&self, event: &SequenceEvent) -> MusicResult<()> {
        // In a real implementation, this would process the sequence event
        // For now, we'll just simulate the processing
        Ok(())
    }

    /// Process track event
    fn process_track_event(&self, event: &TrackEvent) -> MusicResult<()> {
        // In a real implementation, this would process the track event
        // For now, we'll just simulate the processing
        Ok(())
    }

    /// Update adaptive sequencing
    fn update_adaptive_sequencing(&mut self, delta_time: f32) -> MusicResult<()> {
        // In a real implementation, this would update adaptive sequencing
        // For now, we'll just simulate the update
        Ok(())
    }

    /// Update automatic composition
    fn update_automatic_composition(&mut self, delta_time: f32) -> MusicResult<()> {
        // In a real implementation, this would update automatic composition
        // For now, we'll just simulate the update
        Ok(())
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&MusicEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Emit music event
    fn emit_event(&self, event: MusicEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for SequencerConfig {
    fn default() -> Self {
        Self {
            enable_sequencer: true,
            default_tempo: 120.0,
            default_time_signature: (4, 4),
            default_key: "C Major".to_string(),
            default_scale: "Major".to_string(),
            max_sequences: 10,
            max_patterns_per_sequence: 16,
            max_tracks_per_pattern: 8,
            enable_real_time_sequencing: true,
            enable_adaptive_sequencing: true,
            enable_pattern_variation: true,
            enable_automatic_composition: true,
        }
    }
}

impl Default for MusicSequencer {
    fn default() -> Self {
        Self::new(SequencerConfig::default())
    }
}
