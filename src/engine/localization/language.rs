//! Language-specific utilities and constants

use super::{Language, StringId, Translation};
use std::collections::HashMap;

/// Language-specific constants and utilities
pub struct LanguageInfo {
    /// Language name in English
    pub english_name: &'static str,
    /// Native language name
    pub native_name: &'static str,
    /// ISO 639-1 language code
    pub code: &'static str,
    /// Whether the language is RTL
    pub is_rtl: bool,
    /// Character encoding
    pub encoding: &'static str,
    /// Plural rules for the language
    pub plural_rules: PluralRules,
}

/// Plural rules for different languages
#[derive(Debug, Clone, Copy)]
pub enum PluralRules {
    /// English-style pluralization (1 item, 2+ items)
    English,
    /// French-style pluralization (0,1 item, 2+ items)
    French,
    /// Russian-style pluralization (1 item, 2-4 items, 5+ items)
    Russian,
    /// Arabic-style pluralization (0,1 item, 2-10 items, 11+ items)
    Arabic,
    /// Chinese-style (no pluralization)
    Chinese,
    /// Japanese-style (no pluralization)
    Japanese,
    /// Korean-style (no pluralization)
    Korean,
}

impl PluralRules {
    /// Get the plural form index for a given count
    pub fn get_plural_form(&self, count: u32) -> usize {
        match self {
            PluralRules::English => {
                if count == 1 { 0 } else { 1 }
            }
            PluralRules::French => {
                if count <= 1 { 0 } else { 1 }
            }
            PluralRules::Russian => {
                if count % 10 == 1 && count % 100 != 11 { 0 }
                else if count % 10 >= 2 && count % 10 <= 4 && (count % 100 < 10 || count % 100 >= 20) { 1 }
                else { 2 }
            }
            PluralRules::Arabic => {
                if count == 0 { 0 }
                else if count == 1 { 1 }
                else if count == 2 { 2 }
                else if count >= 3 && count <= 10 { 3 }
                else { 4 }
            }
            PluralRules::Chinese | PluralRules::Japanese | PluralRules::Korean => 0,
        }
    }

    /// Get the number of plural forms for this language
    pub fn get_plural_count(&self) -> usize {
        match self {
            PluralRules::English | PluralRules::French => 2,
            PluralRules::Russian => 3,
            PluralRules::Arabic => 5,
            PluralRules::Chinese | PluralRules::Japanese | PluralRules::Korean => 1,
        }
    }
}

impl Language {
    /// Get detailed language information
    pub fn info(&self) -> LanguageInfo {
        match self {
            Language::English => LanguageInfo {
                english_name: "English",
                native_name: "English",
                code: "en",
                is_rtl: false,
                encoding: "UTF-8",
                plural_rules: PluralRules::English,
            },
            Language::French => LanguageInfo {
                english_name: "French",
                native_name: "Français",
                code: "fr",
                is_rtl: false,
                encoding: "UTF-8",
                plural_rules: PluralRules::French,
            },
            Language::Italian => LanguageInfo {
                english_name: "Italian",
                native_name: "Italiano",
                code: "it",
                is_rtl: false,
                encoding: "UTF-8",
                plural_rules: PluralRules::English,
            },
            Language::German => LanguageInfo {
                english_name: "German",
                native_name: "Deutsch",
                code: "de",
                is_rtl: false,
                encoding: "UTF-8",
                plural_rules: PluralRules::English,
            },
            Language::Spanish => LanguageInfo {
                english_name: "Spanish",
                native_name: "Español",
                code: "es",
                is_rtl: false,
                encoding: "UTF-8",
                plural_rules: PluralRules::English,
            },
            Language::BrazilianPortuguese => LanguageInfo {
                english_name: "Brazilian Portuguese",
                native_name: "Português (Brasil)",
                code: "pt-BR",
                is_rtl: false,
                encoding: "UTF-8",
                plural_rules: PluralRules::English,
            },
            Language::Korean => LanguageInfo {
                english_name: "Korean",
                native_name: "한국어",
                code: "ko",
                is_rtl: false,
                encoding: "UTF-8",
                plural_rules: PluralRules::Korean,
            },
            Language::Japanese => LanguageInfo {
                english_name: "Japanese",
                native_name: "日本語",
                code: "ja",
                is_rtl: false,
                encoding: "UTF-8",
                plural_rules: PluralRules::Japanese,
            },
            Language::ChineseSimplified => LanguageInfo {
                english_name: "Chinese Simplified",
                native_name: "简体中文",
                code: "zh-CN",
                is_rtl: false,
                encoding: "UTF-8",
                plural_rules: PluralRules::Chinese,
            },
            Language::ChineseTraditional => LanguageInfo {
                english_name: "Chinese Traditional",
                native_name: "繁體中文",
                code: "zh-TW",
                is_rtl: false,
                encoding: "UTF-8",
                plural_rules: PluralRules::Chinese,
            },
            Language::Turkish => LanguageInfo {
                english_name: "Turkish",
                native_name: "Türkçe",
                code: "tr",
                is_rtl: false,
                encoding: "UTF-8",
                plural_rules: PluralRules::English,
            },
            Language::Arabic => LanguageInfo {
                english_name: "Arabic",
                native_name: "العربية",
                code: "ar",
                is_rtl: true,
                encoding: "UTF-8",
                plural_rules: PluralRules::Arabic,
            },
        }
    }

    /// Get the plural form for a given count
    pub fn get_plural_form(&self, count: u32) -> usize {
        self.info().plural_rules.get_plural_form(count)
    }

    /// Get the number of plural forms for this language
    pub fn get_plural_count(&self) -> usize {
        self.info().plural_rules.get_plural_count()
    }


    /// Get the character encoding for this language
    pub fn encoding(&self) -> &'static str {
        self.info().encoding
    }
}

/// Text direction for UI layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
}

impl From<Language> for TextDirection {
    fn from(language: Language) -> Self {
        if language.is_rtl() {
            TextDirection::RightToLeft
        } else {
            TextDirection::LeftToRight
        }
    }
}

/// Font information for different languages
#[derive(Debug, Clone)]
pub struct FontInfo {
    /// Primary font family
    pub primary_font: String,
    /// Fallback font families
    pub fallback_fonts: Vec<String>,
    /// Font size multiplier for this language
    pub size_multiplier: f32,
    /// Whether the font supports this language
    pub supports_language: bool,
}

impl Language {
    /// Get recommended font information for this language
    pub fn get_font_info(&self) -> FontInfo {
        match self {
            Language::English | Language::French | Language::Italian | 
            Language::German | Language::Spanish | Language::BrazilianPortuguese => FontInfo {
                primary_font: "Arial".to_string(),
                fallback_fonts: vec!["Helvetica".to_string(), "sans-serif".to_string()],
                size_multiplier: 1.0,
                supports_language: true,
            },
            Language::Korean => FontInfo {
                primary_font: "Malgun Gothic".to_string(),
                fallback_fonts: vec!["Dotum".to_string(), "Arial Unicode MS".to_string()],
                size_multiplier: 1.1,
                supports_language: true,
            },
            Language::Japanese => FontInfo {
                primary_font: "Hiragino Sans".to_string(),
                fallback_fonts: vec!["Yu Gothic".to_string(), "Meiryo".to_string()],
                size_multiplier: 1.0,
                supports_language: true,
            },
            Language::ChineseSimplified | Language::ChineseTraditional => FontInfo {
                primary_font: "PingFang SC".to_string(),
                fallback_fonts: vec!["Microsoft YaHei".to_string(), "SimHei".to_string()],
                size_multiplier: 1.0,
                supports_language: true,
            },
            Language::Turkish => FontInfo {
                primary_font: "Arial".to_string(),
                fallback_fonts: vec!["Helvetica".to_string(), "sans-serif".to_string()],
                size_multiplier: 1.0,
                supports_language: true,
            },
            Language::Arabic => FontInfo {
                primary_font: "Arial".to_string(),
                fallback_fonts: vec!["Tahoma".to_string(), "sans-serif".to_string()],
                size_multiplier: 1.2,
                supports_language: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_info() {
        let info = Language::English.info();
        assert_eq!(info.english_name, "English");
        assert_eq!(info.code, "en");
        assert!(!info.is_rtl);
    }

    #[test]
    fn test_rtl_language() {
        let info = Language::Arabic.info();
        assert!(info.is_rtl);
    }

    #[test]
    fn test_plural_rules() {
        assert_eq!(PluralRules::English.get_plural_form(1), 0);
        assert_eq!(PluralRules::English.get_plural_form(2), 1);
        
        assert_eq!(PluralRules::French.get_plural_form(0), 0);
        assert_eq!(PluralRules::French.get_plural_form(1), 0);
        assert_eq!(PluralRules::French.get_plural_form(2), 1);
    }

    #[test]
    fn test_text_direction() {
        assert_eq!(TextDirection::from(Language::English), TextDirection::LeftToRight);
        assert_eq!(TextDirection::from(Language::Arabic), TextDirection::RightToLeft);
    }

    #[test]
    fn test_font_info() {
        let font_info = Language::Korean.get_font_info();
        assert_eq!(font_info.primary_font, "Malgun Gothic");
        assert!(font_info.supports_language);
    }
}
