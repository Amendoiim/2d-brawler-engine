//! Localization manager for the game engine
//! 
//! This module provides the main localization manager that coordinates
//! all localization functionality.

use super::{
    Language, StringId, LocalizationManager, LocalizationError, LocalizationResult,
};
use super::translation::{TranslationManager, TranslationFile, TranslationFormat};
use super::fallback::{FallbackManager, FallbackStrategy};
use super::string_id::{StringIdRegistry, StringCategory};
use super::ui_mirroring::{UIMirroringManager, UIElementType};
use super::TextAlignment;
use std::collections::HashMap;
use std::path::Path;

/// Main localization manager that coordinates all localization functionality
pub struct GameLocalizationManager {
    /// Core localization manager
    localization_manager: LocalizationManager,
    /// Translation manager
    translation_manager: TranslationManager,
    /// Fallback manager
    fallback_manager: FallbackManager,
    /// String ID registry
    string_id_registry: StringIdRegistry,
    /// UI mirroring manager
    ui_mirroring_manager: UIMirroringManager,
    /// Current language
    current_language: Language,
    /// Available languages
    available_languages: Vec<Language>,
}

impl GameLocalizationManager {
    /// Create a new game localization manager
    pub fn new() -> Self {
        let mut manager = Self {
            localization_manager: LocalizationManager::new(),
            translation_manager: TranslationManager::new(),
            fallback_manager: FallbackManager::new(FallbackStrategy::UseStringId),
            string_id_registry: StringIdRegistry::new(),
            ui_mirroring_manager: UIMirroringManager::new(),
            current_language: Language::default(),
            available_languages: vec![Language::English],
        };
        
        // Initialize with default fallback texts
        manager.initialize_default_fallbacks();
        
        manager
    }

    /// Initialize default fallback texts
    fn initialize_default_fallbacks(&mut self) {
        let fallbacks = crate::engine::localization::fallback::FallbackTextGenerator::generate_all_fallbacks();
        
        for (id, text) in fallbacks {
            self.fallback_manager.add_fallback_translation(id, text);
        }
    }

    /// Load translations from a directory
    pub fn load_translations_from_directory<P: AsRef<Path>>(&mut self, directory: P) -> LocalizationResult<()> {
        self.translation_manager.load_from_directory(directory)?;
        self.available_languages = self.translation_manager.available_languages();
        Ok(())
    }

    /// Load a specific translation file
    pub fn load_translation_file(&mut self, translation_file: TranslationFile) {
        self.translation_manager.load_translation_file(translation_file);
        self.available_languages = self.translation_manager.available_languages();
    }

    /// Set the current language
    pub fn set_language(&mut self, language: Language) -> LocalizationResult<()> {
        if !self.available_languages.contains(&language) {
            return Err(LocalizationError::LanguageNotSupported(language));
        }
        
        self.current_language = language;
        self.localization_manager.set_language(language);
        self.translation_manager.set_language(language);
        
        Ok(())
    }

    /// Get the current language
    pub fn current_language(&self) -> Language {
        self.current_language
    }

    /// Get available languages
    pub fn available_languages(&self) -> &[Language] {
        &self.available_languages
    }

    /// Set the fallback language
    pub fn set_fallback_language(&mut self, language: Language) {
        self.localization_manager.set_fallback_language(language);
        self.translation_manager.set_fallback_language(language);
    }

    /// Get the fallback language
    pub fn fallback_language(&self) -> Language {
        self.localization_manager.fallback_language()
    }

    /// Get a translated string by ID
    pub fn get_string(&self, id: &StringId) -> String {
        // Try translation manager first
        if let Some(text) = self.translation_manager.get_string(id) {
            return text.to_string();
        }
        
        // Try fallback manager
        if let Some(text) = self.fallback_manager.get_fallback_translation(id) {
            return text.clone();
        }
        
        // Use fallback strategy
        self.fallback_manager.get_fallback_text(id, &self.available_languages)
    }

    /// Get a translated string with variables
    pub fn get_string_with_vars(&self, id: &StringId, variables: &HashMap<String, String>) -> String {
        let text = self.get_string(id);
        self.interpolate_string(&text, variables)
    }

    /// Interpolate variables in a string
    fn interpolate_string(&self, text: &str, variables: &HashMap<String, String>) -> String {
        let mut result = text.to_string();
        
        for (key, value) in variables {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        result
    }

    /// Set a variable for interpolation
    pub fn set_variable(&mut self, key: String, value: String) {
        self.localization_manager.set_variable(key, value);
    }

    /// Get a variable
    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.localization_manager.get_variable(key)
    }

    /// Clear all variables
    pub fn clear_variables(&mut self) {
        self.localization_manager.clear_variables();
    }

    /// Check if a translation exists
    pub fn has_translation(&self, id: &StringId) -> bool {
        self.translation_manager.get_string(id).is_some() || 
        self.fallback_manager.has_fallback_translation(id)
    }

    /// Register a string ID
    pub fn register_string_id(&mut self, id: StringId, category: StringCategory, description: Option<String>) {
        use super::string_id::StringIdMetadata;
        
        let metadata = StringIdMetadata {
            id: id.clone(),
            category: category.clone(),
            description,
            context: None,
            has_plural: false,
            max_length: None,
            can_be_empty: false,
        };
        
        self.string_id_registry.register(id, category, metadata);
    }

    /// Get string ID statistics
    pub fn get_string_id_stats(&self) -> super::string_id::StringIdStats {
        self.string_id_registry.get_stats()
    }

    /// Get translation statistics
    pub fn get_translation_stats(&self) -> super::translation::TranslationStats {
        self.translation_manager.get_stats()
    }

    /// Get localization statistics
    pub fn get_localization_stats(&self) -> super::LocalizationStats {
        self.localization_manager.get_stats()
    }

    /// Export translations for a language
    pub fn export_language(&self, language: Language, format: TranslationFormat) -> LocalizationResult<String> {
        self.translation_manager.export_language(language, format)
    }

    /// Get text direction for the current language
    pub fn get_text_direction(&self) -> super::language::TextDirection {
        self.current_language.into()
    }

    /// Get font information for the current language
    pub fn get_font_info(&self) -> super::language::FontInfo {
        self.current_language.get_font_info()
    }

    /// Check if the current language is RTL
    pub fn is_rtl(&self) -> bool {
        self.current_language.is_rtl()
    }

    /// Get plural form for a count
    pub fn get_plural_form(&self, count: u32) -> usize {
        self.current_language.get_plural_form(count)
    }

    /// Get the number of plural forms for the current language
    pub fn get_plural_count(&self) -> usize {
        self.current_language.get_plural_count()
    }

    /// Get a translated string with pluralization
    pub fn get_plural_string(&self, id: &StringId, count: u32) -> String {
        let plural_form = self.get_plural_form(count);
        let plural_id = if plural_form == 0 {
            id.clone()
        } else {
            StringId::new(format!("{}.plural_{}", id.as_str(), plural_form))
        };
        
        self.get_string(&plural_id)
    }

    /// Get a translated string with pluralization and variables
    pub fn get_plural_string_with_vars(&self, id: &StringId, count: u32, variables: &HashMap<String, String>) -> String {
        let text = self.get_plural_string(id, count);
        self.interpolate_string(&text, variables)
    }

    /// Validate all translations
    pub fn validate_translations(&self) -> Vec<super::translation::TranslationValidationError> {
        let mut all_errors = Vec::new();
        
        for language in &self.available_languages {
            if let Some(file) = self.translation_manager.translation_files.get(language) {
                let errors = super::translation::TranslationValidator::validate(file);
                all_errors.extend(errors);
            }
        }
        
        all_errors
    }

    /// Get missing translations for a language
    pub fn get_missing_translations(&self, language: Language) -> Vec<StringId> {
        let mut missing = Vec::new();
        
        if let Some(file) = self.translation_manager.translation_files.get(&language) {
            for id in self.string_id_registry.get_all_ids() {
                if !file.translations.contains_key(id) {
                    missing.push(id.clone());
                }
            }
        }
        
        missing
    }

    /// Get translation coverage for a language
    pub fn get_translation_coverage(&self, language: Language) -> f32 {
        if let Some(file) = self.translation_manager.translation_files.get(&language) {
            let total_strings = self.string_id_registry.get_all_ids().len();
            let translated_strings = file.translations.len();
            
            if total_strings == 0 {
                1.0
            } else {
                translated_strings as f32 / total_strings as f32
            }
        } else {
            0.0
        }
    }

    /// Get all translation coverage statistics
    pub fn get_all_translation_coverage(&self) -> HashMap<Language, f32> {
        let mut coverage = HashMap::new();
        
        for language in &self.available_languages {
            coverage.insert(*language, self.get_translation_coverage(*language));
        }
        
        coverage
    }

    /// Check if UI mirroring is required for the current language
    pub fn requires_ui_mirroring(&self) -> bool {
        self.current_language.requires_ui_mirroring()
    }

    /// Get text alignment for the current language
    pub fn get_text_alignment(&self) -> TextAlignment {
        self.current_language.text_alignment()
    }

    /// Get UI element type for a given element
    pub fn get_ui_element_type(&self, element_name: &str) -> UIElementType {
        self.ui_mirroring_manager.get_element_type(element_name)
    }

    /// Check if a UI element should be mirrored
    pub fn should_mirror_element(&self, element_name: &str) -> bool {
        self.requires_ui_mirroring() && 
        self.ui_mirroring_manager.should_mirror_element(element_name)
    }

    /// Get mirrored position for a UI element
    pub fn get_mirrored_position(&self, element_name: &str, x: f32, y: f32, width: f32, height: f32) -> (f32, f32) {
        if self.should_mirror_element(element_name) {
            let element_type = self.get_ui_element_type(element_name);
            let position = glam::Vec2::new(x, y);
            let bounds = (width, height);
            let mirrored_pos = self.ui_mirroring_manager.mirror_position(position, bounds, element_type);
            (mirrored_pos.x, mirrored_pos.y)
        } else {
            (x, y)
        }
    }

    /// Get mirrored bounds for a UI element
    pub fn get_mirrored_bounds(&self, element_name: &str, x: f32, y: f32, width: f32, height: f32) -> (f32, f32, f32, f32) {
        if self.should_mirror_element(element_name) {
            let element_type = self.get_ui_element_type(element_name);
            let position = glam::Vec2::new(x, y);
            let bounds = (width, height);
            let mirrored_pos = self.ui_mirroring_manager.mirror_position(position, bounds, element_type);
            (mirrored_pos.x, mirrored_pos.y, width, height)
        } else {
            (x, y, width, height)
        }
    }

    /// Register a UI element for mirroring
    pub fn register_ui_element(&mut self, element_name: &str, element_type: UIElementType) {
        self.ui_mirroring_manager.register_element(element_name, element_type);
    }

    /// Get UI mirroring statistics
    pub fn get_ui_mirroring_stats(&self) -> super::ui_mirroring::UIMirroringStats {
        self.ui_mirroring_manager.get_stats()
    }
}

impl Default for GameLocalizationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for common localization tasks
pub mod helpers {
    use super::*;
    use crate::engine::localization::string_id::helpers as string_helpers;

    /// Get a UI string
    pub fn ui(manager: &GameLocalizationManager, subcategory: &str, identifier: &str) -> String {
        let id = string_helpers::ui(subcategory, identifier);
        manager.get_string(&id)
    }

    /// Get a gameplay string
    pub fn gameplay(manager: &GameLocalizationManager, subcategory: &str, identifier: &str) -> String {
        let id = string_helpers::gameplay(subcategory, identifier);
        manager.get_string(&id)
    }

    /// Get a character string
    pub fn character(manager: &GameLocalizationManager, subcategory: &str, identifier: &str) -> String {
        let id = string_helpers::character(subcategory, identifier);
        manager.get_string(&id)
    }

    /// Get an item string
    pub fn item(manager: &GameLocalizationManager, subcategory: &str, identifier: &str) -> String {
        let id = string_helpers::item(subcategory, identifier);
        manager.get_string(&id)
    }

    /// Get a combat string
    pub fn combat(manager: &GameLocalizationManager, subcategory: &str, identifier: &str) -> String {
        let id = string_helpers::combat(subcategory, identifier);
        manager.get_string(&id)
    }

    /// Get an error string
    pub fn error(manager: &GameLocalizationManager, identifier: &str) -> String {
        let id = string_helpers::error(identifier);
        manager.get_string(&id)
    }

    /// Get a tutorial string
    pub fn tutorial(manager: &GameLocalizationManager, subcategory: &str, identifier: &str) -> String {
        let id = string_helpers::tutorial(subcategory, identifier);
        manager.get_string(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_localization_manager() {
        let manager = GameLocalizationManager::new();
        assert_eq!(manager.current_language(), Language::English);
        assert!(manager.available_languages().contains(&Language::English));
    }

    #[test]
    fn test_get_string() {
        let manager = GameLocalizationManager::new();
        let id = string_id!("ui.main_menu");
        let text = manager.get_string(&id);
        assert_eq!(text, "Main Menu");
    }

    #[test]
    fn test_get_string_with_vars() {
        let manager = GameLocalizationManager::new();
        let id = string_id!("ui.welcome");
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "Player".to_string());
        
        // This would need a translation file with the welcome string
        let text = manager.get_string_with_vars(&id, &variables);
        assert!(!text.is_empty());
    }

    #[test]
    fn test_helpers() {
        let manager = GameLocalizationManager::new();
        let text = helpers::ui(&manager, "menu", "play");
        assert_eq!(text, "ui.menu.play"); // Fallback to string ID
    }

    #[test]
    fn test_rtl_detection() {
        let mut manager = GameLocalizationManager::new();
        manager.set_language(Language::Arabic).unwrap();
        assert!(manager.is_rtl());
        
        manager.set_language(Language::English).unwrap();
        assert!(!manager.is_rtl());
    }

    #[test]
    fn test_plural_forms() {
        let manager = GameLocalizationManager::new();
        assert_eq!(manager.get_plural_form(1), 0);
        assert_eq!(manager.get_plural_form(2), 1);
    }

    #[test]
    fn test_ui_mirroring() {
        let mut manager = GameLocalizationManager::new();
        
        // Test English (LTR)
        manager.set_language(Language::English).unwrap();
        assert!(!manager.requires_ui_mirroring());
        assert_eq!(manager.get_text_alignment(), TextAlignment::Left);
        
        // Test Arabic (RTL)
        manager.set_language(Language::Arabic).unwrap();
        assert!(manager.requires_ui_mirroring());
        assert_eq!(manager.get_text_alignment(), TextAlignment::Right);
    }

    #[test]
    fn test_ui_element_mirroring() {
        let mut manager = GameLocalizationManager::new();
        manager.set_language(Language::Arabic).unwrap();
        
        // Register a UI element
        manager.register_ui_element("main_menu_button", UIElementType::Button);
        
        // Test mirroring
        assert!(manager.should_mirror_element("main_menu_button"));
        
        // Test mirrored position
        let (x, y) = manager.get_mirrored_position("main_menu_button", 100.0, 50.0, 200.0, 40.0);
        assert_eq!(x, 700.0); // 800 - 100 - 200
        assert_eq!(y, 50.0);
    }
}
