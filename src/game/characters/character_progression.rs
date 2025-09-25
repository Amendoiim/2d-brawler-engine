//! Character Progression Module
//! 
//! This module provides character progression and leveling functionality including:
//! - Experience gain and leveling
//! - Stat progression and allocation
//! - Ability unlocking and progression
//! - Achievement tracking
//! - Character milestones

use super::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Character progression manager
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterProgression {
    /// Character progression data
    pub progression_data: HashMap<String, CharacterProgressionData>,
    /// Level requirements
    pub level_requirements: HashMap<u32, LevelRequirement>,
    /// Ability unlock requirements
    pub ability_requirements: HashMap<String, AbilityRequirement>,
    /// Achievement definitions
    pub achievements: HashMap<String, Achievement>,
    /// Milestone definitions
    pub milestones: HashMap<String, Milestone>,
}

/// Character progression data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterProgressionData {
    /// Character ID
    pub character_id: String,
    /// Current level
    pub current_level: u32,
    /// Current experience
    pub current_experience: u32,
    /// Total experience earned
    pub total_experience: u32,
    /// Available stat points
    pub available_stat_points: u32,
    /// Available ability points
    pub available_ability_points: u32,
    /// Unlocked abilities
    pub unlocked_abilities: Vec<String>,
    /// Completed achievements
    pub completed_achievements: Vec<String>,
    /// Reached milestones
    pub reached_milestones: Vec<String>,
    /// Progression statistics
    pub statistics: ProgressionStatistics,
}

/// Level requirement
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LevelRequirement {
    /// Level number
    pub level: u32,
    /// Required experience
    pub required_experience: u32,
    /// Stat points gained
    pub stat_points_gained: u32,
    /// Ability points gained
    pub ability_points_gained: u32,
    /// Unlocked abilities
    pub unlocked_abilities: Vec<String>,
    /// Unlocked features
    pub unlocked_features: Vec<String>,
}

/// Ability requirement
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbilityRequirement {
    /// Ability ID
    pub ability_id: String,
    /// Required level
    pub required_level: u32,
    /// Required experience
    pub required_experience: u32,
    /// Required abilities
    pub required_abilities: Vec<String>,
    /// Required achievements
    pub required_achievements: Vec<String>,
    /// Required items
    pub required_items: Vec<String>,
}

/// Achievement definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Achievement {
    /// Achievement ID
    pub id: String,
    /// Achievement name
    pub name: String,
    /// Achievement description
    pub description: String,
    /// Achievement category
    pub category: AchievementCategory,
    /// Achievement rarity
    pub rarity: AchievementRarity,
    /// Achievement requirements
    pub requirements: AchievementRequirement,
    /// Achievement rewards
    pub rewards: AchievementReward,
    /// Achievement icon
    pub icon: String,
}

/// Achievement category
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AchievementCategory {
    Combat,
    Exploration,
    Social,
    Crafting,
    Collection,
    Progression,
    Special,
}

/// Achievement rarity
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AchievementRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

/// Achievement requirement
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AchievementRequirement {
    /// Kill a certain number of enemies
    KillEnemies { count: u32, enemy_type: Option<String> },
    /// Complete a certain number of levels
    CompleteLevels { count: u32, level_type: Option<String> },
    /// Reach a certain level
    ReachLevel { level: u32 },
    /// Collect a certain number of items
    CollectItems { count: u32, item_type: Option<String> },
    /// Use a certain ability a number of times
    UseAbility { ability_id: String, count: u32 },
    /// Deal a certain amount of damage
    DealDamage { amount: u32 },
    /// Survive for a certain amount of time
    SurviveTime { duration: f32 },
    /// Complete a specific quest
    CompleteQuest { quest_id: String },
    /// Multiple requirements
    Multiple { requirements: Vec<AchievementRequirement> },
}

/// Achievement reward
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementReward {
    /// Experience reward
    pub experience: u32,
    /// Stat points reward
    pub stat_points: u32,
    /// Ability points reward
    pub ability_points: u32,
    /// Item rewards
    pub items: Vec<String>,
    /// Title reward
    pub title: Option<String>,
    /// Unlocked features
    pub unlocked_features: Vec<String>,
}

/// Milestone definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Milestone {
    /// Milestone ID
    pub id: String,
    /// Milestone name
    pub name: String,
    /// Milestone description
    pub description: String,
    /// Milestone type
    pub milestone_type: MilestoneType,
    /// Milestone requirements
    pub requirements: MilestoneRequirement,
    /// Milestone rewards
    pub rewards: MilestoneReward,
}

/// Milestone type
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum MilestoneType {
    Level,
    Experience,
    Combat,
    Exploration,
    Social,
    Crafting,
    Collection,
}

/// Milestone requirement
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MilestoneRequirement {
    /// Reach a certain level
    ReachLevel { level: u32 },
    /// Gain a certain amount of experience
    GainExperience { amount: u32 },
    /// Kill a certain number of enemies
    KillEnemies { count: u32 },
    /// Complete a certain number of levels
    CompleteLevels { count: u32 },
    /// Collect a certain number of items
    CollectItems { count: u32 },
    /// Use a certain ability a number of times
    UseAbility { ability_id: String, count: u32 },
    /// Deal a certain amount of damage
    DealDamage { amount: u32 },
    /// Survive for a certain amount of time
    SurviveTime { duration: f32 },
}

/// Milestone reward
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MilestoneReward {
    /// Experience reward
    pub experience: u32,
    /// Stat points reward
    pub stat_points: u32,
    /// Ability points reward
    pub ability_points: u32,
    /// Item rewards
    pub items: Vec<String>,
    /// Unlocked features
    pub unlocked_features: Vec<String>,
}

/// Progression statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressionStatistics {
    /// Total enemies killed
    pub enemies_killed: u32,
    /// Total levels completed
    pub levels_completed: u32,
    /// Total damage dealt
    pub damage_dealt: u32,
    /// Total damage taken
    pub damage_taken: u32,
    /// Total healing done
    pub healing_done: u32,
    /// Total items collected
    pub items_collected: u32,
    /// Total time played
    pub time_played: f32,
    /// Total distance traveled
    pub distance_traveled: f32,
    /// Total abilities used
    pub abilities_used: u32,
    /// Total deaths
    pub deaths: u32,
    /// Total resurrections
    pub resurrections: u32,
    /// Total experience gained
    pub experience_gained: u32,
}

/// Character progression manager implementation
impl CharacterProgression {
    /// Create a new character progression manager
    pub fn new() -> Self {
        let mut progression = Self {
            progression_data: HashMap::new(),
            level_requirements: HashMap::new(),
            ability_requirements: HashMap::new(),
            achievements: HashMap::new(),
            milestones: HashMap::new(),
        };
        
        progression.initialize_level_requirements();
        progression.initialize_ability_requirements();
        progression.initialize_achievements();
        progression.initialize_milestones();
        
        progression
    }

    /// Initialize level requirements
    fn initialize_level_requirements(&mut self) {
        for level in 1..=100 {
            let required_exp = self.calculate_experience_for_level(level);
            let stat_points = if level > 1 { 3 } else { 0 };
            let ability_points = if level > 1 && level % 2 == 0 { 1 } else { 0 };
            
            self.level_requirements.insert(level, LevelRequirement {
                level,
                required_experience: required_exp,
                stat_points_gained: stat_points,
                ability_points_gained: ability_points,
                unlocked_abilities: Vec::new(),
                unlocked_features: Vec::new(),
            });
        }
    }

    /// Calculate experience required for a level
    fn calculate_experience_for_level(&self, level: u32) -> u32 {
        if level <= 1 {
            return 0;
        }
        
        // Cap level to prevent overflow
        let capped_level = level.min(100);
        
        let base_exp = 100;
        let level_multiplier = 1.2f32;
        let mut total_exp = 0u32;
        
        for i in 2..=capped_level {
            let exp_for_level = (base_exp as f32 * level_multiplier.powi(i as i32 - 2)) as u32;
            // Check for overflow before adding
            if total_exp > u32::MAX - exp_for_level {
                return u32::MAX;
            }
            total_exp += exp_for_level;
        }
        
        total_exp
    }

    /// Initialize ability requirements
    fn initialize_ability_requirements(&mut self) {
        // Basic abilities (unlocked at level 1)
        self.ability_requirements.insert("power_strike".to_string(), AbilityRequirement {
            ability_id: "power_strike".to_string(),
            required_level: 1,
            required_experience: 0,
            required_abilities: Vec::new(),
            required_achievements: Vec::new(),
            required_items: Vec::new(),
        });

        self.ability_requirements.insert("fireball".to_string(), AbilityRequirement {
            ability_id: "fireball".to_string(),
            required_level: 1,
            required_experience: 0,
            required_abilities: Vec::new(),
            required_achievements: Vec::new(),
            required_items: Vec::new(),
        });

        // Advanced abilities (unlocked at higher levels)
        self.ability_requirements.insert("meteor_strike".to_string(), AbilityRequirement {
            ability_id: "meteor_strike".to_string(),
            required_level: 10,
            required_experience: 5000,
            required_abilities: vec!["fireball".to_string()],
            required_achievements: Vec::new(),
            required_items: Vec::new(),
        });

        self.ability_requirements.insert("divine_smite".to_string(), AbilityRequirement {
            ability_id: "divine_smite".to_string(),
            required_level: 15,
            required_experience: 10000,
            required_abilities: vec!["power_strike".to_string()],
            required_achievements: vec!["holy_warrior".to_string()],
            required_items: Vec::new(),
        });
    }

    /// Initialize achievements
    fn initialize_achievements(&mut self) {
        // Combat achievements
        self.achievements.insert("first_kill".to_string(), Achievement {
            id: "first_kill".to_string(),
            name: "First Blood".to_string(),
            description: "Kill your first enemy".to_string(),
            category: AchievementCategory::Combat,
            rarity: AchievementRarity::Common,
            requirements: AchievementRequirement::KillEnemies { count: 1, enemy_type: None },
            rewards: AchievementReward {
                experience: 100,
                stat_points: 0,
                ability_points: 0,
                items: Vec::new(),
                title: Some("Warrior".to_string()),
                unlocked_features: Vec::new(),
            },
            icon: "achievement_first_kill".to_string(),
        });

        self.achievements.insert("killer".to_string(), Achievement {
            id: "killer".to_string(),
            name: "Killer".to_string(),
            description: "Kill 100 enemies".to_string(),
            category: AchievementCategory::Combat,
            rarity: AchievementRarity::Uncommon,
            requirements: AchievementRequirement::KillEnemies { count: 100, enemy_type: None },
            rewards: AchievementReward {
                experience: 1000,
                stat_points: 5,
                ability_points: 1,
                items: vec!["killer_sword".to_string()],
                title: Some("Killer".to_string()),
                unlocked_features: Vec::new(),
            },
            icon: "achievement_killer".to_string(),
        });

        // Progression achievements
        self.achievements.insert("level_10".to_string(), Achievement {
            id: "level_10".to_string(),
            name: "Rising Star".to_string(),
            description: "Reach level 10".to_string(),
            category: AchievementCategory::Progression,
            rarity: AchievementRarity::Common,
            requirements: AchievementRequirement::ReachLevel { level: 10 },
            rewards: AchievementReward {
                experience: 500,
                stat_points: 10,
                ability_points: 2,
                items: Vec::new(),
                title: Some("Rising Star".to_string()),
                unlocked_features: vec!["advanced_abilities".to_string()],
            },
            icon: "achievement_level_10".to_string(),
        });

        // Special achievements
        self.achievements.insert("holy_warrior".to_string(), Achievement {
            id: "holy_warrior".to_string(),
            name: "Holy Warrior".to_string(),
            description: "Complete the divine quest".to_string(),
            category: AchievementCategory::Special,
            rarity: AchievementRarity::Rare,
            requirements: AchievementRequirement::CompleteQuest { quest_id: "divine_quest".to_string() },
            rewards: AchievementReward {
                experience: 2000,
                stat_points: 15,
                ability_points: 3,
                items: vec!["holy_sword".to_string()],
                title: Some("Holy Warrior".to_string()),
                unlocked_features: vec!["divine_abilities".to_string()],
            },
            icon: "achievement_holy_warrior".to_string(),
        });
    }

    /// Initialize milestones
    fn initialize_milestones(&mut self) {
        // Level milestones
        for level in [5, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100] {
            self.milestones.insert(format!("level_{}", level), Milestone {
                id: format!("level_{}", level),
                name: format!("Level {} Milestone", level),
                description: format!("Reach level {}", level),
                milestone_type: MilestoneType::Level,
                requirements: MilestoneRequirement::ReachLevel { level },
                rewards: MilestoneReward {
                    experience: level * 100,
                    stat_points: level * 2,
                    ability_points: level / 5,
                    items: Vec::new(),
                    unlocked_features: Vec::new(),
                },
            });
        }

        // Combat milestones
        self.milestones.insert("killer_1000".to_string(), Milestone {
            id: "killer_1000".to_string(),
            name: "Master Killer".to_string(),
            description: "Kill 1000 enemies".to_string(),
            milestone_type: MilestoneType::Combat,
            requirements: MilestoneRequirement::KillEnemies { count: 1000 },
            rewards: MilestoneReward {
                experience: 5000,
                stat_points: 20,
                ability_points: 5,
                items: vec!["master_sword".to_string()],
                unlocked_features: vec!["master_abilities".to_string()],
            },
        });
    }

    /// Add character to progression tracking
    pub fn add_character(&mut self, character_id: String, character: &Character) {
        self.progression_data.insert(character_id.clone(), CharacterProgressionData {
            character_id: character_id.clone(),
            current_level: character.level,
            current_experience: character.experience,
            total_experience: character.experience,
            available_stat_points: 0,
            available_ability_points: 0,
            unlocked_abilities: character.abilities.clone(),
            completed_achievements: Vec::new(),
            reached_milestones: Vec::new(),
            statistics: ProgressionStatistics::default(),
        });
    }

    /// Add experience to character
    pub fn add_experience(&mut self, character_id: &str, amount: u32) -> Result<Vec<ProgressionEvent>, String> {
        let progression_data = self.progression_data.get_mut(character_id)
            .ok_or_else(|| format!("Character '{}' not found in progression data", character_id))?;

        let mut events = Vec::new();
        
        progression_data.current_experience += amount;
        progression_data.total_experience += amount;
        progression_data.statistics.experience_gained += amount;

        // Check for level up
        while self.can_level_up(character_id)? {
            if let Some(level_event) = self.level_up_character(character_id)? {
                events.push(level_event);
            }
        }

        // Check for milestone completion
        if let Some(milestone_event) = self.check_milestones(character_id)? {
            events.push(milestone_event);
        }

        // Check for achievement completion
        if let Some(achievement_event) = self.check_achievements(character_id)? {
            events.push(achievement_event);
        }

        Ok(events)
    }

    /// Check if character can level up
    pub fn can_level_up(&self, character_id: &str) -> Result<bool, String> {
        let progression_data = self.progression_data.get(character_id)
            .ok_or_else(|| format!("Character '{}' not found in progression data", character_id))?;

        let current_level = progression_data.current_level;
        let required_exp = self.level_requirements.get(&(current_level + 1))
            .map(|req| req.required_experience)
            .unwrap_or(u32::MAX);

        Ok(progression_data.current_experience >= required_exp)
    }

    /// Level up character
    pub fn level_up_character(&mut self, character_id: &str) -> Result<Option<ProgressionEvent>, String> {
        let progression_data = self.progression_data.get_mut(character_id)
            .ok_or_else(|| format!("Character '{}' not found in progression data", character_id))?;

        let new_level = progression_data.current_level + 1;
        let level_req = self.level_requirements.get(&new_level)
            .ok_or_else(|| format!("Level {} requirements not found", new_level))?;

        progression_data.current_level = new_level;
        progression_data.available_stat_points += level_req.stat_points_gained;
        progression_data.available_ability_points += level_req.ability_points_gained;

        let mut event = ProgressionEvent {
            event_type: ProgressionEventType::LevelUp {
                new_level,
                stat_points_gained: level_req.stat_points_gained,
                ability_points_gained: level_req.ability_points_gained,
            },
            character_id: character_id.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        Ok(Some(event))
    }

    /// Check for milestone completion
    fn check_milestones(&mut self, character_id: &str) -> Result<Option<ProgressionEvent>, String> {
        let progression_data = self.progression_data.get(character_id)
            .ok_or_else(|| format!("Character '{}' not found in progression data", character_id))?;

        for (milestone_id, milestone) in &self.milestones {
            if progression_data.reached_milestones.contains(milestone_id) {
                continue;
            }

            if self.is_milestone_completed(progression_data, milestone)? {
                // Mark milestone as reached
                if let Some(progression_data) = self.progression_data.get_mut(character_id) {
                    progression_data.reached_milestones.push(milestone_id.clone());
                    
                    // Apply rewards
                    progression_data.available_stat_points += milestone.rewards.stat_points;
                    progression_data.available_ability_points += milestone.rewards.ability_points;
                }

                return Ok(Some(ProgressionEvent {
                    event_type: ProgressionEventType::MilestoneReached {
                        milestone_id: milestone_id.clone(),
                        rewards: milestone.rewards.clone(),
                    },
                    character_id: character_id.to_string(),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                }));
            }
        }

        Ok(None)
    }

    /// Check if milestone is completed
    fn is_milestone_completed(&self, progression_data: &CharacterProgressionData, milestone: &Milestone) -> Result<bool, String> {
        match &milestone.requirements {
            MilestoneRequirement::ReachLevel { level } => {
                Ok(progression_data.current_level >= *level)
            }
            MilestoneRequirement::GainExperience { amount } => {
                Ok(progression_data.total_experience >= *amount)
            }
            MilestoneRequirement::KillEnemies { count } => {
                Ok(progression_data.statistics.enemies_killed >= *count)
            }
            MilestoneRequirement::CompleteLevels { count } => {
                Ok(progression_data.statistics.levels_completed >= *count)
            }
            MilestoneRequirement::CollectItems { count } => {
                Ok(progression_data.statistics.items_collected >= *count)
            }
            MilestoneRequirement::UseAbility { ability_id, count } => {
                // This would need to be tracked separately
                Ok(false) // Placeholder
            }
            MilestoneRequirement::DealDamage { amount } => {
                Ok(progression_data.statistics.damage_dealt >= *amount)
            }
            MilestoneRequirement::SurviveTime { duration } => {
                Ok(progression_data.statistics.time_played >= *duration)
            }
        }
    }

    /// Check for achievement completion
    fn check_achievements(&mut self, character_id: &str) -> Result<Option<ProgressionEvent>, String> {
        let progression_data = self.progression_data.get(character_id)
            .ok_or_else(|| format!("Character '{}' not found in progression data", character_id))?;

        for (achievement_id, achievement) in &self.achievements {
            if progression_data.completed_achievements.contains(achievement_id) {
                continue;
            }

            if self.is_achievement_completed(progression_data, achievement)? {
                // Mark achievement as completed
                if let Some(progression_data) = self.progression_data.get_mut(character_id) {
                    progression_data.completed_achievements.push(achievement_id.clone());
                    
                    // Apply rewards
                    progression_data.available_stat_points += achievement.rewards.stat_points;
                    progression_data.available_ability_points += achievement.rewards.ability_points;
                }

                return Ok(Some(ProgressionEvent {
                    event_type: ProgressionEventType::AchievementUnlocked {
                        achievement_id: achievement_id.clone(),
                        rewards: achievement.rewards.clone(),
                    },
                    character_id: character_id.to_string(),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                }));
            }
        }

        Ok(None)
    }

    /// Check if achievement is completed
    fn is_achievement_completed(&self, progression_data: &CharacterProgressionData, achievement: &Achievement) -> Result<bool, String> {
        match &achievement.requirements {
            AchievementRequirement::KillEnemies { count, enemy_type: _ } => {
                Ok(progression_data.statistics.enemies_killed >= *count)
            }
            AchievementRequirement::CompleteLevels { count, level_type: _ } => {
                Ok(progression_data.statistics.levels_completed >= *count)
            }
            AchievementRequirement::ReachLevel { level } => {
                Ok(progression_data.current_level >= *level)
            }
            AchievementRequirement::CollectItems { count, item_type: _ } => {
                Ok(progression_data.statistics.items_collected >= *count)
            }
            AchievementRequirement::UseAbility { ability_id: _, count: _ } => {
                // This would need to be tracked separately
                Ok(false) // Placeholder
            }
            AchievementRequirement::DealDamage { amount } => {
                Ok(progression_data.statistics.damage_dealt >= *amount)
            }
            AchievementRequirement::SurviveTime { duration } => {
                Ok(progression_data.statistics.time_played >= *duration)
            }
            AchievementRequirement::CompleteQuest { quest_id: _ } => {
                // This would need to be tracked separately
                Ok(false) // Placeholder
            }
            AchievementRequirement::Multiple { requirements } => {
                for requirement in requirements {
                    if !self.is_achievement_completed(progression_data, &Achievement {
                        id: "".to_string(),
                        name: "".to_string(),
                        description: "".to_string(),
                        category: AchievementCategory::Combat,
                        rarity: AchievementRarity::Common,
                        requirements: requirement.clone(),
                        rewards: AchievementReward {
                            experience: 0,
                            stat_points: 0,
                            ability_points: 0,
                            items: Vec::new(),
                            title: None,
                            unlocked_features: Vec::new(),
                        },
                        icon: "".to_string(),
                    })? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
        }
    }

    /// Get character progression data
    pub fn get_character_progression(&self, character_id: &str) -> Option<&CharacterProgressionData> {
        self.progression_data.get(character_id)
    }

    /// Get character progression data (mutable)
    pub fn get_character_progression_mut(&mut self, character_id: &str) -> Option<&mut CharacterProgressionData> {
        self.progression_data.get_mut(character_id)
    }

    /// Update character statistics
    pub fn update_statistics(&mut self, character_id: &str, statistics: ProgressionStatistics) -> Result<(), String> {
        let progression_data = self.progression_data.get_mut(character_id)
            .ok_or_else(|| format!("Character '{}' not found in progression data", character_id))?;

        progression_data.statistics = statistics;
        Ok(())
    }

    /// Get available stat points
    pub fn get_available_stat_points(&self, character_id: &str) -> Result<u32, String> {
        let progression_data = self.progression_data.get(character_id)
            .ok_or_else(|| format!("Character '{}' not found in progression data", character_id))?;

        Ok(progression_data.available_stat_points)
    }

    /// Get available ability points
    pub fn get_available_ability_points(&self, character_id: &str) -> Result<u32, String> {
        let progression_data = self.progression_data.get(character_id)
            .ok_or_else(|| format!("Character '{}' not found in progression data", character_id))?;

        Ok(progression_data.available_ability_points)
    }

    /// Get character achievements
    pub fn get_character_achievements(&self, character_id: &str) -> Result<Vec<&Achievement>, String> {
        let progression_data = self.progression_data.get(character_id)
            .ok_or_else(|| format!("Character '{}' not found in progression data", character_id))?;

        let achievements: Vec<&Achievement> = progression_data.completed_achievements.iter()
            .filter_map(|achievement_id| self.achievements.get(achievement_id))
            .collect();

        Ok(achievements)
    }

    /// Get character milestones
    pub fn get_character_milestones(&self, character_id: &str) -> Result<Vec<&Milestone>, String> {
        let progression_data = self.progression_data.get(character_id)
            .ok_or_else(|| format!("Character '{}' not found in progression data", character_id))?;

        let milestones: Vec<&Milestone> = progression_data.reached_milestones.iter()
            .filter_map(|milestone_id| self.milestones.get(milestone_id))
            .collect();

        Ok(milestones)
    }
}

/// Progression event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressionEvent {
    /// Event type
    pub event_type: ProgressionEventType,
    /// Character ID
    pub character_id: String,
    /// Timestamp
    pub timestamp: u64,
}

/// Progression event type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProgressionEventType {
    /// Level up event
    LevelUp {
        new_level: u32,
        stat_points_gained: u32,
        ability_points_gained: u32,
    },
    /// Achievement unlocked event
    AchievementUnlocked {
        achievement_id: String,
        rewards: AchievementReward,
    },
    /// Milestone reached event
    MilestoneReached {
        milestone_id: String,
        rewards: MilestoneReward,
    },
}

impl Default for ProgressionStatistics {
    fn default() -> Self {
        Self {
            enemies_killed: 0,
            levels_completed: 0,
            damage_dealt: 0,
            damage_taken: 0,
            healing_done: 0,
            items_collected: 0,
            time_played: 0.0,
            distance_traveled: 0.0,
            abilities_used: 0,
            deaths: 0,
            resurrections: 0,
            experience_gained: 0,
        }
    }
}

impl Default for CharacterProgression {
    fn default() -> Self {
        Self::new()
    }
}
