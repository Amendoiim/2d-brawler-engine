//! Music States
//! 
//! This module provides dynamic music state management with automatic state transitions
//! based on gameplay events, analysis data, and user preferences.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{MusicResult, MusicError, MusicEvent, MusicStateType, MusicStemType};

/// Music states manager
pub struct MusicStates {
    /// State configuration
    pub config: StateConfig,
    /// Current state
    pub current_state: MusicStateType,
    /// Previous state
    pub previous_state: MusicStateType,
    /// State definitions
    pub state_definitions: HashMap<MusicStateType, MusicStateDefinition>,
    /// State transitions
    pub state_transitions: HashMap<(MusicStateType, MusicStateType), StateTransition>,
    /// State history
    pub state_history: Vec<StateHistoryEntry>,
    /// State timers
    pub state_timers: HashMap<MusicStateType, f32>,
    /// State conditions
    pub state_conditions: HashMap<MusicStateType, Vec<StateCondition>>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&MusicEvent) + Send + Sync>>,
}

/// State configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    /// Enable automatic state transitions
    pub enable_automatic_transitions: bool,
    /// State transition delay (seconds)
    pub state_transition_delay: f32,
    /// State analysis window (seconds)
    pub state_analysis_window: f32,
    /// State condition threshold
    pub state_condition_threshold: f32,
    /// State priority weight
    pub state_priority_weight: f32,
    /// State energy weight
    pub state_energy_weight: f32,
    /// State mood weight
    pub state_mood_weight: f32,
    /// State BPM weight
    pub state_bpm_weight: f32,
    /// State key weight
    pub state_key_weight: f32,
    /// Enable state blending
    pub enable_state_blending: bool,
    /// State blend time (seconds)
    pub state_blend_time: f32,
    /// Enable state effects
    pub enable_state_effects: bool,
    /// Enable state analysis
    pub enable_state_analysis: bool,
}

/// Music state definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicStateDefinition {
    /// State type
    pub state_type: MusicStateType,
    /// State name
    pub name: String,
    /// State description
    pub description: String,
    /// State priority (0-100)
    pub priority: u8,
    /// State energy range (0.0 to 1.0)
    pub energy_range: (f32, f32),
    /// State mood preferences
    pub mood_preferences: Vec<String>,
    /// State BPM range
    pub bpm_range: (f32, f32),
    /// State key preferences
    pub key_preferences: Vec<String>,
    /// State stem configuration
    pub stem_configuration: HashMap<MusicStemType, StemStateConfig>,
    /// State effects
    pub state_effects: Vec<StateEffect>,
    /// State transitions
    pub allowed_transitions: Vec<MusicStateType>,
    /// State duration (seconds, 0 = infinite)
    pub duration: f32,
    /// State cooldown (seconds)
    pub cooldown: f32,
    /// State conditions
    pub conditions: Vec<StateCondition>,
    /// State parameters
    pub parameters: HashMap<String, f32>,
}

/// Stem state configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StemStateConfig {
    /// Stem type
    pub stem_type: MusicStemType,
    /// Volume (0.0 to 1.0)
    pub volume: f32,
    /// Pan (-1.0 to 1.0)
    pub pan: f32,
    /// Pitch (0.1 to 4.0)
    pub pitch: f32,
    /// Playback speed (0.25 to 4.0)
    pub playback_speed: f32,
    /// Is enabled
    pub enabled: bool,
    /// Is muted
    pub muted: bool,
    /// Is looping
    pub looping: bool,
    /// Fade in time (seconds)
    pub fade_in_time: f32,
    /// Fade out time (seconds)
    pub fade_out_time: f32,
    /// Priority (0-100)
    pub priority: u8,
    /// Layer (0-10)
    pub layer: u8,
    /// Effects enabled
    pub effects_enabled: bool,
    /// Spatial audio enabled
    pub spatial_audio_enabled: bool,
    /// 3D position
    pub position_3d: (f32, f32, f32),
    /// 3D direction
    pub direction_3d: (f32, f32, f32),
    /// Rolloff factor
    pub rolloff_factor: f32,
    /// Reference distance
    pub reference_distance: f32,
    /// Maximum distance
    pub max_distance: f32,
}

/// State transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    /// Source state
    pub source_state: MusicStateType,
    /// Target state
    pub target_state: MusicStateType,
    /// Transition type
    pub transition_type: StateTransitionType,
    /// Transition duration (seconds)
    pub duration: f32,
    /// Transition conditions
    pub conditions: Vec<StateCondition>,
    /// Transition probability (0.0 to 1.0)
    pub probability: f32,
    /// Transition priority (0-100)
    pub priority: u8,
    /// Transition effects
    pub effects: Vec<StateEffect>,
    /// Transition parameters
    pub parameters: HashMap<String, f32>,
}

/// State transition type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StateTransitionType {
    /// Immediate
    Immediate,
    /// Fade
    Fade,
    /// Crossfade
    Crossfade,
    /// Beat synchronized
    BeatSync,
    /// Phrase synchronized
    PhraseSync,
    /// Measure synchronized
    MeasureSync,
    /// Custom
    Custom(String),
}

/// State condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateCondition {
    /// Condition type
    pub condition_type: StateConditionType,
    /// Condition parameter
    pub parameter: String,
    /// Condition operator
    pub operator: StateConditionOperator,
    /// Condition value
    pub value: f32,
    /// Condition weight (0.0 to 1.0)
    pub weight: f32,
    /// Condition enabled
    pub enabled: bool,
}

/// State condition type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StateConditionType {
    /// Energy level
    Energy,
    /// Mood
    Mood,
    /// BPM
    BPM,
    /// Key
    Key,
    /// Gameplay event
    GameplayEvent,
    /// Time
    Time,
    /// Random
    Random,
    /// Custom
    Custom(String),
}

/// State condition operator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StateConditionOperator {
    /// Equal
    Equal,
    /// Not equal
    NotEqual,
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
    /// Greater than or equal
    GreaterThanOrEqual,
    /// Less than or equal
    LessThanOrEqual,
    /// Contains
    Contains,
    /// Not contains
    NotContains,
    /// In range
    InRange,
    /// Not in range
    NotInRange,
}

/// State effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateEffect {
    /// Effect type
    pub effect_type: StateEffectType,
    /// Effect parameters
    pub parameters: HashMap<String, f32>,
    /// Effect enabled
    pub enabled: bool,
    /// Effect intensity (0.0 to 1.0)
    pub intensity: f32,
    /// Effect duration (seconds, 0 = permanent)
    pub duration: f32,
    /// Effect start time (seconds)
    pub start_time: f32,
    /// Effect end time (seconds)
    pub end_time: f32,
}

/// State effect type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StateEffectType {
    /// Volume change
    VolumeChange,
    /// Pan change
    PanChange,
    /// Pitch change
    PitchChange,
    /// Speed change
    SpeedChange,
    /// Filter change
    FilterChange,
    /// EQ change
    EQChange,
    /// Reverb change
    ReverbChange,
    /// Delay change
    DelayChange,
    /// Chorus change
    ChorusChange,
    /// Flanger change
    FlangerChange,
    /// Phaser change
    PhaserChange,
    /// Distortion change
    DistortionChange,
    /// Compressor change
    CompressorChange,
    /// Limiter change
    LimiterChange,
    /// Gate change
    GateChange,
    /// Expander change
    ExpanderChange,
    /// Pitch shifter change
    PitchShifterChange,
    /// Harmonizer change
    HarmonizerChange,
    /// Vocoder change
    VocoderChange,
    /// Bit crusher change
    BitCrusherChange,
    /// Sample rate reducer change
    SampleRateReducerChange,
    /// Ring modulator change
    RingModulatorChange,
    /// Frequency shifter change
    FrequencyShifterChange,
    /// Stereo widener change
    StereoWidenerChange,
    /// Mono maker change
    MonoMakerChange,
    /// Stereo imager change
    StereoImagerChange,
    /// Spatial processor change
    SpatialProcessorChange,
    /// Convolution change
    ConvolutionChange,
    /// Impulse response change
    ImpulseResponseChange,
    /// Custom effect change
    CustomEffectChange(String),
}

/// State history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateHistoryEntry {
    /// State type
    pub state_type: MusicStateType,
    /// Start time
    pub start_time: f32,
    /// End time
    pub end_time: f32,
    /// Duration
    pub duration: f32,
    /// Transition type
    pub transition_type: StateTransitionType,
    /// Success
    pub success: bool,
    /// Error message
    pub error_message: Option<String>,
}

impl MusicStates {
    /// Create new music states manager
    pub fn new(config: StateConfig) -> Self {
        let mut manager = Self {
            config,
            current_state: MusicStateType::Calm,
            previous_state: MusicStateType::Calm,
            state_definitions: HashMap::new(),
            state_transitions: HashMap::new(),
            state_history: Vec::new(),
            state_timers: HashMap::new(),
            state_conditions: HashMap::new(),
            event_handlers: Vec::new(),
        };

        // Initialize default states
        manager.initialize_default_states();
        manager
    }

    /// Update music states
    pub fn update(&mut self, delta_time: f32, analysis_data: &AnalysisData) -> MusicResult<()> {
        // Update state timers
        self.update_state_timers(delta_time)?;
        
        // Check for automatic state transitions
        if self.config.enable_automatic_transitions {
            self.check_automatic_transitions(analysis_data)?;
        }
        
        // Update state effects
        self.update_state_effects(delta_time)?;
        
        // Update state analysis
        if self.config.enable_state_analysis {
            self.update_state_analysis(analysis_data)?;
        }
        
        Ok(())
    }

    /// Set current state
    pub fn set_current_state(&mut self, state: MusicStateType) -> MusicResult<()> {
        if !self.state_definitions.contains_key(&state) {
            return Err(MusicError::InvalidMusicState(format!("{:?}", state)));
        }
        
        let old_state = self.current_state.clone();
        self.previous_state = old_state.clone();
        self.current_state = state.clone();
        
        // Update state timer
        self.state_timers.insert(state.clone(), 0.0);
        
        // Create state history entry
        let history_entry = StateHistoryEntry {
            state_type: state.clone(),
            start_time: 0.0, // Will be set by caller
            end_time: 0.0,
            duration: 0.0,
            transition_type: StateTransitionType::Immediate,
            success: true,
            error_message: None,
        };
        
        self.state_history.push(history_entry);
        
        // Emit state changed event
        self.emit_event(MusicEvent::StateChanged {
            old_state,
            new_state: state,
        });
        
        Ok(())
    }

    /// Get current state
    pub fn get_current_state(&self) -> &MusicStateType {
        &self.current_state
    }

    /// Get previous state
    pub fn get_previous_state(&self) -> &MusicStateType {
        &self.previous_state
    }

    /// Get state definition
    pub fn get_state_definition(&self, state: &MusicStateType) -> Option<&MusicStateDefinition> {
        self.state_definitions.get(state)
    }

    /// Get state definition mutably
    pub fn get_state_definition_mut(&mut self, state: &MusicStateType) -> Option<&mut MusicStateDefinition> {
        self.state_definitions.get_mut(state)
    }

    /// Add state definition
    pub fn add_state_definition(&mut self, definition: MusicStateDefinition) -> MusicResult<()> {
        let state_type = definition.state_type.clone();
        self.state_definitions.insert(state_type, definition);
        Ok(())
    }

    /// Remove state definition
    pub fn remove_state_definition(&mut self, state: &MusicStateType) -> MusicResult<()> {
        if !self.state_definitions.contains_key(state) {
            return Err(MusicError::InvalidMusicState(format!("{:?}", state)));
        }
        
        self.state_definitions.remove(state);
        Ok(())
    }

    /// Add state transition
    pub fn add_state_transition(&mut self, transition: StateTransition) -> MusicResult<()> {
        let key = (transition.source_state.clone(), transition.target_state.clone());
        self.state_transitions.insert(key, transition);
        Ok(())
    }

    /// Remove state transition
    pub fn remove_state_transition(&mut self, source: &MusicStateType, target: &MusicStateType) -> MusicResult<()> {
        let key = (source.clone(), target.clone());
        if !self.state_transitions.contains_key(&key) {
            return Err(MusicError::InvalidConfig("State transition not found".to_string()));
        }
        
        self.state_transitions.remove(&key);
        Ok(())
    }

    /// Get state transitions
    pub fn get_state_transitions(&self, source: &MusicStateType) -> Vec<&StateTransition> {
        self.state_transitions.iter()
            .filter(|((s, _), _)| s == source)
            .map(|(_, transition)| transition)
            .collect()
    }

    /// Get state history
    pub fn get_state_history(&self) -> &Vec<StateHistoryEntry> {
        &self.state_history
    }

    /// Clear state history
    pub fn clear_state_history(&mut self) {
        self.state_history.clear();
    }

    /// Update state timers
    fn update_state_timers(&mut self, delta_time: f32) -> MusicResult<()> {
        for (state, timer) in self.state_timers.iter_mut() {
            *timer += delta_time;
            
            // Check if state duration has been exceeded
            if let Some(definition) = self.state_definitions.get(state) {
                if definition.duration > 0.0 && *timer >= definition.duration {
                    // State duration exceeded, transition to default state
                    self.set_current_state(MusicStateType::Calm)?;
                }
            }
        }
        
        Ok(())
    }

    /// Check for automatic state transitions
    fn check_automatic_transitions(&mut self, analysis_data: &AnalysisData) -> MusicResult<()> {
        let current_state = self.current_state.clone();
        let transitions = self.get_state_transitions(&current_state);
        
        for transition in transitions {
            if self.evaluate_transition_conditions(transition, analysis_data)? {
                // Transition conditions met, perform transition
                self.perform_state_transition(transition)?;
                break;
            }
        }
        
        Ok(())
    }

    /// Evaluate transition conditions
    fn evaluate_transition_conditions(&self, transition: &StateTransition, analysis_data: &AnalysisData) -> MusicResult<bool> {
        let mut total_weight = 0.0;
        let mut satisfied_conditions = 0.0;
        
        for condition in &transition.conditions {
            if condition.enabled {
                let condition_satisfied = self.evaluate_condition(condition, analysis_data)?;
                if condition_satisfied {
                    satisfied_conditions += condition.weight;
                }
                total_weight += condition.weight;
            }
        }
        
        if total_weight > 0.0 {
            let satisfaction_ratio = satisfied_conditions / total_weight;
            Ok(satisfaction_ratio >= self.config.state_condition_threshold)
        } else {
            Ok(false)
        }
    }

    /// Evaluate condition
    fn evaluate_condition(&self, condition: &StateCondition, analysis_data: &AnalysisData) -> MusicResult<bool> {
        let value = match condition.condition_type {
            StateConditionType::Energy => analysis_data.energy,
            StateConditionType::BPM => analysis_data.bpm,
            StateConditionType::Mood => {
                // Simple mood evaluation based on energy and spectral features
                if analysis_data.energy > 0.7 {
                    1.0
                } else if analysis_data.energy > 0.4 {
                    0.5
                } else {
                    0.0
                }
            },
            StateConditionType::Key => {
                // Simple key evaluation based on key confidence
                analysis_data.key_confidence
            },
            StateConditionType::GameplayEvent => {
                // This would be set by gameplay events
                0.0
            },
            StateConditionType::Time => {
                // Current time in seconds
                0.0
            },
            StateConditionType::Random => {
                // Random value between 0.0 and 1.0
                rand::random::<f32>()
            },
            StateConditionType::Custom(_) => {
                // Custom condition evaluation
                0.0
            },
        };
        
        let result = match condition.operator {
            StateConditionOperator::Equal => (value - condition.value).abs() < 0.01,
            StateConditionOperator::NotEqual => (value - condition.value).abs() >= 0.01,
            StateConditionOperator::GreaterThan => value > condition.value,
            StateConditionOperator::LessThan => value < condition.value,
            StateConditionOperator::GreaterThanOrEqual => value >= condition.value,
            StateConditionOperator::LessThanOrEqual => value <= condition.value,
            StateConditionOperator::Contains => {
                // String contains check
                false
            },
            StateConditionOperator::NotContains => {
                // String not contains check
                true
            },
            StateConditionOperator::InRange => {
                // Range check
                false
            },
            StateConditionOperator::NotInRange => {
                // Not in range check
                true
            },
        };
        
        Ok(result)
    }

    /// Perform state transition
    fn perform_state_transition(&mut self, transition: &StateTransition) -> MusicResult<()> {
        // Check if transition is allowed
        if !self.state_definitions.contains_key(&transition.target_state) {
            return Err(MusicError::InvalidMusicState(format!("{:?}", transition.target_state)));
        }
        
        // Perform transition
        self.set_current_state(transition.target_state.clone())?;
        
        Ok(())
    }

    /// Update state effects
    fn update_state_effects(&mut self, delta_time: f32) -> MusicResult<()> {
        if let Some(definition) = self.state_definitions.get(&self.current_state) {
            for effect in &definition.state_effects {
                if effect.enabled {
                    // Update effect based on time
                    self.update_state_effect(effect, delta_time)?;
                }
            }
        }
        
        Ok(())
    }

    /// Update state effect
    fn update_state_effect(&self, effect: &StateEffect, delta_time: f32) -> MusicResult<()> {
        // In a real implementation, this would update the effect parameters
        // For now, we'll just simulate the effect update
        
        Ok(())
    }

    /// Update state analysis
    fn update_state_analysis(&mut self, analysis_data: &AnalysisData) -> MusicResult<()> {
        // Analyze current state performance
        if let Some(definition) = self.state_definitions.get(&self.current_state) {
            // Check if current analysis data matches state preferences
            let energy_match = analysis_data.energy >= definition.energy_range.0 && 
                              analysis_data.energy <= definition.energy_range.1;
            let bpm_match = analysis_data.bpm >= definition.bpm_range.0 && 
                           analysis_data.bpm <= definition.bpm_range.1;
            
            // Update state parameters based on analysis
            if let Some(definition) = self.state_definitions.get_mut(&self.current_state) {
                definition.parameters.insert("energy_match".to_string(), if energy_match { 1.0 } else { 0.0 });
                definition.parameters.insert("bpm_match".to_string(), if bpm_match { 1.0 } else { 0.0 });
            }
        }
        
        Ok(())
    }

    /// Initialize default states
    fn initialize_default_states(&mut self) {
        // Calm state
        let calm_state = MusicStateDefinition {
            state_type: MusicStateType::Calm,
            name: "Calm".to_string(),
            description: "Peaceful and relaxed state".to_string(),
            priority: 10,
            energy_range: (0.0, 0.3),
            mood_preferences: vec!["Calm".to_string(), "Peaceful".to_string()],
            bpm_range: (60.0, 100.0),
            key_preferences: vec!["C Major".to_string(), "A Minor".to_string()],
            stem_configuration: HashMap::new(),
            state_effects: Vec::new(),
            allowed_transitions: vec![MusicStateType::Tension, MusicStateType::Exploration],
            duration: 0.0,
            cooldown: 0.0,
            conditions: Vec::new(),
            parameters: HashMap::new(),
        };
        self.state_definitions.insert(MusicStateType::Calm, calm_state);
        
        // Tension state
        let tension_state = MusicStateDefinition {
            state_type: MusicStateType::Tension,
            name: "Tension".to_string(),
            description: "Building tension and suspense".to_string(),
            priority: 30,
            energy_range: (0.3, 0.6),
            mood_preferences: vec!["Tense".to_string(), "Suspenseful".to_string()],
            bpm_range: (100.0, 140.0),
            key_preferences: vec!["D Minor".to_string(), "F Minor".to_string()],
            stem_configuration: HashMap::new(),
            state_effects: Vec::new(),
            allowed_transitions: vec![MusicStateType::Calm, MusicStateType::Combat],
            duration: 0.0,
            cooldown: 0.0,
            conditions: Vec::new(),
            parameters: HashMap::new(),
        };
        self.state_definitions.insert(MusicStateType::Tension, tension_state);
        
        // Combat state
        let combat_state = MusicStateDefinition {
            state_type: MusicStateType::Combat,
            name: "Combat".to_string(),
            description: "Active combat and action".to_string(),
            priority: 80,
            energy_range: (0.6, 0.9),
            mood_preferences: vec!["Energetic".to_string(), "Intense".to_string()],
            bpm_range: (140.0, 180.0),
            key_preferences: vec!["E Minor".to_string(), "G Major".to_string()],
            stem_configuration: HashMap::new(),
            state_effects: Vec::new(),
            allowed_transitions: vec![MusicStateType::Tension, MusicStateType::Intense, MusicStateType::Victory],
            duration: 0.0,
            cooldown: 0.0,
            conditions: Vec::new(),
            parameters: HashMap::new(),
        };
        self.state_definitions.insert(MusicStateType::Combat, combat_state);
        
        // Intense state
        let intense_state = MusicStateDefinition {
            state_type: MusicStateType::Intense,
            name: "Intense".to_string(),
            description: "High intensity action".to_string(),
            priority: 90,
            energy_range: (0.8, 1.0),
            mood_preferences: vec!["Intense".to_string(), "Energetic".to_string()],
            bpm_range: (160.0, 200.0),
            key_preferences: vec!["A Minor".to_string(), "C Major".to_string()],
            stem_configuration: HashMap::new(),
            state_effects: Vec::new(),
            allowed_transitions: vec![MusicStateType::Combat, MusicStateType::Boss],
            duration: 0.0,
            cooldown: 0.0,
            conditions: Vec::new(),
            parameters: HashMap::new(),
        };
        self.state_definitions.insert(MusicStateType::Intense, intense_state);
        
        // Boss state
        let boss_state = MusicStateDefinition {
            state_type: MusicStateType::Boss,
            name: "Boss".to_string(),
            description: "Boss battle music".to_string(),
            priority: 100,
            energy_range: (0.9, 1.0),
            mood_preferences: vec!["Epic".to_string(), "Dramatic".to_string()],
            bpm_range: (120.0, 160.0),
            key_preferences: vec!["D Minor".to_string(), "B Minor".to_string()],
            stem_configuration: HashMap::new(),
            state_effects: Vec::new(),
            allowed_transitions: vec![MusicStateType::Victory, MusicStateType::Defeat],
            duration: 0.0,
            cooldown: 0.0,
            conditions: Vec::new(),
            parameters: HashMap::new(),
        };
        self.state_definitions.insert(MusicStateType::Boss, boss_state);
        
        // Victory state
        let victory_state = MusicStateDefinition {
            state_type: MusicStateType::Victory,
            name: "Victory".to_string(),
            description: "Victory celebration music".to_string(),
            priority: 70,
            energy_range: (0.7, 1.0),
            mood_preferences: vec!["Triumphant".to_string(), "Celebratory".to_string()],
            bpm_range: (100.0, 140.0),
            key_preferences: vec!["C Major".to_string(), "G Major".to_string()],
            stem_configuration: HashMap::new(),
            state_effects: Vec::new(),
            allowed_transitions: vec![MusicStateType::Calm, MusicStateType::Menu],
            duration: 10.0,
            cooldown: 5.0,
            conditions: Vec::new(),
            parameters: HashMap::new(),
        };
        self.state_definitions.insert(MusicStateType::Victory, victory_state);
        
        // Defeat state
        let defeat_state = MusicStateDefinition {
            state_type: MusicStateType::Defeat,
            name: "Defeat".to_string(),
            description: "Defeat/game over music".to_string(),
            priority: 60,
            energy_range: (0.0, 0.4),
            mood_preferences: vec!["Sad".to_string(), "Melancholic".to_string()],
            bpm_range: (60.0, 100.0),
            key_preferences: vec!["A Minor".to_string(), "D Minor".to_string()],
            stem_configuration: HashMap::new(),
            state_effects: Vec::new(),
            allowed_transitions: vec![MusicStateType::Calm, MusicStateType::Menu],
            duration: 8.0,
            cooldown: 3.0,
            conditions: Vec::new(),
            parameters: HashMap::new(),
        };
        self.state_definitions.insert(MusicStateType::Defeat, defeat_state);
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

impl Default for StateConfig {
    fn default() -> Self {
        Self {
            enable_automatic_transitions: true,
            state_transition_delay: 0.5,
            state_analysis_window: 2.0,
            state_condition_threshold: 0.7,
            state_priority_weight: 0.3,
            state_energy_weight: 0.4,
            state_mood_weight: 0.2,
            state_bpm_weight: 0.1,
            state_key_weight: 0.0,
            enable_state_blending: true,
            state_blend_time: 1.0,
            enable_state_effects: true,
            enable_state_analysis: true,
        }
    }
}

impl Default for MusicStates {
    fn default() -> Self {
        Self::new(StateConfig::default())
    }
}

// Import AnalysisData from music_analyzer module
use crate::engine::dynamic_music::music_analyzer::AnalysisData;
