//! Achievement system
//! 
//! This module provides a comprehensive achievement system for tracking player
//! progress and unlocking rewards based on various game activities.

pub mod manager;
pub mod achievement;
pub mod progress;
pub mod category;
pub mod reward;

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// Re-export main components
pub use manager::AchievementManager;
pub use progress::{OverallProgress, ProgressStats};
pub use achievement::{Achievement, AchievementRequirement, AchievementReward};
pub use progress::AchievementProgress;
pub use category::{AchievementCategory, AchievementDifficulty};
pub use reward::{AchievementRewardType, AchievementRewardData};

/// Achievement system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementConfig {
    /// Enable achievement system
    pub enabled: bool,
    /// Show achievement notifications
    pub show_notifications: bool,
    /// Play achievement sounds
    pub play_sounds: bool,
    /// Auto-save progress
    pub auto_save: bool,
    /// Notification duration (seconds)
    pub notification_duration: f32,
    /// Sound volume
    pub sound_volume: f32,
    /// Notification position
    pub notification_position: (f32, f32),
    /// Notification size
    pub notification_size: (f32, f32),
}

impl Default for AchievementConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            show_notifications: true,
            play_sounds: true,
            auto_save: true,
            notification_duration: 3.0,
            sound_volume: 0.8,
            notification_position: (10.0, 10.0),
            notification_size: (300.0, 100.0),
        }
    }
}

/// Achievement statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AchievementStats {
    /// Total achievements unlocked
    pub total_unlocked: u32,
    /// Total achievements available
    pub total_available: u32,
    /// Completion percentage
    pub completion_percentage: f32,
    /// Total points earned
    pub total_points: u32,
    /// Achievements by category
    pub by_category: HashMap<AchievementCategory, u32>,
    /// Achievements by difficulty
    pub by_difficulty: HashMap<AchievementDifficulty, u32>,
    /// Recent achievements (last 10)
    pub recent_achievements: Vec<String>,
    /// Longest streak
    pub longest_streak: u32,
    /// Current streak
    pub current_streak: u32,
    /// Average time to unlock
    pub average_unlock_time: f32,
}

impl AchievementStats {
    /// Update statistics
    pub fn update(&mut self, achievement: &Achievement, unlock_time: f32) {
        self.total_unlocked += 1;
        self.completion_percentage = (self.total_unlocked as f32 / self.total_available as f32) * 100.0;
        self.total_points += achievement.points;
        
        // Update category count
        *self.by_category.entry(achievement.category.clone()).or_insert(0) += 1;
        
        // Update difficulty count
        *self.by_difficulty.entry(achievement.difficulty.clone()).or_insert(0) += 1;
        
        // Add to recent achievements
        self.recent_achievements.insert(0, achievement.id.clone());
        if self.recent_achievements.len() > 10 {
            self.recent_achievements.pop();
        }
        
        // Update streak
        self.current_streak += 1;
        if self.current_streak > self.longest_streak {
            self.longest_streak = self.current_streak;
        }
        
        // Update average unlock time
        self.average_unlock_time = (self.average_unlock_time * (self.total_unlocked - 1) as f32 + unlock_time) / self.total_unlocked as f32;
    }
    
    /// Reset current streak
    pub fn reset_streak(&mut self) {
        self.current_streak = 0;
    }
}

/// Achievement event types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementEvent {
    /// Achievement unlocked
    Unlocked(String),
    /// Achievement progress updated
    ProgressUpdated(String, u32),
    /// Achievement requirement completed
    RequirementCompleted(String, String),
    /// Achievement category completed
    CategoryCompleted(AchievementCategory),
    /// All achievements completed
    AllCompleted,
    /// Achievement system enabled
    SystemEnabled,
    /// Achievement system disabled
    SystemDisabled,
}

/// Achievement error types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementError {
    /// Achievement not found
    AchievementNotFound(String),
    /// Achievement already unlocked
    AlreadyUnlocked(String),
    /// Invalid requirement
    InvalidRequirement(String),
    /// Configuration error
    ConfigError(String),
    /// Save error
    SaveError(String),
    /// Load error
    LoadError(String),
    /// Notification error
    NotificationError(String),
    /// Sound error
    SoundError(String),
}

impl std::fmt::Display for AchievementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AchievementError::AchievementNotFound(id) => write!(f, "Achievement not found: {}", id),
            AchievementError::AlreadyUnlocked(id) => write!(f, "Achievement already unlocked: {}", id),
            AchievementError::InvalidRequirement(req) => write!(f, "Invalid requirement: {}", req),
            AchievementError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            AchievementError::SaveError(msg) => write!(f, "Save error: {}", msg),
            AchievementError::LoadError(msg) => write!(f, "Load error: {}", msg),
            AchievementError::NotificationError(msg) => write!(f, "Notification error: {}", msg),
            AchievementError::SoundError(msg) => write!(f, "Sound error: {}", msg),
        }
    }
}

impl std::error::Error for AchievementError {}

/// Result type for achievement operations
pub type AchievementResult<T> = Result<T, AchievementError>;

/// Achievement notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementNotification {
    /// Achievement ID
    pub achievement_id: String,
    /// Achievement name
    pub name: String,
    /// Achievement description
    pub description: String,
    /// Points earned
    pub points: u32,
    /// Category
    pub category: AchievementCategory,
    /// Difficulty
    pub difficulty: AchievementDifficulty,
    /// Icon path
    pub icon_path: Option<String>,
    /// Sound path
    pub sound_path: Option<String>,
    /// Display duration
    pub duration: f32,
    /// Animation type
    pub animation_type: NotificationAnimation,
    /// Position
    pub position: (f32, f32),
    /// Size
    pub size: (f32, f32),
}

/// Notification animation types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationAnimation {
    /// Slide in from top
    SlideInTop,
    /// Slide in from bottom
    SlideInBottom,
    /// Slide in from left
    SlideInLeft,
    /// Slide in from right
    SlideInRight,
    /// Fade in
    FadeIn,
    /// Scale in
    ScaleIn,
    /// Bounce in
    BounceIn,
    /// Custom animation
    Custom(String),
}

impl Default for NotificationAnimation {
    fn default() -> Self {
        Self::SlideInTop
    }
}

/// Achievement filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementFilter {
    /// Category filter
    pub category: Option<AchievementCategory>,
    /// Difficulty filter
    pub difficulty: Option<AchievementDifficulty>,
    /// Unlocked filter
    pub unlocked: Option<bool>,
    /// Hidden filter
    pub hidden: Option<bool>,
    /// Search term
    pub search_term: Option<String>,
    /// Sort by
    pub sort_by: AchievementSortBy,
    /// Sort order
    pub sort_order: SortOrder,
}

/// Achievement sort options
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementSortBy {
    /// Sort by name
    Name,
    /// Sort by category
    Category,
    /// Sort by difficulty
    Difficulty,
    /// Sort by points
    Points,
    /// Sort by unlock date
    UnlockDate,
    /// Sort by progress
    Progress,
}

/// Sort order
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SortOrder {
    /// Ascending order
    Ascending,
    /// Descending order
    Descending,
}

impl Default for AchievementFilter {
    fn default() -> Self {
        Self {
            category: None,
            difficulty: None,
            unlocked: None,
            hidden: None,
            search_term: None,
            sort_by: AchievementSortBy::Name,
            sort_order: SortOrder::Ascending,
        }
    }
}

/// Achievement search result
#[derive(Debug, Clone)]
pub struct AchievementSearchResult {
    /// Achievement ID
    pub achievement_id: String,
    /// Achievement
    pub achievement: Achievement,
    /// Progress
    pub progress: AchievementProgress,
    /// Match score
    pub match_score: f32,
}

/// Achievement system settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementSettings {
    /// Show progress bars
    pub show_progress_bars: bool,
    /// Show point values
    pub show_points: bool,
    /// Show difficulty indicators
    pub show_difficulty: bool,
    /// Show category icons
    pub show_category_icons: bool,
    /// Enable sound effects
    pub enable_sounds: bool,
    /// Enable notifications
    pub enable_notifications: bool,
    /// Notification style
    pub notification_style: NotificationStyle,
    /// UI theme
    pub ui_theme: AchievementTheme,
    /// Language
    pub language: String,
}

/// Notification style
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationStyle {
    /// Minimal style
    Minimal,
    /// Standard style
    Standard,
    /// Detailed style
    Detailed,
    /// Custom style
    Custom(String),
}

/// Achievement theme
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementTheme {
    /// Default theme
    Default,
    /// Dark theme
    Dark,
    /// Light theme
    Light,
    /// Colorful theme
    Colorful,
    /// Custom theme
    Custom(String),
}

impl Default for AchievementSettings {
    fn default() -> Self {
        Self {
            show_progress_bars: true,
            show_points: true,
            show_difficulty: true,
            show_category_icons: true,
            enable_sounds: true,
            enable_notifications: true,
            notification_style: NotificationStyle::Standard,
            ui_theme: AchievementTheme::Default,
            language: "en".to_string(),
        }
    }
}
