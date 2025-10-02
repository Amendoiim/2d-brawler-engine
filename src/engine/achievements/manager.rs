//! Achievement manager
//! 
//! This module provides the main achievement management system.

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use super::{
    Achievement, AchievementProgress, AchievementConfig, AchievementStats, 
    AchievementEvent, AchievementError, AchievementResult, AchievementFilter,
    AchievementSearchResult, AchievementSettings,
    AchievementCategory, AchievementDifficulty, AchievementRewardType, AchievementRewardData,
    OverallProgress, ProgressStats
};

/// Main achievement manager
pub struct AchievementManager {
    /// Available achievements
    pub achievements: HashMap<String, Achievement>,
    /// Achievement progress
    pub progress: HashMap<String, AchievementProgress>,
    /// Unlocked achievements
    pub unlocked_achievements: HashSet<String>,
    /// Configuration
    pub config: AchievementConfig,
    /// Settings
    pub settings: AchievementSettings,
    /// Statistics
    pub stats: AchievementStats,
    /// Overall progress
    pub overall_progress: OverallProgress,
    /// Progress statistics
    pub progress_stats: ProgressStats,
    /// Event handlers
    pub event_handlers: HashMap<AchievementEvent, Vec<Box<dyn Fn(&AchievementEvent) + Send + Sync>>>,
    /// Notifications
    pub notifications: Vec<super::AchievementNotification>,
}

impl AchievementManager {
    /// Create a new achievement manager
    pub fn new() -> Self {
        Self {
            achievements: HashMap::new(),
            progress: HashMap::new(),
            unlocked_achievements: HashSet::new(),
            config: AchievementConfig::default(),
            settings: AchievementSettings::default(),
            stats: AchievementStats::default(),
            overall_progress: OverallProgress::new(),
            progress_stats: ProgressStats::default(),
            event_handlers: HashMap::new(),
            notifications: Vec::new(),
        }
    }

    /// Add an achievement
    pub fn add_achievement(&mut self, achievement: Achievement) -> AchievementResult<()> {
        if self.achievements.contains_key(&achievement.id) {
            return Err(AchievementError::ConfigError(format!("Achievement '{}' already exists", achievement.id)));
        }

        self.achievements.insert(achievement.id.clone(), achievement);
        self.progress.insert(achievement.id.clone(), AchievementProgress::new(achievement.id.clone()));
        self.stats.total_available += 1;
        Ok(())
    }

    /// Update achievement progress
    pub fn update_progress(&mut self, achievement_id: &str, requirement_id: &str, progress: f32) -> AchievementResult<()> {
        let achievement = self.achievements.get(achievement_id)
            .ok_or_else(|| AchievementError::AchievementNotFound(achievement_id.to_string()))?;

        let progress_entry = self.progress.get_mut(achievement_id)
            .ok_or_else(|| AchievementError::AchievementNotFound(achievement_id.to_string()))?;

        // Update requirement progress
        progress_entry.update_requirement_progress(requirement_id.to_string(), progress);

        // Calculate overall progress
        let overall_progress = achievement.get_progress_percentage(&progress_entry.requirement_progress);
        progress_entry.update_progress_percentage(overall_progress);

        // Check if achievement is unlocked
        if overall_progress >= 100.0 && !progress_entry.unlocked {
            self.unlock_achievement(achievement_id)?;
        }

        // Emit progress update event
        self.emit_event(AchievementEvent::ProgressUpdated(achievement_id.to_string(), overall_progress as u32));

        Ok(())
    }

    /// Unlock an achievement
    pub fn unlock_achievement(&mut self, achievement_id: &str) -> AchievementResult<()> {
        let achievement = self.achievements.get(achievement_id)
            .ok_or_else(|| AchievementError::AchievementNotFound(achievement_id.to_string()))?;

        if self.unlocked_achievements.contains(achievement_id) {
            return Err(AchievementError::AlreadyUnlocked(achievement_id.to_string()));
        }

        // Check prerequisites
        if !achievement.can_unlock(&self.unlocked_achievements) {
            return Err(AchievementError::InvalidRequirement("Prerequisites not met".to_string()));
        }

        // Unlock the achievement
        let progress_entry = self.progress.get_mut(achievement_id)
            .ok_or_else(|| AchievementError::AchievementNotFound(achievement_id.to_string()))?;

        progress_entry.unlock()?;
        self.unlocked_achievements.insert(achievement_id.to_string());

        // Update statistics
        let unlock_time = progress_entry.get_time_since_first_progress().unwrap_or(0.0);
        self.stats.update(achievement, unlock_time);
        self.overall_progress.update(
            achievement_id.to_string(),
            true,
            achievement.points,
            achievement.category.name(),
            achievement.difficulty.name(),
        );

        // Create notification
        if self.config.show_notifications {
            let notification = super::AchievementNotification {
                achievement_id: achievement_id.to_string(),
                name: achievement.name.clone(),
                description: achievement.description.clone(),
                points: achievement.points,
                category: achievement.category.clone(),
                difficulty: achievement.difficulty.clone(),
                icon_path: achievement.icon_path.clone(),
                sound_path: achievement.sound_path.clone(),
                duration: self.config.notification_duration,
                animation_type: super::NotificationAnimation::default(),
                position: self.config.notification_position,
                size: self.config.notification_size,
            };
            self.notifications.push(notification);
        }

        // Emit unlock event
        self.emit_event(AchievementEvent::Unlocked(achievement_id.to_string()));

        Ok(())
    }

    /// Get achievement by ID
    pub fn get_achievement(&self, achievement_id: &str) -> Option<&Achievement> {
        self.achievements.get(achievement_id)
    }

    /// Get achievement progress
    pub fn get_progress(&self, achievement_id: &str) -> Option<&AchievementProgress> {
        self.progress.get(achievement_id)
    }

    /// Check if achievement is unlocked
    pub fn is_unlocked(&self, achievement_id: &str) -> bool {
        self.unlocked_achievements.contains(achievement_id)
    }

    /// Get all achievements
    pub fn get_all_achievements(&self) -> Vec<&Achievement> {
        self.achievements.values().collect()
    }

    /// Get unlocked achievements
    pub fn get_unlocked_achievements(&self) -> Vec<&Achievement> {
        self.unlocked_achievements.iter()
            .filter_map(|id| self.achievements.get(id))
            .collect()
    }

    /// Get achievements by category
    pub fn get_achievements_by_category(&self, category: &AchievementCategory) -> Vec<&Achievement> {
        self.achievements.values()
            .filter(|achievement| &achievement.category == category)
            .collect()
    }

    /// Get achievements by difficulty
    pub fn get_achievements_by_difficulty(&self, difficulty: &AchievementDifficulty) -> Vec<&Achievement> {
        self.achievements.values()
            .filter(|achievement| &achievement.difficulty == difficulty)
            .collect()
    }

    /// Search achievements
    pub fn search_achievements(&self, filter: &AchievementFilter) -> Vec<AchievementSearchResult> {
        let mut results = Vec::new();

        for (achievement_id, achievement) in &self.achievements {
            let progress = self.progress.get(achievement_id).unwrap();
            let mut match_score = 1.0;

            // Apply filters
            if let Some(category) = &filter.category {
                if &achievement.category != category {
                    continue;
                }
            }

            if let Some(difficulty) = &filter.difficulty {
                if &achievement.difficulty != difficulty {
                    continue;
                }
            }

            if let Some(unlocked) = filter.unlocked {
                if progress.unlocked != unlocked {
                    continue;
                }
            }

            if let Some(hidden) = filter.hidden {
                if achievement.hidden != hidden {
                    continue;
                }
            }

            if let Some(search_term) = &filter.search_term {
                let search_lower = search_term.to_lowercase();
                let name_match = achievement.name.to_lowercase().contains(&search_lower);
                let desc_match = achievement.description.to_lowercase().contains(&search_lower);
                let tag_match = achievement.tags.iter().any(|tag| tag.to_lowercase().contains(&search_lower));
                
                if !name_match && !desc_match && !tag_match {
                    continue;
                }

                // Calculate match score
                if name_match {
                    match_score += 2.0;
                }
                if desc_match {
                    match_score += 1.0;
                }
                if tag_match {
                    match_score += 0.5;
                }
            }

            results.push(AchievementSearchResult {
                achievement_id: achievement_id.clone(),
                achievement: achievement.clone(),
                progress: progress.clone(),
                match_score,
            });
        }

        // Sort results
        results.sort_by(|a, b| {
            let comparison = match filter.sort_by {
                super::AchievementSortBy::Name => a.achievement.name.cmp(&b.achievement.name),
                super::AchievementSortBy::Category => a.achievement.category.name().cmp(b.achievement.category.name()),
                super::AchievementSortBy::Difficulty => a.achievement.difficulty.level().cmp(&b.achievement.difficulty.level()),
                super::AchievementSortBy::Points => a.achievement.points.cmp(&b.achievement.points),
                super::AchievementSortBy::UnlockDate => {
                    let a_date = a.progress.unlock_date.as_ref().map(|d| d.as_str()).unwrap_or("");
                    let b_date = b.progress.unlock_date.as_ref().map(|d| d.as_str()).unwrap_or("");
                    a_date.cmp(b_date)
                },
                super::AchievementSortBy::Progress => a.progress.progress_percentage.partial_cmp(&b.progress.progress_percentage).unwrap_or(std::cmp::Ordering::Equal),
            };

            match filter.sort_order {
                super::SortOrder::Ascending => comparison,
                super::SortOrder::Descending => comparison.reverse(),
            }
        });

        results
    }

    /// Update the achievement system
    pub fn update(&mut self, delta_time: f32) {
        // Update progress statistics
        self.progress_stats.add_time(delta_time);

        // Update notifications
        self.notifications.retain(|notification| {
            // This would be handled by the UI system
            true
        });
    }

    /// Register event handler
    pub fn register_event_handler<F>(&mut self, event: AchievementEvent, handler: F)
    where
        F: Fn(&AchievementEvent) + Send + Sync + 'static,
    {
        self.event_handlers.entry(event).or_insert_with(Vec::new).push(Box::new(handler));
    }

    /// Emit event
    fn emit_event(&self, event: AchievementEvent) {
        if let Some(handlers) = self.event_handlers.get(&event) {
            for handler in handlers {
                handler(&event);
            }
        }
    }

    /// Get statistics
    pub fn get_stats(&self) -> &AchievementStats {
        &self.stats
    }

    /// Get overall progress
    pub fn get_overall_progress(&self) -> &OverallProgress {
        &self.overall_progress
    }

    /// Get progress statistics
    pub fn get_progress_stats(&self) -> &ProgressStats {
        &self.progress_stats
    }

    /// Get notifications
    pub fn get_notifications(&self) -> &[super::AchievementNotification] {
        &self.notifications
    }

    /// Clear notifications
    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    /// Enable achievement system
    pub fn enable(&mut self) {
        self.config.enabled = true;
        self.emit_event(AchievementEvent::SystemEnabled);
    }

    /// Disable achievement system
    pub fn disable(&mut self) {
        self.config.enabled = false;
        self.emit_event(AchievementEvent::SystemDisabled);
    }

    /// Set configuration
    pub fn set_config(&mut self, config: AchievementConfig) {
        self.config = config;
    }

    /// Set settings
    pub fn set_settings(&mut self, settings: AchievementSettings) {
        self.settings = settings;
    }

    /// Reset all progress
    pub fn reset_all_progress(&mut self) {
        self.progress.clear();
        self.unlocked_achievements.clear();
        self.stats = AchievementStats::default();
        self.overall_progress = OverallProgress::new();
        self.progress_stats = ProgressStats::default();
        self.notifications.clear();

        // Reinitialize progress for all achievements
        for achievement_id in self.achievements.keys() {
            self.progress.insert(achievement_id.clone(), AchievementProgress::new(achievement_id.clone()));
        }
    }

    /// Reset specific achievement progress
    pub fn reset_achievement_progress(&mut self, achievement_id: &str) -> AchievementResult<()> {
        if let Some(progress) = self.progress.get_mut(achievement_id) {
            *progress = AchievementProgress::new(achievement_id.to_string());
            self.unlocked_achievements.remove(achievement_id);
            Ok(())
        } else {
            Err(AchievementError::AchievementNotFound(achievement_id.to_string()))
        }
    }

    /// Get achievement completion rate
    pub fn get_completion_rate(&self) -> f32 {
        if self.achievements.is_empty() {
            0.0
        } else {
            self.unlocked_achievements.len() as f32 / self.achievements.len() as f32
        }
    }

    /// Get points earned
    pub fn get_points_earned(&self) -> u32 {
        self.unlocked_achievements.iter()
            .filter_map(|id| self.achievements.get(id))
            .map(|achievement| achievement.points)
            .sum()
    }

    /// Get total points available
    pub fn get_total_points(&self) -> u32 {
        self.achievements.values()
            .map(|achievement| achievement.points)
            .sum()
    }

    /// Get points rate
    pub fn get_points_rate(&self) -> f32 {
        let total_points = self.get_total_points();
        if total_points == 0 {
            0.0
        } else {
            self.get_points_earned() as f32 / total_points as f32
        }
    }
}

impl Default for AchievementManager {
    fn default() -> Self {
        Self::new()
    }
}
