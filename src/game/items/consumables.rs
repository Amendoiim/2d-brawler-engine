//! Consumable Items System
//! 
//! This module provides consumable item management including potions, food, scrolls,
//! and other temporary items with various effects and durations.

use crate::engine::ecs::{Component, Entity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Item, ItemEffect, ItemEffectType, ItemRarity};

/// Consumable item component
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consumable {
    pub item_id: String,
    pub consumable_type: ConsumableType,
    pub effects: Vec<ConsumableEffect>,
    pub duration: Option<f32>,
    pub cooldown: f32,
    pub stackable: bool,
    pub max_stack: u32,
    pub usage_count: u32,
    pub max_usage: u32,
}

/// Consumable types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsumableType {
    Potion,
    Food,
    Scroll,
    Book,
    Key,
    Tool,
    Bomb,
    Trap,
    Medicine,
    Elixir,
}

/// Consumable effect
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumableEffect {
    pub effect_type: ConsumableEffectType,
    pub value: f32,
    pub duration: Option<f32>,
    pub target: ConsumableTarget,
    pub condition: Option<String>,
    pub description: String,
}

/// Consumable effect types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsumableEffectType {
    // Healing effects
    InstantHeal,
    HealOverTime,
    MaxHealthIncrease,
    HealthRegen,
    
    // Mana effects
    InstantMana,
    ManaOverTime,
    MaxManaIncrease,
    ManaRegen,
    
    // Stamina effects
    InstantStamina,
    StaminaOverTime,
    MaxStaminaIncrease,
    StaminaRegen,
    
    // Stat bonuses
    StrengthBonus,
    DexterityBonus,
    IntelligenceBonus,
    ConstitutionBonus,
    WisdomBonus,
    CharismaBonus,
    
    // Combat bonuses
    AttackDamageBonus,
    MagicDamageBonus,
    DefenseBonus,
    MagicResistanceBonus,
    CriticalChanceBonus,
    CriticalDamageBonus,
    AttackSpeedBonus,
    MovementSpeedBonus,
    
    // Status effects
    Poison,
    Burn,
    Freeze,
    Stun,
    Slow,
    Haste,
    Invisibility,
    Invulnerability,
    Shield,
    
    // Utility effects
    ExperienceBonus,
    GoldBonus,
    LootBonus,
    Teleportation,
    Resurrection,
    CureAll,
    RemoveDebuffs,
    
    // Special effects
    Transform,
    Summon,
    Explosion,
    AreaHeal,
    AreaDamage,
}

/// Consumable target
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsumableTarget {
    SelfTarget,
    Ally,
    Enemy,
    AllAllies,
    AllEnemies,
    Area,
    Random,
}

/// Active consumable effect on a character
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveConsumableEffect {
    pub consumable_id: String,
    pub effect: ConsumableEffect,
    pub remaining_duration: f32,
    pub stacks: u32,
    pub max_stacks: u32,
}

/// Consumable manager for handling consumable operations
#[derive(Debug, Clone)]
pub struct ConsumableManager {
    pub active_effects: HashMap<Entity, Vec<ActiveConsumableEffect>>,
    pub cooldowns: HashMap<Entity, HashMap<String, f32>>,
    pub consumable_templates: HashMap<String, ConsumableTemplate>,
}

/// Consumable template for creating consumables
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumableTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub consumable_type: ConsumableType,
    pub rarity: ItemRarity,
    pub effects: Vec<ConsumableEffect>,
    pub duration: Option<f32>,
    pub cooldown: f32,
    pub stackable: bool,
    pub max_stack: u32,
    pub max_usage: u32,
    pub value: u32,
}

impl Consumable {
    /// Create a new consumable from a template
    pub fn from_template(template: &ConsumableTemplate) -> Self {
        Self {
            item_id: template.id.clone(),
            consumable_type: template.consumable_type.clone(),
            effects: template.effects.clone(),
            duration: template.duration,
            cooldown: template.cooldown,
            stackable: template.stackable,
            max_stack: template.max_stack,
            usage_count: 0,
            max_usage: template.max_usage,
        }
    }
    
    /// Check if consumable can be used
    pub fn can_use(&self, current_usage: u32) -> bool {
        current_usage < self.max_usage
    }
    
    /// Check if consumable is on cooldown
    pub fn is_on_cooldown(&self, remaining_cooldown: f32) -> bool {
        remaining_cooldown > 0.0
    }
    
    /// Get the total effect value (for stacking effects)
    pub fn get_total_effect_value(&self, effect_type: &ConsumableEffectType, stacks: u32) -> f32 {
        if let Some(effect) = self.effects.iter().find(|e| &e.effect_type == effect_type) {
            effect.value * stacks as f32
        } else {
            0.0
        }
    }
}

impl ConsumableManager {
    /// Create new consumable manager
    pub fn new() -> Self {
        Self {
            active_effects: HashMap::new(),
            cooldowns: HashMap::new(),
            consumable_templates: HashMap::new(),
        }
    }
    
    /// Add a consumable template
    pub fn add_template(&mut self, template: ConsumableTemplate) {
        self.consumable_templates.insert(template.id.clone(), template);
    }
    
    /// Use a consumable on an entity
    pub fn use_consumable(&mut self, entity: Entity, consumable: &Consumable) -> Result<(), String> {
        // Check cooldown
        if let Some(entity_cooldowns) = self.cooldowns.get(&entity) {
            if let Some(remaining_cooldown) = entity_cooldowns.get(&consumable.item_id) {
                if *remaining_cooldown > 0.0 {
                    return Err(format!("Consumable is on cooldown for {:.1} seconds", remaining_cooldown));
                }
            }
        }
        
        // Apply effects
        for effect in &consumable.effects {
            self.apply_consumable_effect(entity, consumable, effect)?;
        }
        
        // Set cooldown
        self.set_cooldown(entity, &consumable.item_id, consumable.cooldown);
        
        Ok(())
    }
    
    /// Apply a consumable effect to an entity
    fn apply_consumable_effect(&mut self, entity: Entity, consumable: &Consumable, effect: &ConsumableEffect) -> Result<(), String> {
        let active_effects = self.active_effects.entry(entity).or_insert_with(Vec::new);
        
        // Check if effect already exists
        if let Some(existing_effect) = active_effects.iter_mut().find(|ae| ae.effect.effect_type == effect.effect_type) {
            // Stack the effect if possible
            if existing_effect.max_stacks > 1 && existing_effect.stacks < existing_effect.max_stacks {
                existing_effect.stacks += 1;
                existing_effect.remaining_duration = effect.duration.unwrap_or(0.0);
            } else {
                // Refresh duration
                existing_effect.remaining_duration = effect.duration.unwrap_or(0.0);
            }
        } else {
            // Add new effect
            active_effects.push(ActiveConsumableEffect {
                consumable_id: consumable.item_id.clone(),
                effect: effect.clone(),
                remaining_duration: effect.duration.unwrap_or(0.0),
                stacks: 1,
                max_stacks: 1, // Default to 1, can be modified based on effect type
            });
        }
        
        Ok(())
    }
    
    /// Update active effects (call this every frame)
    pub fn update_effects(&mut self, delta_time: f32) {
        for (entity, effects) in self.active_effects.iter_mut() {
            effects.retain_mut(|effect| {
                effect.remaining_duration -= delta_time;
                effect.remaining_duration > 0.0
            });
        }
        
        // Remove empty effect lists
        self.active_effects.retain(|_, effects| !effects.is_empty());
        
        // Update cooldowns
        for (entity, cooldowns) in self.cooldowns.iter_mut() {
            for (_, cooldown) in cooldowns.iter_mut() {
                *cooldown = (*cooldown - delta_time).max(0.0);
            }
        }
    }
    
    /// Set cooldown for a consumable
    fn set_cooldown(&mut self, entity: Entity, consumable_id: &str, cooldown: f32) {
        self.cooldowns.entry(entity)
            .or_insert_with(HashMap::new)
            .insert(consumable_id.to_string(), cooldown);
    }
    
    /// Get remaining cooldown for a consumable
    pub fn get_remaining_cooldown(&self, entity: Entity, consumable_id: &str) -> f32 {
        self.cooldowns.get(&entity)
            .and_then(|cooldowns| cooldowns.get(consumable_id))
            .copied()
            .unwrap_or(0.0)
    }
    
    /// Get active effects for an entity
    pub fn get_active_effects(&self, entity: Entity) -> Option<&Vec<ActiveConsumableEffect>> {
        self.active_effects.get(&entity)
    }
    
    /// Remove all effects from an entity
    pub fn remove_all_effects(&mut self, entity: Entity) {
        self.active_effects.remove(&entity);
    }
    
    /// Remove specific effect from an entity
    pub fn remove_effect(&mut self, entity: Entity, effect_type: &ConsumableEffectType) {
        if let Some(effects) = self.active_effects.get_mut(&entity) {
            effects.retain(|effect| effect.effect.effect_type != *effect_type);
        }
    }
    
    /// Initialize default consumable templates
    pub fn initialize_default_templates(&mut self) {
        // Health potions
        self.add_template(ConsumableTemplate {
            id: "minor_health_potion".to_string(),
            name: "Minor Health Potion".to_string(),
            description: "Restores a small amount of health instantly.".to_string(),
            consumable_type: ConsumableType::Potion,
            rarity: ItemRarity::Common,
            effects: vec![
                ConsumableEffect {
                    effect_type: ConsumableEffectType::InstantHeal,
                    value: 25.0,
                    duration: None,
                    target: ConsumableTarget::SelfTarget,
                    condition: None,
                    description: "Restores 25 health instantly".to_string(),
                }
            ],
            duration: None,
            cooldown: 2.0,
            stackable: true,
            max_stack: 99,
            max_usage: 1,
            value: 25,
        });
        
        self.add_template(ConsumableTemplate {
            id: "health_potion".to_string(),
            name: "Health Potion".to_string(),
            description: "Restores a moderate amount of health instantly.".to_string(),
            consumable_type: ConsumableType::Potion,
            rarity: ItemRarity::Common,
            effects: vec![
                ConsumableEffect {
                    effect_type: ConsumableEffectType::InstantHeal,
                    value: 50.0,
                    duration: None,
                    target: ConsumableTarget::SelfTarget,
                    condition: None,
                    description: "Restores 50 health instantly".to_string(),
                }
            ],
            duration: None,
            cooldown: 3.0,
            stackable: true,
            max_stack: 99,
            max_usage: 1,
            value: 50,
        });
        
        self.add_template(ConsumableTemplate {
            id: "greater_health_potion".to_string(),
            name: "Greater Health Potion".to_string(),
            description: "Restores a large amount of health instantly.".to_string(),
            consumable_type: ConsumableType::Potion,
            rarity: ItemRarity::Uncommon,
            effects: vec![
                ConsumableEffect {
                    effect_type: ConsumableEffectType::InstantHeal,
                    value: 100.0,
                    duration: None,
                    target: ConsumableTarget::SelfTarget,
                    condition: None,
                    description: "Restores 100 health instantly".to_string(),
                }
            ],
            duration: None,
            cooldown: 5.0,
            stackable: true,
            max_stack: 99,
            max_usage: 1,
            value: 100,
        });
        
        // Mana potions
        self.add_template(ConsumableTemplate {
            id: "mana_potion".to_string(),
            name: "Mana Potion".to_string(),
            description: "Restores a moderate amount of mana instantly.".to_string(),
            consumable_type: ConsumableType::Potion,
            rarity: ItemRarity::Common,
            effects: vec![
                ConsumableEffect {
                    effect_type: ConsumableEffectType::InstantMana,
                    value: 30.0,
                    duration: None,
                    target: ConsumableTarget::SelfTarget,
                    condition: None,
                    description: "Restores 30 mana instantly".to_string(),
                }
            ],
            duration: None,
            cooldown: 3.0,
            stackable: true,
            max_stack: 99,
            max_usage: 1,
            value: 50,
        });
        
        // Stamina potions
        self.add_template(ConsumableTemplate {
            id: "stamina_potion".to_string(),
            name: "Stamina Potion".to_string(),
            description: "Restores stamina instantly.".to_string(),
            consumable_type: ConsumableType::Potion,
            rarity: ItemRarity::Common,
            effects: vec![
                ConsumableEffect {
                    effect_type: ConsumableEffectType::InstantStamina,
                    value: 40.0,
                    duration: None,
                    target: ConsumableTarget::SelfTarget,
                    condition: None,
                    description: "Restores 40 stamina instantly".to_string(),
                }
            ],
            duration: None,
            cooldown: 2.0,
            stackable: true,
            max_stack: 99,
            max_usage: 1,
            value: 30,
        });
        
        // Buff potions
        self.add_template(ConsumableTemplate {
            id: "strength_potion".to_string(),
            name: "Potion of Strength".to_string(),
            description: "Temporarily increases strength.".to_string(),
            consumable_type: ConsumableType::Potion,
            rarity: ItemRarity::Uncommon,
            effects: vec![
                ConsumableEffect {
                    effect_type: ConsumableEffectType::StrengthBonus,
                    value: 5.0,
                    duration: Some(300.0), // 5 minutes
                    target: ConsumableTarget::SelfTarget,
                    condition: None,
                    description: "Increases strength by 5 for 5 minutes".to_string(),
                }
            ],
            duration: Some(300.0),
            cooldown: 10.0,
            stackable: true,
            max_stack: 10,
            max_usage: 1,
            value: 75,
        });
        
        self.add_template(ConsumableTemplate {
            id: "speed_potion".to_string(),
            name: "Potion of Speed".to_string(),
            description: "Temporarily increases movement speed.".to_string(),
            consumable_type: ConsumableType::Potion,
            rarity: ItemRarity::Uncommon,
            effects: vec![
                ConsumableEffect {
                    effect_type: ConsumableEffectType::MovementSpeedBonus,
                    value: 0.5,
                    duration: Some(180.0), // 3 minutes
                    target: ConsumableTarget::SelfTarget,
                    condition: None,
                    description: "Increases movement speed by 50% for 3 minutes".to_string(),
                }
            ],
            duration: Some(180.0),
            cooldown: 15.0,
            stackable: true,
            max_stack: 10,
            max_usage: 1,
            value: 60,
        });
        
        // Food items
        self.add_template(ConsumableTemplate {
            id: "bread".to_string(),
            name: "Bread".to_string(),
            description: "A simple loaf of bread that restores health over time.".to_string(),
            consumable_type: ConsumableType::Food,
            rarity: ItemRarity::Common,
            effects: vec![
                ConsumableEffect {
                    effect_type: ConsumableEffectType::HealOverTime,
                    value: 2.0,
                    duration: Some(30.0),
                    target: ConsumableTarget::SelfTarget,
                    condition: None,
                    description: "Restores 2 health per second for 30 seconds".to_string(),
                }
            ],
            duration: Some(30.0),
            cooldown: 1.0,
            stackable: true,
            max_stack: 50,
            max_usage: 1,
            value: 5,
        });
        
        // Scrolls
        self.add_template(ConsumableTemplate {
            id: "teleport_scroll".to_string(),
            name: "Scroll of Teleportation".to_string(),
            description: "Allows instant teleportation to a safe location.".to_string(),
            consumable_type: ConsumableType::Scroll,
            rarity: ItemRarity::Rare,
            effects: vec![
                ConsumableEffect {
                    effect_type: ConsumableEffectType::Teleportation,
                    value: 1.0,
                    duration: None,
                    target: ConsumableTarget::SelfTarget,
                    condition: None,
                    description: "Teleports to nearest safe location".to_string(),
                }
            ],
            duration: None,
            cooldown: 30.0,
            stackable: true,
            max_stack: 5,
            max_usage: 1,
            value: 200,
        });
    }
}

impl Default for ConsumableManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Consumable {}
