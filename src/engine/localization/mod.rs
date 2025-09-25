//! Localization system for the 2D Brawler Engine
//! 
//! This module provides comprehensive internationalization support with:
//! - String ID-based localization system
//! - Support for 8 target languages (FIGS, BRPT, KO, JP, Chinese Simplified, Chinese Traditional, Turkish, Arabic)
//! - Fallback system for missing translations
//! - Dynamic language switching
//! - Pluralization support
//! - Text formatting and interpolation

pub mod language;
pub mod manager;
pub mod string_id;
pub mod translation;
pub mod fallback;
pub mod string_registry;
pub mod ui_mirroring;

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

/// Text alignment for different languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextAlignment {
    Left,
    Right,
    Center,
    Justify,
}

/// Supported languages in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    /// English (FIGS - French, Italian, German, Spanish)
    English,
    /// French
    French,
    /// Italian
    Italian,
    /// German
    German,
    /// Spanish
    Spanish,
    /// Brazilian Portuguese
    BrazilianPortuguese,
    /// Korean
    Korean,
    /// Japanese
    Japanese,
    /// Chinese Simplified
    ChineseSimplified,
    /// Chinese Traditional
    ChineseTraditional,
    /// Turkish
    Turkish,
    /// Arabic
    Arabic,
}

impl Language {
    /// Get the language code (ISO 639-1)
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::French => "fr",
            Language::Italian => "it",
            Language::German => "de",
            Language::Spanish => "es",
            Language::BrazilianPortuguese => "pt-BR",
            Language::Korean => "ko",
            Language::Japanese => "ja",
            Language::ChineseSimplified => "zh-CN",
            Language::ChineseTraditional => "zh-TW",
            Language::Turkish => "tr",
            Language::Arabic => "ar",
        }
    }

    /// Get the language name in English
    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Italian => "Italiano",
            Language::German => "Deutsch",
            Language::Spanish => "Español",
            Language::BrazilianPortuguese => "Português (Brasil)",
            Language::Korean => "한국어",
            Language::Japanese => "日本語",
            Language::ChineseSimplified => "简体中文",
            Language::ChineseTraditional => "繁體中文",
            Language::Turkish => "Türkçe",
            Language::Arabic => "العربية",
        }
    }

    /// Get the native language name
    pub fn native_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Italian => "Italiano",
            Language::German => "Deutsch",
            Language::Spanish => "Español",
            Language::BrazilianPortuguese => "Português (Brasil)",
            Language::Korean => "한국어",
            Language::Japanese => "日本語",
            Language::ChineseSimplified => "简体中文",
            Language::ChineseTraditional => "繁體中文",
            Language::Turkish => "Türkçe",
            Language::Arabic => "العربية",
        }
    }

    /// Check if the language is RTL (Right-to-Left)
    pub fn is_rtl(&self) -> bool {
        matches!(self, Language::Arabic)
    }

    /// Get UI mirroring requirements for the language
    pub fn requires_ui_mirroring(&self) -> bool {
        self.is_rtl()
    }

    /// Get text alignment for the language
    pub fn text_alignment(&self) -> TextAlignment {
        if self.is_rtl() {
            TextAlignment::Right
        } else {
            TextAlignment::Left
        }
    }

    /// Get all supported languages
    pub fn all() -> Vec<Language> {
        vec![
            Language::English,
            Language::French,
            Language::Italian,
            Language::German,
            Language::Spanish,
            Language::BrazilianPortuguese,
            Language::Korean,
            Language::Japanese,
            Language::ChineseSimplified,
            Language::ChineseTraditional,
            Language::Turkish,
            Language::Arabic,
        ]
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

/// String ID for localization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringId(pub String);

impl StringId {
    /// Create a new string ID
    pub fn new(id: impl Into<String>) -> Self {
        StringId(id.into())
    }

    /// Get the string ID as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for StringId {
    fn from(s: &str) -> Self {
        StringId(s.to_string())
    }
}

impl From<String> for StringId {
    fn from(s: String) -> Self {
        StringId(s)
    }
}

/// Translation entry for a specific string ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    /// The string ID
    pub id: StringId,
    /// The translated text
    pub text: String,
    /// Optional context for translators
    pub context: Option<String>,
    /// Whether this is a plural form
    pub is_plural: bool,
    /// Plural count (for plural forms)
    pub plural_count: Option<u32>,
    /// Whether this string can be empty
    pub can_be_empty: bool,
    /// Maximum number of characters (for UI layout)
    pub max_length: Option<usize>,
}

/// Localization manager for handling translations
#[derive(Debug, Clone)]
pub struct LocalizationManager {
    /// Current language
    current_language: Language,
    /// Fallback language
    fallback_language: Language,
    /// Translations for each language
    translations: HashMap<Language, HashMap<StringId, Translation>>,
    /// String interpolation variables
    variables: HashMap<String, String>,
}

impl LocalizationManager {
    /// Create a new localization manager
    pub fn new() -> Self {
        Self {
            current_language: Language::default(),
            fallback_language: Language::English,
            translations: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    /// Set the current language
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    /// Get the current language
    pub fn current_language(&self) -> Language {
        self.current_language
    }

    /// Set the fallback language
    pub fn set_fallback_language(&mut self, language: Language) {
        self.fallback_language = language;
    }

    /// Get the fallback language
    pub fn fallback_language(&self) -> Language {
        self.fallback_language
    }

    /// Load translations for a language
    pub fn load_translations(&mut self, language: Language, translations: HashMap<StringId, Translation>) {
        self.translations.insert(language, translations);
    }

    /// Get a translated string by ID
    pub fn get(&self, id: &StringId) -> Option<&str> {
        // Try current language first
        if let Some(lang_translations) = self.translations.get(&self.current_language) {
            if let Some(translation) = lang_translations.get(id) {
                return Some(&translation.text);
            }
        }

        // Fall back to fallback language
        if let Some(lang_translations) = self.translations.get(&self.fallback_language) {
            if let Some(translation) = lang_translations.get(id) {
                return Some(&translation.text);
            }
        }

        None
    }

    /// Get a translated string with fallback to ID
    pub fn get_or_id(&self, id: &StringId) -> String {
        self.get(id).map(|s| s.to_string()).unwrap_or_else(|| id.as_str().to_string())
    }

    /// Get a translated string with interpolation
    pub fn get_with_vars(&self, id: &StringId, variables: &HashMap<String, String>) -> String {
        let text = self.get_or_id(id);
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
        self.variables.insert(key, value);
    }

    /// Get a variable
    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    /// Clear all variables
    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }

    /// Check if a translation exists for the current language
    pub fn has_translation(&self, id: &StringId) -> bool {
        self.translations
            .get(&self.current_language)
            .map_or(false, |lang_translations| lang_translations.contains_key(id))
    }

    /// Get all available languages
    pub fn available_languages(&self) -> Vec<Language> {
        self.translations.keys().cloned().collect()
    }

    /// Get translation statistics
    pub fn get_stats(&self) -> LocalizationStats {
        let mut stats = LocalizationStats::new();
        
        for (language, translations) in &self.translations {
            let language_stats = LanguageStats {
                language: *language,
                total_strings: translations.len(),
                translated_strings: translations.len(),
                missing_strings: 0,
            };
            stats.language_stats.insert(*language, language_stats);
        }
        
        stats
    }
}

impl Default for LocalizationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for localization
#[derive(Debug, Clone)]
pub struct LocalizationStats {
    pub language_stats: HashMap<Language, LanguageStats>,
}

impl LocalizationStats {
    pub fn new() -> Self {
        Self {
            language_stats: HashMap::new(),
        }
    }
}

/// Statistics for a specific language
#[derive(Debug, Clone)]
pub struct LanguageStats {
    pub language: Language,
    pub total_strings: usize,
    pub translated_strings: usize,
    pub missing_strings: usize,
}

/// Localization result
pub type LocalizationResult<T> = Result<T, LocalizationError>;

/// Localization errors
#[derive(Debug, Clone, PartialEq)]
pub enum LocalizationError {
    /// String ID not found
    StringIdNotFound(StringId),
    /// Language not supported
    LanguageNotSupported(Language),
    /// Translation file error
    TranslationFileError(String),
    /// Invalid string ID format
    InvalidStringId(String),
}

impl std::fmt::Display for LocalizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocalizationError::StringIdNotFound(id) => {
                write!(f, "String ID not found: {}", id.as_str())
            }
            LocalizationError::LanguageNotSupported(lang) => {
                write!(f, "Language not supported: {:?}", lang)
            }
            LocalizationError::TranslationFileError(msg) => {
                write!(f, "Translation file error: {}", msg)
            }
            LocalizationError::InvalidStringId(id) => {
                write!(f, "Invalid string ID format: {}", id)
            }
        }
    }
}

impl std::error::Error for LocalizationError {}

/// Macro for easy string ID creation
#[macro_export]
macro_rules! string_id {
    ($id:expr) => {
        $crate::engine::localization::StringId::new($id)
    };
}

/// Macro for easy localization
#[macro_export]
macro_rules! t {
    ($manager:expr, $id:expr) => {
        $manager.get_or_id(&string_id!($id))
    };
    ($manager:expr, $id:expr, $vars:expr) => {
        $manager.get_with_vars(&string_id!($id), $vars)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_codes() {
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::French.code(), "fr");
        assert_eq!(Language::Korean.code(), "ko");
        assert_eq!(Language::Japanese.code(), "ja");
        assert_eq!(Language::Arabic.code(), "ar");
    }

    #[test]
    fn test_rtl_languages() {
        assert!(!Language::English.is_rtl());
        assert!(!Language::French.is_rtl());
        assert!(!Language::Korean.is_rtl());
        assert!(Language::Arabic.is_rtl());
    }

    #[test]
    fn test_string_id() {
        let id = StringId::new("test.string");
        assert_eq!(id.as_str(), "test.string");
    }

    #[test]
    fn test_localization_manager() {
        let mut manager = LocalizationManager::new();
        assert_eq!(manager.current_language(), Language::English);
        assert_eq!(manager.fallback_language(), Language::English);
    }

    #[test]
    fn test_translation_loading() {
        let mut manager = LocalizationManager::new();
        let mut translations = HashMap::new();
        translations.insert(
            string_id!("test.hello"),
            Translation {
                id: string_id!("test.hello"),
                text: "Hello".to_string(),
                context: None,
                is_plural: false,
                plural_count: None,
            },
        );
        
        manager.load_translations(Language::English, translations);
        assert!(manager.has_translation(&string_id!("test.hello")));
        assert_eq!(manager.get(&string_id!("test.hello")), Some("Hello"));
    }
}
