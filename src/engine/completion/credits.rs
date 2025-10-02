//! Credits Manager
//! 
//! This module manages the credits system, including credits display, scrolling, and interaction.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use super::{CompletionConfig, CompletionEvent, CompletionResult, CompletionError};

/// Credits manager
pub struct CreditsManager {
    /// Configuration
    pub config: CompletionConfig,
    /// Credits data
    pub credits: CreditsData,
    /// Credits playing
    pub is_playing: bool,
    /// Credits paused
    pub is_paused: bool,
    /// Current position
    pub current_position: f32,
    /// Scroll speed
    pub scroll_speed: f32,
    /// Credits duration
    pub duration: f32,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&CompletionEvent) + Send + Sync>>,
}

/// Credits data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditsData {
    /// Credits sections
    pub sections: Vec<CreditsSection>,
    /// Total height
    pub total_height: f32,
    /// Background music
    pub background_music: Option<String>,
    /// Background image
    pub background_image: Option<String>,
}

/// Credits section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditsSection {
    /// Section title
    pub title: String,
    /// Section entries
    pub entries: Vec<CreditsEntry>,
    /// Section position
    pub position: f32,
    /// Section height
    pub height: f32,
}

/// Credits entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditsEntry {
    /// Entry text
    pub text: String,
    /// Entry type
    pub entry_type: CreditsEntryType,
    /// Entry position
    pub position: f32,
    /// Entry height
    pub height: f32,
    /// Entry style
    pub style: CreditsStyle,
}

/// Credits entry type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreditsEntryType {
    /// Title
    Title,
    /// Subtitle
    Subtitle,
    /// Name
    Name,
    /// Role
    Role,
    /// Company
    Company,
    /// Special thanks
    SpecialThanks,
    /// Copyright
    Copyright,
}

/// Credits style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditsStyle {
    /// Font size
    pub font_size: f32,
    /// Font color
    pub font_color: String,
    /// Bold
    pub bold: bool,
    /// Italic
    pub italic: bool,
    /// Alignment
    pub alignment: CreditsAlignment,
}

/// Credits alignment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreditsAlignment {
    /// Left aligned
    Left,
    /// Center aligned
    Center,
    /// Right aligned
    Right,
}

impl CreditsManager {
    /// Create a new credits manager
    pub fn new(config: CompletionConfig) -> Self {
        let mut manager = Self {
            config,
            credits: CreditsData::default(),
            is_playing: false,
            is_paused: false,
            current_position: 0.0,
            scroll_speed: 50.0, // pixels per second
            duration: 120.0, // 2 minutes
            event_handlers: Vec::new(),
        };

        // Initialize default credits
        manager.initialize_credits();
        manager
    }

    /// Update credits manager
    pub fn update(&mut self, delta_time: f32) -> CompletionResult<()> {
        if !self.config.credits_enabled {
            return Ok(());
        }

        if self.is_playing && !self.is_paused {
            // Update scroll position
            self.current_position += self.scroll_speed * delta_time;

            // Check if credits are finished
            if self.current_position >= self.credits.total_height {
                self.stop_credits()?;
            }
        }

        Ok(())
    }

    /// Start credits
    pub fn start_credits(&mut self) -> CompletionResult<()> {
        if !self.config.credits_enabled {
            return Err(CompletionError::CreditsNotAvailable);
        }

        self.is_playing = true;
        self.is_paused = false;
        self.current_position = 0.0;

        self.emit_event(CompletionEvent::CreditsStarted);
        Ok(())
    }

    /// Stop credits
    pub fn stop_credits(&mut self) -> CompletionResult<()> {
        self.is_playing = false;
        self.is_paused = false;
        self.current_position = 0.0;

        self.emit_event(CompletionEvent::CreditsCompleted);
        Ok(())
    }

    /// Pause credits
    pub fn pause_credits(&mut self) -> CompletionResult<()> {
        if self.is_playing {
            self.is_paused = true;
        }
        Ok(())
    }

    /// Resume credits
    pub fn resume_credits(&mut self) -> CompletionResult<()> {
        if self.is_playing {
            self.is_paused = false;
        }
        Ok(())
    }

    /// Skip credits
    pub fn skip_credits(&mut self) -> CompletionResult<()> {
        self.stop_credits()?;
        Ok(())
    }

    /// Set scroll speed
    pub fn set_scroll_speed(&mut self, speed: f32) {
        self.scroll_speed = speed.max(0.0);
    }

    /// Get current position
    pub fn get_current_position(&self) -> f32 {
        self.current_position
    }

    /// Get progress percentage
    pub fn get_progress_percentage(&self) -> f32 {
        if self.credits.total_height > 0.0 {
            (self.current_position / self.credits.total_height * 100.0).min(100.0)
        } else {
            0.0
        }
    }

    /// Get credits data
    pub fn get_credits_data(&self) -> &CreditsData {
        &self.credits
    }

    /// Check if credits are playing
    pub fn is_credits_playing(&self) -> bool {
        self.is_playing
    }

    /// Check if credits are paused
    pub fn is_credits_paused(&self) -> bool {
        self.is_paused
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&CompletionEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Initialize default credits
    fn initialize_credits(&mut self) {
        let mut sections = Vec::new();
        let mut current_position = 0.0;

        // Game Title Section
        let title_section = CreditsSection {
            title: "Game Title".to_string(),
            entries: vec![
                CreditsEntry {
                    text: "PROJECT REVOLUTION".to_string(),
                    entry_type: CreditsEntryType::Title,
                    position: current_position,
                    height: 60.0,
                    style: CreditsStyle {
                        font_size: 48.0,
                        font_color: "#FFFFFF".to_string(),
                        bold: true,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
                CreditsEntry {
                    text: "A 2D Brawler Adventure".to_string(),
                    entry_type: CreditsEntryType::Subtitle,
                    position: current_position + 60.0,
                    height: 30.0,
                    style: CreditsStyle {
                        font_size: 24.0,
                        font_color: "#CCCCCC".to_string(),
                        bold: false,
                        italic: true,
                        alignment: CreditsAlignment::Center,
                    },
                },
            ],
            position: current_position,
            height: 90.0,
        };
        current_position += 90.0 + 20.0; // Add spacing
        sections.push(title_section);

        // Development Team Section
        let dev_section = CreditsSection {
            title: "Development Team".to_string(),
            entries: vec![
                CreditsEntry {
                    text: "Development Team".to_string(),
                    entry_type: CreditsEntryType::Title,
                    position: current_position,
                    height: 40.0,
                    style: CreditsStyle {
                        font_size: 32.0,
                        font_color: "#FFD700".to_string(),
                        bold: true,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
                CreditsEntry {
                    text: "Lead Developer".to_string(),
                    entry_type: CreditsEntryType::Role,
                    position: current_position + 50.0,
                    height: 25.0,
                    style: CreditsStyle {
                        font_size: 20.0,
                        font_color: "#FFFFFF".to_string(),
                        bold: true,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
                CreditsEntry {
                    text: "Marcelo Martins".to_string(),
                    entry_type: CreditsEntryType::Name,
                    position: current_position + 75.0,
                    height: 25.0,
                    style: CreditsStyle {
                        font_size: 18.0,
                        font_color: "#CCCCCC".to_string(),
                        bold: false,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
                CreditsEntry {
                    text: "Game Designer".to_string(),
                    entry_type: CreditsEntryType::Role,
                    position: current_position + 110.0,
                    height: 25.0,
                    style: CreditsStyle {
                        font_size: 20.0,
                        font_color: "#FFFFFF".to_string(),
                        bold: true,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
                CreditsEntry {
                    text: "Marcelo Martins".to_string(),
                    entry_type: CreditsEntryType::Name,
                    position: current_position + 135.0,
                    height: 25.0,
                    style: CreditsStyle {
                        font_size: 18.0,
                        font_color: "#CCCCCC".to_string(),
                        bold: false,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
            ],
            position: current_position,
            height: 160.0,
        };
        current_position += 160.0 + 20.0;
        sections.push(dev_section);

        // Technology Section
        let tech_section = CreditsSection {
            title: "Technology".to_string(),
            entries: vec![
                CreditsEntry {
                    text: "Technology".to_string(),
                    entry_type: CreditsEntryType::Title,
                    position: current_position,
                    height: 40.0,
                    style: CreditsStyle {
                        font_size: 32.0,
                        font_color: "#FFD700".to_string(),
                        bold: true,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
                CreditsEntry {
                    text: "Built with Rust".to_string(),
                    entry_type: CreditsEntryType::Name,
                    position: current_position + 50.0,
                    height: 25.0,
                    style: CreditsStyle {
                        font_size: 18.0,
                        font_color: "#CCCCCC".to_string(),
                        bold: false,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
                CreditsEntry {
                    text: "WGPU Graphics".to_string(),
                    entry_type: CreditsEntryType::Name,
                    position: current_position + 85.0,
                    height: 25.0,
                    style: CreditsStyle {
                        font_size: 18.0,
                        font_color: "#CCCCCC".to_string(),
                        bold: false,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
                CreditsEntry {
                    text: "Bevy ECS".to_string(),
                    entry_type: CreditsEntryType::Name,
                    position: current_position + 120.0,
                    height: 25.0,
                    style: CreditsStyle {
                        font_size: 18.0,
                        font_color: "#CCCCCC".to_string(),
                        bold: false,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
            ],
            position: current_position,
            height: 145.0,
        };
        current_position += 145.0 + 20.0;
        sections.push(tech_section);

        // Special Thanks Section
        let thanks_section = CreditsSection {
            title: "Special Thanks".to_string(),
            entries: vec![
                CreditsEntry {
                    text: "Special Thanks".to_string(),
                    entry_type: CreditsEntryType::Title,
                    position: current_position,
                    height: 40.0,
                    style: CreditsStyle {
                        font_size: 32.0,
                        font_color: "#FFD700".to_string(),
                        bold: true,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
                CreditsEntry {
                    text: "Rust Community".to_string(),
                    entry_type: CreditsEntryType::SpecialThanks,
                    position: current_position + 50.0,
                    height: 25.0,
                    style: CreditsStyle {
                        font_size: 18.0,
                        font_color: "#CCCCCC".to_string(),
                        bold: false,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
                CreditsEntry {
                    text: "Open Source Contributors".to_string(),
                    entry_type: CreditsEntryType::SpecialThanks,
                    position: current_position + 85.0,
                    height: 25.0,
                    style: CreditsStyle {
                        font_size: 18.0,
                        font_color: "#CCCCCC".to_string(),
                        bold: false,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
                CreditsEntry {
                    text: "Beta Testers".to_string(),
                    entry_type: CreditsEntryType::SpecialThanks,
                    position: current_position + 120.0,
                    height: 25.0,
                    style: CreditsStyle {
                        font_size: 18.0,
                        font_color: "#CCCCCC".to_string(),
                        bold: false,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
            ],
            position: current_position,
            height: 145.0,
        };
        current_position += 145.0 + 20.0;
        sections.push(thanks_section);

        // Copyright Section
        let copyright_section = CreditsSection {
            title: "Copyright".to_string(),
            entries: vec![
                CreditsEntry {
                    text: "© 2025 Marcelo Martins".to_string(),
                    entry_type: CreditsEntryType::Copyright,
                    position: current_position,
                    height: 25.0,
                    style: CreditsStyle {
                        font_size: 16.0,
                        font_color: "#888888".to_string(),
                        bold: false,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
                CreditsEntry {
                    text: "All Rights Reserved".to_string(),
                    entry_type: CreditsEntryType::Copyright,
                    position: current_position + 35.0,
                    height: 25.0,
                    style: CreditsStyle {
                        font_size: 16.0,
                        font_color: "#888888".to_string(),
                        bold: false,
                        italic: false,
                        alignment: CreditsAlignment::Center,
                    },
                },
            ],
            position: current_position,
            height: 60.0,
        };
        current_position += 60.0 + 20.0;
        sections.push(copyright_section);

        self.credits = CreditsData {
            sections,
            total_height: current_position,
            background_music: Some("credits_theme.ogg".to_string()),
            background_image: Some("credits_background.png".to_string()),
        };
    }

    /// Emit completion event
    fn emit_event(&self, event: CompletionEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for CreditsData {
    fn default() -> Self {
        Self {
            sections: Vec::new(),
            total_height: 0.0,
            background_music: None,
            background_image: None,
        }
    }
}

impl Default for CreditsManager {
    fn default() -> Self {
        Self::new(CompletionConfig::default())
    }
}
