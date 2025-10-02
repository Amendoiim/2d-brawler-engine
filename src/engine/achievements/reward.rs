//! Achievement rewards system
//! 
//! This module provides the reward system for achievements.

use serde::{Deserialize, Serialize};

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

impl AchievementRewardType {
    /// Get reward type name
    pub fn name(&self) -> &str {
        match self {
            AchievementRewardType::Experience => "Experience",
            AchievementRewardType::Currency => "Currency",
            AchievementRewardType::Item => "Item",
            AchievementRewardType::Equipment => "Equipment",
            AchievementRewardType::SkillPoint => "Skill Point",
            AchievementRewardType::Unlock => "Unlock",
            AchievementRewardType::Title => "Title",
            AchievementRewardType::Badge => "Badge",
            AchievementRewardType::Custom(name) => name,
        }
    }

    /// Get reward type description
    pub fn description(&self) -> &str {
        match self {
            AchievementRewardType::Experience => "Experience points for character progression",
            AchievementRewardType::Currency => "In-game currency for purchasing items",
            AchievementRewardType::Item => "Consumable or utility item",
            AchievementRewardType::Equipment => "Weapon, armor, or accessory",
            AchievementRewardType::SkillPoint => "Point to spend on character skills",
            AchievementRewardType::Unlock => "Unlock new content or features",
            AchievementRewardType::Title => "Display title for the player",
            AchievementRewardType::Badge => "Visual badge or icon",
            AchievementRewardType::Custom(_) => "Custom reward type",
        }
    }

    /// Get reward type icon
    pub fn icon(&self) -> &str {
        match self {
            AchievementRewardType::Experience => "⭐",
            AchievementRewardType::Currency => "💰",
            AchievementRewardType::Item => "📦",
            AchievementRewardType::Equipment => "⚔️",
            AchievementRewardType::SkillPoint => "🎯",
            AchievementRewardType::Unlock => "🔓",
            AchievementRewardType::Title => "👑",
            AchievementRewardType::Badge => "🏆",
            AchievementRewardType::Custom(_) => "🎁",
        }
    }

    /// Get reward type color
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            AchievementRewardType::Experience => (255, 215, 0),    // Gold
            AchievementRewardType::Currency => (255, 215, 0),      // Gold
            AchievementRewardType::Item => (255, 165, 0),          // Orange
            AchievementRewardType::Equipment => (192, 192, 192),   // Silver
            AchievementRewardType::SkillPoint => (0, 191, 255),    // Deep Sky Blue
            AchievementRewardType::Unlock => (50, 205, 50),        // Lime Green
            AchievementRewardType::Title => (138, 43, 226),        // Blue Violet
            AchievementRewardType::Badge => (255, 20, 147),        // Deep Pink
            AchievementRewardType::Custom(_) => (128, 128, 128),   // Gray
        }
    }
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
        item_name: String,
        item_description: String,
        item_rarity: String,
    },
    /// Equipment data
    EquipmentData {
        equipment_id: String,
        slot: String,
        equipment_name: String,
        equipment_description: String,
        equipment_rarity: String,
        stats: std::collections::HashMap<String, f32>,
    },
    /// Unlock data
    UnlockData {
        unlock_type: String,
        unlock_id: String,
        unlock_name: String,
        unlock_description: String,
    },
    /// Title data
    TitleData {
        title_id: String,
        title_name: String,
        title_description: String,
        title_color: (u8, u8, u8),
    },
    /// Badge data
    BadgeData {
        badge_id: String,
        badge_name: String,
        badge_description: String,
        badge_icon: String,
        badge_rarity: String,
    },
    /// Custom data
    CustomData {
        data_type: String,
        value: String,
        metadata: std::collections::HashMap<String, String>,
    },
}

impl Default for AchievementRewardData {
    fn default() -> Self {
        Self::None
    }
}

impl AchievementRewardData {
    /// Get reward name
    pub fn name(&self) -> &str {
        match self {
            AchievementRewardData::None => "Unknown",
            AchievementRewardData::ItemData { item_name, .. } => item_name,
            AchievementRewardData::EquipmentData { equipment_name, .. } => equipment_name,
            AchievementRewardData::UnlockData { unlock_name, .. } => unlock_name,
            AchievementRewardData::TitleData { title_name, .. } => title_name,
            AchievementRewardData::BadgeData { badge_name, .. } => badge_name,
            AchievementRewardData::CustomData { value, .. } => value,
        }
    }

    /// Get reward description
    pub fn description(&self) -> &str {
        match self {
            AchievementRewardData::None => "No description available",
            AchievementRewardData::ItemData { item_description, .. } => item_description,
            AchievementRewardData::EquipmentData { equipment_description, .. } => equipment_description,
            AchievementRewardData::UnlockData { unlock_description, .. } => unlock_description,
            AchievementRewardData::TitleData { title_description, .. } => title_description,
            AchievementRewardData::BadgeData { badge_description, .. } => badge_description,
            AchievementRewardData::CustomData { .. } => "Custom reward",
        }
    }

    /// Get reward rarity
    pub fn rarity(&self) -> Option<&str> {
        match self {
            AchievementRewardData::ItemData { item_rarity, .. } => Some(item_rarity),
            AchievementRewardData::EquipmentData { equipment_rarity, .. } => Some(equipment_rarity),
            AchievementRewardData::BadgeData { badge_rarity, .. } => Some(badge_rarity),
            _ => None,
        }
    }

    /// Get reward color
    pub fn color(&self) -> Option<(u8, u8, u8)> {
        match self {
            AchievementRewardData::TitleData { title_color, .. } => Some(*title_color),
            _ => None,
        }
    }
}

/// Reward rarity levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RewardRarity {
    /// Common rarity
    Common,
    /// Uncommon rarity
    Uncommon,
    /// Rare rarity
    Rare,
    /// Epic rarity
    Epic,
    /// Legendary rarity
    Legendary,
    /// Mythic rarity
    Mythic,
    /// Custom rarity
    Custom(String),
}

impl RewardRarity {
    /// Get rarity name
    pub fn name(&self) -> &str {
        match self {
            RewardRarity::Common => "Common",
            RewardRarity::Uncommon => "Uncommon",
            RewardRarity::Rare => "Rare",
            RewardRarity::Epic => "Epic",
            RewardRarity::Legendary => "Legendary",
            RewardRarity::Mythic => "Mythic",
            RewardRarity::Custom(name) => name,
        }
    }

    /// Get rarity color
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            RewardRarity::Common => (192, 192, 192),      // Silver
            RewardRarity::Uncommon => (0, 255, 0),        // Green
            RewardRarity::Rare => (0, 0, 255),            // Blue
            RewardRarity::Epic => (128, 0, 128),          // Purple
            RewardRarity::Legendary => (255, 215, 0),     // Gold
            RewardRarity::Mythic => (255, 0, 255),        // Magenta
            RewardRarity::Custom(_) => (128, 128, 128),   // Gray
        }
    }

    /// Get rarity multiplier
    pub fn multiplier(&self) -> f32 {
        match self {
            RewardRarity::Common => 1.0,
            RewardRarity::Uncommon => 1.5,
            RewardRarity::Rare => 2.0,
            RewardRarity::Epic => 3.0,
            RewardRarity::Legendary => 5.0,
            RewardRarity::Mythic => 10.0,
            RewardRarity::Custom(_) => 1.0,
        }
    }

    /// Get rarity icon
    pub fn icon(&self) -> &str {
        match self {
            RewardRarity::Common => "⚪",
            RewardRarity::Uncommon => "🟢",
            RewardRarity::Rare => "🔵",
            RewardRarity::Epic => "🟣",
            RewardRarity::Legendary => "🟡",
            RewardRarity::Mythic => "🔴",
            RewardRarity::Custom(_) => "❓",
        }
    }
}

/// Reward claim status
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RewardClaimStatus {
    /// Not claimed
    NotClaimed,
    /// Claimed
    Claimed,
    /// Claimed and used
    Used,
    /// Expired
    Expired,
    /// Cancelled
    Cancelled,
}

impl RewardClaimStatus {
    /// Get status name
    pub fn name(&self) -> &str {
        match self {
            RewardClaimStatus::NotClaimed => "Not Claimed",
            RewardClaimStatus::Claimed => "Claimed",
            RewardClaimStatus::Used => "Used",
            RewardClaimStatus::Expired => "Expired",
            RewardClaimStatus::Cancelled => "Cancelled",
        }
    }

    /// Get status color
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            RewardClaimStatus::NotClaimed => (128, 128, 128),  // Gray
            RewardClaimStatus::Claimed => (0, 255, 0),         // Green
            RewardClaimStatus::Used => (0, 0, 255),            // Blue
            RewardClaimStatus::Expired => (255, 0, 0),         // Red
            RewardClaimStatus::Cancelled => (255, 165, 0),     // Orange
        }
    }
}

/// Reward statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RewardStats {
    /// Total rewards available
    pub total_rewards: u32,
    /// Rewards claimed
    pub rewards_claimed: u32,
    /// Rewards used
    pub rewards_used: u32,
    /// Rewards expired
    pub rewards_expired: u32,
    /// Rewards cancelled
    pub rewards_cancelled: u32,
    /// Total value of rewards
    pub total_value: u32,
    /// Value of claimed rewards
    pub claimed_value: u32,
    /// Value of used rewards
    pub used_value: u32,
    /// Most common reward type
    pub most_common_type: Option<AchievementRewardType>,
    /// Most common rarity
    pub most_common_rarity: Option<RewardRarity>,
    /// Average claim time
    pub average_claim_time: f32,
    /// Fastest claim time
    pub fastest_claim_time: f32,
    /// Slowest claim time
    pub slowest_claim_time: f32,
}

impl RewardStats {
    /// Update statistics
    pub fn update(&self, reward_type: AchievementRewardType, rarity: RewardRarity, value: u32, claim_time: f32) -> Self {
        let mut new_stats = self.clone();
        
        new_stats.total_rewards += 1;
        new_stats.total_value += value;
        
        if claim_time > 0.0 {
            new_stats.rewards_claimed += 1;
            new_stats.claimed_value += value;
            
            if new_stats.fastest_claim_time == 0.0 || claim_time < new_stats.fastest_claim_time {
                new_stats.fastest_claim_time = claim_time;
            }
            
            if claim_time > new_stats.slowest_claim_time {
                new_stats.slowest_claim_time = claim_time;
            }
            
            new_stats.average_claim_time = (new_stats.average_claim_time * (new_stats.rewards_claimed - 1) as f32 + claim_time) / new_stats.rewards_claimed as f32;
        }
        
        new_stats
    }

    /// Mark reward as used
    pub fn mark_used(&mut self, value: u32) {
        self.rewards_used += 1;
        self.used_value += value;
    }

    /// Mark reward as expired
    pub fn mark_expired(&mut self) {
        self.rewards_expired += 1;
    }

    /// Mark reward as cancelled
    pub fn mark_cancelled(&mut self) {
        self.rewards_cancelled += 1;
    }
}

impl Default for AchievementRewardType {
    fn default() -> Self {
        AchievementRewardType::Experience
    }
}

impl Default for RewardRarity {
    fn default() -> Self {
        RewardRarity::Common
    }
}
