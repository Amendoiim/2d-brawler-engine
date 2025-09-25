//! Character Customization Module
//! 
//! This module provides character customization functionality including:
//! - Appearance customization (hair, eyes, skin, body type, markings)
//! - Equipment customization and appearance overrides
//! - Stat customization and allocation
//! - Ability customization and selection

use super::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Character customization manager
#[derive(Clone, Debug)]
pub struct CharacterCustomization {
    /// Available appearance options
    pub appearance_options: AppearanceOptions,
    /// Available equipment options
    pub equipment_options: EquipmentOptions,
    /// Available stat allocation points
    pub stat_allocation_points: u32,
    /// Available ability points
    pub ability_points: u32,
    /// Customization presets
    pub presets: HashMap<String, CustomizationPreset>,
}

/// Appearance customization options
#[derive(Clone, Debug)]
pub struct AppearanceOptions {
    /// Available hair styles
    pub hair_styles: Vec<HairStyle>,
    /// Available hair colors
    pub hair_colors: Vec<[f32; 3]>,
    /// Available eye colors
    pub eye_colors: Vec<[f32; 3]>,
    /// Available skin tones
    pub skin_tones: Vec<[f32; 3]>,
    /// Available body types
    pub body_types: Vec<BodyType>,
    /// Available markings
    pub markings: Vec<MarkingTemplate>,
    /// Available model variants
    pub model_variants: Vec<ModelVariant>,
}

/// Hair style option
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HairStyle {
    /// Style ID
    pub id: String,
    /// Style name
    pub name: String,
    /// Style description
    pub description: String,
    /// Style category
    pub category: HairCategory,
    /// Gender compatibility
    pub gender_compatibility: GenderCompatibility,
    /// Unlock requirement
    pub unlock_requirement: Option<UnlockRequirement>,
}

/// Hair style category
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum HairCategory {
    Short,
    Medium,
    Long,
    Bald,
    Wild,
    Military,
    Elegant,
    Casual,
}

/// Gender compatibility
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GenderCompatibility {
    Male,
    Female,
    Both,
}

/// Marking template
#[derive(Clone, Debug)]
pub struct MarkingTemplate {
    /// Template ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Marking type
    pub marking_type: MarkingType,
    /// Available positions
    pub available_positions: Vec<MarkingPosition>,
    /// Default scale
    pub default_scale: f32,
    /// Default color
    pub default_color: [f32; 4],
    /// Unlock requirement
    pub unlock_requirement: Option<UnlockRequirement>,
}

/// Marking position
#[derive(Clone, Debug)]
pub struct MarkingPosition {
    /// Position name
    pub name: String,
    /// Position coordinates
    pub position: Vec2,
    /// Rotation range
    pub rotation_range: (f32, f32),
    /// Scale range
    pub scale_range: (f32, f32),
}

/// Model variant
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelVariant {
    /// Variant ID
    pub id: String,
    /// Variant name
    pub name: String,
    /// Model path
    pub model_path: String,
    /// Gender compatibility
    pub gender_compatibility: GenderCompatibility,
    /// Class compatibility
    pub class_compatibility: Vec<CharacterClass>,
    /// Unlock requirement
    pub unlock_requirement: Option<UnlockRequirement>,
}

/// Equipment customization options
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EquipmentOptions {
    /// Available head equipment
    pub head_equipment: Vec<EquipmentItem>,
    /// Available chest equipment
    pub chest_equipment: Vec<EquipmentItem>,
    /// Available legs equipment
    pub legs_equipment: Vec<EquipmentItem>,
    /// Available feet equipment
    pub feet_equipment: Vec<EquipmentItem>,
    /// Available hands equipment
    pub hands_equipment: Vec<EquipmentItem>,
    /// Available main hand weapons
    pub main_hand_weapons: Vec<EquipmentItem>,
    /// Available off hand weapons
    pub off_hand_weapons: Vec<EquipmentItem>,
    /// Available accessories
    pub accessories: Vec<EquipmentItem>,
}

/// Equipment item
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EquipmentItem {
    /// Item ID
    pub id: String,
    /// Item name
    pub name: String,
    /// Item description
    pub description: String,
    /// Item type
    pub item_type: EquipmentType,
    /// Item rarity
    pub rarity: ItemRarity,
    /// Stat bonuses
    pub stat_bonuses: HashMap<String, f32>,
    /// Appearance overrides
    pub appearance_overrides: HashMap<String, String>,
    /// Unlock requirement
    pub unlock_requirement: Option<UnlockRequirement>,
}

/// Equipment type
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EquipmentType {
    Head,
    Chest,
    Legs,
    Feet,
    Hands,
    MainHand,
    OffHand,
    Accessory,
}

/// Item rarity
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

/// Customization preset
#[derive(Clone, Debug)]
pub struct CustomizationPreset {
    /// Preset ID
    pub id: String,
    /// Preset name
    pub name: String,
    /// Preset description
    pub description: String,
    /// Appearance settings
    pub appearance: CharacterAppearance,
    /// Equipment settings
    pub equipment: CharacterEquipment,
    /// Stat allocation
    pub stat_allocation: HashMap<String, u32>,
    /// Ability selection
    pub ability_selection: Vec<String>,
    /// Unlock requirement
    pub unlock_requirement: Option<UnlockRequirement>,
}

/// Character customization manager implementation
impl CharacterCustomization {
    /// Create a new character customization manager
    pub fn new() -> Self {
        let mut customization = Self {
            appearance_options: AppearanceOptions::new(),
            equipment_options: EquipmentOptions::new(),
            stat_allocation_points: 0,
            ability_points: 0,
            presets: HashMap::new(),
        };
        
        customization.initialize_presets();
        customization
    }

    /// Initialize customization presets
    fn initialize_presets(&mut self) {
        // Default Fighter Preset
        self.presets.insert("default_fighter".to_string(), CustomizationPreset {
            id: "default_fighter".to_string(),
            name: "Classic Warrior".to_string(),
            description: "A traditional fighter appearance with standard equipment.".to_string(),
            appearance: CharacterAppearance {
                model_id: "fighter_male_01".to_string(),
                hair_style: "short_military".to_string(),
                hair_color: [0.3, 0.2, 0.1],
                eye_color: [0.2, 0.4, 0.8],
                skin_tone: [0.8, 0.6, 0.4],
                body_type: BodyType::Muscular,
                height_modifier: 0.1,
                weight_modifier: 0.2,
                markings: Vec::new(),
                equipment_overrides: HashMap::new(),
            },
            equipment: CharacterEquipment {
                main_hand: Some("iron_sword".to_string()),
                chest: Some("leather_armor".to_string()),
                ..Default::default()
            },
            stat_allocation: HashMap::new(),
            ability_selection: vec!["power_strike".to_string(), "heavy_armor".to_string()],
            unlock_requirement: None,
        });

        // Default Mage Preset
        self.presets.insert("default_mage".to_string(), CustomizationPreset {
            id: "default_mage".to_string(),
            name: "Arcane Scholar".to_string(),
            description: "A traditional mage appearance with mystical equipment.".to_string(),
            appearance: CharacterAppearance {
                model_id: "mage_male_01".to_string(),
                hair_style: "long_wild".to_string(),
                hair_color: [0.8, 0.8, 0.9],
                eye_color: [0.6, 0.2, 0.8],
                skin_tone: [0.9, 0.8, 0.7],
                body_type: BodyType::Slender,
                height_modifier: 0.0,
                weight_modifier: -0.2,
                markings: Vec::new(),
                equipment_overrides: HashMap::new(),
            },
            equipment: CharacterEquipment {
                main_hand: Some("oak_staff".to_string()),
                chest: Some("robes".to_string()),
                ..Default::default()
            },
            stat_allocation: HashMap::new(),
            ability_selection: vec!["fireball".to_string(), "mana_shield".to_string()],
            unlock_requirement: None,
        });

        // Default Ranger Preset
        self.presets.insert("default_ranger".to_string(), CustomizationPreset {
            id: "default_ranger".to_string(),
            name: "Forest Guardian".to_string(),
            description: "A traditional ranger appearance with nature-themed equipment.".to_string(),
            appearance: CharacterAppearance {
                model_id: "ranger_female_01".to_string(),
                hair_style: "long_braided".to_string(),
                hair_color: [0.1, 0.4, 0.2],
                eye_color: [0.1, 0.6, 0.3],
                skin_tone: [0.7, 0.5, 0.3],
                body_type: BodyType::Athletic,
                height_modifier: 0.0,
                weight_modifier: -0.1,
                markings: Vec::new(),
                equipment_overrides: HashMap::new(),
            },
            equipment: CharacterEquipment {
                main_hand: Some("hunting_bow".to_string()),
                chest: Some("leather_tunic".to_string()),
                ..Default::default()
            },
            stat_allocation: HashMap::new(),
            ability_selection: vec!["precise_shot".to_string(), "nature_awareness".to_string()],
            unlock_requirement: None,
        });
    }

    /// Customize character appearance
    pub fn customize_appearance(&self, character: &mut Character, appearance: CharacterAppearance) {
        character.appearance = appearance;
    }

    /// Customize character equipment
    pub fn customize_equipment(&self, character: &mut Character, equipment: CharacterEquipment) {
        // Convert legacy CharacterEquipment to new Equipment type
        // This is a placeholder - in a real implementation, you'd convert the equipment
        character.equipment = crate::game::items::Equipment::new();
    }

    /// Allocate stat points
    pub fn allocate_stat_points(&self, character: &mut Character, stat_allocations: HashMap<String, u32>) -> Result<(), String> {
        let total_points_used: u32 = stat_allocations.values().sum();
        
        if total_points_used > self.stat_allocation_points {
            return Err(format!("Not enough stat allocation points. Available: {}, Required: {}", 
                self.stat_allocation_points, total_points_used));
        }

        for (stat_name, points) in stat_allocations {
            match stat_name.as_str() {
                "strength" => character.stats.strength += points,
                "agility" => character.stats.agility += points,
                "intelligence" => character.stats.intelligence += points,
                "vitality" => character.stats.vitality += points,
                "wisdom" => character.stats.wisdom += points,
                "charisma" => character.stats.charisma += points,
                "luck" => character.stats.luck += points,
                _ => return Err(format!("Unknown stat: {}", stat_name)),
            }
        }

        Ok(())
    }

    /// Select abilities
    pub fn select_abilities(&self, character: &mut Character, abilities: Vec<String>) -> Result<(), String> {
        if abilities.len() > self.ability_points as usize {
            return Err(format!("Too many abilities selected. Available: {}, Selected: {}", 
                self.ability_points, abilities.len()));
        }

        character.abilities = abilities;
        Ok(())
    }

    /// Apply preset to character
    pub fn apply_preset(&self, character: &mut Character, preset_id: &str) -> Result<(), String> {
        let preset = self.presets.get(preset_id)
            .ok_or_else(|| format!("Preset '{}' not found", preset_id))?;

        character.appearance = preset.appearance.clone();
        // Convert legacy CharacterEquipment to new Equipment type
        character.equipment = crate::game::items::Equipment::new();
        character.abilities = preset.ability_selection.clone();

        // Apply stat allocations
        for (stat_name, points) in &preset.stat_allocation {
            match stat_name.as_str() {
                "strength" => character.stats.strength += points,
                "agility" => character.stats.agility += points,
                "intelligence" => character.stats.intelligence += points,
                "vitality" => character.stats.vitality += points,
                "wisdom" => character.stats.wisdom += points,
                "charisma" => character.stats.charisma += points,
                "luck" => character.stats.luck += points,
                _ => continue,
            }
        }

        Ok(())
    }

    /// Get available presets for character class
    pub fn get_available_presets(&self, character_class: &CharacterClass) -> Vec<&CustomizationPreset> {
        self.presets.values()
            .filter(|preset| {
                // Check if preset is compatible with character class
                // This would need to be implemented based on your requirements
                true
            })
            .collect()
    }

    /// Get appearance options
    pub fn get_appearance_options(&self) -> &AppearanceOptions {
        &self.appearance_options
    }

    /// Get equipment options
    pub fn get_equipment_options(&self) -> &EquipmentOptions {
        &self.equipment_options
    }

    /// Get available hair styles for gender
    pub fn get_hair_styles_for_gender(&self, gender: GenderCompatibility) -> Vec<&HairStyle> {
        self.appearance_options.hair_styles.iter()
            .filter(|style| style.gender_compatibility == gender || style.gender_compatibility == GenderCompatibility::Both)
            .collect()
    }

    /// Get available equipment by type
    pub fn get_equipment_by_type(&self, equipment_type: &EquipmentType) -> Vec<&EquipmentItem> {
        match equipment_type {
            EquipmentType::Head => self.equipment_options.head_equipment.iter().collect(),
            EquipmentType::Chest => self.equipment_options.chest_equipment.iter().collect(),
            EquipmentType::Legs => self.equipment_options.legs_equipment.iter().collect(),
            EquipmentType::Feet => self.equipment_options.feet_equipment.iter().collect(),
            EquipmentType::Hands => self.equipment_options.hands_equipment.iter().collect(),
            EquipmentType::MainHand => self.equipment_options.main_hand_weapons.iter().collect(),
            EquipmentType::OffHand => self.equipment_options.off_hand_weapons.iter().collect(),
            EquipmentType::Accessory => self.equipment_options.accessories.iter().collect(),
        }
    }

    /// Get available markings
    pub fn get_available_markings(&self) -> Vec<&MarkingTemplate> {
        self.appearance_options.markings.iter().collect()
    }

    /// Get available model variants for class
    pub fn get_model_variants_for_class(&self, character_class: &CharacterClass) -> Vec<&ModelVariant> {
        self.appearance_options.model_variants.iter()
            .filter(|variant| variant.class_compatibility.contains(character_class))
            .collect()
    }

    /// Add stat allocation points
    pub fn add_stat_allocation_points(&mut self, points: u32) {
        self.stat_allocation_points += points;
    }

    /// Add ability points
    pub fn add_ability_points(&mut self, points: u32) {
        self.ability_points += points;
    }

    /// Get remaining stat allocation points
    pub fn get_remaining_stat_points(&self) -> u32 {
        self.stat_allocation_points
    }

    /// Get remaining ability points
    pub fn get_remaining_ability_points(&self) -> u32 {
        self.ability_points
    }

    /// Reset character customization
    pub fn reset_character_customization(&self, character: &mut Character) {
        // Reset to default appearance based on class
        character.appearance = self.get_default_appearance_for_class(&character.class);
        character.equipment = crate::game::items::Equipment::new();
        character.abilities.clear();
    }

    /// Get default appearance for character class
    fn get_default_appearance_for_class(&self, class: &CharacterClass) -> CharacterAppearance {
        match class {
            CharacterClass::Fighter => CharacterAppearance {
                model_id: "fighter_male_01".to_string(),
                hair_style: "short_military".to_string(),
                hair_color: [0.3, 0.2, 0.1],
                eye_color: [0.2, 0.4, 0.8],
                skin_tone: [0.8, 0.6, 0.4],
                body_type: BodyType::Muscular,
                height_modifier: 0.1,
                weight_modifier: 0.2,
                markings: Vec::new(),
                equipment_overrides: HashMap::new(),
            },
            CharacterClass::Mage => CharacterAppearance {
                model_id: "mage_male_01".to_string(),
                hair_style: "long_wild".to_string(),
                hair_color: [0.8, 0.8, 0.9],
                eye_color: [0.6, 0.2, 0.8],
                skin_tone: [0.9, 0.8, 0.7],
                body_type: BodyType::Slender,
                height_modifier: 0.0,
                weight_modifier: -0.2,
                markings: Vec::new(),
                equipment_overrides: HashMap::new(),
            },
            _ => CharacterAppearance {
                model_id: "default_male_01".to_string(),
                hair_style: "short_casual".to_string(),
                hair_color: [0.3, 0.2, 0.1],
                eye_color: [0.2, 0.4, 0.8],
                skin_tone: [0.8, 0.6, 0.4],
                body_type: BodyType::Average,
                height_modifier: 0.0,
                weight_modifier: 0.0,
                markings: Vec::new(),
                equipment_overrides: HashMap::new(),
            },
        }
    }
}

impl AppearanceOptions {
    /// Create new appearance options
    pub fn new() -> Self {
        Self {
            hair_styles: Vec::new(),
            hair_colors: Vec::new(),
            eye_colors: Vec::new(),
            skin_tones: Vec::new(),
            body_types: vec![
                BodyType::Slender,
                BodyType::Average,
                BodyType::Muscular,
                BodyType::Heavy,
                BodyType::Athletic,
            ],
            markings: Vec::new(),
            model_variants: Vec::new(),
        }
    }
}

impl EquipmentOptions {
    /// Create new equipment options
    pub fn new() -> Self {
        Self {
            head_equipment: Vec::new(),
            chest_equipment: Vec::new(),
            legs_equipment: Vec::new(),
            feet_equipment: Vec::new(),
            hands_equipment: Vec::new(),
            main_hand_weapons: Vec::new(),
            off_hand_weapons: Vec::new(),
            accessories: Vec::new(),
        }
    }
}

impl Default for CharacterCustomization {
    fn default() -> Self {
        Self::new()
    }
}
