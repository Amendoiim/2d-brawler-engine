//! Achievement progress tracking
//! 
//! This module provides progress tracking for individual achievements and overall progress.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::{AchievementResult, AchievementError};

/// Achievement progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementProgress {
    /// Achievement ID
    pub achievement_id: String,
    /// Overall progress percentage (0.0 to 100.0)
    pub progress_percentage: f32,
    /// Individual requirement progress
    pub requirement_progress: HashMap<String, f32>,
    /// Whether achievement is unlocked
    pub unlocked: bool,
    /// Unlock date
    pub unlock_date: Option<String>,
    /// First progress date
    pub first_progress_date: Option<String>,
    /// Last progress date
    pub last_progress_date: Option<String>,
    /// Total time spent on achievement
    pub total_time: f32,
    /// Number of attempts
    pub attempts: u32,
    /// Current streak
    pub current_streak: u32,
    /// Best streak
    pub best_streak: u32,
    /// Notes
    pub notes: String,
    /// Tags
    pub tags: Vec<String>,
}

impl AchievementProgress {
    /// Create new progress tracking
    pub fn new(achievement_id: String) -> Self {
        Self {
            achievement_id,
            progress_percentage: 0.0,
            requirement_progress: HashMap::new(),
            unlocked: false,
            unlock_date: None,
            first_progress_date: None,
            last_progress_date: None,
            total_time: 0.0,
            attempts: 0,
            current_streak: 0,
            best_streak: 0,
            notes: String::new(),
            tags: Vec::new(),
        }
    }

    /// Update progress for a requirement
    pub fn update_requirement_progress(&mut self, requirement_id: String, progress: f32) {
        self.requirement_progress.insert(requirement_id, progress);
        self.last_progress_date = Some(chrono::Utc::now().to_rfc3339());
        
        if self.first_progress_date.is_none() {
            self.first_progress_date = Some(chrono::Utc::now().to_rfc3339());
        }
    }

    /// Update overall progress percentage
    pub fn update_progress_percentage(&mut self, percentage: f32) {
        self.progress_percentage = percentage.min(100.0).max(0.0);
        self.last_progress_date = Some(chrono::Utc::now().to_rfc3339());
        
        if self.first_progress_date.is_none() {
            self.first_progress_date = Some(chrono::Utc::now().to_rfc3339());
        }
    }

    /// Unlock the achievement
    pub fn unlock(&mut self) -> AchievementResult<()> {
        if self.unlocked {
            return Err(AchievementError::AlreadyUnlocked(self.achievement_id.clone()));
        }

        self.unlocked = true;
        self.progress_percentage = 100.0;
        self.unlock_date = Some(chrono::Utc::now().to_rfc3339());
        self.last_progress_date = Some(chrono::Utc::now().to_rfc3339());
        
        Ok(())
    }

    /// Add attempt
    pub fn add_attempt(&mut self) {
        self.attempts += 1;
    }

    /// Update streak
    pub fn update_streak(&mut self, success: bool) {
        if success {
            self.current_streak += 1;
            if self.current_streak > self.best_streak {
                self.best_streak = self.current_streak;
            }
        } else {
            self.current_streak = 0;
        }
    }

    /// Add time spent
    pub fn add_time(&mut self, time: f32) {
        self.total_time += time;
    }

    /// Add note
    pub fn add_note(&mut self, note: String) {
        if !self.notes.is_empty() {
            self.notes.push('\n');
        }
        self.notes.push_str(&note);
    }

    /// Add tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Remove tag
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    /// Get progress for a specific requirement
    pub fn get_requirement_progress(&self, requirement_id: &str) -> f32 {
        self.requirement_progress.get(requirement_id).copied().unwrap_or(0.0)
    }

    /// Check if achievement is unlocked
    pub fn is_unlocked(&self) -> bool {
        self.unlocked
    }

    /// Get time since first progress
    pub fn get_time_since_first_progress(&self) -> Option<f32> {
        self.first_progress_date.as_ref().map(|date| {
            let first_date = chrono::DateTime::parse_from_rfc3339(date).unwrap();
            let now = chrono::Utc::now();
            (now - first_date.with_timezone(&chrono::Utc)).num_seconds() as f32
        })
    }

    /// Get time since unlock
    pub fn get_time_since_unlock(&self) -> Option<f32> {
        self.unlock_date.as_ref().map(|date| {
            let unlock_date = chrono::DateTime::parse_from_rfc3339(date).unwrap();
            let now = chrono::Utc::now();
            (now - unlock_date.with_timezone(&chrono::Utc)).num_seconds() as f32
        })
    }
}

/// Overall achievement progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallProgress {
    /// Total achievements
    pub total_achievements: u32,
    /// Unlocked achievements
    pub unlocked_achievements: u32,
    /// Overall completion percentage
    pub completion_percentage: f32,
    /// Total points available
    pub total_points: u32,
    /// Points earned
    pub points_earned: u32,
    /// Progress by category
    pub category_progress: HashMap<String, f32>,
    /// Progress by difficulty
    pub difficulty_progress: HashMap<String, f32>,
    /// Recent unlocks
    pub recent_unlocks: Vec<String>,
    /// Current streak
    pub current_streak: u32,
    /// Best streak
    pub best_streak: u32,
    /// Total play time
    pub total_play_time: f32,
    /// Average unlock time
    pub average_unlock_time: f32,
}

impl OverallProgress {
    /// Create new overall progress
    pub fn new() -> Self {
        Self {
            total_achievements: 0,
            unlocked_achievements: 0,
            completion_percentage: 0.0,
            total_points: 0,
            points_earned: 0,
            category_progress: HashMap::new(),
            difficulty_progress: HashMap::new(),
            recent_unlocks: Vec::new(),
            current_streak: 0,
            best_streak: 0,
            total_play_time: 0.0,
            average_unlock_time: 0.0,
        }
    }

    /// Update progress
    pub fn update(&mut self, achievement_id: String, unlocked: bool, points: u32, category: &str, difficulty: &str) {
        if unlocked {
            self.unlocked_achievements += 1;
            self.points_earned += points;
            self.recent_unlocks.insert(0, achievement_id);
            if self.recent_unlocks.len() > 10 {
                self.recent_unlocks.pop();
            }
            self.current_streak += 1;
            if self.current_streak > self.best_streak {
                self.best_streak = self.current_streak;
            }
        } else {
            self.current_streak = 0;
        }

        self.total_achievements += 1;
        self.total_points += points;
        self.completion_percentage = (self.unlocked_achievements as f32 / self.total_achievements as f32) * 100.0;

        // Update category progress
        let category_progress = self.category_progress.entry(category.to_string()).or_insert(0.0);
        *category_progress = (*category_progress * (self.total_achievements - 1) as f32 + if unlocked { 100.0 } else { 0.0 }) / self.total_achievements as f32;

        // Update difficulty progress
        let difficulty_progress = self.difficulty_progress.entry(difficulty.to_string()).or_insert(0.0);
        *difficulty_progress = (*difficulty_progress * (self.total_achievements - 1) as f32 + if unlocked { 100.0 } else { 0.0 }) / self.total_achievements as f32;
    }

    /// Add play time
    pub fn add_play_time(&mut self, time: f32) {
        self.total_play_time += time;
    }

    /// Update average unlock time
    pub fn update_average_unlock_time(&mut self, unlock_time: f32) {
        if self.unlocked_achievements > 0 {
            self.average_unlock_time = (self.average_unlock_time * (self.unlocked_achievements - 1) as f32 + unlock_time) / self.unlocked_achievements as f32;
        }
    }

    /// Get progress for a category
    pub fn get_category_progress(&self, category: &str) -> f32 {
        self.category_progress.get(category).copied().unwrap_or(0.0)
    }

    /// Get progress for a difficulty
    pub fn get_difficulty_progress(&self, difficulty: &str) -> f32 {
        self.difficulty_progress.get(difficulty).copied().unwrap_or(0.0)
    }

    /// Get completion rate
    pub fn get_completion_rate(&self) -> f32 {
        if self.total_achievements == 0 {
            0.0
        } else {
            self.unlocked_achievements as f32 / self.total_achievements as f32
        }
    }

    /// Get points rate
    pub fn get_points_rate(&self) -> f32 {
        if self.total_points == 0 {
            0.0
        } else {
            self.points_earned as f32 / self.total_points as f32
        }
    }
}

impl Default for OverallProgress {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress update event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdateEvent {
    /// Achievement ID
    pub achievement_id: String,
    /// Requirement ID (if specific)
    pub requirement_id: Option<String>,
    /// Old progress
    pub old_progress: f32,
    /// New progress
    pub new_progress: f32,
    /// Timestamp
    pub timestamp: String,
    /// Event type
    pub event_type: ProgressEventType,
}

/// Progress event types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProgressEventType {
    /// Progress updated
    ProgressUpdated,
    /// Achievement unlocked
    AchievementUnlocked,
    /// Requirement completed
    RequirementCompleted,
    /// Streak updated
    StreakUpdated,
    /// Time added
    TimeAdded,
    /// Note added
    NoteAdded,
    /// Tag added
    TagAdded,
    /// Tag removed
    TagRemoved,
}

/// Progress statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgressStats {
    /// Total progress updates
    pub total_updates: u32,
    /// Updates today
    pub updates_today: u32,
    /// Updates this week
    pub updates_this_week: u32,
    /// Updates this month
    pub updates_this_month: u32,
    /// Average progress per update
    pub average_progress_per_update: f32,
    /// Most active day
    pub most_active_day: String,
    /// Most active hour
    pub most_active_hour: u8,
    /// Longest session
    pub longest_session: f32,
    /// Current session
    pub current_session: f32,
    /// Total sessions
    pub total_sessions: u32,
}

impl ProgressStats {
    /// Update statistics
    pub fn update(&mut self, progress_update: f32, session_time: f32) {
        self.total_updates += 1;
        self.updates_today += 1;
        self.updates_this_week += 1;
        self.updates_this_month += 1;
        
        self.average_progress_per_update = (self.average_progress_per_update * (self.total_updates - 1) as f32 + progress_update) / self.total_updates as f32;
        
        if session_time > self.longest_session {
            self.longest_session = session_time;
        }
        
        self.current_session = session_time;
    }

    /// Start new session
    pub fn start_session(&mut self) {
        self.total_sessions += 1;
        self.current_session = 0.0;
    }

    /// End current session
    pub fn end_session(&mut self) {
        if self.current_session > self.longest_session {
            self.longest_session = self.current_session;
        }
        self.current_session = 0.0;
    }

    /// Add time to the stats
    pub fn add_time(&mut self, delta_time: f32) {
        self.current_session += delta_time;
    }
}
