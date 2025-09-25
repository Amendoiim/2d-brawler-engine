//! Equipment System
//! 
//! This module provides equipment management for weapons, armor, and accessories.
//! It includes equipment slots, stat bonuses, set bonuses, and equipment effects.

use crate::engine::ecs::{Component, Entity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Item, ItemStats, ItemEffect, ItemRarity};

/// Equipment component for entities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Equipment {
    pub slots: HashMap<EquipmentSlot, Option<String>>, // Item ID or None
    pub set_bonuses: Vec<SetBonus>,
    pub total_stats: ItemStats,
    pub total_effects: Vec<ItemEffect>,
}

/// Equipment slots
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    // Weapons
    MainHand,
    OffHand,
    
    // Armor
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Gloves,
    Shield,
    Cape,
    Belt,
    
    // Accessories
    Ring1,
    Ring2,
    Amulet,
    Bracelet1,
    Bracelet2,
    Earring1,
    Earring2,
    Pendant,
    Charm,
}

/// Equipment set bonus
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBonus {
    pub set_name: String,
    pub pieces_required: u32,
    pub pieces_equipped: u32,
    pub bonus_stats: ItemStats,
    pub bonus_effects: Vec<ItemEffect>,
    pub description: String,
}

/// Equipment manager for handling equipment operations
#[derive(Debug, Clone)]
pub struct EquipmentManager {
    pub equipment_sets: HashMap<String, EquipmentSet>,
    pub set_bonuses: HashMap<String, Vec<SetBonus>>,
}

/// Equipment set definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentSet {
    pub name: String,
    pub description: String,
    pub pieces: Vec<String>, // Item IDs
    pub bonuses: Vec<SetBonus>,
}

impl Equipment {
    /// Create new empty equipment
    pub fn new() -> Self {
        let mut slots = HashMap::new();
        
        // Initialize all equipment slots
        slots.insert(EquipmentSlot::MainHand, None);
        slots.insert(EquipmentSlot::OffHand, None);
        slots.insert(EquipmentSlot::Helmet, None);
        slots.insert(EquipmentSlot::Chestplate, None);
        slots.insert(EquipmentSlot::Leggings, None);
        slots.insert(EquipmentSlot::Boots, None);
        slots.insert(EquipmentSlot::Gloves, None);
        slots.insert(EquipmentSlot::Shield, None);
        slots.insert(EquipmentSlot::Cape, None);
        slots.insert(EquipmentSlot::Belt, None);
        slots.insert(EquipmentSlot::Ring1, None);
        slots.insert(EquipmentSlot::Ring2, None);
        slots.insert(EquipmentSlot::Amulet, None);
        slots.insert(EquipmentSlot::Bracelet1, None);
        slots.insert(EquipmentSlot::Bracelet2, None);
        slots.insert(EquipmentSlot::Earring1, None);
        slots.insert(EquipmentSlot::Earring2, None);
        slots.insert(EquipmentSlot::Pendant, None);
        slots.insert(EquipmentSlot::Charm, None);
        
        Self {
            slots,
            set_bonuses: vec![],
            total_stats: ItemStats::default(),
            total_effects: vec![],
        }
    }
    
    /// Equip an item to a slot
    pub fn equip_item(&mut self, slot: EquipmentSlot, item_id: String) -> Option<String> {
        // Unequip current item if any
        let old_item = self.slots.insert(slot, Some(item_id));
        old_item.unwrap_or_default()
    }
    
    /// Unequip an item from a slot
    pub fn unequip_item(&mut self, slot: EquipmentSlot) -> Option<String> {
        self.slots.insert(slot, None).unwrap_or_default()
    }
    
    /// Get equipped item in a slot
    pub fn get_equipped_item(&self, slot: &EquipmentSlot) -> Option<&String> {
        self.slots.get(slot)?.as_ref()
    }
    
    /// Check if a slot is empty
    pub fn is_slot_empty(&self, slot: &EquipmentSlot) -> bool {
        self.slots.get(slot).map_or(true, |item| item.is_none())
    }
    
    /// Get all equipped items
    pub fn get_all_equipped_items(&self) -> Vec<&String> {
        self.slots.values()
            .filter_map(|item| item.as_ref())
            .collect()
    }
    
    /// Calculate total stats from all equipped items
    pub fn calculate_total_stats(&self, items: &HashMap<String, Item>) -> ItemStats {
        let mut total_stats = ItemStats::default();
        
        for item_id in self.get_all_equipped_items() {
            if let Some(item) = items.get(item_id) {
                total_stats = self.add_stats(&total_stats, &item.stats);
            }
        }
        
        // Add set bonus stats
        for set_bonus in &self.set_bonuses {
            total_stats = self.add_stats(&total_stats, &set_bonus.bonus_stats);
        }
        
        total_stats
    }
    
    /// Calculate total effects from all equipped items
    pub fn calculate_total_effects(&self, items: &HashMap<String, Item>) -> Vec<ItemEffect> {
        let mut total_effects = vec![];
        
        for item_id in self.get_all_equipped_items() {
            if let Some(item) = items.get(item_id) {
                total_effects.extend(item.effects.clone());
            }
        }
        
        // Add set bonus effects
        for set_bonus in &self.set_bonuses {
            total_effects.extend(set_bonus.bonus_effects.clone());
        }
        
        total_effects
    }
    
    /// Add two stat structures together
    fn add_stats(&self, stats1: &ItemStats, stats2: &ItemStats) -> ItemStats {
        ItemStats {
            health_bonus: stats1.health_bonus + stats2.health_bonus,
            mana_bonus: stats1.mana_bonus + stats2.mana_bonus,
            stamina_bonus: stats1.stamina_bonus + stats2.stamina_bonus,
            strength_bonus: stats1.strength_bonus + stats2.strength_bonus,
            dexterity_bonus: stats1.dexterity_bonus + stats2.dexterity_bonus,
            intelligence_bonus: stats1.intelligence_bonus + stats2.intelligence_bonus,
            constitution_bonus: stats1.constitution_bonus + stats2.constitution_bonus,
            wisdom_bonus: stats1.wisdom_bonus + stats2.wisdom_bonus,
            charisma_bonus: stats1.charisma_bonus + stats2.charisma_bonus,
            attack_damage_bonus: stats1.attack_damage_bonus + stats2.attack_damage_bonus,
            magic_damage_bonus: stats1.magic_damage_bonus + stats2.magic_damage_bonus,
            defense_bonus: stats1.defense_bonus + stats2.defense_bonus,
            magic_resistance_bonus: stats1.magic_resistance_bonus + stats2.magic_resistance_bonus,
            critical_chance_bonus: stats1.critical_chance_bonus + stats2.critical_chance_bonus,
            critical_damage_bonus: stats1.critical_damage_bonus + stats2.critical_damage_bonus,
            attack_speed_bonus: stats1.attack_speed_bonus + stats2.attack_speed_bonus,
            movement_speed_bonus: stats1.movement_speed_bonus + stats2.movement_speed_bonus,
            dodge_chance_bonus: stats1.dodge_chance_bonus + stats2.dodge_chance_bonus,
            block_chance_bonus: stats1.block_chance_bonus + stats2.block_chance_bonus,
            parry_chance_bonus: stats1.parry_chance_bonus + stats2.parry_chance_bonus,
        }
    }
    
    /// Update equipment stats and effects
    pub fn update_equipment(&mut self, items: &HashMap<String, Item>) {
        self.total_stats = self.calculate_total_stats(items);
        self.total_effects = self.calculate_total_effects(items);
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Self::new()
    }
}

impl EquipmentManager {
    /// Create new equipment manager
    pub fn new() -> Self {
        Self {
            equipment_sets: HashMap::new(),
            set_bonuses: HashMap::new(),
        }
    }
    
    /// Add an equipment set
    pub fn add_equipment_set(&mut self, set: EquipmentSet) {
        self.equipment_sets.insert(set.name.clone(), set);
    }
    
    /// Calculate set bonuses for equipped items
    pub fn calculate_set_bonuses(&self, equipped_items: &[&String], items: &HashMap<String, Item>) -> Vec<SetBonus> {
        let mut set_bonuses = vec![];
        let mut set_counts: HashMap<String, u32> = HashMap::new();
        
        // Count pieces of each set
        for item_id in equipped_items {
            if let Some(item) = items.get(*item_id) {
                // Check if item belongs to any set
                for (set_name, set) in &self.equipment_sets {
                    if set.pieces.contains(&item.id) {
                        *set_counts.entry(set_name.clone()).or_insert(0) += 1;
                    }
                }
            }
        }
        
        // Apply set bonuses
        for (set_name, count) in set_counts {
            if let Some(set) = self.equipment_sets.get(&set_name) {
                for bonus in &set.bonuses {
                    if count >= bonus.pieces_required {
                        let mut active_bonus = bonus.clone();
                        active_bonus.pieces_equipped = count;
                        set_bonuses.push(active_bonus);
                    }
                }
            }
        }
        
        set_bonuses
    }
    
    /// Check if an item can be equipped in a slot
    pub fn can_equip_item(&self, item: &Item, slot: &EquipmentSlot) -> bool {
        match (&item.item_type, slot) {
            // Weapons
            (super::ItemType::Weapon(_), EquipmentSlot::MainHand) => true,
            (super::ItemType::Weapon(_), EquipmentSlot::OffHand) => true,
            
            // Armor
            (super::ItemType::Armor(super::ArmorType::Helmet), EquipmentSlot::Helmet) => true,
            (super::ItemType::Armor(super::ArmorType::Chestplate), EquipmentSlot::Chestplate) => true,
            (super::ItemType::Armor(super::ArmorType::Leggings), EquipmentSlot::Leggings) => true,
            (super::ItemType::Armor(super::ArmorType::Boots), EquipmentSlot::Boots) => true,
            (super::ItemType::Armor(super::ArmorType::Gloves), EquipmentSlot::Gloves) => true,
            (super::ItemType::Armor(super::ArmorType::Shield), EquipmentSlot::Shield) => true,
            (super::ItemType::Armor(super::ArmorType::Cape), EquipmentSlot::Cape) => true,
            (super::ItemType::Armor(super::ArmorType::Belt), EquipmentSlot::Belt) => true,
            
            // Accessories
            (super::ItemType::Accessory(super::AccessoryType::Ring), EquipmentSlot::Ring1) => true,
            (super::ItemType::Accessory(super::AccessoryType::Ring), EquipmentSlot::Ring2) => true,
            (super::ItemType::Accessory(super::AccessoryType::Amulet), EquipmentSlot::Amulet) => true,
            (super::ItemType::Accessory(super::AccessoryType::Bracelet), EquipmentSlot::Bracelet1) => true,
            (super::ItemType::Accessory(super::AccessoryType::Bracelet), EquipmentSlot::Bracelet2) => true,
            (super::ItemType::Accessory(super::AccessoryType::Earring), EquipmentSlot::Earring1) => true,
            (super::ItemType::Accessory(super::AccessoryType::Earring), EquipmentSlot::Earring2) => true,
            (super::ItemType::Accessory(super::AccessoryType::Pendant), EquipmentSlot::Pendant) => true,
            (super::ItemType::Accessory(super::AccessoryType::Charm), EquipmentSlot::Charm) => true,
            
            _ => false,
        }
    }
    
    /// Get the appropriate slot for an item type
    pub fn get_slot_for_item(&self, item: &Item) -> Option<EquipmentSlot> {
        match &item.item_type {
            super::ItemType::Weapon(_) => Some(EquipmentSlot::MainHand),
            super::ItemType::Armor(armor_type) => match armor_type {
                super::ArmorType::Helmet => Some(EquipmentSlot::Helmet),
                super::ArmorType::Chestplate => Some(EquipmentSlot::Chestplate),
                super::ArmorType::Leggings => Some(EquipmentSlot::Leggings),
                super::ArmorType::Boots => Some(EquipmentSlot::Boots),
                super::ArmorType::Gloves => Some(EquipmentSlot::Gloves),
                super::ArmorType::Shield => Some(EquipmentSlot::Shield),
                super::ArmorType::Cape => Some(EquipmentSlot::Cape),
                super::ArmorType::Belt => Some(EquipmentSlot::Belt),
            },
            super::ItemType::Accessory(accessory_type) => match accessory_type {
                super::AccessoryType::Ring => Some(EquipmentSlot::Ring1), // Default to first ring slot
                super::AccessoryType::Amulet => Some(EquipmentSlot::Amulet),
                super::AccessoryType::Bracelet => Some(EquipmentSlot::Bracelet1), // Default to first bracelet slot
                super::AccessoryType::Earring => Some(EquipmentSlot::Earring1), // Default to first earring slot
                super::AccessoryType::Pendant => Some(EquipmentSlot::Pendant),
                super::AccessoryType::Charm => Some(EquipmentSlot::Charm),
            },
            _ => None,
        }
    }
    
    /// Initialize default equipment sets
    pub fn initialize_default_sets(&mut self) {
        // Warrior set
        self.add_equipment_set(EquipmentSet {
            name: "Warrior's Set".to_string(),
            description: "A set of armor designed for warriors.".to_string(),
            pieces: vec![
                "warrior_helmet".to_string(),
                "warrior_chestplate".to_string(),
                "warrior_leggings".to_string(),
                "warrior_boots".to_string(),
            ],
            bonuses: vec![
                SetBonus {
                    set_name: "Warrior's Set".to_string(),
                    pieces_required: 2,
                    pieces_equipped: 0,
                    bonus_stats: ItemStats {
                        strength_bonus: 5,
                        defense_bonus: 10,
                        ..Default::default()
                    },
                    bonus_effects: vec![],
                    description: "2 pieces: +5 Strength, +10 Defense".to_string(),
                },
                SetBonus {
                    set_name: "Warrior's Set".to_string(),
                    pieces_required: 4,
                    pieces_equipped: 0,
                    bonus_stats: ItemStats {
                        health_bonus: 50,
                        attack_damage_bonus: 15,
                        ..Default::default()
                    },
                    bonus_effects: vec![
                        ItemEffect {
                            effect_type: super::ItemEffectType::DamageReflection,
                            value: 10.0,
                            duration: None,
                            condition: None,
                            description: "Reflects 10% of damage taken".to_string(),
                        }
                    ],
                    description: "4 pieces: +50 Health, +15 Attack Damage, 10% Damage Reflection".to_string(),
                },
            ],
        });
        
        // Mage set
        self.add_equipment_set(EquipmentSet {
            name: "Mage's Set".to_string(),
            description: "A set of robes designed for mages.".to_string(),
            pieces: vec![
                "mage_hat".to_string(),
                "mage_robe".to_string(),
                "mage_pants".to_string(),
                "mage_boots".to_string(),
            ],
            bonuses: vec![
                SetBonus {
                    set_name: "Mage's Set".to_string(),
                    pieces_required: 2,
                    pieces_equipped: 0,
                    bonus_stats: ItemStats {
                        intelligence_bonus: 5,
                        magic_resistance_bonus: 10,
                        ..Default::default()
                    },
                    bonus_effects: vec![],
                    description: "2 pieces: +5 Intelligence, +10 Magic Resistance".to_string(),
                },
                SetBonus {
                    set_name: "Mage's Set".to_string(),
                    pieces_required: 4,
                    pieces_equipped: 0,
                    bonus_stats: ItemStats {
                        mana_bonus: 100,
                        magic_damage_bonus: 20,
                        ..Default::default()
                    },
                    bonus_effects: vec![
                        ItemEffect {
                            effect_type: super::ItemEffectType::ManaRegen,
                            value: 5.0,
                            duration: None,
                            condition: None,
                            description: "Enhanced mana regeneration".to_string(),
                        }
                    ],
                    description: "4 pieces: +100 Mana, +20 Magic Damage, Enhanced Mana Regen".to_string(),
                },
            ],
        });
    }
}

impl Default for EquipmentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Equipment {}
