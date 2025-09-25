//! String ID system for localization
//! 
//! This module provides a structured way to organize and manage string IDs
//! for the localization system.

use super::{StringId, Language};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Categories for organizing string IDs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StringCategory {
    /// UI elements and menus
    UI,
    /// Game mechanics and gameplay
    Gameplay,
    /// Character names and descriptions
    Characters,
    /// Item names and descriptions
    Items,
    /// Enemy names and descriptions
    Enemies,
    /// Level names and descriptions
    Levels,
    /// Combat messages and effects
    Combat,
    /// Tutorial and help text
    Tutorial,
    /// Error messages
    Errors,
    /// Achievement and progression
    Achievements,
    /// Settings and options
    Settings,
    /// Audio and visual settings
    Audio,
    /// Network and multiplayer
    Network,
    /// Debug and development
    Debug,
}

impl StringCategory {
    /// Get the prefix for this category
    pub fn prefix(&self) -> &'static str {
        match self {
            StringCategory::UI => "ui",
            StringCategory::Gameplay => "gameplay",
            StringCategory::Characters => "characters",
            StringCategory::Items => "items",
            StringCategory::Enemies => "enemies",
            StringCategory::Levels => "levels",
            StringCategory::Combat => "combat",
            StringCategory::Tutorial => "tutorial",
            StringCategory::Errors => "errors",
            StringCategory::Achievements => "achievements",
            StringCategory::Settings => "settings",
            StringCategory::Audio => "audio",
            StringCategory::Network => "network",
            StringCategory::Debug => "debug",
        }
    }
}

/// String ID builder for creating structured string IDs
pub struct StringIdBuilder {
    category: StringCategory,
    subcategory: Option<String>,
    identifier: String,
}

impl StringIdBuilder {
    /// Create a new string ID builder
    pub fn new(category: StringCategory) -> Self {
        Self {
            category,
            subcategory: None,
            identifier: String::new(),
        }
    }

    /// Add a subcategory
    pub fn subcategory(mut self, subcategory: &str) -> Self {
        self.subcategory = Some(subcategory.to_string());
        self
    }

    /// Set the identifier
    pub fn identifier(mut self, identifier: &str) -> Self {
        self.identifier = identifier.to_string();
        self
    }

    /// Build the string ID
    pub fn build(self) -> StringId {
        let mut id = self.category.prefix().to_string();
        
        if let Some(subcategory) = self.subcategory {
            id.push('.');
            id.push_str(&subcategory);
        }
        
        if !self.identifier.is_empty() {
            id.push('.');
            id.push_str(&self.identifier);
        }
        
        StringId::new(id)
    }
}

/// Predefined string IDs for common game elements
pub struct CommonStringIds;

impl CommonStringIds {
    // UI Strings
    pub const MAIN_MENU: &str = "ui.main_menu";
    pub const PLAY: &str = "ui.play";
    pub const SETTINGS: &str = "ui.settings";
    pub const QUIT: &str = "ui.quit";
    pub const BACK: &str = "ui.back";
    pub const NEXT: &str = "ui.next";
    pub const CANCEL: &str = "ui.cancel";
    pub const CONFIRM: &str = "ui.confirm";
    pub const SAVE: &str = "ui.save";
    pub const LOAD: &str = "ui.load";
    pub const NEW_GAME: &str = "ui.new_game";
    pub const CONTINUE: &str = "ui.continue";
    pub const PAUSE: &str = "ui.pause";
    pub const RESUME: &str = "ui.resume";
    pub const RESTART: &str = "ui.restart";
    pub const EXIT: &str = "ui.exit";

    // Gameplay Strings
    pub const LEVEL: &str = "gameplay.level";
    pub const SCORE: &str = "gameplay.score";
    pub const TIME: &str = "gameplay.time";
    pub const HEALTH: &str = "gameplay.health";
    pub const MANA: &str = "gameplay.mana";
    pub const STAMINA: &str = "gameplay.stamina";
    pub const EXPERIENCE: &str = "gameplay.experience";
    pub const GOLD: &str = "gameplay.gold";
    pub const INVENTORY: &str = "gameplay.inventory";
    pub const EQUIPMENT: &str = "gameplay.equipment";
    pub const SKILLS: &str = "gameplay.skills";
    pub const ABILITIES: &str = "gameplay.abilities";

    // Character Strings
    pub const CHARACTER_SELECTION: &str = "characters.selection";
    pub const CHARACTER_CREATION: &str = "characters.creation";
    pub const CHARACTER_CUSTOMIZATION: &str = "characters.customization";
    pub const CHARACTER_STATS: &str = "characters.stats";
    pub const CHARACTER_LEVEL: &str = "characters.level";
    pub const CHARACTER_CLASS: &str = "characters.class";
    pub const CHARACTER_NAME: &str = "characters.name";
    pub const CHARACTER_DESCRIPTION: &str = "characters.description";

    // Item Strings
    pub const ITEM_NAME: &str = "items.name";
    pub const ITEM_DESCRIPTION: &str = "items.description";
    pub const ITEM_RARITY: &str = "items.rarity";
    pub const ITEM_TYPE: &str = "items.type";
    pub const ITEM_VALUE: &str = "items.value";
    pub const ITEM_WEIGHT: &str = "items.weight";
    pub const ITEM_EFFECTS: &str = "items.effects";
    pub const ITEM_REQUIREMENTS: &str = "items.requirements";

    // Combat Strings
    pub const ATTACK: &str = "combat.attack";
    pub const DEFEND: &str = "combat.defend";
    pub const DODGE: &str = "combat.dodge";
    pub const BLOCK: &str = "combat.block";
    pub const PARRY: &str = "combat.parry";
    pub const CRITICAL_HIT: &str = "combat.critical_hit";
    pub const MISS: &str = "combat.miss";
    pub const DAMAGE: &str = "combat.damage";
    pub const HEAL: &str = "combat.heal";
    pub const DEATH: &str = "combat.death";
    pub const VICTORY: &str = "combat.victory";
    pub const DEFEAT: &str = "combat.defeat";

    // Error Strings
    pub const ERROR_GENERIC: &str = "errors.generic";
    pub const ERROR_SAVE_FAILED: &str = "errors.save_failed";
    pub const ERROR_LOAD_FAILED: &str = "errors.load_failed";
    pub const ERROR_NETWORK: &str = "errors.network";
    pub const ERROR_INVALID_INPUT: &str = "errors.invalid_input";
    pub const ERROR_FILE_NOT_FOUND: &str = "errors.file_not_found";
    pub const ERROR_PERMISSION_DENIED: &str = "errors.permission_denied";
    pub const ERROR_OUT_OF_MEMORY: &str = "errors.out_of_memory";

    // Settings Strings
    pub const SETTINGS_GRAPHICS: &str = "settings.graphics";
    pub const SETTINGS_AUDIO: &str = "settings.audio";
    pub const SETTINGS_CONTROLS: &str = "settings.controls";
    pub const SETTINGS_LANGUAGE: &str = "settings.language";
    pub const SETTINGS_RESOLUTION: &str = "settings.resolution";
    pub const SETTINGS_FULLSCREEN: &str = "settings.fullscreen";
    pub const SETTINGS_VSYNC: &str = "settings.vsync";
    pub const SETTINGS_ANTI_ALIASING: &str = "settings.anti_aliasing";
    pub const SETTINGS_SHADOWS: &str = "settings.shadows";
    pub const SETTINGS_PARTICLES: &str = "settings.particles";
    pub const SETTINGS_QUALITY: &str = "settings.quality";
    pub const SETTINGS_VOLUME_MASTER: &str = "settings.volume_master";
    pub const SETTINGS_VOLUME_MUSIC: &str = "settings.volume_music";
    pub const SETTINGS_VOLUME_SFX: &str = "settings.volume_sfx";
    pub const SETTINGS_VOLUME_VOICE: &str = "settings.volume_voice";

    // Tutorial Strings
    pub const TUTORIAL_WELCOME: &str = "tutorial.welcome";
    pub const TUTORIAL_MOVEMENT: &str = "tutorial.movement";
    pub const TUTORIAL_COMBAT: &str = "tutorial.combat";
    pub const TUTORIAL_INVENTORY: &str = "tutorial.inventory";
    pub const TUTORIAL_SKILLS: &str = "tutorial.skills";
    pub const TUTORIAL_ABILITIES: &str = "tutorial.abilities";
    pub const TUTORIAL_LEVELING: &str = "tutorial.leveling";
    pub const TUTORIAL_EQUIPMENT: &str = "tutorial.equipment";
    pub const TUTORIAL_SAVING: &str = "tutorial.saving";
    pub const TUTORIAL_LOADING: &str = "tutorial.loading";
}

/// String ID registry for managing all string IDs
pub struct StringIdRegistry {
    /// Registered string IDs by category
    categories: HashMap<StringCategory, Vec<StringId>>,
    /// All registered string IDs
    all_ids: Vec<StringId>,
    /// String ID metadata
    metadata: HashMap<StringId, StringIdMetadata>,
}

/// Metadata for a string ID
#[derive(Debug, Clone)]
pub struct StringIdMetadata {
    /// The string ID
    pub id: StringId,
    /// Category
    pub category: StringCategory,
    /// Description for translators
    pub description: Option<String>,
    /// Context for translators
    pub context: Option<String>,
    /// Whether this string has plural forms
    pub has_plural: bool,
    /// Maximum number of characters (for UI layout)
    pub max_length: Option<usize>,
    /// Whether this string can be empty
    pub can_be_empty: bool,
}

impl StringIdRegistry {
    /// Create a new string ID registry
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
            all_ids: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Register a string ID
    pub fn register(&mut self, id: StringId, category: StringCategory, metadata: StringIdMetadata) {
        self.categories.entry(category).or_insert_with(Vec::new).push(id.clone());
        self.all_ids.push(id.clone());
        self.metadata.insert(id, metadata);
    }

    /// Get all string IDs for a category
    pub fn get_category_ids(&self, category: &StringCategory) -> Option<&Vec<StringId>> {
        self.categories.get(category)
    }

    /// Get all registered string IDs
    pub fn get_all_ids(&self) -> &Vec<StringId> {
        &self.all_ids
    }

    /// Get metadata for a string ID
    pub fn get_metadata(&self, id: &StringId) -> Option<&StringIdMetadata> {
        self.metadata.get(id)
    }

    /// Check if a string ID is registered
    pub fn is_registered(&self, id: &StringId) -> bool {
        self.metadata.contains_key(id)
    }

    /// Get statistics for the registry
    pub fn get_stats(&self) -> StringIdStats {
        let mut stats = StringIdStats::new();
        
        for (category, ids) in &self.categories {
            stats.category_counts.insert(category.clone(), ids.len());
        }
        
        stats.total_ids = self.all_ids.len();
        stats
    }
}

/// Statistics for the string ID registry
#[derive(Debug, Clone)]
pub struct StringIdStats {
    pub total_ids: usize,
    pub category_counts: HashMap<StringCategory, usize>,
}

impl StringIdStats {
    pub fn new() -> Self {
        Self {
            total_ids: 0,
            category_counts: HashMap::new(),
        }
    }
}

/// Helper functions for creating string IDs
pub mod helpers {
    use super::*;

    /// Create a UI string ID
    pub fn ui(subcategory: &str, identifier: &str) -> StringId {
        StringIdBuilder::new(StringCategory::UI)
            .subcategory(subcategory)
            .identifier(identifier)
            .build()
    }

    /// Create a gameplay string ID
    pub fn gameplay(subcategory: &str, identifier: &str) -> StringId {
        StringIdBuilder::new(StringCategory::Gameplay)
            .subcategory(subcategory)
            .identifier(identifier)
            .build()
    }

    /// Create a character string ID
    pub fn character(subcategory: &str, identifier: &str) -> StringId {
        StringIdBuilder::new(StringCategory::Characters)
            .subcategory(subcategory)
            .identifier(identifier)
            .build()
    }

    /// Create an item string ID
    pub fn item(subcategory: &str, identifier: &str) -> StringId {
        StringIdBuilder::new(StringCategory::Items)
            .subcategory(subcategory)
            .identifier(identifier)
            .build()
    }

    /// Create a combat string ID
    pub fn combat(subcategory: &str, identifier: &str) -> StringId {
        StringIdBuilder::new(StringCategory::Combat)
            .subcategory(subcategory)
            .identifier(identifier)
            .build()
    }

    /// Create an error string ID
    pub fn error(identifier: &str) -> StringId {
        StringIdBuilder::new(StringCategory::Errors)
            .identifier(identifier)
            .build()
    }

    /// Create a tutorial string ID
    pub fn tutorial(subcategory: &str, identifier: &str) -> StringId {
        StringIdBuilder::new(StringCategory::Tutorial)
            .subcategory(subcategory)
            .identifier(identifier)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_id_builder() {
        let id = StringIdBuilder::new(StringCategory::UI)
            .subcategory("menu")
            .identifier("play_button")
            .build();
        
        assert_eq!(id.as_str(), "ui.menu.play_button");
    }

    #[test]
    fn test_string_id_builder_no_subcategory() {
        let id = StringIdBuilder::new(StringCategory::UI)
            .identifier("play")
            .build();
        
        assert_eq!(id.as_str(), "ui.play");
    }

    #[test]
    fn test_string_id_builder_no_identifier() {
        let id = StringIdBuilder::new(StringCategory::UI)
            .build();
        
        assert_eq!(id.as_str(), "ui");
    }

    #[test]
    fn test_common_string_ids() {
        assert_eq!(CommonStringIds::MAIN_MENU, "ui.main_menu");
        assert_eq!(CommonStringIds::PLAY, "ui.play");
        assert_eq!(CommonStringIds::ATTACK, "combat.attack");
    }

    #[test]
    fn test_helpers() {
        let ui_id = helpers::ui("menu", "play");
        assert_eq!(ui_id.as_str(), "ui.menu.play");
        
        let gameplay_id = helpers::gameplay("level", "complete");
        assert_eq!(gameplay_id.as_str(), "gameplay.level.complete");
    }

    #[test]
    fn test_string_id_registry() {
        let mut registry = StringIdRegistry::new();
        let id = string_id!("test.id");
        let metadata = StringIdMetadata {
            id: id.clone(),
            category: StringCategory::UI,
            description: Some("Test string".to_string()),
            context: None,
            has_plural: false,
            max_length: None,
            can_be_empty: false,
        };
        
        registry.register(id.clone(), StringCategory::UI, metadata);
        assert!(registry.is_registered(&id));
        assert_eq!(registry.get_all_ids().len(), 1);
    }
}
