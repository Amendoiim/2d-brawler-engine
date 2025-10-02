//! Save/Load System
//! 
//! This module provides comprehensive save and load functionality for the game,
//! including multiple save slots, auto-save, and save file validation.

pub mod manager;
pub mod slot;
pub mod validation;
pub mod serialization;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Re-export main components
pub use manager::SaveManager;
pub use slot::{SaveSlot, SaveSlotData, SaveSlotMetadata};
pub use validation::{SaveValidator, ValidationResult, ValidationError};
pub use serialization::{SaveSerializer, LoadSerializer, SerializationError};

/// Save system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveConfig {
    /// Maximum number of save slots
    pub max_save_slots: u32,
    /// Auto-save enabled
    pub auto_save_enabled: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval: f32,
    /// Save file directory
    pub save_directory: String,
    /// Backup enabled
    pub backup_enabled: bool,
    /// Maximum number of backups per slot
    pub max_backups: u32,
}

impl Default for SaveConfig {
    fn default() -> Self {
        Self {
            max_save_slots: 10,
            auto_save_enabled: true,
            auto_save_interval: 300.0, // 5 minutes
            save_directory: "saves".to_string(),
            backup_enabled: true,
            max_backups: 5,
        }
    }
}

/// Save system events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SaveEvent {
    /// Save completed successfully
    SaveCompleted(String, SystemTime),
    /// Load completed successfully
    LoadCompleted(String, SystemTime),
    /// Auto-save triggered
    AutoSaveTriggered(SystemTime),
    /// Save slot created
    SaveSlotCreated(u32, String),
    /// Save slot deleted
    SaveSlotDeleted(u32),
    /// Save validation failed
    ValidationFailed(String, ValidationError),
    /// Save system error
    SaveError(String),
}

/// Save system statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveStats {
    /// Total saves performed
    pub total_saves: u32,
    /// Total loads performed
    pub total_loads: u32,
    /// Auto-saves performed
    pub auto_saves: u32,
    /// Manual saves performed
    pub manual_saves: u32,
    /// Failed saves
    pub failed_saves: u32,
    /// Failed loads
    pub failed_loads: u32,
    /// Average save time in milliseconds
    pub average_save_time: f32,
    /// Average load time in milliseconds
    pub average_load_time: f32,
    /// Last save time
    pub last_save_time: Option<SystemTime>,
    /// Last load time
    pub last_load_time: Option<SystemTime>,
}

impl Default for SaveStats {
    fn default() -> Self {
        Self {
            total_saves: 0,
            total_loads: 0,
            auto_saves: 0,
            manual_saves: 0,
            failed_saves: 0,
            failed_loads: 0,
            average_save_time: 0.0,
            average_load_time: 0.0,
            last_save_time: None,
            last_load_time: None,
        }
    }
}

impl SaveStats {
    /// Add time to the stats
    pub fn add_time(&mut self, delta_time: f32) {
        // This method is called during update but doesn't need to do anything specific
        // as the stats are updated during actual save/load operations
    }
}

/// Save system result type
pub type SaveResult<T> = Result<T, SaveError>;

/// Save system error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SaveError {
    /// File I/O error
    IoError(String),
    /// Serialization error
    SerializationError(String),
    /// Validation error
    ValidationError(ValidationError),
    /// Save slot not found
    SaveSlotNotFound(u32),
    /// Save slot already exists
    SaveSlotExists(u32),
    /// Invalid save data
    InvalidSaveData(String),
    /// Save directory not found
    SaveDirectoryNotFound,
    /// Insufficient disk space
    InsufficientSpace,
    /// Save system not initialized
    NotInitialized,
    /// Auto-save disabled
    AutoSaveDisabled,
    /// Backup failed
    BackupFailed(String),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::IoError(msg) => write!(f, "I/O Error: {}", msg),
            SaveError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
            SaveError::ValidationError(err) => write!(f, "Validation Error: {}", err),
            SaveError::SaveSlotNotFound(slot) => write!(f, "Save slot {} not found", slot),
            SaveError::SaveSlotExists(slot) => write!(f, "Save slot {} already exists", slot),
            SaveError::InvalidSaveData(msg) => write!(f, "Invalid save data: {}", msg),
            SaveError::SaveDirectoryNotFound => write!(f, "Save directory not found"),
            SaveError::InsufficientSpace => write!(f, "Insufficient disk space"),
            SaveError::NotInitialized => write!(f, "Save system not initialized"),
            SaveError::AutoSaveDisabled => write!(f, "Auto-save is disabled"),
            SaveError::BackupFailed(msg) => write!(f, "Backup failed: {}", msg),
        }
    }
}

impl std::error::Error for SaveError {}
