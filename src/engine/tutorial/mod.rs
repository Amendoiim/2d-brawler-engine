//! Tutorial system
//! 
//! This module provides a comprehensive tutorial system for guiding new players
//! through the game mechanics and features. It includes step-by-step guidance,
//! interactive elements, and progress tracking.

pub mod manager;
pub mod step;
pub mod ui;
pub mod condition;

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// Re-export main components
pub use manager::TutorialManager;
pub use step::{TutorialStep, TutorialStepType, TutorialReward};
pub use ui::{TutorialUI, TutorialOverlay, TutorialHighlight};
pub use condition::TutorialCondition;

/// Tutorial system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialConfig {
    /// Enable tutorial system
    pub enabled: bool,
    /// Auto-start first tutorial
    pub auto_start: bool,
    /// Show tutorial hints
    pub show_hints: bool,
    /// Tutorial completion rewards
    pub enable_rewards: bool,
    /// Skip completed tutorials
    pub skip_completed: bool,
    /// Tutorial timeout (seconds)
    pub timeout: f32,
    /// Visual highlight intensity
    pub highlight_intensity: f32,
    /// Audio cue volume
    pub audio_volume: f32,
}

impl Default for TutorialConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_start: true,
            show_hints: true,
            enable_rewards: true,
            skip_completed: true,
            timeout: 30.0,
            highlight_intensity: 0.8,
            audio_volume: 0.7,
        }
    }
}

/// Tutorial progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialProgress {
    /// Tutorial ID
    pub tutorial_id: String,
    /// Current step index
    pub current_step: usize,
    /// Steps completed
    pub completed_steps: HashSet<usize>,
    /// Start time
    pub start_time: f32,
    /// Last update time
    pub last_update: f32,
    /// Completion status
    pub completed: bool,
    /// Skipped status
    pub skipped: bool,
    /// Progress percentage (0.0 to 1.0)
    pub progress: f32,
}

impl TutorialProgress {
    /// Create new tutorial progress
    pub fn new(tutorial_id: String) -> Self {
        Self {
            tutorial_id,
            current_step: 0,
            completed_steps: HashSet::new(),
            start_time: 0.0,
            last_update: 0.0,
            completed: false,
            skipped: false,
            progress: 0.0,
        }
    }

    /// Update progress
    pub fn update(&mut self, current_time: f32, total_steps: usize) {
        self.last_update = current_time;
        self.progress = self.completed_steps.len() as f32 / total_steps as f32;
        self.completed = self.completed_steps.len() == total_steps;
    }

    /// Complete a step
    pub fn complete_step(&mut self, step_index: usize) {
        self.completed_steps.insert(step_index);
    }

    /// Skip tutorial
    pub fn skip(&mut self) {
        self.skipped = true;
        self.completed = true;
    }
}

/// Tutorial settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialSettings {
    /// Master enable/disable
    pub enabled: bool,
    /// Show tutorial UI
    pub show_ui: bool,
    /// Show visual highlights
    pub show_highlights: bool,
    /// Play audio cues
    pub play_audio: bool,
    /// Show progress indicators
    pub show_progress: bool,
    /// Allow skipping
    pub allow_skip: bool,
    /// Auto-advance steps
    pub auto_advance: bool,
    /// Step timeout
    pub step_timeout: f32,
    /// Highlight duration
    pub highlight_duration: f32,
    /// Fade in/out duration
    pub fade_duration: f32,
}

impl Default for TutorialSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            show_ui: true,
            show_highlights: true,
            play_audio: true,
            show_progress: true,
            allow_skip: true,
            auto_advance: false,
            step_timeout: 30.0,
            highlight_duration: 2.0,
            fade_duration: 0.5,
        }
    }
}

/// Tutorial statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TutorialStats {
    /// Total tutorials completed
    pub tutorials_completed: u32,
    /// Total steps completed
    pub steps_completed: u32,
    /// Total time spent in tutorials
    pub total_time: f32,
    /// Average completion time per tutorial
    pub average_completion_time: f32,
    /// Tutorials skipped
    pub tutorials_skipped: u32,
    /// Most recent tutorial
    pub last_tutorial: Option<String>,
    /// Completion rate
    pub completion_rate: f32,
}

impl TutorialStats {
    /// Update statistics
    pub fn update(&mut self, tutorial_id: &str, completion_time: f32, skipped: bool) {
        if skipped {
            self.tutorials_skipped += 1;
        } else {
            self.tutorials_completed += 1;
        }
        
        self.total_time += completion_time;
        self.average_completion_time = self.total_time / (self.tutorials_completed + self.tutorials_skipped) as f32;
        self.last_tutorial = Some(tutorial_id.to_string());
        
        let total_attempts = self.tutorials_completed + self.tutorials_skipped;
        if total_attempts > 0 {
            self.completion_rate = self.tutorials_completed as f32 / total_attempts as f32;
        }
    }
}

/// Tutorial event types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TutorialEvent {
    /// Tutorial started
    Started(String),
    /// Tutorial completed
    Completed(String),
    /// Tutorial skipped
    Skipped(String),
    /// Step completed
    StepCompleted(String, usize),
    /// Step started
    StepStarted(String, usize),
    /// Tutorial paused
    Paused(String),
    /// Tutorial resumed
    Resumed(String),
    /// Tutorial failed
    Failed(String, String),
}

/// Tutorial error types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TutorialError {
    /// Tutorial not found
    TutorialNotFound(String),
    /// Step not found
    StepNotFound(String, usize),
    /// Invalid condition
    InvalidCondition(String),
    /// Tutorial already active
    AlreadyActive(String),
    /// Tutorial not active
    NotActive,
    /// Prerequisites not met
    PrerequisitesNotMet(Vec<String>),
    /// Configuration error
    ConfigError(String),
    /// UI error
    UIError(String),
    /// Audio error
    AudioError(String),
}

impl std::fmt::Display for TutorialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TutorialError::TutorialNotFound(id) => write!(f, "Tutorial not found: {}", id),
            TutorialError::StepNotFound(tutorial_id, step) => write!(f, "Step {} not found in tutorial {}", step, tutorial_id),
            TutorialError::InvalidCondition(condition) => write!(f, "Invalid condition: {}", condition),
            TutorialError::AlreadyActive(id) => write!(f, "Tutorial already active: {}", id),
            TutorialError::NotActive => write!(f, "No tutorial is currently active"),
            TutorialError::PrerequisitesNotMet(prereqs) => write!(f, "Prerequisites not met: {:?}", prereqs),
            TutorialError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            TutorialError::UIError(msg) => write!(f, "UI error: {}", msg),
            TutorialError::AudioError(msg) => write!(f, "Audio error: {}", msg),
        }
    }
}

impl std::error::Error for TutorialError {}

/// Result type for tutorial operations
pub type TutorialResult<T> = Result<T, TutorialError>;
