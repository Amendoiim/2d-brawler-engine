//! Special moves system for advanced combat mechanics

use crate::engine::ecs::{Component, System, World};
use std::collections::HashMap;
use std::time::Instant;

/// Special move component for tracking special abilities
#[derive(Clone)]
pub struct SpecialMove {
    pub move_id: String,
    pub name: String,
    pub cooldown_timer: f32,
    pub cooldown_duration: f32,
    pub resource_cost: f32,
    pub resource_type: ResourceType,
    pub is_available: bool,
    pub is_executing: bool,
    pub execution_timer: f32,
    pub execution_duration: f32,
    pub level: u32,
    pub max_level: u32,
}

impl SpecialMove {
    pub fn new(move_id: String, name: String, cooldown_duration: f32, resource_cost: f32, resource_type: ResourceType) -> Self {
        Self {
            move_id,
            name,
            cooldown_timer: 0.0,
            cooldown_duration,
            resource_cost,
            resource_type,
            is_available: true,
            is_executing: false,
            execution_timer: 0.0,
            execution_duration: 0.0,
            level: 1,
            max_level: 10,
        }
    }

    pub fn can_execute(&self, available_resources: &HashMap<ResourceType, f32>) -> bool {
        self.is_available && 
        self.cooldown_timer <= 0.0 && 
        available_resources.get(&self.resource_type).map_or(false, |&amount| amount >= self.resource_cost)
    }

    pub fn start_execution(&mut self) {
        self.is_executing = true;
        self.is_available = false;
        self.execution_timer = 0.0;
    }

    pub fn update(&mut self, dt: f32) {
        // Update cooldown timer
        if self.cooldown_timer > 0.0 {
            self.cooldown_timer -= dt;
            if self.cooldown_timer <= 0.0 {
                self.is_available = true;
            }
        }

        // Update execution timer
        if self.is_executing {
            self.execution_timer += dt;
            if self.execution_timer >= self.execution_duration {
                self.is_executing = false;
                self.cooldown_timer = self.cooldown_duration;
            }
        }
    }
}

/// Resource types for special moves
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ResourceType {
    Mana,
    Stamina,
    Rage,
    Energy,
    Focus,
}

/// Special move effects
#[derive(Clone)]
pub enum SpecialMoveEffect {
    Damage {
        base_damage: f32,
        damage_type: DamageType,
        area_of_effect: Option<AreaOfEffect>,
    },
    Heal {
        amount: f32,
        target: HealTarget,
    },
    Buff {
        buff_type: BuffType,
        duration: f32,
        strength: f32,
    },
    Debuff {
        debuff_type: DebuffType,
        duration: f32,
        strength: f32,
    },
    Movement {
        movement_type: MovementType,
        distance: f32,
        speed: f32,
    },
    Summon {
        entity_type: String,
        count: u32,
        duration: f32,
    },
    Transform {
        form: String,
        duration: f32,
    },
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
pub struct AreaOfEffect {
    pub shape: AoeShape,
    pub radius: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone)]
pub enum AoeShape {
    Circle,
    Rectangle,
    Line,
    Cone,
}

#[derive(Clone)]
pub enum HealTarget {
    SelfTarget,
    Ally,
    AllAllies,
    Area,
}

#[derive(Clone)]
pub enum BuffType {
    Strength,
    Agility,
    Intelligence,
    Defense,
    Speed,
    CriticalChance,
    CriticalDamage,
    DamageReduction,
    Regeneration,
    Invincibility,
}

#[derive(Clone)]
pub enum DebuffType {
    Weakness,
    Slow,
    Stun,
    Silence,
    Blind,
    Poison,
    Burn,
    Freeze,
    Confusion,
    Fear,
}

#[derive(Clone)]
pub enum MovementType {
    Teleport,
    Dash,
    Jump,
    Fly,
    Phase,
}

/// Special move definition
#[derive(Clone)]
pub struct SpecialMoveDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cooldown_duration: f32,
    pub resource_cost: f32,
    pub resource_type: ResourceType,
    pub execution_duration: f32,
    pub effects: Vec<SpecialMoveEffect>,
    pub requirements: SpecialMoveRequirement,
    pub animation: String,
    pub sound_effect: String,
    pub visual_effect: String,
}

/// Requirements for using special moves
#[derive(Clone)]
pub enum SpecialMoveRequirement {
    Level { min_level: u32 },
    CharacterClass { class: String },
    Ability { ability_name: String, min_level: u32 },
    Item { item_name: String },
    None,
}

/// Special move manager
pub struct SpecialMoveManager {
    pub move_definitions: HashMap<String, SpecialMoveDefinition>,
    pub character_moves: HashMap<String, Vec<String>>, // Character class -> move IDs
}

impl SpecialMoveManager {
    pub fn new() -> Self {
        let mut manager = Self {
            move_definitions: HashMap::new(),
            character_moves: HashMap::new(),
        };
        manager.initialize_default_moves();
        manager
    }

    fn initialize_default_moves(&mut self) {
        // Fighter moves
        self.add_move_definition(SpecialMoveDefinition {
            id: "power_strike".to_string(),
            name: "Power Strike".to_string(),
            description: "A devastating melee attack with increased damage".to_string(),
            cooldown_duration: 3.0,
            resource_cost: 20.0,
            resource_type: ResourceType::Stamina,
            execution_duration: 1.0,
            effects: vec![SpecialMoveEffect::Damage {
                base_damage: 150.0,
                damage_type: DamageType::Physical,
                area_of_effect: None,
            }],
            requirements: SpecialMoveRequirement::CharacterClass { class: "Fighter".to_string() },
            animation: "power_strike".to_string(),
            sound_effect: "power_strike_sound".to_string(),
            visual_effect: "power_strike_effect".to_string(),
        });

        self.add_move_definition(SpecialMoveDefinition {
            id: "whirlwind_attack".to_string(),
            name: "Whirlwind Attack".to_string(),
            description: "Spin attack that hits all enemies in range".to_string(),
            cooldown_duration: 5.0,
            resource_cost: 30.0,
            resource_type: ResourceType::Stamina,
            execution_duration: 2.0,
            effects: vec![SpecialMoveEffect::Damage {
                base_damage: 100.0,
                damage_type: DamageType::Physical,
                area_of_effect: Some(AreaOfEffect {
                    shape: AoeShape::Circle,
                    radius: 200.0,
                    width: 0.0,
                    height: 0.0,
                }),
            }],
            requirements: SpecialMoveRequirement::Level { min_level: 5 },
            animation: "whirlwind".to_string(),
            sound_effect: "whirlwind_sound".to_string(),
            visual_effect: "whirlwind_effect".to_string(),
        });

        // Ranger moves
        self.add_move_definition(SpecialMoveDefinition {
            id: "rapid_shot".to_string(),
            name: "Rapid Shot".to_string(),
            description: "Fire multiple arrows in quick succession".to_string(),
            cooldown_duration: 4.0,
            resource_cost: 25.0,
            resource_type: ResourceType::Stamina,
            execution_duration: 1.5,
            effects: vec![SpecialMoveEffect::Damage {
                base_damage: 80.0,
                damage_type: DamageType::Physical,
                area_of_effect: None,
            }],
            requirements: SpecialMoveRequirement::CharacterClass { class: "Ranger".to_string() },
            animation: "rapid_shot".to_string(),
            sound_effect: "rapid_shot_sound".to_string(),
            visual_effect: "rapid_shot_effect".to_string(),
        });

        self.add_move_definition(SpecialMoveDefinition {
            id: "explosive_arrow".to_string(),
            name: "Explosive Arrow".to_string(),
            description: "Arrow that explodes on impact, dealing area damage".to_string(),
            cooldown_duration: 6.0,
            resource_cost: 35.0,
            resource_type: ResourceType::Stamina,
            execution_duration: 1.0,
            effects: vec![SpecialMoveEffect::Damage {
                base_damage: 120.0,
                damage_type: DamageType::Fire,
                area_of_effect: Some(AreaOfEffect {
                    shape: AoeShape::Circle,
                    radius: 150.0,
                    width: 0.0,
                    height: 0.0,
                }),
            }],
            requirements: SpecialMoveRequirement::Level { min_level: 8 },
            animation: "explosive_arrow".to_string(),
            sound_effect: "explosive_arrow_sound".to_string(),
            visual_effect: "explosive_arrow_effect".to_string(),
        });

        // Mage moves
        self.add_move_definition(SpecialMoveDefinition {
            id: "fireball".to_string(),
            name: "Fireball".to_string(),
            description: "Launch a fireball that explodes on impact".to_string(),
            cooldown_duration: 2.0,
            resource_cost: 20.0,
            resource_type: ResourceType::Mana,
            execution_duration: 0.5,
            effects: vec![SpecialMoveEffect::Damage {
                base_damage: 100.0,
                damage_type: DamageType::Fire,
                area_of_effect: Some(AreaOfEffect {
                    shape: AoeShape::Circle,
                    radius: 100.0,
                    width: 0.0,
                    height: 0.0,
                }),
            }],
            requirements: SpecialMoveRequirement::CharacterClass { class: "Mage".to_string() },
            animation: "fireball".to_string(),
            sound_effect: "fireball_sound".to_string(),
            visual_effect: "fireball_effect".to_string(),
        });

        self.add_move_definition(SpecialMoveDefinition {
            id: "heal".to_string(),
            name: "Heal".to_string(),
            description: "Restore health to self or ally".to_string(),
            cooldown_duration: 3.0,
            resource_cost: 30.0,
            resource_type: ResourceType::Mana,
            execution_duration: 1.0,
            effects: vec![SpecialMoveEffect::Heal {
                amount: 150.0,
                target: HealTarget::SelfTarget,
            }],
            requirements: SpecialMoveRequirement::CharacterClass { class: "Mage".to_string() },
            animation: "heal".to_string(),
            sound_effect: "heal_sound".to_string(),
            visual_effect: "heal_effect".to_string(),
        });

        // Tank moves
        self.add_move_definition(SpecialMoveDefinition {
            id: "shield_bash".to_string(),
            name: "Shield Bash".to_string(),
            description: "Bash with shield, stunning enemies".to_string(),
            cooldown_duration: 4.0,
            resource_cost: 25.0,
            resource_type: ResourceType::Stamina,
            execution_duration: 1.0,
            effects: vec![
                SpecialMoveEffect::Damage {
                    base_damage: 80.0,
                    damage_type: DamageType::Physical,
                    area_of_effect: None,
                },
                SpecialMoveEffect::Debuff {
                    debuff_type: DebuffType::Stun,
                    duration: 2.0,
                    strength: 1.0,
                },
            ],
            requirements: SpecialMoveRequirement::CharacterClass { class: "Tank".to_string() },
            animation: "shield_bash".to_string(),
            sound_effect: "shield_bash_sound".to_string(),
            visual_effect: "shield_bash_effect".to_string(),
        });

        self.add_move_definition(SpecialMoveDefinition {
            id: "taunt".to_string(),
            name: "Taunt".to_string(),
            description: "Force enemies to attack you".to_string(),
            cooldown_duration: 8.0,
            resource_cost: 15.0,
            resource_type: ResourceType::Stamina,
            execution_duration: 0.5,
            effects: vec![SpecialMoveEffect::Debuff {
                debuff_type: DebuffType::Confusion,
                duration: 5.0,
                strength: 1.0,
            }],
            requirements: SpecialMoveRequirement::CharacterClass { class: "Tank".to_string() },
            animation: "taunt".to_string(),
            sound_effect: "taunt_sound".to_string(),
            visual_effect: "taunt_effect".to_string(),
        });

        // Assassin moves
        self.add_move_definition(SpecialMoveDefinition {
            id: "backstab".to_string(),
            name: "Backstab".to_string(),
            description: "High damage attack from behind".to_string(),
            cooldown_duration: 3.0,
            resource_cost: 20.0,
            resource_type: ResourceType::Stamina,
            execution_duration: 0.5,
            effects: vec![SpecialMoveEffect::Damage {
                base_damage: 200.0,
                damage_type: DamageType::Physical,
                area_of_effect: None,
            }],
            requirements: SpecialMoveRequirement::CharacterClass { class: "Assassin".to_string() },
            animation: "backstab".to_string(),
            sound_effect: "backstab_sound".to_string(),
            visual_effect: "backstab_effect".to_string(),
        });

        self.add_move_definition(SpecialMoveDefinition {
            id: "stealth".to_string(),
            name: "Stealth".to_string(),
            description: "Become invisible for a short time".to_string(),
            cooldown_duration: 10.0,
            resource_cost: 40.0,
            resource_type: ResourceType::Stamina,
            execution_duration: 0.5,
            effects: vec![SpecialMoveEffect::Buff {
                buff_type: BuffType::Invincibility,
                duration: 3.0,
                strength: 1.0,
            }],
            requirements: SpecialMoveRequirement::CharacterClass { class: "Assassin".to_string() },
            animation: "stealth".to_string(),
            sound_effect: "stealth_sound".to_string(),
            visual_effect: "stealth_effect".to_string(),
        });

        // Set up character move associations
        self.character_moves.insert("Fighter".to_string(), vec![
            "power_strike".to_string(),
            "whirlwind_attack".to_string(),
        ]);
        self.character_moves.insert("Ranger".to_string(), vec![
            "rapid_shot".to_string(),
            "explosive_arrow".to_string(),
        ]);
        self.character_moves.insert("Mage".to_string(), vec![
            "fireball".to_string(),
            "heal".to_string(),
        ]);
        self.character_moves.insert("Tank".to_string(), vec![
            "shield_bash".to_string(),
            "taunt".to_string(),
        ]);
        self.character_moves.insert("Assassin".to_string(), vec![
            "backstab".to_string(),
            "stealth".to_string(),
        ]);
    }

    pub fn add_move_definition(&mut self, definition: SpecialMoveDefinition) {
        self.move_definitions.insert(definition.id.clone(), definition);
    }

    pub fn get_move_definition(&self, move_id: &str) -> Option<&SpecialMoveDefinition> {
        self.move_definitions.get(move_id)
    }

    pub fn get_character_moves(&self, character_class: &str) -> Vec<&SpecialMoveDefinition> {
        self.character_moves.get(character_class)
            .map(|move_ids| {
                move_ids.iter()
                    .filter_map(|id| self.move_definitions.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn can_use_move(&self, move_id: &str, character_level: u32, character_class: &str, abilities: &[String]) -> bool {
        if let Some(definition) = self.get_move_definition(move_id) {
            match &definition.requirements {
                SpecialMoveRequirement::Level { min_level } => character_level >= *min_level,
                SpecialMoveRequirement::CharacterClass { class } => class == character_class,
                SpecialMoveRequirement::Ability { ability_name, min_level: _ } => {
                    abilities.contains(ability_name)
                },
                SpecialMoveRequirement::Item { item_name: _ } => true, // Simplified
                SpecialMoveRequirement::None => true,
            }
        } else {
            false
        }
    }
}

/// Special move system for processing special moves
pub struct SpecialMoveSystem {
    pub move_manager: SpecialMoveManager,
}

impl SpecialMoveSystem {
    pub fn new() -> Self {
        Self {
            move_manager: SpecialMoveManager::new(),
        }
    }
}

impl System for SpecialMoveSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // Update all special moves
        // Process special move execution
        // Handle cooldowns and resource costs
        // Apply special move effects
    }
}

/// Resource management system
pub struct ResourceManager {
    pub resources: HashMap<ResourceType, f32>,
    pub max_resources: HashMap<ResourceType, f32>,
    pub regeneration_rates: HashMap<ResourceType, f32>,
}

impl ResourceManager {
    pub fn new() -> Self {
        let mut manager = Self {
            resources: HashMap::new(),
            max_resources: HashMap::new(),
            regeneration_rates: HashMap::new(),
        };
        
        // Initialize default resources
        manager.resources.insert(ResourceType::Mana, 100.0);
        manager.resources.insert(ResourceType::Stamina, 100.0);
        manager.resources.insert(ResourceType::Rage, 0.0);
        manager.resources.insert(ResourceType::Energy, 100.0);
        manager.resources.insert(ResourceType::Focus, 100.0);

        manager.max_resources.insert(ResourceType::Mana, 100.0);
        manager.max_resources.insert(ResourceType::Stamina, 100.0);
        manager.max_resources.insert(ResourceType::Rage, 100.0);
        manager.max_resources.insert(ResourceType::Energy, 100.0);
        manager.max_resources.insert(ResourceType::Focus, 100.0);

        manager.regeneration_rates.insert(ResourceType::Mana, 5.0);
        manager.regeneration_rates.insert(ResourceType::Stamina, 10.0);
        manager.regeneration_rates.insert(ResourceType::Rage, 0.0);
        manager.regeneration_rates.insert(ResourceType::Energy, 8.0);
        manager.regeneration_rates.insert(ResourceType::Focus, 3.0);

        manager
    }

    pub fn consume_resource(&mut self, resource_type: ResourceType, amount: f32) -> bool {
        if let Some(current) = self.resources.get_mut(&resource_type) {
            if *current >= amount {
                *current -= amount;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn restore_resource(&mut self, resource_type: ResourceType, amount: f32) {
        if let Some(current) = self.resources.get_mut(&resource_type) {
            if let Some(max) = self.max_resources.get(&resource_type) {
                *current = (*current + amount).min(*max);
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        for (resource_type, regen_rate) in &self.regeneration_rates {
            if let Some(current) = self.resources.get_mut(resource_type) {
                if let Some(max) = self.max_resources.get(resource_type) {
                    *current = (*current + regen_rate * dt).min(*max);
                }
            }
        }
    }
}

impl Component for SpecialMove {}
