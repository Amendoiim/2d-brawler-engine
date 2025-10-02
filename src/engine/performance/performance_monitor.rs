//! Performance Monitor
//! 
//! This module provides comprehensive performance monitoring and metrics collection.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::{PerformanceMetrics, PerformanceEvent, PerformanceResult, PerformanceError};

/// Performance monitor
pub struct PerformanceMonitor {
    /// Current metrics
    pub metrics: PerformanceMetrics,
    /// Metrics history
    pub metrics_history: Vec<PerformanceMetrics>,
    /// Monitoring enabled
    pub monitoring_enabled: bool,
    /// Update interval in seconds
    pub update_interval: f32,
    /// Last update time
    pub last_update_time: Instant,
    /// Frame time samples
    pub frame_time_samples: Vec<f32>,
    /// FPS samples
    pub fps_samples: Vec<f32>,
    /// CPU usage samples
    pub cpu_samples: Vec<f32>,
    /// GPU usage samples
    pub gpu_samples: Vec<f32>,
    /// Memory usage samples
    pub memory_samples: Vec<f64>,
    /// Performance counters
    pub counters: HashMap<String, u64>,
    /// Performance timers
    pub timers: HashMap<String, Instant>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&PerformanceEvent) + Send + Sync>>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            metrics: PerformanceMetrics::default(),
            metrics_history: Vec::new(),
            monitoring_enabled: true,
            update_interval: 1.0, // 1 second
            last_update_time: Instant::now(),
            frame_time_samples: Vec::new(),
            fps_samples: Vec::new(),
            cpu_samples: Vec::new(),
            gpu_samples: Vec::new(),
            memory_samples: Vec::new(),
            counters: HashMap::new(),
            timers: HashMap::new(),
            event_handlers: Vec::new(),
        }
    }

    /// Update performance monitor
    pub fn update(&mut self, delta_time: f32) -> PerformanceResult<()> {
        if !self.monitoring_enabled {
            return Ok(());
        }

        // Update frame time
        self.metrics.frame_time = delta_time * 1000.0; // Convert to milliseconds
        self.metrics.total_frame_time = self.metrics.frame_time;

        // Update FPS
        self.metrics.fps = 1000.0 / self.metrics.frame_time;
        self.frame_time_samples.push(self.metrics.frame_time);
        self.fps_samples.push(self.metrics.fps);

        // Keep only last 60 samples
        if self.frame_time_samples.len() > 60 {
            self.frame_time_samples.remove(0);
        }
        if self.fps_samples.len() > 60 {
            self.fps_samples.remove(0);
        }

        // Calculate average FPS
        self.metrics.average_fps = self.calculate_average_fps();

        // Update other metrics (in a real implementation, these would be queried from the system)
        self.update_system_metrics()?;

        // Check if it's time to update metrics history
        if Instant::now().duration_since(self.last_update_time).as_secs_f32() >= self.update_interval {
            self.update_metrics_history();
            self.last_update_time = Instant::now();
        }

        // Check for performance issues
        self.check_performance_issues()?;

        Ok(())
    }

    /// Start timing an operation
    pub fn start_timer(&mut self, name: &str) {
        self.timers.insert(name.to_string(), Instant::now());
    }

    /// End timing an operation
    pub fn end_timer(&mut self, name: &str) -> PerformanceResult<f32> {
        if let Some(start_time) = self.timers.remove(name) {
            let duration = Instant::now().duration_since(start_time);
            let time_ms = duration.as_secs_f32() * 1000.0;
            Ok(time_ms)
        } else {
            Err(PerformanceError::Unknown(format!("Timer '{}' not found", name)))
        }
    }

    /// Increment a counter
    pub fn increment_counter(&mut self, name: &str, value: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += value;
    }

    /// Set a counter value
    pub fn set_counter(&mut self, name: &str, value: u64) {
        self.counters.insert(name.to_string(), value);
    }

    /// Get counter value
    pub fn get_counter(&self, name: &str) -> u64 {
        self.counters.get(name).copied().unwrap_or(0)
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    /// Get metrics history
    pub fn get_metrics_history(&self) -> &Vec<PerformanceMetrics> {
        &self.metrics_history
    }

    /// Get performance statistics
    pub fn get_statistics(&self) -> PerformanceStatistics {
        PerformanceStatistics {
            average_fps: self.calculate_average_fps(),
            min_fps: self.fps_samples.iter().copied().fold(f32::INFINITY, f32::min),
            max_fps: self.fps_samples.iter().copied().fold(0.0, f32::max),
            average_frame_time: self.calculate_average_frame_time(),
            min_frame_time: self.frame_time_samples.iter().copied().fold(f32::INFINITY, f32::min),
            max_frame_time: self.frame_time_samples.iter().copied().fold(0.0, f32::max),
            frame_time_std_dev: self.calculate_frame_time_std_dev(),
            average_cpu_usage: self.calculate_average_cpu_usage(),
            average_gpu_usage: self.calculate_average_gpu_usage(),
            average_memory_usage: self.calculate_average_memory_usage(),
            peak_memory_usage: self.memory_samples.iter().copied().fold(0.0, f64::max),
            total_frames: self.fps_samples.len() as u64,
            total_time: self.fps_samples.len() as f32 * self.metrics.frame_time / 1000.0,
        }
    }

    /// Enable/disable monitoring
    pub fn set_monitoring_enabled(&mut self, enabled: bool) {
        self.monitoring_enabled = enabled;
    }

    /// Set update interval
    pub fn set_update_interval(&mut self, interval: f32) -> PerformanceResult<()> {
        if interval <= 0.0 {
            return Err(PerformanceError::InvalidConfig("Update interval must be positive".to_string()));
        }
        self.update_interval = interval;
        Ok(())
    }

    /// Clear metrics history
    pub fn clear_history(&mut self) {
        self.metrics_history.clear();
        self.frame_time_samples.clear();
        self.fps_samples.clear();
        self.cpu_samples.clear();
        self.gpu_samples.clear();
        self.memory_samples.clear();
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&PerformanceEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Update system metrics
    fn update_system_metrics(&mut self) -> PerformanceResult<()> {
        // In a real implementation, these would query the actual system
        // For now, we'll simulate some values
        
        // Simulate CPU usage (0-100%)
        let cpu_usage = 50.0 + (self.metrics.fps - 60.0) * 0.5;
        self.metrics.cpu_usage = cpu_usage.max(0.0).min(100.0);
        self.cpu_samples.push(self.metrics.cpu_usage);

        // Simulate GPU usage (0-100%)
        let gpu_usage = 60.0 + (self.metrics.fps - 60.0) * 0.3;
        self.metrics.gpu_usage = gpu_usage.max(0.0).min(100.0);
        self.gpu_samples.push(self.metrics.gpu_usage);

        // Simulate memory usage (in MB)
        let memory_usage = 200.0 + (self.metrics.fps - 60.0) * 2.0;
        self.metrics.memory_usage = memory_usage.max(0.0) as f64;
        self.memory_samples.push(self.metrics.memory_usage);

        // Update memory percentage
        self.metrics.memory_percentage = (self.metrics.memory_usage / 1024.0 * 100.0) as f32;

        // Simulate other metrics
        self.metrics.draw_calls = (100.0 + (self.metrics.fps - 60.0) * 2.0) as u32;
        self.metrics.triangles = (5000.0 + (self.metrics.fps - 60.0) * 100.0) as u32;
        self.metrics.texture_memory = self.metrics.memory_usage * 0.4;
        self.metrics.buffer_memory = self.metrics.memory_usage * 0.3;

        // Keep only last 60 samples
        if self.cpu_samples.len() > 60 {
            self.cpu_samples.remove(0);
        }
        if self.gpu_samples.len() > 60 {
            self.gpu_samples.remove(0);
        }
        if self.memory_samples.len() > 60 {
            self.memory_samples.remove(0);
        }

        Ok(())
    }

    /// Update metrics history
    fn update_metrics_history(&mut self) {
        self.metrics_history.push(self.metrics.clone());
        
        // Keep only last 100 entries
        if self.metrics_history.len() > 100 {
            self.metrics_history.remove(0);
        }
    }

    /// Check for performance issues
    fn check_performance_issues(&self) -> PerformanceResult<()> {
        // Check for low FPS
        if self.metrics.average_fps < 30.0 {
            self.emit_event(PerformanceEvent::LowFrameRate {
                fps: self.metrics.average_fps,
                threshold: 30.0,
            });
        }

        // Check for high CPU usage
        if self.metrics.cpu_usage > 80.0 {
            self.emit_event(PerformanceEvent::HighCpuUsage {
                usage: self.metrics.cpu_usage,
                threshold: 80.0,
            });
        }

        // Check for high GPU usage
        if self.metrics.gpu_usage > 80.0 {
            self.emit_event(PerformanceEvent::HighGpuUsage {
                usage: self.metrics.gpu_usage,
                threshold: 80.0,
            });
        }

        // Check for high memory usage
        if self.metrics.memory_usage > 800.0 { // 800MB
            self.emit_event(PerformanceEvent::HighMemoryUsage {
                usage: self.metrics.memory_usage,
                threshold: 800.0,
            });
        }

        Ok(())
    }

    /// Calculate average FPS
    fn calculate_average_fps(&self) -> f32 {
        if self.fps_samples.is_empty() {
            return self.metrics.fps;
        }
        self.fps_samples.iter().sum::<f32>() / self.fps_samples.len() as f32
    }

    /// Calculate average frame time
    fn calculate_average_frame_time(&self) -> f32 {
        if self.frame_time_samples.is_empty() {
            return self.metrics.frame_time;
        }
        self.frame_time_samples.iter().sum::<f32>() / self.frame_time_samples.len() as f32
    }

    /// Calculate frame time standard deviation
    fn calculate_frame_time_std_dev(&self) -> f32 {
        if self.frame_time_samples.len() < 2 {
            return 0.0;
        }

        let mean = self.calculate_average_frame_time();
        let variance = self.frame_time_samples.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f32>() / (self.frame_time_samples.len() - 1) as f32;
        variance.sqrt()
    }

    /// Calculate average CPU usage
    fn calculate_average_cpu_usage(&self) -> f32 {
        if self.cpu_samples.is_empty() {
            return self.metrics.cpu_usage;
        }
        self.cpu_samples.iter().sum::<f32>() / self.cpu_samples.len() as f32
    }

    /// Calculate average GPU usage
    fn calculate_average_gpu_usage(&self) -> f32 {
        if self.gpu_samples.is_empty() {
            return self.metrics.gpu_usage;
        }
        self.gpu_samples.iter().sum::<f32>() / self.gpu_samples.len() as f32
    }

    /// Calculate average memory usage
    fn calculate_average_memory_usage(&self) -> f64 {
        if self.memory_samples.is_empty() {
            return self.metrics.memory_usage;
        }
        self.memory_samples.iter().sum::<f64>() / self.memory_samples.len() as f64
    }

    /// Emit performance event
    fn emit_event(&self, event: PerformanceEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

/// Performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStatistics {
    /// Average FPS
    pub average_fps: f32,
    /// Minimum FPS
    pub min_fps: f32,
    /// Maximum FPS
    pub max_fps: f32,
    /// Average frame time in ms
    pub average_frame_time: f32,
    /// Minimum frame time in ms
    pub min_frame_time: f32,
    /// Maximum frame time in ms
    pub max_frame_time: f32,
    /// Frame time standard deviation
    pub frame_time_std_dev: f32,
    /// Average CPU usage
    pub average_cpu_usage: f32,
    /// Average GPU usage
    pub average_gpu_usage: f32,
    /// Average memory usage in MB
    pub average_memory_usage: f64,
    /// Peak memory usage in MB
    pub peak_memory_usage: f64,
    /// Total frames processed
    pub total_frames: u64,
    /// Total time in seconds
    pub total_time: f32,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}
