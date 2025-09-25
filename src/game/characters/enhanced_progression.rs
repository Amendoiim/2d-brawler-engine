//! Enhanced character progression system with item integration

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game::items::{ItemManager, Item, ItemType, EquipmentSlot, ItemStats};
use crate::game::characters::skill_trees::{SkillTreeManager, StatType, SkillEffect};
use crate::game::characters::{Character, CharacterStats, CharacterClass};

/// Enhanced experience system with item bonuses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedExperienceSystem {
    /// Base experience points
    pub base_experience: u32,
    /// Experience from items
    pub item_experience: u32,
    /// Experience from equipment
    pub equipment_experience: u32,
    /// Experience from skills
    pub skill_experience: u32,
    /// Total experience
    pub total_experience: u32,
    /// Current level
    pub current_level: u32,
    /// Experience required for next level
    pub experience_to_next_level: u32,
    /// Item bonus multipliers
    pub item_bonuses: HashMap<String, f32>,
    /// Equipment multipliers
    pub equipment_multipliers: HashMap<EquipmentSlot, f32>,
    /// Skill tree progression
    pub skill_tree_progression: SkillTreeManager,
}

/// Experience bonus sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperienceBonusSource {
    /// Item-based bonus
    Item(String, f32),
    /// Equipment-based bonus
    Equipment(EquipmentSlot, f32),
    /// Skill-based bonus
    Skill(String, f32),
    /// Class-based bonus
    Class(CharacterClass, f32),
    /// Event-based bonus
    Event(String, f32),
}

/// Level progression data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelProgression {
    /// Level number
    pub level: u32,
    /// Experience required for this level
    pub experience_required: u32,
    /// Stat points gained at this level
    pub stat_points: u32,
    /// Skill points gained at this level
    pub skill_points: u32,
    /// Ability points gained at this level
    pub ability_points: u32,
    /// Special rewards at this level
    pub special_rewards: Vec<LevelReward>,
}

/// Special rewards for leveling up
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LevelReward {
    /// Unlock new skill tree
    SkillTreeUnlock(String),
    /// Unlock new ability
    AbilityUnlock(String),
    /// Unlock new equipment slot
    EquipmentSlotUnlock(EquipmentSlot),
    /// Unlock new item type
    ItemTypeUnlock(ItemType),
    /// Permanent stat bonus
    PermanentStatBonus(StatType, f32),
    /// Special title
    Title(String),
    /// Achievement
    Achievement(String),
}

/// Enhanced progression manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedProgressionManager {
    /// Experience system
    pub experience_system: EnhancedExperienceSystem,
    /// Level progression data
    pub level_progression: HashMap<u32, LevelProgression>,
    /// Experience bonus sources
    pub bonus_sources: Vec<ExperienceBonusSource>,
    /// Progression statistics
    pub statistics: ProgressionStatistics,
}

/// Progression statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionStatistics {
    /// Total experience gained
    pub total_experience_gained: u32,
    /// Experience from items
    pub experience_from_items: u32,
    /// Experience from equipment
    pub experience_from_equipment: u32,
    /// Experience from skills
    pub experience_from_skills: u32,
    /// Total levels gained
    pub total_levels_gained: u32,
    /// Items used for progression
    pub items_used: HashMap<String, u32>,
    /// Equipment pieces used
    pub equipment_used: HashMap<EquipmentSlot, u32>,
    /// Skills unlocked
    pub skills_unlocked: u32,
    /// Abilities unlocked
    pub abilities_unlocked: u32,
}

impl EnhancedExperienceSystem {
    /// Create a new enhanced experience system
    pub fn new() -> Self {
        Self {
            base_experience: 0,
            item_experience: 0,
            equipment_experience: 0,
            skill_experience: 0,
            total_experience: 0,
            current_level: 1,
            experience_to_next_level: 1000,
            item_bonuses: HashMap::new(),
            equipment_multipliers: HashMap::new(),
            skill_tree_progression: SkillTreeManager::new(),
        }
    }
    
    /// Add experience from various sources
    pub fn add_experience(&mut self, amount: u32, source: ExperienceSource) {
        let mut bonus_multiplier = 1.0;
        
        // Apply item bonuses
        for (item_id, bonus) in &self.item_bonuses {
            bonus_multiplier += bonus;
        }
        
        // Apply equipment multipliers
        for (slot, multiplier) in &self.equipment_multipliers {
            bonus_multiplier += multiplier;
        }
        
        // Apply skill bonuses
        for effect in self.skill_tree_progression.get_active_effects() {
            if let SkillEffect::StatBonus(StatType::Intelligence, bonus) = effect {
                bonus_multiplier += bonus * 0.01; // 1% per intelligence point
            }
        }
        
        let final_amount = (amount as f32 * bonus_multiplier) as u32;
        
        match source {
            ExperienceSource::Base => {
                self.base_experience += final_amount;
            }
            ExperienceSource::Item(_item_id) => {
                self.item_experience += final_amount;
                // Track item usage
            }
            ExperienceSource::Equipment(_slot) => {
                self.equipment_experience += final_amount;
                // Track equipment usage
            }
            ExperienceSource::Skill(_skill_id) => {
                self.skill_experience += final_amount;
                // Track skill usage
            }
            ExperienceSource::Combat => {
                self.base_experience += final_amount;
            }
            ExperienceSource::Quest => {
                self.base_experience += final_amount;
            }
            ExperienceSource::Exploration => {
                self.base_experience += final_amount;
            }
        }
        
        self.total_experience = self.base_experience + self.item_experience + 
                               self.equipment_experience + self.skill_experience;
        
        self.check_level_up();
    }
    
    /// Check if character should level up
    fn check_level_up(&mut self) {
        while self.total_experience >= self.experience_to_next_level {
            self.level_up();
        }
    }
    
    /// Level up the character
    fn level_up(&mut self) {
        self.current_level += 1;
        self.experience_to_next_level = self.calculate_experience_for_level(self.current_level + 1);
        
        // Add skill points based on level
        let skill_points_gained = self.calculate_skill_points_for_level(self.current_level);
        self.skill_tree_progression.add_skill_points(skill_points_gained);
    }
    
    /// Calculate experience required for a level
    fn calculate_experience_for_level(&self, level: u32) -> u32 {
        // Exponential growth: base * (level ^ 1.5)
        let base_experience = 1000.0;
        (base_experience * (level as f32).powf(1.5)) as u32
    }
    
    /// Calculate skill points gained at a level
    fn calculate_skill_points_for_level(&self, level: u32) -> u32 {
        // Every 2 levels, gain 1 skill point
        if level % 2 == 0 { 1 } else { 0 }
    }
    
    /// Add item bonus
    pub fn add_item_bonus(&mut self, item_id: String, bonus: f32) {
        self.item_bonuses.insert(item_id, bonus);
    }
    
    /// Remove item bonus
    pub fn remove_item_bonus(&mut self, item_id: &str) {
        self.item_bonuses.remove(item_id);
    }
    
    /// Add equipment multiplier
    pub fn add_equipment_multiplier(&mut self, slot: EquipmentSlot, multiplier: f32) {
        self.equipment_multipliers.insert(slot, multiplier);
    }
    
    /// Remove equipment multiplier
    pub fn remove_equipment_multiplier(&mut self, slot: &EquipmentSlot) {
        self.equipment_multipliers.remove(slot);
    }
    
    /// Get total experience bonus multiplier
    pub fn get_total_bonus_multiplier(&self) -> f32 {
        let mut multiplier = 1.0;
        
        for bonus in self.item_bonuses.values() {
            multiplier += bonus;
        }
        
        for equipment_multiplier in self.equipment_multipliers.values() {
            multiplier += equipment_multiplier;
        }
        
        multiplier
    }
    
    /// Get experience progress to next level
    pub fn get_level_progress(&self) -> f32 {
        let current_level_exp = self.calculate_experience_for_level(self.current_level);
        let next_level_exp = self.calculate_experience_for_level(self.current_level + 1);
        let progress_exp = self.total_experience - current_level_exp;
        let required_exp = next_level_exp - current_level_exp;
        
        if required_exp > 0 {
            progress_exp as f32 / required_exp as f32
        } else {
            1.0
        }
    }
}

/// Experience source types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExperienceSource {
    Base,
    Item(String),
    Equipment(EquipmentSlot),
    Skill(String),
    Combat,
    Quest,
    Exploration,
}

impl ExperienceSource {
    /// Get experience source from item
    pub fn from_item(item: &Item) -> Self {
        match &item.item_type {
            ItemType::Consumable(_) => ExperienceSource::Item(item.id.clone()),
            ItemType::Weapon(_) => ExperienceSource::Equipment(EquipmentSlot::MainHand),
            ItemType::Armor(_) => ExperienceSource::Equipment(EquipmentSlot::Helmet), // Default to helmet
            ItemType::Accessory(_) => ExperienceSource::Equipment(EquipmentSlot::Ring1), // Default to ring
            _ => ExperienceSource::Item(item.id.clone()),
        }
    }
}

impl EnhancedProgressionManager {
    /// Create a new enhanced progression manager
    pub fn new() -> Self {
        let mut manager = Self {
            experience_system: EnhancedExperienceSystem::new(),
            level_progression: HashMap::new(),
            bonus_sources: Vec::new(),
            statistics: ProgressionStatistics {
                total_experience_gained: 0,
                experience_from_items: 0,
                experience_from_equipment: 0,
                experience_from_skills: 0,
                total_levels_gained: 0,
                items_used: HashMap::new(),
                equipment_used: HashMap::new(),
                skills_unlocked: 0,
                abilities_unlocked: 0,
            },
        };
        
        // Initialize level progression data
        manager.initialize_level_progression();
        manager
    }
    
    /// Initialize level progression data
    fn initialize_level_progression(&mut self) {
        for level in 1..=100 {
            let progression = LevelProgression {
                level,
                experience_required: self.experience_system.calculate_experience_for_level(level),
                stat_points: if level % 2 == 0 { 2 } else { 1 },
                skill_points: if level % 3 == 0 { 1 } else { 0 },
                ability_points: if level % 5 == 0 { 1 } else { 0 },
                special_rewards: self.get_special_rewards_for_level(level),
            };
            self.level_progression.insert(level, progression);
        }
    }
    
    /// Get special rewards for a level
    fn get_special_rewards_for_level(&self, level: u32) -> Vec<LevelReward> {
        let mut rewards = Vec::new();
        
        // Special rewards at certain levels
        match level {
            5 => rewards.push(LevelReward::SkillTreeUnlock("combat_basics".to_string())),
            10 => rewards.push(LevelReward::AbilityUnlock("special_attack".to_string())),
            15 => rewards.push(LevelReward::EquipmentSlotUnlock(EquipmentSlot::Ring1)),
            20 => rewards.push(LevelReward::ItemTypeUnlock(ItemType::Accessory(crate::game::items::AccessoryType::Ring))),
            25 => rewards.push(LevelReward::SkillTreeUnlock("advanced_combat".to_string())),
            30 => rewards.push(LevelReward::Title("Veteran".to_string())),
            50 => rewards.push(LevelReward::Achievement("Half Century".to_string())),
            100 => rewards.push(LevelReward::Achievement("Century Master".to_string())),
            _ => {}
        }
        
        rewards
    }
    
    /// Add experience from various sources
    pub fn add_experience(&mut self, amount: u32, source: ExperienceSource) {
        self.experience_system.add_experience(amount, source.clone());
        self.update_statistics(amount, source);
    }
    
    /// Update progression statistics
    fn update_statistics(&mut self, amount: u32, source: ExperienceSource) {
        self.statistics.total_experience_gained += amount;
        
        match source {
            ExperienceSource::Item(_) => {
                self.statistics.experience_from_items += amount;
            }
            ExperienceSource::Equipment(_) => {
                self.statistics.experience_from_equipment += amount;
            }
            ExperienceSource::Skill(_) => {
                self.statistics.experience_from_skills += amount;
            }
            _ => {}
        }
    }
    
    /// Apply item bonuses to experience system
    pub fn apply_item_bonuses(&mut self, character: &Character, item_manager: &ItemManager) {
        // Clear existing bonuses
        self.experience_system.item_bonuses.clear();
        
        // Apply bonuses from equipped items
        for (slot, item_id) in &character.equipment.slots {
            if let Some(item_id) = item_id {
                if let Some(item) = item_manager.get_item(item_id) {
                    if let Some(experience_bonus) = item.experience_bonus {
                        self.experience_system.add_item_bonus(
                            item_id.clone(),
                            experience_bonus
                        );
                    }
                }
            }
        }
        
        // Apply bonuses from inventory items
        for (slot_index, slot) in &character.inventory.items {
            if let Some(item) = item_manager.get_item(&slot.item_id) {
                if let Some(experience_bonus) = item.experience_bonus {
                    self.experience_system.add_item_bonus(
                        slot.item_id.clone(),
                        experience_bonus
                    );
                }
            }
        }
    }
    
    /// Apply equipment multipliers
    pub fn apply_equipment_multipliers(&mut self, character: &Character, item_manager: &ItemManager) {
        // Clear existing multipliers
        self.experience_system.equipment_multipliers.clear();
        
        // Apply multipliers from equipped items
        for (slot, item_id) in &character.equipment.slots {
            if let Some(item_id) = item_id {
                if let Some(item) = item_manager.get_item(item_id) {
                    if let Some(experience_multiplier) = item.experience_multiplier {
                        self.experience_system.add_equipment_multiplier(
                            slot.clone(),
                            experience_multiplier
                        );
                    }
                }
            }
        }
    }
    
    /// Get enhanced character stats with item and skill bonuses
    pub fn get_enhanced_stats(&self, base_stats: &CharacterStats) -> CharacterStats {
        let mut enhanced_stats = base_stats.clone();
        
        // Apply skill bonuses
        let skill_bonuses = self.experience_system.skill_tree_progression.get_stat_bonuses();
        for (stat_type, bonus) in skill_bonuses {
            match stat_type {
                StatType::Strength => enhanced_stats.strength += bonus as u32,
                StatType::Dexterity => enhanced_stats.agility += bonus as u32, // Map Dexterity to Agility
                StatType::Intelligence => enhanced_stats.intelligence += bonus as u32,
                StatType::Constitution => enhanced_stats.vitality += bonus as u32, // Map Constitution to Vitality
                StatType::Health => enhanced_stats.vitality += bonus as u32, // Map Health to Vitality
                StatType::Mana => enhanced_stats.intelligence += bonus as u32, // Map Mana to Intelligence
                StatType::Stamina => enhanced_stats.vitality += bonus as u32, // Map Stamina to Vitality
                StatType::Attack => enhanced_stats.strength += bonus as u32, // Map Attack to Strength
                StatType::Defense => enhanced_stats.vitality += bonus as u32, // Map Defense to Vitality
                StatType::Speed => enhanced_stats.agility += bonus as u32, // Map Speed to Agility
                _ => {} // Other stats not directly mapped to CharacterStats
            }
        }
        
        enhanced_stats
    }
    
    /// Get progression statistics
    pub fn get_statistics(&self) -> &ProgressionStatistics {
        &self.statistics
    }
    
    /// Get level progression data
    pub fn get_level_progression(&self, level: u32) -> Option<&LevelProgression> {
        self.level_progression.get(&level)
    }
    
    /// Get current level progress
    pub fn get_current_level_progress(&self) -> f32 {
        self.experience_system.get_level_progress()
    }
    
    /// Get total bonus multiplier
    pub fn get_total_bonus_multiplier(&self) -> f32 {
        self.experience_system.get_total_bonus_multiplier()
    }
}

impl Default for EnhancedProgressionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_experience_system_creation() {
        let system = EnhancedExperienceSystem::new();
        assert_eq!(system.base_experience, 0);
        assert_eq!(system.current_level, 1);
        assert_eq!(system.experience_to_next_level, 1000);
    }

    #[test]
    fn test_add_experience() {
        let mut system = EnhancedExperienceSystem::new();
        system.add_experience(500, ExperienceSource::Combat);
        assert_eq!(system.base_experience, 500);
        assert_eq!(system.total_experience, 500);
    }

    #[test]
    fn test_level_up() {
        let mut system = EnhancedExperienceSystem::new();
        system.add_experience(1000, ExperienceSource::Combat);
        assert_eq!(system.current_level, 2);
    }

    #[test]
    fn test_item_bonus() {
        let mut system = EnhancedExperienceSystem::new();
        system.add_item_bonus("test_item".to_string(), 0.5);
        assert_eq!(system.item_bonuses.get("test_item"), Some(&0.5));
    }

    #[test]
    fn test_equipment_multiplier() {
        let mut system = EnhancedExperienceSystem::new();
        system.add_equipment_multiplier(EquipmentSlot::Weapon, 0.25);
        assert_eq!(system.equipment_multipliers.get(&EquipmentSlot::Weapon), Some(&0.25));
    }
}
