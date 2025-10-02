//! Completion Rewards Manager
//! 
//! This module manages completion rewards, including unlockable content, items, and special features.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use super::{CompletionConfig, CompletionEvent, CompletionResult, CompletionError};

/// Completion rewards manager
pub struct CompletionRewardManager {
    /// Configuration
    pub config: CompletionConfig,
    /// Available rewards
    pub rewards: HashMap<String, CompletionReward>,
    /// Earned rewards
    pub earned_rewards: Vec<String>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&CompletionEvent) + Send + Sync>>,
}

/// Completion reward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionReward {
    /// Reward ID
    pub id: String,
    /// Reward name
    pub name: String,
    /// Reward description
    pub description: String,
    /// Reward type
    pub reward_type: CompletionRewardType,
    /// Required completion percentage
    pub required_completion: f32,
    /// Required milestones
    pub required_milestones: Vec<String>,
    /// Reward data
    pub data: HashMap<String, serde_json::Value>,
    /// Reward unlocked
    pub unlocked: bool,
    /// Reward claimed
    pub claimed: bool,
}

/// Completion reward type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompletionRewardType {
    /// Unlockable character
    UnlockableCharacter,
    /// Unlockable weapon
    UnlockableWeapon,
    /// Unlockable ability
    UnlockableAbility,
    /// Unlockable level
    UnlockableLevel,
    /// Unlockable boss
    UnlockableBoss,
    /// Unlockable secret
    UnlockableSecret,
    /// Unlockable mode
    UnlockableMode,
    /// Special item
    SpecialItem,
    /// Special title
    SpecialTitle,
    /// Special achievement
    SpecialAchievement,
    /// Bonus content
    BonusContent,
}

impl CompletionRewardManager {
    /// Create a new completion rewards manager
    pub fn new(config: CompletionConfig) -> Self {
        let mut manager = Self {
            config,
            rewards: HashMap::new(),
            earned_rewards: Vec::new(),
            event_handlers: Vec::new(),
        };

        // Initialize default rewards
        manager.initialize_rewards();
        manager
    }

    /// Update completion rewards manager
    pub fn update(&mut self, delta_time: f32) -> CompletionResult<()> {
        if !self.config.rewards_enabled {
            return Ok(());
        }

        // Check for new rewards based on completion
        self.check_new_rewards()?;

        Ok(())
    }

    /// Check completion and unlock rewards
    pub fn check_completion_rewards(&mut self, completion_percentage: f32, milestones: &[String]) -> CompletionResult<()> {
        if !self.config.rewards_enabled {
            return Ok(());
        }

        let mut rewards_to_unlock = Vec::new();
        
        for reward in self.rewards.values() {
            if !reward.unlocked && self.is_reward_available(reward, completion_percentage, milestones) {
                rewards_to_unlock.push((reward.id.clone(), format!("{:?}", reward.reward_type)));
            }
        }
        
        for (reward_id, reward_type) in rewards_to_unlock {
            if let Some(reward) = self.rewards.get_mut(&reward_id) {
                reward.unlocked = true;
            }
            self.earned_rewards.push(reward_id.clone());
            
            self.emit_event(CompletionEvent::RewardEarned {
                reward_id,
                reward_type,
            });
        }

        Ok(())
    }

    /// Claim reward
    pub fn claim_reward(&mut self, reward_id: &str) -> CompletionResult<()> {
        if let Some(reward) = self.rewards.get_mut(reward_id) {
            if reward.unlocked && !reward.claimed {
                reward.claimed = true;
                Ok(())
            } else if !reward.unlocked {
                Err(CompletionError::RewardNotFound(reward_id.to_string()))
            } else {
                Err(CompletionError::Unknown("Reward already claimed".to_string()))
            }
        } else {
            Err(CompletionError::RewardNotFound(reward_id.to_string()))
        }
    }

    /// Get available rewards
    pub fn get_available_rewards(&self) -> Vec<&CompletionReward> {
        self.rewards
            .values()
            .filter(|reward| reward.unlocked && !reward.claimed)
            .collect()
    }

    /// Get earned rewards
    pub fn get_earned_rewards(&self) -> &Vec<String> {
        &self.earned_rewards
    }

    /// Get all rewards
    pub fn get_all_rewards(&self) -> &HashMap<String, CompletionReward> {
        &self.rewards
    }

    /// Check if reward is available
    fn is_reward_available(&self, reward: &CompletionReward, completion_percentage: f32, milestones: &[String]) -> bool {
        // Check completion percentage requirement
        if completion_percentage < reward.required_completion {
            return false;
        }

        // Check milestone requirements
        for required_milestone in &reward.required_milestones {
            if !milestones.contains(required_milestone) {
                return false;
            }
        }

        true
    }

    /// Check for new rewards
    fn check_new_rewards(&mut self) -> CompletionResult<()> {
        // This would be called with current completion data
        // For now, just return Ok
        Ok(())
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&CompletionEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Initialize default rewards
    fn initialize_rewards(&mut self) {
        let mut rewards = HashMap::new();

        // 25% Completion Rewards
        rewards.insert("completion_25_weapon".to_string(), CompletionReward {
            id: "completion_25_weapon".to_string(),
            name: "Enhanced Combat Weapon".to_string(),
            description: "Unlock a powerful new weapon for your arsenal".to_string(),
            reward_type: CompletionRewardType::UnlockableWeapon,
            required_completion: 25.0,
            required_milestones: vec!["quarter".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        // 50% Completion Rewards
        rewards.insert("completion_50_character".to_string(), CompletionReward {
            id: "completion_50_character".to_string(),
            name: "New Playable Character".to_string(),
            description: "Unlock a new character with unique abilities".to_string(),
            reward_type: CompletionRewardType::UnlockableCharacter,
            required_completion: 50.0,
            required_milestones: vec!["half".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        rewards.insert("completion_50_level".to_string(), CompletionReward {
            id: "completion_50_level".to_string(),
            name: "Secret Level".to_string(),
            description: "Unlock a hidden level with special challenges".to_string(),
            reward_type: CompletionRewardType::UnlockableLevel,
            required_completion: 50.0,
            required_milestones: vec!["half".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        // 75% Completion Rewards
        rewards.insert("completion_75_ability".to_string(), CompletionReward {
            id: "completion_75_ability".to_string(),
            name: "Ultimate Ability".to_string(),
            description: "Unlock a devastating ultimate ability".to_string(),
            reward_type: CompletionRewardType::UnlockableAbility,
            required_completion: 75.0,
            required_milestones: vec!["three_quarters".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        rewards.insert("completion_75_boss".to_string(), CompletionReward {
            id: "completion_75_boss".to_string(),
            name: "Elite Boss Challenge".to_string(),
            description: "Unlock an elite boss encounter".to_string(),
            reward_type: CompletionRewardType::UnlockableBoss,
            required_completion: 75.0,
            required_milestones: vec!["three_quarters".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        // 90% Completion Rewards
        rewards.insert("completion_90_mode".to_string(), CompletionReward {
            id: "completion_90_mode".to_string(),
            name: "Challenge Mode".to_string(),
            description: "Unlock a special challenge mode".to_string(),
            reward_type: CompletionRewardType::UnlockableMode,
            required_completion: 90.0,
            required_milestones: vec!["near_complete".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        rewards.insert("completion_90_secret".to_string(), CompletionReward {
            id: "completion_90_secret".to_string(),
            name: "Hidden Secret Area".to_string(),
            description: "Unlock access to a hidden secret area".to_string(),
            reward_type: CompletionRewardType::UnlockableSecret,
            required_completion: 90.0,
            required_milestones: vec!["near_complete".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        // 100% Completion Rewards
        rewards.insert("completion_100_title".to_string(), CompletionReward {
            id: "completion_100_title".to_string(),
            name: "Revolution Master".to_string(),
            description: "Earn the ultimate title for completing the game".to_string(),
            reward_type: CompletionRewardType::SpecialTitle,
            required_completion: 100.0,
            required_milestones: vec!["complete".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        rewards.insert("completion_100_achievement".to_string(), CompletionReward {
            id: "completion_100_achievement".to_string(),
            name: "Perfect Completion".to_string(),
            description: "Achieve perfect completion of the game".to_string(),
            reward_type: CompletionRewardType::SpecialAchievement,
            required_completion: 100.0,
            required_milestones: vec!["complete".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        rewards.insert("completion_100_bonus".to_string(), CompletionReward {
            id: "completion_100_bonus".to_string(),
            name: "Bonus Content Pack".to_string(),
            description: "Unlock exclusive bonus content".to_string(),
            reward_type: CompletionRewardType::BonusContent,
            required_completion: 100.0,
            required_milestones: vec!["complete".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        // All Achievements Reward
        rewards.insert("all_achievements_reward".to_string(), CompletionReward {
            id: "all_achievements_reward".to_string(),
            name: "Achievement Master".to_string(),
            description: "Unlock special content for completing all achievements".to_string(),
            reward_type: CompletionRewardType::SpecialTitle,
            required_completion: 0.0,
            required_milestones: vec!["all_achievements".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        // All Secrets Reward
        rewards.insert("all_secrets_reward".to_string(), CompletionReward {
            id: "all_secrets_reward".to_string(),
            name: "Secret Hunter".to_string(),
            description: "Unlock special content for finding all secrets".to_string(),
            reward_type: CompletionRewardType::SpecialTitle,
            required_completion: 0.0,
            required_milestones: vec!["all_secrets".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        // Perfect Completion Reward
        rewards.insert("perfect_completion_reward".to_string(), CompletionReward {
            id: "perfect_completion_reward".to_string(),
            name: "Perfect Master".to_string(),
            description: "Achieve the ultimate perfect completion".to_string(),
            reward_type: CompletionRewardType::SpecialAchievement,
            required_completion: 100.0,
            required_milestones: vec!["perfect".to_string()],
            data: HashMap::new(),
            unlocked: false,
            claimed: false,
        });

        self.rewards = rewards;
    }

    /// Emit completion event
    fn emit_event(&self, event: CompletionEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for CompletionRewardManager {
    fn default() -> Self {
        Self::new(CompletionConfig::default())
    }
}
