//! Game Completion System
//! 
//! This module provides comprehensive game completion tracking, end game content,
//! credits system, and New Game+ features for the 2D brawler game.

pub mod completion_tracker;
pub mod end_game;
pub mod credits;
pub mod new_game_plus;
pub mod completion_rewards;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Re-export main components
pub use completion_tracker::CompletionTracker;
pub use end_game::EndGameManager;
pub use credits::CreditsManager;
pub use new_game_plus::NewGamePlusManager;
pub use completion_rewards::CompletionRewardManager;

/// Game completion configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionConfig {
    /// Enable completion tracking
    pub tracking_enabled: bool,
    /// Enable end game content
    pub end_game_enabled: bool,
    /// Enable credits system
    pub credits_enabled: bool,
    /// Enable New Game+ features
    pub new_game_plus_enabled: bool,
    /// Completion percentage threshold for end game
    pub end_game_threshold: f32,
    /// Maximum completion percentage
    pub max_completion: f32,
    /// Enable completion rewards
    pub rewards_enabled: bool,
}

impl Default for CompletionConfig {
    fn default() -> Self {
        Self {
            tracking_enabled: true,
            end_game_enabled: true,
            credits_enabled: true,
            new_game_plus_enabled: true,
            end_game_threshold: 80.0,
            max_completion: 100.0,
            rewards_enabled: true,
        }
    }
}

/// Game completion events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionEvent {
    /// Game completed
    GameCompleted { completion_percentage: f32, completion_time: SystemTime },
    /// End game unlocked
    EndGameUnlocked { completion_percentage: f32 },
    /// Credits started
    CreditsStarted,
    /// Credits completed
    CreditsCompleted,
    /// New Game+ unlocked
    NewGamePlusUnlocked,
    /// Completion milestone reached
    MilestoneReached { milestone: CompletionMilestone, percentage: f32 },
    /// Completion reward earned
    RewardEarned { reward_id: String, reward_type: String },
}

/// Completion milestones
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompletionMilestone {
    /// 25% completion
    Quarter,
    /// 50% completion
    Half,
    /// 75% completion
    ThreeQuarters,
    /// 90% completion
    NearComplete,
    /// 100% completion
    Complete,
    /// All achievements unlocked
    AllAchievements,
    /// All levels completed
    AllLevels,
    /// All secrets found
    AllSecrets,
    /// Perfect completion
    Perfect,
}

/// Completion statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionStats {
    /// Overall completion percentage
    pub overall_completion: f32,
    /// Story completion percentage
    pub story_completion: f32,
    /// Side quest completion percentage
    pub side_quest_completion: f32,
    /// Achievement completion percentage
    pub achievement_completion: f32,
    /// Secret completion percentage
    pub secret_completion: f32,
    /// Level completion percentage
    pub level_completion: f32,
    /// Total play time in seconds
    pub total_play_time: f32,
    /// Completion time
    pub completion_time: Option<SystemTime>,
    /// Number of deaths
    pub death_count: u32,
    /// Number of saves
    pub save_count: u32,
    /// Number of loads
    pub load_count: u32,
    /// Reached milestones
    pub reached_milestones: Vec<CompletionMilestone>,
    /// Completion rewards earned
    pub rewards_earned: Vec<String>,
}

impl Default for CompletionStats {
    fn default() -> Self {
        Self {
            overall_completion: 0.0,
            story_completion: 0.0,
            side_quest_completion: 0.0,
            achievement_completion: 0.0,
            secret_completion: 0.0,
            level_completion: 0.0,
            total_play_time: 0.0,
            completion_time: None,
            death_count: 0,
            save_count: 0,
            load_count: 0,
            reached_milestones: Vec::new(),
            rewards_earned: Vec::new(),
        }
    }
}

/// Completion result type
pub type CompletionResult<T> = Result<T, CompletionError>;

/// Completion error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionError {
    /// Completion tracking disabled
    TrackingDisabled,
    /// Invalid completion percentage
    InvalidPercentage(f32),
    /// Milestone already reached
    MilestoneAlreadyReached(CompletionMilestone),
    /// End game not unlocked
    EndGameNotUnlocked,
    /// Credits not available
    CreditsNotAvailable,
    /// New Game+ not unlocked
    NewGamePlusNotUnlocked,
    /// Reward not found
    RewardNotFound(String),
    /// Completion data corrupted
    DataCorrupted,
    /// I/O error
    IoError(String),
    /// Unknown error
    Unknown(String),
}

impl std::fmt::Display for CompletionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompletionError::TrackingDisabled => write!(f, "Completion tracking is disabled"),
            CompletionError::InvalidPercentage(pct) => write!(f, "Invalid completion percentage: {}", pct),
            CompletionError::MilestoneAlreadyReached(milestone) => write!(f, "Milestone already reached: {:?}", milestone),
            CompletionError::EndGameNotUnlocked => write!(f, "End game content not unlocked"),
            CompletionError::CreditsNotAvailable => write!(f, "Credits not available"),
            CompletionError::NewGamePlusNotUnlocked => write!(f, "New Game+ not unlocked"),
            CompletionError::RewardNotFound(id) => write!(f, "Reward not found: {}", id),
            CompletionError::DataCorrupted => write!(f, "Completion data is corrupted"),
            CompletionError::IoError(msg) => write!(f, "I/O error: {}", msg),
            CompletionError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for CompletionError {}
