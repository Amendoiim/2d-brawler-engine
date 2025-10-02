//! Achievement categories and difficulty levels
//! 
//! This module defines achievement categories and difficulty levels for organizing achievements.

use serde::{Deserialize, Serialize};

/// Achievement categories
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementCategory {
    /// Combat achievements
    Combat,
    /// Exploration achievements
    Exploration,
    /// Collection achievements
    Collection,
    /// Progression achievements
    Progression,
    /// Skill achievements
    Skill,
    /// Social achievements
    Social,
    /// Time-based achievements
    Time,
    /// Special achievements
    Special,
    /// Hidden achievements
    Hidden,
    /// Custom category
    Custom(String),
}

impl AchievementCategory {
    /// Get category name
    pub fn name(&self) -> &str {
        match self {
            AchievementCategory::Combat => "Combat",
            AchievementCategory::Exploration => "Exploration",
            AchievementCategory::Collection => "Collection",
            AchievementCategory::Progression => "Progression",
            AchievementCategory::Skill => "Skill",
            AchievementCategory::Social => "Social",
            AchievementCategory::Time => "Time",
            AchievementCategory::Special => "Special",
            AchievementCategory::Hidden => "Hidden",
            AchievementCategory::Custom(name) => name,
        }
    }

    /// Get category description
    pub fn description(&self) -> &str {
        match self {
            AchievementCategory::Combat => "Achievements related to combat and fighting",
            AchievementCategory::Exploration => "Achievements for exploring the world",
            AchievementCategory::Collection => "Achievements for collecting items and objects",
            AchievementCategory::Progression => "Achievements for character and game progression",
            AchievementCategory::Skill => "Achievements for mastering skills and abilities",
            AchievementCategory::Social => "Achievements for social interactions",
            AchievementCategory::Time => "Achievements based on time and speed",
            AchievementCategory::Special => "Special and unique achievements",
            AchievementCategory::Hidden => "Hidden achievements that are not visible until unlocked",
            AchievementCategory::Custom(_) => "Custom achievement category",
        }
    }

    /// Get category icon
    pub fn icon(&self) -> &str {
        match self {
            AchievementCategory::Combat => "⚔️",
            AchievementCategory::Exploration => "🗺️",
            AchievementCategory::Collection => "📦",
            AchievementCategory::Progression => "📈",
            AchievementCategory::Skill => "🎯",
            AchievementCategory::Social => "👥",
            AchievementCategory::Time => "⏰",
            AchievementCategory::Special => "⭐",
            AchievementCategory::Hidden => "🔒",
            AchievementCategory::Custom(_) => "🏷️",
        }
    }

    /// Get category color
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            AchievementCategory::Combat => (220, 20, 60),      // Crimson
            AchievementCategory::Exploration => (34, 139, 34),  // Forest Green
            AchievementCategory::Collection => (255, 165, 0),   // Orange
            AchievementCategory::Progression => (75, 0, 130),   // Indigo
            AchievementCategory::Skill => (255, 20, 147),       // Deep Pink
            AchievementCategory::Social => (0, 191, 255),       // Deep Sky Blue
            AchievementCategory::Time => (255, 215, 0),         // Gold
            AchievementCategory::Special => (138, 43, 226),     // Blue Violet
            AchievementCategory::Hidden => (105, 105, 105),     // Dim Gray
            AchievementCategory::Custom(_) => (128, 128, 128),  // Gray
        }
    }

    /// Get all standard categories
    pub fn all_standard() -> Vec<AchievementCategory> {
        vec![
            AchievementCategory::Combat,
            AchievementCategory::Exploration,
            AchievementCategory::Collection,
            AchievementCategory::Progression,
            AchievementCategory::Skill,
            AchievementCategory::Social,
            AchievementCategory::Time,
            AchievementCategory::Special,
            AchievementCategory::Hidden,
        ]
    }
}

/// Achievement difficulty levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementDifficulty {
    /// Very easy difficulty
    VeryEasy,
    /// Easy difficulty
    Easy,
    /// Normal difficulty
    Normal,
    /// Hard difficulty
    Hard,
    /// Very hard difficulty
    VeryHard,
    /// Extreme difficulty
    Extreme,
    /// Legendary difficulty
    Legendary,
    /// Custom difficulty
    Custom(String),
}

impl AchievementDifficulty {
    /// Get difficulty name
    pub fn name(&self) -> &str {
        match self {
            AchievementDifficulty::VeryEasy => "Very Easy",
            AchievementDifficulty::Easy => "Easy",
            AchievementDifficulty::Normal => "Normal",
            AchievementDifficulty::Hard => "Hard",
            AchievementDifficulty::VeryHard => "Very Hard",
            AchievementDifficulty::Extreme => "Extreme",
            AchievementDifficulty::Legendary => "Legendary",
            AchievementDifficulty::Custom(name) => name,
        }
    }

    /// Get difficulty description
    pub fn description(&self) -> &str {
        match self {
            AchievementDifficulty::VeryEasy => "Can be completed by any player with minimal effort",
            AchievementDifficulty::Easy => "Can be completed by most players with little effort",
            AchievementDifficulty::Normal => "Requires moderate skill and effort to complete",
            AchievementDifficulty::Hard => "Requires significant skill and effort to complete",
            AchievementDifficulty::VeryHard => "Requires advanced skill and dedication to complete",
            AchievementDifficulty::Extreme => "Requires expert skill and extensive dedication to complete",
            AchievementDifficulty::Legendary => "Requires mastery and exceptional dedication to complete",
            AchievementDifficulty::Custom(_) => "Custom difficulty level",
        }
    }

    /// Get difficulty level (1-7)
    pub fn level(&self) -> u8 {
        match self {
            AchievementDifficulty::VeryEasy => 1,
            AchievementDifficulty::Easy => 2,
            AchievementDifficulty::Normal => 3,
            AchievementDifficulty::Hard => 4,
            AchievementDifficulty::VeryHard => 5,
            AchievementDifficulty::Extreme => 6,
            AchievementDifficulty::Legendary => 7,
            AchievementDifficulty::Custom(_) => 3, // Default to normal
        }
    }

    /// Get difficulty multiplier for points
    pub fn multiplier(&self) -> f32 {
        match self {
            AchievementDifficulty::VeryEasy => 0.5,
            AchievementDifficulty::Easy => 0.75,
            AchievementDifficulty::Normal => 1.0,
            AchievementDifficulty::Hard => 1.5,
            AchievementDifficulty::VeryHard => 2.0,
            AchievementDifficulty::Extreme => 3.0,
            AchievementDifficulty::Legendary => 5.0,
            AchievementDifficulty::Custom(_) => 1.0, // Default multiplier
        }
    }

    /// Get difficulty color
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            AchievementDifficulty::VeryEasy => (144, 238, 144),  // Light Green
            AchievementDifficulty::Easy => (152, 251, 152),      // Pale Green
            AchievementDifficulty::Normal => (255, 255, 0),      // Yellow
            AchievementDifficulty::Hard => (255, 165, 0),        // Orange
            AchievementDifficulty::VeryHard => (255, 69, 0),     // Red Orange
            AchievementDifficulty::Extreme => (220, 20, 60),     // Crimson
            AchievementDifficulty::Legendary => (75, 0, 130),    // Indigo
            AchievementDifficulty::Custom(_) => (128, 128, 128), // Gray
        }
    }

    /// Get difficulty icon
    pub fn icon(&self) -> &str {
        match self {
            AchievementDifficulty::VeryEasy => "🟢",
            AchievementDifficulty::Easy => "🟡",
            AchievementDifficulty::Normal => "🟠",
            AchievementDifficulty::Hard => "🔴",
            AchievementDifficulty::VeryHard => "🟣",
            AchievementDifficulty::Extreme => "⚫",
            AchievementDifficulty::Legendary => "💎",
            AchievementDifficulty::Custom(_) => "❓",
        }
    }

    /// Get all standard difficulties
    pub fn all_standard() -> Vec<AchievementDifficulty> {
        vec![
            AchievementDifficulty::VeryEasy,
            AchievementDifficulty::Easy,
            AchievementDifficulty::Normal,
            AchievementDifficulty::Hard,
            AchievementDifficulty::VeryHard,
            AchievementDifficulty::Extreme,
            AchievementDifficulty::Legendary,
        ]
    }

    /// Get difficulty from level
    pub fn from_level(level: u8) -> Self {
        match level {
            1 => AchievementDifficulty::VeryEasy,
            2 => AchievementDifficulty::Easy,
            3 => AchievementDifficulty::Normal,
            4 => AchievementDifficulty::Hard,
            5 => AchievementDifficulty::VeryHard,
            6 => AchievementDifficulty::Extreme,
            7 => AchievementDifficulty::Legendary,
            _ => AchievementDifficulty::Normal,
        }
    }
}

impl Default for AchievementDifficulty {
    fn default() -> Self {
        Self::Normal
    }
}

/// Achievement category statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryStats {
    /// Total achievements in category
    pub total_achievements: u32,
    /// Unlocked achievements in category
    pub unlocked_achievements: u32,
    /// Completion percentage
    pub completion_percentage: f32,
    /// Total points in category
    pub total_points: u32,
    /// Earned points in category
    pub earned_points: u32,
    /// Average difficulty
    pub average_difficulty: f32,
    /// Most common difficulty
    pub most_common_difficulty: AchievementDifficulty,
    /// Recent achievements
    pub recent_achievements: Vec<String>,
}

impl CategoryStats {
    /// Update statistics
    pub fn update(&self, achievement: &super::Achievement, unlocked: bool) -> Self {
        let mut new_stats = self.clone();
        
        if unlocked {
            new_stats.unlocked_achievements += 1;
            new_stats.earned_points += achievement.points;
        }
        
        new_stats.total_achievements += 1;
        new_stats.total_points += achievement.points;
        new_stats.completion_percentage = (new_stats.unlocked_achievements as f32 / new_stats.total_achievements as f32) * 100.0;
        
        if unlocked {
            new_stats.recent_achievements.insert(0, achievement.id.clone());
            if new_stats.recent_achievements.len() > 5 {
                new_stats.recent_achievements.pop();
            }
        }
        
        new_stats
    }
}

/// Achievement difficulty statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DifficultyStats {
    /// Total achievements of this difficulty
    pub total_achievements: u32,
    /// Unlocked achievements of this difficulty
    pub unlocked_achievements: u32,
    /// Completion percentage
    pub completion_percentage: f32,
    /// Total points for this difficulty
    pub total_points: u32,
    /// Earned points for this difficulty
    pub earned_points: u32,
    /// Average completion time
    pub average_completion_time: f32,
    /// Fastest completion time
    pub fastest_completion_time: f32,
    /// Slowest completion time
    pub slowest_completion_time: f32,
}

impl DifficultyStats {
    /// Update statistics
    pub fn update(&self, achievement: &super::Achievement, unlocked: bool, completion_time: f32) -> Self {
        let mut new_stats = self.clone();
        
        if unlocked {
            new_stats.unlocked_achievements += 1;
            new_stats.earned_points += achievement.points;
            
            if new_stats.fastest_completion_time == 0.0 || completion_time < new_stats.fastest_completion_time {
                new_stats.fastest_completion_time = completion_time;
            }
            
            if completion_time > new_stats.slowest_completion_time {
                new_stats.slowest_completion_time = completion_time;
            }
            
            new_stats.average_completion_time = (new_stats.average_completion_time * (new_stats.unlocked_achievements - 1) as f32 + completion_time) / new_stats.unlocked_achievements as f32;
        }
        
        new_stats.total_achievements += 1;
        new_stats.total_points += achievement.points;
        new_stats.completion_percentage = (new_stats.unlocked_achievements as f32 / new_stats.total_achievements as f32) * 100.0;
        
        new_stats
    }
}
