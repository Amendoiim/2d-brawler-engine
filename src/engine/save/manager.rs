//! Save Manager
//! 
//! This module provides the main save/load management system, including
//! multiple save slots, auto-save, and save file operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, Duration};
use std::fs;

use super::{
    SaveConfig, SaveEvent, SaveStats, SaveResult, SaveError,
    SaveSlot, SaveSlotData, SaveSlotMetadata,
    SaveValidator, ValidationResult,
    SaveSerializer, LoadSerializer, SerializationError,
};

/// Save manager
pub struct SaveManager {
    /// Save configuration
    pub config: SaveConfig,
    /// Save slots
    pub save_slots: HashMap<u32, SaveSlot>,
    /// Auto-save slot
    pub auto_save_slot: Option<SaveSlot>,
    /// Save validator
    pub validator: SaveValidator,
    /// Save serializer
    pub serializer: SaveSerializer,
    /// Load serializer
    pub loader: LoadSerializer,
    /// Save statistics
    pub stats: SaveStats,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&SaveEvent) + Send + Sync>>,
    /// Last auto-save time
    pub last_auto_save: Option<SystemTime>,
    /// Save directory path
    pub save_directory: PathBuf,
}

impl SaveManager {
    /// Create a new save manager
    pub fn new(config: SaveConfig) -> Self {
        let save_directory = PathBuf::from(&config.save_directory);
        
        Self {
            config,
            save_slots: HashMap::new(),
            auto_save_slot: None,
            validator: SaveValidator::new(
                "1.0.0".to_string(),
                "1.0.0".to_string(),
            ),
            serializer: SaveSerializer::new(),
            loader: LoadSerializer::new(),
            stats: SaveStats::default(),
            event_handlers: Vec::new(),
            last_auto_save: None,
            save_directory,
        }
    }

    /// Initialize the save manager
    pub fn initialize(&mut self) -> SaveResult<()> {
        // Create save directory if it doesn't exist
        if !self.save_directory.exists() {
            fs::create_dir_all(&self.save_directory)
                .map_err(|e| SaveError::IoError(e.to_string()))?;
        }

        // Load existing save slots
        self.load_save_slots()?;

        // Initialize auto-save if enabled
        if self.config.auto_save_enabled {
            self.initialize_auto_save()?;
        }

        Ok(())
    }

    /// Update the save manager
    pub fn update(&mut self, delta_time: f32) -> SaveResult<()> {
        // Update statistics
        self.stats.add_time(delta_time);

        // Check for auto-save
        if self.config.auto_save_enabled {
            self.check_auto_save()?;
        }

        Ok(())
    }

    /// Save game to a specific slot
    pub fn save_game(&mut self, slot_number: u32, metadata: SaveSlotMetadata, data: SaveSlotData) -> SaveResult<()> {
        let start_time = SystemTime::now();

        // Create save slot
        let save_slot = SaveSlot::new(slot_number, metadata, data);

        // Validate save slot
        let validation_result = self.validator.validate_save_slot(&save_slot);
        if !validation_result.is_valid {
            self.stats.failed_saves += 1;
            self.emit_event(SaveEvent::ValidationFailed(
                format!("Slot {}", slot_number),
                validation_result.errors[0].clone(),
            ));
            return Err(SaveError::ValidationError(validation_result.errors[0].clone()));
        }

        // Serialize save slot
        let serialization_result = self.serializer.serialize_save_slot(&save_slot);
        if !serialization_result.success {
            self.stats.failed_saves += 1;
            self.emit_event(SaveEvent::SaveError(
                serialization_result.error.unwrap_or("Unknown error".to_string()),
            ));
            return Err(SaveError::SerializationError(
                serialization_result.error.unwrap_or("Unknown error".to_string()),
            ));
        }

        // Save to file
        let file_path = self.get_save_file_path(slot_number);
        fs::write(&file_path, &serialization_result.data)
            .map_err(|e| SaveError::IoError(e.to_string()))?;

        // Update save slots
        self.save_slots.insert(slot_number, save_slot);

        // Update statistics
        self.stats.total_saves += 1;
        self.stats.manual_saves += 1;
        self.stats.last_save_time = Some(SystemTime::now());
        self.update_average_save_time(start_time);

        // Emit event
        self.emit_event(SaveEvent::SaveCompleted(
            format!("Slot {}", slot_number),
            SystemTime::now(),
        ));

        Ok(())
    }

    /// Load game from a specific slot
    pub fn load_game(&mut self, slot_number: u32) -> SaveResult<SaveSlot> {
        let start_time = SystemTime::now();

        // Check if slot exists
        if let Some(save_slot) = self.save_slots.get(&slot_number) {
            // Update statistics
            self.stats.total_loads += 1;
            self.stats.last_load_time = Some(SystemTime::now());
            self.update_average_load_time(start_time);

            // Emit event
            self.emit_event(SaveEvent::LoadCompleted(
                format!("Slot {}", slot_number),
                SystemTime::now(),
            ));

            return Ok(save_slot.clone());
        }

        // Try to load from file
        let file_path = self.get_save_file_path(slot_number);
        if !file_path.exists() {
            self.stats.failed_loads += 1;
            return Err(SaveError::SaveSlotNotFound(slot_number));
        }

        // Read file
        let file_data = fs::read(&file_path)
            .map_err(|e| SaveError::IoError(e.to_string()))?;

        // Deserialize save slot
        let save_slot = self.loader.deserialize_save_slot(&file_data)
            .map_err(|e| SaveError::SerializationError(e.to_string()))?;

        // Validate save slot
        let validation_result = self.validator.validate_save_slot(&save_slot);
        if !validation_result.is_valid {
            self.stats.failed_loads += 1;
            self.emit_event(SaveEvent::ValidationFailed(
                format!("Slot {}", slot_number),
                validation_result.errors[0].clone(),
            ));
            return Err(SaveError::ValidationError(validation_result.errors[0].clone()));
        }

        // Update save slots
        self.save_slots.insert(slot_number, save_slot.clone());

        // Update statistics
        self.stats.total_loads += 1;
        self.stats.last_load_time = Some(SystemTime::now());
        self.update_average_load_time(start_time);

        // Emit event
        self.emit_event(SaveEvent::LoadCompleted(
            format!("Slot {}", slot_number),
            SystemTime::now(),
        ));

        Ok(save_slot)
    }

    /// Auto-save the game
    pub fn auto_save(&mut self, metadata: SaveSlotMetadata, data: SaveSlotData) -> SaveResult<()> {
        if !self.config.auto_save_enabled {
            return Err(SaveError::AutoSaveDisabled);
        }

        let start_time = SystemTime::now();

        // Create auto-save slot
        let auto_save_slot = SaveSlot::new_auto_save(999, metadata, data);

        // Validate save slot
        let validation_result = self.validator.validate_save_slot(&auto_save_slot);
        if !validation_result.is_valid {
            self.stats.failed_saves += 1;
            self.emit_event(SaveEvent::ValidationFailed(
                "Auto-save".to_string(),
                validation_result.errors[0].clone(),
            ));
            return Err(SaveError::ValidationError(validation_result.errors[0].clone()));
        }

        // Serialize save slot
        let serialization_result = self.serializer.serialize_save_slot(&auto_save_slot);
        if !serialization_result.success {
            self.stats.failed_saves += 1;
            self.emit_event(SaveEvent::SaveError(
                serialization_result.error.unwrap_or("Unknown error".to_string()),
            ));
            return Err(SaveError::SerializationError(
                serialization_result.error.unwrap_or("Unknown error".to_string()),
            ));
        }

        // Save to file
        let file_path = self.get_auto_save_file_path();
        fs::write(&file_path, &serialization_result.data)
            .map_err(|e| SaveError::IoError(e.to_string()))?;

        // Update auto-save slot
        self.auto_save_slot = Some(auto_save_slot);

        // Update statistics
        self.stats.total_saves += 1;
        self.stats.auto_saves += 1;
        self.stats.last_save_time = Some(SystemTime::now());
        self.update_average_save_time(start_time);

        // Update last auto-save time
        self.last_auto_save = Some(SystemTime::now());

        // Emit event
        self.emit_event(SaveEvent::AutoSaveTriggered(SystemTime::now()));

        Ok(())
    }

    /// Delete a save slot
    pub fn delete_save_slot(&mut self, slot_number: u32) -> SaveResult<()> {
        // Check if slot exists
        if !self.save_slots.contains_key(&slot_number) {
            return Err(SaveError::SaveSlotNotFound(slot_number));
        }

        // Delete file
        let file_path = self.get_save_file_path(slot_number);
        if file_path.exists() {
            fs::remove_file(&file_path)
                .map_err(|e| SaveError::IoError(e.to_string()))?;
        }

        // Remove from save slots
        self.save_slots.remove(&slot_number);

        // Emit event
        self.emit_event(SaveEvent::SaveSlotDeleted(slot_number));

        Ok(())
    }

    /// Get save slot metadata
    pub fn get_save_slot_metadata(&self, slot_number: u32) -> Option<&SaveSlotMetadata> {
        self.save_slots.get(&slot_number).map(|slot| &slot.metadata)
    }

    /// Get all save slots
    pub fn get_all_save_slots(&self) -> &HashMap<u32, SaveSlot> {
        &self.save_slots
    }

    /// Get available save slots
    pub fn get_available_save_slots(&self) -> Vec<u32> {
        let mut available = Vec::new();
        for i in 0..self.config.max_save_slots {
            if !self.save_slots.contains_key(&i) {
                available.push(i);
            }
        }
        available
    }

    /// Check if save slot exists
    pub fn save_slot_exists(&self, slot_number: u32) -> bool {
        self.save_slots.contains_key(&slot_number)
    }

    /// Get save statistics
    pub fn get_stats(&self) -> &SaveStats {
        &self.stats
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&SaveEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Load existing save slots
    fn load_save_slots(&mut self) -> SaveResult<()> {
        if !self.save_directory.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.save_directory)
            .map_err(|e| SaveError::IoError(e.to_string()))?
        {
            let entry = entry.map_err(|e| SaveError::IoError(e.to_string()))?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("save") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(slot_number) = file_name.parse::<u32>() {
                        // Try to load the save slot
                        match self.load_game(slot_number) {
                            Ok(_) => {
                                // Save slot loaded successfully
                            }
                            Err(e) => {
                                // Log error but continue loading other slots
                                eprintln!("Failed to load save slot {}: {}", slot_number, e);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Initialize auto-save
    fn initialize_auto_save(&mut self) -> SaveResult<()> {
        // Check if auto-save file exists
        let auto_save_path = self.get_auto_save_file_path();
        if auto_save_path.exists() {
            // Load existing auto-save
            let file_data = fs::read(&auto_save_path)
                .map_err(|e| SaveError::IoError(e.to_string()))?;

            match self.loader.deserialize_save_slot(&file_data) {
                Ok(save_slot) => {
                    self.auto_save_slot = Some(save_slot);
                }
                Err(e) => {
                    eprintln!("Failed to load auto-save: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Check for auto-save
    fn check_auto_save(&mut self) -> SaveResult<()> {
        if let Some(last_auto_save) = self.last_auto_save {
            let elapsed = SystemTime::now()
                .duration_since(last_auto_save)
                .unwrap_or_default();

            if elapsed >= Duration::from_secs_f32(self.config.auto_save_interval) {
                // Auto-save should be triggered
                // This would be called by the game engine with current game state
                // For now, we just update the last auto-save time
                self.last_auto_save = Some(SystemTime::now());
            }
        } else {
            // First auto-save
            self.last_auto_save = Some(SystemTime::now());
        }

        Ok(())
    }

    /// Get save file path
    fn get_save_file_path(&self, slot_number: u32) -> PathBuf {
        self.save_directory.join(format!("{}.save", slot_number))
    }

    /// Get auto-save file path
    fn get_auto_save_file_path(&self) -> PathBuf {
        self.save_directory.join("autosave.save")
    }

    /// Update average save time
    fn update_average_save_time(&mut self, start_time: SystemTime) {
        let elapsed = start_time.elapsed().unwrap_or_default().as_millis() as f32;
        self.stats.average_save_time = (self.stats.average_save_time * (self.stats.total_saves - 1) as f32 + elapsed) / self.stats.total_saves as f32;
    }

    /// Update average load time
    fn update_average_load_time(&mut self, start_time: SystemTime) {
        let elapsed = start_time.elapsed().unwrap_or_default().as_millis() as f32;
        self.stats.average_load_time = (self.stats.average_load_time * (self.stats.total_loads - 1) as f32 + elapsed) / self.stats.total_loads as f32;
    }

    /// Emit save event
    fn emit_event(&self, event: SaveEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for SaveManager {
    fn default() -> Self {
        Self::new(SaveConfig::default())
    }
}
