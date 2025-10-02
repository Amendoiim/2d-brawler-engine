//! Sound Categories
//! 
//! This module provides sound categorization for the sound test system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sound category
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoundCategory {
    /// Sound effects
    SFX,
    /// Background music
    BGM,
    /// Voice/voiceover
    Voice,
    /// Ambient sounds
    Ambient,
    /// UI sounds
    UI,
    /// Footstep sounds
    Footstep,
    /// Weapon sounds
    Weapon,
    /// Environmental sounds
    Environmental,
    /// Character sounds
    Character,
    /// Menu sounds
    Menu,
    /// Notification sounds
    Notification,
    /// Custom category
    Custom(String),
}

/// Sound category info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundCategoryInfo {
    /// Category name
    pub name: String,
    /// Category description
    pub description: String,
    /// Category icon
    pub icon: String,
    /// Category color (RGBA)
    pub color: (f32, f32, f32, f32),
    /// Default volume
    pub default_volume: f32,
    /// Default pitch
    pub default_pitch: f32,
    /// Default pan
    pub default_pan: f32,
    /// Enable looping
    pub enable_looping: bool,
    /// Enable pitch control
    pub enable_pitch_control: bool,
    /// Enable volume control
    pub enable_volume_control: bool,
    /// Enable pan control
    pub enable_pan_control: bool,
    /// Enable effects
    pub enable_effects: bool,
    /// Enable spatial audio
    pub enable_spatial_audio: bool,
    /// Sound count
    pub sound_count: usize,
    /// Total duration
    pub total_duration: f32,
}

/// Sound category manager
#[derive(Debug, Clone)]
pub struct SoundCategoryManager {
    /// Category info
    pub categories: HashMap<SoundCategory, SoundCategoryInfo>,
    /// Current category
    pub current_category: SoundCategory,
    /// Category order
    pub category_order: Vec<SoundCategory>,
}

impl SoundCategoryManager {
    /// Create a new sound category manager
    pub fn new() -> Self {
        let mut manager = Self {
            categories: HashMap::new(),
            current_category: SoundCategory::SFX,
            category_order: Vec::new(),
        };

        // Initialize default categories
        manager.initialize_default_categories();
        manager
    }

    /// Initialize default categories
    fn initialize_default_categories(&mut self) {
        // SFX category
        self.add_category(SoundCategory::SFX, SoundCategoryInfo {
            name: "Sound Effects".to_string(),
            description: "Game sound effects and audio cues".to_string(),
            icon: "sfx".to_string(),
            color: (1.0, 0.8, 0.0, 1.0), // Orange
            default_volume: 0.8,
            default_pitch: 1.0,
            default_pan: 0.0,
            enable_looping: false,
            enable_pitch_control: true,
            enable_volume_control: true,
            enable_pan_control: true,
            enable_effects: true,
            enable_spatial_audio: true,
            sound_count: 0,
            total_duration: 0.0,
        });

        // BGM category
        self.add_category(SoundCategory::BGM, SoundCategoryInfo {
            name: "Background Music".to_string(),
            description: "Background music and soundtrack".to_string(),
            icon: "bgm".to_string(),
            color: (0.0, 0.8, 1.0, 1.0), // Cyan
            default_volume: 0.7,
            default_pitch: 1.0,
            default_pan: 0.0,
            enable_looping: true,
            enable_pitch_control: false,
            enable_volume_control: true,
            enable_pan_control: false,
            enable_effects: true,
            enable_spatial_audio: false,
            sound_count: 0,
            total_duration: 0.0,
        });

        // Voice category
        self.add_category(SoundCategory::Voice, SoundCategoryInfo {
            name: "Voice".to_string(),
            description: "Character voices and dialogue".to_string(),
            icon: "voice".to_string(),
            color: (1.0, 0.0, 0.8, 1.0), // Magenta
            default_volume: 0.9,
            default_pitch: 1.0,
            default_pan: 0.0,
            enable_looping: false,
            enable_pitch_control: false,
            enable_volume_control: true,
            enable_pan_control: false,
            enable_effects: false,
            enable_spatial_audio: true,
            sound_count: 0,
            total_duration: 0.0,
        });

        // Ambient category
        self.add_category(SoundCategory::Ambient, SoundCategoryInfo {
            name: "Ambient".to_string(),
            description: "Ambient sounds and atmosphere".to_string(),
            icon: "ambient".to_string(),
            color: (0.0, 1.0, 0.0, 1.0), // Green
            default_volume: 0.6,
            default_pitch: 1.0,
            default_pan: 0.0,
            enable_looping: true,
            enable_pitch_control: true,
            enable_volume_control: true,
            enable_pan_control: true,
            enable_effects: true,
            enable_spatial_audio: true,
            sound_count: 0,
            total_duration: 0.0,
        });

        // UI category
        self.add_category(SoundCategory::UI, SoundCategoryInfo {
            name: "UI Sounds".to_string(),
            description: "User interface sounds and feedback".to_string(),
            icon: "ui".to_string(),
            color: (0.8, 0.8, 0.8, 1.0), // Gray
            default_volume: 0.7,
            default_pitch: 1.0,
            default_pan: 0.0,
            enable_looping: false,
            enable_pitch_control: true,
            enable_volume_control: true,
            enable_pan_control: false,
            enable_effects: false,
            enable_spatial_audio: false,
            sound_count: 0,
            total_duration: 0.0,
        });

        // Footstep category
        self.add_category(SoundCategory::Footstep, SoundCategoryInfo {
            name: "Footsteps".to_string(),
            description: "Character footstep sounds".to_string(),
            icon: "footstep".to_string(),
            color: (0.6, 0.4, 0.2, 1.0), // Brown
            default_volume: 0.8,
            default_pitch: 1.0,
            default_pan: 0.0,
            enable_looping: false,
            enable_pitch_control: true,
            enable_volume_control: true,
            enable_pan_control: true,
            enable_effects: true,
            enable_spatial_audio: true,
            sound_count: 0,
            total_duration: 0.0,
        });

        // Weapon category
        self.add_category(SoundCategory::Weapon, SoundCategoryInfo {
            name: "Weapons".to_string(),
            description: "Weapon sounds and combat audio".to_string(),
            icon: "weapon".to_string(),
            color: (1.0, 0.0, 0.0, 1.0), // Red
            default_volume: 0.9,
            default_pitch: 1.0,
            default_pan: 0.0,
            enable_looping: false,
            enable_pitch_control: true,
            enable_volume_control: true,
            enable_pan_control: true,
            enable_effects: true,
            enable_spatial_audio: true,
            sound_count: 0,
            total_duration: 0.0,
        });

        // Environmental category
        self.add_category(SoundCategory::Environmental, SoundCategoryInfo {
            name: "Environmental".to_string(),
            description: "Environmental sounds and effects".to_string(),
            icon: "environmental".to_string(),
            color: (0.0, 0.6, 0.0, 1.0), // Dark Green
            default_volume: 0.7,
            default_pitch: 1.0,
            default_pan: 0.0,
            enable_looping: true,
            enable_pitch_control: true,
            enable_volume_control: true,
            enable_pan_control: true,
            enable_effects: true,
            enable_spatial_audio: true,
            sound_count: 0,
            total_duration: 0.0,
        });

        // Character category
        self.add_category(SoundCategory::Character, SoundCategoryInfo {
            name: "Character".to_string(),
            description: "Character-specific sounds and reactions".to_string(),
            icon: "character".to_string(),
            color: (0.8, 0.0, 0.8, 1.0), // Purple
            default_volume: 0.8,
            default_pitch: 1.0,
            default_pan: 0.0,
            enable_looping: false,
            enable_pitch_control: true,
            enable_volume_control: true,
            enable_pan_control: true,
            enable_effects: true,
            enable_spatial_audio: true,
            sound_count: 0,
            total_duration: 0.0,
        });

        // Menu category
        self.add_category(SoundCategory::Menu, SoundCategoryInfo {
            name: "Menu".to_string(),
            description: "Menu and navigation sounds".to_string(),
            icon: "menu".to_string(),
            color: (0.0, 0.0, 1.0, 1.0), // Blue
            default_volume: 0.7,
            default_pitch: 1.0,
            default_pan: 0.0,
            enable_looping: false,
            enable_pitch_control: true,
            enable_volume_control: true,
            enable_pan_control: false,
            enable_effects: false,
            enable_spatial_audio: false,
            sound_count: 0,
            total_duration: 0.0,
        });

        // Notification category
        self.add_category(SoundCategory::Notification, SoundCategoryInfo {
            name: "Notifications".to_string(),
            description: "Notification and alert sounds".to_string(),
            icon: "notification".to_string(),
            color: (1.0, 1.0, 0.0, 1.0), // Yellow
            default_volume: 0.8,
            default_pitch: 1.0,
            default_pan: 0.0,
            enable_looping: false,
            enable_pitch_control: true,
            enable_volume_control: true,
            enable_pan_control: false,
            enable_effects: false,
            enable_spatial_audio: false,
            sound_count: 0,
            total_duration: 0.0,
        });

        // Set category order
        self.category_order = vec![
            SoundCategory::SFX,
            SoundCategory::BGM,
            SoundCategory::Voice,
            SoundCategory::Ambient,
            SoundCategory::UI,
            SoundCategory::Footstep,
            SoundCategory::Weapon,
            SoundCategory::Environmental,
            SoundCategory::Character,
            SoundCategory::Menu,
            SoundCategory::Notification,
        ];
    }

    /// Add category
    pub fn add_category(&mut self, category: SoundCategory, info: SoundCategoryInfo) {
        self.categories.insert(category, info);
    }

    /// Remove category
    pub fn remove_category(&mut self, category: &SoundCategory) {
        self.categories.remove(category);
        self.category_order.retain(|c| c != category);
    }

    /// Get category info
    pub fn get_category_info(&self, category: &SoundCategory) -> Option<&SoundCategoryInfo> {
        self.categories.get(category)
    }

    /// Get category info mutably
    pub fn get_category_info_mut(&mut self, category: &SoundCategory) -> Option<&mut SoundCategoryInfo> {
        self.categories.get_mut(category)
    }

    /// Set current category
    pub fn set_current_category(&mut self, category: SoundCategory) {
        self.current_category = category;
    }

    /// Get current category
    pub fn get_current_category(&self) -> &SoundCategory {
        &self.current_category
    }

    /// Get category order
    pub fn get_category_order(&self) -> &Vec<SoundCategory> {
        &self.category_order
    }

    /// Get all categories
    pub fn get_all_categories(&self) -> Vec<&SoundCategory> {
        self.category_order.iter().collect()
    }

    /// Update sound count for category
    pub fn update_sound_count(&mut self, category: &SoundCategory, count: usize) {
        if let Some(info) = self.categories.get_mut(category) {
            info.sound_count = count;
        }
    }

    /// Update total duration for category
    pub fn update_total_duration(&mut self, category: &SoundCategory, duration: f32) {
        if let Some(info) = self.categories.get_mut(category) {
            info.total_duration = duration;
        }
    }

    /// Get category by name
    pub fn get_category_by_name(&self, name: &str) -> Option<&SoundCategory> {
        self.categories.iter()
            .find(|(_, info)| info.name == name)
            .map(|(category, _)| category)
    }

    /// Get category by icon
    pub fn get_category_by_icon(&self, icon: &str) -> Option<&SoundCategory> {
        self.categories.iter()
            .find(|(_, info)| info.icon == icon)
            .map(|(category, _)| category)
    }
}

impl Default for SoundCategoryManager {
    fn default() -> Self {
        Self::new()
    }
}
