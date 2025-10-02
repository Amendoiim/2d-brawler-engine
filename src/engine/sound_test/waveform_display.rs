//! Waveform Display
//! 
//! This module provides waveform visualization for the sound test system.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Waveform display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveformDisplay {
    /// Waveform data
    pub waveform_data: Vec<f32>,
    /// Display width
    pub display_width: u32,
    /// Display height
    pub display_height: u32,
    /// Sample rate
    pub sample_rate: u32,
    /// Time window (seconds)
    pub time_window: f32,
    /// Amplitude scale
    pub amplitude_scale: f32,
    /// Color scheme
    pub color_scheme: ColorScheme,
    /// Display mode
    pub display_mode: DisplayMode,
    /// Show grid
    pub show_grid: bool,
    /// Show labels
    pub show_labels: bool,
    /// Show cursor
    pub show_cursor: bool,
    /// Cursor position
    pub cursor_position: f32,
    /// Zoom level
    pub zoom_level: f32,
    /// Scroll position
    pub scroll_position: f32,
    /// Real-time mode
    pub real_time_mode: bool,
    /// Peak hold
    pub peak_hold: bool,
    /// Peak hold time
    pub peak_hold_time: f32,
    /// Peak data
    pub peak_data: Vec<f32>,
    /// RMS data
    pub rms_data: Vec<f32>,
    /// Spectrum data
    pub spectrum_data: Vec<f32>,
    /// Frequency bins
    pub frequency_bins: Vec<f32>,
    /// Update rate
    pub update_rate: f32,
    /// Last update time
    pub last_update_time: f32,
}

/// Color scheme
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColorScheme {
    /// Classic green
    Classic,
    /// Blue
    Blue,
    /// Red
    Red,
    /// Purple
    Purple,
    /// Rainbow
    Rainbow,
    /// Monochrome
    Monochrome,
    /// Custom
    Custom {
        background: (f32, f32, f32, f32),
        waveform: (f32, f32, f32, f32),
        grid: (f32, f32, f32, f32),
        cursor: (f32, f32, f32, f32),
        peak: (f32, f32, f32, f32),
        rms: (f32, f32, f32, f32),
    },
}

/// Display mode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisplayMode {
    /// Waveform
    Waveform,
    /// Spectrum
    Spectrum,
    /// Both
    Both,
    /// 3D
    ThreeD,
    /// Oscilloscope
    Oscilloscope,
}

impl WaveformDisplay {
    /// Create new waveform display
    pub fn new() -> Self {
        Self {
            waveform_data: Vec::new(),
            display_width: 800,
            display_height: 200,
            sample_rate: 44100,
            time_window: 1.0,
            amplitude_scale: 1.0,
            color_scheme: ColorScheme::Classic,
            display_mode: DisplayMode::Waveform,
            show_grid: true,
            show_labels: true,
            show_cursor: true,
            cursor_position: 0.0,
            zoom_level: 1.0,
            scroll_position: 0.0,
            real_time_mode: false,
            peak_hold: false,
            peak_hold_time: 0.5,
            peak_data: Vec::new(),
            rms_data: Vec::new(),
            spectrum_data: Vec::new(),
            frequency_bins: Vec::new(),
            update_rate: 60.0,
            last_update_time: 0.0,
        }
    }

    /// Update waveform display
    pub fn update(&mut self, delta_time: f32) {
        self.last_update_time += delta_time;
        
        if self.last_update_time >= 1.0 / self.update_rate {
            self.last_update_time = 0.0;
            
            if self.real_time_mode {
                self.update_real_time();
            }
            
            if self.peak_hold {
                self.update_peak_hold();
            }
        }
    }

    /// Set waveform data
    pub fn set_waveform_data(&mut self, data: Vec<f32>) {
        self.waveform_data = data;
        self.update_display_data();
    }

    /// Add sample to real-time display
    pub fn add_sample(&mut self, sample: f32) {
        if self.real_time_mode {
            self.waveform_data.push(sample);
            
            // Keep only the last N samples based on time window
            let max_samples = (self.sample_rate as f32 * self.time_window) as usize;
            if self.waveform_data.len() > max_samples {
                self.waveform_data.remove(0);
            }
        }
    }

    /// Set cursor position
    pub fn set_cursor_position(&mut self, position: f32) {
        self.cursor_position = position.max(0.0).min(1.0);
    }

    /// Set zoom level
    pub fn set_zoom_level(&mut self, zoom: f32) {
        self.zoom_level = zoom.max(0.1).min(10.0);
    }

    /// Set scroll position
    pub fn set_scroll_position(&mut self, position: f32) {
        self.scroll_position = position.max(0.0).min(1.0);
    }

    /// Set display mode
    pub fn set_display_mode(&mut self, mode: DisplayMode) {
        self.display_mode = mode;
        self.update_display_data();
    }

    /// Set color scheme
    pub fn set_color_scheme(&mut self, scheme: ColorScheme) {
        self.color_scheme = scheme;
    }

    /// Toggle grid
    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    /// Toggle labels
    pub fn toggle_labels(&mut self) {
        self.show_labels = !self.show_labels;
    }

    /// Toggle cursor
    pub fn toggle_cursor(&mut self) {
        self.show_cursor = !self.show_cursor;
    }

    /// Toggle real-time mode
    pub fn toggle_real_time_mode(&mut self) {
        self.real_time_mode = !self.real_time_mode;
        if self.real_time_mode {
            self.waveform_data.clear();
        }
    }

    /// Toggle peak hold
    pub fn toggle_peak_hold(&mut self) {
        self.peak_hold = !self.peak_hold;
        if !self.peak_hold {
            self.peak_data.clear();
        }
    }

    /// Reset display
    pub fn reset(&mut self) {
        self.waveform_data.clear();
        self.peak_data.clear();
        self.rms_data.clear();
        self.spectrum_data.clear();
        self.cursor_position = 0.0;
        self.scroll_position = 0.0;
        self.zoom_level = 1.0;
    }

    /// Get visible samples
    pub fn get_visible_samples(&self) -> Vec<f32> {
        let start_index = (self.scroll_position * self.waveform_data.len() as f32) as usize;
        let end_index = ((self.scroll_position + 1.0 / self.zoom_level) * self.waveform_data.len() as f32) as usize;
        
        self.waveform_data[start_index..end_index.min(self.waveform_data.len())].to_vec()
    }

    /// Get cursor sample index
    pub fn get_cursor_sample_index(&self) -> usize {
        (self.cursor_position * self.waveform_data.len() as f32) as usize
    }

    /// Get cursor time
    pub fn get_cursor_time(&self) -> f32 {
        self.cursor_position * self.time_window
    }

    /// Get amplitude at cursor
    pub fn get_amplitude_at_cursor(&self) -> f32 {
        let index = self.get_cursor_sample_index();
        if index < self.waveform_data.len() {
            self.waveform_data[index]
        } else {
            0.0
        }
    }

    /// Get peak amplitude
    pub fn get_peak_amplitude(&self) -> f32 {
        self.waveform_data.iter().fold(0.0, |acc, &x| acc.max(x.abs()))
    }

    /// Get RMS amplitude
    pub fn get_rms_amplitude(&self) -> f32 {
        if self.waveform_data.is_empty() {
            return 0.0;
        }
        
        let sum_squares: f32 = self.waveform_data.iter().map(|x| x * x).sum();
        (sum_squares / self.waveform_data.len() as f32).sqrt()
    }

    /// Get frequency at cursor
    pub fn get_frequency_at_cursor(&self) -> f32 {
        let index = self.get_cursor_sample_index();
        if index < self.frequency_bins.len() {
            self.frequency_bins[index]
        } else {
            0.0
        }
    }

    /// Get spectrum amplitude at cursor
    pub fn get_spectrum_amplitude_at_cursor(&self) -> f32 {
        let index = self.get_cursor_sample_index();
        if index < self.spectrum_data.len() {
            self.spectrum_data[index]
        } else {
            0.0
        }
    }

    /// Update real-time display
    fn update_real_time(&mut self) {
        // In a real implementation, this would update the display with new audio data
        // For now, we'll just simulate the update
    }

    /// Update peak hold
    fn update_peak_hold(&mut self) {
        // Update peak data with current waveform data
        if self.peak_data.len() != self.waveform_data.len() {
            self.peak_data.resize(self.waveform_data.len(), 0.0);
        }
        
        for (i, &sample) in self.waveform_data.iter().enumerate() {
            self.peak_data[i] = self.peak_data[i].max(sample.abs());
        }
    }

    /// Update display data
    fn update_display_data(&mut self) {
        match self.display_mode {
            DisplayMode::Waveform => {
                // Calculate RMS data for waveform display
                self.calculate_rms_data();
            },
            DisplayMode::Spectrum => {
                // Calculate spectrum data
                self.calculate_spectrum_data();
            },
            DisplayMode::Both => {
                // Calculate both RMS and spectrum data
                self.calculate_rms_data();
                self.calculate_spectrum_data();
            },
            DisplayMode::ThreeD => {
                // Calculate 3D display data
                self.calculate_3d_data();
            },
            DisplayMode::Oscilloscope => {
                // Calculate oscilloscope data
                self.calculate_oscilloscope_data();
            },
        }
    }

    /// Calculate RMS data
    fn calculate_rms_data(&mut self) {
        if self.waveform_data.is_empty() {
            return;
        }
        
        let window_size = (self.sample_rate as f32 * 0.01) as usize; // 10ms window
        self.rms_data.clear();
        
        for i in 0..self.waveform_data.len() {
            let start = i.saturating_sub(window_size / 2);
            let end = (i + window_size / 2).min(self.waveform_data.len());
            
            let sum_squares: f32 = self.waveform_data[start..end].iter()
                .map(|x| x * x)
                .sum();
            
            let rms = (sum_squares / (end - start) as f32).sqrt();
            self.rms_data.push(rms);
        }
    }

    /// Calculate spectrum data
    fn calculate_spectrum_data(&mut self) {
        if self.waveform_data.is_empty() {
            return;
        }
        
        // Simple FFT simulation (in a real implementation, you'd use a proper FFT library)
        let fft_size = 1024;
        let mut spectrum = vec![0.0; fft_size / 2];
        
        for i in 0..spectrum.len() {
            let frequency = (i as f32 * self.sample_rate as f32) / fft_size as f32;
            self.frequency_bins.push(frequency);
            
            // Simple spectrum calculation (not a real FFT)
            let mut magnitude = 0.0;
            for j in 0..self.waveform_data.len() {
                let phase = 2.0 * std::f32::consts::PI * frequency * j as f32 / self.sample_rate as f32;
                magnitude += self.waveform_data[j] * phase.cos();
            }
            
            spectrum[i] = magnitude.abs();
        }
        
        self.spectrum_data = spectrum;
    }

    /// Calculate 3D data
    fn calculate_3d_data(&mut self) {
        // In a real implementation, this would calculate 3D visualization data
        // For now, we'll just use the waveform data
        self.spectrum_data = self.waveform_data.clone();
    }

    /// Calculate oscilloscope data
    fn calculate_oscilloscope_data(&mut self) {
        // In a real implementation, this would calculate oscilloscope display data
        // For now, we'll just use the waveform data
        self.rms_data = self.waveform_data.clone();
    }
}

impl Default for WaveformDisplay {
    fn default() -> Self {
        Self::new()
    }
}
