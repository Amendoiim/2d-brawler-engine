//! Character Selection Module
//! 
//! This module provides character selection and management functionality including:
//! - Character selection interface
//! - Character creation and deletion
//! - Character management and organization
//! - Character comparison and statistics

use super::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Character selection manager
#[derive(Clone, Debug)]
pub struct CharacterSelection {
    /// Available characters
    pub characters: HashMap<String, Character>,
    /// Character selection state
    pub selection_state: SelectionState,
    /// Character filters
    pub filters: CharacterFilters,
    /// Character sorting options
    pub sorting_options: SortingOptions,
    /// Character comparison data
    pub comparison_data: CharacterComparison,
}

/// Character selection state
#[derive(Clone, Debug)]
pub struct SelectionState {
    /// Currently selected character
    pub selected_character: Option<String>,
    /// Character selection mode
    pub selection_mode: SelectionMode,
    /// Character creation state
    pub creation_state: CreationState,
    /// Character deletion confirmation
    pub deletion_confirmation: Option<String>,
}

/// Selection mode
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SelectionMode {
    /// Viewing character list
    Viewing,
    /// Creating new character
    Creating,
    /// Editing existing character
    Editing,
    /// Comparing characters
    Comparing,
    /// Deleting character
    Deleting,
}

/// Character creation state
#[derive(Clone, Debug)]
pub struct CreationState {
    /// Current step in creation process
    pub current_step: CreationStep,
    /// Character being created
    pub character_in_creation: Option<Character>,
    /// Creation progress
    pub progress: f32,
    /// Available templates
    pub available_templates: Vec<String>,
}

/// Character creation step
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CreationStep {
    /// Select character class
    SelectClass,
    /// Customize appearance
    CustomizeAppearance,
    /// Allocate stats
    AllocateStats,
    /// Select abilities
    SelectAbilities,
    /// Choose equipment
    ChooseEquipment,
    /// Finalize character
    Finalize,
}

/// Character filters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterFilters {
    /// Filter by character class
    pub class_filter: Option<CharacterClass>,
    /// Filter by level range
    pub level_range: Option<(u32, u32)>,
    /// Filter by name
    pub name_filter: Option<String>,
    /// Filter by creation date
    pub creation_date_filter: Option<(u64, u64)>,
    /// Filter by last played
    pub last_played_filter: Option<(u64, u64)>,
    /// Filter by completion status
    pub completion_filter: Option<CompletionStatus>,
}

/// Completion status
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CompletionStatus {
    /// Character is new
    New,
    /// Character is in progress
    InProgress,
    /// Character is completed
    Completed,
    /// Character is abandoned
    Abandoned,
}

/// Sorting options
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortingOptions {
    /// Sort by field
    pub sort_by: SortField,
    /// Sort order
    pub sort_order: SortOrder,
    /// Secondary sort field
    pub secondary_sort: Option<SortField>,
}

/// Sort field
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SortField {
    /// Sort by name
    Name,
    /// Sort by level
    Level,
    /// Sort by class
    Class,
    /// Sort by creation date
    CreationDate,
    /// Sort by last played
    LastPlayed,
    /// Sort by experience
    Experience,
    /// Sort by play time
    PlayTime,
}

/// Sort order
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SortOrder {
    /// Ascending order
    Ascending,
    /// Descending order
    Descending,
}

/// Character comparison data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterComparison {
    /// Characters being compared
    pub compared_characters: Vec<String>,
    /// Comparison statistics
    pub comparison_stats: HashMap<String, ComparisonStat>,
    /// Comparison mode
    pub comparison_mode: ComparisonMode,
}

/// Comparison mode
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ComparisonMode {
    /// Compare stats
    Stats,
    /// Compare abilities
    Abilities,
    /// Compare equipment
    Equipment,
    /// Compare progression
    Progression,
}

/// Comparison statistic
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComparisonStat {
    /// Stat name
    pub name: String,
    /// Stat values for each character
    pub values: HashMap<String, f32>,
    /// Stat unit
    pub unit: String,
    /// Higher is better
    pub higher_is_better: bool,
}

/// Character selection manager implementation
impl CharacterSelection {
    /// Create a new character selection manager
    pub fn new() -> Self {
        Self {
            characters: HashMap::new(),
            selection_state: SelectionState {
                selected_character: None,
                selection_mode: SelectionMode::Viewing,
                creation_state: CreationState {
                    current_step: CreationStep::SelectClass,
                    character_in_creation: None,
                    progress: 0.0,
                    available_templates: Vec::new(),
                },
                deletion_confirmation: None,
            },
            filters: CharacterFilters {
                class_filter: None,
                level_range: None,
                name_filter: None,
                creation_date_filter: None,
                last_played_filter: None,
                completion_filter: None,
            },
            sorting_options: SortingOptions {
                sort_by: SortField::Name,
                sort_order: SortOrder::Ascending,
                secondary_sort: None,
            },
            comparison_data: CharacterComparison {
                compared_characters: Vec::new(),
                comparison_stats: HashMap::new(),
                comparison_mode: ComparisonMode::Stats,
            },
        }
    }

    /// Add character to selection
    pub fn add_character(&mut self, character: Character) {
        self.characters.insert(character.id.clone(), character);
    }

    /// Remove character from selection
    pub fn remove_character(&mut self, character_id: &str) -> Option<Character> {
        self.characters.remove(character_id)
    }

    /// Get character by ID
    pub fn get_character(&self, character_id: &str) -> Option<&Character> {
        self.characters.get(character_id)
    }

    /// Get character by ID (mutable)
    pub fn get_character_mut(&mut self, character_id: &str) -> Option<&mut Character> {
        self.characters.get_mut(character_id)
    }

    /// Select character
    pub fn select_character(&mut self, character_id: &str) -> Result<(), String> {
        if !self.characters.contains_key(character_id) {
            return Err(format!("Character '{}' not found", character_id));
        }

        self.selection_state.selected_character = Some(character_id.to_string());
        self.selection_state.selection_mode = SelectionMode::Viewing;
        Ok(())
    }

    /// Deselect character
    pub fn deselect_character(&mut self) {
        self.selection_state.selected_character = None;
        self.selection_state.selection_mode = SelectionMode::Viewing;
    }

    /// Get selected character
    pub fn get_selected_character(&self) -> Option<&Character> {
        self.selection_state.selected_character.as_ref()
            .and_then(|id| self.characters.get(id))
    }

    /// Get selected character (mutable)
    pub fn get_selected_character_mut(&mut self) -> Option<&mut Character> {
        self.selection_state.selected_character.as_ref()
            .and_then(|id| self.characters.get_mut(id))
    }

    /// Start character creation
    pub fn start_character_creation(&mut self, templates: Vec<String>) {
        self.selection_state.selection_mode = SelectionMode::Creating;
        self.selection_state.creation_state = CreationState {
            current_step: CreationStep::SelectClass,
            character_in_creation: None,
            progress: 0.0,
            available_templates: templates,
        };
    }

    /// Cancel character creation
    pub fn cancel_character_creation(&mut self) {
        self.selection_state.selection_mode = SelectionMode::Viewing;
        self.selection_state.creation_state = CreationState {
            current_step: CreationStep::SelectClass,
            character_in_creation: None,
            progress: 0.0,
            available_templates: Vec::new(),
        };
    }

    /// Complete character creation
    pub fn complete_character_creation(&mut self) -> Result<Character, String> {
        let character = self.selection_state.creation_state.character_in_creation.take()
            .ok_or("No character in creation")?;

        let character_id = character.id.clone();
        self.characters.insert(character_id, character.clone());
        
        self.selection_state.selection_mode = SelectionMode::Viewing;
        self.selection_state.creation_state = CreationState {
            current_step: CreationStep::SelectClass,
            character_in_creation: None,
            progress: 0.0,
            available_templates: Vec::new(),
        };

        Ok(character)
    }

    /// Start character deletion
    pub fn start_character_deletion(&mut self, character_id: &str) -> Result<(), String> {
        if !self.characters.contains_key(character_id) {
            return Err(format!("Character '{}' not found", character_id));
        }

        self.selection_state.selection_mode = SelectionMode::Deleting;
        self.selection_state.deletion_confirmation = Some(character_id.to_string());
        Ok(())
    }

    /// Confirm character deletion
    pub fn confirm_character_deletion(&mut self) -> Result<Character, String> {
        let character_id = self.selection_state.deletion_confirmation.take()
            .ok_or("No character selected for deletion")?;

        let character = self.characters.remove(&character_id)
            .ok_or_else(|| format!("Character '{}' not found", character_id))?;

        self.selection_state.selection_mode = SelectionMode::Viewing;
        Ok(character)
    }

    /// Cancel character deletion
    pub fn cancel_character_deletion(&mut self) {
        self.selection_state.selection_mode = SelectionMode::Viewing;
        self.selection_state.deletion_confirmation = None;
    }

    /// Get filtered characters
    pub fn get_filtered_characters(&self) -> Vec<&Character> {
        let mut characters: Vec<&Character> = self.characters.values().collect();

        // Apply filters
        if let Some(class_filter) = &self.filters.class_filter {
            characters.retain(|character| &character.class == class_filter);
        }

        if let Some((min_level, max_level)) = &self.filters.level_range {
            characters.retain(|character| character.level >= *min_level && character.level <= *max_level);
        }

        if let Some(name_filter) = &self.filters.name_filter {
            characters.retain(|character| character.name.to_lowercase().contains(&name_filter.to_lowercase()));
        }

        // Apply sorting
        characters.sort_by(|a, b| {
            let comparison = match self.sorting_options.sort_by {
                SortField::Name => a.name.cmp(&b.name),
                SortField::Level => a.level.cmp(&b.level),
                SortField::Class => a.class.partial_cmp(&b.class).unwrap_or(std::cmp::Ordering::Equal),
                SortField::Experience => a.experience.cmp(&b.experience),
                SortField::CreationDate => {
                    // This would need to be tracked separately
                    std::cmp::Ordering::Equal
                }
                SortField::LastPlayed => {
                    // This would need to be tracked separately
                    std::cmp::Ordering::Equal
                }
                SortField::PlayTime => {
                    // This would need to be tracked separately
                    std::cmp::Ordering::Equal
                }
            };

            match self.sorting_options.sort_order {
                SortOrder::Ascending => comparison,
                SortOrder::Descending => comparison.reverse(),
            }
        });

        characters
    }

    /// Set character filter
    pub fn set_filter(&mut self, filter_type: FilterType, value: Option<FilterValue>) {
        match filter_type {
            FilterType::Class => {
                self.filters.class_filter = value.and_then(|v| match v {
                    FilterValue::Class(c) => Some(c),
                    _ => None,
                });
            }
            FilterType::LevelRange => {
                self.filters.level_range = value.and_then(|v| match v {
                    FilterValue::LevelRange(range) => Some(range),
                    _ => None,
                });
            }
            FilterType::Name => {
                self.filters.name_filter = value.and_then(|v| match v {
                    FilterValue::String(s) => Some(s),
                    _ => None,
                });
            }
            FilterType::CreationDate => {
                self.filters.creation_date_filter = value.and_then(|v| match v {
                    FilterValue::DateRange(range) => Some(range),
                    _ => None,
                });
            }
            FilterType::LastPlayed => {
                self.filters.last_played_filter = value.and_then(|v| match v {
                    FilterValue::DateRange(range) => Some(range),
                    _ => None,
                });
            }
            FilterType::CompletionStatus => {
                self.filters.completion_filter = value.and_then(|v| match v {
                    FilterValue::CompletionStatus(status) => Some(status),
                    _ => None,
                });
            }
        }
    }

    /// Set sorting options
    pub fn set_sorting(&mut self, sort_by: SortField, sort_order: SortOrder, secondary_sort: Option<SortField>) {
        self.sorting_options.sort_by = sort_by;
        self.sorting_options.sort_order = sort_order;
        self.sorting_options.secondary_sort = secondary_sort;
    }

    /// Start character comparison
    pub fn start_character_comparison(&mut self, character_ids: Vec<String>) -> Result<(), String> {
        for character_id in &character_ids {
            if !self.characters.contains_key(character_id) {
                return Err(format!("Character '{}' not found", character_id));
            }
        }

        self.selection_state.selection_mode = SelectionMode::Comparing;
        self.comparison_data.compared_characters = character_ids;
        self.generate_comparison_stats();
        Ok(())
    }

    /// Stop character comparison
    pub fn stop_character_comparison(&mut self) {
        self.selection_state.selection_mode = SelectionMode::Viewing;
        self.comparison_data.compared_characters.clear();
        self.comparison_data.comparison_stats.clear();
    }

    /// Generate comparison statistics
    fn generate_comparison_stats(&mut self) {
        self.comparison_data.comparison_stats.clear();

        for character_id in &self.comparison_data.compared_characters {
            if let Some(character) = self.characters.get(character_id) {
                // Level comparison
                self.comparison_data.comparison_stats.insert(
                    "level".to_string(),
                    ComparisonStat {
                        name: "Level".to_string(),
                        values: HashMap::new(),
                        unit: "".to_string(),
                        higher_is_better: true,
                    }
                );

                // Experience comparison
                self.comparison_data.comparison_stats.insert(
                    "experience".to_string(),
                    ComparisonStat {
                        name: "Experience".to_string(),
                        values: HashMap::new(),
                        unit: "XP".to_string(),
                        higher_is_better: true,
                    }
                );

                // Health comparison
                self.comparison_data.comparison_stats.insert(
                    "health".to_string(),
                    ComparisonStat {
                        name: "Health".to_string(),
                        values: HashMap::new(),
                        unit: "HP".to_string(),
                        higher_is_better: true,
                    }
                );

                // Mana comparison
                self.comparison_data.comparison_stats.insert(
                    "mana".to_string(),
                    ComparisonStat {
                        name: "Mana".to_string(),
                        values: HashMap::new(),
                        unit: "MP".to_string(),
                        higher_is_better: true,
                    }
                );

                // Damage comparison
                self.comparison_data.comparison_stats.insert(
                    "damage".to_string(),
                    ComparisonStat {
                        name: "Damage".to_string(),
                        values: HashMap::new(),
                        unit: "DMG".to_string(),
                        higher_is_better: true,
                    }
                );

                // Defense comparison
                self.comparison_data.comparison_stats.insert(
                    "defense".to_string(),
                    ComparisonStat {
                        name: "Defense".to_string(),
                        values: HashMap::new(),
                        unit: "DEF".to_string(),
                        higher_is_better: true,
                    }
                );

                // Speed comparison
                self.comparison_data.comparison_stats.insert(
                    "speed".to_string(),
                    ComparisonStat {
                        name: "Speed".to_string(),
                        values: HashMap::new(),
                        unit: "SPD".to_string(),
                        higher_is_better: true,
                    }
                );
            }
        }

        // Populate comparison values
        for character_id in &self.comparison_data.compared_characters {
            if let Some(character) = self.characters.get(character_id) {
                if let Some(level_stat) = self.comparison_data.comparison_stats.get_mut("level") {
                    level_stat.values.insert(character_id.clone(), character.level as f32);
                }
                if let Some(exp_stat) = self.comparison_data.comparison_stats.get_mut("experience") {
                    exp_stat.values.insert(character_id.clone(), character.experience as f32);
                }
                if let Some(health_stat) = self.comparison_data.comparison_stats.get_mut("health") {
                    health_stat.values.insert(character_id.clone(), character.status.max_health);
                }
                if let Some(mana_stat) = self.comparison_data.comparison_stats.get_mut("mana") {
                    mana_stat.values.insert(character_id.clone(), character.status.max_mana);
                }
                if let Some(damage_stat) = self.comparison_data.comparison_stats.get_mut("damage") {
                    damage_stat.values.insert(character_id.clone(), character.get_total_damage());
                }
                if let Some(defense_stat) = self.comparison_data.comparison_stats.get_mut("defense") {
                    defense_stat.values.insert(character_id.clone(), character.get_total_defense());
                }
                if let Some(speed_stat) = self.comparison_data.comparison_stats.get_mut("speed") {
                    speed_stat.values.insert(character_id.clone(), character.get_movement_speed());
                }
            }
        }
    }

    /// Get comparison statistics
    pub fn get_comparison_stats(&self) -> &HashMap<String, ComparisonStat> {
        &self.comparison_data.comparison_stats
    }

    /// Get character count
    pub fn get_character_count(&self) -> usize {
        self.characters.len()
    }

    /// Get character count by class
    pub fn get_character_count_by_class(&self, class: &CharacterClass) -> usize {
        self.characters.values()
            .filter(|character| &character.class == class)
            .count()
    }

    /// Get character statistics
    pub fn get_character_statistics(&self) -> CharacterSelectionStatistics {
        let total_characters = self.characters.len();
        let characters_by_class: HashMap<CharacterClass, usize> = self.characters.values()
            .fold(HashMap::new(), |mut acc, character| {
                *acc.entry(character.class.clone()).or_insert(0) += 1;
                acc
            });

        let average_level = if total_characters > 0 {
            self.characters.values()
                .map(|character| character.level)
                .sum::<u32>() as f32 / total_characters as f32
        } else {
            0.0
        };

        let total_experience = self.characters.values()
            .map(|character| character.experience)
            .sum();

        CharacterSelectionStatistics {
            total_characters,
            characters_by_class,
            average_level,
            total_experience,
            selected_character: self.selection_state.selected_character.clone(),
            selection_mode: self.selection_state.selection_mode.clone(),
        }
    }

    /// Clear all filters
    pub fn clear_filters(&mut self) {
        self.filters = CharacterFilters {
            class_filter: None,
            level_range: None,
            name_filter: None,
            creation_date_filter: None,
            last_played_filter: None,
            completion_filter: None,
        };
    }

    /// Reset sorting to default
    pub fn reset_sorting(&mut self) {
        self.sorting_options = SortingOptions {
            sort_by: SortField::Name,
            sort_order: SortOrder::Ascending,
            secondary_sort: None,
        };
    }
}

/// Filter type
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FilterType {
    Class,
    LevelRange,
    Name,
    CreationDate,
    LastPlayed,
    CompletionStatus,
}

/// Filter value
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FilterValue {
    Class(CharacterClass),
    LevelRange((u32, u32)),
    String(String),
    DateRange((u64, u64)),
    CompletionStatus(CompletionStatus),
}

/// Character selection statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterSelectionStatistics {
    /// Total number of characters
    pub total_characters: usize,
    /// Characters by class
    pub characters_by_class: HashMap<CharacterClass, usize>,
    /// Average character level
    pub average_level: f32,
    /// Total experience across all characters
    pub total_experience: u32,
    /// Currently selected character
    pub selected_character: Option<String>,
    /// Current selection mode
    pub selection_mode: SelectionMode,
}

impl Default for CharacterSelection {
    fn default() -> Self {
        Self::new()
    }
}
