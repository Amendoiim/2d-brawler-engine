//! Dynamic Music Manager
//! 
//! This module provides the main dynamic music management system that coordinates
//! all music analysis, stems, transitions, and states.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    DynamicMusicConfig, MusicResult, MusicError, MusicEvent, MusicQuality,
    MusicAnalyzer, MusicStems, MusicTransitions, MusicStates, MusicEffects, MusicSequencer
};

/// Dynamic music manager
pub struct DynamicMusicManager {
    /// Configuration
    pub config: DynamicMusicConfig,
    /// Music analyzer
    pub analyzer: MusicAnalyzer,
    /// Music stems
    pub stems: MusicStems,
    /// Music transitions
    pub transitions: MusicTransitions,
    /// Music states
    pub states: MusicStates,
    /// Music effects
    pub effects: MusicEffects,
    /// Music sequencer
    pub sequencer: MusicSequencer,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&MusicEvent) + Send + Sync>>,
}

impl DynamicMusicManager {
    /// Create new dynamic music manager
    pub fn new(config: DynamicMusicConfig) -> Self {
        Self {
            config,
            analyzer: MusicAnalyzer::default(),
            stems: MusicStems::default(),
            transitions: MusicTransitions::default(),
            states: MusicStates::default(),
            effects: MusicEffects::default(),
            sequencer: MusicSequencer::default(),
            event_handlers: Vec::new(),
        }
    }

    /// Update dynamic music manager
    pub fn update(&mut self, delta_time: f32, audio_data: &[f32]) -> MusicResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        // Update music analyzer
        if self.config.enable_real_time_analysis {
            self.analyzer.update(delta_time, audio_data)?;
        }

        // Update music stems
        self.stems.update(delta_time)?;

        // Update music transitions
        self.transitions.update(delta_time)?;

        // Update music states
        if self.config.enable_dynamic_states {
            self.states.update(delta_time, &self.analyzer.analysis_data)?;
        }

        // Update music effects
        if self.config.enable_music_effects {
            self.effects.update(delta_time)?;
        }

        // Update music sequencer
        self.sequencer.update(delta_time)?;

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

impl Default for DynamicMusicManager {
    fn default() -> Self {
        Self::new(DynamicMusicConfig::default())
    }
}
