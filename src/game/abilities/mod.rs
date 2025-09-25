//! Character abilities system implementation

use crate::engine::ecs::{Component, System, World};
use std::collections::HashMap;

// Import ability modules
pub mod character_abilities;
pub mod effects;

// Re-export for easy access
pub use character_abilities::*;
pub use effects::*;

// Implement Component trait for ability components
impl Component for Ability {}
impl Component for AbilityTree {}
impl Component for CharacterAbilities {}

/// Individual ability definition
#[derive(Clone)]
pub struct Ability {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: u32,
    pub max_level: u32,
    pub cost: u32,
    pub cooldown: f32,
    pub resource_cost: f32,
    pub resource_type: ResourceType,
    pub effects: Vec<AbilityEffect>,
    pub requirements: Vec<AbilityRequirement>,
    pub is_unlocked: bool,
    pub is_active: bool,
}

impl Ability {
    pub fn new(id: String, name: String, description: String, max_level: u32, cost: u32) -> Self {
        Self {
            id,
            name,
            description,
            level: 0,
            max_level,
            cost,
            cooldown: 0.0,
            resource_cost: 0.0,
            resource_type: ResourceType::Mana,
            effects: Vec::new(),
            requirements: Vec::new(),
            is_unlocked: false,
            is_active: false,
        }
    }

    pub fn can_level_up(&self, available_points: u32) -> bool {
        self.level < self.max_level && available_points >= self.cost
    }

    pub fn level_up(&mut self) -> bool {
        if self.level < self.max_level {
            self.level += 1;
            true
        } else {
            false
        }
    }

    pub fn can_use(&self, available_resources: &HashMap<ResourceType, f32>) -> bool {
        if !self.is_unlocked || self.is_active {
            return false;
        }

        available_resources.get(&self.resource_type)
            .map_or(false, |&amount| amount >= self.resource_cost)
    }

    pub fn activate(&mut self) -> bool {
        if self.can_use(&HashMap::new()) { // Simplified - would need actual resource check
            self.is_active = true;
            true
        } else {
            false
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

/// Ability tree for character progression
#[derive(Clone)]
pub struct AbilityTree {
    pub character_class: String,
    pub abilities: HashMap<String, Ability>,
    pub connections: Vec<AbilityConnection>,
    pub max_points: u32,
    pub used_points: u32,
}

impl AbilityTree {
    pub fn new(character_class: String) -> Self {
        Self {
            character_class,
            abilities: HashMap::new(),
            connections: Vec::new(),
            max_points: 0,
            used_points: 0,
        }
    }

    pub fn add_ability(&mut self, ability: Ability) {
        self.abilities.insert(ability.id.clone(), ability);
    }

    pub fn add_connection(&mut self, from: String, to: String) {
        self.connections.push(AbilityConnection { from, to });
    }

    pub fn can_unlock_ability(&self, ability_id: &str) -> bool {
        if let Some(ability) = self.abilities.get(ability_id) {
            if ability.is_unlocked {
                return false; // Already unlocked
            }

            // Check if all requirements are met
            for requirement in &ability.requirements {
                if !self.meets_requirement(requirement) {
                    return false;
                }
            }

            // Check if we have enough points
            if self.used_points + ability.cost > self.max_points {
                return false;
            }

            true
        } else {
            false
        }
    }

    pub fn unlock_ability(&mut self, ability_id: &str) -> bool {
        if self.can_unlock_ability(ability_id) {
            if let Some(ability) = self.abilities.get_mut(ability_id) {
                ability.is_unlocked = true;
                self.used_points += ability.cost;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn meets_requirement(&self, requirement: &AbilityRequirement) -> bool {
        match requirement {
            AbilityRequirement::Level { min_level } => {
                // Would need character level - simplified
                true
            },
            AbilityRequirement::Ability { ability_id, min_level } => {
                self.abilities.get(ability_id)
                    .map_or(false, |ability| ability.level >= *min_level)
            },
            AbilityRequirement::CharacterClass { class } => {
                self.character_class == *class
            },
            AbilityRequirement::None => true,
        }
    }

    pub fn get_available_abilities(&self) -> Vec<&Ability> {
        self.abilities.values()
            .filter(|ability| self.can_unlock_ability(&ability.id))
            .collect()
    }

    pub fn get_unlocked_abilities(&self) -> Vec<&Ability> {
        self.abilities.values()
            .filter(|ability| ability.is_unlocked)
            .collect()
    }
}

/// Connection between abilities in the tree
#[derive(Clone)]
pub struct AbilityConnection {
    pub from: String,
    pub to: String,
}

/// Requirements for unlocking abilities
#[derive(Clone)]
pub enum AbilityRequirement {
    Level { min_level: u32 },
    Ability { ability_id: String, min_level: u32 },
    CharacterClass { class: String },
    None,
}

/// Resource types for abilities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ResourceType {
    Mana,
    Stamina,
    Rage,
    Energy,
    Focus,
}

/// Character abilities component
#[derive(Clone)]
pub struct CharacterAbilities {
    pub ability_trees: HashMap<String, AbilityTree>,
    pub active_abilities: Vec<String>,
    pub ability_points: u32,
    pub total_ability_points: u32,
}

impl CharacterAbilities {
    pub fn new() -> Self {
        Self {
            ability_trees: HashMap::new(),
            active_abilities: Vec::new(),
            ability_points: 0,
            total_ability_points: 0,
        }
    }

    pub fn add_ability_tree(&mut self, tree: AbilityTree) {
        self.ability_trees.insert(tree.character_class.clone(), tree);
    }

    pub fn get_ability_tree(&self, character_class: &str) -> Option<&AbilityTree> {
        self.ability_trees.get(character_class)
    }

    pub fn get_ability_tree_mut(&mut self, character_class: &str) -> Option<&mut AbilityTree> {
        self.ability_trees.get_mut(character_class)
    }

    pub fn add_ability_points(&mut self, points: u32) {
        self.ability_points += points;
        self.total_ability_points += points;
    }

    pub fn spend_ability_points(&mut self, points: u32) -> bool {
        if self.ability_points >= points {
            self.ability_points -= points;
            true
        } else {
            false
        }
    }

    pub fn activate_ability(&mut self, character_class: &str, ability_id: &str) -> bool {
        if let Some(tree) = self.ability_trees.get_mut(character_class) {
            if let Some(ability) = tree.abilities.get_mut(ability_id) {
                if ability.activate() {
                    self.active_abilities.push(ability_id.to_string());
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn deactivate_ability(&mut self, character_class: &str, ability_id: &str) -> bool {
        if let Some(tree) = self.ability_trees.get_mut(character_class) {
            if let Some(ability) = tree.abilities.get_mut(ability_id) {
                ability.deactivate();
                self.active_abilities.retain(|id| id != ability_id);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// Ability system for processing abilities
pub struct AbilitySystem;

impl System for AbilitySystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // Update ability cooldowns
        // Process ability effects
        // Handle ability activation/deactivation
        // Apply ability bonuses
    }
}

/// Ability manager for handling all abilities
pub struct AbilityManager {
    pub ability_definitions: HashMap<String, AbilityDefinition>,
    pub character_ability_trees: HashMap<String, AbilityTree>,
}

impl AbilityManager {
    pub fn new() -> Self {
        let mut manager = Self {
            ability_definitions: HashMap::new(),
            character_ability_trees: HashMap::new(),
        };
        manager.initialize_abilities();
        manager
    }

    fn initialize_abilities(&mut self) {
        // Initialize ability definitions for each character class
        self.initialize_fighter_abilities();
        self.initialize_ranger_abilities();
        self.initialize_mage_abilities();
        self.initialize_tank_abilities();
        self.initialize_assassin_abilities();
    }

    fn initialize_fighter_abilities(&mut self) {
        let mut tree = AbilityTree::new("Fighter".to_string());
        tree.max_points = 50;

        // Basic abilities
        let mut power_strike = Ability::new(
            "power_strike".to_string(),
            "Power Strike".to_string(),
            "Increases melee damage".to_string(),
            5,
            1,
        );
        power_strike.effects.push(AbilityEffect::DamageBonus { multiplier: 0.2 });
        tree.add_ability(power_strike);

        let mut heavy_armor = Ability::new(
            "heavy_armor".to_string(),
            "Heavy Armor".to_string(),
            "Increases defense and reduces movement speed".to_string(),
            3,
            2,
        );
        heavy_armor.effects.push(AbilityEffect::DefenseBonus { multiplier: 0.3 });
        heavy_armor.effects.push(AbilityEffect::SpeedPenalty { multiplier: -0.1 });
        tree.add_ability(heavy_armor);

        // Advanced abilities
        let mut whirlwind = Ability::new(
            "whirlwind".to_string(),
            "Whirlwind Attack".to_string(),
            "Spin attack that hits all enemies in range".to_string(),
            3,
            3,
        );
        whirlwind.requirements.push(AbilityRequirement::Ability {
            ability_id: "power_strike".to_string(),
            min_level: 3,
        });
        whirlwind.effects.push(AbilityEffect::AreaAttack { radius: 200.0 });
        tree.add_ability(whirlwind);

        self.character_ability_trees.insert("Fighter".to_string(), tree);
    }

    fn initialize_ranger_abilities(&mut self) {
        let mut tree = AbilityTree::new("Ranger".to_string());
        tree.max_points = 50;

        // Basic abilities
        let mut quick_shot = Ability::new(
            "quick_shot".to_string(),
            "Quick Shot".to_string(),
            "Increases ranged attack speed".to_string(),
            5,
            1,
        );
        quick_shot.effects.push(AbilityEffect::AttackSpeedBonus { multiplier: 0.25 });
        tree.add_ability(quick_shot);

        let mut eagle_eye = Ability::new(
            "eagle_eye".to_string(),
            "Eagle Eye".to_string(),
            "Increases ranged attack range and accuracy".to_string(),
            3,
            2,
        );
        eagle_eye.effects.push(AbilityEffect::RangeBonus { multiplier: 0.5 });
        eagle_eye.effects.push(AbilityEffect::AccuracyBonus { multiplier: 0.3 });
        tree.add_ability(eagle_eye);

        // Advanced abilities
        let mut explosive_arrow = Ability::new(
            "explosive_arrow".to_string(),
            "Explosive Arrow".to_string(),
            "Arrow that explodes on impact".to_string(),
            3,
            3,
        );
        explosive_arrow.requirements.push(AbilityRequirement::Ability {
            ability_id: "eagle_eye".to_string(),
            min_level: 2,
        });
        explosive_arrow.effects.push(AbilityEffect::ExplosiveDamage { radius: 150.0 });
        tree.add_ability(explosive_arrow);

        self.character_ability_trees.insert("Ranger".to_string(), tree);
    }

    fn initialize_mage_abilities(&mut self) {
        let mut tree = AbilityTree::new("Mage".to_string());
        tree.max_points = 50;

        // Basic abilities
        let mut mana_mastery = Ability::new(
            "mana_mastery".to_string(),
            "Mana Mastery".to_string(),
            "Increases mana pool and regeneration".to_string(),
            5,
            1,
        );
        mana_mastery.effects.push(AbilityEffect::ManaBonus { multiplier: 0.3 });
        mana_mastery.effects.push(AbilityEffect::ManaRegenBonus { multiplier: 0.5 });
        tree.add_ability(mana_mastery);

        let mut fire_magic = Ability::new(
            "fire_magic".to_string(),
            "Fire Magic".to_string(),
            "Increases fire spell damage".to_string(),
            5,
            1,
        );
        fire_magic.effects.push(AbilityEffect::ElementalDamageBonus {
            element: effects::ElementType::Fire,
            multiplier: 0.4,
        });
        tree.add_ability(fire_magic);

        // Advanced abilities
        let mut meteor = Ability::new(
            "meteor".to_string(),
            "Meteor".to_string(),
            "Summon a meteor from the sky".to_string(),
            3,
            5,
        );
        meteor.requirements.push(AbilityRequirement::Ability {
            ability_id: "fire_magic".to_string(),
            min_level: 3,
        });
        meteor.effects.push(AbilityEffect::MeteorStrike { damage: 300.0, radius: 200.0 });
        tree.add_ability(meteor);

        self.character_ability_trees.insert("Mage".to_string(), tree);
    }

    fn initialize_tank_abilities(&mut self) {
        let mut tree = AbilityTree::new("Tank".to_string());
        tree.max_points = 50;

        // Basic abilities
        let mut shield_mastery = Ability::new(
            "shield_mastery".to_string(),
            "Shield Mastery".to_string(),
            "Increases block effectiveness and reduces stamina cost".to_string(),
            5,
            1,
        );
        shield_mastery.effects.push(AbilityEffect::BlockBonus { multiplier: 0.3 });
        shield_mastery.effects.push(AbilityEffect::StaminaCostReduction { multiplier: 0.2 });
        tree.add_ability(shield_mastery);

        let mut taunt = Ability::new(
            "taunt".to_string(),
            "Taunt".to_string(),
            "Force enemies to attack you".to_string(),
            3,
            2,
        );
        taunt.effects.push(AbilityEffect::Taunt { range: 300.0, duration: 5.0 });
        tree.add_ability(taunt);

        // Advanced abilities
        let mut shield_wall = Ability::new(
            "shield_wall".to_string(),
            "Shield Wall".to_string(),
            "Create a protective barrier".to_string(),
            3,
            4,
        );
        shield_wall.requirements.push(AbilityRequirement::Ability {
            ability_id: "shield_mastery".to_string(),
            min_level: 3,
        });
        shield_wall.effects.push(AbilityEffect::ShieldWall { duration: 10.0, damage_reduction: 0.8 });
        tree.add_ability(shield_wall);

        self.character_ability_trees.insert("Tank".to_string(), tree);
    }

    fn initialize_assassin_abilities(&mut self) {
        let mut tree = AbilityTree::new("Assassin".to_string());
        tree.max_points = 50;

        // Basic abilities
        let mut stealth = Ability::new(
            "stealth".to_string(),
            "Stealth".to_string(),
            "Become invisible for a short time".to_string(),
            5,
            1,
        );
        stealth.effects.push(AbilityEffect::Invisibility { duration: 3.0 });
        tree.add_ability(stealth);

        let mut critical_strike = Ability::new(
            "critical_strike".to_string(),
            "Critical Strike".to_string(),
            "Increases critical hit chance and damage".to_string(),
            5,
            1,
        );
        critical_strike.effects.push(AbilityEffect::CriticalChanceBonus { multiplier: 0.2 });
        critical_strike.effects.push(AbilityEffect::CriticalDamageBonus { multiplier: 0.5 });
        tree.add_ability(critical_strike);

        // Advanced abilities
        let mut shadow_strike = Ability::new(
            "shadow_strike".to_string(),
            "Shadow Strike".to_string(),
            "Teleport behind enemy and attack".to_string(),
            3,
            4,
        );
        shadow_strike.requirements.push(AbilityRequirement::Ability {
            ability_id: "stealth".to_string(),
            min_level: 3,
        });
        shadow_strike.effects.push(AbilityEffect::TeleportAttack { damage: 250.0 });
        tree.add_ability(shadow_strike);

        self.character_ability_trees.insert("Assassin".to_string(), tree);
    }

    pub fn get_character_ability_tree(&self, character_class: &str) -> Option<&AbilityTree> {
        self.character_ability_trees.get(character_class)
    }

    pub fn create_character_abilities(&self, character_class: &str) -> CharacterAbilities {
        let mut character_abilities = CharacterAbilities::new();
        
        if let Some(tree) = self.character_ability_trees.get(character_class) {
            character_abilities.add_ability_tree(tree.clone());
        }

        character_abilities
    }
}

/// Ability definition for creating abilities
#[derive(Clone)]
pub struct AbilityDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub max_level: u32,
    pub cost: u32,
    pub effects: Vec<AbilityEffect>,
    pub requirements: Vec<AbilityRequirement>,
}

/// Element types for abilities
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ElementType {
    Fire,
    Ice,
    Lightning,
    Earth,
    Water,
    Air,
    Dark,
    Light,
}
