//! Items and Equipment System
//! 
//! This module provides a comprehensive item and equipment system for the 2D Brawler Engine.
//! It includes item data structures, rarity systems, inventory management, equipment types,
//! consumable items, and integration with the character and progression systems.

use crate::engine::ecs::{Component, Entity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod equipment;
pub mod consumables;
pub mod inventory;

pub use equipment::*;
pub use consumables::*;
pub use inventory::*;

/// Core item data structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub rarity: ItemRarity,
    pub value: u32,
    pub stack_size: u32,
    pub max_stack: u32,
    pub level_requirement: u32,
    pub class_requirement: Option<String>,
    pub stats: ItemStats,
    pub effects: Vec<ItemEffect>,
    pub tags: Vec<String>,
}

/// Item types for categorization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    Weapon(WeaponType),
    Armor(ArmorType),
    Accessory(AccessoryType),
    Consumable(ConsumableType),
    Material(MaterialType),
    Quest(QuestType),
    Misc(MiscType),
}

/// Weapon types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Axe,
    Mace,
    Dagger,
    Bow,
    Crossbow,
    Staff,
    Wand,
    Spear,
    Hammer,
}

/// Armor types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorType {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Gloves,
    Shield,
    Cape,
    Belt,
}

/// Accessory types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccessoryType {
    Ring,
    Amulet,
    Bracelet,
    Earring,
    Pendant,
    Charm,
}

/// Consumable types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsumableType {
    Potion,
    Food,
    Scroll,
    Book,
    Key,
    Tool,
}

/// Material types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaterialType {
    Ore,
    Gem,
    Wood,
    Cloth,
    Leather,
    Metal,
    Crystal,
    Essence,
}

/// Quest item types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuestType {
    Artifact,
    Document,
    Token,
    Relic,
    Key,
    Evidence,
}

/// Miscellaneous item types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MiscType {
    Currency,
    Container,
    Decoration,
    Trophy,
    Collectible,
}

/// Item rarity levels
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

/// Item statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemStats {
    pub health_bonus: i32,
    pub mana_bonus: i32,
    pub stamina_bonus: i32,
    pub strength_bonus: i32,
    pub dexterity_bonus: i32,
    pub intelligence_bonus: i32,
    pub constitution_bonus: i32,
    pub wisdom_bonus: i32,
    pub charisma_bonus: i32,
    pub attack_damage_bonus: i32,
    pub magic_damage_bonus: i32,
    pub defense_bonus: i32,
    pub magic_resistance_bonus: i32,
    pub critical_chance_bonus: f32,
    pub critical_damage_bonus: f32,
    pub attack_speed_bonus: f32,
    pub movement_speed_bonus: f32,
    pub dodge_chance_bonus: f32,
    pub block_chance_bonus: f32,
    pub parry_chance_bonus: f32,
}

impl Default for ItemStats {
    fn default() -> Self {
        Self {
            health_bonus: 0,
            mana_bonus: 0,
            stamina_bonus: 0,
            strength_bonus: 0,
            dexterity_bonus: 0,
            intelligence_bonus: 0,
            constitution_bonus: 0,
            wisdom_bonus: 0,
            charisma_bonus: 0,
            attack_damage_bonus: 0,
            magic_damage_bonus: 0,
            defense_bonus: 0,
            magic_resistance_bonus: 0,
            critical_chance_bonus: 0.0,
            critical_damage_bonus: 0.0,
            attack_speed_bonus: 0.0,
            movement_speed_bonus: 0.0,
            dodge_chance_bonus: 0.0,
            block_chance_bonus: 0.0,
            parry_chance_bonus: 0.0,
        }
    }
}

/// Item effects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemEffect {
    pub effect_type: ItemEffectType,
    pub value: f32,
    pub duration: Option<f32>,
    pub condition: Option<String>,
    pub description: String,
}

/// Item effect types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemEffectType {
    // Stat bonuses
    HealthRegen,
    ManaRegen,
    StaminaRegen,
    ExperienceBonus,
    GoldBonus,
    
    // Combat effects
    DamageOverTime,
    HealOverTime,
    DamageReflection,
    LifeSteal,
    ManaSteal,
    
    // Status effects
    PoisonResistance,
    FireResistance,
    IceResistance,
    LightningResistance,
    StunResistance,
    SlowResistance,
    
    // Utility effects
    MovementSpeed,
    JumpHeight,
    FallDamageReduction,
    WaterBreathing,
    NightVision,
    
    // Special effects
    Teleportation,
    Invisibility,
    Flight,
    PhaseShift,
    TimeSlow,
}

/// Item manager for handling item creation and management
#[derive(Debug, Clone)]
pub struct ItemManager {
    pub items: HashMap<String, Item>,
    pub item_templates: HashMap<String, ItemTemplate>,
}

/// Item template for creating items
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub rarity: ItemRarity,
    pub base_value: u32,
    pub base_stats: ItemStats,
    pub base_effects: Vec<ItemEffect>,
    pub level_scaling: bool,
    pub stat_scaling: f32,
}

impl ItemManager {
    /// Create a new item manager
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            item_templates: HashMap::new(),
        }
    }
    
    /// Add an item template
    pub fn add_template(&mut self, template: ItemTemplate) {
        self.item_templates.insert(template.id.clone(), template);
    }
    
    /// Create an item from a template
    pub fn create_item(&mut self, template_id: &str, level: u32) -> Option<Item> {
        let template = self.item_templates.get(template_id)?;
        
        let mut item = Item {
            id: format!("{}_{}", template_id, level),
            name: template.name.clone(),
            description: template.description.clone(),
            item_type: template.item_type.clone(),
            rarity: template.rarity.clone(),
            value: if template.level_scaling {
                (template.base_value as f32 * (1.0 + level as f32 * 0.1)) as u32
            } else {
                template.base_value
            },
            stack_size: 1,
            max_stack: 1,
            level_requirement: level,
            class_requirement: None,
            stats: if template.level_scaling {
                self.scale_stats(&template.base_stats, level, template.stat_scaling)
            } else {
                template.base_stats.clone()
            },
            effects: template.base_effects.clone(),
            tags: vec![],
        };
        
        self.items.insert(item.id.clone(), item.clone());
        Some(item)
    }
    
    /// Scale item stats based on level
    fn scale_stats(&self, base_stats: &ItemStats, level: u32, scaling_factor: f32) -> ItemStats {
        let multiplier = 1.0 + (level as f32 - 1.0) * scaling_factor;
        
        ItemStats {
            health_bonus: (base_stats.health_bonus as f32 * multiplier) as i32,
            mana_bonus: (base_stats.mana_bonus as f32 * multiplier) as i32,
            stamina_bonus: (base_stats.stamina_bonus as f32 * multiplier) as i32,
            strength_bonus: (base_stats.strength_bonus as f32 * multiplier) as i32,
            dexterity_bonus: (base_stats.dexterity_bonus as f32 * multiplier) as i32,
            intelligence_bonus: (base_stats.intelligence_bonus as f32 * multiplier) as i32,
            constitution_bonus: (base_stats.constitution_bonus as f32 * multiplier) as i32,
            wisdom_bonus: (base_stats.wisdom_bonus as f32 * multiplier) as i32,
            charisma_bonus: (base_stats.charisma_bonus as f32 * multiplier) as i32,
            attack_damage_bonus: (base_stats.attack_damage_bonus as f32 * multiplier) as i32,
            magic_damage_bonus: (base_stats.magic_damage_bonus as f32 * multiplier) as i32,
            defense_bonus: (base_stats.defense_bonus as f32 * multiplier) as i32,
            magic_resistance_bonus: (base_stats.magic_resistance_bonus as f32 * multiplier) as i32,
            critical_chance_bonus: base_stats.critical_chance_bonus * multiplier,
            critical_damage_bonus: base_stats.critical_damage_bonus * multiplier,
            attack_speed_bonus: base_stats.attack_speed_bonus * multiplier,
            movement_speed_bonus: base_stats.movement_speed_bonus * multiplier,
            dodge_chance_bonus: base_stats.dodge_chance_bonus * multiplier,
            block_chance_bonus: base_stats.block_chance_bonus * multiplier,
            parry_chance_bonus: base_stats.parry_chance_bonus * multiplier,
        }
    }
    
    /// Get an item by ID
    pub fn get_item(&self, item_id: &str) -> Option<&Item> {
        self.items.get(item_id)
    }
    
    /// Get all items of a specific type
    pub fn get_items_by_type(&self, item_type: &ItemType) -> Vec<&Item> {
        self.items.values()
            .filter(|item| &item.item_type == item_type)
            .collect()
    }
    
    /// Get all items of a specific rarity
    pub fn get_items_by_rarity(&self, rarity: &ItemRarity) -> Vec<&Item> {
        self.items.values()
            .filter(|item| &item.rarity == rarity)
            .collect()
    }
    
    /// Initialize default item templates
    pub fn initialize_default_templates(&mut self) {
        // Initialize weapon templates
        self.initialize_weapon_templates();
        
        // Initialize armor templates
        self.initialize_armor_templates();
        
        // Initialize accessory templates
        self.initialize_accessory_templates();
        
        // Initialize consumable templates
        self.initialize_consumable_templates();
    }
    
    /// Initialize weapon templates
    fn initialize_weapon_templates(&mut self) {
        // Basic sword
        self.add_template(ItemTemplate {
            id: "basic_sword".to_string(),
            name: "Iron Sword".to_string(),
            description: "A sturdy iron sword with good balance.".to_string(),
            item_type: ItemType::Weapon(WeaponType::Sword),
            rarity: ItemRarity::Common,
            base_value: 50,
            base_stats: ItemStats {
                attack_damage_bonus: 10,
                strength_bonus: 2,
                ..Default::default()
            },
            base_effects: vec![],
            level_scaling: true,
            stat_scaling: 0.1,
        });
        
        // Magic staff
        self.add_template(ItemTemplate {
            id: "magic_staff".to_string(),
            name: "Apprentice Staff".to_string(),
            description: "A wooden staff imbued with magical energy.".to_string(),
            item_type: ItemType::Weapon(WeaponType::Staff),
            rarity: ItemRarity::Uncommon,
            base_value: 100,
            base_stats: ItemStats {
                magic_damage_bonus: 15,
                intelligence_bonus: 3,
                mana_bonus: 20,
                ..Default::default()
            },
            base_effects: vec![
                ItemEffect {
                    effect_type: ItemEffectType::ManaRegen,
                    value: 2.0,
                    duration: None,
                    condition: None,
                    description: "Regenerates mana over time".to_string(),
                }
            ],
            level_scaling: true,
            stat_scaling: 0.15,
        });
    }
    
    /// Initialize armor templates
    fn initialize_armor_templates(&mut self) {
        // Basic helmet
        self.add_template(ItemTemplate {
            id: "leather_helmet".to_string(),
            name: "Leather Helmet".to_string(),
            description: "A simple leather helmet for basic protection.".to_string(),
            item_type: ItemType::Armor(ArmorType::Helmet),
            rarity: ItemRarity::Common,
            base_value: 30,
            base_stats: ItemStats {
                defense_bonus: 5,
                constitution_bonus: 1,
                ..Default::default()
            },
            base_effects: vec![],
            level_scaling: true,
            stat_scaling: 0.08,
        });
        
        // Magic robe
        self.add_template(ItemTemplate {
            id: "magic_robe".to_string(),
            name: "Apprentice Robe".to_string(),
            description: "A flowing robe that enhances magical abilities.".to_string(),
            item_type: ItemType::Armor(ArmorType::Chestplate),
            rarity: ItemRarity::Uncommon,
            base_value: 80,
            base_stats: ItemStats {
                magic_resistance_bonus: 8,
                intelligence_bonus: 2,
                mana_bonus: 15,
                ..Default::default()
            },
            base_effects: vec![
                ItemEffect {
                    effect_type: ItemEffectType::ManaRegen,
                    value: 1.5,
                    duration: None,
                    condition: None,
                    description: "Enhanced mana regeneration".to_string(),
                }
            ],
            level_scaling: true,
            stat_scaling: 0.12,
        });
    }
    
    /// Initialize accessory templates
    fn initialize_accessory_templates(&mut self) {
        // Health ring
        self.add_template(ItemTemplate {
            id: "health_ring".to_string(),
            name: "Ring of Vitality".to_string(),
            description: "A ring that increases health and regeneration.".to_string(),
            item_type: ItemType::Accessory(AccessoryType::Ring),
            rarity: ItemRarity::Rare,
            base_value: 150,
            base_stats: ItemStats {
                health_bonus: 25,
                constitution_bonus: 3,
                ..Default::default()
            },
            base_effects: vec![
                ItemEffect {
                    effect_type: ItemEffectType::HealthRegen,
                    value: 3.0,
                    duration: None,
                    condition: None,
                    description: "Regenerates health over time".to_string(),
                }
            ],
            level_scaling: true,
            stat_scaling: 0.2,
        });
    }
    
    /// Initialize consumable templates
    fn initialize_consumable_templates(&mut self) {
        // Health potion
        self.add_template(ItemTemplate {
            id: "health_potion".to_string(),
            name: "Health Potion".to_string(),
            description: "Restores health when consumed.".to_string(),
            item_type: ItemType::Consumable(ConsumableType::Potion),
            rarity: ItemRarity::Common,
            base_value: 25,
            base_stats: ItemStats::default(),
            base_effects: vec![
                ItemEffect {
                    effect_type: ItemEffectType::HealOverTime,
                    value: 50.0,
                    duration: Some(5.0),
                    condition: None,
                    description: "Restores 50 health over 5 seconds".to_string(),
                }
            ],
            level_scaling: false,
            stat_scaling: 0.0,
        });
        
        // Mana potion
        self.add_template(ItemTemplate {
            id: "mana_potion".to_string(),
            name: "Mana Potion".to_string(),
            description: "Restores mana when consumed.".to_string(),
            item_type: ItemType::Consumable(ConsumableType::Potion),
            rarity: ItemRarity::Common,
            base_value: 25,
            base_stats: ItemStats::default(),
            base_effects: vec![
                ItemEffect {
                    effect_type: ItemEffectType::ManaRegen,
                    value: 30.0,
                    duration: Some(3.0),
                    condition: None,
                    description: "Restores 30 mana over 3 seconds".to_string(),
                }
            ],
            level_scaling: false,
            stat_scaling: 0.0,
        });
    }
}

impl Default for ItemManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Item {}
