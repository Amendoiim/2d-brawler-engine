//! Combo system implementation for advanced combat mechanics

use crate::engine::ecs::{Component, System, World};
use std::collections::HashMap;
use std::time::Instant;

/// Combo component for tracking combo state
#[derive(Clone)]
pub struct Combo {
    pub current_combo: Vec<ComboInput>,
    pub combo_timer: f32,
    pub max_combo_timer: f32,
    pub combo_multiplier: f32,
    pub max_combo_multiplier: f32,
    pub combo_break_timer: f32,
    pub combo_chain_count: u32,
    pub last_input_time: f32,
}

impl Default for Combo {
    fn default() -> Self {
        Self {
            current_combo: Vec::new(),
            combo_timer: 0.0,
            max_combo_timer: 2.0, // 2 seconds to continue combo
            combo_multiplier: 1.0,
            max_combo_multiplier: 5.0,
            combo_break_timer: 0.0,
            combo_chain_count: 0,
            last_input_time: 0.0,
        }
    }
}

/// Input types for combo system
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ComboInput {
    LightAttack,
    HeavyAttack,
    SpecialMove,
    Block,
    Dodge,
    Jump,
    Crouch,
}

/// Combo sequence definition
#[derive(Clone, Debug)]
pub struct ComboSequence {
    pub inputs: Vec<ComboInput>,
    pub name: String,
    pub damage_multiplier: f32,
    pub special_effect: Option<ComboEffect>,
    pub unlock_requirement: ComboRequirement,
}

/// Special effects that can be triggered by combos
#[derive(Clone, Debug)]
pub enum ComboEffect {
    Knockback { force: f32 },
    Stun { duration: f32 },
    Bleed { damage_per_second: f32, duration: f32 },
    Burn { damage_per_second: f32, duration: f32 },
    Freeze { duration: f32 },
    Heal { amount: f32 },
    ManaRestore { amount: f32 },
    Invincibility { duration: f32 },
}

/// Requirements for unlocking combos
#[derive(Clone, Debug)]
pub enum ComboRequirement {
    Level { min_level: u32 },
    Ability { ability_name: String, min_level: u32 },
    CharacterClass { class: String },
    None,
}

/// Combo manager for handling combo sequences
pub struct ComboManager {
    pub combo_sequences: HashMap<String, ComboSequence>,
    pub combo_templates: Vec<ComboTemplate>,
}

impl ComboManager {
    pub fn new() -> Self {
        let mut manager = Self {
            combo_sequences: HashMap::new(),
            combo_templates: Vec::new(),
        };
        manager.initialize_default_combos();
        manager
    }

    fn initialize_default_combos(&mut self) {
        // Basic combo sequences
        self.add_combo_sequence(ComboSequence {
            inputs: vec![ComboInput::LightAttack, ComboInput::LightAttack],
            name: "Double Strike".to_string(),
            damage_multiplier: 1.5,
            special_effect: None,
            unlock_requirement: ComboRequirement::None,
        });

        self.add_combo_sequence(ComboSequence {
            inputs: vec![ComboInput::LightAttack, ComboInput::HeavyAttack],
            name: "Light Heavy".to_string(),
            damage_multiplier: 2.0,
            special_effect: Some(ComboEffect::Knockback { force: 100.0 }),
            unlock_requirement: ComboRequirement::None,
        });

        self.add_combo_sequence(ComboSequence {
            inputs: vec![ComboInput::HeavyAttack, ComboInput::LightAttack, ComboInput::LightAttack],
            name: "Heavy Combo".to_string(),
            damage_multiplier: 2.5,
            special_effect: Some(ComboEffect::Stun { duration: 1.0 }),
            unlock_requirement: ComboRequirement::Level { min_level: 5 },
        });

        self.add_combo_sequence(ComboSequence {
            inputs: vec![ComboInput::LightAttack, ComboInput::LightAttack, ComboInput::HeavyAttack],
            name: "Triple Strike".to_string(),
            damage_multiplier: 3.0,
            special_effect: Some(ComboEffect::Bleed { 
                damage_per_second: 10.0, 
                duration: 5.0 
            }),
            unlock_requirement: ComboRequirement::Level { min_level: 10 },
        });

        self.add_combo_sequence(ComboSequence {
            inputs: vec![ComboInput::SpecialMove, ComboInput::LightAttack, ComboInput::HeavyAttack],
            name: "Special Combo".to_string(),
            damage_multiplier: 4.0,
            special_effect: Some(ComboEffect::Burn { 
                damage_per_second: 15.0, 
                duration: 8.0 
            }),
            unlock_requirement: ComboRequirement::Ability { 
                ability_name: "Special Move Mastery".to_string(), 
                min_level: 3 
            },
        });

        // Advanced combos
        self.add_combo_sequence(ComboSequence {
            inputs: vec![
                ComboInput::LightAttack, 
                ComboInput::LightAttack, 
                ComboInput::LightAttack, 
                ComboInput::HeavyAttack
            ],
            name: "Quad Strike".to_string(),
            damage_multiplier: 4.5,
            special_effect: Some(ComboEffect::Freeze { duration: 2.0 }),
            unlock_requirement: ComboRequirement::Level { min_level: 15 },
        });

        self.add_combo_sequence(ComboSequence {
            inputs: vec![
                ComboInput::Dodge, 
                ComboInput::LightAttack, 
                ComboInput::HeavyAttack
            ],
            name: "Counter Strike".to_string(),
            damage_multiplier: 3.5,
            special_effect: Some(ComboEffect::Invincibility { duration: 0.5 }),
            unlock_requirement: ComboRequirement::Ability { 
                ability_name: "Counter Attack".to_string(), 
                min_level: 2 
            },
        });
    }

    pub fn add_combo_sequence(&mut self, sequence: ComboSequence) {
        let key = self.generate_combo_key(&sequence.inputs);
        self.combo_sequences.insert(key, sequence);
    }

    fn generate_combo_key(&self, inputs: &[ComboInput]) -> String {
        inputs.iter()
            .map(|input| format!("{:?}", input))
            .collect::<Vec<String>>()
            .join("_")
    }

    pub fn check_combo(&self, combo: &Combo) -> Option<&ComboSequence> {
        if combo.current_combo.is_empty() {
            return None;
        }

        let key = self.generate_combo_key(&combo.current_combo);
        self.combo_sequences.get(&key)
    }

    pub fn get_available_combos(&self, character_level: u32, abilities: &[String]) -> Vec<&ComboSequence> {
        self.combo_sequences.values()
            .filter(|combo| self.can_use_combo(combo, character_level, abilities))
            .collect()
    }

    fn can_use_combo(&self, combo: &ComboSequence, character_level: u32, abilities: &[String]) -> bool {
        match &combo.unlock_requirement {
            ComboRequirement::Level { min_level } => character_level >= *min_level,
            ComboRequirement::Ability { ability_name, min_level } => {
                abilities.contains(ability_name) // Simplified - would need to check ability levels
            },
            ComboRequirement::CharacterClass { class: _ } => true, // Simplified
            ComboRequirement::None => true,
        }
    }
}

/// Combo template for procedural combo generation
#[derive(Clone)]
pub struct ComboTemplate {
    pub input_pattern: Vec<ComboInput>,
    pub damage_range: (f32, f32),
    pub effect_types: Vec<ComboEffectType>,
    pub difficulty: ComboDifficulty,
}

#[derive(Clone)]
pub enum ComboEffectType {
    Damage,
    Status,
    Utility,
    Defensive,
}

#[derive(Clone)]
pub enum ComboDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

/// Combo system for processing combo logic
pub struct ComboSystem;

impl System for ComboSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // Update combo timers
        // Process combo inputs
        // Check for combo completion
        // Apply combo effects
        // Reset expired combos
    }
}

/// Combo input processor
pub struct ComboInputProcessor {
    pub combo_manager: ComboManager,
}

impl ComboInputProcessor {
    pub fn new() -> Self {
        Self {
            combo_manager: ComboManager::new(),
        }
    }

    pub fn process_input(&mut self, combo: &mut Combo, input: ComboInput, current_time: f32) -> ComboResult {
        // Check if combo timer has expired
        if current_time - combo.last_input_time > combo.max_combo_timer {
            combo.current_combo.clear();
            combo.combo_chain_count = 0;
            combo.combo_multiplier = 1.0;
        }

        // Add input to current combo
        combo.current_combo.push(input);
        combo.last_input_time = current_time;
        combo.combo_timer = combo.max_combo_timer;

        // Check if current combo matches any known sequence
        if let Some(sequence) = self.combo_manager.check_combo(combo) {
            combo.combo_chain_count += 1;
            combo.combo_multiplier = (combo.combo_multiplier + 0.2).min(combo.max_combo_multiplier);
            
            ComboResult::ComboHit {
                sequence: sequence.clone(),
                multiplier: combo.combo_multiplier,
                chain_count: combo.combo_chain_count,
            }
        } else {
            // Check if this could be the start of a combo
            let possible_combos = self.combo_manager.get_available_combos(1, &[]);
            let is_valid_start = possible_combos.iter()
                .any(|combo_seq| !combo_seq.inputs.is_empty() && combo_seq.inputs[0] == input);

            if is_valid_start {
                ComboResult::ComboStart
            } else {
                ComboResult::ComboMiss
            }
        }
    }

    pub fn reset_combo(&self, combo: &mut Combo) {
        combo.current_combo.clear();
        combo.combo_chain_count = 0;
        combo.combo_multiplier = 1.0;
        combo.combo_timer = 0.0;
    }
}

/// Result of processing a combo input
#[derive(Clone, Debug)]
pub enum ComboResult {
    ComboStart,
    ComboHit {
        sequence: ComboSequence,
        multiplier: f32,
        chain_count: u32,
    },
    ComboMiss,
    ComboBreak,
}

impl Component for Combo {}

/// Combo visual feedback system
pub struct ComboVisualFeedback {
    pub combo_text_scale: f32,
    pub combo_text_color: (f32, f32, f32, f32),
    pub combo_text_duration: f32,
    pub combo_effect_particles: bool,
}

impl ComboVisualFeedback {
    pub fn new() -> Self {
        Self {
            combo_text_scale: 1.0,
            combo_text_color: (1.0, 1.0, 0.0, 1.0), // Yellow
            combo_text_duration: 2.0,
            combo_effect_particles: true,
        }
    }

    pub fn update_combo_display(&mut self, combo: &Combo, dt: f32) {
        // Update combo display based on current combo state
        if combo.combo_chain_count > 0 {
            self.combo_text_scale = 1.0 + (combo.combo_chain_count as f32 * 0.1);
            self.combo_text_color = self.get_combo_color(combo.combo_chain_count);
        }
    }

    fn get_combo_color(&self, chain_count: u32) -> (f32, f32, f32, f32) {
        match chain_count {
            1..=2 => (1.0, 1.0, 0.0, 1.0), // Yellow
            3..=4 => (1.0, 0.5, 0.0, 1.0), // Orange
            5..=7 => (1.0, 0.0, 0.0, 1.0), // Red
            8..=10 => (0.5, 0.0, 1.0, 1.0), // Purple
            _ => (1.0, 1.0, 1.0, 1.0), // White
        }
    }
}
