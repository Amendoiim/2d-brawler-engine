//! Advanced skill tree system with item-based unlocks and progression

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::game::items::{ItemType, EquipmentSlot};

/// Represents a skill node in the skill tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillNode {
    /// Unique identifier for the skill
    pub id: String,
    /// Display name of the skill
    pub name: String,
    /// Description of the skill
    pub description: String,
    /// Requirements to unlock this skill
    pub requirements: Vec<SkillRequirement>,
    /// Effects provided by this skill
    pub effects: Vec<SkillEffect>,
    /// Items required to unlock this skill
    pub item_unlocks: Vec<ItemRequirement>,
    /// Skill tier (1-5, higher tiers require more investment)
    pub tier: u32,
    /// Maximum level this skill can reach
    pub max_level: u32,
    /// Current level of this skill
    pub current_level: u32,
    /// Experience points invested in this skill
    pub experience_invested: u32,
    /// Whether this skill is unlocked
    pub is_unlocked: bool,
    /// Whether this skill is active
    pub is_active: bool,
}

/// Requirements to unlock a skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillRequirement {
    /// Character level requirement
    Level(u32),
    /// Previous skill requirement
    PreviousSkill(String),
    /// Item requirement
    Item(String),
    /// Equipment requirement
    Equipment(EquipmentSlot),
    /// Stat requirement
    Stat(StatType, u32),
    /// Experience requirement
    Experience(u32),
    /// Multiple requirements (all must be met)
    All(Vec<SkillRequirement>),
    /// Any requirement (at least one must be met)
    Any(Vec<SkillRequirement>),
}

/// Item requirements for skill unlocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRequirement {
    /// Item ID or type
    pub item_id: String,
    /// Required quantity
    pub quantity: u32,
    /// Whether the item must be equipped
    pub must_be_equipped: bool,
    /// Whether the item must be in inventory
    pub must_be_in_inventory: bool,
}

/// Effects provided by skills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillEffect {
    /// Stat bonus
    StatBonus(StatType, f32),
    /// Damage bonus
    DamageBonus(f32),
    /// Defense bonus
    DefenseBonus(f32),
    /// Speed bonus
    SpeedBonus(f32),
    /// Health bonus
    HealthBonus(f32),
    /// Mana bonus
    ManaBonus(f32),
    /// Stamina bonus
    StaminaBonus(f32),
    /// Critical hit chance bonus
    CriticalChanceBonus(f32),
    /// Critical hit damage bonus
    CriticalDamageBonus(f32),
    /// Ability cooldown reduction
    CooldownReduction(f32),
    /// Resource cost reduction
    ResourceCostReduction(f32),
    /// Special ability unlock
    AbilityUnlock(String),
    /// Passive effect
    PassiveEffect(PassiveEffectType),
    /// Equipment bonus
    EquipmentBonus(EquipmentSlot, f32),
}

/// Passive effect types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassiveEffectType {
    /// Health regeneration
    HealthRegeneration(f32),
    /// Mana regeneration
    ManaRegeneration(f32),
    /// Stamina regeneration
    StaminaRegeneration(f32),
    /// Damage over time resistance
    DamageOverTimeResistance(f32),
    /// Status effect resistance
    StatusEffectResistance(f32),
    /// Movement speed bonus
    MovementSpeedBonus(f32),
    /// Jump height bonus
    JumpHeightBonus(f32),
    /// Attack speed bonus
    AttackSpeedBonus(f32),
    /// Block chance bonus
    BlockChanceBonus(f32),
    /// Dodge chance bonus
    DodgeChanceBonus(f32),
    /// Parry chance bonus
    ParryChanceBonus(f32),
}

/// Stat types for skill requirements and effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatType {
    Strength,
    Dexterity,
    Intelligence,
    Constitution,
    Wisdom,
    Charisma,
    Health,
    Mana,
    Stamina,
    Attack,
    Defense,
    Speed,
    CriticalChance,
    CriticalDamage,
}

/// Skill tree progression system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTreeProgression {
    /// Available skill trees
    pub skill_trees: HashMap<String, SkillTree>,
    /// Unlocked skills across all trees
    pub unlocked_skills: HashSet<String>,
    /// Available skill points
    pub skill_points: u32,
    /// Total experience invested in skills
    pub total_experience_invested: u32,
    /// Item requirements for skill unlocks
    pub item_requirements: HashMap<String, Vec<ItemRequirement>>,
    /// Active skill effects
    pub active_effects: Vec<SkillEffect>,
}

/// Individual skill tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTree {
    /// Tree identifier
    pub id: String,
    /// Tree name
    pub name: String,
    /// Tree description
    pub description: String,
    /// Skills in this tree
    pub skills: HashMap<String, SkillNode>,
    /// Tree requirements
    pub requirements: Vec<SkillRequirement>,
    /// Whether this tree is unlocked
    pub is_unlocked: bool,
    /// Tree tier (affects skill availability)
    pub tier: u32,
}

/// Skill tree manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTreeManager {
    /// Progression data
    pub progression: SkillTreeProgression,
    /// Skill point costs for each tier
    pub skill_point_costs: HashMap<u32, u32>,
    /// Experience costs for skill levels
    pub experience_costs: HashMap<u32, u32>,
}

impl SkillTreeManager {
    /// Create a new skill tree manager
    pub fn new() -> Self {
        let mut manager = Self {
            progression: SkillTreeProgression {
                skill_trees: HashMap::new(),
                unlocked_skills: HashSet::new(),
                skill_points: 0,
                total_experience_invested: 0,
                item_requirements: HashMap::new(),
                active_effects: Vec::new(),
            },
            skill_point_costs: HashMap::new(),
            experience_costs: HashMap::new(),
        };
        
        // Initialize skill point costs (tier 1 = 1 point, tier 2 = 2 points, etc.)
        for tier in 1..=5 {
            manager.skill_point_costs.insert(tier, tier);
        }
        
        // Initialize experience costs (exponential growth)
        for level in 1..=20 {
            manager.experience_costs.insert(level, level * level * 100);
        }
        
        manager
    }
    
    /// Add a skill tree
    pub fn add_skill_tree(&mut self, tree: SkillTree) {
        self.progression.skill_trees.insert(tree.id.clone(), tree);
    }
    
    /// Unlock a skill tree
    pub fn unlock_skill_tree(&mut self, tree_id: &str) -> bool {
        let can_unlock = if let Some(tree) = self.progression.skill_trees.get(tree_id) {
            self.check_requirements(&tree.requirements)
        } else {
            false
        };
        
        if can_unlock {
            if let Some(tree) = self.progression.skill_trees.get_mut(tree_id) {
                tree.is_unlocked = true;
                return true;
            }
        }
        false
    }
    
    /// Unlock a skill
    pub fn unlock_skill(&mut self, tree_id: &str, skill_id: &str) -> bool {
        let can_unlock = if let Some(tree) = self.progression.skill_trees.get(tree_id) {
            if let Some(skill) = tree.skills.get(skill_id) {
                self.check_skill_requirements(skill)
            } else {
                false
            }
        } else {
            false
        };
        
        if can_unlock {
            if let Some(tree) = self.progression.skill_trees.get_mut(tree_id) {
                if let Some(skill) = tree.skills.get_mut(skill_id) {
                    skill.is_unlocked = true;
                    self.progression.unlocked_skills.insert(skill_id.to_string());
                    return true;
                }
            }
        }
        false
    }
    
    /// Level up a skill
    pub fn level_up_skill(&mut self, tree_id: &str, skill_id: &str, experience: u32) -> bool {
        let can_level_up = if let Some(tree) = self.progression.skill_trees.get(tree_id) {
            if let Some(skill) = tree.skills.get(skill_id) {
                skill.is_unlocked && skill.current_level < skill.max_level
            } else {
                false
            }
        } else {
            false
        };
        
        if can_level_up {
            let required_experience = if let Some(tree) = self.progression.skill_trees.get(tree_id) {
                if let Some(skill) = tree.skills.get(skill_id) {
                    self.get_skill_level_cost(skill.current_level + 1)
                } else {
                    return false;
                }
            } else {
                return false;
            };
            
            if experience >= required_experience {
                if let Some(tree) = self.progression.skill_trees.get_mut(tree_id) {
                    if let Some(skill) = tree.skills.get_mut(skill_id) {
                        skill.current_level += 1;
                        skill.experience_invested += required_experience;
                        self.progression.total_experience_invested += required_experience;
                        
                        // Update active effects
                        self.update_active_effects();
                        return true;
                    }
                }
            }
        }
        false
    }
    
    /// Activate a skill
    pub fn activate_skill(&mut self, tree_id: &str, skill_id: &str) -> bool {
        if let Some(tree) = self.progression.skill_trees.get_mut(tree_id) {
            if let Some(skill) = tree.skills.get_mut(skill_id) {
                if skill.is_unlocked && !skill.is_active {
                    skill.is_active = true;
                    self.update_active_effects();
                    return true;
                }
            }
        }
        false
    }
    
    /// Deactivate a skill
    pub fn deactivate_skill(&mut self, tree_id: &str, skill_id: &str) -> bool {
        if let Some(tree) = self.progression.skill_trees.get_mut(tree_id) {
            if let Some(skill) = tree.skills.get_mut(skill_id) {
                if skill.is_active {
                    skill.is_active = false;
                    self.update_active_effects();
                    return true;
                }
            }
        }
        false
    }
    
    /// Check if requirements are met
    fn check_requirements(&self, requirements: &[SkillRequirement]) -> bool {
        requirements.iter().all(|req| self.check_requirement(req))
    }
    
    /// Check a single requirement
    fn check_requirement(&self, requirement: &SkillRequirement) -> bool {
        match requirement {
            SkillRequirement::Level(required_level) => {
                // This would need to be connected to character level
                // For now, return true as placeholder
                true
            }
            SkillRequirement::PreviousSkill(skill_id) => {
                self.progression.unlocked_skills.contains(skill_id)
            }
            SkillRequirement::Item(item_id) => {
                // This would need to be connected to inventory system
                // For now, return true as placeholder
                true
            }
            SkillRequirement::Equipment(_slot) => {
                // This would need to be connected to equipment system
                // For now, return true as placeholder
                true
            }
            SkillRequirement::Stat(_stat_type, _value) => {
                // This would need to be connected to character stats
                // For now, return true as placeholder
                true
            }
            SkillRequirement::Experience(required_exp) => {
                self.progression.total_experience_invested >= *required_exp
            }
            SkillRequirement::All(requirements) => {
                requirements.iter().all(|req| self.check_requirement(req))
            }
            SkillRequirement::Any(requirements) => {
                requirements.iter().any(|req| self.check_requirement(req))
            }
        }
    }
    
    /// Check skill-specific requirements
    fn check_skill_requirements(&self, skill: &SkillNode) -> bool {
        self.check_requirements(&skill.requirements)
    }
    
    /// Get experience cost for a skill level
    fn get_skill_level_cost(&self, level: u32) -> u32 {
        self.experience_costs.get(&level).copied().unwrap_or(level * level * 100)
    }
    
    /// Update active effects based on active skills
    fn update_active_effects(&mut self) {
        self.progression.active_effects.clear();
        
        for tree in self.progression.skill_trees.values() {
            for skill in tree.skills.values() {
                if skill.is_active && skill.current_level > 0 {
                    for effect in &skill.effects {
                        // Apply level scaling to effects
                        let scaled_effect = self.scale_effect(effect, skill.current_level);
                        self.progression.active_effects.push(scaled_effect);
                    }
                }
            }
        }
    }
    
    /// Scale effect based on skill level
    fn scale_effect(&self, effect: &SkillEffect, level: u32) -> SkillEffect {
        match effect {
            SkillEffect::StatBonus(stat_type, value) => {
                SkillEffect::StatBonus(*stat_type, value * level as f32)
            }
            SkillEffect::DamageBonus(value) => {
                SkillEffect::DamageBonus(value * level as f32)
            }
            SkillEffect::DefenseBonus(value) => {
                SkillEffect::DefenseBonus(value * level as f32)
            }
            SkillEffect::SpeedBonus(value) => {
                SkillEffect::SpeedBonus(value * level as f32)
            }
            SkillEffect::HealthBonus(value) => {
                SkillEffect::HealthBonus(value * level as f32)
            }
            SkillEffect::ManaBonus(value) => {
                SkillEffect::ManaBonus(value * level as f32)
            }
            SkillEffect::StaminaBonus(value) => {
                SkillEffect::StaminaBonus(value * level as f32)
            }
            SkillEffect::CriticalChanceBonus(value) => {
                SkillEffect::CriticalChanceBonus(value * level as f32)
            }
            SkillEffect::CriticalDamageBonus(value) => {
                SkillEffect::CriticalDamageBonus(value * level as f32)
            }
            SkillEffect::CooldownReduction(value) => {
                SkillEffect::CooldownReduction(value * level as f32)
            }
            SkillEffect::ResourceCostReduction(value) => {
                SkillEffect::ResourceCostReduction(value * level as f32)
            }
            SkillEffect::AbilityUnlock(ability_id) => {
                SkillEffect::AbilityUnlock(ability_id.clone())
            }
            SkillEffect::PassiveEffect(passive_type) => {
                SkillEffect::PassiveEffect(passive_type.clone())
            }
            SkillEffect::EquipmentBonus(slot, value) => {
                SkillEffect::EquipmentBonus(slot.clone(), value * level as f32)
            }
        }
    }
    
    /// Get skill points cost for a skill
    pub fn get_skill_point_cost(&self, skill: &SkillNode) -> u32 {
        self.skill_point_costs.get(&skill.tier).copied().unwrap_or(skill.tier)
    }
    
    /// Add skill points
    pub fn add_skill_points(&mut self, points: u32) {
        self.progression.skill_points += points;
    }
    
    /// Spend skill points
    pub fn spend_skill_points(&mut self, points: u32) -> bool {
        if self.progression.skill_points >= points {
            self.progression.skill_points -= points;
            true
        } else {
            false
        }
    }
    
    /// Get total stat bonuses from active skills
    pub fn get_stat_bonuses(&self) -> HashMap<StatType, f32> {
        let mut bonuses = HashMap::new();
        
        for effect in &self.progression.active_effects {
            if let SkillEffect::StatBonus(stat_type, value) = effect {
                *bonuses.entry(*stat_type).or_insert(0.0) += value;
            }
        }
        
        bonuses
    }
    
    /// Get all active effects
    pub fn get_active_effects(&self) -> &[SkillEffect] {
        &self.progression.active_effects
    }
    
    /// Get skill by ID
    pub fn get_skill(&self, tree_id: &str, skill_id: &str) -> Option<&SkillNode> {
        self.progression.skill_trees.get(tree_id)?.skills.get(skill_id)
    }
    
    /// Get skill tree by ID
    pub fn get_skill_tree(&self, tree_id: &str) -> Option<&SkillTree> {
        self.progression.skill_trees.get(tree_id)
    }
    
    /// Get all skill trees
    pub fn get_skill_trees(&self) -> &HashMap<String, SkillTree> {
        &self.progression.skill_trees
    }
    
    /// Get progression statistics
    pub fn get_progression_stats(&self) -> SkillProgressionStats {
        let total_skills = self.progression.skill_trees.values()
            .map(|tree| tree.skills.len())
            .sum::<usize>();
        
        let unlocked_skills = self.progression.unlocked_skills.len();
        let active_skills = self.progression.skill_trees.values()
            .flat_map(|tree| tree.skills.values())
            .filter(|skill| skill.is_active)
            .count();
        
        SkillProgressionStats {
            total_skills,
            unlocked_skills,
            active_skills,
            skill_points: self.progression.skill_points,
            total_experience_invested: self.progression.total_experience_invested,
        }
    }
}

/// Skill progression statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillProgressionStats {
    pub total_skills: usize,
    pub unlocked_skills: usize,
    pub active_skills: usize,
    pub skill_points: u32,
    pub total_experience_invested: u32,
}

impl Default for SkillTreeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_tree_manager_creation() {
        let manager = SkillTreeManager::new();
        assert_eq!(manager.progression.skill_points, 0);
        assert_eq!(manager.progression.total_experience_invested, 0);
        assert!(manager.progression.skill_trees.is_empty());
    }

    #[test]
    fn test_skill_point_costs() {
        let manager = SkillTreeManager::new();
        assert_eq!(manager.skill_point_costs.get(&1), Some(&1));
        assert_eq!(manager.skill_point_costs.get(&5), Some(&5));
    }

    #[test]
    fn test_experience_costs() {
        let manager = SkillTreeManager::new();
        assert_eq!(manager.experience_costs.get(&1), Some(&100));
        assert_eq!(manager.experience_costs.get(&5), Some(&2500));
    }

    #[test]
    fn test_add_skill_points() {
        let mut manager = SkillTreeManager::new();
        manager.add_skill_points(10);
        assert_eq!(manager.progression.skill_points, 10);
    }

    #[test]
    fn test_spend_skill_points() {
        let mut manager = SkillTreeManager::new();
        manager.add_skill_points(10);
        
        assert!(manager.spend_skill_points(5));
        assert_eq!(manager.progression.skill_points, 5);
        
        assert!(!manager.spend_skill_points(10));
        assert_eq!(manager.progression.skill_points, 5);
    }
}
