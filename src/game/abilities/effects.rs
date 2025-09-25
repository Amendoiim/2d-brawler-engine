//! Ability effects system for character abilities

use crate::engine::ecs::{Component, System, World};
use std::collections::HashMap;

/// Ability effect types
#[derive(Clone)]
pub enum AbilityEffect {
    // Damage effects
    DamageBonus { multiplier: f32 },
    ElementalDamageBonus { element: ElementType, multiplier: f32 },
    CriticalChanceBonus { multiplier: f32 },
    CriticalDamageBonus { multiplier: f32 },
    AreaAttack { radius: f32 },
    ExplosiveDamage { radius: f32 },
    MeteorStrike { damage: f32, radius: f32 },
    TeleportAttack { damage: f32 },

    // Defense effects
    DefenseBonus { multiplier: f32 },
    BlockBonus { multiplier: f32 },
    ShieldWall { duration: f32, damage_reduction: f32 },
    DamageReduction { multiplier: f32 },
    Invincibility { duration: f32 },

    // Movement effects
    SpeedBonus { multiplier: f32 },
    SpeedPenalty { multiplier: f32 },
    JumpBonus { multiplier: f32 },
    Teleport { range: f32 },

    // Resource effects
    ManaBonus { multiplier: f32 },
    ManaRegenBonus { multiplier: f32 },
    StaminaBonus { multiplier: f32 },
    StaminaRegenBonus { multiplier: f32 },
    StaminaCostReduction { multiplier: f32 },

    // Utility effects
    Invisibility { duration: f32 },
    Taunt { range: f32, duration: f32 },
    RangeBonus { multiplier: f32 },
    AccuracyBonus { multiplier: f32 },
    AttackSpeedBonus { multiplier: f32 },

    // Status effects
    StatusImmunity { status_type: StatusType },
    StatusResistance { status_type: StatusType, multiplier: f32 },
    StatusDurationReduction { status_type: StatusType, multiplier: f32 },

    // Special effects
    LifeSteal { multiplier: f32 },
    ManaSteal { multiplier: f32 },
    ReflectDamage { multiplier: f32 },
    Thorns { damage: f32 },
    Regeneration { health_per_second: f32, duration: f32 },
    ManaRegeneration { mana_per_second: f32, duration: f32 },
}

/// Status types for ability effects
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum StatusType {
    Stun,
    Slow,
    Poison,
    Burn,
    Freeze,
    Confusion,
    Fear,
    Charm,
    Silence,
    Blind,
    Deaf,
    Paralyze,
    Sleep,
    Bleed,
    Curse,
    Bless,
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

/// Ability effect processor
pub struct AbilityEffectProcessor {
    pub active_effects: HashMap<String, ActiveEffect>,
    pub effect_counter: u32,
}

impl AbilityEffectProcessor {
    pub fn new() -> Self {
        Self {
            active_effects: HashMap::new(),
            effect_counter: 0,
        }
    }

    pub fn apply_effect(&mut self, effect: AbilityEffect, target: u32, duration: f32) -> String {
        let effect_id = format!("effect_{}", self.effect_counter);
        self.effect_counter += 1;

        let active_effect = ActiveEffect {
            id: effect_id.clone(),
            effect,
            target,
            duration,
            remaining_time: duration,
            is_permanent: duration <= 0.0,
        };

        self.active_effects.insert(effect_id.clone(), active_effect);
        effect_id
    }

    pub fn remove_effect(&mut self, effect_id: &str) -> bool {
        self.active_effects.remove(effect_id).is_some()
    }

    pub fn update_effects(&mut self, dt: f32) {
        let mut to_remove = Vec::new();

        for (effect_id, active_effect) in &mut self.active_effects {
            if !active_effect.is_permanent {
                active_effect.remaining_time -= dt;
                if active_effect.remaining_time <= 0.0 {
                    to_remove.push(effect_id.clone());
                }
            }
        }

        for effect_id in to_remove {
            self.active_effects.remove(&effect_id);
        }
    }

    pub fn get_effects_for_target(&self, target: u32) -> Vec<&ActiveEffect> {
        self.active_effects.values()
            .filter(|effect| effect.target == target)
            .collect()
    }

    pub fn calculate_damage_bonus(&self, target: u32) -> f32 {
        let mut bonus = 1.0;
        
        for effect in self.get_effects_for_target(target) {
            match &effect.effect {
                AbilityEffect::DamageBonus { multiplier } => {
                    bonus += multiplier;
                },
                AbilityEffect::ElementalDamageBonus { multiplier, .. } => {
                    bonus += multiplier;
                },
                _ => {},
            }
        }

        bonus
    }

    pub fn calculate_defense_bonus(&self, target: u32) -> f32 {
        let mut bonus = 1.0;
        
        for effect in self.get_effects_for_target(target) {
            match &effect.effect {
                AbilityEffect::DefenseBonus { multiplier } => {
                    bonus += multiplier;
                },
                AbilityEffect::BlockBonus { multiplier } => {
                    bonus += multiplier;
                },
                _ => {},
            }
        }

        bonus
    }

    pub fn calculate_speed_bonus(&self, target: u32) -> f32 {
        let mut bonus = 1.0;
        
        for effect in self.get_effects_for_target(target) {
            match &effect.effect {
                AbilityEffect::SpeedBonus { multiplier } => {
                    bonus += multiplier;
                },
                AbilityEffect::SpeedPenalty { multiplier } => {
                    bonus += multiplier; // multiplier is negative
                },
                _ => {},
            }
        }

        bonus
    }

    pub fn calculate_mana_bonus(&self, target: u32) -> f32 {
        let mut bonus = 1.0;
        
        for effect in self.get_effects_for_target(target) {
            match &effect.effect {
                AbilityEffect::ManaBonus { multiplier } => {
                    bonus += multiplier;
                },
                _ => {},
            }
        }

        bonus
    }

    pub fn is_immune_to_status(&self, target: u32, status_type: StatusType) -> bool {
        for effect in self.get_effects_for_target(target) {
            match &effect.effect {
                AbilityEffect::StatusImmunity { status_type: immune_type } => {
                    if *immune_type == status_type {
                        return true;
                    }
                },
                _ => {},
            }
        }
        false
    }

    pub fn get_status_resistance(&self, target: u32, status_type: StatusType) -> f32 {
        let mut resistance = 1.0;
        
        for effect in self.get_effects_for_target(target) {
            match &effect.effect {
                AbilityEffect::StatusResistance { status_type: resistant_type, multiplier } => {
                    if *resistant_type == status_type {
                        resistance *= 1.0 - multiplier;
                    }
                },
                _ => {},
            }
        }

        resistance
    }
}

/// Active effect instance
#[derive(Clone)]
pub struct ActiveEffect {
    pub id: String,
    pub effect: AbilityEffect,
    pub target: u32,
    pub duration: f32,
    pub remaining_time: f32,
    pub is_permanent: bool,
}

/// Ability effect system
pub struct AbilityEffectSystem {
    pub processor: AbilityEffectProcessor,
    pub effect_definitions: HashMap<String, EffectDefinition>,
}

impl AbilityEffectSystem {
    pub fn new() -> Self {
        let mut system = Self {
            processor: AbilityEffectProcessor::new(),
            effect_definitions: HashMap::new(),
        };
        system.initialize_effect_definitions();
        system
    }

    fn initialize_effect_definitions(&mut self) {
        // Initialize effect definitions for different ability types
        self.effect_definitions.insert("damage_boost".to_string(), EffectDefinition {
            name: "Damage Boost".to_string(),
            description: "Increases damage dealt".to_string(),
            effect_type: EffectType::Damage,
            duration: 10.0,
            stackable: true,
            max_stacks: 5,
        });

        self.effect_definitions.insert("defense_boost".to_string(), EffectDefinition {
            name: "Defense Boost".to_string(),
            description: "Increases defense".to_string(),
            effect_type: EffectType::Defense,
            duration: 15.0,
            stackable: true,
            max_stacks: 3,
        });

        self.effect_definitions.insert("speed_boost".to_string(), EffectDefinition {
            name: "Speed Boost".to_string(),
            description: "Increases movement speed".to_string(),
            effect_type: EffectType::Movement,
            duration: 8.0,
            stackable: false,
            max_stacks: 1,
        });

        self.effect_definitions.insert("invisibility".to_string(), EffectDefinition {
            name: "Invisibility".to_string(),
            description: "Makes character invisible".to_string(),
            effect_type: EffectType::Utility,
            duration: 5.0,
            stackable: false,
            max_stacks: 1,
        });
    }

    pub fn apply_ability_effect(&mut self, ability: &super::Ability, target: u32) -> Vec<String> {
        let mut applied_effects = Vec::new();

        for effect in &ability.effects {
        let duration = match effect {
            AbilityEffect::Invisibility { duration } => *duration,
            AbilityEffect::ShieldWall { duration, .. } => *duration,
            AbilityEffect::Regeneration { duration, .. } => *duration,
            AbilityEffect::ManaRegeneration { duration, .. } => *duration,
            _ => 0.0, // Permanent effects
        };

            let effect_id = self.processor.apply_effect(effect.clone(), target, duration);
            applied_effects.push(effect_id);
        }

        applied_effects
    }

    pub fn remove_ability_effects(&mut self, effect_ids: &[String]) {
        for effect_id in effect_ids {
            self.processor.remove_effect(effect_id);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.processor.update_effects(dt);
    }
}

/// Effect definition
#[derive(Clone)]
pub struct EffectDefinition {
    pub name: String,
    pub description: String,
    pub effect_type: EffectType,
    pub duration: f32,
    pub stackable: bool,
    pub max_stacks: u32,
}

#[derive(Clone)]
pub enum EffectType {
    Damage,
    Defense,
    Movement,
    Resource,
    Utility,
    Status,
    Special,
}

impl System for AbilityEffectSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        self.update(dt);
    }
}

/// Ability effect visual feedback
pub struct AbilityEffectVisuals {
    pub effect_particles: HashMap<String, Vec<ParticleEffect>>,
    pub effect_auras: HashMap<String, AuraEffect>,
    pub effect_animations: HashMap<String, String>,
}

impl AbilityEffectVisuals {
    pub fn new() -> Self {
        Self {
            effect_particles: HashMap::new(),
            effect_auras: HashMap::new(),
            effect_animations: HashMap::new(),
        }
    }

    pub fn create_effect_visual(&mut self, effect: &AbilityEffect, target: u32) {
        match effect {
            AbilityEffect::Invisibility { .. } => {
                self.create_invisibility_effect(target);
            },
            AbilityEffect::ShieldWall { .. } => {
                self.create_shield_wall_effect(target);
            },
            AbilityEffect::Regeneration { .. } => {
                self.create_regeneration_effect(target);
            },
            AbilityEffect::ManaRegeneration { .. } => {
                self.create_mana_regeneration_effect(target);
            },
            _ => {},
        }
    }

    fn create_invisibility_effect(&mut self, target: u32) {
        // Create invisibility visual effect
        let particles = vec![
            ParticleEffect {
                particle_type: "stealth".to_string(),
                color: (0.5, 0.5, 0.5, 0.3),
                size: 10.0,
                duration: 3.0,
            }
        ];
        self.effect_particles.insert(format!("invisibility_{}", target), particles);
    }

    fn create_shield_wall_effect(&mut self, target: u32) {
        // Create shield wall visual effect
        let aura = AuraEffect {
            aura_type: "shield".to_string(),
            color: (0.0, 0.5, 1.0, 0.5),
            radius: 100.0,
            duration: 10.0,
        };
        self.effect_auras.insert(format!("shield_wall_{}", target), aura);
    }

    fn create_regeneration_effect(&mut self, target: u32) {
        // Create regeneration visual effect
        let particles = vec![
            ParticleEffect {
                particle_type: "heal".to_string(),
                color: (0.0, 1.0, 0.0, 0.7),
                size: 5.0,
                duration: 1.0,
            }
        ];
        self.effect_particles.insert(format!("regeneration_{}", target), particles);
    }

    fn create_mana_regeneration_effect(&mut self, target: u32) {
        // Create mana regeneration visual effect
        let particles = vec![
            ParticleEffect {
                particle_type: "mana".to_string(),
                color: (0.0, 0.5, 1.0, 0.7),
                size: 5.0,
                duration: 1.0,
            }
        ];
        self.effect_particles.insert(format!("mana_regeneration_{}", target), particles);
    }
}

/// Particle effect for visual feedback
#[derive(Clone)]
pub struct ParticleEffect {
    pub particle_type: String,
    pub color: (f32, f32, f32, f32),
    pub size: f32,
    pub duration: f32,
}

/// Aura effect for visual feedback
#[derive(Clone)]
pub struct AuraEffect {
    pub aura_type: String,
    pub color: (f32, f32, f32, f32),
    pub radius: f32,
    pub duration: f32,
}
