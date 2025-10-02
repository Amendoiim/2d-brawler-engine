//! Transition Effects
//! 
//! This module provides various transition effects for enhancing music
//! transitions including reverb, delay, echo, distortion, and custom effects.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Transition effect types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransitionEffect {
    /// Reverb effect
    Reverb {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Reverb intensity (0.0 to 1.0)
        intensity: f32,
        /// Reverb room size
        room_size: f32,
        /// Reverb damping
        damping: f32,
        /// Reverb wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Delay effect
    Delay {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Delay intensity (0.0 to 1.0)
        intensity: f32,
        /// Delay time (seconds)
        delay_time: f32,
        /// Delay feedback
        feedback: f32,
        /// Delay wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Echo effect
    Echo {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Echo intensity (0.0 to 1.0)
        intensity: f32,
        /// Echo delay time (seconds)
        delay_time: f32,
        /// Echo feedback
        feedback: f32,
        /// Echo wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Distortion effect
    Distortion {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Distortion intensity (0.0 to 1.0)
        intensity: f32,
        /// Distortion drive
        drive: f32,
        /// Distortion tone
        tone: f32,
        /// Distortion wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Chorus effect
    Chorus {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Chorus intensity (0.0 to 1.0)
        intensity: f32,
        /// Chorus rate (Hz)
        rate: f32,
        /// Chorus depth
        depth: f32,
        /// Chorus wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Flanger effect
    Flanger {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Flanger intensity (0.0 to 1.0)
        intensity: f32,
        /// Flanger rate (Hz)
        rate: f32,
        /// Flanger depth
        depth: f32,
        /// Flanger feedback
        feedback: f32,
        /// Flanger wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Phaser effect
    Phaser {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Phaser intensity (0.0 to 1.0)
        intensity: f32,
        /// Phaser rate (Hz)
        rate: f32,
        /// Phaser depth
        depth: f32,
        /// Phaser feedback
        feedback: f32,
        /// Phaser wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Wah-wah effect
    WahWah {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Wah-wah intensity (0.0 to 1.0)
        intensity: f32,
        /// Wah-wah rate (Hz)
        rate: f32,
        /// Wah-wah depth
        depth: f32,
        /// Wah-wah wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Tremolo effect
    Tremolo {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Tremolo intensity (0.0 to 1.0)
        intensity: f32,
        /// Tremolo rate (Hz)
        rate: f32,
        /// Tremolo depth
        depth: f32,
        /// Tremolo wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Vibrato effect
    Vibrato {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Vibrato intensity (0.0 to 1.0)
        intensity: f32,
        /// Vibrato rate (Hz)
        rate: f32,
        /// Vibrato depth
        depth: f32,
        /// Vibrato wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Filter effect
    Filter {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Filter intensity (0.0 to 1.0)
        intensity: f32,
        /// Filter type
        filter_type: FilterType,
        /// Filter frequency (Hz)
        frequency: f32,
        /// Filter resonance
        resonance: f32,
        /// Filter wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// EQ effect
    EQ {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// EQ intensity (0.0 to 1.0)
        intensity: f32,
        /// EQ bands
        bands: Vec<EQBand>,
        /// EQ wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Compressor effect
    Compressor {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Compressor intensity (0.0 to 1.0)
        intensity: f32,
        /// Compressor threshold
        threshold: f32,
        /// Compressor ratio
        ratio: f32,
        /// Compressor attack
        attack: f32,
        /// Compressor release
        release: f32,
        /// Compressor wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Limiter effect
    Limiter {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Limiter intensity (0.0 to 1.0)
        intensity: f32,
        /// Limiter threshold
        threshold: f32,
        /// Limiter ceiling
        ceiling: f32,
        /// Limiter release
        release: f32,
        /// Limiter wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Gate effect
    Gate {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Gate intensity (0.0 to 1.0)
        intensity: f32,
        /// Gate threshold
        threshold: f32,
        /// Gate ratio
        ratio: f32,
        /// Gate attack
        attack: f32,
        /// Gate release
        release: f32,
        /// Gate wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Expander effect
    Expander {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Expander intensity (0.0 to 1.0)
        intensity: f32,
        /// Expander threshold
        threshold: f32,
        /// Expander ratio
        ratio: f32,
        /// Expander attack
        attack: f32,
        /// Expander release
        release: f32,
        /// Expander wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Pitch shift effect
    PitchShift {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Pitch shift intensity (0.0 to 1.0)
        intensity: f32,
        /// Pitch shift amount (semitones)
        shift_amount: f32,
        /// Pitch shift wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Time stretch effect
    TimeStretch {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Time stretch intensity (0.0 to 1.0)
        intensity: f32,
        /// Time stretch factor
        stretch_factor: f32,
        /// Time stretch wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Reverse effect
    Reverse {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Reverse intensity (0.0 to 1.0)
        intensity: f32,
        /// Reverse wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Granular effect
    Granular {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Granular intensity (0.0 to 1.0)
        intensity: f32,
        /// Granular grain size
        grain_size: f32,
        /// Granular grain density
        grain_density: f32,
        /// Granular wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Spectral effect
    Spectral {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Spectral intensity (0.0 to 1.0)
        intensity: f32,
        /// Spectral processing type
        processing_type: SpectralProcessingType,
        /// Spectral wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Convolution effect
    Convolution {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Convolution intensity (0.0 to 1.0)
        intensity: f32,
        /// Convolution impulse response
        impulse_response: String,
        /// Convolution wet/dry mix
        wet_dry_mix: f32,
        /// Effect enabled
        enabled: bool,
        /// Effect parameters
        parameters: HashMap<String, f32>,
    },
    /// Custom effect
    Custom {
        /// Effect ID
        id: String,
        /// Effect name
        name: String,
        /// Custom effect type
        effect_type: String,
        /// Custom effect intensity (0.0 to 1.0)
        intensity: f32,
        /// Custom effect wet/dry mix
        wet_dry_mix: f32,
        /// Custom effect enabled
        enabled: bool,
        /// Custom effect parameters
        parameters: HashMap<String, f32>,
    },
}

/// Filter types for filter effects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FilterType {
    /// Low-pass filter
    LowPass,
    /// High-pass filter
    HighPass,
    /// Band-pass filter
    BandPass,
    /// Band-stop filter
    BandStop,
    /// Notch filter
    Notch,
    /// All-pass filter
    AllPass,
    /// Shelf filter
    Shelf,
    /// Peak filter
    Peak,
    /// Custom filter
    Custom(String),
}

/// EQ band
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EQBand {
    /// Band frequency (Hz)
    pub frequency: f32,
    /// Band gain (dB)
    pub gain: f32,
    /// Band Q factor
    pub q: f32,
    /// Band enabled
    pub enabled: bool,
}

/// Spectral processing types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpectralProcessingType {
    /// Spectral filtering
    Filtering,
    /// Spectral masking
    Masking,
    /// Spectral morphing
    Morphing,
    /// Spectral synthesis
    Synthesis,
    /// Spectral analysis
    Analysis,
    /// Spectral transformation
    Transformation,
    /// Custom spectral processing
    Custom(String),
}

/// Transition effect utilities
impl TransitionEffect {
    /// Get effect ID
    pub fn get_id(&self) -> &str {
        match self {
            TransitionEffect::Reverb { id, .. } => id,
            TransitionEffect::Delay { id, .. } => id,
            TransitionEffect::Echo { id, .. } => id,
            TransitionEffect::Distortion { id, .. } => id,
            TransitionEffect::Chorus { id, .. } => id,
            TransitionEffect::Flanger { id, .. } => id,
            TransitionEffect::Phaser { id, .. } => id,
            TransitionEffect::WahWah { id, .. } => id,
            TransitionEffect::Tremolo { id, .. } => id,
            TransitionEffect::Vibrato { id, .. } => id,
            TransitionEffect::Filter { id, .. } => id,
            TransitionEffect::EQ { id, .. } => id,
            TransitionEffect::Compressor { id, .. } => id,
            TransitionEffect::Limiter { id, .. } => id,
            TransitionEffect::Gate { id, .. } => id,
            TransitionEffect::Expander { id, .. } => id,
            TransitionEffect::PitchShift { id, .. } => id,
            TransitionEffect::TimeStretch { id, .. } => id,
            TransitionEffect::Reverse { id, .. } => id,
            TransitionEffect::Granular { id, .. } => id,
            TransitionEffect::Spectral { id, .. } => id,
            TransitionEffect::Convolution { id, .. } => id,
            TransitionEffect::Custom { id, .. } => id,
        }
    }

    /// Get effect name
    pub fn get_name(&self) -> &str {
        match self {
            TransitionEffect::Reverb { name, .. } => name,
            TransitionEffect::Delay { name, .. } => name,
            TransitionEffect::Echo { name, .. } => name,
            TransitionEffect::Distortion { name, .. } => name,
            TransitionEffect::Chorus { name, .. } => name,
            TransitionEffect::Flanger { name, .. } => name,
            TransitionEffect::Phaser { name, .. } => name,
            TransitionEffect::WahWah { name, .. } => name,
            TransitionEffect::Tremolo { name, .. } => name,
            TransitionEffect::Vibrato { name, .. } => name,
            TransitionEffect::Filter { name, .. } => name,
            TransitionEffect::EQ { name, .. } => name,
            TransitionEffect::Compressor { name, .. } => name,
            TransitionEffect::Limiter { name, .. } => name,
            TransitionEffect::Gate { name, .. } => name,
            TransitionEffect::Expander { name, .. } => name,
            TransitionEffect::PitchShift { name, .. } => name,
            TransitionEffect::TimeStretch { name, .. } => name,
            TransitionEffect::Reverse { name, .. } => name,
            TransitionEffect::Granular { name, .. } => name,
            TransitionEffect::Spectral { name, .. } => name,
            TransitionEffect::Convolution { name, .. } => name,
            TransitionEffect::Custom { name, .. } => name,
        }
    }

    /// Get effect intensity
    pub fn get_intensity(&self) -> f32 {
        match self {
            TransitionEffect::Reverb { intensity, .. } => *intensity,
            TransitionEffect::Delay { intensity, .. } => *intensity,
            TransitionEffect::Echo { intensity, .. } => *intensity,
            TransitionEffect::Distortion { intensity, .. } => *intensity,
            TransitionEffect::Chorus { intensity, .. } => *intensity,
            TransitionEffect::Flanger { intensity, .. } => *intensity,
            TransitionEffect::Phaser { intensity, .. } => *intensity,
            TransitionEffect::WahWah { intensity, .. } => *intensity,
            TransitionEffect::Tremolo { intensity, .. } => *intensity,
            TransitionEffect::Vibrato { intensity, .. } => *intensity,
            TransitionEffect::Filter { intensity, .. } => *intensity,
            TransitionEffect::EQ { intensity, .. } => *intensity,
            TransitionEffect::Compressor { intensity, .. } => *intensity,
            TransitionEffect::Limiter { intensity, .. } => *intensity,
            TransitionEffect::Gate { intensity, .. } => *intensity,
            TransitionEffect::Expander { intensity, .. } => *intensity,
            TransitionEffect::PitchShift { intensity, .. } => *intensity,
            TransitionEffect::TimeStretch { intensity, .. } => *intensity,
            TransitionEffect::Reverse { intensity, .. } => *intensity,
            TransitionEffect::Granular { intensity, .. } => *intensity,
            TransitionEffect::Spectral { intensity, .. } => *intensity,
            TransitionEffect::Convolution { intensity, .. } => *intensity,
            TransitionEffect::Custom { intensity, .. } => *intensity,
        }
    }

    /// Get effect wet/dry mix
    pub fn get_wet_dry_mix(&self) -> f32 {
        match self {
            TransitionEffect::Reverb { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Delay { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Echo { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Distortion { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Chorus { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Flanger { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Phaser { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::WahWah { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Tremolo { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Vibrato { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Filter { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::EQ { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Compressor { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Limiter { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Gate { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Expander { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::PitchShift { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::TimeStretch { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Reverse { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Granular { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Spectral { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Convolution { wet_dry_mix, .. } => *wet_dry_mix,
            TransitionEffect::Custom { wet_dry_mix, .. } => *wet_dry_mix,
        }
    }

    /// Check if effect is enabled
    pub fn is_enabled(&self) -> bool {
        match self {
            TransitionEffect::Reverb { enabled, .. } => *enabled,
            TransitionEffect::Delay { enabled, .. } => *enabled,
            TransitionEffect::Echo { enabled, .. } => *enabled,
            TransitionEffect::Distortion { enabled, .. } => *enabled,
            TransitionEffect::Chorus { enabled, .. } => *enabled,
            TransitionEffect::Flanger { enabled, .. } => *enabled,
            TransitionEffect::Phaser { enabled, .. } => *enabled,
            TransitionEffect::WahWah { enabled, .. } => *enabled,
            TransitionEffect::Tremolo { enabled, .. } => *enabled,
            TransitionEffect::Vibrato { enabled, .. } => *enabled,
            TransitionEffect::Filter { enabled, .. } => *enabled,
            TransitionEffect::EQ { enabled, .. } => *enabled,
            TransitionEffect::Compressor { enabled, .. } => *enabled,
            TransitionEffect::Limiter { enabled, .. } => *enabled,
            TransitionEffect::Gate { enabled, .. } => *enabled,
            TransitionEffect::Expander { enabled, .. } => *enabled,
            TransitionEffect::PitchShift { enabled, .. } => *enabled,
            TransitionEffect::TimeStretch { enabled, .. } => *enabled,
            TransitionEffect::Reverse { enabled, .. } => *enabled,
            TransitionEffect::Granular { enabled, .. } => *enabled,
            TransitionEffect::Spectral { enabled, .. } => *enabled,
            TransitionEffect::Convolution { enabled, .. } => *enabled,
            TransitionEffect::Custom { enabled, .. } => *enabled,
        }
    }

    /// Get effect type name
    pub fn get_type_name(&self) -> &'static str {
        match self {
            TransitionEffect::Reverb { .. } => "Reverb",
            TransitionEffect::Delay { .. } => "Delay",
            TransitionEffect::Echo { .. } => "Echo",
            TransitionEffect::Distortion { .. } => "Distortion",
            TransitionEffect::Chorus { .. } => "Chorus",
            TransitionEffect::Flanger { .. } => "Flanger",
            TransitionEffect::Phaser { .. } => "Phaser",
            TransitionEffect::WahWah { .. } => "WahWah",
            TransitionEffect::Tremolo { .. } => "Tremolo",
            TransitionEffect::Vibrato { .. } => "Vibrato",
            TransitionEffect::Filter { .. } => "Filter",
            TransitionEffect::EQ { .. } => "EQ",
            TransitionEffect::Compressor { .. } => "Compressor",
            TransitionEffect::Limiter { .. } => "Limiter",
            TransitionEffect::Gate { .. } => "Gate",
            TransitionEffect::Expander { .. } => "Expander",
            TransitionEffect::PitchShift { .. } => "PitchShift",
            TransitionEffect::TimeStretch { .. } => "TimeStretch",
            TransitionEffect::Reverse { .. } => "Reverse",
            TransitionEffect::Granular { .. } => "Granular",
            TransitionEffect::Spectral { .. } => "Spectral",
            TransitionEffect::Convolution { .. } => "Convolution",
            TransitionEffect::Custom { .. } => "Custom",
        }
    }

    /// Set effect intensity
    pub fn set_intensity(&mut self, intensity: f32) {
        let intensity = intensity.max(0.0).min(1.0);
        match self {
            TransitionEffect::Reverb { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Delay { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Echo { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Distortion { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Chorus { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Flanger { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Phaser { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::WahWah { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Tremolo { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Vibrato { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Filter { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::EQ { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Compressor { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Limiter { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Gate { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Expander { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::PitchShift { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::TimeStretch { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Reverse { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Granular { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Spectral { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Convolution { intensity: ref mut i, .. } => *i = intensity,
            TransitionEffect::Custom { intensity: ref mut i, .. } => *i = intensity,
        }
    }

    /// Set effect enabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        match self {
            TransitionEffect::Reverb { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Delay { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Echo { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Distortion { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Chorus { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Flanger { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Phaser { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::WahWah { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Tremolo { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Vibrato { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Filter { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::EQ { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Compressor { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Limiter { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Gate { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Expander { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::PitchShift { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::TimeStretch { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Reverse { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Granular { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Spectral { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Convolution { enabled: ref mut e, .. } => *e = enabled,
            TransitionEffect::Custom { enabled: ref mut e, .. } => *e = enabled,
        }
    }

    /// Process audio through effect
    pub fn process_audio(&self, audio_data: &mut [f32]) -> Result<(), String> {
        if !self.is_enabled() {
            return Ok(());
        }

        // In a real implementation, this would process the audio through the effect
        // For now, we'll just simulate the processing based on intensity and wet/dry mix
        
        let intensity = self.get_intensity();
        let wet_dry_mix = self.get_wet_dry_mix();
        
        for sample in audio_data.iter_mut() {
            // Apply intensity scaling
            *sample *= intensity;
            
            // Apply wet/dry mix
            *sample = *sample * wet_dry_mix + *sample * (1.0 - wet_dry_mix);
        }
        
        Ok(())
    }
}

impl Default for FilterType {
    fn default() -> Self {
        FilterType::LowPass
    }
}

impl Default for SpectralProcessingType {
    fn default() -> Self {
        SpectralProcessingType::Filtering
    }
}
