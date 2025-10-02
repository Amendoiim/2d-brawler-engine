//! Pitch Effects
//! 
//! This module provides pitch effects including vibrato, tremolo, wah-wah,
//! chorus, flanger, and other pitch-based effects.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{SFXPitchResult, SFXPitchError, PitchEffectType, PitchEffect, SFXPitchEvent};

/// Pitch effects manager
pub struct PitchEffects {
    /// Effects configuration
    pub config: EffectsConfig,
    /// Available effects
    pub effects: HashMap<String, PitchEffect>,
    /// Effect presets
    pub effect_presets: HashMap<String, Vec<PitchEffect>>,
    /// Active effects
    pub active_effects: HashMap<String, ActiveEffect>,
    /// Effect chains
    pub effect_chains: HashMap<String, EffectChain>,
    /// Effect history
    pub effect_history: Vec<EffectHistoryEntry>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&SFXPitchEvent) + Send + Sync>>,
}

/// Effects configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectsConfig {
    /// Enable effects
    pub enable_effects: bool,
    /// Enable vibrato
    pub enable_vibrato: bool,
    /// Enable tremolo
    pub enable_tremolo: bool,
    /// Enable wah-wah
    pub enable_wah_wah: bool,
    /// Enable chorus
    pub enable_chorus: bool,
    /// Enable flanger
    pub enable_flanger: bool,
    /// Enable phaser
    pub enable_phaser: bool,
    /// Enable pitch shifter
    pub enable_pitch_shifter: bool,
    /// Enable harmonizer
    pub enable_harmonizer: bool,
    /// Enable pitch bender
    pub enable_pitch_bender: bool,
    /// Enable pitch corrector
    pub enable_pitch_corrector: bool,
    /// Enable pitch smoother
    pub enable_pitch_smoother: bool,
    /// Enable pitch analyzer
    pub enable_pitch_analyzer: bool,
    /// Enable pitch tracker
    pub enable_pitch_tracker: bool,
    /// Enable pitch detector
    pub enable_pitch_detector: bool,
    /// Enable pitch quantizer
    pub enable_pitch_quantizer: bool,
    /// Enable pitch scaler
    pub enable_pitch_scaler: bool,
    /// Enable pitch compressor
    pub enable_pitch_compressor: bool,
    /// Enable pitch expander
    pub enable_pitch_expander: bool,
    /// Enable pitch gate
    pub enable_pitch_gate: bool,
    /// Enable pitch limiter
    pub enable_pitch_limiter: bool,
    /// Enable pitch filter
    pub enable_pitch_filter: bool,
    /// Enable pitch EQ
    pub enable_pitch_eq: bool,
    /// Enable pitch reverb
    pub enable_pitch_reverb: bool,
    /// Enable pitch delay
    pub enable_pitch_delay: bool,
    /// Enable pitch echo
    pub enable_pitch_echo: bool,
    /// Enable pitch distortion
    pub enable_pitch_distortion: bool,
    /// Enable pitch overdrive
    pub enable_pitch_overdrive: bool,
    /// Enable pitch fuzz
    pub enable_pitch_fuzz: bool,
    /// Enable pitch bit crusher
    pub enable_pitch_bit_crusher: bool,
    /// Enable pitch sample rate reducer
    pub enable_pitch_sample_rate_reducer: bool,
    /// Enable pitch ring modulator
    pub enable_pitch_ring_modulator: bool,
    /// Enable pitch frequency shifter
    pub enable_pitch_frequency_shifter: bool,
    /// Enable pitch stereo widener
    pub enable_pitch_stereo_widener: bool,
    /// Enable pitch mono maker
    pub enable_pitch_mono_maker: bool,
    /// Enable pitch stereo imager
    pub enable_pitch_stereo_imager: bool,
    /// Enable pitch spatial processor
    pub enable_pitch_spatial_processor: bool,
    /// Enable pitch convolution
    pub enable_pitch_convolution: bool,
    /// Enable pitch impulse response
    pub enable_pitch_impulse_response: bool,
    /// Maximum effects per sound
    pub max_effects_per_sound: usize,
    /// Maximum effects per chain
    pub max_effects_per_chain: usize,
    /// Effect quality
    pub effect_quality: EffectQuality,
    /// Sample rate
    pub sample_rate: u32,
    /// Buffer size
    pub buffer_size: u32,
    /// Enable real-time processing
    pub enable_real_time_processing: bool,
    /// Enable effect automation
    pub enable_effect_automation: bool,
    /// Enable effect presets
    pub enable_effect_presets: bool,
}

/// Effect quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EffectQuality {
    /// Low quality - maximum performance
    Low,
    /// Medium quality - balanced performance
    Medium,
    /// High quality - good performance
    High,
    /// Ultra quality - maximum quality
    Ultra,
}

/// Active effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveEffect {
    /// Effect ID
    pub effect_id: String,
    /// Sound ID
    pub sound_id: String,
    /// Effect start time
    pub start_time: f32,
    /// Effect duration
    pub duration: f32,
    /// Effect progress (0.0 to 1.0)
    pub progress: f32,
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

/// Effect chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectChain {
    /// Chain ID
    pub id: String,
    /// Chain name
    pub name: String,
    /// Effect IDs in order
    pub effect_ids: Vec<String>,
    /// Chain enabled
    pub enabled: bool,
    /// Chain bypass
    pub bypass: bool,
    /// Chain parameters
    pub parameters: HashMap<String, f32>,
}

/// Effect history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectHistoryEntry {
    /// Effect ID
    pub effect_id: String,
    /// Sound ID
    pub sound_id: String,
    /// Effect type
    pub effect_type: PitchEffectType,
    /// Effect start time
    pub start_time: f32,
    /// Effect end time
    pub end_time: f32,
    /// Effect duration
    pub duration: f32,
    /// Effect success
    pub success: bool,
    /// Effect parameters
    pub parameters: HashMap<String, f32>,
}

impl PitchEffects {
    /// Create new pitch effects manager
    pub fn new(config: EffectsConfig) -> Self {
        Self {
            config,
            effects: HashMap::new(),
            effect_presets: HashMap::new(),
            active_effects: HashMap::new(),
            effect_chains: HashMap::new(),
            effect_history: Vec::new(),
            event_handlers: Vec::new(),
        }
    }

    /// Update pitch effects
    pub fn update(&mut self, delta_time: f32) -> SFXPitchResult<()> {
        if !self.config.enable_effects {
            return Ok(());
        }

        // Update active effects
        self.update_active_effects(delta_time)?;

        // Update effect chains
        self.update_effect_chains(delta_time)?;

        // Update effect automation
        self.update_effect_automation(delta_time)?;

        Ok(())
    }

    /// Add effect
    pub fn add_effect(&mut self, effect: PitchEffect) -> SFXPitchResult<()> {
        let id = effect.id.clone();
        self.effects.insert(id, effect);
        Ok(())
    }

    /// Remove effect
    pub fn remove_effect(&mut self, id: &str) -> SFXPitchResult<()> {
        if !self.effects.contains_key(id) {
            return Err(SFXPitchError::EffectNotFound(id.to_string()));
        }

        self.effects.remove(id);
        Ok(())
    }

    /// Get effect
    pub fn get_effect(&self, id: &str) -> Option<&PitchEffect> {
        self.effects.get(id)
    }

    /// Get effect mutably
    pub fn get_effect_mut(&mut self, id: &str) -> Option<&mut PitchEffect> {
        self.effects.get_mut(id)
    }

    /// Apply effect to sound
    pub fn apply_effect(&mut self, sound_id: &str, effect_id: &str) -> SFXPitchResult<()> {
        let effect = self.effects.get(effect_id)
            .ok_or_else(|| SFXPitchError::EffectNotFound(effect_id.to_string()))?;

        if !effect.enabled {
            return Err(SFXPitchError::InvalidConfig("Effect is disabled".to_string()));
        }

        // Check if effect is enabled in config
        if !self.is_effect_enabled_in_config(effect.effect_type) {
            return Err(SFXPitchError::InvalidConfig("Effect type is disabled in configuration".to_string()));
        }

        // Create active effect
        let active_effect = ActiveEffect {
            effect_id: effect_id.to_string(),
            sound_id: sound_id.to_string(),
            start_time: 0.0, // Will be set by caller
            duration: 1.0, // Default duration
            progress: 0.0,
            intensity: effect.intensity,
            rate: effect.rate,
            depth: effect.depth,
            phase: effect.phase,
            feedback: effect.feedback,
            enabled: true,
            bypass: false,
            parameters: effect.parameters.clone(),
        };

        let active_id = format!("{}:{}", sound_id, effect_id);
        self.active_effects.insert(active_id, active_effect);

        // Emit effect applied event
        self.emit_event(SFXPitchEvent::PitchEffectApplied {
            sound_id: sound_id.to_string(),
            effect_type: effect.effect_type.clone(),
            intensity: effect.intensity,
        });

        Ok(())
    }

    /// Remove effect from sound
    pub fn remove_effect_from_sound(&mut self, sound_id: &str, effect_id: &str) -> SFXPitchResult<()> {
        let active_id = format!("{}:{}", sound_id, effect_id);
        if !self.active_effects.contains_key(&active_id) {
            return Err(SFXPitchError::EffectNotFound(active_id));
        }

        self.active_effects.remove(&active_id);
        Ok(())
    }

    /// Get active effects for sound
    pub fn get_active_effects_for_sound(&self, sound_id: &str) -> Vec<&ActiveEffect> {
        self.active_effects.values()
            .filter(|e| e.sound_id == sound_id)
            .collect()
    }

    /// Get all active effects
    pub fn get_all_active_effects(&self) -> Vec<&ActiveEffect> {
        self.active_effects.values().collect()
    }

    /// Create vibrato effect
    pub fn create_vibrato_effect(&mut self, id: &str, rate: f32, depth: f32) -> SFXPitchResult<()> {
        let effect = PitchEffect {
            id: id.to_string(),
            name: "Vibrato".to_string(),
            effect_type: PitchEffectType::Vibrato,
            intensity: 1.0,
            rate,
            depth,
            phase: 0.0,
            feedback: 0.0,
            enabled: true,
            bypass: false,
            parameters: HashMap::new(),
        };

        self.add_effect(effect)?;
        Ok(())
    }

    /// Create tremolo effect
    pub fn create_tremolo_effect(&mut self, id: &str, rate: f32, depth: f32) -> SFXPitchResult<()> {
        let effect = PitchEffect {
            id: id.to_string(),
            name: "Tremolo".to_string(),
            effect_type: PitchEffectType::Tremolo,
            intensity: 1.0,
            rate,
            depth,
            phase: 0.0,
            feedback: 0.0,
            enabled: true,
            bypass: false,
            parameters: HashMap::new(),
        };

        self.add_effect(effect)?;
        Ok(())
    }

    /// Create wah-wah effect
    pub fn create_wah_wah_effect(&mut self, id: &str, rate: f32, depth: f32) -> SFXPitchResult<()> {
        let effect = PitchEffect {
            id: id.to_string(),
            name: "Wah-Wah".to_string(),
            effect_type: PitchEffectType::WahWah,
            intensity: 1.0,
            rate,
            depth,
            phase: 0.0,
            feedback: 0.0,
            enabled: true,
            bypass: false,
            parameters: HashMap::new(),
        };

        self.add_effect(effect)?;
        Ok(())
    }

    /// Create chorus effect
    pub fn create_chorus_effect(&mut self, id: &str, rate: f32, depth: f32, feedback: f32) -> SFXPitchResult<()> {
        let effect = PitchEffect {
            id: id.to_string(),
            name: "Chorus".to_string(),
            effect_type: PitchEffectType::Chorus,
            intensity: 1.0,
            rate,
            depth,
            phase: 0.0,
            feedback,
            enabled: true,
            bypass: false,
            parameters: HashMap::new(),
        };

        self.add_effect(effect)?;
        Ok(())
    }

    /// Create flanger effect
    pub fn create_flanger_effect(&mut self, id: &str, rate: f32, depth: f32, feedback: f32) -> SFXPitchResult<()> {
        let effect = PitchEffect {
            id: id.to_string(),
            name: "Flanger".to_string(),
            effect_type: PitchEffectType::Flanger,
            intensity: 1.0,
            rate,
            depth,
            phase: 0.0,
            feedback,
            enabled: true,
            bypass: false,
            parameters: HashMap::new(),
        };

        self.add_effect(effect)?;
        Ok(())
    }

    /// Create phaser effect
    pub fn create_phaser_effect(&mut self, id: &str, rate: f32, depth: f32, feedback: f32) -> SFXPitchResult<()> {
        let effect = PitchEffect {
            id: id.to_string(),
            name: "Phaser".to_string(),
            effect_type: PitchEffectType::Phaser,
            intensity: 1.0,
            rate,
            depth,
            phase: 0.0,
            feedback,
            enabled: true,
            bypass: false,
            parameters: HashMap::new(),
        };

        self.add_effect(effect)?;
        Ok(())
    }

    /// Create pitch shifter effect
    pub fn create_pitch_shifter_effect(&mut self, id: &str, shift: f32) -> SFXPitchResult<()> {
        let effect = PitchEffect {
            id: id.to_string(),
            name: "Pitch Shifter".to_string(),
            effect_type: PitchEffectType::PitchShifter,
            intensity: 1.0,
            rate: 0.0,
            depth: shift,
            phase: 0.0,
            feedback: 0.0,
            enabled: true,
            bypass: false,
            parameters: HashMap::new(),
        };

        self.add_effect(effect)?;
        Ok(())
    }

    /// Create harmonizer effect
    pub fn create_harmonizer_effect(&mut self, id: &str, harmony: f32) -> SFXPitchResult<()> {
        let effect = PitchEffect {
            id: id.to_string(),
            name: "Harmonizer".to_string(),
            effect_type: PitchEffectType::Harmonizer,
            intensity: 1.0,
            rate: 0.0,
            depth: harmony,
            phase: 0.0,
            feedback: 0.0,
            enabled: true,
            bypass: false,
            parameters: HashMap::new(),
        };

        self.add_effect(effect)?;
        Ok(())
    }

    /// Add effect chain
    pub fn add_effect_chain(&mut self, chain: EffectChain) -> SFXPitchResult<()> {
        let id = chain.id.clone();
        self.effect_chains.insert(id, chain);
        Ok(())
    }

    /// Remove effect chain
    pub fn remove_effect_chain(&mut self, id: &str) -> SFXPitchResult<()> {
        if !self.effect_chains.contains_key(id) {
            return Err(SFXPitchError::EffectNotFound(id.to_string()));
        }

        self.effect_chains.remove(id);
        Ok(())
    }

    /// Get effect chain
    pub fn get_effect_chain(&self, id: &str) -> Option<&EffectChain> {
        self.effect_chains.get(id)
    }

    /// Get effect chain mutably
    pub fn get_effect_chain_mut(&mut self, id: &str) -> Option<&mut EffectChain> {
        self.effect_chains.get_mut(id)
    }

    /// Apply effect chain to sound
    pub fn apply_effect_chain(&mut self, sound_id: &str, chain_id: &str) -> SFXPitchResult<()> {
        let chain = self.effect_chains.get(chain_id)
            .ok_or_else(|| SFXPitchError::EffectNotFound(chain_id.to_string()))?;

        if !chain.enabled {
            return Err(SFXPitchError::InvalidConfig("Effect chain is disabled".to_string()));
        }

        // Apply all effects in the chain
        for effect_id in &chain.effect_ids {
            self.apply_effect(sound_id, effect_id)?;
        }

        Ok(())
    }

    /// Add effect preset
    pub fn add_effect_preset(&mut self, name: &str, effects: Vec<PitchEffect>) -> SFXPitchResult<()> {
        self.effect_presets.insert(name.to_string(), effects);
        Ok(())
    }

    /// Remove effect preset
    pub fn remove_effect_preset(&mut self, name: &str) -> SFXPitchResult<()> {
        if !self.effect_presets.contains_key(name) {
            return Err(SFXPitchError::PresetNotFound(name.to_string()));
        }

        self.effect_presets.remove(name);
        Ok(())
    }

    /// Get effect preset
    pub fn get_effect_preset(&self, name: &str) -> Option<&Vec<PitchEffect>> {
        self.effect_presets.get(name)
    }

    /// Apply effect preset
    pub fn apply_effect_preset(&mut self, sound_id: &str, preset_name: &str) -> SFXPitchResult<()> {
        let effects = self.effect_presets.get(preset_name)
            .ok_or_else(|| SFXPitchError::PresetNotFound(preset_name.to_string()))?;

        for effect in effects {
            self.apply_effect(sound_id, &effect.id)?;
        }

        // Emit preset loaded event
        self.emit_event(SFXPitchEvent::PitchEffectPresetLoaded {
            preset_name: preset_name.to_string(),
            effects: effects.clone(),
        });

        Ok(())
    }

    /// Get effect history
    pub fn get_effect_history(&self) -> &Vec<EffectHistoryEntry> {
        &self.effect_history
    }

    /// Clear effect history
    pub fn clear_effect_history(&mut self) {
        self.effect_history.clear();
    }

    /// Process audio through effects
    pub fn process_audio(&self, audio_data: &mut [f32]) -> SFXPitchResult<()> {
        for effect in self.active_effects.values() {
            if effect.enabled && !effect.bypass {
                self.process_effect(effect, audio_data)?;
            }
        }
        Ok(())
    }

    /// Update active effects
    fn update_active_effects(&mut self, delta_time: f32) -> SFXPitchResult<()> {
        let mut completed_effects = Vec::new();

        for (id, effect) in self.active_effects.iter_mut() {
            if effect.enabled && !effect.bypass {
                // Update effect progress
                effect.progress += delta_time / effect.duration;
                effect.progress = effect.progress.min(1.0);

                // Update effect phase
                effect.phase += delta_time * effect.rate;
                effect.phase = effect.phase % 1.0;

                // Check if effect is completed
                if effect.progress >= 1.0 {
                    completed_effects.push(id.clone());
                }
            }
        }

        // Handle completed effects
        for id in completed_effects {
            self.handle_completed_effect(&id)?;
        }

        Ok(())
    }

    /// Update effect chains
    fn update_effect_chains(&mut self, delta_time: f32) -> SFXPitchResult<()> {
        // Update effect chains
        for chain in self.effect_chains.values_mut() {
            if chain.enabled && !chain.bypass {
                // Update chain processing
                self.update_chain_processing(chain, delta_time)?;
            }
        }
        Ok(())
    }

    /// Update effect automation
    fn update_effect_automation(&mut self, delta_time: f32) -> SFXPitchResult<()> {
        if !self.config.enable_effect_automation {
            return Ok(());
        }

        // Update effect automation
        for effect in self.active_effects.values_mut() {
            if effect.enabled && !effect.bypass {
                // Update effect parameters based on automation
                self.update_effect_automation_for_effect(effect, delta_time)?;
            }
        }
        Ok(())
    }

    /// Handle completed effect
    fn handle_completed_effect(&mut self, id: &str) -> SFXPitchResult<()> {
        if let Some(effect) = self.active_effects.remove(id) {
            // Create history entry
            let history_entry = EffectHistoryEntry {
                effect_id: effect.effect_id.clone(),
                sound_id: effect.sound_id.clone(),
                effect_type: PitchEffectType::CustomPitchEffect("Completed".to_string()),
                start_time: effect.start_time,
                end_time: effect.start_time + effect.duration,
                duration: effect.duration,
                success: true,
                parameters: effect.parameters,
            };

            self.effect_history.push(history_entry);
        }

        Ok(())
    }

    /// Update chain processing
    fn update_chain_processing(&self, chain: &EffectChain, delta_time: f32) -> SFXPitchResult<()> {
        // Update chain processing
        // In a real implementation, this would update the chain's internal state
        Ok(())
    }

    /// Update effect automation for specific effect
    fn update_effect_automation_for_effect(&self, effect: &mut ActiveEffect, delta_time: f32) -> SFXPitchResult<()> {
        // Update effect automation
        // In a real implementation, this would update effect parameters based on automation
        Ok(())
    }

    /// Process effect
    fn process_effect(&self, effect: &ActiveEffect, audio_data: &mut [f32]) -> SFXPitchResult<()> {
        // In a real implementation, this would process the audio through the effect
        // For now, we'll just simulate the processing
        
        match effect.effect_id.as_str() {
            "vibrato" => {
                // Process vibrato effect
                self.process_vibrato_effect(effect, audio_data)?;
            },
            "tremolo" => {
                // Process tremolo effect
                self.process_tremolo_effect(effect, audio_data)?;
            },
            "wah_wah" => {
                // Process wah-wah effect
                self.process_wah_wah_effect(effect, audio_data)?;
            },
            "chorus" => {
                // Process chorus effect
                self.process_chorus_effect(effect, audio_data)?;
            },
            "flanger" => {
                // Process flanger effect
                self.process_flanger_effect(effect, audio_data)?;
            },
            "phaser" => {
                // Process phaser effect
                self.process_phaser_effect(effect, audio_data)?;
            },
            "pitch_shifter" => {
                // Process pitch shifter effect
                self.process_pitch_shifter_effect(effect, audio_data)?;
            },
            "harmonizer" => {
                // Process harmonizer effect
                self.process_harmonizer_effect(effect, audio_data)?;
            },
            _ => {
                // Process generic effect
                self.process_generic_effect(effect, audio_data)?;
            },
        }
        
        Ok(())
    }

    /// Process vibrato effect
    fn process_vibrato_effect(&self, effect: &ActiveEffect, audio_data: &mut [f32]) -> SFXPitchResult<()> {
        // Vibrato effect processing
        // In a real implementation, this would apply vibrato to the audio
        Ok(())
    }

    /// Process tremolo effect
    fn process_tremolo_effect(&self, effect: &ActiveEffect, audio_data: &mut [f32]) -> SFXPitchResult<()> {
        // Tremolo effect processing
        // In a real implementation, this would apply tremolo to the audio
        Ok(())
    }

    /// Process wah-wah effect
    fn process_wah_wah_effect(&self, effect: &ActiveEffect, audio_data: &mut [f32]) -> SFXPitchResult<()> {
        // Wah-wah effect processing
        // In a real implementation, this would apply wah-wah to the audio
        Ok(())
    }

    /// Process chorus effect
    fn process_chorus_effect(&self, effect: &ActiveEffect, audio_data: &mut [f32]) -> SFXPitchResult<()> {
        // Chorus effect processing
        // In a real implementation, this would apply chorus to the audio
        Ok(())
    }

    /// Process flanger effect
    fn process_flanger_effect(&self, effect: &ActiveEffect, audio_data: &mut [f32]) -> SFXPitchResult<()> {
        // Flanger effect processing
        // In a real implementation, this would apply flanger to the audio
        Ok(())
    }

    /// Process phaser effect
    fn process_phaser_effect(&self, effect: &ActiveEffect, audio_data: &mut [f32]) -> SFXPitchResult<()> {
        // Phaser effect processing
        // In a real implementation, this would apply phaser to the audio
        Ok(())
    }

    /// Process pitch shifter effect
    fn process_pitch_shifter_effect(&self, effect: &ActiveEffect, audio_data: &mut [f32]) -> SFXPitchResult<()> {
        // Pitch shifter effect processing
        // In a real implementation, this would apply pitch shifting to the audio
        Ok(())
    }

    /// Process harmonizer effect
    fn process_harmonizer_effect(&self, effect: &ActiveEffect, audio_data: &mut [f32]) -> SFXPitchResult<()> {
        // Harmonizer effect processing
        // In a real implementation, this would apply harmonization to the audio
        Ok(())
    }

    /// Process generic effect
    fn process_generic_effect(&self, effect: &ActiveEffect, audio_data: &mut [f32]) -> SFXPitchResult<()> {
        // Generic effect processing
        // In a real implementation, this would apply the effect to the audio
        Ok(())
    }

    /// Check if effect is enabled in configuration
    fn is_effect_enabled_in_config(&self, effect_type: PitchEffectType) -> bool {
        match effect_type {
            PitchEffectType::Vibrato => self.config.enable_vibrato,
            PitchEffectType::Tremolo => self.config.enable_tremolo,
            PitchEffectType::WahWah => self.config.enable_wah_wah,
            PitchEffectType::Chorus => self.config.enable_chorus,
            PitchEffectType::Flanger => self.config.enable_flanger,
            PitchEffectType::Phaser => self.config.enable_phaser,
            PitchEffectType::PitchShifter => self.config.enable_pitch_shifter,
            PitchEffectType::Harmonizer => self.config.enable_harmonizer,
            PitchEffectType::PitchBender => self.config.enable_pitch_bender,
            PitchEffectType::PitchCorrector => self.config.enable_pitch_corrector,
            PitchEffectType::PitchSmoother => self.config.enable_pitch_smoother,
            PitchEffectType::PitchAnalyzer => self.config.enable_pitch_analyzer,
            PitchEffectType::PitchTracker => self.config.enable_pitch_tracker,
            PitchEffectType::PitchDetector => self.config.enable_pitch_detector,
            PitchEffectType::PitchQuantizer => self.config.enable_pitch_quantizer,
            PitchEffectType::PitchScaler => self.config.enable_pitch_scaler,
            PitchEffectType::PitchCompressor => self.config.enable_pitch_compressor,
            PitchEffectType::PitchExpander => self.config.enable_pitch_expander,
            PitchEffectType::PitchGate => self.config.enable_pitch_gate,
            PitchEffectType::PitchLimiter => self.config.enable_pitch_limiter,
            PitchEffectType::PitchFilter => self.config.enable_pitch_filter,
            PitchEffectType::PitchEQ => self.config.enable_pitch_eq,
            PitchEffectType::PitchReverb => self.config.enable_pitch_reverb,
            PitchEffectType::PitchDelay => self.config.enable_pitch_delay,
            PitchEffectType::PitchEcho => self.config.enable_pitch_echo,
            PitchEffectType::PitchDistortion => self.config.enable_pitch_distortion,
            PitchEffectType::PitchOverdrive => self.config.enable_pitch_overdrive,
            PitchEffectType::PitchFuzz => self.config.enable_pitch_fuzz,
            PitchEffectType::PitchBitCrusher => self.config.enable_pitch_bit_crusher,
            PitchEffectType::PitchSampleRateReducer => self.config.enable_pitch_sample_rate_reducer,
            PitchEffectType::PitchRingModulator => self.config.enable_pitch_ring_modulator,
            PitchEffectType::PitchFrequencyShifter => self.config.enable_pitch_frequency_shifter,
            PitchEffectType::PitchStereoWidener => self.config.enable_pitch_stereo_widener,
            PitchEffectType::PitchMonoMaker => self.config.enable_pitch_mono_maker,
            PitchEffectType::PitchStereoImager => self.config.enable_pitch_stereo_imager,
            PitchEffectType::PitchSpatialProcessor => self.config.enable_pitch_spatial_processor,
            PitchEffectType::PitchConvolution => self.config.enable_pitch_convolution,
            PitchEffectType::PitchImpulseResponse => self.config.enable_pitch_impulse_response,
            PitchEffectType::CustomPitchEffect(_) => true,
        }
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

impl Default for EffectsConfig {
    fn default() -> Self {
        Self {
            enable_effects: true,
            enable_vibrato: true,
            enable_tremolo: true,
            enable_wah_wah: true,
            enable_chorus: true,
            enable_flanger: true,
            enable_phaser: true,
            enable_pitch_shifter: true,
            enable_harmonizer: true,
            enable_pitch_bender: true,
            enable_pitch_corrector: true,
            enable_pitch_smoother: true,
            enable_pitch_analyzer: true,
            enable_pitch_tracker: true,
            enable_pitch_detector: true,
            enable_pitch_quantizer: true,
            enable_pitch_scaler: true,
            enable_pitch_compressor: true,
            enable_pitch_expander: true,
            enable_pitch_gate: true,
            enable_pitch_limiter: true,
            enable_pitch_filter: true,
            enable_pitch_eq: true,
            enable_pitch_reverb: true,
            enable_pitch_delay: true,
            enable_pitch_echo: true,
            enable_pitch_distortion: true,
            enable_pitch_overdrive: true,
            enable_pitch_fuzz: true,
            enable_pitch_bit_crusher: true,
            enable_pitch_sample_rate_reducer: true,
            enable_pitch_ring_modulator: true,
            enable_pitch_frequency_shifter: true,
            enable_pitch_stereo_widener: true,
            enable_pitch_mono_maker: true,
            enable_pitch_stereo_imager: true,
            enable_pitch_spatial_processor: true,
            enable_pitch_convolution: true,
            enable_pitch_impulse_response: true,
            max_effects_per_sound: 4,
            max_effects_per_chain: 8,
            effect_quality: EffectQuality::High,
            sample_rate: 44100,
            buffer_size: 1024,
            enable_real_time_processing: true,
            enable_effect_automation: true,
            enable_effect_presets: true,
        }
    }
}

impl Default for PitchEffects {
    fn default() -> Self {
        Self::new(EffectsConfig::default())
    }
}
