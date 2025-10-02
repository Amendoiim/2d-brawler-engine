//! Pitch Variations
//! 
//! This module provides pitch variation algorithms including random variations,
//! preset variations, semitone control, and microtone control.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{SFXPitchResult, SFXPitchError, PitchVariationType, PitchVariation, SFXPitchEvent};

/// Pitch variations manager
pub struct PitchVariations {
    /// Variations configuration
    pub config: VariationsConfig,
    /// Available variations
    pub variations: HashMap<String, PitchVariation>,
    /// Variation presets
    pub variation_presets: HashMap<String, Vec<PitchVariation>>,
    /// Active variations
    pub active_variations: HashMap<String, ActiveVariation>,
    /// Variation history
    pub variation_history: Vec<VariationHistoryEntry>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&SFXPitchEvent) + Send + Sync>>,
}

/// Variations configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariationsConfig {
    /// Enable variations
    pub enable_variations: bool,
    /// Enable random variations
    pub enable_random_variations: bool,
    /// Enable preset variations
    pub enable_preset_variations: bool,
    /// Enable custom variations
    pub enable_custom_variations: bool,
    /// Enable semitone variations
    pub enable_semitone_variations: bool,
    /// Enable microtone variations
    pub enable_microtone_variations: bool,
    /// Enable harmonic variations
    pub enable_harmonic_variations: bool,
    /// Enable subharmonic variations
    pub enable_subharmonic_variations: bool,
    /// Enable chromatic variations
    pub enable_chromatic_variations: bool,
    /// Enable scale variations
    pub enable_scale_variations: bool,
    /// Maximum variations per sound
    pub max_variations_per_sound: usize,
    /// Variation probability threshold
    pub variation_probability_threshold: f32,
    /// Variation weight threshold
    pub variation_weight_threshold: f32,
    /// Variation history size
    pub variation_history_size: usize,
    /// Enable variation analysis
    pub enable_variation_analysis: bool,
    /// Enable variation optimization
    pub enable_variation_optimization: bool,
}

/// Active variation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveVariation {
    /// Variation ID
    pub variation_id: String,
    /// Sound ID
    pub sound_id: String,
    /// Variation start time
    pub start_time: f32,
    /// Variation duration
    pub duration: f32,
    /// Variation progress (0.0 to 1.0)
    pub progress: f32,
    /// Variation intensity (0.0 to 1.0)
    pub intensity: f32,
    /// Variation enabled
    pub enabled: bool,
    /// Variation parameters
    pub parameters: HashMap<String, f32>,
}

/// Variation history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariationHistoryEntry {
    /// Variation ID
    pub variation_id: String,
    /// Sound ID
    pub sound_id: String,
    /// Variation type
    pub variation_type: PitchVariationType,
    /// Variation start time
    pub start_time: f32,
    /// Variation end time
    pub end_time: f32,
    /// Variation duration
    pub duration: f32,
    /// Variation success
    pub success: bool,
    /// Variation parameters
    pub parameters: HashMap<String, f32>,
}

impl PitchVariations {
    /// Create new pitch variations manager
    pub fn new(config: VariationsConfig) -> Self {
        Self {
            config,
            variations: HashMap::new(),
            variation_presets: HashMap::new(),
            active_variations: HashMap::new(),
            variation_history: Vec::new(),
            event_handlers: Vec::new(),
        }
    }

    /// Update pitch variations
    pub fn update(&mut self, delta_time: f32) -> SFXPitchResult<()> {
        if !self.config.enable_variations {
            return Ok(());
        }

        // Update active variations
        self.update_active_variations(delta_time)?;

        // Analyze variations
        if self.config.enable_variation_analysis {
            self.analyze_variations()?;
        }

        // Optimize variations
        if self.config.enable_variation_optimization {
            self.optimize_variations()?;
        }

        Ok(())
    }

    /// Add variation
    pub fn add_variation(&mut self, variation: PitchVariation) -> SFXPitchResult<()> {
        let id = variation.id.clone();
        self.variations.insert(id, variation);
        Ok(())
    }

    /// Remove variation
    pub fn remove_variation(&mut self, id: &str) -> SFXPitchResult<()> {
        if !self.variations.contains_key(id) {
            return Err(SFXPitchError::VariationNotFound(id.to_string()));
        }

        self.variations.remove(id);
        Ok(())
    }

    /// Get variation
    pub fn get_variation(&self, id: &str) -> Option<&PitchVariation> {
        self.variations.get(id)
    }

    /// Get variation mutably
    pub fn get_variation_mut(&mut self, id: &str) -> Option<&mut PitchVariation> {
        self.variations.get_mut(id)
    }

    /// Apply variation to sound
    pub fn apply_variation(&mut self, sound_id: &str, variation_id: &str) -> SFXPitchResult<()> {
        let variation = self.variations.get(variation_id)
            .ok_or_else(|| SFXPitchError::VariationNotFound(variation_id.to_string()))?;

        if !variation.enabled {
            return Err(SFXPitchError::InvalidConfig("Variation is disabled".to_string()));
        }

        // Check if variation should be applied based on probability
        if variation.probability < self.config.variation_probability_threshold {
            return Ok(());
        }

        // Create active variation
        let active_variation = ActiveVariation {
            variation_id: variation_id.to_string(),
            sound_id: sound_id.to_string(),
            start_time: 0.0, // Will be set by caller
            duration: 1.0, // Default duration
            progress: 0.0,
            intensity: variation.weight,
            enabled: true,
            parameters: variation.parameters.clone(),
        };

        let active_id = format!("{}:{}", sound_id, variation_id);
        self.active_variations.insert(active_id, active_variation);

        // Emit variation applied event
        self.emit_event(SFXPitchEvent::PitchVariationApplied {
            sound_id: sound_id.to_string(),
            variation_type: variation.variation_type.clone(),
            pitch: variation.pitch_multiplier,
        });

        Ok(())
    }

    /// Remove variation from sound
    pub fn remove_variation_from_sound(&mut self, sound_id: &str, variation_id: &str) -> SFXPitchResult<()> {
        let active_id = format!("{}:{}", sound_id, variation_id);
        if !self.active_variations.contains_key(&active_id) {
            return Err(SFXPitchError::VariationNotFound(active_id));
        }

        self.active_variations.remove(&active_id);
        Ok(())
    }

    /// Get active variations for sound
    pub fn get_active_variations_for_sound(&self, sound_id: &str) -> Vec<&ActiveVariation> {
        self.active_variations.values()
            .filter(|v| v.sound_id == sound_id)
            .collect()
    }

    /// Get all active variations
    pub fn get_all_active_variations(&self) -> Vec<&ActiveVariation> {
        self.active_variations.values().collect()
    }

    /// Generate random variation
    pub fn generate_random_variation(&mut self, sound_id: &str) -> SFXPitchResult<()> {
        if !self.config.enable_random_variations {
            return Ok(());
        }

        // Generate random variation parameters
        let pitch_multiplier = self.generate_random_pitch_multiplier();
        let pitch_offset = self.generate_random_pitch_offset();
        let probability = self.generate_random_probability();
        let weight = self.generate_random_weight();

        // Create random variation
        let variation = PitchVariation {
            id: format!("random_{}", uuid::Uuid::new_v4()),
            name: "Random Variation".to_string(),
            variation_type: PitchVariationType::Random,
            pitch_multiplier,
            pitch_offset,
            probability,
            weight,
            enabled: true,
            parameters: HashMap::new(),
        };

        // Apply variation
        self.apply_variation(sound_id, &variation.id)?;

        Ok(())
    }

    /// Generate semitone variation
    pub fn generate_semitone_variation(&mut self, sound_id: &str, semitones: i32) -> SFXPitchResult<()> {
        if !self.config.enable_semitone_variations {
            return Err(SFXPitchError::InvalidConfig("Semitone variations disabled".to_string()));
        }

        // Calculate pitch multiplier from semitones
        let pitch_multiplier = 2.0_f32.powf(semitones as f32 / 12.0);
        let pitch_offset = semitones as f32;

        // Create semitone variation
        let variation = PitchVariation {
            id: format!("semitone_{}_{}", sound_id, semitones),
            name: format!("Semitone +{}", semitones),
            variation_type: PitchVariationType::Semitone,
            pitch_multiplier,
            pitch_offset,
            probability: 1.0,
            weight: 1.0,
            enabled: true,
            parameters: HashMap::new(),
        };

        // Apply variation
        self.apply_variation(sound_id, &variation.id)?;

        Ok(())
    }

    /// Generate microtone variation
    pub fn generate_microtone_variation(&mut self, sound_id: &str, microtones: f32) -> SFXPitchResult<()> {
        if !self.config.enable_microtone_variations {
            return Err(SFXPitchError::InvalidConfig("Microtone variations disabled".to_string()));
        }

        // Calculate pitch multiplier from microtones
        let pitch_multiplier = 2.0_f32.powf(microtones / 12.0);
        let pitch_offset = microtones;

        // Create microtone variation
        let variation = PitchVariation {
            id: format!("microtone_{}_{}", sound_id, microtones),
            name: format!("Microtone +{:.2}", microtones),
            variation_type: PitchVariationType::Microtone,
            pitch_multiplier,
            pitch_offset,
            probability: 1.0,
            weight: 1.0,
            enabled: true,
            parameters: HashMap::new(),
        };

        // Apply variation
        self.apply_variation(sound_id, &variation.id)?;

        Ok(())
    }

    /// Generate harmonic variation
    pub fn generate_harmonic_variation(&mut self, sound_id: &str, harmonic: u32) -> SFXPitchResult<()> {
        if !self.config.enable_harmonic_variations {
            return Err(SFXPitchError::InvalidConfig("Harmonic variations disabled".to_string()));
        }

        // Calculate pitch multiplier from harmonic
        let pitch_multiplier = harmonic as f32;
        let pitch_offset = 12.0 * (harmonic as f32).log2();

        // Create harmonic variation
        let variation = PitchVariation {
            id: format!("harmonic_{}_{}", sound_id, harmonic),
            name: format!("Harmonic {}", harmonic),
            variation_type: PitchVariationType::Harmonic,
            pitch_multiplier,
            pitch_offset,
            probability: 1.0,
            weight: 1.0,
            enabled: true,
            parameters: HashMap::new(),
        };

        // Apply variation
        self.apply_variation(sound_id, &variation.id)?;

        Ok(())
    }

    /// Generate subharmonic variation
    pub fn generate_subharmonic_variation(&mut self, sound_id: &str, subharmonic: u32) -> SFXPitchResult<()> {
        if !self.config.enable_subharmonic_variations {
            return Err(SFXPitchError::InvalidConfig("Subharmonic variations disabled".to_string()));
        }

        // Calculate pitch multiplier from subharmonic
        let pitch_multiplier = 1.0 / subharmonic as f32;
        let pitch_offset = -12.0 * (subharmonic as f32).log2();

        // Create subharmonic variation
        let variation = PitchVariation {
            id: format!("subharmonic_{}_{}", sound_id, subharmonic),
            name: format!("Subharmonic 1/{}", subharmonic),
            variation_type: PitchVariationType::Subharmonic,
            pitch_multiplier,
            pitch_offset,
            probability: 1.0,
            weight: 1.0,
            enabled: true,
            parameters: HashMap::new(),
        };

        // Apply variation
        self.apply_variation(sound_id, &variation.id)?;

        Ok(())
    }

    /// Generate chromatic variation
    pub fn generate_chromatic_variation(&mut self, sound_id: &str, note: i32) -> SFXPitchResult<()> {
        if !self.config.enable_chromatic_variations {
            return Err(SFXPitchError::InvalidConfig("Chromatic variations disabled".to_string()));
        }

        // Calculate pitch multiplier from chromatic note
        let pitch_multiplier = 2.0_f32.powf(note as f32 / 12.0);
        let pitch_offset = note as f32;

        // Create chromatic variation
        let variation = PitchVariation {
            id: format!("chromatic_{}_{}", sound_id, note),
            name: format!("Chromatic +{}", note),
            variation_type: PitchVariationType::Chromatic,
            pitch_multiplier,
            pitch_offset,
            probability: 1.0,
            weight: 1.0,
            enabled: true,
            parameters: HashMap::new(),
        };

        // Apply variation
        self.apply_variation(sound_id, &variation.id)?;

        Ok(())
    }

    /// Generate scale variation
    pub fn generate_scale_variation(&mut self, sound_id: &str, scale: &str, degree: i32) -> SFXPitchResult<()> {
        if !self.config.enable_scale_variations {
            return Err(SFXPitchError::InvalidConfig("Scale variations disabled".to_string()));
        }

        // Calculate pitch multiplier from scale degree
        let pitch_multiplier = self.calculate_scale_pitch_multiplier(scale, degree);
        let pitch_offset = 12.0 * pitch_multiplier.log2();

        // Create scale variation
        let variation = PitchVariation {
            id: format!("scale_{}_{}_{}", sound_id, scale, degree),
            name: format!("{} Scale Degree {}", scale, degree),
            variation_type: PitchVariationType::Custom,
            pitch_multiplier,
            pitch_offset,
            probability: 1.0,
            weight: 1.0,
            enabled: true,
            parameters: HashMap::new(),
        };

        // Apply variation
        self.apply_variation(sound_id, &variation.id)?;

        Ok(())
    }

    /// Add variation preset
    pub fn add_variation_preset(&mut self, name: &str, variations: Vec<PitchVariation>) -> SFXPitchResult<()> {
        self.variation_presets.insert(name.to_string(), variations);
        Ok(())
    }

    /// Remove variation preset
    pub fn remove_variation_preset(&mut self, name: &str) -> SFXPitchResult<()> {
        if !self.variation_presets.contains_key(name) {
            return Err(SFXPitchError::PresetNotFound(name.to_string()));
        }

        self.variation_presets.remove(name);
        Ok(())
    }

    /// Get variation preset
    pub fn get_variation_preset(&self, name: &str) -> Option<&Vec<PitchVariation>> {
        self.variation_presets.get(name)
    }

    /// Apply variation preset
    pub fn apply_variation_preset(&mut self, sound_id: &str, preset_name: &str) -> SFXPitchResult<()> {
        let variations = self.variation_presets.get(preset_name)
            .ok_or_else(|| SFXPitchError::PresetNotFound(preset_name.to_string()))?;

        for variation in variations {
            self.apply_variation(sound_id, &variation.id)?;
        }

        // Emit preset loaded event
        self.emit_event(SFXPitchEvent::PitchVariationPresetLoaded {
            preset_name: preset_name.to_string(),
            variations: variations.clone(),
        });

        Ok(())
    }

    /// Get variation history
    pub fn get_variation_history(&self) -> &Vec<VariationHistoryEntry> {
        &self.variation_history
    }

    /// Clear variation history
    pub fn clear_variation_history(&mut self) {
        self.variation_history.clear();
    }

    /// Update active variations
    fn update_active_variations(&mut self, delta_time: f32) -> SFXPitchResult<()> {
        let mut completed_variations = Vec::new();

        for (id, variation) in self.active_variations.iter_mut() {
            if variation.enabled {
                // Update variation progress
                variation.progress += delta_time / variation.duration;
                variation.progress = variation.progress.min(1.0);

                // Check if variation is completed
                if variation.progress >= 1.0 {
                    completed_variations.push(id.clone());
                }
            }
        }

        // Handle completed variations
        for id in completed_variations {
            self.handle_completed_variation(&id)?;
        }

        Ok(())
    }

    /// Handle completed variation
    fn handle_completed_variation(&mut self, id: &str) -> SFXPitchResult<()> {
        if let Some(variation) = self.active_variations.remove(id) {
            // Create history entry
            let history_entry = VariationHistoryEntry {
                variation_id: variation.variation_id.clone(),
                sound_id: variation.sound_id.clone(),
                variation_type: PitchVariationType::Custom,
                start_time: variation.start_time,
                end_time: variation.start_time + variation.duration,
                duration: variation.duration,
                success: true,
                parameters: variation.parameters,
            };

            self.variation_history.push(history_entry);

            // Keep only recent history
            if self.variation_history.len() > self.config.variation_history_size {
                self.variation_history.remove(0);
            }
        }

        Ok(())
    }

    /// Analyze variations
    fn analyze_variations(&mut self) -> SFXPitchResult<()> {
        // Analyze variation usage patterns
        // In a real implementation, this would analyze variation effectiveness
        Ok(())
    }

    /// Optimize variations
    fn optimize_variations(&mut self) -> SFXPitchResult<()> {
        // Optimize variation parameters
        // In a real implementation, this would optimize variation settings
        Ok(())
    }

    /// Generate random pitch multiplier
    fn generate_random_pitch_multiplier(&self) -> f32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(0.5..2.0)
    }

    /// Generate random pitch offset
    fn generate_random_pitch_offset(&self) -> f32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(-12.0..12.0)
    }

    /// Generate random probability
    fn generate_random_probability(&self) -> f32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0..1.0)
    }

    /// Generate random weight
    fn generate_random_weight(&self) -> f32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0..1.0)
    }

    /// Calculate scale pitch multiplier
    fn calculate_scale_pitch_multiplier(&self, scale: &str, degree: i32) -> f32 {
        // Simple scale calculation
        // In a real implementation, this would use proper scale theory
        match scale {
            "major" => {
                let intervals = [0, 2, 4, 5, 7, 9, 11]; // Major scale intervals
                let interval = intervals[degree as usize % 7];
                2.0_f32.powf(interval as f32 / 12.0)
            },
            "minor" => {
                let intervals = [0, 2, 3, 5, 7, 8, 10]; // Natural minor scale intervals
                let interval = intervals[degree as usize % 7];
                2.0_f32.powf(interval as f32 / 12.0)
            },
            "pentatonic" => {
                let intervals = [0, 2, 4, 7, 9]; // Pentatonic scale intervals
                let interval = intervals[degree as usize % 5];
                2.0_f32.powf(interval as f32 / 12.0)
            },
            _ => 1.0,
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

impl Default for VariationsConfig {
    fn default() -> Self {
        Self {
            enable_variations: true,
            enable_random_variations: true,
            enable_preset_variations: true,
            enable_custom_variations: true,
            enable_semitone_variations: true,
            enable_microtone_variations: true,
            enable_harmonic_variations: true,
            enable_subharmonic_variations: true,
            enable_chromatic_variations: true,
            enable_scale_variations: true,
            max_variations_per_sound: 4,
            variation_probability_threshold: 0.5,
            variation_weight_threshold: 0.3,
            variation_history_size: 100,
            enable_variation_analysis: true,
            enable_variation_optimization: true,
        }
    }
}

impl Default for PitchVariations {
    fn default() -> Self {
        Self::new(VariationsConfig::default())
    }
}
