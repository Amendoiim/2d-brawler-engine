//! Completion Tracker
//! 
//! This module tracks game completion progress, milestones, and statistics.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

use super::{CompletionConfig, CompletionEvent, CompletionMilestone, CompletionStats, CompletionResult, CompletionError};

/// Completion tracker
pub struct CompletionTracker {
    /// Configuration
    pub config: CompletionConfig,
    /// Current completion statistics
    pub stats: CompletionStats,
    /// Completed objectives
    pub completed_objectives: HashSet<String>,
    /// Completed levels
    pub completed_levels: HashSet<String>,
    /// Found secrets
    pub found_secrets: HashSet<String>,
    /// Unlocked achievements
    pub unlocked_achievements: HashSet<String>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&CompletionEvent) + Send + Sync>>,
    /// Completion history
    pub completion_history: Vec<CompletionSnapshot>,
}

/// Completion snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionSnapshot {
    /// Timestamp
    pub timestamp: SystemTime,
    /// Completion percentage
    pub completion_percentage: f32,
    /// Play time
    pub play_time: f32,
    /// Milestones reached
    pub milestones: Vec<CompletionMilestone>,
}

impl CompletionTracker {
    /// Create a new completion tracker
    pub fn new(config: CompletionConfig) -> Self {
        Self {
            config,
            stats: CompletionStats::default(),
            completed_objectives: HashSet::new(),
            completed_levels: HashSet::new(),
            found_secrets: HashSet::new(),
            unlocked_achievements: HashSet::new(),
            event_handlers: Vec::new(),
            completion_history: Vec::new(),
        }
    }

    /// Update completion tracker
    pub fn update(&mut self, delta_time: f32) -> CompletionResult<()> {
        if !self.config.tracking_enabled {
            return Ok(());
        }

        // Update play time
        self.stats.total_play_time += delta_time;

        // Calculate overall completion
        self.calculate_overall_completion()?;

        // Check for milestones
        self.check_milestones()?;

        // Create completion snapshot
        self.create_snapshot();

        Ok(())
    }

    /// Mark objective as completed
    pub fn complete_objective(&mut self, objective_id: String) -> CompletionResult<()> {
        if !self.config.tracking_enabled {
            return Err(CompletionError::TrackingDisabled);
        }

        self.completed_objectives.insert(objective_id);
        self.calculate_overall_completion()?;
        self.check_milestones()?;

        Ok(())
    }

    /// Mark level as completed
    pub fn complete_level(&mut self, level_id: String) -> CompletionResult<()> {
        if !self.config.tracking_enabled {
            return Err(CompletionError::TrackingDisabled);
        }

        self.completed_levels.insert(level_id);
        self.calculate_overall_completion()?;
        self.check_milestones()?;

        Ok(())
    }

    /// Mark secret as found
    pub fn find_secret(&mut self, secret_id: String) -> CompletionResult<()> {
        if !self.config.tracking_enabled {
            return Err(CompletionError::TrackingDisabled);
        }

        self.found_secrets.insert(secret_id);
        self.calculate_overall_completion()?;
        self.check_milestones()?;

        Ok(())
    }

    /// Mark achievement as unlocked
    pub fn unlock_achievement(&mut self, achievement_id: String) -> CompletionResult<()> {
        if !self.config.tracking_enabled {
            return Err(CompletionError::TrackingDisabled);
        }

        self.unlocked_achievements.insert(achievement_id);
        self.calculate_overall_completion()?;
        self.check_milestones()?;

        Ok(())
    }

    /// Increment death count
    pub fn record_death(&mut self) -> CompletionResult<()> {
        if !self.config.tracking_enabled {
            return Err(CompletionError::TrackingDisabled);
        }

        self.stats.death_count += 1;
        Ok(())
    }

    /// Increment save count
    pub fn record_save(&mut self) -> CompletionResult<()> {
        if !self.config.tracking_enabled {
            return Err(CompletionError::TrackingDisabled);
        }

        self.stats.save_count += 1;
        Ok(())
    }

    /// Increment load count
    pub fn record_load(&mut self) -> CompletionResult<()> {
        if !self.config.tracking_enabled {
            return Err(CompletionError::TrackingDisabled);
        }

        self.stats.load_count += 1;
        Ok(())
    }

    /// Check if game is completed
    pub fn is_game_completed(&self) -> bool {
        self.stats.overall_completion >= self.config.max_completion
    }

    /// Check if end game is unlocked
    pub fn is_end_game_unlocked(&self) -> bool {
        self.stats.overall_completion >= self.config.end_game_threshold
    }

    /// Get completion percentage
    pub fn get_completion_percentage(&self) -> f32 {
        self.stats.overall_completion
    }

    /// Get completion statistics
    pub fn get_stats(&self) -> &CompletionStats {
        &self.stats
    }

    /// Get completed objectives
    pub fn get_completed_objectives(&self) -> &HashSet<String> {
        &self.completed_objectives
    }

    /// Get completed levels
    pub fn get_completed_levels(&self) -> &HashSet<String> {
        &self.completed_levels
    }

    /// Get found secrets
    pub fn get_found_secrets(&self) -> &HashSet<String> {
        &self.found_secrets
    }

    /// Get unlocked achievements
    pub fn get_unlocked_achievements(&self) -> &HashSet<String> {
        &self.unlocked_achievements
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&CompletionEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Calculate overall completion percentage
    fn calculate_overall_completion(&mut self) -> CompletionResult<()> {
        // Calculate story completion (based on main objectives)
        self.stats.story_completion = self.calculate_story_completion();

        // Calculate side quest completion (based on side objectives)
        self.stats.side_quest_completion = self.calculate_side_quest_completion();

        // Calculate achievement completion (based on unlocked achievements)
        self.stats.achievement_completion = self.calculate_achievement_completion();

        // Calculate secret completion (based on found secrets)
        self.stats.secret_completion = self.calculate_secret_completion();

        // Calculate level completion (based on completed levels)
        self.stats.level_completion = self.calculate_level_completion();

        // Calculate overall completion (weighted average)
        self.stats.overall_completion = self.calculate_weighted_completion();

        // Validate completion percentage
        if self.stats.overall_completion < 0.0 || self.stats.overall_completion > self.config.max_completion {
            return Err(CompletionError::InvalidPercentage(self.stats.overall_completion));
        }

        Ok(())
    }

    /// Calculate story completion percentage
    fn calculate_story_completion(&self) -> f32 {
        // This would be calculated based on main story objectives
        // For now, return a placeholder calculation
        let total_story_objectives = 20.0; // This would come from game data
        let completed_story_objectives = self.completed_objectives.len() as f32;
        (completed_story_objectives / total_story_objectives * 100.0).min(100.0)
    }

    /// Calculate side quest completion percentage
    fn calculate_side_quest_completion(&self) -> f32 {
        // This would be calculated based on side quest objectives
        // For now, return a placeholder calculation
        let total_side_quests = 15.0; // This would come from game data
        let completed_side_quests = self.completed_objectives.len() as f32 * 0.3; // Estimate
        (completed_side_quests / total_side_quests * 100.0).min(100.0)
    }

    /// Calculate achievement completion percentage
    fn calculate_achievement_completion(&self) -> f32 {
        // This would be calculated based on total achievements
        // For now, return a placeholder calculation
        let total_achievements = 50.0; // This would come from game data
        let unlocked_achievements = self.unlocked_achievements.len() as f32;
        (unlocked_achievements / total_achievements * 100.0).min(100.0)
    }

    /// Calculate secret completion percentage
    fn calculate_secret_completion(&self) -> f32 {
        // This would be calculated based on total secrets
        // For now, return a placeholder calculation
        let total_secrets = 25.0; // This would come from game data
        let found_secrets = self.found_secrets.len() as f32;
        (found_secrets / total_secrets * 100.0).min(100.0)
    }

    /// Calculate level completion percentage
    fn calculate_level_completion(&self) -> f32 {
        // This would be calculated based on total levels
        // For now, return a placeholder calculation
        let total_levels = 30.0; // This would come from game data
        let completed_levels = self.completed_levels.len() as f32;
        (completed_levels / total_levels * 100.0).min(100.0)
    }

    /// Calculate weighted completion percentage
    fn calculate_weighted_completion(&self) -> f32 {
        // Weighted average of different completion types
        let story_weight = 0.4;
        let side_quest_weight = 0.2;
        let achievement_weight = 0.2;
        let secret_weight = 0.1;
        let level_weight = 0.1;

        self.stats.story_completion * story_weight +
        self.stats.side_quest_completion * side_quest_weight +
        self.stats.achievement_completion * achievement_weight +
        self.stats.secret_completion * secret_weight +
        self.stats.level_completion * level_weight
    }

    /// Check for completion milestones
    fn check_milestones(&mut self) -> CompletionResult<()> {
        let completion = self.stats.overall_completion;
        let mut new_milestones = Vec::new();

        // Check percentage milestones
        if completion >= 25.0 && !self.stats.reached_milestones.contains(&CompletionMilestone::Quarter) {
            new_milestones.push(CompletionMilestone::Quarter);
        }
        if completion >= 50.0 && !self.stats.reached_milestones.contains(&CompletionMilestone::Half) {
            new_milestones.push(CompletionMilestone::Half);
        }
        if completion >= 75.0 && !self.stats.reached_milestones.contains(&CompletionMilestone::ThreeQuarters) {
            new_milestones.push(CompletionMilestone::ThreeQuarters);
        }
        if completion >= 90.0 && !self.stats.reached_milestones.contains(&CompletionMilestone::NearComplete) {
            new_milestones.push(CompletionMilestone::NearComplete);
        }
        if completion >= 100.0 && !self.stats.reached_milestones.contains(&CompletionMilestone::Complete) {
            new_milestones.push(CompletionMilestone::Complete);
        }

        // Check special milestones
        if self.unlocked_achievements.len() >= 50 && !self.stats.reached_milestones.contains(&CompletionMilestone::AllAchievements) {
            new_milestones.push(CompletionMilestone::AllAchievements);
        }
        if self.completed_levels.len() >= 30 && !self.stats.reached_milestones.contains(&CompletionMilestone::AllLevels) {
            new_milestones.push(CompletionMilestone::AllLevels);
        }
        if self.found_secrets.len() >= 25 && !self.stats.reached_milestones.contains(&CompletionMilestone::AllSecrets) {
            new_milestones.push(CompletionMilestone::AllSecrets);
        }

        // Check for perfect completion
        if completion >= 100.0 && 
           self.stats.achievement_completion >= 100.0 && 
           self.stats.secret_completion >= 100.0 &&
           !self.stats.reached_milestones.contains(&CompletionMilestone::Perfect) {
            new_milestones.push(CompletionMilestone::Perfect);
        }

        // Add new milestones and emit events
        for milestone in new_milestones {
            self.stats.reached_milestones.push(milestone.clone());
            self.emit_event(CompletionEvent::MilestoneReached {
                milestone: milestone.clone(),
                percentage: completion,
            });
        }

        // Check for game completion
        if self.is_game_completed() && self.stats.completion_time.is_none() {
            self.stats.completion_time = Some(SystemTime::now());
            self.emit_event(CompletionEvent::GameCompleted {
                completion_percentage: completion,
                completion_time: SystemTime::now(),
            });
        }

        // Check for end game unlock
        if self.is_end_game_unlocked() && !self.stats.reached_milestones.contains(&CompletionMilestone::NearComplete) {
            self.emit_event(CompletionEvent::EndGameUnlocked {
                completion_percentage: completion,
            });
        }

        Ok(())
    }

    /// Create completion snapshot
    fn create_snapshot(&mut self) {
        let snapshot = CompletionSnapshot {
            timestamp: SystemTime::now(),
            completion_percentage: self.stats.overall_completion,
            play_time: self.stats.total_play_time,
            milestones: self.stats.reached_milestones.clone(),
        };

        self.completion_history.push(snapshot);

        // Keep only last 100 snapshots to prevent memory bloat
        if self.completion_history.len() > 100 {
            self.completion_history.remove(0);
        }
    }

    /// Emit completion event
    fn emit_event(&self, event: CompletionEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for CompletionTracker {
    fn default() -> Self {
        Self::new(CompletionConfig::default())
    }
}
