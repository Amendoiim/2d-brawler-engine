//! SFX Pitch Manager
//! 
//! This module provides the main SFX pitch management system that coordinates
//! all pitch processing, variations, and effects.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    SFXPitchConfig, SFXPitchResult, SFXPitchError, SFXPitchEvent,
    PitchProcessor, PitchVariations, PitchEffects
};

/// SFX pitch manager
pub struct SFXPitchManager {
    /// Configuration
    pub config: SFXPitchConfig,
    /// Pitch processor
    pub processor: PitchProcessor,
    /// Pitch variations
    pub variations: PitchVariations,
    /// Pitch effects
    pub effects: PitchEffects,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&SFXPitchEvent) + Send + Sync>>,
}

impl SFXPitchManager {
    /// Create new SFX pitch manager
    pub fn new(config: SFXPitchConfig) -> Self {
        Self {
            config,
            processor: PitchProcessor::default(),
            variations: PitchVariations::default(),
            effects: PitchEffects::default(),
            event_handlers: Vec::new(),
        }
    }

    /// Update SFX pitch manager
    pub fn update(&mut self, delta_time: f32, audio_data: &[f32]) -> SFXPitchResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        // Update pitch processor
        if self.config.enable_real_time_processing {
            self.processor.update(delta_time, audio_data)?;
        }

        // Update pitch variations
        if self.config.enable_pitch_variations {
            self.variations.update(delta_time)?;
        }

        // Update pitch effects
        if self.config.enable_pitch_effects {
            self.effects.update(delta_time)?;
        }

        Ok(())
    }

    /// Process audio through SFX pitch system
    pub fn process_audio(&self, audio_data: &mut [f32]) -> SFXPitchResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        // Process through pitch processor
        self.processor.process_audio(audio_data)?;

        // Process through pitch effects
        self.effects.process_audio(audio_data)?;

        Ok(())
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&SFXPitchEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Emit SFX pitch event
    fn emit_event(&self, event: SFXPitchEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for SFXPitchManager {
    fn default() -> Self {
        Self::new(SFXPitchConfig::default())
    }
}
