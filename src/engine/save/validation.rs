//! Save File Validation
//! 
//! This module provides comprehensive validation for save files, including
//! data integrity checks, version compatibility, and corruption detection.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use super::slot::{SaveSlot, SaveSlotData, SaveSlotMetadata};

/// Save file validator
#[derive(Debug, Clone)]
pub struct SaveValidator {
    /// Current game version
    pub current_version: String,
    /// Minimum supported version
    pub min_supported_version: String,
    /// Validation rules
    pub rules: ValidationRules,
}

/// Validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    /// Maximum save file size in bytes
    pub max_file_size: usize,
    /// Maximum play time in seconds
    pub max_play_time: f32,
    /// Maximum character level
    pub max_character_level: u32,
    /// Required fields
    pub required_fields: Vec<String>,
    /// Forbidden characters in names
    pub forbidden_characters: Vec<char>,
    /// Maximum name length
    pub max_name_length: usize,
    /// Maximum description length
    pub max_description_length: usize,
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Is valid
    pub is_valid: bool,
    /// Validation errors
    pub errors: Vec<ValidationError>,
    /// Validation warnings
    pub warnings: Vec<ValidationWarning>,
    /// Validation score (0.0 to 1.0)
    pub score: f32,
    /// Validation timestamp
    pub timestamp: SystemTime,
}

/// Validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationError {
    /// File not found
    FileNotFound,
    /// File corrupted
    FileCorrupted,
    /// Version incompatible
    VersionIncompatible { current: String, required: String },
    /// Data missing
    DataMissing(String),
    /// Data invalid
    DataInvalid(String, String),
    /// File too large
    FileTooLarge { size: usize, max_size: usize },
    /// Play time too high
    PlayTimeTooHigh { time: f32, max_time: f32 },
    /// Character level too high
    CharacterLevelTooHigh { level: u32, max_level: u32 },
    /// Name too long
    NameTooLong { length: usize, max_length: usize },
    /// Name contains forbidden characters
    NameContainsForbiddenCharacters { characters: Vec<char> },
    /// Description too long
    DescriptionTooLong { length: usize, max_length: usize },
    /// Save slot not found
    SaveSlotNotFound(u32),
    /// Save slot corrupted
    SaveSlotCorrupted(u32),
    /// Checksum mismatch
    ChecksumMismatch,
    /// Serialization error
    SerializationError(String),
    /// Unknown error
    Unknown(String),
}

/// Validation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationWarning {
    /// Old save format
    OldSaveFormat { version: String },
    /// Unusual play time
    UnusualPlayTime { time: f32 },
    /// High character level
    HighCharacterLevel { level: u32 },
    /// Missing optional data
    MissingOptionalData(String),
    /// Deprecated feature used
    DeprecatedFeature(String),
    /// Performance warning
    PerformanceWarning(String),
}

impl SaveValidator {
    /// Create a new save validator
    pub fn new(current_version: String, min_supported_version: String) -> Self {
        Self {
            current_version,
            min_supported_version,
            rules: ValidationRules::default(),
        }
    }

    /// Validate a save slot
    pub fn validate_save_slot(&self, save_slot: &SaveSlot) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut score = 1.0;

        // Validate metadata
        self.validate_metadata(&save_slot.metadata, &mut errors, &mut warnings, &mut score);

        // Validate data
        self.validate_data(&save_slot.data, &mut errors, &mut warnings, &mut score);

        // Validate file size
        self.validate_file_size(save_slot, &mut errors, &mut warnings, &mut score);

        // Validate version compatibility
        self.validate_version(&save_slot.metadata.game_version, &mut errors, &mut warnings, &mut score);

        // Calculate final score
        score = score.max(0.0).min(1.0);

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            score,
            timestamp: SystemTime::now(),
        }
    }

    /// Validate save slot metadata
    fn validate_metadata(
        &self,
        metadata: &SaveSlotMetadata,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
        score: &mut f32,
    ) {
        // Check required fields
        if metadata.name.is_empty() {
            errors.push(ValidationError::DataMissing("name".to_string()));
            *score -= 0.2;
        }

        if metadata.player_name.is_empty() {
            errors.push(ValidationError::DataMissing("player_name".to_string()));
            *score -= 0.2;
        }

        if metadata.game_version.is_empty() {
            errors.push(ValidationError::DataMissing("game_version".to_string()));
            *score -= 0.1;
        }

        // Check name length
        if metadata.name.len() > self.rules.max_name_length {
            errors.push(ValidationError::NameTooLong {
                length: metadata.name.len(),
                max_length: self.rules.max_name_length,
            });
            *score -= 0.1;
        }

        // Check for forbidden characters
        let forbidden_chars: Vec<char> = metadata.name
            .chars()
            .filter(|c| self.rules.forbidden_characters.contains(c))
            .collect();

        if !forbidden_chars.is_empty() {
            errors.push(ValidationError::NameContainsForbiddenCharacters {
                characters: forbidden_chars,
            });
            *score -= 0.1;
        }

        // Check description length
        if let Some(description) = &metadata.description {
            if description.len() > self.rules.max_description_length {
                errors.push(ValidationError::DescriptionTooLong {
                    length: description.len(),
                    max_length: self.rules.max_description_length,
                });
                *score -= 0.05;
            }
        }

        // Check play time
        if metadata.play_time > self.rules.max_play_time {
            errors.push(ValidationError::PlayTimeTooHigh {
                time: metadata.play_time,
                max_time: self.rules.max_play_time,
            });
            *score -= 0.1;
        } else if metadata.play_time > self.rules.max_play_time * 0.8 {
            warnings.push(ValidationWarning::UnusualPlayTime {
                time: metadata.play_time,
            });
        }

        // Check character level
        if metadata.character_level > self.rules.max_character_level {
            errors.push(ValidationError::CharacterLevelTooHigh {
                level: metadata.character_level,
                max_level: self.rules.max_character_level,
            });
            *score -= 0.1;
        } else if metadata.character_level > (self.rules.max_character_level as f32 * 0.8) as u32 {
            warnings.push(ValidationWarning::HighCharacterLevel {
                level: metadata.character_level,
            });
        }

        // Check completion percentage
        if metadata.completion_percentage < 0.0 || metadata.completion_percentage > 100.0 {
            errors.push(ValidationError::DataInvalid(
                "completion_percentage".to_string(),
                format!("{}", metadata.completion_percentage),
            ));
            *score -= 0.1;
        }
    }

    /// Validate save slot data
    fn validate_data(
        &self,
        data: &SaveSlotData,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
        score: &mut f32,
    ) {
        // Validate game state
        if data.game_state.current_level.is_empty() {
            errors.push(ValidationError::DataMissing("current_level".to_string()));
            *score -= 0.2;
        }

        // Validate player data
        if data.player_data.character_name.is_empty() {
            errors.push(ValidationError::DataMissing("character_name".to_string()));
            *score -= 0.1;
        }

        if data.player_data.level == 0 {
            errors.push(ValidationError::DataInvalid(
                "character_level".to_string(),
                "0".to_string(),
            ));
            *score -= 0.1;
        }

        // Validate health values
        if data.player_data.health < 0.0 || data.player_data.health > data.player_data.max_health {
            errors.push(ValidationError::DataInvalid(
                "health".to_string(),
                format!("{}", data.player_data.health),
            ));
            *score -= 0.1;
        }

        if data.player_data.max_health <= 0.0 {
            errors.push(ValidationError::DataInvalid(
                "max_health".to_string(),
                format!("{}", data.player_data.max_health),
            ));
            *score -= 0.1;
        }

        // Validate mana values
        if data.player_data.mana < 0.0 || data.player_data.mana > data.player_data.max_mana {
            errors.push(ValidationError::DataInvalid(
                "mana".to_string(),
                format!("{}", data.player_data.mana),
            ));
            *score -= 0.05;
        }

        if data.player_data.max_mana < 0.0 {
            errors.push(ValidationError::DataInvalid(
                "max_mana".to_string(),
                format!("{}", data.player_data.max_mana),
            ));
            *score -= 0.05;
        }

        // Validate level data
        if data.level_data.level_id.is_empty() {
            errors.push(ValidationError::DataMissing("level_id".to_string()));
            *score -= 0.1;
        }

        if data.level_data.level_progress < 0.0 || data.level_data.level_progress > 1.0 {
            errors.push(ValidationError::DataInvalid(
                "level_progress".to_string(),
                format!("{}", data.level_data.level_progress),
            ));
            *score -= 0.05;
        }

        // Check for missing optional data
        if data.inventory_data.equipped_items.is_empty() {
            warnings.push(ValidationWarning::MissingOptionalData(
                "equipped_items".to_string(),
            ));
        }

        if data.achievement_data.unlocked_achievements.is_empty() {
            warnings.push(ValidationWarning::MissingOptionalData(
                "unlocked_achievements".to_string(),
            ));
        }
    }

    /// Validate file size
    fn validate_file_size(
        &self,
        save_slot: &SaveSlot,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
        score: &mut f32,
    ) {
        let size = save_slot.size_bytes();
        if size > self.rules.max_file_size {
            errors.push(ValidationError::FileTooLarge {
                size,
                max_size: self.rules.max_file_size,
            });
            *score -= 0.2;
        } else if size > (self.rules.max_file_size as f32 * 0.8) as usize {
            warnings.push(ValidationWarning::PerformanceWarning(
                "Large save file may impact performance".to_string(),
            ));
        }
    }

    /// Validate version compatibility
    fn validate_version(
        &self,
        version: &str,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
        score: &mut f32,
    ) {
        if version.is_empty() {
            errors.push(ValidationError::DataMissing("game_version".to_string()));
            *score -= 0.1;
            return;
        }

        // Simple version comparison (in real implementation, use proper semver)
        if version != self.current_version {
            if version < self.min_supported_version.as_str() {
                errors.push(ValidationError::VersionIncompatible {
                    current: version.to_string(),
                    required: self.min_supported_version.clone(),
                });
                *score -= 0.3;
            } else {
                warnings.push(ValidationWarning::OldSaveFormat {
                    version: version.to_string(),
                });
            }
        }
    }

    /// Calculate checksum for save slot
    pub fn calculate_checksum(&self, save_slot: &SaveSlot) -> u32 {
        // Simple checksum calculation (in real implementation, use proper hash)
        let mut checksum = 0u32;
        checksum = checksum.wrapping_add(save_slot.slot_number);
        checksum = checksum.wrapping_add(save_slot.metadata.name.len() as u32);
        checksum = checksum.wrapping_add(save_slot.metadata.player_name.len() as u32);
        checksum = checksum.wrapping_add(save_slot.metadata.character_level);
        checksum = checksum.wrapping_add(save_slot.data.player_data.level);
        checksum
    }

    /// Verify checksum
    pub fn verify_checksum(&self, save_slot: &SaveSlot, expected_checksum: u32) -> bool {
        self.calculate_checksum(save_slot) == expected_checksum
    }
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024, // 10 MB
            max_play_time: 1000.0 * 3600.0, // 1000 hours
            max_character_level: 100,
            required_fields: vec![
                "name".to_string(),
                "player_name".to_string(),
                "game_version".to_string(),
                "current_level".to_string(),
                "character_name".to_string(),
            ],
            forbidden_characters: vec!['<', '>', ':', '"', '|', '?', '*', '\\', '/'],
            max_name_length: 50,
            max_description_length: 200,
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::FileNotFound => write!(f, "Save file not found"),
            ValidationError::FileCorrupted => write!(f, "Save file is corrupted"),
            ValidationError::VersionIncompatible { current, required } => {
                write!(f, "Version {} is incompatible, requires {}", current, required)
            }
            ValidationError::DataMissing(field) => write!(f, "Required field '{}' is missing", field),
            ValidationError::DataInvalid(field, value) => {
                write!(f, "Field '{}' has invalid value: {}", field, value)
            }
            ValidationError::FileTooLarge { size, max_size } => {
                write!(f, "File size {} bytes exceeds maximum {} bytes", size, max_size)
            }
            ValidationError::PlayTimeTooHigh { time, max_time } => {
                write!(f, "Play time {} seconds exceeds maximum {} seconds", time, max_time)
            }
            ValidationError::CharacterLevelTooHigh { level, max_level } => {
                write!(f, "Character level {} exceeds maximum {}", level, max_level)
            }
            ValidationError::NameTooLong { length, max_length } => {
                write!(f, "Name length {} exceeds maximum {}", length, max_length)
            }
            ValidationError::NameContainsForbiddenCharacters { characters } => {
                write!(f, "Name contains forbidden characters: {:?}", characters)
            }
            ValidationError::DescriptionTooLong { length, max_length } => {
                write!(f, "Description length {} exceeds maximum {}", length, max_length)
            }
            ValidationError::SaveSlotNotFound(slot) => write!(f, "Save slot {} not found", slot),
            ValidationError::SaveSlotCorrupted(slot) => write!(f, "Save slot {} is corrupted", slot),
            ValidationError::ChecksumMismatch => write!(f, "Checksum mismatch"),
            ValidationError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            ValidationError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationWarning::OldSaveFormat { version } => {
                write!(f, "Save file is from older version: {}", version)
            }
            ValidationWarning::UnusualPlayTime { time } => {
                write!(f, "Unusually high play time: {} seconds", time)
            }
            ValidationWarning::HighCharacterLevel { level } => {
                write!(f, "High character level: {}", level)
            }
            ValidationWarning::MissingOptionalData(field) => {
                write!(f, "Missing optional data: {}", field)
            }
            ValidationWarning::DeprecatedFeature(feature) => {
                write!(f, "Deprecated feature used: {}", feature)
            }
            ValidationWarning::PerformanceWarning(msg) => write!(f, "Performance warning: {}", msg),
        }
    }
}
