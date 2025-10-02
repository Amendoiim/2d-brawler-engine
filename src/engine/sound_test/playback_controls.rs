//! Playback Controls
//! 
//! This module provides playback controls for the sound test system.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Playback controls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackControls {
    /// Play button state
    pub play_button: ButtonState,
    /// Pause button state
    pub pause_button: ButtonState,
    /// Stop button state
    pub stop_button: ButtonState,
    /// Loop button state
    pub loop_button: ButtonState,
    /// Volume slider
    pub volume_slider: SliderControl,
    /// Pitch slider
    pub pitch_slider: SliderControl,
    /// Pan slider
    pub pan_slider: SliderControl,
    /// Playback speed slider
    pub speed_slider: SliderControl,
    /// Position slider
    pub position_slider: SliderControl,
    /// Current time
    pub current_time: f32,
    /// Total time
    pub total_time: f32,
    /// Is playing
    pub is_playing: bool,
    /// Is paused
    pub is_paused: bool,
    /// Is looping
    pub is_looping: bool,
    /// Is muted
    pub is_muted: bool,
    /// Volume
    pub volume: f32,
    /// Pitch
    pub pitch: f32,
    /// Pan
    pub pan: f32,
    /// Playback speed
    pub playback_speed: f32,
    /// Position
    pub position: f32,
    /// Duration
    pub duration: f32,
    /// Fade in time
    pub fade_in_time: f32,
    /// Fade out time
    pub fade_out_time: f32,
    /// Crossfade time
    pub crossfade_time: f32,
    /// Enable effects
    pub enable_effects: bool,
    /// Enable spatial audio
    pub enable_spatial_audio: bool,
}

/// Button state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonState {
    /// Button pressed
    pub pressed: bool,
    /// Button enabled
    pub enabled: bool,
    /// Button visible
    pub visible: bool,
    /// Button text
    pub text: String,
    /// Button icon
    pub icon: String,
    /// Button tooltip
    pub tooltip: String,
    /// Button color
    pub color: (f32, f32, f32, f32),
    /// Button hover color
    pub hover_color: (f32, f32, f32, f32),
    /// Button pressed color
    pub pressed_color: (f32, f32, f32, f32),
    /// Button disabled color
    pub disabled_color: (f32, f32, f32, f32),
}

/// Slider control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliderControl {
    /// Slider value
    pub value: f32,
    /// Minimum value
    pub min_value: f32,
    /// Maximum value
    pub max_value: f32,
    /// Step size
    pub step_size: f32,
    /// Slider enabled
    pub enabled: bool,
    /// Slider visible
    pub visible: bool,
    /// Slider label
    pub label: String,
    /// Slider unit
    pub unit: String,
    /// Slider format
    pub format: String,
    /// Slider color
    pub color: (f32, f32, f32, f32),
    /// Slider track color
    pub track_color: (f32, f32, f32, f32),
    /// Slider handle color
    pub handle_color: (f32, f32, f32, f32),
    /// Slider disabled color
    pub disabled_color: (f32, f32, f32, f32),
}

impl PlaybackControls {
    /// Create new playback controls
    pub fn new() -> Self {
        Self {
            play_button: ButtonState {
                pressed: false,
                enabled: true,
                visible: true,
                text: "Play".to_string(),
                icon: "play".to_string(),
                tooltip: "Play sound".to_string(),
                color: (0.0, 0.8, 0.0, 1.0), // Green
                hover_color: (0.0, 1.0, 0.0, 1.0),
                pressed_color: (0.0, 0.6, 0.0, 1.0),
                disabled_color: (0.5, 0.5, 0.5, 1.0),
            },
            pause_button: ButtonState {
                pressed: false,
                enabled: false,
                visible: true,
                text: "Pause".to_string(),
                icon: "pause".to_string(),
                tooltip: "Pause sound".to_string(),
                color: (1.0, 0.8, 0.0, 1.0), // Orange
                hover_color: (1.0, 1.0, 0.0, 1.0),
                pressed_color: (0.8, 0.6, 0.0, 1.0),
                disabled_color: (0.5, 0.5, 0.5, 1.0),
            },
            stop_button: ButtonState {
                pressed: false,
                enabled: false,
                visible: true,
                text: "Stop".to_string(),
                icon: "stop".to_string(),
                tooltip: "Stop sound".to_string(),
                color: (1.0, 0.0, 0.0, 1.0), // Red
                hover_color: (1.0, 0.2, 0.2, 1.0),
                pressed_color: (0.8, 0.0, 0.0, 1.0),
                disabled_color: (0.5, 0.5, 0.5, 1.0),
            },
            loop_button: ButtonState {
                pressed: false,
                enabled: true,
                visible: true,
                text: "Loop".to_string(),
                icon: "loop".to_string(),
                tooltip: "Toggle looping".to_string(),
                color: (0.0, 0.0, 1.0, 1.0), // Blue
                hover_color: (0.2, 0.2, 1.0, 1.0),
                pressed_color: (0.0, 0.0, 0.8, 1.0),
                disabled_color: (0.5, 0.5, 0.5, 1.0),
            },
            volume_slider: SliderControl {
                value: 0.8,
                min_value: 0.0,
                max_value: 1.0,
                step_size: 0.01,
                enabled: true,
                visible: true,
                label: "Volume".to_string(),
                unit: "%".to_string(),
                format: "{:.0}".to_string(),
                color: (0.0, 0.8, 0.0, 1.0),
                track_color: (0.3, 0.3, 0.3, 1.0),
                handle_color: (0.0, 1.0, 0.0, 1.0),
                disabled_color: (0.5, 0.5, 0.5, 1.0),
            },
            pitch_slider: SliderControl {
                value: 1.0,
                min_value: 0.1,
                max_value: 4.0,
                step_size: 0.01,
                enabled: true,
                visible: true,
                label: "Pitch".to_string(),
                unit: "x".to_string(),
                format: "{:.2}".to_string(),
                color: (0.8, 0.0, 0.8, 1.0),
                track_color: (0.3, 0.3, 0.3, 1.0),
                handle_color: (1.0, 0.0, 1.0, 1.0),
                disabled_color: (0.5, 0.5, 0.5, 1.0),
            },
            pan_slider: SliderControl {
                value: 0.0,
                min_value: -1.0,
                max_value: 1.0,
                step_size: 0.01,
                enabled: true,
                visible: true,
                label: "Pan".to_string(),
                unit: "L/R".to_string(),
                format: "{:.2}".to_string(),
                color: (0.0, 0.8, 0.8, 1.0),
                track_color: (0.3, 0.3, 0.3, 1.0),
                handle_color: (0.0, 1.0, 1.0, 1.0),
                disabled_color: (0.5, 0.5, 0.5, 1.0),
            },
            speed_slider: SliderControl {
                value: 1.0,
                min_value: 0.25,
                max_value: 4.0,
                step_size: 0.01,
                enabled: true,
                visible: true,
                label: "Speed".to_string(),
                unit: "x".to_string(),
                format: "{:.2}".to_string(),
                color: (1.0, 0.8, 0.0, 1.0),
                track_color: (0.3, 0.3, 0.3, 1.0),
                handle_color: (1.0, 1.0, 0.0, 1.0),
                disabled_color: (0.5, 0.5, 0.5, 1.0),
            },
            position_slider: SliderControl {
                value: 0.0,
                min_value: 0.0,
                max_value: 1.0,
                step_size: 0.001,
                enabled: false,
                visible: true,
                label: "Position".to_string(),
                unit: "s".to_string(),
                format: "{:.2}".to_string(),
                color: (0.8, 0.8, 0.8, 1.0),
                track_color: (0.3, 0.3, 0.3, 1.0),
                handle_color: (1.0, 1.0, 1.0, 1.0),
                disabled_color: (0.5, 0.5, 0.5, 1.0),
            },
            current_time: 0.0,
            total_time: 0.0,
            is_playing: false,
            is_paused: false,
            is_looping: false,
            is_muted: false,
            volume: 0.8,
            pitch: 1.0,
            pan: 0.0,
            playback_speed: 1.0,
            position: 0.0,
            duration: 0.0,
            fade_in_time: 0.0,
            fade_out_time: 0.0,
            crossfade_time: 0.0,
            enable_effects: true,
            enable_spatial_audio: true,
        }
    }

    /// Update playback controls
    pub fn update(&mut self, delta_time: f32) {
        // Update position if playing
        if self.is_playing && !self.is_paused {
            self.position += delta_time * self.playback_speed;
            self.current_time = self.position;
            
            // Update position slider
            if self.duration > 0.0 {
                self.position_slider.value = self.position / self.duration;
            }
            
            // Check if finished
            if self.position >= self.duration {
                if self.is_looping {
                    self.position = 0.0;
                    self.current_time = 0.0;
                } else {
                    self.stop();
                }
            }
        }
        
        // Update button states
        self.update_button_states();
    }

    /// Play sound
    pub fn play(&mut self) {
        self.is_playing = true;
        self.is_paused = false;
        self.play_button.pressed = true;
        self.pause_button.pressed = false;
        self.stop_button.pressed = false;
    }

    /// Pause sound
    pub fn pause(&mut self) {
        self.is_paused = true;
        self.is_playing = false;
        self.play_button.pressed = false;
        self.pause_button.pressed = true;
        self.stop_button.pressed = false;
    }

    /// Stop sound
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.is_paused = false;
        self.position = 0.0;
        self.current_time = 0.0;
        self.play_button.pressed = false;
        self.pause_button.pressed = false;
        self.stop_button.pressed = true;
    }

    /// Toggle loop
    pub fn toggle_loop(&mut self) {
        self.is_looping = !self.is_looping;
        self.loop_button.pressed = self.is_looping;
    }

    /// Set volume
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.max(0.0).min(1.0);
        self.volume_slider.value = self.volume;
    }

    /// Set pitch
    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.max(0.1).min(4.0);
        self.pitch_slider.value = self.pitch;
    }

    /// Set pan
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan.max(-1.0).min(1.0);
        self.pan_slider.value = self.pan;
    }

    /// Set playback speed
    pub fn set_playback_speed(&mut self, speed: f32) {
        self.playback_speed = speed.max(0.25).min(4.0);
        self.speed_slider.value = self.playback_speed;
    }

    /// Set position
    pub fn set_position(&mut self, position: f32) {
        self.position = position.max(0.0).min(self.duration);
        self.current_time = self.position;
        if self.duration > 0.0 {
            self.position_slider.value = self.position / self.duration;
        }
    }

    /// Set duration
    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration.max(0.0);
        self.total_time = duration;
        self.position_slider.max_value = duration;
    }

    /// Toggle mute
    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }

    /// Reset controls
    pub fn reset(&mut self) {
        self.stop();
        self.set_volume(0.8);
        self.set_pitch(1.0);
        self.set_pan(0.0);
        self.set_playback_speed(1.0);
        self.set_position(0.0);
        self.is_looping = false;
        self.is_muted = false;
    }

    /// Update button states
    fn update_button_states(&mut self) {
        // Play button
        self.play_button.enabled = !self.is_playing || self.is_paused;
        
        // Pause button
        self.pause_button.enabled = self.is_playing && !self.is_paused;
        
        // Stop button
        self.stop_button.enabled = self.is_playing || self.is_paused;
        
        // Loop button
        self.loop_button.enabled = true;
        
        // Position slider
        self.position_slider.enabled = self.is_playing || self.is_paused;
    }

    /// Get formatted time string
    pub fn get_time_string(&self, time: f32) -> String {
        let minutes = (time / 60.0) as u32;
        let seconds = (time % 60.0) as u32;
        let milliseconds = ((time % 1.0) * 1000.0) as u32;
        format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
    }

    /// Get current time string
    pub fn get_current_time_string(&self) -> String {
        self.get_time_string(self.current_time)
    }

    /// Get total time string
    pub fn get_total_time_string(&self) -> String {
        self.get_time_string(self.total_time)
    }

    /// Get volume percentage
    pub fn get_volume_percentage(&self) -> u32 {
        (self.volume * 100.0) as u32
    }

    /// Get pitch multiplier
    pub fn get_pitch_multiplier(&self) -> f32 {
        self.pitch
    }

    /// Get pan value
    pub fn get_pan_value(&self) -> f32 {
        self.pan
    }

    /// Get speed multiplier
    pub fn get_speed_multiplier(&self) -> f32 {
        self.playback_speed
    }

    /// Get position percentage
    pub fn get_position_percentage(&self) -> u32 {
        if self.duration > 0.0 {
            ((self.position / self.duration) * 100.0) as u32
        } else {
            0
        }
    }
}

impl Default for PlaybackControls {
    fn default() -> Self {
        Self::new()
    }
}
