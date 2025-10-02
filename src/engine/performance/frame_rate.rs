//! Frame Rate Management
//! 
//! This module manages frame rate optimization, limiting, and adaptive quality.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use super::{PerformanceResult, PerformanceError, PerformanceEvent};

/// Frame rate manager
pub struct FrameRateManager {
    /// Target FPS
    pub target_fps: u32,
    /// Current FPS
    pub current_fps: f32,
    /// Average FPS over last second
    pub average_fps: f32,
    /// Frame time in milliseconds
    pub frame_time: f32,
    /// Frame time history
    pub frame_times: Vec<f32>,
    /// Last frame time
    pub last_frame_time: Instant,
    /// Frame count
    pub frame_count: u64,
    /// FPS counter reset time
    pub fps_reset_time: Instant,
    /// Frame rate limiting enabled
    pub limiting_enabled: bool,
    /// Adaptive quality enabled
    pub adaptive_quality: bool,
    /// Low FPS threshold
    pub low_fps_threshold: f32,
    /// High FPS threshold
    pub high_fps_threshold: f32,
    /// Frame rate smoothing factor
    pub smoothing_factor: f32,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&PerformanceEvent) + Send + Sync>>,
}

impl FrameRateManager {
    /// Create a new frame rate manager
    pub fn new(target_fps: u32) -> Self {
        Self {
            target_fps,
            current_fps: target_fps as f32,
            average_fps: target_fps as f32,
            frame_time: 1000.0 / target_fps as f32,
            frame_times: Vec::new(),
            last_frame_time: Instant::now(),
            frame_count: 0,
            fps_reset_time: Instant::now(),
            limiting_enabled: true,
            adaptive_quality: true,
            low_fps_threshold: target_fps as f32 * 0.8,
            high_fps_threshold: target_fps as f32 * 1.2,
            smoothing_factor: 0.1,
            event_handlers: Vec::new(),
        }
    }

    /// Update frame rate manager
    pub fn update(&mut self) -> PerformanceResult<()> {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_frame_time);
        let frame_time_ms = delta_time.as_secs_f32() * 1000.0;

        // Update frame time
        self.frame_time = frame_time_ms;
        self.frame_times.push(frame_time_ms);

        // Keep only last 60 frames for averaging
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }

        // Update frame count
        self.frame_count += 1;

        // Calculate current FPS
        self.current_fps = 1000.0 / frame_time_ms;

        // Update average FPS every second
        if now.duration_since(self.fps_reset_time) >= Duration::from_secs(1) {
            self.average_fps = self.calculate_average_fps();
            self.fps_reset_time = now;
            self.frame_count = 0;
        }

        // Apply frame rate limiting
        if self.limiting_enabled {
            self.apply_frame_rate_limiting()?;
        }

        // Check for performance events
        self.check_performance_events()?;

        self.last_frame_time = now;
        Ok(())
    }

    /// Set target FPS
    pub fn set_target_fps(&mut self, fps: u32) -> PerformanceResult<()> {
        if fps == 0 {
            return Err(PerformanceError::InvalidConfig("Target FPS cannot be zero".to_string()));
        }

        self.target_fps = fps;
        self.low_fps_threshold = fps as f32 * 0.8;
        self.high_fps_threshold = fps as f32 * 1.2;
        Ok(())
    }

    /// Get current FPS
    pub fn get_current_fps(&self) -> f32 {
        self.current_fps
    }

    /// Get average FPS
    pub fn get_average_fps(&self) -> f32 {
        self.average_fps
    }

    /// Get frame time
    pub fn get_frame_time(&self) -> f32 {
        self.frame_time
    }

    /// Enable/disable frame rate limiting
    pub fn set_limiting_enabled(&mut self, enabled: bool) {
        self.limiting_enabled = enabled;
    }

    /// Enable/disable adaptive quality
    pub fn set_adaptive_quality(&mut self, enabled: bool) {
        self.adaptive_quality = enabled;
    }

    /// Set FPS thresholds
    pub fn set_thresholds(&mut self, low: f32, high: f32) -> PerformanceResult<()> {
        if low >= high {
            return Err(PerformanceError::InvalidConfig("Low threshold must be less than high threshold".to_string()));
        }

        self.low_fps_threshold = low;
        self.high_fps_threshold = high;
        Ok(())
    }

    /// Set smoothing factor
    pub fn set_smoothing_factor(&mut self, factor: f32) -> PerformanceResult<()> {
        if factor < 0.0 || factor > 1.0 {
            return Err(PerformanceError::InvalidConfig("Smoothing factor must be between 0.0 and 1.0".to_string()));
        }

        self.smoothing_factor = factor;
        Ok(())
    }

    /// Get frame rate stability
    pub fn get_stability(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 1.0;
        }

        let mean = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
        let variance = self.frame_times.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f32>() / self.frame_times.len() as f32;
        let std_dev = variance.sqrt();

        // Stability is inverse of coefficient of variation
        if mean > 0.0 {
            (1.0 / (1.0 + std_dev / mean)).min(1.0)
        } else {
            0.0
        }
    }

    /// Check if frame rate is stable
    pub fn is_stable(&self) -> bool {
        self.get_stability() > 0.8
    }

    /// Check if frame rate is too low
    pub fn is_too_low(&self) -> bool {
        self.average_fps < self.low_fps_threshold
    }

    /// Check if frame rate is too high
    pub fn is_too_high(&self) -> bool {
        self.average_fps > self.high_fps_threshold
    }

    /// Get recommended quality adjustment
    pub fn get_quality_adjustment(&self) -> QualityAdjustment {
        if self.is_too_low() {
            QualityAdjustment::Decrease
        } else if self.is_too_high() && self.is_stable() {
            QualityAdjustment::Increase
        } else {
            QualityAdjustment::Maintain
        }
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&PerformanceEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Calculate average FPS
    fn calculate_average_fps(&self) -> f32 {
        if self.frame_times.is_empty() {
            return self.current_fps;
        }

        let total_time: f32 = self.frame_times.iter().sum();
        let frame_count = self.frame_times.len() as f32;
        let average_frame_time = total_time / frame_count;

        if average_frame_time > 0.0 {
            1000.0 / average_frame_time
        } else {
            0.0
        }
    }

    /// Apply frame rate limiting
    fn apply_frame_rate_limiting(&mut self) -> PerformanceResult<()> {
        let target_frame_time = 1000.0 / self.target_fps as f32;
        
        if self.frame_time < target_frame_time {
            let sleep_time = target_frame_time - self.frame_time;
            if sleep_time > 0.0 {
                std::thread::sleep(Duration::from_millis(sleep_time as u64));
            }
        }

        Ok(())
    }

    /// Check for performance events
    fn check_performance_events(&self) -> PerformanceResult<()> {
        // Check for low frame rate
        if self.is_too_low() {
            self.emit_event(PerformanceEvent::LowFrameRate {
                fps: self.average_fps,
                threshold: self.low_fps_threshold,
            });
        }

        // Check for frame rate stabilization
        if self.is_stable() && !self.is_too_low() {
            self.emit_event(PerformanceEvent::FrameRateStabilized {
                fps: self.average_fps,
            });
        }

        Ok(())
    }

    /// Emit performance event
    fn emit_event(&self, event: PerformanceEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

/// Quality adjustment recommendation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QualityAdjustment {
    /// Increase quality
    Increase,
    /// Decrease quality
    Decrease,
    /// Maintain current quality
    Maintain,
}

impl Default for FrameRateManager {
    fn default() -> Self {
        Self::new(60)
    }
}
