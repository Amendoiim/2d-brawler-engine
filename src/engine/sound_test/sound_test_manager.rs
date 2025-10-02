//! Sound Test Manager
//! 
//! This module provides the main sound test management system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::{
    SoundTestConfig, SoundTestEvent, SoundTestResult, SoundTestError,
    SoundCategory, PlaybackControls, WaveformDisplay, SoundLibrary
};
use crate::engine::sound_test::sound_library::SoundEntry;

/// Sound test manager
pub struct SoundTestManager {
    /// Configuration
    pub config: SoundTestConfig,
    /// Category manager
    pub category_manager: SoundCategoryManager,
    /// Playback controls
    pub playback_controls: PlaybackControls,
    /// Waveform display
    pub waveform_display: WaveformDisplay,
    /// Sound library
    pub sound_library: SoundLibrary,
    /// Currently playing sounds
    pub playing_sounds: HashMap<String, PlayingSound>,
    /// Sound test state
    pub state: SoundTestState,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&SoundTestEvent) + Send + Sync>>,
}

/// Sound category manager (re-export from sound_categories)
use crate::engine::sound_test::sound_categories::SoundCategoryManager;

/// Playing sound
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayingSound {
    /// Sound ID
    pub sound_id: String,
    /// Category
    pub category: SoundCategory,
    /// Start time
    pub start_time: f32,
    /// Current time
    pub current_time: f32,
    /// Duration
    pub duration: f32,
    /// Volume
    pub volume: f32,
    /// Pitch
    pub pitch: f32,
    /// Pan
    pub pan: f32,
    /// Playback speed
    pub playback_speed: f32,
    /// Is looping
    pub is_looping: bool,
    /// Is paused
    pub is_paused: bool,
    /// Is muted
    pub is_muted: bool,
    /// Fade in time
    pub fade_in_time: f32,
    /// Fade out time
    pub fade_out_time: f32,
    /// Fade in progress
    pub fade_in_progress: f32,
    /// Fade out progress
    pub fade_out_progress: f32,
}

/// Sound test state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SoundTestState {
    /// Stopped
    Stopped,
    /// Playing
    Playing,
    /// Paused
    Paused,
    /// Recording
    Recording,
    /// Exporting
    Exporting,
    /// Importing
    Importing,
}

impl SoundTestManager {
    /// Create new sound test manager
    pub fn new(config: SoundTestConfig) -> Self {
        Self {
            config,
            category_manager: SoundCategoryManager::new(),
            playback_controls: PlaybackControls::new(),
            waveform_display: WaveformDisplay::new(),
            sound_library: SoundLibrary::new(),
            playing_sounds: HashMap::new(),
            state: SoundTestState::Stopped,
            event_handlers: Vec::new(),
        }
    }

    /// Update sound test manager
    pub fn update(&mut self, delta_time: f32) -> SoundTestResult<()> {
        // Update playback controls
        self.playback_controls.update(delta_time);
        
        // Update waveform display
        self.waveform_display.update(delta_time);
        
        // Update playing sounds
        self.update_playing_sounds(delta_time)?;
        
        // Update sound test state
        self.update_sound_test_state()?;
        
        Ok(())
    }

    /// Play sound
    pub fn play_sound(&mut self, sound_id: &str) -> SoundTestResult<()> {
        if !self.config.enabled {
            return Err(SoundTestError::NotInitialized);
        }

        let sound_entry = self.sound_library.get_sound(sound_id)
            .ok_or_else(|| SoundTestError::SoundNotFound(sound_id.to_string()))?;

        // Check if already playing
        if self.playing_sounds.contains_key(sound_id) {
            return Ok(());
        }

        // Check maximum simultaneous sounds
        if self.playing_sounds.len() >= self.config.max_simultaneous_sounds {
            return Err(SoundTestError::MaximumSoundsExceeded);
        }

        // Get category info for default values
        let category_info = self.category_manager.get_category_info(&sound_entry.category)
            .unwrap_or_else(|| panic!("Category info not found for category: {:?}", sound_entry.category));

        // Create playing sound
        let playing_sound = PlayingSound {
            sound_id: sound_id.to_string(),
            category: sound_entry.category.clone(),
            start_time: 0.0,
            current_time: 0.0,
            duration: sound_entry.duration,
            volume: category_info.default_volume,
            pitch: category_info.default_pitch,
            pan: category_info.default_pan,
            playback_speed: 1.0,
            is_looping: category_info.enable_looping,
            is_paused: false,
            is_muted: false,
            fade_in_time: 0.0,
            fade_out_time: 0.0,
            fade_in_progress: 0.0,
            fade_out_progress: 0.0,
        };

        self.playing_sounds.insert(sound_id.to_string(), playing_sound);

        // Update play count
        self.sound_library.update_play_count(sound_id);

        // Emit event
        self.emit_event(SoundTestEvent::SoundStarted {
            sound_id: sound_id.to_string(),
            category: sound_entry.category.clone(),
        });

        Ok(())
    }

    /// Stop sound
    pub fn stop_sound(&mut self, sound_id: &str) -> SoundTestResult<()> {
        if let Some(playing_sound) = self.playing_sounds.remove(sound_id) {
            // Emit event
            self.emit_event(SoundTestEvent::SoundStopped {
                sound_id: sound_id.to_string(),
                category: playing_sound.category,
            });
        }

        Ok(())
    }

    /// Pause sound
    pub fn pause_sound(&mut self, sound_id: &str) -> SoundTestResult<()> {
        if let Some(playing_sound) = self.playing_sounds.get_mut(sound_id) {
            playing_sound.is_paused = true;
            
            // Emit event
            self.emit_event(SoundTestEvent::SoundPaused {
                sound_id: sound_id.to_string(),
                category: playing_sound.category.clone(),
            });
        }

        Ok(())
    }

    /// Resume sound
    pub fn resume_sound(&mut self, sound_id: &str) -> SoundTestResult<()> {
        if let Some(playing_sound) = self.playing_sounds.get_mut(sound_id) {
            playing_sound.is_paused = false;
            
            // Emit event
            self.emit_event(SoundTestEvent::SoundResumed {
                sound_id: sound_id.to_string(),
                category: playing_sound.category.clone(),
            });
        }

        Ok(())
    }

    /// Stop all sounds
    pub fn stop_all_sounds(&mut self) -> SoundTestResult<()> {
        let sound_ids: Vec<String> = self.playing_sounds.keys().cloned().collect();
        
        for sound_id in sound_ids {
            self.stop_sound(&sound_id)?;
        }

        Ok(())
    }

    /// Set sound volume
    pub fn set_sound_volume(&mut self, sound_id: &str, volume: f32) -> SoundTestResult<()> {
        if let Some(playing_sound) = self.playing_sounds.get_mut(sound_id) {
            let old_volume = playing_sound.volume;
            playing_sound.volume = volume.max(0.0).min(1.0);
            
            // Emit event
            self.emit_event(SoundTestEvent::VolumeChanged {
                sound_id: sound_id.to_string(),
                old_volume,
                new_volume: playing_sound.volume,
            });
        }

        Ok(())
    }

    /// Set sound pitch
    pub fn set_sound_pitch(&mut self, sound_id: &str, pitch: f32) -> SoundTestResult<()> {
        if let Some(playing_sound) = self.playing_sounds.get_mut(sound_id) {
            let old_pitch = playing_sound.pitch;
            playing_sound.pitch = pitch.max(0.1).min(4.0);
            
            // Emit event
            self.emit_event(SoundTestEvent::PitchChanged {
                sound_id: sound_id.to_string(),
                old_pitch,
                new_pitch: playing_sound.pitch,
            });
        }

        Ok(())
    }

    /// Set sound pan
    pub fn set_sound_pan(&mut self, sound_id: &str, pan: f32) -> SoundTestResult<()> {
        if let Some(playing_sound) = self.playing_sounds.get_mut(sound_id) {
            let old_pan = playing_sound.pan;
            playing_sound.pan = pan.max(-1.0).min(1.0);
            
            // Emit event
            self.emit_event(SoundTestEvent::PanChanged {
                sound_id: sound_id.to_string(),
                old_pan,
                new_pan: playing_sound.pan,
            });
        }

        Ok(())
    }

    /// Set sound looping
    pub fn set_sound_looping(&mut self, sound_id: &str, looping: bool) -> SoundTestResult<()> {
        if let Some(playing_sound) = self.playing_sounds.get_mut(sound_id) {
            playing_sound.is_looping = looping;
        }

        Ok(())
    }

    /// Set sound muted
    pub fn set_sound_muted(&mut self, sound_id: &str, muted: bool) -> SoundTestResult<()> {
        if let Some(playing_sound) = self.playing_sounds.get_mut(sound_id) {
            playing_sound.is_muted = muted;
        }

        Ok(())
    }

    /// Set current category
    pub fn set_current_category(&mut self, category: SoundCategory) -> SoundTestResult<()> {
        let old_category = self.category_manager.get_current_category().clone();
        self.category_manager.set_current_category(category.clone());
        
        // Emit event
        self.emit_event(SoundTestEvent::CategoryChanged {
            old_category,
            new_category: category,
        });

        Ok(())
    }

    /// Get sounds for current category
    pub fn get_sounds_for_current_category(&self) -> Vec<&SoundEntry> {
        let current_category = self.category_manager.get_current_category();
        self.sound_library.get_sounds_by_category(current_category)
    }

    /// Search sounds
    pub fn search_sounds(&self, query: &str) -> Vec<&SoundEntry> {
        self.sound_library.search_sounds(query)
    }

    /// Get playing sounds
    pub fn get_playing_sounds(&self) -> Vec<&PlayingSound> {
        self.playing_sounds.values().collect()
    }

    /// Get sound test state
    pub fn get_sound_test_state(&self) -> &SoundTestState {
        &self.state
    }

    /// Start sound test
    pub fn start_sound_test(&mut self) -> SoundTestResult<()> {
        self.state = SoundTestState::Playing;
        
        // Emit event
        self.emit_event(SoundTestEvent::SoundTestStarted);
        
        Ok(())
    }

    /// Stop sound test
    pub fn stop_sound_test(&mut self) -> SoundTestResult<()> {
        self.state = SoundTestState::Stopped;
        self.stop_all_sounds()?;
        
        // Emit event
        self.emit_event(SoundTestEvent::SoundTestStopped);
        
        Ok(())
    }

    /// Pause sound test
    pub fn pause_sound_test(&mut self) -> SoundTestResult<()> {
        self.state = SoundTestState::Paused;
        
        // Pause all playing sounds
        for sound_id in self.playing_sounds.keys() {
            self.pause_sound(sound_id)?;
        }
        
        // Emit event
        self.emit_event(SoundTestEvent::SoundTestPaused);
        
        Ok(())
    }

    /// Resume sound test
    pub fn resume_sound_test(&mut self) -> SoundTestResult<()> {
        self.state = SoundTestState::Playing;
        
        // Resume all paused sounds
        for sound_id in self.playing_sounds.keys() {
            self.resume_sound(sound_id)?;
        }
        
        // Emit event
        self.emit_event(SoundTestEvent::SoundTestResumed);
        
        Ok(())
    }

    /// Reset sound test
    pub fn reset_sound_test(&mut self) -> SoundTestResult<()> {
        self.stop_all_sounds()?;
        self.playback_controls.reset();
        self.waveform_display.reset();
        self.state = SoundTestState::Stopped;
        
        // Emit event
        self.emit_event(SoundTestEvent::SoundTestReset);
        
        Ok(())
    }

    /// Export sound test
    pub fn export_sound_test(&mut self, file_path: &str) -> SoundTestResult<()> {
        self.state = SoundTestState::Exporting;
        
        // In a real implementation, this would export the sound test configuration
        // For now, we'll just simulate the export
        
        self.state = SoundTestState::Stopped;
        
        // Emit event
        self.emit_event(SoundTestEvent::SoundTestExported {
            file_path: file_path.to_string(),
        });
        
        Ok(())
    }

    /// Import sound test
    pub fn import_sound_test(&mut self, file_path: &str) -> SoundTestResult<()> {
        self.state = SoundTestState::Importing;
        
        // In a real implementation, this would import the sound test configuration
        // For now, we'll just simulate the import
        
        self.state = SoundTestState::Stopped;
        
        // Emit event
        self.emit_event(SoundTestEvent::SoundTestImported {
            file_path: file_path.to_string(),
        });
        
        Ok(())
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&SoundTestEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Update playing sounds
    fn update_playing_sounds(&mut self, delta_time: f32) -> SoundTestResult<()> {
        let mut sounds_to_remove = Vec::new();
        
        for (sound_id, playing_sound) in self.playing_sounds.iter_mut() {
            if !playing_sound.is_paused {
                // Update current time
                playing_sound.current_time += delta_time * playing_sound.playback_speed;
                
                // Check if finished
                if playing_sound.current_time >= playing_sound.duration {
                    if playing_sound.is_looping {
                        playing_sound.current_time = 0.0;
                        
                        // Emit loop event
                        self.emit_event(SoundTestEvent::SoundLooped {
                            sound_id: sound_id.clone(),
                            category: playing_sound.category.clone(),
                        });
                    } else {
                        sounds_to_remove.push(sound_id.clone());
                    }
                }
            }
        }
        
        // Remove finished sounds
        for sound_id in sounds_to_remove {
            self.stop_sound(&sound_id)?;
        }
        
        Ok(())
    }

    /// Update sound test state
    fn update_sound_test_state(&mut self) -> SoundTestResult<()> {
        if self.playing_sounds.is_empty() {
            if self.state == SoundTestState::Playing {
                self.state = SoundTestState::Stopped;
            }
        } else {
            if self.state == SoundTestState::Stopped {
                self.state = SoundTestState::Playing;
            }
        }
        
        Ok(())
    }

    /// Emit sound test event
    fn emit_event(&self, event: SoundTestEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for SoundTestManager {
    fn default() -> Self {
        Self::new(SoundTestConfig::default())
    }
}
