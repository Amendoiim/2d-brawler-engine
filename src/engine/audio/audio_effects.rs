//! Audio Effects System
//! 
//! This module provides comprehensive audio effects processing including filters, modulation, and special effects.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::{AudioResult, AudioError, AudioEvent, AudioSourceType};

/// Audio effects manager
pub struct AudioEffectsManager {
    /// Active effects
    pub active_effects: HashMap<String, AudioEffect>,
    /// Effect presets
    pub effect_presets: HashMap<String, EffectPreset>,
    /// Global effects settings
    pub global_settings: EffectsSettings,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&AudioEvent) + Send + Sync>>,
}

/// Audio effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEffect {
    /// Effect ID
    pub id: String,
    /// Source ID
    pub source_id: String,
    /// Effect type
    pub effect_type: EffectType,
    /// Effect parameters
    pub parameters: EffectParameters,
    /// Wet mix (0.0 to 1.0)
    pub wet_mix: f32,
    /// Dry mix (0.0 to 1.0)
    pub dry_mix: f32,
    /// Enabled
    pub enabled: bool,
    /// Bypass
    pub bypass: bool,
    /// Order in chain
    pub order: u32,
}

/// Effect type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EffectType {
    /// Low pass filter
    LowPassFilter,
    /// High pass filter
    HighPassFilter,
    /// Band pass filter
    BandPassFilter,
    /// Notch filter
    NotchFilter,
    /// Low shelf filter
    LowShelfFilter,
    /// High shelf filter
    HighShelfFilter,
    /// Peak filter
    PeakFilter,
    /// Distortion
    Distortion,
    /// Overdrive
    Overdrive,
    /// Fuzz
    Fuzz,
    /// Chorus
    Chorus,
    /// Flanger
    Flanger,
    /// Phaser
    Phaser,
    /// Tremolo
    Tremolo,
    /// Vibrato
    Vibrato,
    /// Delay
    Delay,
    /// Echo
    Echo,
    /// Reverb
    Reverb,
    /// Compressor
    Compressor,
    /// Limiter
    Limiter,
    /// Gate
    Gate,
    /// Expander
    Expander,
    /// EQ
    EQ,
    /// Wah
    Wah,
    /// Auto-wah
    AutoWah,
    /// Bit crusher
    BitCrusher,
    /// Sample rate reducer
    SampleRateReducer,
    /// Ring modulator
    RingModulator,
    /// Frequency shifter
    FrequencyShifter,
    /// Pitch shifter
    PitchShifter,
    /// Harmonizer
    Harmonizer,
    /// Vocoder
    Vocoder,
    /// Auto-tune
    AutoTune,
    /// De-esser
    DeEsser,
    /// Noise gate
    NoiseGate,
    /// Exciter
    Exciter,
    /// Enhancer
    Enhancer,
    /// Stereo widener
    StereoWidener,
    /// Mono maker
    MonoMaker,
    /// Stereo imager
    StereoImager,
    /// Spatial processor
    SpatialProcessor,
    /// Convolution
    Convolution,
    /// Impulse response
    ImpulseResponse,
    /// Custom effect
    Custom(String),
}

/// Effect parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectParameters {
    /// Frequency (Hz)
    pub frequency: f32,
    /// Gain (dB)
    pub gain: f32,
    /// Q factor
    pub q_factor: f32,
    /// Resonance
    pub resonance: f32,
    /// Cutoff frequency
    pub cutoff_frequency: f32,
    /// Bandwidth
    pub bandwidth: f32,
    /// Slope
    pub slope: f32,
    /// Drive
    pub drive: f32,
    /// Tone
    pub tone: f32,
    /// Mix
    pub mix: f32,
    /// Feedback
    pub feedback: f32,
    /// Delay time (ms)
    pub delay_time: f32,
    /// Decay time (ms)
    pub decay_time: f32,
    /// Rate (Hz)
    pub rate: f32,
    /// Depth
    pub depth: f32,
    /// Phase
    pub phase: f32,
    /// Width
    pub width: f32,
    /// Center frequency
    pub center_frequency: f32,
    /// Bandwidth
    pub bandwidth_frequency: f32,
    /// Threshold (dB)
    pub threshold: f32,
    /// Ratio
    pub ratio: f32,
    /// Attack time (ms)
    pub attack_time: f32,
    /// Release time (ms)
    pub release_time: f32,
    /// Knee width (dB)
    pub knee_width: f32,
    /// Makeup gain (dB)
    pub makeup_gain: f32,
    /// Lookahead time (ms)
    pub lookahead_time: f32,
    /// Hold time (ms)
    pub hold_time: f32,
    /// Range (dB)
    pub range: f32,
    /// Floor (dB)
    pub floor: f32,
    /// Hysteresis (dB)
    pub hysteresis: f32,
    /// Sidechain source
    pub sidechain_source: String,
    /// Sidechain filter
    pub sidechain_filter: bool,
    /// Sidechain filter frequency (Hz)
    pub sidechain_filter_frequency: f32,
    /// Sidechain filter slope
    pub sidechain_filter_slope: f32,
    /// Sidechain filter resonance
    pub sidechain_filter_resonance: f32,
    /// Custom parameters
    pub custom: HashMap<String, f32>,
}

/// Effect preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectPreset {
    /// Preset name
    pub name: String,
    /// Preset description
    pub description: String,
    /// Effect type
    pub effect_type: EffectType,
    /// Effect parameters
    pub parameters: EffectParameters,
    /// Preset category
    pub category: EffectCategory,
}

/// Effect category
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EffectCategory {
    /// Filter effects
    Filter,
    /// Distortion effects
    Distortion,
    /// Modulation effects
    Modulation,
    /// Time-based effects
    TimeBased,
    /// Dynamic effects
    Dynamic,
    /// EQ effects
    EQ,
    /// Pitch effects
    Pitch,
    /// Spatial effects
    Spatial,
    /// Utility effects
    Utility,
    /// Custom effects
    Custom,
}

/// Effects settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectsSettings {
    /// Global effects enabled
    pub global_effects_enabled: bool,
    /// Maximum effects per source
    pub max_effects_per_source: usize,
    /// Effects quality
    pub effects_quality: EffectsQuality,
    /// Enable real-time processing
    pub enable_real_time_processing: bool,
    /// Enable effect chains
    pub enable_effect_chains: bool,
    /// Enable effect presets
    pub enable_effect_presets: bool,
    /// Enable custom effects
    pub enable_custom_effects: bool,
}

/// Effects quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EffectsQuality {
    /// Low quality
    Low,
    /// Medium quality
    Medium,
    /// High quality
    High,
    /// Ultra quality
    Ultra,
}

impl AudioEffectsManager {
    /// Create a new audio effects manager
    pub fn new() -> Self {
        let mut manager = Self {
            active_effects: HashMap::new(),
            effect_presets: HashMap::new(),
            global_settings: EffectsSettings::default(),
            event_handlers: Vec::new(),
        };

        // Initialize default presets
        manager.initialize_default_presets();
        manager
    }

    /// Update audio effects manager
    pub fn update(&mut self, delta_time: f32) -> AudioResult<()> {
        // Update active effects
        for (id, effect) in self.active_effects.iter_mut() {
            if effect.enabled && !effect.bypass {
                // Process effect
                self.process_effect(effect)?;
            }
        }

        Ok(())
    }

    /// Add effect preset
    pub fn add_effect_preset(&mut self, preset: EffectPreset) {
        self.effect_presets.insert(preset.name.clone(), preset);
    }

    /// Remove effect preset
    pub fn remove_effect_preset(&mut self, name: &str) {
        self.effect_presets.remove(name);
    }

    /// Get effect preset
    pub fn get_effect_preset(&self, name: &str) -> Option<&EffectPreset> {
        self.effect_presets.get(name)
    }

    /// Apply effect preset to source
    pub fn apply_effect_preset(&mut self, source_id: &str, preset_name: &str) -> AudioResult<()> {
        let preset = self.effect_presets.get(preset_name)
            .ok_or_else(|| AudioError::SourceNotFound(preset_name.to_string()))?;

        let effect = AudioEffect {
            id: format!("{}_effect_{}", source_id, preset_name),
            source_id: source_id.to_string(),
            effect_type: preset.effect_type.clone(),
            parameters: preset.parameters.clone(),
            wet_mix: 0.5,
            dry_mix: 0.5,
            enabled: true,
            bypass: false,
            order: 0,
        };

        self.active_effects.insert(effect.id.clone(), effect);
        Ok(())
    }

    /// Remove effect
    pub fn remove_effect(&mut self, effect_id: &str) -> AudioResult<()> {
        if !self.active_effects.contains_key(effect_id) {
            return Err(AudioError::SourceNotFound(effect_id.to_string()));
        }

        self.active_effects.remove(effect_id);
        Ok(())
    }

    /// Get effect
    pub fn get_effect(&self, effect_id: &str) -> Option<&AudioEffect> {
        self.active_effects.get(effect_id)
    }

    /// Get effect mutably
    pub fn get_effect_mut(&mut self, effect_id: &str) -> Option<&mut AudioEffect> {
        self.active_effects.get_mut(effect_id)
    }

    /// Set effect parameters
    pub fn set_effect_parameters(&mut self, effect_id: &str, parameters: EffectParameters) -> AudioResult<()> {
        let effect = self.active_effects.get_mut(effect_id)
            .ok_or_else(|| AudioError::SourceNotFound(effect_id.to_string()))?;

        effect.parameters = parameters;
        Ok(())
    }

    /// Set effect wet mix
    pub fn set_effect_wet_mix(&mut self, effect_id: &str, wet_mix: f32) -> AudioResult<()> {
        let effect = self.active_effects.get_mut(effect_id)
            .ok_or_else(|| AudioError::SourceNotFound(effect_id.to_string()))?;

        effect.wet_mix = wet_mix.max(0.0).min(1.0);
        Ok(())
    }

    /// Set effect dry mix
    pub fn set_effect_dry_mix(&mut self, effect_id: &str, dry_mix: f32) -> AudioResult<()> {
        let effect = self.active_effects.get_mut(effect_id)
            .ok_or_else(|| AudioError::SourceNotFound(effect_id.to_string()))?;

        effect.dry_mix = dry_mix.max(0.0).min(1.0);
        Ok(())
    }

    /// Enable/disable effect
    pub fn set_effect_enabled(&mut self, effect_id: &str, enabled: bool) -> AudioResult<()> {
        let effect = self.active_effects.get_mut(effect_id)
            .ok_or_else(|| AudioError::SourceNotFound(effect_id.to_string()))?;

        effect.enabled = enabled;
        Ok(())
    }

    /// Bypass effect
    pub fn set_effect_bypass(&mut self, effect_id: &str, bypass: bool) -> AudioResult<()> {
        let effect = self.active_effects.get_mut(effect_id)
            .ok_or_else(|| AudioError::SourceNotFound(effect_id.to_string()))?;

        effect.bypass = bypass;
        Ok(())
    }

    /// Set effect order
    pub fn set_effect_order(&mut self, effect_id: &str, order: u32) -> AudioResult<()> {
        let effect = self.active_effects.get_mut(effect_id)
            .ok_or_else(|| AudioError::SourceNotFound(effect_id.to_string()))?;

        effect.order = order;
        Ok(())
    }

    /// Get effects for source
    pub fn get_effects_for_source(&self, source_id: &str) -> Vec<&AudioEffect> {
        self.active_effects.values()
            .filter(|effect| effect.source_id == source_id)
            .collect()
    }

    /// Set global effects settings
    pub fn set_global_settings(&mut self, settings: EffectsSettings) {
        self.global_settings = settings;
    }

    /// Get global effects settings
    pub fn get_global_settings(&self) -> &EffectsSettings {
        &self.global_settings
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&AudioEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Process effect
    fn process_effect(&self, effect: &AudioEffect) -> AudioResult<()> {
        // In a real implementation, this would process the audio through the effect algorithm
        // For now, we'll just simulate the processing
        
        match effect.effect_type {
            EffectType::LowPassFilter => {
                // Process low pass filter
                let cutoff = effect.parameters.cutoff_frequency;
                let resonance = effect.parameters.resonance;
                // Apply filter processing
            },
            EffectType::HighPassFilter => {
                // Process high pass filter
                let cutoff = effect.parameters.cutoff_frequency;
                let resonance = effect.parameters.resonance;
                // Apply filter processing
            },
            EffectType::BandPassFilter => {
                // Process band pass filter
                let frequency = effect.parameters.frequency;
                let q_factor = effect.parameters.q_factor;
                // Apply filter processing
            },
            EffectType::NotchFilter => {
                // Process notch filter
                let frequency = effect.parameters.frequency;
                let q_factor = effect.parameters.q_factor;
                // Apply filter processing
            },
            EffectType::LowShelfFilter => {
                // Process low shelf filter
                let frequency = effect.parameters.frequency;
                let gain = effect.parameters.gain;
                // Apply filter processing
            },
            EffectType::HighShelfFilter => {
                // Process high shelf filter
                let frequency = effect.parameters.frequency;
                let gain = effect.parameters.gain;
                // Apply filter processing
            },
            EffectType::PeakFilter => {
                // Process peak filter
                let frequency = effect.parameters.frequency;
                let gain = effect.parameters.gain;
                let q_factor = effect.parameters.q_factor;
                // Apply filter processing
            },
            EffectType::Distortion => {
                // Process distortion
                let drive = effect.parameters.drive;
                let tone = effect.parameters.tone;
                // Apply distortion processing
            },
            EffectType::Overdrive => {
                // Process overdrive
                let drive = effect.parameters.drive;
                let tone = effect.parameters.tone;
                // Apply overdrive processing
            },
            EffectType::Fuzz => {
                // Process fuzz
                let drive = effect.parameters.drive;
                let tone = effect.parameters.tone;
                // Apply fuzz processing
            },
            EffectType::Chorus => {
                // Process chorus
                let rate = effect.parameters.rate;
                let depth = effect.parameters.depth;
                let feedback = effect.parameters.feedback;
                // Apply chorus processing
            },
            EffectType::Flanger => {
                // Process flanger
                let rate = effect.parameters.rate;
                let depth = effect.parameters.depth;
                let feedback = effect.parameters.feedback;
                // Apply flanger processing
            },
            EffectType::Phaser => {
                // Process phaser
                let rate = effect.parameters.rate;
                let depth = effect.parameters.depth;
                let feedback = effect.parameters.feedback;
                // Apply phaser processing
            },
            EffectType::Tremolo => {
                // Process tremolo
                let rate = effect.parameters.rate;
                let depth = effect.parameters.depth;
                // Apply tremolo processing
            },
            EffectType::Vibrato => {
                // Process vibrato
                let rate = effect.parameters.rate;
                let depth = effect.parameters.depth;
                // Apply vibrato processing
            },
            EffectType::Delay => {
                // Process delay
                let delay_time = effect.parameters.delay_time;
                let feedback = effect.parameters.feedback;
                let mix = effect.parameters.mix;
                // Apply delay processing
            },
            EffectType::Echo => {
                // Process echo
                let delay_time = effect.parameters.delay_time;
                let feedback = effect.parameters.feedback;
                let mix = effect.parameters.mix;
                // Apply echo processing
            },
            EffectType::Reverb => {
                // Process reverb
                let decay_time = effect.parameters.decay_time;
                let mix = effect.parameters.mix;
                // Apply reverb processing
            },
            EffectType::Compressor => {
                // Process compressor
                let threshold = effect.parameters.threshold;
                let ratio = effect.parameters.ratio;
                let attack_time = effect.parameters.attack_time;
                let release_time = effect.parameters.release_time;
                // Apply compressor processing
            },
            EffectType::Limiter => {
                // Process limiter
                let threshold = effect.parameters.threshold;
                let attack_time = effect.parameters.attack_time;
                let release_time = effect.parameters.release_time;
                // Apply limiter processing
            },
            EffectType::Gate => {
                // Process gate
                let threshold = effect.parameters.threshold;
                let attack_time = effect.parameters.attack_time;
                let release_time = effect.parameters.release_time;
                // Apply gate processing
            },
            EffectType::Expander => {
                // Process expander
                let threshold = effect.parameters.threshold;
                let ratio = effect.parameters.ratio;
                let attack_time = effect.parameters.attack_time;
                let release_time = effect.parameters.release_time;
                // Apply expander processing
            },
            EffectType::EQ => {
                // Process EQ
                let frequency = effect.parameters.frequency;
                let gain = effect.parameters.gain;
                let q_factor = effect.parameters.q_factor;
                // Apply EQ processing
            },
            EffectType::Wah => {
                // Process wah
                let frequency = effect.parameters.frequency;
                let q_factor = effect.parameters.q_factor;
                // Apply wah processing
            },
            EffectType::AutoWah => {
                // Process auto-wah
                let frequency = effect.parameters.frequency;
                let q_factor = effect.parameters.q_factor;
                let rate = effect.parameters.rate;
                // Apply auto-wah processing
            },
            EffectType::BitCrusher => {
                // Process bit crusher
                let bit_depth = effect.parameters.custom.get("bit_depth").unwrap_or(&16.0);
                let sample_rate = effect.parameters.custom.get("sample_rate").unwrap_or(&44100.0);
                // Apply bit crusher processing
            },
            EffectType::SampleRateReducer => {
                // Process sample rate reducer
                let sample_rate = effect.parameters.custom.get("sample_rate").unwrap_or(&44100.0);
                // Apply sample rate reducer processing
            },
            EffectType::RingModulator => {
                // Process ring modulator
                let frequency = effect.parameters.frequency;
                let mix = effect.parameters.mix;
                // Apply ring modulator processing
            },
            EffectType::FrequencyShifter => {
                // Process frequency shifter
                let frequency = effect.parameters.frequency;
                let mix = effect.parameters.mix;
                // Apply frequency shifter processing
            },
            EffectType::PitchShifter => {
                // Process pitch shifter
                let pitch = effect.parameters.custom.get("pitch").unwrap_or(&0.0);
                let mix = effect.parameters.mix;
                // Apply pitch shifter processing
            },
            EffectType::Harmonizer => {
                // Process harmonizer
                let pitch = effect.parameters.custom.get("pitch").unwrap_or(&0.0);
                let mix = effect.parameters.mix;
                // Apply harmonizer processing
            },
            EffectType::Vocoder => {
                // Process vocoder
                let carrier_frequency = effect.parameters.frequency;
                let modulator_frequency = effect.parameters.custom.get("modulator_frequency").unwrap_or(&1000.0);
                // Apply vocoder processing
            },
            EffectType::AutoTune => {
                // Process auto-tune
                let pitch = effect.parameters.custom.get("pitch").unwrap_or(&0.0);
                let mix = effect.parameters.mix;
                // Apply auto-tune processing
            },
            EffectType::DeEsser => {
                // Process de-esser
                let frequency = effect.parameters.frequency;
                let threshold = effect.parameters.threshold;
                let ratio = effect.parameters.ratio;
                // Apply de-esser processing
            },
            EffectType::NoiseGate => {
                // Process noise gate
                let threshold = effect.parameters.threshold;
                let attack_time = effect.parameters.attack_time;
                let release_time = effect.parameters.release_time;
                // Apply noise gate processing
            },
            EffectType::Exciter => {
                // Process exciter
                let frequency = effect.parameters.frequency;
                let gain = effect.parameters.gain;
                let mix = effect.parameters.mix;
                // Apply exciter processing
            },
            EffectType::Enhancer => {
                // Process enhancer
                let frequency = effect.parameters.frequency;
                let gain = effect.parameters.gain;
                let mix = effect.parameters.mix;
                // Apply enhancer processing
            },
            EffectType::StereoWidener => {
                // Process stereo widener
                let width = effect.parameters.width;
                let mix = effect.parameters.mix;
                // Apply stereo widener processing
            },
            EffectType::MonoMaker => {
                // Process mono maker
                let mix = effect.parameters.mix;
                // Apply mono maker processing
            },
            EffectType::StereoImager => {
                // Process stereo imager
                let width = effect.parameters.width;
                let mix = effect.parameters.mix;
                // Apply stereo imager processing
            },
            EffectType::SpatialProcessor => {
                // Process spatial processor
                let width = effect.parameters.width;
                let mix = effect.parameters.mix;
                // Apply spatial processor processing
            },
            EffectType::Convolution => {
                // Process convolution
                let impulse_response = effect.parameters.custom.get("impulse_response").unwrap_or(&0.0);
                let mix = effect.parameters.mix;
                // Apply convolution processing
            },
            EffectType::ImpulseResponse => {
                // Process impulse response
                let impulse_response = effect.parameters.custom.get("impulse_response").unwrap_or(&0.0);
                let mix = effect.parameters.mix;
                // Apply impulse response processing
            },
            EffectType::Custom(_) => {
                // Process custom effect
                // Apply custom effect processing
            },
        }

        Ok(())
    }

    /// Initialize default effect presets
    fn initialize_default_presets(&mut self) {
        // Low pass filter preset
        self.add_effect_preset(EffectPreset {
            name: "Low Pass Filter".to_string(),
            description: "Basic low pass filter".to_string(),
            effect_type: EffectType::LowPassFilter,
            parameters: EffectParameters {
                cutoff_frequency: 1000.0,
                resonance: 0.7,
                ..Default::default()
            },
            category: EffectCategory::Filter,
        });

        // High pass filter preset
        self.add_effect_preset(EffectPreset {
            name: "High Pass Filter".to_string(),
            description: "Basic high pass filter".to_string(),
            effect_type: EffectType::HighPassFilter,
            parameters: EffectParameters {
                cutoff_frequency: 1000.0,
                resonance: 0.7,
                ..Default::default()
            },
            category: EffectCategory::Filter,
        });

        // Distortion preset
        self.add_effect_preset(EffectPreset {
            name: "Distortion".to_string(),
            description: "Basic distortion effect".to_string(),
            effect_type: EffectType::Distortion,
            parameters: EffectParameters {
                drive: 0.5,
                tone: 0.5,
                mix: 0.5,
                ..Default::default()
            },
            category: EffectCategory::Distortion,
        });

        // Chorus preset
        self.add_effect_preset(EffectPreset {
            name: "Chorus".to_string(),
            description: "Basic chorus effect".to_string(),
            effect_type: EffectType::Chorus,
            parameters: EffectParameters {
                rate: 0.5,
                depth: 0.5,
                feedback: 0.3,
                mix: 0.5,
                ..Default::default()
            },
            category: EffectCategory::Modulation,
        });

        // Delay preset
        self.add_effect_preset(EffectPreset {
            name: "Delay".to_string(),
            description: "Basic delay effect".to_string(),
            effect_type: EffectType::Delay,
            parameters: EffectParameters {
                delay_time: 250.0,
                feedback: 0.3,
                mix: 0.5,
                ..Default::default()
            },
            category: EffectCategory::TimeBased,
        });

        // Compressor preset
        self.add_effect_preset(EffectPreset {
            name: "Compressor".to_string(),
            description: "Basic compressor".to_string(),
            effect_type: EffectType::Compressor,
            parameters: EffectParameters {
                threshold: -12.0,
                ratio: 4.0,
                attack_time: 10.0,
                release_time: 100.0,
                makeup_gain: 0.0,
                ..Default::default()
            },
            category: EffectCategory::Dynamic,
        });

        // EQ preset
        self.add_effect_preset(EffectPreset {
            name: "EQ".to_string(),
            description: "Basic EQ".to_string(),
            effect_type: EffectType::EQ,
            parameters: EffectParameters {
                frequency: 1000.0,
                gain: 0.0,
                q_factor: 1.0,
                ..Default::default()
            },
            category: EffectCategory::EQ,
        });
    }

    /// Emit audio event
    fn emit_event(&self, event: AudioEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for EffectParameters {
    fn default() -> Self {
        Self {
            frequency: 1000.0,
            gain: 0.0,
            q_factor: 1.0,
            resonance: 0.7,
            cutoff_frequency: 1000.0,
            bandwidth: 1.0,
            slope: 12.0,
            drive: 0.5,
            tone: 0.5,
            mix: 0.5,
            feedback: 0.3,
            delay_time: 250.0,
            decay_time: 1000.0,
            rate: 0.5,
            depth: 0.5,
            phase: 0.0,
            width: 1.0,
            center_frequency: 1000.0,
            bandwidth_frequency: 1000.0,
            threshold: -12.0,
            ratio: 4.0,
            attack_time: 10.0,
            release_time: 100.0,
            knee_width: 2.0,
            makeup_gain: 0.0,
            lookahead_time: 0.0,
            hold_time: 0.0,
            range: 0.0,
            floor: -60.0,
            hysteresis: 0.0,
            sidechain_source: String::new(),
            sidechain_filter: false,
            sidechain_filter_frequency: 1000.0,
            sidechain_filter_slope: 12.0,
            sidechain_filter_resonance: 0.7,
            custom: HashMap::new(),
        }
    }
}

impl Default for EffectsSettings {
    fn default() -> Self {
        Self {
            global_effects_enabled: true,
            max_effects_per_source: 8,
            effects_quality: EffectsQuality::High,
            enable_real_time_processing: true,
            enable_effect_chains: true,
            enable_effect_presets: true,
            enable_custom_effects: true,
        }
    }
}

impl Default for AudioEffectsManager {
    fn default() -> Self {
        Self::new()
    }
}
