//! Translation system for managing localized strings
//! 
//! This module provides functionality for loading, managing, and using
//! translations for different languages.

use super::{Language, StringId, Translation, LocalizationError, LocalizationResult};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

/// Translation file format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationFile {
    /// Language of this translation file
    pub language: Language,
    /// Version of the translation file
    pub version: String,
    /// Last updated timestamp
    pub last_updated: String,
    /// Translations
    pub translations: HashMap<StringId, Translation>,
}

/// Translation loader for loading translation files
pub struct TranslationLoader;

impl TranslationLoader {
    /// Load translations from a JSON file
    pub fn load_from_json<P: AsRef<Path>>(path: P) -> LocalizationResult<TranslationFile> {
        let content = fs::read_to_string(path)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to read file: {}", e)))?;
        
        let translation_file: TranslationFile = serde_json::from_str(&content)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to parse JSON: {}", e)))?;
        
        Ok(translation_file)
    }

    /// Load translations from a TOML file
    pub fn load_from_toml<P: AsRef<Path>>(path: P) -> LocalizationResult<TranslationFile> {
        let content = fs::read_to_string(path)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to read file: {}", e)))?;
        
        let translation_file: TranslationFile = toml::from_str(&content)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to parse TOML: {}", e)))?;
        
        Ok(translation_file)
    }

    /// Load translations from a YAML file
    pub fn load_from_yaml<P: AsRef<Path>>(path: P) -> LocalizationResult<TranslationFile> {
        let content = fs::read_to_string(path)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to read file: {}", e)))?;
        
        let translation_file: TranslationFile = serde_yaml::from_str(&content)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to parse YAML: {}", e)))?;
        
        Ok(translation_file)
    }

    /// Save translations to a JSON file
    pub fn save_to_json<P: AsRef<Path>>(translation_file: &TranslationFile, path: P) -> LocalizationResult<()> {
        let content = serde_json::to_string_pretty(translation_file)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to serialize JSON: {}", e)))?;
        
        fs::write(path, content)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to write file: {}", e)))?;
        
        Ok(())
    }

    /// Save translations to a TOML file
    pub fn save_to_toml<P: AsRef<Path>>(translation_file: &TranslationFile, path: P) -> LocalizationResult<()> {
        let content = toml::to_string_pretty(translation_file)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to serialize TOML: {}", e)))?;
        
        fs::write(path, content)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to write file: {}", e)))?;
        
        Ok(())
    }

    /// Save translations to a YAML file
    pub fn save_to_yaml<P: AsRef<Path>>(translation_file: &TranslationFile, path: P) -> LocalizationResult<()> {
        let content = serde_yaml::to_string(translation_file)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to serialize YAML: {}", e)))?;
        
        fs::write(path, content)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to write file: {}", e)))?;
        
        Ok(())
    }
}

/// Translation manager for handling multiple translation files
pub struct TranslationManager {
    /// Translation files by language
    pub translation_files: HashMap<Language, TranslationFile>,
    /// Current language
    current_language: Language,
    /// Fallback language
    fallback_language: Language,
}

impl TranslationManager {
    /// Create a new translation manager
    pub fn new() -> Self {
        Self {
            translation_files: HashMap::new(),
            current_language: Language::default(),
            fallback_language: Language::English,
        }
    }

    /// Load a translation file
    pub fn load_translation_file(&mut self, translation_file: TranslationFile) {
        self.translation_files.insert(translation_file.language, translation_file);
    }

    /// Load translations from a directory
    pub fn load_from_directory<P: AsRef<Path>>(&mut self, directory: P) -> LocalizationResult<()> {
        let dir = directory.as_ref();
        
        if !dir.exists() {
            return Err(LocalizationError::TranslationFileError(
                format!("Directory does not exist: {}", dir.display())
            ));
        }

        for entry in fs::read_dir(dir)
            .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to read directory: {}", e)))? {
            let entry = entry
                .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to read directory entry: {}", e)))?;
            
            let path = entry.path();
            
            if path.is_file() {
                let extension = path.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("");
                
                let translation_file = match extension {
                    "json" => TranslationLoader::load_from_json(&path)?,
                    "toml" => TranslationLoader::load_from_toml(&path)?,
                    "yaml" | "yml" => TranslationLoader::load_from_yaml(&path)?,
                    _ => continue,
                };
                
                self.load_translation_file(translation_file);
            }
        }
        
        Ok(())
    }

    /// Get a translation for a string ID
    pub fn get_translation(&self, id: &StringId) -> Option<&Translation> {
        // Try current language first
        if let Some(file) = self.translation_files.get(&self.current_language) {
            if let Some(translation) = file.translations.get(id) {
                return Some(translation);
            }
        }

        // Fall back to fallback language
        if let Some(file) = self.translation_files.get(&self.fallback_language) {
            if let Some(translation) = file.translations.get(id) {
                return Some(translation);
            }
        }

        None
    }

    /// Get a translated string
    pub fn get_string(&self, id: &StringId) -> Option<&str> {
        self.get_translation(id).map(|t| t.text.as_str())
    }

    /// Get a translated string with fallback to ID
    pub fn get_string_or_id(&self, id: &StringId) -> String {
        self.get_string(id).map(|s| s.to_string()).unwrap_or_else(|| id.as_str().to_string())
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

    /// Check if a language is available
    pub fn is_language_available(&self, language: Language) -> bool {
        self.translation_files.contains_key(&language)
    }

    /// Get available languages
    pub fn available_languages(&self) -> Vec<Language> {
        self.translation_files.keys().cloned().collect()
    }

    /// Get translation statistics
    pub fn get_stats(&self) -> TranslationStats {
        let mut stats = TranslationStats::new();
        
        for (language, file) in &self.translation_files {
            let language_stats = LanguageTranslationStats {
                language: *language,
                total_strings: file.translations.len(),
                version: file.version.clone(),
                last_updated: file.last_updated.clone(),
            };
            stats.language_stats.insert(*language, language_stats);
        }
        
        stats
    }

    /// Export translations for a language
    pub fn export_language(&self, language: Language, format: TranslationFormat) -> LocalizationResult<String> {
        let file = self.translation_files.get(&language)
            .ok_or_else(|| LocalizationError::LanguageNotSupported(language))?;
        
        match format {
            TranslationFormat::Json => {
                serde_json::to_string_pretty(file)
                    .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to serialize JSON: {}", e)))
            }
            TranslationFormat::Toml => {
                toml::to_string_pretty(file)
                    .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to serialize TOML: {}", e)))
            }
            TranslationFormat::Yaml => {
                serde_yaml::to_string(file)
                    .map_err(|e| LocalizationError::TranslationFileError(format!("Failed to serialize YAML: {}", e)))
            }
        }
    }
}

impl Default for TranslationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Translation format for export
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslationFormat {
    Json,
    Toml,
    Yaml,
}

/// Translation statistics
#[derive(Debug, Clone)]
pub struct TranslationStats {
    pub language_stats: HashMap<Language, LanguageTranslationStats>,
}

impl TranslationStats {
    pub fn new() -> Self {
        Self {
            language_stats: HashMap::new(),
        }
    }
}

/// Language-specific translation statistics
#[derive(Debug, Clone)]
pub struct LanguageTranslationStats {
    pub language: Language,
    pub total_strings: usize,
    pub version: String,
    pub last_updated: String,
}

/// Translation validation
pub struct TranslationValidator;

impl TranslationValidator {
    /// Validate a translation file
    pub fn validate(translation_file: &TranslationFile) -> Vec<TranslationValidationError> {
        let mut errors = Vec::new();
        
        for (id, translation) in &translation_file.translations {
            // Check if translation ID matches the key
            if id.as_str() != translation.id.as_str() {
                errors.push(TranslationValidationError::IdMismatch {
                    key: id.as_str().to_string(),
                    translation_id: translation.id.as_str().to_string(),
                });
            }
            
            // Check if text is empty when it shouldn't be
            if translation.text.trim().is_empty() && !translation.can_be_empty {
                errors.push(TranslationValidationError::EmptyText {
                    id: id.as_str().to_string(),
                });
            }
            
            // Check text length if max_length is specified
            if let Some(max_length) = translation.max_length {
                if translation.text.len() > max_length {
                    errors.push(TranslationValidationError::TextTooLong {
                        id: id.as_str().to_string(),
                        length: translation.text.len(),
                        max_length,
                    });
                }
            }
        }
        
        errors
    }
}

/// Translation validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum TranslationValidationError {
    IdMismatch { key: String, translation_id: String },
    EmptyText { id: String },
    TextTooLong { id: String, length: usize, max_length: usize },
}

impl std::fmt::Display for TranslationValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslationValidationError::IdMismatch { key, translation_id } => {
                write!(f, "ID mismatch: key '{}' has translation ID '{}'", key, translation_id)
            }
            TranslationValidationError::EmptyText { id } => {
                write!(f, "Empty text for ID: {}", id)
            }
            TranslationValidationError::TextTooLong { id, length, max_length } => {
                write!(f, "Text too long for ID '{}': {} characters (max: {})", id, length, max_length)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_file() {
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
        
        let file = TranslationFile {
            language: Language::English,
            version: "1.0.0".to_string(),
            last_updated: "2024-01-01".to_string(),
            translations,
        };
        
        assert_eq!(file.language, Language::English);
        assert_eq!(file.translations.len(), 1);
    }

    #[test]
    fn test_translation_manager() {
        let mut manager = TranslationManager::new();
        assert_eq!(manager.current_language(), Language::English);
        assert_eq!(manager.fallback_language(), Language::English);
    }

    #[test]
    fn test_translation_validation() {
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
        
        let file = TranslationFile {
            language: Language::English,
            version: "1.0.0".to_string(),
            last_updated: "2024-01-01".to_string(),
            translations,
        };
        
        let errors = TranslationValidator::validate(&file);
        assert!(errors.is_empty());
    }
}
