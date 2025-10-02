//! Sound Library
//! 
//! This module provides sound library management for the sound test system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::SoundCategory;

/// Sound library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundLibrary {
    /// Sound entries
    pub sounds: HashMap<String, SoundEntry>,
    /// Category sounds
    pub category_sounds: HashMap<SoundCategory, Vec<String>>,
    /// Search index
    pub search_index: SearchIndex,
    /// Library metadata
    pub metadata: LibraryMetadata,
}

/// Sound entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEntry {
    /// Sound ID
    pub id: String,
    /// Sound name
    pub name: String,
    /// Sound description
    pub description: String,
    /// Sound category
    pub category: SoundCategory,
    /// File path
    pub file_path: String,
    /// File format
    pub file_format: String,
    /// Sample rate
    pub sample_rate: u32,
    /// Bit depth
    pub bit_depth: u32,
    /// Channels
    pub channels: u32,
    /// Duration (seconds)
    pub duration: f32,
    /// File size (bytes)
    pub file_size: u64,
    /// Tags
    pub tags: Vec<String>,
    /// Keywords
    pub keywords: Vec<String>,
    /// Author
    pub author: String,
    /// License
    pub license: String,
    /// Created date
    pub created_date: String,
    /// Modified date
    pub modified_date: String,
    /// Play count
    pub play_count: u32,
    /// Last played
    pub last_played: Option<String>,
    /// Rating
    pub rating: u8,
    /// Favorites
    pub is_favorite: bool,
    /// Custom properties
    pub custom_properties: HashMap<String, String>,
}

/// Search index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndex {
    /// Name index
    pub name_index: HashMap<String, Vec<String>>,
    /// Tag index
    pub tag_index: HashMap<String, Vec<String>>,
    /// Keyword index
    pub keyword_index: HashMap<String, Vec<String>>,
    /// Category index
    pub category_index: HashMap<SoundCategory, Vec<String>>,
    /// Author index
    pub author_index: HashMap<String, Vec<String>>,
    /// Rating index
    pub rating_index: HashMap<u8, Vec<String>>,
}

/// Library metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryMetadata {
    /// Library name
    pub name: String,
    /// Library version
    pub version: String,
    /// Library description
    pub description: String,
    /// Total sounds
    pub total_sounds: usize,
    /// Total duration
    pub total_duration: f32,
    /// Total size
    pub total_size: u64,
    /// Categories
    pub categories: Vec<SoundCategory>,
    /// Tags
    pub tags: Vec<String>,
    /// Authors
    pub authors: Vec<String>,
    /// Created date
    pub created_date: String,
    /// Modified date
    pub modified_date: String,
}

impl SoundLibrary {
    /// Create new sound library
    pub fn new() -> Self {
        Self {
            sounds: HashMap::new(),
            category_sounds: HashMap::new(),
            search_index: SearchIndex::new(),
            metadata: LibraryMetadata::new(),
        }
    }

    /// Add sound entry
    pub fn add_sound(&mut self, sound: SoundEntry) {
        let id = sound.id.clone();
        let category = sound.category.clone();
        
        // Add to sounds
        self.sounds.insert(id.clone(), sound);
        
        // Add to category sounds
        self.category_sounds.entry(category.clone()).or_insert_with(Vec::new).push(id.clone());
        
        // Update search index
        self.update_search_index(&id);
        
        // Update metadata
        self.update_metadata();
    }

    /// Remove sound entry
    pub fn remove_sound(&mut self, id: &str) -> bool {
        if let Some(sound) = self.sounds.remove(id) {
            // Remove from category sounds
            if let Some(category_sounds) = self.category_sounds.get_mut(&sound.category) {
                category_sounds.retain(|sound_id| sound_id != id);
            }
            
            // Update search index
            self.remove_from_search_index(id);
            
            // Update metadata
            self.update_metadata();
            
            true
        } else {
            false
        }
    }

    /// Get sound entry
    pub fn get_sound(&self, id: &str) -> Option<&SoundEntry> {
        self.sounds.get(id)
    }

    /// Get sound entry mutably
    pub fn get_sound_mut(&mut self, id: &str) -> Option<&mut SoundEntry> {
        self.sounds.get_mut(id)
    }

    /// Get sounds by category
    pub fn get_sounds_by_category(&self, category: &SoundCategory) -> Vec<&SoundEntry> {
        self.category_sounds.get(category)
            .map(|ids| ids.iter().filter_map(|id| self.sounds.get(id)).collect())
            .unwrap_or_default()
    }

    /// Search sounds
    pub fn search_sounds(&self, query: &str) -> Vec<&SoundEntry> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        
        for sound in self.sounds.values() {
            if sound.name.to_lowercase().contains(&query_lower) ||
               sound.description.to_lowercase().contains(&query_lower) ||
               sound.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower)) ||
               sound.keywords.iter().any(|keyword| keyword.to_lowercase().contains(&query_lower)) ||
               sound.author.to_lowercase().contains(&query_lower) {
                results.push(sound);
            }
        }
        
        results
    }

    /// Get sounds by tag
    pub fn get_sounds_by_tag(&self, tag: &str) -> Vec<&SoundEntry> {
        self.search_index.tag_index.get(tag)
            .map(|ids| ids.iter().filter_map(|id| self.sounds.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get sounds by author
    pub fn get_sounds_by_author(&self, author: &str) -> Vec<&SoundEntry> {
        self.search_index.author_index.get(author)
            .map(|ids| ids.iter().filter_map(|id| self.sounds.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get sounds by rating
    pub fn get_sounds_by_rating(&self, rating: u8) -> Vec<&SoundEntry> {
        self.search_index.rating_index.get(&rating)
            .map(|ids| ids.iter().filter_map(|id| self.sounds.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get favorite sounds
    pub fn get_favorite_sounds(&self) -> Vec<&SoundEntry> {
        self.sounds.values().filter(|sound| sound.is_favorite).collect()
    }

    /// Get recently played sounds
    pub fn get_recently_played_sounds(&self, limit: usize) -> Vec<&SoundEntry> {
        let mut sounds: Vec<&SoundEntry> = self.sounds.values()
            .filter(|sound| sound.last_played.is_some())
            .collect();
        
        sounds.sort_by(|a, b| {
            b.last_played.as_ref().unwrap().cmp(a.last_played.as_ref().unwrap())
        });
        
        sounds.truncate(limit);
        sounds
    }

    /// Get most played sounds
    pub fn get_most_played_sounds(&self, limit: usize) -> Vec<&SoundEntry> {
        let mut sounds: Vec<&SoundEntry> = self.sounds.values().collect();
        
        sounds.sort_by(|a, b| b.play_count.cmp(&a.play_count));
        
        sounds.truncate(limit);
        sounds
    }

    /// Update sound play count
    pub fn update_play_count(&mut self, id: &str) {
        if let Some(sound) = self.sounds.get_mut(id) {
            sound.play_count += 1;
            sound.last_played = Some(chrono::Utc::now().to_rfc3339());
        }
    }

    /// Set sound rating
    pub fn set_sound_rating(&mut self, id: &str, rating: u8) {
        if let Some(sound) = self.sounds.get_mut(id) {
            let old_rating = sound.rating;
            sound.rating = rating.max(0).min(5);
            
            // Update rating index
            if old_rating != 0 {
                if let Some(sounds) = self.search_index.rating_index.get_mut(&old_rating) {
                    sounds.retain(|sound_id| sound_id != id);
                }
            }
            
            if sound.rating != 0 {
                self.search_index.rating_index.entry(sound.rating).or_insert_with(Vec::new).push(id.to_string());
            }
        }
    }

    /// Toggle sound favorite
    pub fn toggle_sound_favorite(&mut self, id: &str) {
        if let Some(sound) = self.sounds.get_mut(id) {
            sound.is_favorite = !sound.is_favorite;
        }
    }

    /// Get library statistics
    pub fn get_library_statistics(&self) -> LibraryStatistics {
        let mut total_play_count = 0;
        let mut total_rating = 0.0;
        let mut rated_sounds = 0;
        let mut favorite_count = 0;
        
        for sound in self.sounds.values() {
            total_play_count += sound.play_count;
            if sound.rating > 0 {
                total_rating += sound.rating as f32;
                rated_sounds += 1;
            }
            if sound.is_favorite {
                favorite_count += 1;
            }
        }
        
        let average_rating = if rated_sounds > 0 {
            total_rating / rated_sounds as f32
        } else {
            0.0
        };
        
        LibraryStatistics {
            total_sounds: self.sounds.len(),
            total_duration: self.metadata.total_duration,
            total_size: self.metadata.total_size,
            total_play_count,
            average_rating,
            favorite_count,
            category_count: self.category_sounds.len(),
            tag_count: self.search_index.tag_index.len(),
            author_count: self.search_index.author_index.len(),
        }
    }

    /// Update search index
    fn update_search_index(&mut self, id: &str) {
        if let Some(sound) = self.sounds.get(id) {
            // Name index
            for word in sound.name.split_whitespace() {
                let word_lower = word.to_lowercase();
                self.search_index.name_index.entry(word_lower).or_insert_with(Vec::new).push(id.to_string());
            }
            
            // Tag index
            for tag in &sound.tags {
                let tag_lower = tag.to_lowercase();
                self.search_index.tag_index.entry(tag_lower).or_insert_with(Vec::new).push(id.to_string());
            }
            
            // Keyword index
            for keyword in &sound.keywords {
                let keyword_lower = keyword.to_lowercase();
                self.search_index.keyword_index.entry(keyword_lower).or_insert_with(Vec::new).push(id.to_string());
            }
            
            // Category index
            self.search_index.category_index.entry(sound.category.clone()).or_insert_with(Vec::new).push(id.to_string());
            
            // Author index
            if !sound.author.is_empty() {
                let author_lower = sound.author.to_lowercase();
                self.search_index.author_index.entry(author_lower).or_insert_with(Vec::new).push(id.to_string());
            }
            
            // Rating index
            if sound.rating > 0 {
                self.search_index.rating_index.entry(sound.rating).or_insert_with(Vec::new).push(id.to_string());
            }
        }
    }

    /// Remove from search index
    fn remove_from_search_index(&mut self, id: &str) {
        // Remove from all indexes
        for sounds in self.search_index.name_index.values_mut() {
            sounds.retain(|sound_id| sound_id != id);
        }
        
        for sounds in self.search_index.tag_index.values_mut() {
            sounds.retain(|sound_id| sound_id != id);
        }
        
        for sounds in self.search_index.keyword_index.values_mut() {
            sounds.retain(|sound_id| sound_id != id);
        }
        
        for sounds in self.search_index.category_index.values_mut() {
            sounds.retain(|sound_id| sound_id != id);
        }
        
        for sounds in self.search_index.author_index.values_mut() {
            sounds.retain(|sound_id| sound_id != id);
        }
        
        for sounds in self.search_index.rating_index.values_mut() {
            sounds.retain(|sound_id| sound_id != id);
        }
    }

    /// Update metadata
    fn update_metadata(&mut self) {
        self.metadata.total_sounds = self.sounds.len();
        
        let mut total_duration = 0.0;
        let mut total_size = 0;
        let mut categories = std::collections::HashSet::new();
        let mut tags = std::collections::HashSet::new();
        let mut authors = std::collections::HashSet::new();
        
        for sound in self.sounds.values() {
            total_duration += sound.duration;
            total_size += sound.file_size as u64;
            categories.insert(sound.category.clone());
            
            for tag in &sound.tags {
                tags.insert(tag.clone());
            }
            
            if !sound.author.is_empty() {
                authors.insert(sound.author.clone());
            }
        }
        
        self.metadata.total_duration = total_duration;
        self.metadata.total_size = total_size;
        self.metadata.categories = categories.into_iter().collect();
        self.metadata.tags = tags.into_iter().collect();
        self.metadata.authors = authors.into_iter().collect();
        self.metadata.modified_date = chrono::Utc::now().to_rfc3339();
    }
}

/// Library statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryStatistics {
    /// Total sounds
    pub total_sounds: usize,
    /// Total duration (seconds)
    pub total_duration: f32,
    /// Total size (bytes)
    pub total_size: u64,
    /// Total play count
    pub total_play_count: u32,
    /// Average rating
    pub average_rating: f32,
    /// Favorite count
    pub favorite_count: usize,
    /// Category count
    pub category_count: usize,
    /// Tag count
    pub tag_count: usize,
    /// Author count
    pub author_count: usize,
}

impl SearchIndex {
    /// Create new search index
    pub fn new() -> Self {
        Self {
            name_index: HashMap::new(),
            tag_index: HashMap::new(),
            keyword_index: HashMap::new(),
            category_index: HashMap::new(),
            author_index: HashMap::new(),
            rating_index: HashMap::new(),
        }
    }
}

impl LibraryMetadata {
    /// Create new library metadata
    pub fn new() -> Self {
        Self {
            name: "Sound Library".to_string(),
            version: "1.0.0".to_string(),
            description: "Game sound library".to_string(),
            total_sounds: 0,
            total_duration: 0.0,
            total_size: 0,
            categories: Vec::new(),
            tags: Vec::new(),
            authors: Vec::new(),
            created_date: chrono::Utc::now().to_rfc3339(),
            modified_date: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl Default for SoundLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LibraryMetadata {
    fn default() -> Self {
        Self::new()
    }
}
