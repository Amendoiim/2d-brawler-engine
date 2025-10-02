//! New Game+ Manager
//! 
//! This module manages New Game+ features, including carryover items, enhanced difficulty, and new content.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use super::{CompletionConfig, CompletionEvent, CompletionResult, CompletionError};

/// New Game+ manager
pub struct NewGamePlusManager {
    /// Configuration
    pub config: CompletionConfig,
    /// New Game+ unlocked
    pub new_game_plus_unlocked: bool,
    /// New Game+ level
    pub new_game_plus_level: u32,
    /// Carryover settings
    pub carryover_settings: CarryoverSettings,
    /// Enhanced difficulty
    pub enhanced_difficulty: bool,
    /// New content unlocked
    pub new_content_unlocked: Vec<String>,
    /// New Game+ rewards
    pub rewards: HashMap<String, NewGamePlusReward>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&CompletionEvent) + Send + Sync>>,
}

/// Carryover settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarryoverSettings {
    /// Carryover items
    pub carryover_items: bool,
    /// Carryover equipment
    pub carryover_equipment: bool,
    /// Carryover currency
    pub carryover_currency: bool,
    /// Carryover achievements
    pub carryover_achievements: bool,
    /// Carryover secrets
    pub carryover_secrets: bool,
    /// Carryover character level
    pub carryover_character_level: bool,
    /// Carryover skills
    pub carryover_skills: bool,
    /// Carryover stats
    pub carryover_stats: bool,
}

/// New Game+ reward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGamePlusReward {
    /// Reward ID
    pub id: String,
    /// Reward name
    pub name: String,
    /// Reward type
    pub reward_type: NewGamePlusRewardType,
    /// Required NG+ level
    pub required_level: u32,
    /// Reward data
    pub data: HashMap<String, serde_json::Value>,
    /// Reward unlocked
    pub unlocked: bool,
}

/// New Game+ reward type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NewGamePlusRewardType {
    /// New character
    NewCharacter,
    /// New weapon
    NewWeapon,
    /// New ability
    NewAbility,
    /// New level
    NewLevel,
    /// New boss
    NewBoss,
    /// New secret
    NewSecret,
    /// Enhanced item
    EnhancedItem,
    /// Special title
    SpecialTitle,
}

impl NewGamePlusManager {
    /// Create a new New Game+ manager
    pub fn new(config: CompletionConfig) -> Self {
        let mut manager = Self {
            config,
            new_game_plus_unlocked: false,
            new_game_plus_level: 0,
            carryover_settings: CarryoverSettings::default(),
            enhanced_difficulty: false,
            new_content_unlocked: Vec::new(),
            rewards: HashMap::new(),
            event_handlers: Vec::new(),
        };

        // Initialize default rewards
        manager.initialize_rewards();
        manager
    }

    /// Update New Game+ manager
    pub fn update(&mut self, delta_time: f32) -> CompletionResult<()> {
        if !self.config.new_game_plus_enabled {
            return Ok(());
        }

        // Check for new rewards based on NG+ level
        self.check_new_rewards()?;

        Ok(())
    }

    /// Unlock New Game+
    pub fn unlock_new_game_plus(&mut self) -> CompletionResult<()> {
        if !self.config.new_game_plus_enabled {
            return Err(CompletionError::NewGamePlusNotUnlocked);
        }

        self.new_game_plus_unlocked = true;
        self.new_game_plus_level = 1;

        self.emit_event(CompletionEvent::NewGamePlusUnlocked);
        Ok(())
    }

    /// Start New Game+
    pub fn start_new_game_plus(&mut self, carryover_settings: CarryoverSettings) -> CompletionResult<()> {
        if !self.new_game_plus_unlocked {
            return Err(CompletionError::NewGamePlusNotUnlocked);
        }

        self.carryover_settings = carryover_settings;
        self.enhanced_difficulty = true;
        self.new_game_plus_level += 1;

        // Unlock new content based on NG+ level
        self.unlock_new_content()?;

        Ok(())
    }

    /// Get carryover settings
    pub fn get_carryover_settings(&self) -> &CarryoverSettings {
        &self.carryover_settings
    }

    /// Set carryover settings
    pub fn set_carryover_settings(&mut self, settings: CarryoverSettings) {
        self.carryover_settings = settings;
    }

    /// Get New Game+ level
    pub fn get_new_game_plus_level(&self) -> u32 {
        self.new_game_plus_level
    }

    /// Check if New Game+ is unlocked
    pub fn is_new_game_plus_unlocked(&self) -> bool {
        self.new_game_plus_unlocked
    }

    /// Check if enhanced difficulty is enabled
    pub fn is_enhanced_difficulty_enabled(&self) -> bool {
        self.enhanced_difficulty
    }

    /// Get new content unlocked
    pub fn get_new_content_unlocked(&self) -> &Vec<String> {
        &self.new_content_unlocked
    }

    /// Get rewards
    pub fn get_rewards(&self) -> &HashMap<String, NewGamePlusReward> {
        &self.rewards
    }

    /// Get available rewards for current NG+ level
    pub fn get_available_rewards(&self) -> Vec<&NewGamePlusReward> {
        self.rewards
            .values()
            .filter(|reward| reward.required_level <= self.new_game_plus_level && !reward.unlocked)
            .collect()
    }

    /// Unlock reward
    pub fn unlock_reward(&mut self, reward_id: &str) -> CompletionResult<()> {
        if let Some(reward) = self.rewards.get_mut(reward_id) {
            if reward.required_level <= self.new_game_plus_level {
                let reward_type = format!("{:?}", reward.reward_type);
                reward.unlocked = true;
                self.emit_event(CompletionEvent::RewardEarned {
                    reward_id: reward_id.to_string(),
                    reward_type,
                });
                Ok(())
            } else {
                Err(CompletionError::Unknown(format!("Reward not available for current NG+ level")))
            }
        } else {
            Err(CompletionError::RewardNotFound(reward_id.to_string()))
        }
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&CompletionEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Check for new rewards
    fn check_new_rewards(&mut self) -> CompletionResult<()> {
        let mut rewards_to_unlock = Vec::new();
        
        for reward in self.rewards.values() {
            if reward.required_level == self.new_game_plus_level && !reward.unlocked {
                rewards_to_unlock.push((reward.id.clone(), format!("{:?}", reward.reward_type)));
            }
        }
        
        for (reward_id, reward_type) in rewards_to_unlock {
            if let Some(reward) = self.rewards.get_mut(&reward_id) {
                reward.unlocked = true;
            }
            self.emit_event(CompletionEvent::RewardEarned {
                reward_id,
                reward_type,
            });
        }
        
        Ok(())
    }

    /// Unlock new content based on NG+ level
    fn unlock_new_content(&mut self) -> CompletionResult<()> {
        match self.new_game_plus_level {
            1 => {
                self.new_content_unlocked.push("ng_plus_1_boss".to_string());
                self.new_content_unlocked.push("ng_plus_1_weapon".to_string());
            }
            2 => {
                self.new_content_unlocked.push("ng_plus_2_character".to_string());
                self.new_content_unlocked.push("ng_plus_2_level".to_string());
            }
            3 => {
                self.new_content_unlocked.push("ng_plus_3_secret".to_string());
                self.new_content_unlocked.push("ng_plus_3_ability".to_string());
            }
            4 => {
                self.new_content_unlocked.push("ng_plus_4_enhanced_boss".to_string());
                self.new_content_unlocked.push("ng_plus_4_special_title".to_string());
            }
            5 => {
                self.new_content_unlocked.push("ng_plus_5_perfect_mode".to_string());
                self.new_content_unlocked.push("ng_plus_5_ultimate_weapon".to_string());
            }
            _ => {
                // Additional NG+ levels unlock random content
                self.new_content_unlocked.push(format!("ng_plus_{}_random_content", self.new_game_plus_level));
            }
        }
        Ok(())
    }

    /// Initialize default rewards
    fn initialize_rewards(&mut self) {
        let mut rewards = HashMap::new();

        // NG+ Level 1 Rewards
        rewards.insert("ng_plus_1_weapon".to_string(), NewGamePlusReward {
            id: "ng_plus_1_weapon".to_string(),
            name: "Enhanced Starter Weapon".to_string(),
            reward_type: NewGamePlusRewardType::EnhancedItem,
            required_level: 1,
            data: HashMap::new(),
            unlocked: false,
        });

        rewards.insert("ng_plus_1_boss".to_string(), NewGamePlusReward {
            id: "ng_plus_1_boss".to_string(),
            name: "Enhanced Boss Encounters".to_string(),
            reward_type: NewGamePlusRewardType::NewBoss,
            required_level: 1,
            data: HashMap::new(),
            unlocked: false,
        });

        // NG+ Level 2 Rewards
        rewards.insert("ng_plus_2_character".to_string(), NewGamePlusReward {
            id: "ng_plus_2_character".to_string(),
            name: "New Playable Character".to_string(),
            reward_type: NewGamePlusRewardType::NewCharacter,
            required_level: 2,
            data: HashMap::new(),
            unlocked: false,
        });

        rewards.insert("ng_plus_2_level".to_string(), NewGamePlusReward {
            id: "ng_plus_2_level".to_string(),
            name: "New Level: The Void".to_string(),
            reward_type: NewGamePlusRewardType::NewLevel,
            required_level: 2,
            data: HashMap::new(),
            unlocked: false,
        });

        // NG+ Level 3 Rewards
        rewards.insert("ng_plus_3_secret".to_string(), NewGamePlusReward {
            id: "ng_plus_3_secret".to_string(),
            name: "Hidden Secret Area".to_string(),
            reward_type: NewGamePlusRewardType::NewSecret,
            required_level: 3,
            data: HashMap::new(),
            unlocked: false,
        });

        rewards.insert("ng_plus_3_ability".to_string(), NewGamePlusReward {
            id: "ng_plus_3_ability".to_string(),
            name: "Ultimate Ability".to_string(),
            reward_type: NewGamePlusRewardType::NewAbility,
            required_level: 3,
            data: HashMap::new(),
            unlocked: false,
        });

        // NG+ Level 4 Rewards
        rewards.insert("ng_plus_4_enhanced_boss".to_string(), NewGamePlusReward {
            id: "ng_plus_4_enhanced_boss".to_string(),
            name: "Ultimate Boss Challenge".to_string(),
            reward_type: NewGamePlusRewardType::NewBoss,
            required_level: 4,
            data: HashMap::new(),
            unlocked: false,
        });

        rewards.insert("ng_plus_4_special_title".to_string(), NewGamePlusReward {
            id: "ng_plus_4_special_title".to_string(),
            name: "Master of the Revolution".to_string(),
            reward_type: NewGamePlusRewardType::SpecialTitle,
            required_level: 4,
            data: HashMap::new(),
            unlocked: false,
        });

        // NG+ Level 5 Rewards
        rewards.insert("ng_plus_5_perfect_mode".to_string(), NewGamePlusReward {
            id: "ng_plus_5_perfect_mode".to_string(),
            name: "Perfect Mode".to_string(),
            reward_type: NewGamePlusRewardType::NewLevel,
            required_level: 5,
            data: HashMap::new(),
            unlocked: false,
        });

        rewards.insert("ng_plus_5_ultimate_weapon".to_string(), NewGamePlusReward {
            id: "ng_plus_5_ultimate_weapon".to_string(),
            name: "Ultimate Weapon".to_string(),
            reward_type: NewGamePlusRewardType::NewWeapon,
            required_level: 5,
            data: HashMap::new(),
            unlocked: false,
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

impl Default for CarryoverSettings {
    fn default() -> Self {
        Self {
            carryover_items: true,
            carryover_equipment: true,
            carryover_currency: true,
            carryover_achievements: true,
            carryover_secrets: true,
            carryover_character_level: false, // Start at level 1 but keep stats
            carryover_skills: true,
            carryover_stats: true,
        }
    }
}

impl Default for NewGamePlusManager {
    fn default() -> Self {
        Self::new(CompletionConfig::default())
    }
}
