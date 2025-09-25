//! Character-specific abilities implementation

use crate::engine::ecs::{Component, System, World};
use std::collections::HashMap;
use super::*;
use crate::game::combat::Combat;

/// Character class definitions
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CharacterClass {
    Fighter,
    Ranger,
    Mage,
    Tank,
    Assassin,
}

impl CharacterClass {
    pub fn name(&self) -> &'static str {
        match self {
            CharacterClass::Fighter => "Fighter",
            CharacterClass::Ranger => "Ranger",
            CharacterClass::Mage => "Mage",
            CharacterClass::Tank => "Tank",
            CharacterClass::Assassin => "Assassin",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            CharacterClass::Fighter => "Balanced melee combatant with strong attacks and defense",
            CharacterClass::Ranger => "Ranged specialist with high mobility and precision",
            CharacterClass::Mage => "Magic user with powerful spells and area effects",
            CharacterClass::Tank => "High defense specialist with crowd control abilities",
            CharacterClass::Assassin => "High damage, low health stealth specialist",
        }
    }

    pub fn primary_stats(&self) -> (f32, f32, f32, f32) {
        match self {
            CharacterClass::Fighter => (100.0, 80.0, 60.0, 90.0), // Strength, Agility, Intelligence, Vitality
            CharacterClass::Ranger => (70.0, 120.0, 80.0, 70.0),
            CharacterClass::Mage => (50.0, 70.0, 130.0, 60.0),
            CharacterClass::Tank => (90.0, 60.0, 70.0, 130.0),
            CharacterClass::Assassin => (80.0, 130.0, 70.0, 50.0),
        }
    }

    pub fn resource_type(&self) -> ResourceType {
        match self {
            CharacterClass::Fighter => ResourceType::Stamina,
            CharacterClass::Ranger => ResourceType::Stamina,
            CharacterClass::Mage => ResourceType::Mana,
            CharacterClass::Tank => ResourceType::Stamina,
            CharacterClass::Assassin => ResourceType::Energy,
        }
    }
}

/// Character class manager
pub struct CharacterClassManager {
    pub class_definitions: HashMap<CharacterClass, ClassDefinition>,
    pub ability_trees: HashMap<CharacterClass, AbilityTree>,
}

impl CharacterClassManager {
    pub fn new() -> Self {
        let mut manager = Self {
            class_definitions: HashMap::new(),
            ability_trees: HashMap::new(),
        };
        manager.initialize_classes();
        manager
    }

    fn initialize_classes(&mut self) {
        // Initialize Fighter class
        let fighter_definition = ClassDefinition {
            class: CharacterClass::Fighter,
            name: "Fighter".to_string(),
            description: "Balanced melee combatant with strong attacks and defense".to_string(),
            primary_stats: (100.0, 80.0, 60.0, 90.0),
            resource_type: ResourceType::Stamina,
            starting_abilities: vec![
                "power_strike".to_string(),
                "heavy_armor".to_string(),
            ],
            special_abilities: vec![
                "whirlwind_attack".to_string(),
                "berserker_rage".to_string(),
                "weapon_mastery".to_string(),
            ],
        };
        self.class_definitions.insert(CharacterClass::Fighter, fighter_definition);

        // Initialize Ranger class
        let ranger_definition = ClassDefinition {
            class: CharacterClass::Ranger,
            name: "Ranger".to_string(),
            description: "Ranged specialist with high mobility and precision".to_string(),
            primary_stats: (70.0, 120.0, 80.0, 70.0),
            resource_type: ResourceType::Stamina,
            starting_abilities: vec![
                "quick_shot".to_string(),
                "eagle_eye".to_string(),
            ],
            special_abilities: vec![
                "explosive_arrow".to_string(),
                "rapid_fire".to_string(),
                "trap_mastery".to_string(),
            ],
        };
        self.class_definitions.insert(CharacterClass::Ranger, ranger_definition);

        // Initialize Mage class
        let mage_definition = ClassDefinition {
            class: CharacterClass::Mage,
            name: "Mage".to_string(),
            description: "Magic user with powerful spells and area effects".to_string(),
            primary_stats: (50.0, 70.0, 130.0, 60.0),
            resource_type: ResourceType::Mana,
            starting_abilities: vec![
                "mana_mastery".to_string(),
                "fire_magic".to_string(),
            ],
            special_abilities: vec![
                "meteor".to_string(),
                "teleport".to_string(),
                "elemental_mastery".to_string(),
            ],
        };
        self.class_definitions.insert(CharacterClass::Mage, mage_definition);

        // Initialize Tank class
        let tank_definition = ClassDefinition {
            class: CharacterClass::Tank,
            name: "Tank".to_string(),
            description: "High defense specialist with crowd control abilities".to_string(),
            primary_stats: (90.0, 60.0, 70.0, 130.0),
            resource_type: ResourceType::Stamina,
            starting_abilities: vec![
                "shield_mastery".to_string(),
                "taunt".to_string(),
            ],
            special_abilities: vec![
                "shield_wall".to_string(),
                "intimidating_presence".to_string(),
                "fortress".to_string(),
            ],
        };
        self.class_definitions.insert(CharacterClass::Tank, tank_definition);

        // Initialize Assassin class
        let assassin_definition = ClassDefinition {
            class: CharacterClass::Assassin,
            name: "Assassin".to_string(),
            description: "High damage, low health stealth specialist".to_string(),
            primary_stats: (80.0, 130.0, 70.0, 50.0),
            resource_type: ResourceType::Energy,
            starting_abilities: vec![
                "stealth".to_string(),
                "critical_strike".to_string(),
            ],
            special_abilities: vec![
                "shadow_strike".to_string(),
                "poison_mastery".to_string(),
                "shadow_clone".to_string(),
            ],
        };
        self.class_definitions.insert(CharacterClass::Assassin, assassin_definition);

        // Initialize ability trees for each class
        self.initialize_ability_trees();
    }

    fn initialize_ability_trees(&mut self) {
        // Fighter ability tree
        let mut fighter_tree = AbilityTree::new("Fighter".to_string());
        fighter_tree.max_points = 50;

        // Basic abilities
        let mut power_strike = Ability::new(
            "power_strike".to_string(),
            "Power Strike".to_string(),
            "Increases melee damage by 20% per level".to_string(),
            5,
            1,
        );
        power_strike.effects.push(AbilityEffect::DamageBonus { multiplier: 0.2 });
        power_strike.is_unlocked = true; // Starting ability
        fighter_tree.add_ability(power_strike);

        let mut heavy_armor = Ability::new(
            "heavy_armor".to_string(),
            "Heavy Armor".to_string(),
            "Increases defense by 30% per level, reduces speed by 10%".to_string(),
            3,
            2,
        );
        heavy_armor.effects.push(AbilityEffect::DefenseBonus { multiplier: 0.3 });
        heavy_armor.effects.push(AbilityEffect::SpeedPenalty { multiplier: -0.1 });
        heavy_armor.is_unlocked = true; // Starting ability
        fighter_tree.add_ability(heavy_armor);

        // Advanced abilities
        let mut whirlwind = Ability::new(
            "whirlwind_attack".to_string(),
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
        fighter_tree.add_ability(whirlwind);

        let mut berserker_rage = Ability::new(
            "berserker_rage".to_string(),
            "Berserker Rage".to_string(),
            "Increases damage and attack speed, reduces defense".to_string(),
            3,
            4,
        );
        berserker_rage.requirements.push(AbilityRequirement::Ability {
            ability_id: "power_strike".to_string(),
            min_level: 5,
        });
        berserker_rage.effects.push(AbilityEffect::DamageBonus { multiplier: 0.5 });
        berserker_rage.effects.push(AbilityEffect::AttackSpeedBonus { multiplier: 0.3 });
        berserker_rage.effects.push(AbilityEffect::DefenseBonus { multiplier: -0.2 });
        fighter_tree.add_ability(berserker_rage);

        self.ability_trees.insert(CharacterClass::Fighter, fighter_tree);

        // Ranger ability tree
        let mut ranger_tree = AbilityTree::new("Ranger".to_string());
        ranger_tree.max_points = 50;

        // Basic abilities
        let mut quick_shot = Ability::new(
            "quick_shot".to_string(),
            "Quick Shot".to_string(),
            "Increases ranged attack speed by 25% per level".to_string(),
            5,
            1,
        );
        quick_shot.effects.push(AbilityEffect::AttackSpeedBonus { multiplier: 0.25 });
        quick_shot.is_unlocked = true; // Starting ability
        ranger_tree.add_ability(quick_shot);

        let mut eagle_eye = Ability::new(
            "eagle_eye".to_string(),
            "Eagle Eye".to_string(),
            "Increases ranged attack range by 50% and accuracy by 30%".to_string(),
            3,
            2,
        );
        eagle_eye.effects.push(AbilityEffect::RangeBonus { multiplier: 0.5 });
        eagle_eye.effects.push(AbilityEffect::AccuracyBonus { multiplier: 0.3 });
        eagle_eye.is_unlocked = true; // Starting ability
        ranger_tree.add_ability(eagle_eye);

        // Advanced abilities
        let mut explosive_arrow = Ability::new(
            "explosive_arrow".to_string(),
            "Explosive Arrow".to_string(),
            "Arrow that explodes on impact, dealing area damage".to_string(),
            3,
            3,
        );
        explosive_arrow.requirements.push(AbilityRequirement::Ability {
            ability_id: "eagle_eye".to_string(),
            min_level: 2,
        });
        explosive_arrow.effects.push(AbilityEffect::ExplosiveDamage { radius: 150.0 });
        ranger_tree.add_ability(explosive_arrow);

        let mut rapid_fire = Ability::new(
            "rapid_fire".to_string(),
            "Rapid Fire".to_string(),
            "Fire multiple arrows in quick succession".to_string(),
            3,
            4,
        );
        rapid_fire.requirements.push(AbilityRequirement::Ability {
            ability_id: "quick_shot".to_string(),
            min_level: 5,
        });
        rapid_fire.effects.push(AbilityEffect::AttackSpeedBonus { multiplier: 0.8 });
        ranger_tree.add_ability(rapid_fire);

        self.ability_trees.insert(CharacterClass::Ranger, ranger_tree);

        // Mage ability tree
        let mut mage_tree = AbilityTree::new("Mage".to_string());
        mage_tree.max_points = 50;

        // Basic abilities
        let mut mana_mastery = Ability::new(
            "mana_mastery".to_string(),
            "Mana Mastery".to_string(),
            "Increases mana pool by 30% and regeneration by 50%".to_string(),
            5,
            1,
        );
        mana_mastery.effects.push(AbilityEffect::ManaBonus { multiplier: 0.3 });
        mana_mastery.effects.push(AbilityEffect::ManaRegenBonus { multiplier: 0.5 });
        mana_mastery.is_unlocked = true; // Starting ability
        mage_tree.add_ability(mana_mastery);

        let mut fire_magic = Ability::new(
            "fire_magic".to_string(),
            "Fire Magic".to_string(),
            "Increases fire spell damage by 40% per level".to_string(),
            5,
            1,
        );
        fire_magic.effects.push(AbilityEffect::ElementalDamageBonus {
            element: effects::ElementType::Fire,
            multiplier: 0.4,
        });
        fire_magic.is_unlocked = true; // Starting ability
        mage_tree.add_ability(fire_magic);

        // Advanced abilities
        let mut meteor = Ability::new(
            "meteor".to_string(),
            "Meteor".to_string(),
            "Summon a meteor from the sky, dealing massive damage".to_string(),
            3,
            5,
        );
        meteor.requirements.push(AbilityRequirement::Ability {
            ability_id: "fire_magic".to_string(),
            min_level: 3,
        });
        meteor.effects.push(AbilityEffect::MeteorStrike { damage: 300.0, radius: 200.0 });
        mage_tree.add_ability(meteor);

        let mut teleport = Ability::new(
            "teleport".to_string(),
            "Teleport".to_string(),
            "Instantly move to a nearby location".to_string(),
            3,
            3,
        );
        teleport.requirements.push(AbilityRequirement::Ability {
            ability_id: "mana_mastery".to_string(),
            min_level: 2,
        });
        teleport.effects.push(AbilityEffect::Teleport { range: 300.0 });
        mage_tree.add_ability(teleport);

        self.ability_trees.insert(CharacterClass::Mage, mage_tree);

        // Tank ability tree
        let mut tank_tree = AbilityTree::new("Tank".to_string());
        tank_tree.max_points = 50;

        // Basic abilities
        let mut shield_mastery = Ability::new(
            "shield_mastery".to_string(),
            "Shield Mastery".to_string(),
            "Increases block effectiveness by 30% and reduces stamina cost by 20%".to_string(),
            5,
            1,
        );
        shield_mastery.effects.push(AbilityEffect::BlockBonus { multiplier: 0.3 });
        shield_mastery.effects.push(AbilityEffect::StaminaCostReduction { multiplier: 0.2 });
        shield_mastery.is_unlocked = true; // Starting ability
        tank_tree.add_ability(shield_mastery);

        let mut taunt = Ability::new(
            "taunt".to_string(),
            "Taunt".to_string(),
            "Force enemies to attack you for 5 seconds".to_string(),
            3,
            2,
        );
        taunt.effects.push(AbilityEffect::Taunt { range: 300.0, duration: 5.0 });
        taunt.is_unlocked = true; // Starting ability
        tank_tree.add_ability(taunt);

        // Advanced abilities
        let mut shield_wall = Ability::new(
            "shield_wall".to_string(),
            "Shield Wall".to_string(),
            "Create a protective barrier that reduces damage by 80%".to_string(),
            3,
            4,
        );
        shield_wall.requirements.push(AbilityRequirement::Ability {
            ability_id: "shield_mastery".to_string(),
            min_level: 3,
        });
        shield_wall.effects.push(AbilityEffect::ShieldWall { duration: 10.0, damage_reduction: 0.8 });
        tank_tree.add_ability(shield_wall);

        let mut intimidating_presence = Ability::new(
            "intimidating_presence".to_string(),
            "Intimidating Presence".to_string(),
            "Reduces enemy damage and movement speed in area".to_string(),
            3,
            3,
        );
        intimidating_presence.requirements.push(AbilityRequirement::Ability {
            ability_id: "taunt".to_string(),
            min_level: 2,
        });
        intimidating_presence.effects.push(AbilityEffect::Taunt { range: 400.0, duration: 8.0 });
        tank_tree.add_ability(intimidating_presence);

        self.ability_trees.insert(CharacterClass::Tank, tank_tree);

        // Assassin ability tree
        let mut assassin_tree = AbilityTree::new("Assassin".to_string());
        assassin_tree.max_points = 50;

        // Basic abilities
        let mut stealth = Ability::new(
            "stealth".to_string(),
            "Stealth".to_string(),
            "Become invisible for 3 seconds per level".to_string(),
            5,
            1,
        );
        stealth.effects.push(AbilityEffect::Invisibility { duration: 3.0 });
        stealth.is_unlocked = true; // Starting ability
        assassin_tree.add_ability(stealth);

        let mut critical_strike = Ability::new(
            "critical_strike".to_string(),
            "Critical Strike".to_string(),
            "Increases critical hit chance by 20% and damage by 50%".to_string(),
            5,
            1,
        );
        critical_strike.effects.push(AbilityEffect::CriticalChanceBonus { multiplier: 0.2 });
        critical_strike.effects.push(AbilityEffect::CriticalDamageBonus { multiplier: 0.5 });
        critical_strike.is_unlocked = true; // Starting ability
        assassin_tree.add_ability(critical_strike);

        // Advanced abilities
        let mut shadow_strike = Ability::new(
            "shadow_strike".to_string(),
            "Shadow Strike".to_string(),
            "Teleport behind enemy and attack for massive damage".to_string(),
            3,
            4,
        );
        shadow_strike.requirements.push(AbilityRequirement::Ability {
            ability_id: "stealth".to_string(),
            min_level: 3,
        });
        shadow_strike.effects.push(AbilityEffect::TeleportAttack { damage: 250.0 });
        assassin_tree.add_ability(shadow_strike);

        let mut poison_mastery = Ability::new(
            "poison_mastery".to_string(),
            "Poison Mastery".to_string(),
            "Attacks have a chance to poison enemies".to_string(),
            3,
            3,
        );
        poison_mastery.requirements.push(AbilityRequirement::Ability {
            ability_id: "critical_strike".to_string(),
            min_level: 2,
        });
        poison_mastery.effects.push(AbilityEffect::StatusImmunity { status_type: StatusType::Poison });
        assassin_tree.add_ability(poison_mastery);

        self.ability_trees.insert(CharacterClass::Assassin, assassin_tree);
    }

    pub fn get_class_definition(&self, class: CharacterClass) -> Option<&ClassDefinition> {
        self.class_definitions.get(&class)
    }

    pub fn get_ability_tree(&self, class: CharacterClass) -> Option<&AbilityTree> {
        self.ability_trees.get(&class)
    }

    pub fn create_character_abilities(&self, class: CharacterClass) -> CharacterAbilities {
        let mut character_abilities = CharacterAbilities::new();
        
        if let Some(tree) = self.ability_trees.get(&class) {
            character_abilities.add_ability_tree(tree.clone());
        }

        character_abilities
    }
}

/// Class definition
#[derive(Clone)]
pub struct ClassDefinition {
    pub class: CharacterClass,
    pub name: String,
    pub description: String,
    pub primary_stats: (f32, f32, f32, f32), // Strength, Agility, Intelligence, Vitality
    pub resource_type: ResourceType,
    pub starting_abilities: Vec<String>,
    pub special_abilities: Vec<String>,
}

/// Character creation system
pub struct CharacterCreationSystem {
    pub class_manager: CharacterClassManager,
    pub ability_manager: AbilityManager,
}

impl CharacterCreationSystem {
    pub fn new() -> Self {
        Self {
            class_manager: CharacterClassManager::new(),
            ability_manager: AbilityManager::new(),
        }
    }

    pub fn create_character(&self, class: CharacterClass, name: String) -> Character {
        let class_definition = self.class_manager.get_class_definition(class).unwrap();
        let abilities = self.class_manager.create_character_abilities(class);
        
        Character {
            name,
            class,
            level: 1,
            experience: 0,
            stats: CharacterStats {
                strength: class_definition.primary_stats.0,
                agility: class_definition.primary_stats.1,
                intelligence: class_definition.primary_stats.2,
                vitality: class_definition.primary_stats.3,
            },
            abilities,
            combat: Combat::default(),
        }
    }

    pub fn get_available_classes(&self) -> Vec<CharacterClass> {
        vec![
            CharacterClass::Fighter,
            CharacterClass::Ranger,
            CharacterClass::Mage,
            CharacterClass::Tank,
            CharacterClass::Assassin,
        ]
    }
}

/// Character definition
#[derive(Clone)]
pub struct Character {
    pub name: String,
    pub class: CharacterClass,
    pub level: u32,
    pub experience: u32,
    pub stats: CharacterStats,
    pub abilities: CharacterAbilities,
    pub combat: Combat,
}

/// Character stats
#[derive(Clone)]
pub struct CharacterStats {
    pub strength: f32,
    pub agility: f32,
    pub intelligence: f32,
    pub vitality: f32,
}

impl CharacterStats {
    pub fn calculate_health(&self) -> f32 {
        self.vitality * 10.0
    }

    pub fn calculate_mana(&self) -> f32 {
        self.intelligence * 8.0
    }

    pub fn calculate_stamina(&self) -> f32 {
        self.agility * 6.0
    }

    pub fn calculate_damage(&self) -> f32 {
        self.strength * 2.0
    }

    pub fn calculate_defense(&self) -> f32 {
        self.vitality * 1.5
    }

    pub fn calculate_speed(&self) -> f32 {
        self.agility * 0.5
    }
}
