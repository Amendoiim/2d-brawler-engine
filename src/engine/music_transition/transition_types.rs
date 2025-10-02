//! Transition Types
//! 
//! This module provides various music transition types including fade in/out,
//! crossfade, beat-synchronized transitions, and advanced transition types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::TransitionCurve;

/// Music transition types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransitionType {
    /// Fade in from silence
    FadeIn {
        /// Fade in duration
        duration: f32,
        /// Fade in curve
        curve: TransitionCurve,
        /// Fade in effects
        effects: Vec<String>,
    },
    /// Fade out to silence
    FadeOut {
        /// Fade out duration
        duration: f32,
        /// Fade out curve
        curve: TransitionCurve,
        /// Fade out effects
        effects: Vec<String>,
    },
    /// Crossfade between two tracks
    Crossfade {
        /// Crossfade duration
        duration: f32,
        /// Crossfade curve
        curve: TransitionCurve,
        /// Crossfade effects
        effects: Vec<String>,
        /// Crossfade balance (0.0 = full from, 1.0 = full to)
        balance: f32,
    },
    /// Beat-synchronized transition
    BeatSync {
        /// Beat sync duration (in beats)
        duration_beats: f32,
        /// Beat sync curve
        curve: TransitionCurve,
        /// Beat sync effects
        effects: Vec<String>,
        /// Beat sync accuracy (0.0 to 1.0)
        accuracy: f32,
    },
    /// Phrase-synchronized transition
    PhraseSync {
        /// Phrase sync duration (in phrases)
        duration_phrases: f32,
        /// Phrase sync curve
        curve: TransitionCurve,
        /// Phrase sync effects
        effects: Vec<String>,
        /// Phrase sync accuracy (0.0 to 1.0)
        accuracy: f32,
    },
    /// Measure-synchronized transition
    MeasureSync {
        /// Measure sync duration (in measures)
        duration_measures: f32,
        /// Measure sync curve
        curve: TransitionCurve,
        /// Measure sync effects
        effects: Vec<String>,
        /// Measure sync accuracy (0.0 to 1.0)
        accuracy: f32,
    },
    /// Instant transition (no fade)
    Instant {
        /// Instant transition effects
        effects: Vec<String>,
    },
    /// Segue transition (overlapping tracks)
    Segue {
        /// Segue duration
        duration: f32,
        /// Segue curve
        curve: TransitionCurve,
        /// Segue effects
        effects: Vec<String>,
        /// Segue overlap (0.0 to 1.0)
        overlap: f32,
    },
    /// Stinger transition (short accent)
    Stinger {
        /// Stinger duration
        duration: f32,
        /// Stinger curve
        curve: TransitionCurve,
        /// Stinger effects
        effects: Vec<String>,
        /// Stinger intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Swell transition (volume swell)
    Swell {
        /// Swell duration
        duration: f32,
        /// Swell curve
        curve: TransitionCurve,
        /// Swell effects
        effects: Vec<String>,
        /// Swell intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Duck transition (volume duck)
    Duck {
        /// Duck duration
        duration: f32,
        /// Duck curve
        curve: TransitionCurve,
        /// Duck effects
        effects: Vec<String>,
        /// Duck intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Filter transition (frequency filtering)
    Filter {
        /// Filter duration
        duration: f32,
        /// Filter curve
        curve: TransitionCurve,
        /// Filter effects
        effects: Vec<String>,
        /// Filter type
        filter_type: FilterType,
        /// Filter frequency
        frequency: f32,
    },
    /// Reverb transition (reverb tail)
    Reverb {
        /// Reverb duration
        duration: f32,
        /// Reverb curve
        curve: TransitionCurve,
        /// Reverb effects
        effects: Vec<String>,
        /// Reverb intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Delay transition (delay tail)
    Delay {
        /// Delay duration
        duration: f32,
        /// Delay curve
        curve: TransitionCurve,
        /// Delay effects
        effects: Vec<String>,
        /// Delay intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Echo transition (echo tail)
    Echo {
        /// Echo duration
        duration: f32,
        /// Echo curve
        curve: TransitionCurve,
        /// Echo effects
        effects: Vec<String>,
        /// Echo intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Distortion transition (distortion effect)
    Distortion {
        /// Distortion duration
        duration: f32,
        /// Distortion curve
        curve: TransitionCurve,
        /// Distortion effects
        effects: Vec<String>,
        /// Distortion intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Chorus transition (chorus effect)
    Chorus {
        /// Chorus duration
        duration: f32,
        /// Chorus curve
        curve: TransitionCurve,
        /// Chorus effects
        effects: Vec<String>,
        /// Chorus intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Flanger transition (flanger effect)
    Flanger {
        /// Flanger duration
        duration: f32,
        /// Flanger curve
        curve: TransitionCurve,
        /// Flanger effects
        effects: Vec<String>,
        /// Flanger intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Phaser transition (phaser effect)
    Phaser {
        /// Phaser duration
        duration: f32,
        /// Phaser curve
        curve: TransitionCurve,
        /// Phaser effects
        effects: Vec<String>,
        /// Phaser intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Wah-wah transition (wah-wah effect)
    WahWah {
        /// Wah-wah duration
        duration: f32,
        /// Wah-wah curve
        curve: TransitionCurve,
        /// Wah-wah effects
        effects: Vec<String>,
        /// Wah-wah intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Tremolo transition (tremolo effect)
    Tremolo {
        /// Tremolo duration
        duration: f32,
        /// Tremolo curve
        curve: TransitionCurve,
        /// Tremolo effects
        effects: Vec<String>,
        /// Tremolo intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Vibrato transition (vibrato effect)
    Vibrato {
        /// Vibrato duration
        duration: f32,
        /// Vibrato curve
        curve: TransitionCurve,
        /// Vibrato effects
        effects: Vec<String>,
        /// Vibrato intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Pitch shift transition (pitch shifting)
    PitchShift {
        /// Pitch shift duration
        duration: f32,
        /// Pitch shift curve
        curve: TransitionCurve,
        /// Pitch shift effects
        effects: Vec<String>,
        /// Pitch shift amount (semitones)
        shift_amount: f32,
    },
    /// Time stretch transition (time stretching)
    TimeStretch {
        /// Time stretch duration
        duration: f32,
        /// Time stretch curve
        curve: TransitionCurve,
        /// Time stretch effects
        effects: Vec<String>,
        /// Time stretch factor
        stretch_factor: f32,
    },
    /// Reverse transition (reverse playback)
    Reverse {
        /// Reverse duration
        duration: f32,
        /// Reverse curve
        curve: TransitionCurve,
        /// Reverse effects
        effects: Vec<String>,
        /// Reverse intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Granular transition (granular synthesis)
    Granular {
        /// Granular duration
        duration: f32,
        /// Granular curve
        curve: TransitionCurve,
        /// Granular effects
        effects: Vec<String>,
        /// Granular intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Spectral transition (spectral processing)
    Spectral {
        /// Spectral duration
        duration: f32,
        /// Spectral curve
        curve: TransitionCurve,
        /// Spectral effects
        effects: Vec<String>,
        /// Spectral intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Convolution transition (convolution reverb)
    Convolution {
        /// Convolution duration
        duration: f32,
        /// Convolution curve
        curve: TransitionCurve,
        /// Convolution effects
        effects: Vec<String>,
        /// Convolution intensity (0.0 to 1.0)
        intensity: f32,
    },
    /// Custom transition (user-defined)
    Custom {
        /// Custom transition name
        name: String,
        /// Custom transition duration
        duration: f32,
        /// Custom transition curve
        curve: TransitionCurve,
        /// Custom transition effects
        effects: Vec<String>,
        /// Custom transition parameters
        parameters: HashMap<String, f32>,
    },
}

/// Filter types for filter transitions
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

/// Transition type utilities
impl TransitionType {
    /// Get transition duration
    pub fn get_duration(&self) -> f32 {
        match self {
            TransitionType::FadeIn { duration, .. } => *duration,
            TransitionType::FadeOut { duration, .. } => *duration,
            TransitionType::Crossfade { duration, .. } => *duration,
            TransitionType::BeatSync { duration_beats, .. } => *duration_beats,
            TransitionType::PhraseSync { duration_phrases, .. } => *duration_phrases,
            TransitionType::MeasureSync { duration_measures, .. } => *duration_measures,
            TransitionType::Instant { .. } => 0.0,
            TransitionType::Segue { duration, .. } => *duration,
            TransitionType::Stinger { duration, .. } => *duration,
            TransitionType::Swell { duration, .. } => *duration,
            TransitionType::Duck { duration, .. } => *duration,
            TransitionType::Filter { duration, .. } => *duration,
            TransitionType::Reverb { duration, .. } => *duration,
            TransitionType::Delay { duration, .. } => *duration,
            TransitionType::Echo { duration, .. } => *duration,
            TransitionType::Distortion { duration, .. } => *duration,
            TransitionType::Chorus { duration, .. } => *duration,
            TransitionType::Flanger { duration, .. } => *duration,
            TransitionType::Phaser { duration, .. } => *duration,
            TransitionType::WahWah { duration, .. } => *duration,
            TransitionType::Tremolo { duration, .. } => *duration,
            TransitionType::Vibrato { duration, .. } => *duration,
            TransitionType::PitchShift { duration, .. } => *duration,
            TransitionType::TimeStretch { duration, .. } => *duration,
            TransitionType::Reverse { duration, .. } => *duration,
            TransitionType::Granular { duration, .. } => *duration,
            TransitionType::Spectral { duration, .. } => *duration,
            TransitionType::Convolution { duration, .. } => *duration,
            TransitionType::Custom { duration, .. } => *duration,
        }
    }

    /// Get transition curve
    pub fn get_curve(&self) -> Option<TransitionCurve> {
        match self {
            TransitionType::FadeIn { curve, .. } => Some(curve.clone()),
            TransitionType::FadeOut { curve, .. } => Some(curve.clone()),
            TransitionType::Crossfade { curve, .. } => Some(curve.clone()),
            TransitionType::BeatSync { curve, .. } => Some(curve.clone()),
            TransitionType::PhraseSync { curve, .. } => Some(curve.clone()),
            TransitionType::MeasureSync { curve, .. } => Some(curve.clone()),
            TransitionType::Instant { .. } => None,
            TransitionType::Segue { curve, .. } => Some(curve.clone()),
            TransitionType::Stinger { curve, .. } => Some(curve.clone()),
            TransitionType::Swell { curve, .. } => Some(curve.clone()),
            TransitionType::Duck { curve, .. } => Some(curve.clone()),
            TransitionType::Filter { curve, .. } => Some(curve.clone()),
            TransitionType::Reverb { curve, .. } => Some(curve.clone()),
            TransitionType::Delay { curve, .. } => Some(curve.clone()),
            TransitionType::Echo { curve, .. } => Some(curve.clone()),
            TransitionType::Distortion { curve, .. } => Some(curve.clone()),
            TransitionType::Chorus { curve, .. } => Some(curve.clone()),
            TransitionType::Flanger { curve, .. } => Some(curve.clone()),
            TransitionType::Phaser { curve, .. } => Some(curve.clone()),
            TransitionType::WahWah { curve, .. } => Some(curve.clone()),
            TransitionType::Tremolo { curve, .. } => Some(curve.clone()),
            TransitionType::Vibrato { curve, .. } => Some(curve.clone()),
            TransitionType::PitchShift { curve, .. } => Some(curve.clone()),
            TransitionType::TimeStretch { curve, .. } => Some(curve.clone()),
            TransitionType::Reverse { curve, .. } => Some(curve.clone()),
            TransitionType::Granular { curve, .. } => Some(curve.clone()),
            TransitionType::Spectral { curve, .. } => Some(curve.clone()),
            TransitionType::Convolution { curve, .. } => Some(curve.clone()),
            TransitionType::Custom { curve, .. } => Some(curve.clone()),
        }
    }

    /// Get transition effects
    pub fn get_effects(&self) -> Vec<String> {
        match self {
            TransitionType::FadeIn { effects, .. } => effects.clone(),
            TransitionType::FadeOut { effects, .. } => effects.clone(),
            TransitionType::Crossfade { effects, .. } => effects.clone(),
            TransitionType::BeatSync { effects, .. } => effects.clone(),
            TransitionType::PhraseSync { effects, .. } => effects.clone(),
            TransitionType::MeasureSync { effects, .. } => effects.clone(),
            TransitionType::Instant { effects, .. } => effects.clone(),
            TransitionType::Segue { effects, .. } => effects.clone(),
            TransitionType::Stinger { effects, .. } => effects.clone(),
            TransitionType::Swell { effects, .. } => effects.clone(),
            TransitionType::Duck { effects, .. } => effects.clone(),
            TransitionType::Filter { effects, .. } => effects.clone(),
            TransitionType::Reverb { effects, .. } => effects.clone(),
            TransitionType::Delay { effects, .. } => effects.clone(),
            TransitionType::Echo { effects, .. } => effects.clone(),
            TransitionType::Distortion { effects, .. } => effects.clone(),
            TransitionType::Chorus { effects, .. } => effects.clone(),
            TransitionType::Flanger { effects, .. } => effects.clone(),
            TransitionType::Phaser { effects, .. } => effects.clone(),
            TransitionType::WahWah { effects, .. } => effects.clone(),
            TransitionType::Tremolo { effects, .. } => effects.clone(),
            TransitionType::Vibrato { effects, .. } => effects.clone(),
            TransitionType::PitchShift { effects, .. } => effects.clone(),
            TransitionType::TimeStretch { effects, .. } => effects.clone(),
            TransitionType::Reverse { effects, .. } => effects.clone(),
            TransitionType::Granular { effects, .. } => effects.clone(),
            TransitionType::Spectral { effects, .. } => effects.clone(),
            TransitionType::Convolution { effects, .. } => effects.clone(),
            TransitionType::Custom { effects, .. } => effects.clone(),
        }
    }

    /// Check if transition is synchronized
    pub fn is_synchronized(&self) -> bool {
        matches!(
            self,
            TransitionType::BeatSync { .. } | 
            TransitionType::PhraseSync { .. } | 
            TransitionType::MeasureSync { .. }
        )
    }

    /// Check if transition has effects
    pub fn has_effects(&self) -> bool {
        !self.get_effects().is_empty()
    }

    /// Check if transition is instant
    pub fn is_instant(&self) -> bool {
        matches!(self, TransitionType::Instant { .. })
    }

    /// Get transition type name
    pub fn get_type_name(&self) -> &'static str {
        match self {
            TransitionType::FadeIn { .. } => "FadeIn",
            TransitionType::FadeOut { .. } => "FadeOut",
            TransitionType::Crossfade { .. } => "Crossfade",
            TransitionType::BeatSync { .. } => "BeatSync",
            TransitionType::PhraseSync { .. } => "PhraseSync",
            TransitionType::MeasureSync { .. } => "MeasureSync",
            TransitionType::Instant { .. } => "Instant",
            TransitionType::Segue { .. } => "Segue",
            TransitionType::Stinger { .. } => "Stinger",
            TransitionType::Swell { .. } => "Swell",
            TransitionType::Duck { .. } => "Duck",
            TransitionType::Filter { .. } => "Filter",
            TransitionType::Reverb { .. } => "Reverb",
            TransitionType::Delay { .. } => "Delay",
            TransitionType::Echo { .. } => "Echo",
            TransitionType::Distortion { .. } => "Distortion",
            TransitionType::Chorus { .. } => "Chorus",
            TransitionType::Flanger { .. } => "Flanger",
            TransitionType::Phaser { .. } => "Phaser",
            TransitionType::WahWah { .. } => "WahWah",
            TransitionType::Tremolo { .. } => "Tremolo",
            TransitionType::Vibrato { .. } => "Vibrato",
            TransitionType::PitchShift { .. } => "PitchShift",
            TransitionType::TimeStretch { .. } => "TimeStretch",
            TransitionType::Reverse { .. } => "Reverse",
            TransitionType::Granular { .. } => "Granular",
            TransitionType::Spectral { .. } => "Spectral",
            TransitionType::Convolution { .. } => "Convolution",
            TransitionType::Custom { name, .. } => name,
        }
    }
}

impl Default for FilterType {
    fn default() -> Self {
        FilterType::LowPass
    }
}
