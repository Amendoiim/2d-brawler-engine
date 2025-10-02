//! Music Transition Manager
//! 
//! This module provides the main music transition management system that
//! coordinates all transition types, curves, effects, and synchronization.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    MusicTransitionConfig, MusicTransitionResult, MusicTransitionError, MusicTransitionEvent,
    TransitionType, TransitionCurve, TransitionEffect, TransitionPreset
};

/// Music transition manager
pub struct MusicTransitionManager {
    /// Configuration
    pub config: MusicTransitionConfig,
    /// Active transitions
    pub active_transitions: HashMap<String, ActiveTransition>,
    /// Transition presets
    pub transition_presets: HashMap<String, Vec<TransitionPreset>>,
    /// Transition history
    pub transition_history: Vec<TransitionHistoryEntry>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&MusicTransitionEvent) + Send + Sync>>,
}

/// Active transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTransition {
    /// Transition ID
    pub id: String,
    /// From track (None for fade in)
    pub from_track: Option<String>,
    /// To track
    pub to_track: String,
    /// Transition type
    pub transition_type: TransitionType,
    /// Transition start time
    pub start_time: f32,
    /// Transition duration
    pub duration: f32,
    /// Transition progress (0.0 to 1.0)
    pub progress: f32,
    /// Transition enabled
    pub enabled: bool,
    /// Transition paused
    pub paused: bool,
    /// Transition parameters
    pub parameters: HashMap<String, f32>,
}

/// Transition history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionHistoryEntry {
    /// Transition ID
    pub transition_id: String,
    /// From track
    pub from_track: Option<String>,
    /// To track
    pub to_track: String,
    /// Transition type
    pub transition_type: String,
    /// Transition start time
    pub start_time: f32,
    /// Transition end time
    pub end_time: f32,
    /// Transition duration
    pub duration: f32,
    /// Transition success
    pub success: bool,
    /// Transition parameters
    pub parameters: HashMap<String, f32>,
}

impl MusicTransitionManager {
    /// Create new music transition manager
    pub fn new(config: MusicTransitionConfig) -> Self {
        Self {
            config,
            active_transitions: HashMap::new(),
            transition_presets: HashMap::new(),
            transition_history: Vec::new(),
            event_handlers: Vec::new(),
        }
    }

    /// Update music transition manager
    pub fn update(&mut self, delta_time: f32) -> MusicTransitionResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        // Update active transitions
        self.update_active_transitions(delta_time)?;

        // Analyze transitions
        if self.config.enable_transition_analysis {
            self.analyze_transitions()?;
        }

        // Optimize transitions
        if self.config.enable_transition_optimization {
            self.optimize_transitions()?;
        }

        Ok(())
    }

    /// Start transition
    pub fn start_transition(
        &mut self,
        id: &str,
        from_track: Option<&str>,
        to_track: &str,
        transition_type: TransitionType,
    ) -> MusicTransitionResult<()> {
        if self.active_transitions.len() >= self.config.max_simultaneous_transitions {
            return Err(MusicTransitionError::ProcessingError(
                "Maximum simultaneous transitions exceeded".to_string(),
            ));
        }

        // Validate transition duration
        let duration = transition_type.get_duration();
        if duration < self.config.min_transition_duration || duration > self.config.max_transition_duration {
            return Err(MusicTransitionError::InvalidTransitionDuration(duration));
        }

        // Create active transition
        let active_transition = ActiveTransition {
            id: id.to_string(),
            from_track: from_track.map(|s| s.to_string()),
            to_track: to_track.to_string(),
            transition_type,
            start_time: 0.0, // Will be set by caller
            duration,
            progress: 0.0,
            enabled: true,
            paused: false,
            parameters: HashMap::new(),
        };

        self.active_transitions.insert(id.to_string(), active_transition);

        // Emit transition started event
        self.emit_event(MusicTransitionEvent::TransitionStarted {
            transition_id: id.to_string(),
            from_track: from_track.map(|s| s.to_string()),
            to_track: to_track.to_string(),
            transition_type: transition_type.clone(),
        });

        Ok(())
    }

    /// Stop transition
    pub fn stop_transition(&mut self, id: &str) -> MusicTransitionResult<()> {
        if let Some(transition) = self.active_transitions.remove(id) {
            // Create history entry
            let history_entry = TransitionHistoryEntry {
                transition_id: id.to_string(),
                from_track: transition.from_track.clone(),
                to_track: transition.to_track.clone(),
                transition_type: transition.transition_type.get_type_name().to_string(),
                start_time: transition.start_time,
                end_time: transition.start_time + transition.duration,
                duration: transition.duration,
                success: false,
                parameters: transition.parameters,
            };

            self.transition_history.push(history_entry);

            // Emit transition cancelled event
            self.emit_event(MusicTransitionEvent::TransitionCancelled {
                transition_id: id.to_string(),
                reason: "Stopped by user".to_string(),
            });
        }

        Ok(())
    }

    /// Pause transition
    pub fn pause_transition(&mut self, id: &str) -> MusicTransitionResult<()> {
        if let Some(transition) = self.active_transitions.get_mut(id) {
            transition.paused = true;
        }
        Ok(())
    }

    /// Resume transition
    pub fn resume_transition(&mut self, id: &str) -> MusicTransitionResult<()> {
        if let Some(transition) = self.active_transitions.get_mut(id) {
            transition.paused = false;
        }
        Ok(())
    }

    /// Get transition
    pub fn get_transition(&self, id: &str) -> Option<&ActiveTransition> {
        self.active_transitions.get(id)
    }

    /// Get transition mutably
    pub fn get_transition_mut(&mut self, id: &str) -> Option<&mut ActiveTransition> {
        self.active_transitions.get_mut(id)
    }

    /// Get all active transitions
    pub fn get_all_active_transitions(&self) -> Vec<&ActiveTransition> {
        self.active_transitions.values().collect()
    }

    /// Get transitions for track
    pub fn get_transitions_for_track(&self, track: &str) -> Vec<&ActiveTransition> {
        self.active_transitions
            .values()
            .filter(|t| t.from_track.as_ref().map_or(false, |f| f == track) || t.to_track == track)
            .collect()
    }

    /// Add transition preset
    pub fn add_transition_preset(&mut self, name: &str, presets: Vec<TransitionPreset>) -> MusicTransitionResult<()> {
        self.transition_presets.insert(name.to_string(), presets);
        Ok(())
    }

    /// Remove transition preset
    pub fn remove_transition_preset(&mut self, name: &str) -> MusicTransitionResult<()> {
        if !self.transition_presets.contains_key(name) {
            return Err(MusicTransitionError::PresetNotFound(name.to_string()));
        }

        self.transition_presets.remove(name);
        Ok(())
    }

    /// Get transition preset
    pub fn get_transition_preset(&self, name: &str) -> Option<&Vec<TransitionPreset>> {
        self.transition_presets.get(name)
    }

    /// Apply transition preset
    pub fn apply_transition_preset(
        &mut self,
        preset_name: &str,
        from_track: Option<&str>,
        to_track: &str,
    ) -> MusicTransitionResult<()> {
        let presets = self.transition_presets.get(preset_name)
            .ok_or_else(|| MusicTransitionError::PresetNotFound(preset_name.to_string()))?;

        for (i, preset) in presets.iter().enumerate() {
            let transition_id = format!("{}_{}", preset_name, i);
            self.start_transition(&transition_id, from_track, to_track, preset.transition_type.clone())?;
        }

        // Emit preset loaded event
        self.emit_event(MusicTransitionEvent::TransitionPresetLoaded {
            preset_name: preset_name.to_string(),
            transitions: presets.clone(),
        });

        Ok(())
    }

    /// Create fade in transition
    pub fn create_fade_in(
        &mut self,
        id: &str,
        to_track: &str,
        duration: f32,
        curve: TransitionCurve,
        effects: Vec<TransitionEffect>,
    ) -> MusicTransitionResult<()> {
        if !self.config.enable_fade_in {
            return Err(MusicTransitionError::InvalidConfig("Fade in transitions disabled".to_string()));
        }

        let transition_type = TransitionType::FadeIn {
            duration,
            curve,
            effects: effects.iter().map(|e| e.get_id().to_string()).collect(),
        };

        self.start_transition(id, None, to_track, transition_type)?;
        Ok(())
    }

    /// Create fade out transition
    pub fn create_fade_out(
        &mut self,
        id: &str,
        from_track: &str,
        duration: f32,
        curve: TransitionCurve,
        effects: Vec<TransitionEffect>,
    ) -> MusicTransitionResult<()> {
        if !self.config.enable_fade_out {
            return Err(MusicTransitionError::InvalidConfig("Fade out transitions disabled".to_string()));
        }

        let transition_type = TransitionType::FadeOut {
            duration,
            curve,
            effects: effects.iter().map(|e| e.get_id().to_string()).collect(),
        };

        self.start_transition(id, Some(from_track), "", transition_type)?;
        Ok(())
    }

    /// Create crossfade transition
    pub fn create_crossfade(
        &mut self,
        id: &str,
        from_track: &str,
        to_track: &str,
        duration: f32,
        curve: TransitionCurve,
        balance: f32,
        effects: Vec<TransitionEffect>,
    ) -> MusicTransitionResult<()> {
        if !self.config.enable_crossfade {
            return Err(MusicTransitionError::InvalidConfig("Crossfade transitions disabled".to_string()));
        }

        let transition_type = TransitionType::Crossfade {
            duration,
            curve,
            effects: effects.iter().map(|e| e.get_id().to_string()).collect(),
            balance: balance.max(0.0).min(1.0),
        };

        self.start_transition(id, Some(from_track), to_track, transition_type)?;
        Ok(())
    }

    /// Create beat-synchronized transition
    pub fn create_beat_sync_transition(
        &mut self,
        id: &str,
        from_track: Option<&str>,
        to_track: &str,
        duration_beats: f32,
        curve: TransitionCurve,
        accuracy: f32,
        effects: Vec<TransitionEffect>,
    ) -> MusicTransitionResult<()> {
        if !self.config.enable_beat_sync {
            return Err(MusicTransitionError::InvalidConfig("Beat-synchronized transitions disabled".to_string()));
        }

        let transition_type = TransitionType::BeatSync {
            duration_beats,
            curve,
            effects: effects.iter().map(|e| e.get_id().to_string()).collect(),
            accuracy: accuracy.max(0.0).min(1.0),
        };

        self.start_transition(id, from_track, to_track, transition_type)?;
        Ok(())
    }

    /// Create phrase-synchronized transition
    pub fn create_phrase_sync_transition(
        &mut self,
        id: &str,
        from_track: Option<&str>,
        to_track: &str,
        duration_phrases: f32,
        curve: TransitionCurve,
        accuracy: f32,
        effects: Vec<TransitionEffect>,
    ) -> MusicTransitionResult<()> {
        if !self.config.enable_phrase_sync {
            return Err(MusicTransitionError::InvalidConfig("Phrase-synchronized transitions disabled".to_string()));
        }

        let transition_type = TransitionType::PhraseSync {
            duration_phrases,
            curve,
            effects: effects.iter().map(|e| e.get_id().to_string()).collect(),
            accuracy: accuracy.max(0.0).min(1.0),
        };

        self.start_transition(id, from_track, to_track, transition_type)?;
        Ok(())
    }

    /// Create measure-synchronized transition
    pub fn create_measure_sync_transition(
        &mut self,
        id: &str,
        from_track: Option<&str>,
        to_track: &str,
        duration_measures: f32,
        curve: TransitionCurve,
        accuracy: f32,
        effects: Vec<TransitionEffect>,
    ) -> MusicTransitionResult<()> {
        if !self.config.enable_measure_sync {
            return Err(MusicTransitionError::InvalidConfig("Measure-synchronized transitions disabled".to_string()));
        }

        let transition_type = TransitionType::MeasureSync {
            duration_measures,
            curve,
            effects: effects.iter().map(|e| e.get_id().to_string()).collect(),
            accuracy: accuracy.max(0.0).min(1.0),
        };

        self.start_transition(id, from_track, to_track, transition_type)?;
        Ok(())
    }

    /// Process audio through transitions
    pub fn process_audio(&self, audio_data: &mut [f32]) -> MusicTransitionResult<()> {
        for transition in self.active_transitions.values() {
            if transition.enabled && !transition.paused {
                self.process_transition(transition, audio_data)?;
            }
        }
        Ok(())
    }

    /// Get transition history
    pub fn get_transition_history(&self) -> &Vec<TransitionHistoryEntry> {
        &self.transition_history
    }

    /// Clear transition history
    pub fn clear_transition_history(&mut self) {
        self.transition_history.clear();
    }

    /// Update active transitions
    fn update_active_transitions(&mut self, delta_time: f32) -> MusicTransitionResult<()> {
        let mut completed_transitions = Vec::new();

        for (id, transition) in self.active_transitions.iter_mut() {
            if transition.enabled && !transition.paused {
                // Update transition progress
                transition.progress += delta_time / transition.duration;
                transition.progress = transition.progress.min(1.0);

                // Emit progress updated event
                self.emit_event(MusicTransitionEvent::TransitionProgressUpdated {
                    transition_id: id.clone(),
                    progress: transition.progress,
                    time_remaining: transition.duration * (1.0 - transition.progress),
                });

                // Check if transition is completed
                if transition.progress >= 1.0 {
                    completed_transitions.push(id.clone());
                }
            }
        }

        // Handle completed transitions
        for id in completed_transitions {
            self.handle_completed_transition(&id)?;
        }

        Ok(())
    }

    /// Handle completed transition
    fn handle_completed_transition(&mut self, id: &str) -> MusicTransitionResult<()> {
        if let Some(transition) = self.active_transitions.remove(id) {
            // Create history entry
            let history_entry = TransitionHistoryEntry {
                transition_id: id.to_string(),
                from_track: transition.from_track.clone(),
                to_track: transition.to_track.clone(),
                transition_type: transition.transition_type.get_type_name().to_string(),
                start_time: transition.start_time,
                end_time: transition.start_time + transition.duration,
                duration: transition.duration,
                success: true,
                parameters: transition.parameters,
            };

            self.transition_history.push(history_entry);

            // Emit transition completed event
            self.emit_event(MusicTransitionEvent::TransitionCompleted {
                transition_id: id.to_string(),
                success: true,
                duration: transition.duration,
            });
        }

        Ok(())
    }

    /// Analyze transitions
    fn analyze_transitions(&mut self) -> MusicTransitionResult<()> {
        // Analyze transition usage patterns
        // In a real implementation, this would analyze transition effectiveness
        Ok(())
    }

    /// Optimize transitions
    fn optimize_transitions(&mut self) -> MusicTransitionResult<()> {
        // Optimize transition parameters
        // In a real implementation, this would optimize transition settings
        Ok(())
    }

    /// Process transition
    fn process_transition(&self, transition: &ActiveTransition, audio_data: &mut [f32]) -> MusicTransitionResult<()> {
        // In a real implementation, this would process the audio through the transition
        // For now, we'll just simulate the processing based on transition type and progress
        
        match &transition.transition_type {
            TransitionType::FadeIn { curve, .. } => {
                self.process_fade_in(transition, curve, audio_data)?;
            },
            TransitionType::FadeOut { curve, .. } => {
                self.process_fade_out(transition, curve, audio_data)?;
            },
            TransitionType::Crossfade { curve, balance, .. } => {
                self.process_crossfade(transition, curve, *balance, audio_data)?;
            },
            TransitionType::BeatSync { curve, .. } => {
                self.process_beat_sync(transition, curve, audio_data)?;
            },
            TransitionType::PhraseSync { curve, .. } => {
                self.process_phrase_sync(transition, curve, audio_data)?;
            },
            TransitionType::MeasureSync { curve, .. } => {
                self.process_measure_sync(transition, curve, audio_data)?;
            },
            TransitionType::Instant { .. } => {
                self.process_instant(transition, audio_data)?;
            },
            _ => {
                // Process other transition types
                self.process_generic_transition(transition, audio_data)?;
            },
        }
        
        Ok(())
    }

    /// Process fade in transition
    fn process_fade_in(&self, transition: &ActiveTransition, curve: &TransitionCurve, audio_data: &mut [f32]) -> MusicTransitionResult<()> {
        let fade_value = curve.evaluate(transition.progress);
        for sample in audio_data.iter_mut() {
            *sample *= fade_value;
        }
        Ok(())
    }

    /// Process fade out transition
    fn process_fade_out(&self, transition: &ActiveTransition, curve: &TransitionCurve, audio_data: &mut [f32]) -> MusicTransitionResult<()> {
        let fade_value = 1.0 - curve.evaluate(transition.progress);
        for sample in audio_data.iter_mut() {
            *sample *= fade_value;
        }
        Ok(())
    }

    /// Process crossfade transition
    fn process_crossfade(&self, transition: &ActiveTransition, curve: &TransitionCurve, balance: f32, audio_data: &mut [f32]) -> MusicTransitionResult<()> {
        let crossfade_value = curve.evaluate(transition.progress);
        let from_volume = (1.0 - crossfade_value) * (1.0 - balance);
        let to_volume = crossfade_value * balance;
        
        for sample in audio_data.iter_mut() {
            *sample = *sample * from_volume + *sample * to_volume;
        }
        Ok(())
    }

    /// Process beat-synchronized transition
    fn process_beat_sync(&self, transition: &ActiveTransition, curve: &TransitionCurve, audio_data: &mut [f32]) -> MusicTransitionResult<()> {
        // Beat-synchronized processing
        // In a real implementation, this would sync with the beat
        self.process_crossfade(transition, curve, 0.5, audio_data)?;
        Ok(())
    }

    /// Process phrase-synchronized transition
    fn process_phrase_sync(&self, transition: &ActiveTransition, curve: &TransitionCurve, audio_data: &mut [f32]) -> MusicTransitionResult<()> {
        // Phrase-synchronized processing
        // In a real implementation, this would sync with the phrase
        self.process_crossfade(transition, curve, 0.5, audio_data)?;
        Ok(())
    }

    /// Process measure-synchronized transition
    fn process_measure_sync(&self, transition: &ActiveTransition, curve: &TransitionCurve, audio_data: &mut [f32]) -> MusicTransitionResult<()> {
        // Measure-synchronized processing
        // In a real implementation, this would sync with the measure
        self.process_crossfade(transition, curve, 0.5, audio_data)?;
        Ok(())
    }

    /// Process instant transition
    fn process_instant(&self, _transition: &ActiveTransition, _audio_data: &mut [f32]) -> MusicTransitionResult<()> {
        // Instant transition - no processing needed
        Ok(())
    }

    /// Process generic transition
    fn process_generic_transition(&self, transition: &ActiveTransition, audio_data: &mut [f32]) -> MusicTransitionResult<()> {
        // Generic transition processing
        // In a real implementation, this would handle all other transition types
        let fade_value = transition.progress;
        for sample in audio_data.iter_mut() {
            *sample *= fade_value;
        }
        Ok(())
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&MusicTransitionEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Emit music transition event
    fn emit_event(&self, event: MusicTransitionEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for MusicTransitionManager {
    fn default() -> Self {
        Self::new(MusicTransitionConfig::default())
    }
}
