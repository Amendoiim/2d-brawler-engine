//! Save Slot Management
//! 
//! This module handles individual save slots, including metadata, data storage,
//! and slot management operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Individual save slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSlot {
    /// Slot number (0-based)
    pub slot_number: u32,
    /// Slot metadata
    pub metadata: SaveSlotMetadata,
    /// Save data
    pub data: SaveSlotData,
    /// Creation time
    pub created_at: SystemTime,
    /// Last modified time
    pub last_modified: SystemTime,
    /// Is auto-save slot
    pub is_auto_save: bool,
    /// Is backup slot
    pub is_backup: bool,
    /// Backup number (if applicable)
    pub backup_number: Option<u32>,
}

/// Save slot metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSlotMetadata {
    /// Save name/title
    pub name: String,
    /// Player name
    pub player_name: String,
    /// Game version when saved
    pub game_version: String,
    /// Save description
    pub description: Option<String>,
    /// Thumbnail data (base64 encoded)
    pub thumbnail: Option<String>,
    /// Play time in seconds
    pub play_time: f32,
    /// Level/area name
    pub level_name: String,
    /// Character level
    pub character_level: u32,
    /// Character class
    pub character_class: String,
    /// Difficulty setting
    pub difficulty: String,
    /// Completion percentage
    pub completion_percentage: f32,
    /// Custom tags
    pub tags: Vec<String>,
}

/// Save slot data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSlotData {
    /// Game state data
    pub game_state: GameStateData,
    /// Player data
    pub player_data: PlayerData,
    /// Level data
    pub level_data: LevelData,
    /// Inventory data
    pub inventory_data: InventoryData,
    /// Settings data
    pub settings_data: SettingsData,
    /// Achievement data
    pub achievement_data: AchievementData,
    /// Custom data
    pub custom_data: HashMap<String, serde_json::Value>,
}

/// Game state data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateData {
    /// Current level ID
    pub current_level: String,
    /// Current checkpoint
    pub current_checkpoint: String,
    /// Game mode
    pub game_mode: String,
    /// Game phase
    pub game_phase: String,
    /// Time of day
    pub time_of_day: f32,
    /// Weather conditions
    pub weather: String,
    /// Active events
    pub active_events: Vec<String>,
    /// Completed objectives
    pub completed_objectives: Vec<String>,
    /// Active quests
    pub active_quests: Vec<String>,
}

/// Player data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    /// Player ID
    pub player_id: String,
    /// Character name
    pub character_name: String,
    /// Character class
    pub character_class: String,
    /// Character level
    pub level: u32,
    /// Experience points
    pub experience: u32,
    /// Health points
    pub health: f32,
    /// Maximum health
    pub max_health: f32,
    /// Mana/energy points
    pub mana: f32,
    /// Maximum mana
    pub max_mana: f32,
    /// Position (x, y)
    pub position: (f32, f32),
    /// Direction facing
    pub direction: f32,
    /// Stats
    pub stats: PlayerStats,
    /// Skills
    pub skills: HashMap<String, u32>,
    /// Traits
    pub traits: Vec<String>,
    /// Status effects
    pub status_effects: Vec<StatusEffect>,
}

/// Player statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    /// Strength
    pub strength: u32,
    /// Dexterity
    pub dexterity: u32,
    /// Intelligence
    pub intelligence: u32,
    /// Constitution
    pub constitution: u32,
    /// Charisma
    pub charisma: u32,
    /// Luck
    pub luck: u32,
}

/// Status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    /// Effect name
    pub name: String,
    /// Effect type
    pub effect_type: String,
    /// Duration in seconds
    pub duration: f32,
    /// Intensity/level
    pub intensity: f32,
    /// Source
    pub source: String,
}

/// Level data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    /// Level ID
    pub level_id: String,
    /// Level name
    pub level_name: String,
    /// Level type
    pub level_type: String,
    /// Level seed
    pub level_seed: u64,
    /// Level progress
    pub level_progress: f32,
    /// Checkpoints reached
    pub checkpoints: Vec<String>,
    /// Secrets found
    pub secrets_found: Vec<String>,
    /// Enemies defeated
    pub enemies_defeated: u32,
    /// Items collected
    pub items_collected: u32,
    /// Time spent in level
    pub time_spent: f32,
}

/// Inventory data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryData {
    /// Equipped items
    pub equipped_items: HashMap<String, ItemData>,
    /// Inventory items
    pub inventory_items: Vec<ItemData>,
    /// Currency
    pub currency: HashMap<String, u32>,
    /// Key items
    pub key_items: Vec<String>,
    /// Consumables
    pub consumables: HashMap<String, u32>,
}

/// Item data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    /// Item ID
    pub item_id: String,
    /// Item name
    pub name: String,
    /// Item type
    pub item_type: String,
    /// Item rarity
    pub rarity: String,
    /// Item level
    pub level: u32,
    /// Item quantity
    pub quantity: u32,
    /// Item properties
    pub properties: HashMap<String, serde_json::Value>,
    /// Item durability
    pub durability: Option<f32>,
    /// Item enchantments
    pub enchantments: Vec<String>,
}

/// Settings data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsData {
    /// Graphics settings
    pub graphics: GraphicsSettings,
    /// Audio settings
    pub audio: AudioSettings,
    /// Control settings
    pub controls: ControlSettings,
    /// Gameplay settings
    pub gameplay: GameplaySettings,
}

/// Graphics settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsSettings {
    /// Resolution width
    pub resolution_width: u32,
    /// Resolution height
    pub resolution_height: u32,
    /// Fullscreen
    pub fullscreen: bool,
    /// VSync
    pub vsync: bool,
    /// Quality level
    pub quality_level: String,
    /// Anti-aliasing
    pub anti_aliasing: bool,
    /// Shadows
    pub shadows: bool,
    /// Particle effects
    pub particle_effects: bool,
}

/// Audio settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    /// Master volume
    pub master_volume: f32,
    /// Music volume
    pub music_volume: f32,
    /// SFX volume
    pub sfx_volume: f32,
    /// Voice volume
    pub voice_volume: f32,
    /// Ambient volume
    pub ambient_volume: f32,
    /// Audio device
    pub audio_device: String,
    /// Audio quality
    pub audio_quality: String,
}

/// Control settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlSettings {
    /// Key bindings
    pub key_bindings: HashMap<String, String>,
    /// Mouse sensitivity
    pub mouse_sensitivity: f32,
    /// Invert mouse Y
    pub invert_mouse_y: bool,
    /// Controller enabled
    pub controller_enabled: bool,
    /// Controller sensitivity
    pub controller_sensitivity: f32,
}

/// Gameplay settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplaySettings {
    /// Difficulty level
    pub difficulty: String,
    /// Auto-save enabled
    pub auto_save_enabled: bool,
    /// Auto-save interval
    pub auto_save_interval: f32,
    /// Subtitles enabled
    pub subtitles_enabled: bool,
    /// Language
    pub language: String,
    /// Tutorial enabled
    pub tutorial_enabled: bool,
}

/// Achievement data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementData {
    /// Unlocked achievements
    pub unlocked_achievements: Vec<String>,
    /// Achievement progress
    pub achievement_progress: HashMap<String, f32>,
    /// Achievement statistics
    pub achievement_stats: HashMap<String, u32>,
    /// Achievement rewards claimed
    pub rewards_claimed: Vec<String>,
}

impl SaveSlot {
    /// Create a new save slot
    pub fn new(slot_number: u32, metadata: SaveSlotMetadata, data: SaveSlotData) -> Self {
        let now = SystemTime::now();
        Self {
            slot_number,
            metadata,
            data,
            created_at: now,
            last_modified: now,
            is_auto_save: false,
            is_backup: false,
            backup_number: None,
        }
    }

    /// Create an auto-save slot
    pub fn new_auto_save(slot_number: u32, metadata: SaveSlotMetadata, data: SaveSlotData) -> Self {
        let mut slot = Self::new(slot_number, metadata, data);
        slot.is_auto_save = true;
        slot
    }

    /// Create a backup slot
    pub fn new_backup(slot_number: u32, metadata: SaveSlotMetadata, data: SaveSlotData, backup_number: u32) -> Self {
        let mut slot = Self::new(slot_number, metadata, data);
        slot.is_backup = true;
        slot.backup_number = Some(backup_number);
        slot
    }

    /// Update the save slot data
    pub fn update_data(&mut self, data: SaveSlotData) {
        self.data = data;
        self.last_modified = SystemTime::now();
    }

    /// Update the save slot metadata
    pub fn update_metadata(&mut self, metadata: SaveSlotMetadata) {
        self.metadata = metadata;
        self.last_modified = SystemTime::now();
    }

    /// Get the save slot size in bytes
    pub fn size_bytes(&self) -> usize {
        // This would be calculated based on the actual serialized size
        // For now, return a placeholder
        1024
    }

    /// Check if the save slot is valid
    pub fn is_valid(&self) -> bool {
        !self.metadata.name.is_empty() && 
        !self.metadata.player_name.is_empty() &&
        !self.data.game_state.current_level.is_empty()
    }

    /// Get the save slot age in seconds
    pub fn age_seconds(&self) -> f32 {
        self.last_modified
            .duration_since(self.created_at)
            .unwrap_or_default()
            .as_secs_f32()
    }
}

impl Default for SaveSlotMetadata {
    fn default() -> Self {
        Self {
            name: "New Save".to_string(),
            player_name: "Player".to_string(),
            game_version: "1.0.0".to_string(),
            description: None,
            thumbnail: None,
            play_time: 0.0,
            level_name: "Tutorial".to_string(),
            character_level: 1,
            character_class: "Warrior".to_string(),
            difficulty: "Normal".to_string(),
            completion_percentage: 0.0,
            tags: Vec::new(),
        }
    }
}

impl Default for SaveSlotData {
    fn default() -> Self {
        Self {
            game_state: GameStateData::default(),
            player_data: PlayerData::default(),
            level_data: LevelData::default(),
            inventory_data: InventoryData::default(),
            settings_data: SettingsData::default(),
            achievement_data: AchievementData::default(),
            custom_data: HashMap::new(),
        }
    }
}

impl Default for GameStateData {
    fn default() -> Self {
        Self {
            current_level: "tutorial_01".to_string(),
            current_checkpoint: "start".to_string(),
            game_mode: "story".to_string(),
            game_phase: "tutorial".to_string(),
            time_of_day: 0.0,
            weather: "clear".to_string(),
            active_events: Vec::new(),
            completed_objectives: Vec::new(),
            active_quests: Vec::new(),
        }
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            player_id: "player_001".to_string(),
            character_name: "Hero".to_string(),
            character_class: "Warrior".to_string(),
            level: 1,
            experience: 0,
            health: 100.0,
            max_health: 100.0,
            mana: 50.0,
            max_mana: 50.0,
            position: (0.0, 0.0),
            direction: 0.0,
            stats: PlayerStats::default(),
            skills: HashMap::new(),
            traits: Vec::new(),
            status_effects: Vec::new(),
        }
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            strength: 10,
            dexterity: 10,
            intelligence: 10,
            constitution: 10,
            charisma: 10,
            luck: 10,
        }
    }
}

impl Default for LevelData {
    fn default() -> Self {
        Self {
            level_id: "tutorial_01".to_string(),
            level_name: "Tutorial Level".to_string(),
            level_type: "tutorial".to_string(),
            level_seed: 12345,
            level_progress: 0.0,
            checkpoints: vec!["start".to_string()],
            secrets_found: Vec::new(),
            enemies_defeated: 0,
            items_collected: 0,
            time_spent: 0.0,
        }
    }
}

impl Default for InventoryData {
    fn default() -> Self {
        Self {
            equipped_items: HashMap::new(),
            inventory_items: Vec::new(),
            currency: HashMap::new(),
            key_items: Vec::new(),
            consumables: HashMap::new(),
        }
    }
}

impl Default for SettingsData {
    fn default() -> Self {
        Self {
            graphics: GraphicsSettings::default(),
            audio: AudioSettings::default(),
            controls: ControlSettings::default(),
            gameplay: GameplaySettings::default(),
        }
    }
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            resolution_width: 1920,
            resolution_height: 1080,
            fullscreen: false,
            vsync: true,
            quality_level: "High".to_string(),
            anti_aliasing: true,
            shadows: true,
            particle_effects: true,
        }
    }
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.8,
            sfx_volume: 0.9,
            voice_volume: 0.8,
            ambient_volume: 0.7,
            audio_device: "default".to_string(),
            audio_quality: "High".to_string(),
        }
    }
}

impl Default for ControlSettings {
    fn default() -> Self {
        Self {
            key_bindings: HashMap::new(),
            mouse_sensitivity: 1.0,
            invert_mouse_y: false,
            controller_enabled: false,
            controller_sensitivity: 1.0,
        }
    }
}

impl Default for GameplaySettings {
    fn default() -> Self {
        Self {
            difficulty: "Normal".to_string(),
            auto_save_enabled: true,
            auto_save_interval: 300.0,
            subtitles_enabled: true,
            language: "en".to_string(),
            tutorial_enabled: true,
        }
    }
}

impl Default for AchievementData {
    fn default() -> Self {
        Self {
            unlocked_achievements: Vec::new(),
            achievement_progress: HashMap::new(),
            achievement_stats: HashMap::new(),
            rewards_claimed: Vec::new(),
        }
    }
}
