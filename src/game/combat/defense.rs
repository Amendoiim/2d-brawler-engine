//! Defensive mechanics system for advanced combat

use crate::engine::ecs::{Component, System, World};
use std::collections::HashMap;

/// Defense component for tracking defensive state
#[derive(Clone)]
pub struct Defense {
    pub is_blocking: bool,
    pub is_parrying: bool,
    pub is_dodging: bool,
    pub block_timer: f32,
    pub parry_timer: f32,
    pub dodge_timer: f32,
    pub invincibility_timer: f32,
    pub block_damage_reduction: f32,
    pub parry_window: f32,
    pub dodge_duration: f32,
    pub dodge_distance: f32,
    pub dodge_direction: DodgeDirection,
    pub counter_attack_available: bool,
    pub counter_attack_timer: f32,
    pub counter_attack_duration: f32,
}

impl Default for Defense {
    fn default() -> Self {
        Self {
            is_blocking: false,
            is_parrying: false,
            is_dodging: false,
            block_timer: 0.0,
            parry_timer: 0.0,
            dodge_timer: 0.0,
            invincibility_timer: 0.0,
            block_damage_reduction: 0.5, // 50% damage reduction
            parry_window: 0.3, // 300ms parry window
            dodge_duration: 0.5, // 500ms dodge duration
            dodge_distance: 100.0, // 100 units dodge distance
            dodge_direction: DodgeDirection::Backward,
            counter_attack_available: false,
            counter_attack_timer: 0.0,
            counter_attack_duration: 1.0,
        }
    }
}

/// Dodge directions
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DodgeDirection {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

/// Block types
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlockType {
    Light,      // Quick block, less damage reduction
    Heavy,      // Strong block, more damage reduction
    Perfect,    // Perfect timing, maximum damage reduction
    Counter,    // Counter-attack block
}

/// Parry types
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ParryType {
    Light,      // Quick parry, short window
    Heavy,      // Strong parry, longer window
    Perfect,    // Perfect timing, maximum effect
    Counter,    // Counter-attack parry
}

/// Defense state
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DefenseState {
    Idle,
    Blocking,
    Parrying,
    Dodging,
    Invincible,
    Stunned,
    CounterAttacking,
}

/// Defense system for processing defensive mechanics
pub struct DefenseSystem {
    pub defense_effects: HashMap<String, DefenseEffect>,
    pub active_defenses: Vec<ActiveDefense>,
}

impl DefenseSystem {
    pub fn new() -> Self {
        let mut system = Self {
            defense_effects: HashMap::new(),
            active_defenses: Vec::new(),
        };
        system.initialize_defense_effects();
        system
    }

    fn initialize_defense_effects(&mut self) {
        // Block effects
        self.defense_effects.insert("light_block".to_string(), DefenseEffect {
            name: "Light Block".to_string(),
            damage_reduction: 0.3,
            stamina_cost: 5.0,
            duration: 0.5,
            cooldown: 0.2,
            effect_type: DefenseEffectType::Block,
        });

        self.defense_effects.insert("heavy_block".to_string(), DefenseEffect {
            name: "Heavy Block".to_string(),
            damage_reduction: 0.7,
            stamina_cost: 15.0,
            duration: 1.0,
            cooldown: 0.5,
            effect_type: DefenseEffectType::Block,
        });

        self.defense_effects.insert("perfect_block".to_string(), DefenseEffect {
            name: "Perfect Block".to_string(),
            damage_reduction: 0.9,
            stamina_cost: 10.0,
            duration: 0.3,
            cooldown: 0.1,
            effect_type: DefenseEffectType::Block,
        });

        // Parry effects
        self.defense_effects.insert("light_parry".to_string(), DefenseEffect {
            name: "Light Parry".to_string(),
            damage_reduction: 1.0,
            stamina_cost: 8.0,
            duration: 0.2,
            cooldown: 0.3,
            effect_type: DefenseEffectType::Parry,
        });

        self.defense_effects.insert("heavy_parry".to_string(), DefenseEffect {
            name: "Heavy Parry".to_string(),
            damage_reduction: 1.0,
            stamina_cost: 20.0,
            duration: 0.4,
            cooldown: 0.6,
            effect_type: DefenseEffectType::Parry,
        });

        self.defense_effects.insert("perfect_parry".to_string(), DefenseEffect {
            name: "Perfect Parry".to_string(),
            damage_reduction: 1.0,
            stamina_cost: 12.0,
            duration: 0.15,
            cooldown: 0.2,
            effect_type: DefenseEffectType::Parry,
        });

        // Dodge effects
        self.defense_effects.insert("quick_dodge".to_string(), DefenseEffect {
            name: "Quick Dodge".to_string(),
            damage_reduction: 1.0,
            stamina_cost: 10.0,
            duration: 0.3,
            cooldown: 0.4,
            effect_type: DefenseEffectType::Dodge,
        });

        self.defense_effects.insert("roll_dodge".to_string(), DefenseEffect {
            name: "Roll Dodge".to_string(),
            damage_reduction: 1.0,
            stamina_cost: 15.0,
            duration: 0.6,
            cooldown: 0.8,
            effect_type: DefenseEffectType::Dodge,
        });

        self.defense_effects.insert("perfect_dodge".to_string(), DefenseEffect {
            name: "Perfect Dodge".to_string(),
            damage_reduction: 1.0,
            stamina_cost: 8.0,
            duration: 0.2,
            cooldown: 0.3,
            effect_type: DefenseEffectType::Dodge,
        });
    }

    pub fn start_block(&mut self, defense: &mut Defense, block_type: BlockType) -> bool {
        if defense.is_blocking || defense.is_parrying || defense.is_dodging {
            return false;
        }

        let effect_key = match block_type {
            BlockType::Light => "light_block",
            BlockType::Heavy => "heavy_block",
            BlockType::Perfect => "perfect_block",
            BlockType::Counter => "perfect_block", // Use perfect block for counter
        };

        if let Some(effect) = self.defense_effects.get(effect_key) {
            defense.is_blocking = true;
            defense.block_timer = effect.duration;
            defense.block_damage_reduction = effect.damage_reduction;
            true
        } else {
            false
        }
    }

    pub fn start_parry(&mut self, defense: &mut Defense, parry_type: ParryType) -> bool {
        if defense.is_blocking || defense.is_parrying || defense.is_dodging {
            return false;
        }

        let effect_key = match parry_type {
            ParryType::Light => "light_parry",
            ParryType::Heavy => "heavy_parry",
            ParryType::Perfect => "perfect_parry",
            ParryType::Counter => "perfect_parry", // Use perfect parry for counter
        };

        if let Some(effect) = self.defense_effects.get(effect_key) {
            defense.is_parrying = true;
            defense.parry_timer = effect.duration;
            defense.parry_window = effect.duration;
            true
        } else {
            false
        }
    }

    pub fn start_dodge(&mut self, defense: &mut Defense, direction: DodgeDirection) -> bool {
        if defense.is_blocking || defense.is_parrying || defense.is_dodging {
            return false;
        }

        defense.is_dodging = true;
        defense.dodge_timer = defense.dodge_duration;
        defense.dodge_direction = direction;
        defense.invincibility_timer = defense.dodge_duration;
        true
    }

    pub fn update_defense(&mut self, defense: &mut Defense, dt: f32) {
        // Update block timer
        if defense.is_blocking {
            defense.block_timer -= dt;
            if defense.block_timer <= 0.0 {
                defense.is_blocking = false;
            }
        }

        // Update parry timer
        if defense.is_parrying {
            defense.parry_timer -= dt;
            if defense.parry_timer <= 0.0 {
                defense.is_parrying = false;
            }
        }

        // Update dodge timer
        if defense.is_dodging {
            defense.dodge_timer -= dt;
            if defense.dodge_timer <= 0.0 {
                defense.is_dodging = false;
            }
        }

        // Update invincibility timer
        if defense.invincibility_timer > 0.0 {
            defense.invincibility_timer -= dt;
        }

        // Update counter attack timer
        if defense.counter_attack_timer > 0.0 {
            defense.counter_attack_timer -= dt;
            if defense.counter_attack_timer <= 0.0 {
                defense.counter_attack_available = false;
            }
        }
    }

    pub fn process_attack(&mut self, defense: &mut Defense, attack: &Attack) -> DefenseResult {
        if defense.invincibility_timer > 0.0 {
            return DefenseResult::Invincible;
        }

        if defense.is_parrying && defense.parry_timer > 0.0 {
            defense.counter_attack_available = true;
            defense.counter_attack_timer = defense.counter_attack_duration;
            return DefenseResult::Parried { counter_available: true };
        }

        if defense.is_dodging {
            return DefenseResult::Dodged;
        }

        if defense.is_blocking {
            let reduced_damage = attack.damage * (1.0 - defense.block_damage_reduction);
            return DefenseResult::Blocked { 
                damage_taken: reduced_damage,
                damage_reduced: attack.damage - reduced_damage,
            };
        }

        DefenseResult::Hit { damage_taken: attack.damage }
    }

    pub fn can_counter_attack(&self, defense: &Defense) -> bool {
        defense.counter_attack_available && defense.counter_attack_timer > 0.0
    }

    pub fn execute_counter_attack(&mut self, defense: &mut Defense) -> bool {
        if self.can_counter_attack(defense) {
            defense.counter_attack_available = false;
            defense.counter_attack_timer = 0.0;
            true
        } else {
            false
        }
    }
}

/// Defense effect definition
#[derive(Clone)]
pub struct DefenseEffect {
    pub name: String,
    pub damage_reduction: f32,
    pub stamina_cost: f32,
    pub duration: f32,
    pub cooldown: f32,
    pub effect_type: DefenseEffectType,
}

#[derive(Clone)]
pub enum DefenseEffectType {
    Block,
    Parry,
    Dodge,
    Counter,
}

/// Active defense tracking
#[derive(Clone)]
pub struct ActiveDefense {
    pub entity_id: u32,
    pub defense_type: DefenseEffectType,
    pub remaining_time: f32,
    pub effect: DefenseEffect,
}

/// Attack structure for defense processing
#[derive(Clone)]
pub struct Attack {
    pub damage: f32,
    pub damage_type: DamageType,
    pub direction: AttackDirection,
    pub speed: f32,
    pub range: f32,
}

#[derive(Clone)]
pub enum DamageType {
    Physical,
    Magical,
    Fire,
    Ice,
    Lightning,
    Poison,
    Dark,
    Light,
}

#[derive(Clone)]
pub enum AttackDirection {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
    All,
}

/// Result of processing an attack against defense
#[derive(Clone)]
pub enum DefenseResult {
    Hit { damage_taken: f32 },
    Blocked { damage_taken: f32, damage_reduced: f32 },
    Parried { counter_available: bool },
    Dodged,
    Invincible,
}

impl System for DefenseSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // Update all active defenses
        // Process defense timers
        // Handle counter attacks
        // Apply defense effects
    }
}

/// Defense stance manager
pub struct DefenseStanceManager {
    pub stances: HashMap<String, DefenseStance>,
    pub current_stance: Option<String>,
}

impl DefenseStanceManager {
    pub fn new() -> Self {
        let mut manager = Self {
            stances: HashMap::new(),
            current_stance: None,
        };
        manager.initialize_stances();
        manager
    }

    fn initialize_stances(&mut self) {
        // Offensive stance
        self.stances.insert("offensive".to_string(), DefenseStance {
            name: "Offensive".to_string(),
            block_damage_reduction: 0.3,
            parry_window: 0.2,
            dodge_duration: 0.4,
            counter_attack_damage: 1.5,
            stamina_cost_multiplier: 0.8,
        });

        // Defensive stance
        self.stances.insert("defensive".to_string(), DefenseStance {
            name: "Defensive".to_string(),
            block_damage_reduction: 0.8,
            parry_window: 0.4,
            dodge_duration: 0.6,
            counter_attack_damage: 1.2,
            stamina_cost_multiplier: 1.2,
        });

        // Balanced stance
        self.stances.insert("balanced".to_string(), DefenseStance {
            name: "Balanced".to_string(),
            block_damage_reduction: 0.5,
            parry_window: 0.3,
            dodge_duration: 0.5,
            counter_attack_damage: 1.3,
            stamina_cost_multiplier: 1.0,
        });

        // Counter stance
        self.stances.insert("counter".to_string(), DefenseStance {
            name: "Counter".to_string(),
            block_damage_reduction: 0.4,
            parry_window: 0.5,
            dodge_duration: 0.3,
            counter_attack_damage: 2.0,
            stamina_cost_multiplier: 1.5,
        });
    }

    pub fn set_stance(&mut self, stance_name: &str) -> bool {
        if self.stances.contains_key(stance_name) {
            self.current_stance = Some(stance_name.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_stance(&self) -> Option<&DefenseStance> {
        self.current_stance.as_ref()
            .and_then(|name| self.stances.get(name))
    }
}

/// Defense stance definition
#[derive(Clone)]
pub struct DefenseStance {
    pub name: String,
    pub block_damage_reduction: f32,
    pub parry_window: f32,
    pub dodge_duration: f32,
    pub counter_attack_damage: f32,
    pub stamina_cost_multiplier: f32,
}

impl Component for Defense {}
