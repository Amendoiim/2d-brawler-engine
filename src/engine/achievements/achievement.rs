//! Achievement definitions and management
//! 
//! This module defines individual achievements, their requirements, and rewards.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use super::{AchievementCategory, AchievementDifficulty, AchievementResult, AchievementError};

/// Individual achievement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    /// Achievement ID
    pub id: String,
    /// Achievement name
    pub name: String,
    /// Achievement description
    pub description: String,
    /// Achievement category
    pub category: AchievementCategory,
    /// Achievement difficulty
    pub difficulty: AchievementDifficulty,
    /// Points awarded
    pub points: u32,
    /// Requirements to unlock
    pub requirements: Vec<AchievementRequirement>,
    /// Rewards for unlocking
    pub rewards: Vec<AchievementReward>,
    /// Whether achievement is hidden until unlocked
    pub hidden: bool,
    /// Prerequisites (other achievements that must be unlocked first)
    pub prerequisites: Vec<String>,
    /// Icon path
    pub icon_path: Option<String>,
    /// Sound path for unlock
    pub sound_path: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Creation date
    pub created_date: String,
    /// Last modified date
    pub last_modified: String,
    /// Version
    pub version: u32,
}

impl Achievement {
    /// Create a new achievement
    pub fn new(
        id: String,
        name: String,
        description: String,
        category: AchievementCategory,
        difficulty: AchievementDifficulty,
        points: u32,
    ) -> Self {
        Self {
            id,
            name,
            description,
            category,
            difficulty,
            points,
            requirements: Vec::new(),
            rewards: Vec::new(),
            hidden: false,
            prerequisites: Vec::new(),
            icon_path: None,
            sound_path: None,
            tags: Vec::new(),
            created_date: chrono::Utc::now().to_rfc3339(),
            last_modified: chrono::Utc::now().to_rfc3339(),
            version: 1,
        }
    }

    /// Add a requirement
    pub fn add_requirement(&mut self, requirement: AchievementRequirement) {
        self.requirements.push(requirement);
        self.last_modified = chrono::Utc::now().to_rfc3339();
    }

    /// Add a reward
    pub fn add_reward(&mut self, reward: AchievementReward) {
        self.rewards.push(reward);
        self.last_modified = chrono::Utc::now().to_rfc3339();
    }

    /// Add a prerequisite
    pub fn add_prerequisite(&mut self, prerequisite: String) {
        self.prerequisites.push(prerequisite);
        self.last_modified = chrono::Utc::now().to_rfc3339();
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
        self.last_modified = chrono::Utc::now().to_rfc3339();
    }

    /// Set as hidden
    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
        self.last_modified = chrono::Utc::now().to_rfc3339();
    }

    /// Set icon path
    pub fn set_icon_path(&mut self, icon_path: String) {
        self.icon_path = Some(icon_path);
        self.last_modified = chrono::Utc::now().to_rfc3339();
    }

    /// Set sound path
    pub fn set_sound_path(&mut self, sound_path: String) {
        self.sound_path = Some(sound_path);
        self.last_modified = chrono::Utc::now().to_rfc3339();
    }

    /// Check if achievement can be unlocked
    pub fn can_unlock(&self, unlocked_achievements: &HashSet<String>) -> bool {
        // Check prerequisites
        for prerequisite in &self.prerequisites {
            if !unlocked_achievements.contains(prerequisite) {
                return false;
            }
        }
        true
    }

    /// Get total progress percentage
    pub fn get_progress_percentage(&self, progress_values: &HashMap<String, f32>) -> f32 {
        if self.requirements.is_empty() {
            return 100.0;
        }

        let mut total_progress = 0.0;
        for requirement in &self.requirements {
            let progress = progress_values.get(&requirement.id).copied().unwrap_or(0.0);
            total_progress += progress;
        }

        total_progress / self.requirements.len() as f32
    }

    /// Check if achievement is unlocked
    pub fn is_unlocked(&self, progress_values: &HashMap<String, f32>) -> bool {
        self.get_progress_percentage(progress_values) >= 100.0
    }
}

/// Achievement requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementRequirement {
    /// Requirement ID
    pub id: String,
    /// Requirement type
    pub requirement_type: RequirementType,
    /// Target value
    pub target_value: f32,
    /// Current value
    pub current_value: f32,
    /// Description
    pub description: String,
    /// Whether requirement is optional
    pub optional: bool,
    /// Weight for progress calculation
    pub weight: f32,
    /// Additional data
    pub data: HashMap<String, String>,
}

impl AchievementRequirement {
    /// Create a new requirement
    pub fn new(
        id: String,
        requirement_type: RequirementType,
        target_value: f32,
        description: String,
    ) -> Self {
        Self {
            id,
            requirement_type,
            target_value,
            current_value: 0.0,
            description,
            optional: false,
            weight: 1.0,
            data: HashMap::new(),
        }
    }

    /// Set as optional
    pub fn set_optional(&mut self, optional: bool) {
        self.optional = optional;
    }

    /// Set weight
    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
    }

    /// Add data
    pub fn add_data(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Get progress percentage
    pub fn get_progress_percentage(&self) -> f32 {
        if self.target_value <= 0.0 {
            return 100.0;
        }
        (self.current_value / self.target_value * 100.0).min(100.0)
    }

    /// Check if requirement is completed
    pub fn is_completed(&self) -> bool {
        self.current_value >= self.target_value
    }

    /// Update progress
    pub fn update_progress(&mut self, value: f32) {
        self.current_value = self.current_value.max(value);
    }
}

/// Requirement types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RequirementType {
    /// Kill enemies
    KillEnemies,
    /// Kill specific enemy type
    KillEnemyType(String),
    /// Deal damage
    DealDamage,
    /// Take damage
    TakeDamage,
    /// Complete levels
    CompleteLevels,
    /// Complete specific level
    CompleteLevel(String),
    /// Collect items
    CollectItems,
    /// Collect specific item
    CollectItem(String),
    /// Use items
    UseItems,
    /// Use specific item
    UseItem(String),
    /// Perform combos
    PerformCombos,
    /// Perform specific combo
    PerformCombo(String),
    /// Survive time
    SurviveTime,
    /// Travel distance
    TravelDistance,
    /// Jump count
    JumpCount,
    /// Block attacks
    BlockAttacks,
    /// Dodge attacks
    DodgeAttacks,
    /// Perfect levels
    PerfectLevels,
    /// Speed run levels
    SpeedRunLevels,
    /// Collect secrets
    CollectSecrets,
    /// Interact with objects
    InteractWithObjects,
    /// Custom requirement
    Custom(String),
}

/// Achievement reward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementReward {
    /// Reward ID
    pub id: String,
    /// Reward type
    pub reward_type: AchievementRewardType,
    /// Reward data
    pub reward_data: AchievementRewardData,
    /// Quantity
    pub quantity: u32,
    /// Description
    pub description: String,
    /// Whether reward is claimed
    pub claimed: bool,
    /// Claim date
    pub claim_date: Option<String>,
}

impl AchievementReward {
    /// Create a new reward
    pub fn new(
        id: String,
        reward_type: AchievementRewardType,
        reward_data: AchievementRewardData,
        quantity: u32,
        description: String,
    ) -> Self {
        Self {
            id,
            reward_type,
            reward_data,
            quantity,
            description,
            claimed: false,
            claim_date: None,
        }
    }

    /// Claim the reward
    pub fn claim(&mut self) -> AchievementResult<()> {
        if self.claimed {
            return Err(AchievementError::AlreadyUnlocked(self.id.clone()));
        }

        self.claimed = true;
        self.claim_date = Some(chrono::Utc::now().to_rfc3339());
        Ok(())
    }

    /// Check if reward is claimed
    pub fn is_claimed(&self) -> bool {
        self.claimed
    }
}

/// Achievement reward types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementRewardType {
    /// Experience points
    Experience,
    /// Gold/currency
    Currency,
    /// Item
    Item,
    /// Equipment
    Equipment,
    /// Skill point
    SkillPoint,
    /// Unlock
    Unlock,
    /// Title
    Title,
    /// Badge
    Badge,
    /// Custom reward
    Custom(String),
}

/// Achievement reward data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementRewardData {
    /// No data
    None,
    /// Item data
    ItemData {
        item_id: String,
        item_type: String,
    },
    /// Equipment data
    EquipmentData {
        equipment_id: String,
        slot: String,
    },
    /// Unlock data
    UnlockData {
        unlock_type: String,
        unlock_id: String,
    },
    /// Title data
    TitleData {
        title_id: String,
        title_name: String,
    },
    /// Badge data
    BadgeData {
        badge_id: String,
        badge_name: String,
    },
    /// Custom data
    CustomData {
        data_type: String,
        value: String,
    },
}

impl Default for AchievementRewardData {
    fn default() -> Self {
        Self::None
    }
}

/// Achievement builder for creating achievements
pub struct AchievementBuilder {
    achievement: Achievement,
}

impl AchievementBuilder {
    /// Create a new achievement builder
    pub fn new(
        id: String,
        name: String,
        description: String,
        category: AchievementCategory,
        difficulty: AchievementDifficulty,
        points: u32,
    ) -> Self {
        Self {
            achievement: Achievement::new(id, name, description, category, difficulty, points),
        }
    }

    /// Add a requirement
    pub fn add_requirement(mut self, requirement: AchievementRequirement) -> Self {
        self.achievement.add_requirement(requirement);
        self
    }

    /// Add a reward
    pub fn add_reward(mut self, reward: AchievementReward) -> Self {
        self.achievement.add_reward(reward);
        self
    }

    /// Add a prerequisite
    pub fn add_prerequisite(mut self, prerequisite: String) -> Self {
        self.achievement.add_prerequisite(prerequisite);
        self
    }

    /// Add a tag
    pub fn add_tag(mut self, tag: String) -> Self {
        self.achievement.add_tag(tag);
        self
    }

    /// Set as hidden
    pub fn set_hidden(mut self, hidden: bool) -> Self {
        self.achievement.set_hidden(hidden);
        self
    }

    /// Set icon path
    pub fn set_icon_path(mut self, icon_path: String) -> Self {
        self.achievement.set_icon_path(icon_path);
        self
    }

    /// Set sound path
    pub fn set_sound_path(mut self, sound_path: String) -> Self {
        self.achievement.set_sound_path(sound_path);
        self
    }

    /// Build the achievement
    pub fn build(self) -> Achievement {
        self.achievement
    }
}

/// Requirement builder for creating requirements
pub struct RequirementBuilder {
    requirement: AchievementRequirement,
}

impl RequirementBuilder {
    /// Create a new requirement builder
    pub fn new(
        id: String,
        requirement_type: RequirementType,
        target_value: f32,
        description: String,
    ) -> Self {
        Self {
            requirement: AchievementRequirement::new(id, requirement_type, target_value, description),
        }
    }

    /// Set as optional
    pub fn set_optional(mut self, optional: bool) -> Self {
        self.requirement.set_optional(optional);
        self
    }

    /// Set weight
    pub fn set_weight(mut self, weight: f32) -> Self {
        self.requirement.set_weight(weight);
        self
    }

    /// Add data
    pub fn add_data(mut self, key: String, value: String) -> Self {
        self.requirement.add_data(key, value);
        self
    }

    /// Build the requirement
    pub fn build(self) -> AchievementRequirement {
        self.requirement
    }
}

/// Reward builder for creating rewards
pub struct RewardBuilder {
    reward: AchievementReward,
}

impl RewardBuilder {
    /// Create a new reward builder
    pub fn new(
        id: String,
        reward_type: AchievementRewardType,
        reward_data: AchievementRewardData,
        quantity: u32,
        description: String,
    ) -> Self {
        Self {
            reward: AchievementReward::new(id, reward_type, reward_data, quantity, description),
        }
    }

    /// Build the reward
    pub fn build(self) -> AchievementReward {
        self.reward
    }
}
