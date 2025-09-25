//! Playable Characters Module
//! 
//! This module provides a comprehensive character system including:
//! - Diverse character roster with unique abilities and stats
//! - Character customization (appearance, stats, abilities)
//! - Character progression and leveling
//! - Character selection and management

use crate::engine::ecs::{Component, Entity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use glam::Vec2;

pub mod character_roster;
pub mod character_customization;
pub mod character_progression;
pub mod character_selection;

pub use character_roster::*;
pub use character_customization::*;
pub use character_progression::*;
pub use character_selection::*;

/// Character component for ECS
#[derive(Clone, Debug)]
pub struct Character {
    /// Character ID
    pub id: String,
    /// Character name
    pub name: String,
    /// Character class
    pub class: CharacterClass,
    /// Character level
    pub level: u32,
    /// Character experience
    pub experience: u32,
    /// Character stats
    pub stats: CharacterStats,
    /// Character appearance
    pub appearance: CharacterAppearance,
    /// Character abilities
    pub abilities: Vec<String>,
    /// Character equipment
    pub equipment: CharacterEquipment,
    /// Character status
    pub status: CharacterStatus,
    /// Character preferences
    pub preferences: CharacterPreferences,
}

/// Character class enumeration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CharacterClass {
    Fighter,
    Ranger,
    Mage,
    Tank,
    Assassin,
    Paladin,
    Rogue,
    Cleric,
    Barbarian,
    Monk,
}

/// Character stats
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterStats {
    /// Strength - affects physical damage and carrying capacity
    pub strength: u32,
    /// Agility - affects speed, dodge chance, and critical hit chance
    pub agility: u32,
    /// Intelligence - affects mana, spell damage, and experience gain
    pub intelligence: u32,
    /// Vitality - affects health and stamina
    pub vitality: u32,
    /// Wisdom - affects mana regeneration and status resistance
    pub wisdom: u32,
    /// Charisma - affects trading prices and social interactions
    pub charisma: u32,
    /// Luck - affects critical hit chance and rare item drops
    pub luck: u32,
}

/// Character appearance customization
#[derive(Clone, Debug)]
pub struct CharacterAppearance {
    /// Character model/avatar ID
    pub model_id: String,
    /// Hair style
    pub hair_style: String,
    /// Hair color
    pub hair_color: [f32; 3],
    /// Eye color
    pub eye_color: [f32; 3],
    /// Skin tone
    pub skin_tone: [f32; 3],
    /// Body type
    pub body_type: BodyType,
    /// Height modifier (-1.0 to 1.0)
    pub height_modifier: f32,
    /// Weight modifier (-1.0 to 1.0)
    pub weight_modifier: f32,
    /// Custom markings/tattoos
    pub markings: Vec<Marking>,
    /// Equipment appearance overrides
    pub equipment_overrides: HashMap<String, String>,
}

/// Body type enumeration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BodyType {
    Slender,
    Average,
    Muscular,
    Heavy,
    Athletic,
}

/// Character marking/tattoo
#[derive(Clone, Debug)]
pub struct Marking {
    /// Marking ID
    pub id: String,
    /// Marking type
    pub marking_type: MarkingType,
    /// Position on body
    pub position: Vec2,
    /// Scale
    pub scale: f32,
    /// Color
    pub color: [f32; 4],
    /// Rotation
    pub rotation: f32,
}

/// Marking type enumeration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum MarkingType {
    Tattoo,
    Scar,
    Birthmark,
    WarPaint,
    Decoration,
}

/// Character equipment
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterEquipment {
    /// Head equipment
    pub head: Option<String>,
    /// Chest equipment
    pub chest: Option<String>,
    /// Legs equipment
    pub legs: Option<String>,
    /// Feet equipment
    pub feet: Option<String>,
    /// Hands equipment
    pub hands: Option<String>,
    /// Main hand weapon
    pub main_hand: Option<String>,
    /// Off hand weapon/shield
    pub off_hand: Option<String>,
    /// Accessories
    pub accessories: Vec<String>,
}

/// Character status
#[derive(Clone, Debug)]
pub struct CharacterStatus {
    /// Current health
    pub health: f32,
    /// Maximum health
    pub max_health: f32,
    /// Current mana
    pub mana: f32,
    /// Maximum mana
    pub max_mana: f32,
    /// Current stamina
    pub stamina: f32,
    /// Maximum stamina
    pub max_stamina: f32,
    /// Current position
    pub position: Vec2,
    /// Current state
    pub state: CharacterState,
    /// Active status effects
    pub status_effects: Vec<StatusEffect>,
    /// Is alive
    pub is_alive: bool,
    /// Is in combat
    pub in_combat: bool,
    /// Last damage taken
    pub last_damage: f32,
    /// Last damage source
    pub last_damage_source: Option<String>,
}

/// Character state enumeration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CharacterState {
    Idle,
    Moving,
    Attacking,
    Defending,
    Casting,
    Stunned,
    Dead,
    Resting,
    Interacting,
}

/// Status effect
#[derive(Clone, Debug)]
pub struct StatusEffect {
    /// Effect ID
    pub id: String,
    /// Effect type
    pub effect_type: String,
    /// Duration remaining
    pub duration: f32,
    /// Intensity/strength
    pub intensity: f32,
    /// Source
    pub source: String,
}

/// Character preferences
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterPreferences {
    /// Preferred difficulty
    pub difficulty: DifficultyPreference,
    /// Preferred play style
    pub play_style: PlayStyle,
    /// Auto-save frequency
    pub auto_save_frequency: u32,
    /// UI preferences
    pub ui_preferences: UIPreferences,
    /// Control preferences
    pub control_preferences: ControlPreferences,
    /// Audio preferences
    pub audio_preferences: AudioPreferences,
}

/// Difficulty preference
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DifficultyPreference {
    Easy,
    Normal,
    Hard,
    Nightmare,
    Custom,
}

/// Play style preference
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PlayStyle {
    Aggressive,
    Defensive,
    Balanced,
    Stealth,
    Support,
    Exploration,
}

/// UI preferences
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UIPreferences {
    /// Scale factor
    pub scale: f32,
    /// Show damage numbers
    pub show_damage_numbers: bool,
    /// Show health bars
    pub show_health_bars: bool,
    /// Show minimap
    pub show_minimap: bool,
    /// Show quest markers
    pub show_quest_markers: bool,
    /// Color scheme
    pub color_scheme: String,
}

/// Control preferences
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ControlPreferences {
    /// Mouse sensitivity
    pub mouse_sensitivity: f32,
    /// Invert mouse Y
    pub invert_mouse_y: bool,
    /// Key bindings
    pub key_bindings: HashMap<String, String>,
    /// Controller settings
    pub controller_settings: ControllerSettings,
}

/// Controller settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ControllerSettings {
    /// Controller sensitivity
    pub sensitivity: f32,
    /// Vibration enabled
    pub vibration: bool,
    /// Dead zone
    pub dead_zone: f32,
    /// Button mappings
    pub button_mappings: HashMap<String, String>,
}

/// Audio preferences
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioPreferences {
    /// Master volume
    pub master_volume: f32,
    /// Music volume
    pub music_volume: f32,
    /// SFX volume
    pub sfx_volume: f32,
    /// Voice volume
    pub voice_volume: f32,
    /// Ambient volume
    pub ambient_volume: f32,
    /// Mute all
    pub mute_all: bool,
}

impl Character {
    /// Create a new character
    pub fn new(
        id: String,
        name: String,
        class: CharacterClass,
        level: u32,
        stats: CharacterStats,
        appearance: CharacterAppearance,
    ) -> Self {
        let max_health = Self::calculate_max_health(&stats, level);
        let max_mana = Self::calculate_max_mana(&stats, level);
        let max_stamina = Self::calculate_max_stamina(&stats, level);

        Self {
            id,
            name,
            class,
            level,
            experience: 0,
            stats,
            appearance,
            abilities: Vec::new(),
            equipment: CharacterEquipment::default(),
            status: CharacterStatus {
                health: max_health,
                max_health,
                mana: max_mana,
                max_mana,
                stamina: max_stamina,
                max_stamina,
                position: Vec2::ZERO,
                state: CharacterState::Idle,
                status_effects: Vec::new(),
                is_alive: true,
                in_combat: false,
                last_damage: 0.0,
                last_damage_source: None,
            },
            preferences: CharacterPreferences::default(),
        }
    }

    /// Calculate maximum health based on stats and level
    fn calculate_max_health(stats: &CharacterStats, level: u32) -> f32 {
        let base_health = 100.0;
        let vitality_bonus = stats.vitality as f32 * 10.0;
        let level_bonus = level as f32 * 5.0;
        base_health + vitality_bonus + level_bonus
    }

    /// Calculate maximum mana based on stats and level
    fn calculate_max_mana(stats: &CharacterStats, level: u32) -> f32 {
        let base_mana = 50.0;
        let intelligence_bonus = stats.intelligence as f32 * 8.0;
        let wisdom_bonus = stats.wisdom as f32 * 5.0;
        let level_bonus = level as f32 * 3.0;
        base_mana + intelligence_bonus + wisdom_bonus + level_bonus
    }

    /// Calculate maximum stamina based on stats and level
    fn calculate_max_stamina(stats: &CharacterStats, level: u32) -> f32 {
        let base_stamina = 100.0;
        let vitality_bonus = stats.vitality as f32 * 5.0;
        let agility_bonus = stats.agility as f32 * 3.0;
        let level_bonus = level as f32 * 2.0;
        base_stamina + vitality_bonus + agility_bonus + level_bonus
    }

    /// Get character's total damage
    pub fn get_total_damage(&self) -> f32 {
        let base_damage = self.stats.strength as f32 * 2.0;
        let level_bonus = self.level as f32 * 1.5;
        base_damage + level_bonus
    }

    /// Get character's total defense
    pub fn get_total_defense(&self) -> f32 {
        let base_defense = self.stats.vitality as f32 * 1.5;
        let level_bonus = self.level as f32 * 1.0;
        base_defense + level_bonus
    }

    /// Get character's movement speed
    pub fn get_movement_speed(&self) -> f32 {
        let base_speed = 100.0;
        let agility_bonus = self.stats.agility as f32 * 2.0;
        let level_bonus = self.level as f32 * 0.5;
        base_speed + agility_bonus + level_bonus
    }

    /// Check if character can level up
    pub fn can_level_up(&self) -> bool {
        let required_exp = self.get_required_experience_for_level(self.level + 1);
        self.experience >= required_exp
    }

    /// Get required experience for a specific level
    pub fn get_required_experience_for_level(&self, level: u32) -> u32 {
        if level <= 1 {
            return 0;
        }
        let base_exp = 100;
        let level_multiplier: f32 = 1.2;
        let mut total_exp = 0;
        for i in 2..=level {
            total_exp += (base_exp as f32 * level_multiplier.powi(i as i32 - 2)) as u32;
        }
        total_exp
    }

    /// Level up the character
    pub fn level_up(&mut self) -> bool {
        if !self.can_level_up() {
            return false;
        }

        self.level += 1;
        
        // Increase stats based on class
        self.increase_stats_for_class();
        
        // Update max values
        self.status.max_health = Self::calculate_max_health(&self.stats, self.level);
        self.status.max_mana = Self::calculate_max_mana(&self.stats, self.level);
        self.status.max_stamina = Self::calculate_max_stamina(&self.stats, self.level);
        
        // Restore health and mana
        self.status.health = self.status.max_health;
        self.status.mana = self.status.max_mana;
        self.status.stamina = self.status.max_stamina;

        true
    }

    /// Increase stats based on character class
    fn increase_stats_for_class(&mut self) {
        match self.class {
            CharacterClass::Fighter => {
                self.stats.strength += 3;
                self.stats.vitality += 2;
                self.stats.agility += 1;
            }
            CharacterClass::Ranger => {
                self.stats.agility += 3;
                self.stats.intelligence += 2;
                self.stats.strength += 1;
            }
            CharacterClass::Mage => {
                self.stats.intelligence += 3;
                self.stats.wisdom += 2;
                self.stats.vitality += 1;
            }
            CharacterClass::Tank => {
                self.stats.vitality += 3;
                self.stats.strength += 2;
                self.stats.wisdom += 1;
            }
            CharacterClass::Assassin => {
                self.stats.agility += 3;
                self.stats.strength += 2;
                self.stats.luck += 1;
            }
            CharacterClass::Paladin => {
                self.stats.strength += 2;
                self.stats.vitality += 2;
                self.stats.wisdom += 2;
            }
            CharacterClass::Rogue => {
                self.stats.agility += 2;
                self.stats.luck += 2;
                self.stats.intelligence += 2;
            }
            CharacterClass::Cleric => {
                self.stats.wisdom += 3;
                self.stats.vitality += 2;
                self.stats.intelligence += 1;
            }
            CharacterClass::Barbarian => {
                self.stats.strength += 3;
                self.stats.vitality += 2;
                self.stats.agility += 1;
            }
            CharacterClass::Monk => {
                self.stats.agility += 2;
                self.stats.wisdom += 2;
                self.stats.vitality += 2;
            }
        }
    }

    /// Add experience to the character
    pub fn add_experience(&mut self, amount: u32) -> bool {
        self.experience += amount;
        self.can_level_up()
    }

    /// Take damage
    pub fn take_damage(&mut self, damage: f32, source: Option<String>) {
        if !self.status.is_alive {
            return;
        }

        let actual_damage = damage.max(0.0);
        self.status.health = (self.status.health - actual_damage).max(0.0);
        self.status.last_damage = actual_damage;
        self.status.last_damage_source = source;

        if self.status.health <= 0.0 {
            self.status.is_alive = false;
            self.status.state = CharacterState::Dead;
        }
    }

    /// Heal the character
    pub fn heal(&mut self, amount: f32) {
        if !self.status.is_alive {
            return;
        }

        self.status.health = (self.status.health + amount).min(self.status.max_health);
    }

    /// Restore mana
    pub fn restore_mana(&mut self, amount: f32) {
        self.status.mana = (self.status.mana + amount).min(self.status.max_mana);
    }

    /// Restore stamina
    pub fn restore_stamina(&mut self, amount: f32) {
        self.status.stamina = (self.status.stamina + amount).min(self.status.max_stamina);
    }

    /// Consume mana
    pub fn consume_mana(&mut self, amount: f32) -> bool {
        if self.status.mana >= amount {
            self.status.mana -= amount;
            true
        } else {
            false
        }
    }

    /// Consume stamina
    pub fn consume_stamina(&mut self, amount: f32) -> bool {
        if self.status.stamina >= amount {
            self.status.stamina -= amount;
            true
        } else {
            false
        }
    }

    /// Add status effect
    pub fn add_status_effect(&mut self, effect: StatusEffect) {
        // Check if effect already exists and update or add
        if let Some(existing) = self.status.status_effects.iter_mut().find(|e| e.id == effect.id) {
            existing.duration = effect.duration;
            existing.intensity = effect.intensity;
        } else {
            self.status.status_effects.push(effect);
        }
    }

    /// Remove status effect
    pub fn remove_status_effect(&mut self, effect_id: &str) -> bool {
        if let Some(pos) = self.status.status_effects.iter().position(|e| e.id == effect_id) {
            self.status.status_effects.remove(pos);
            true
        } else {
            false
        }
    }

    /// Update status effects
    pub fn update_status_effects(&mut self, delta_time: f32) {
        self.status.status_effects.retain_mut(|effect| {
            effect.duration -= delta_time;
            effect.duration > 0.0
        });
    }

    /// Get character's current state description
    pub fn get_state_description(&self) -> String {
        match self.status.state {
            CharacterState::Idle => "Standing idle".to_string(),
            CharacterState::Moving => "Moving around".to_string(),
            CharacterState::Attacking => "Attacking".to_string(),
            CharacterState::Defending => "Defending".to_string(),
            CharacterState::Casting => "Casting a spell".to_string(),
            CharacterState::Stunned => "Stunned".to_string(),
            CharacterState::Dead => "Dead".to_string(),
            CharacterState::Resting => "Resting".to_string(),
            CharacterState::Interacting => "Interacting".to_string(),
        }
    }

    /// Get character's health percentage
    pub fn get_health_percentage(&self) -> f32 {
        if self.status.max_health > 0.0 {
            self.status.health / self.status.max_health
        } else {
            0.0
        }
    }

    /// Get character's mana percentage
    pub fn get_mana_percentage(&self) -> f32 {
        if self.status.max_mana > 0.0 {
            self.status.mana / self.status.max_mana
        } else {
            0.0
        }
    }

    /// Get character's stamina percentage
    pub fn get_stamina_percentage(&self) -> f32 {
        if self.status.max_stamina > 0.0 {
            self.status.stamina / self.status.max_stamina
        } else {
            0.0
        }
    }
}

impl Default for CharacterEquipment {
    fn default() -> Self {
        Self {
            head: None,
            chest: None,
            legs: None,
            feet: None,
            hands: None,
            main_hand: None,
            off_hand: None,
            accessories: Vec::new(),
        }
    }
}

impl Default for CharacterPreferences {
    fn default() -> Self {
        Self {
            difficulty: DifficultyPreference::Normal,
            play_style: PlayStyle::Balanced,
            auto_save_frequency: 300, // 5 minutes
            ui_preferences: UIPreferences::default(),
            control_preferences: ControlPreferences::default(),
            audio_preferences: AudioPreferences::default(),
        }
    }
}

impl Default for UIPreferences {
    fn default() -> Self {
        Self {
            scale: 1.0,
            show_damage_numbers: true,
            show_health_bars: true,
            show_minimap: true,
            show_quest_markers: true,
            color_scheme: "default".to_string(),
        }
    }
}

impl Default for ControlPreferences {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 1.0,
            invert_mouse_y: false,
            key_bindings: HashMap::new(),
            controller_settings: ControllerSettings::default(),
        }
    }
}

impl Default for ControllerSettings {
    fn default() -> Self {
        Self {
            sensitivity: 1.0,
            vibration: true,
            dead_zone: 0.1,
            button_mappings: HashMap::new(),
        }
    }
}

impl Default for AudioPreferences {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.8,
            sfx_volume: 1.0,
            voice_volume: 1.0,
            ambient_volume: 0.6,
            mute_all: false,
        }
    }
}