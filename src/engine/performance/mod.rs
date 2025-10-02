//! Performance Optimization System
//! 
//! This module provides comprehensive performance optimization, quality settings,
//! frame rate management, memory optimization, and performance monitoring for the 2D brawler game.

pub mod quality_settings;
pub mod frame_rate;
pub mod memory_management;
pub mod performance_monitor;
pub mod optimization;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Re-export main components
pub use quality_settings::QualitySettings;
pub use frame_rate::FrameRateManager;
pub use memory_management::MemoryManager;
pub use performance_monitor::PerformanceMonitor;
pub use optimization::OptimizationManager;

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable performance monitoring
    pub monitoring_enabled: bool,
    /// Target frame rate
    pub target_fps: u32,
    /// Enable adaptive quality
    pub adaptive_quality: bool,
    /// Enable memory optimization
    pub memory_optimization: bool,
    /// Enable frame rate limiting
    pub frame_rate_limiting: bool,
    /// Performance logging enabled
    pub logging_enabled: bool,
    /// Quality level
    pub quality_level: QualityLevel,
    /// Memory budget (in MB)
    pub memory_budget: u64,
    /// CPU usage threshold
    pub cpu_threshold: f32,
    /// GPU usage threshold
    pub gpu_threshold: f32,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            monitoring_enabled: true,
            target_fps: 60,
            adaptive_quality: true,
            memory_optimization: true,
            frame_rate_limiting: true,
            logging_enabled: false,
            quality_level: QualityLevel::High,
            memory_budget: 1024, // 1GB
            cpu_threshold: 80.0,
            gpu_threshold: 80.0,
        }
    }
}

/// Quality level
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QualityLevel {
    /// Low quality - maximum performance
    Low,
    /// Medium quality - balanced performance
    Medium,
    /// High quality - good performance
    High,
    /// Ultra quality - maximum quality
    Ultra,
    /// Custom quality settings
    Custom,
}

impl Default for QualityLevel {
    fn default() -> Self {
        Self::High
    }
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Current FPS
    pub fps: f32,
    /// Average FPS over last second
    pub average_fps: f32,
    /// Frame time in milliseconds
    pub frame_time: f32,
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// GPU usage percentage
    pub gpu_usage: f32,
    /// Memory usage in MB
    pub memory_usage: f64,
    /// Memory usage percentage
    pub memory_percentage: f32,
    /// Draw calls per frame
    pub draw_calls: u32,
    /// Triangles rendered per frame
    pub triangles: u32,
    /// Texture memory usage in MB
    pub texture_memory: f64,
    /// Buffer memory usage in MB
    pub buffer_memory: f64,
    /// Shader compilation time in ms
    pub shader_compile_time: f32,
    /// Asset loading time in ms
    pub asset_load_time: f32,
    /// Physics update time in ms
    pub physics_time: f32,
    /// Audio processing time in ms
    pub audio_time: f32,
    /// Input processing time in ms
    pub input_time: f32,
    /// Rendering time in ms
    pub rendering_time: f32,
    /// Total frame time in ms
    pub total_frame_time: f32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            fps: 60.0,
            average_fps: 60.0,
            frame_time: 16.67,
            cpu_usage: 0.0,
            gpu_usage: 0.0,
            memory_usage: 0.0,
            memory_percentage: 0.0,
            draw_calls: 0,
            triangles: 0,
            texture_memory: 0.0,
            buffer_memory: 0.0,
            shader_compile_time: 0.0,
            asset_load_time: 0.0,
            physics_time: 0.0,
            audio_time: 0.0,
            input_time: 0.0,
            rendering_time: 0.0,
            total_frame_time: 16.67,
        }
    }
}

/// Performance events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceEvent {
    /// Frame rate dropped below threshold
    LowFrameRate { fps: f32, threshold: f32 },
    /// High CPU usage detected
    HighCpuUsage { usage: f32, threshold: f32 },
    /// High GPU usage detected
    HighGpuUsage { usage: f32, threshold: f32 },
    /// High memory usage detected
    HighMemoryUsage { usage: f64, threshold: f64 },
    /// Quality level changed
    QualityLevelChanged { from: QualityLevel, to: QualityLevel },
    /// Performance optimization applied
    OptimizationApplied { optimization: String, impact: f32 },
    /// Memory cleanup performed
    MemoryCleanup { freed_mb: f64 },
    /// Frame rate stabilized
    FrameRateStabilized { fps: f32 },
}

/// Performance result type
pub type PerformanceResult<T> = Result<T, PerformanceError>;

/// Performance error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceError {
    /// Performance monitoring disabled
    MonitoringDisabled,
    /// Invalid configuration
    InvalidConfig(String),
    /// Quality level not supported
    UnsupportedQualityLevel(QualityLevel),
    /// Memory allocation failed
    MemoryAllocationFailed,
    /// Frame rate too low
    FrameRateTooLow(f32),
    /// Performance threshold exceeded
    ThresholdExceeded(String),
    /// Optimization failed
    OptimizationFailed(String),
    /// Unknown error
    Unknown(String),
}

impl std::fmt::Display for PerformanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerformanceError::MonitoringDisabled => write!(f, "Performance monitoring is disabled"),
            PerformanceError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            PerformanceError::UnsupportedQualityLevel(level) => write!(f, "Unsupported quality level: {:?}", level),
            PerformanceError::MemoryAllocationFailed => write!(f, "Memory allocation failed"),
            PerformanceError::FrameRateTooLow(fps) => write!(f, "Frame rate too low: {}", fps),
            PerformanceError::ThresholdExceeded(threshold) => write!(f, "Performance threshold exceeded: {}", threshold),
            PerformanceError::OptimizationFailed(msg) => write!(f, "Optimization failed: {}", msg),
            PerformanceError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for PerformanceError {}
