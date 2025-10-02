//! Performance Optimization
//! 
//! This module provides automatic performance optimization and adaptive quality adjustment.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::{PerformanceResult, PerformanceError, PerformanceEvent, QualityLevel, PerformanceMetrics};

/// Optimization manager
pub struct OptimizationManager {
    /// Optimization enabled
    pub optimization_enabled: bool,
    /// Adaptive quality enabled
    pub adaptive_quality: bool,
    /// Current quality level
    pub current_quality: QualityLevel,
    /// Optimization history
    pub optimization_history: Vec<OptimizationRecord>,
    /// Performance targets
    pub targets: PerformanceTargets,
    /// Optimization rules
    pub rules: Vec<OptimizationRule>,
    /// Last optimization time
    pub last_optimization_time: Instant,
    /// Optimization interval in seconds
    pub optimization_interval: f32,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&PerformanceEvent) + Send + Sync>>,
}

/// Optimization record
#[derive(Debug, Clone)]
pub struct OptimizationRecord {
    /// Timestamp
    pub timestamp: Instant,
    /// Optimization applied
    pub optimization: String,
    /// Performance impact
    pub impact: f32,
    /// Quality level before
    pub quality_before: QualityLevel,
    /// Quality level after
    pub quality_after: QualityLevel,
    /// FPS before
    pub fps_before: f32,
    /// FPS after
    pub fps_after: f32,
}

/// Performance targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTargets {
    /// Target FPS
    pub target_fps: f32,
    /// Minimum FPS
    pub min_fps: f32,
    /// Target frame time in ms
    pub target_frame_time: f32,
    /// Maximum frame time in ms
    pub max_frame_time: f32,
    /// Target CPU usage percentage
    pub target_cpu_usage: f32,
    /// Maximum CPU usage percentage
    pub max_cpu_usage: f32,
    /// Target GPU usage percentage
    pub target_gpu_usage: f32,
    /// Maximum GPU usage percentage
    pub max_gpu_usage: f32,
    /// Target memory usage in MB
    pub target_memory_usage: f64,
    /// Maximum memory usage in MB
    pub max_memory_usage: f64,
}

/// Optimization rule
#[derive(Debug, Clone)]
pub struct OptimizationRule {
    /// Rule name
    pub name: String,
    /// Condition to trigger optimization
    pub condition: OptimizationCondition,
    /// Action to take
    pub action: OptimizationAction,
    /// Priority (higher = more important)
    pub priority: u32,
    /// Cooldown in seconds
    pub cooldown: f32,
    /// Last applied time
    pub last_applied: Option<Instant>,
}

/// Optimization condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationCondition {
    /// FPS below threshold
    LowFps { threshold: f32 },
    /// FPS above threshold
    HighFps { threshold: f32 },
    /// CPU usage above threshold
    HighCpuUsage { threshold: f32 },
    /// GPU usage above threshold
    HighGpuUsage { threshold: f32 },
    /// Memory usage above threshold
    HighMemoryUsage { threshold: f64 },
    /// Frame time above threshold
    HighFrameTime { threshold: f32 },
    /// Frame time variance above threshold
    HighFrameTimeVariance { threshold: f32 },
    /// Combined condition
    And { conditions: Vec<OptimizationCondition> },
    /// Either condition
    Or { conditions: Vec<OptimizationCondition> },
}

/// Optimization action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationAction {
    /// Decrease quality level
    DecreaseQuality,
    /// Increase quality level
    IncreaseQuality,
    /// Set specific quality level
    SetQuality { level: QualityLevel },
    /// Enable/disable specific feature
    ToggleFeature { feature: String, enabled: bool },
    /// Adjust rendering setting
    AdjustRendering { setting: String, value: f32 },
    /// Adjust audio setting
    AdjustAudio { setting: String, value: f32 },
    /// Adjust physics setting
    AdjustPhysics { setting: String, value: f32 },
    /// Adjust visual effects setting
    AdjustVisualEffects { setting: String, value: f32 },
    /// Run garbage collection
    RunGarbageCollection,
    /// Clear caches
    ClearCaches,
    /// Custom action
    Custom { action: String, parameters: HashMap<String, String> },
}

impl OptimizationManager {
    /// Create a new optimization manager
    pub fn new() -> Self {
        let mut manager = Self {
            optimization_enabled: true,
            adaptive_quality: true,
            current_quality: QualityLevel::High,
            optimization_history: Vec::new(),
            targets: PerformanceTargets::default(),
            rules: Vec::new(),
            last_optimization_time: Instant::now(),
            optimization_interval: 2.0, // 2 seconds
            event_handlers: Vec::new(),
        };

        // Initialize default optimization rules
        manager.initialize_default_rules();
        manager
    }

    /// Update optimization manager
    pub fn update(&mut self, metrics: &PerformanceMetrics) -> PerformanceResult<()> {
        if !self.optimization_enabled {
            return Ok(());
        }

        // Check if it's time to run optimization
        if Instant::now().duration_since(self.last_optimization_time).as_secs_f32() >= self.optimization_interval {
            self.run_optimization(metrics)?;
            self.last_optimization_time = Instant::now();
        }

        Ok(())
    }

    /// Run optimization based on current metrics
    pub fn run_optimization(&mut self, metrics: &PerformanceMetrics) -> PerformanceResult<()> {
        let mut applied_optimizations = Vec::new();

        // Sort rules by priority (highest first)
        let mut sorted_rules = self.rules.clone();
        sorted_rules.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Check each rule
        for rule in sorted_rules {
            if self.should_apply_rule(&rule, metrics)? {
                if let Some(optimization) = self.apply_rule(&rule, metrics)? {
                    applied_optimizations.push(optimization);
                }
            }
        }

        // Record applied optimizations
        for optimization in applied_optimizations {
            self.optimization_history.push(optimization);
            
            // Keep only last 100 records
            if self.optimization_history.len() > 100 {
                self.optimization_history.remove(0);
            }
        }

        Ok(())
    }

    /// Add optimization rule
    pub fn add_rule(&mut self, rule: OptimizationRule) {
        self.rules.push(rule);
    }

    /// Remove optimization rule
    pub fn remove_rule(&mut self, name: &str) {
        self.rules.retain(|rule| rule.name != name);
    }

    /// Set performance targets
    pub fn set_targets(&mut self, targets: PerformanceTargets) {
        self.targets = targets;
    }

    /// Enable/disable optimization
    pub fn set_optimization_enabled(&mut self, enabled: bool) {
        self.optimization_enabled = enabled;
    }

    /// Enable/disable adaptive quality
    pub fn set_adaptive_quality(&mut self, enabled: bool) {
        self.adaptive_quality = enabled;
    }

    /// Set optimization interval
    pub fn set_optimization_interval(&mut self, interval: f32) -> PerformanceResult<()> {
        if interval <= 0.0 {
            return Err(PerformanceError::InvalidConfig("Optimization interval must be positive".to_string()));
        }
        self.optimization_interval = interval;
        Ok(())
    }

    /// Get optimization history
    pub fn get_optimization_history(&self) -> &Vec<OptimizationRecord> {
        &self.optimization_history
    }

    /// Get current quality level
    pub fn get_current_quality(&self) -> QualityLevel {
        self.current_quality.clone()
    }

    /// Set quality level
    pub fn set_quality_level(&mut self, level: QualityLevel) {
        self.current_quality = level;
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&PerformanceEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Check if rule should be applied
    fn should_apply_rule(&self, rule: &OptimizationRule, metrics: &PerformanceMetrics) -> PerformanceResult<bool> {
        // Check cooldown
        if let Some(last_applied) = rule.last_applied {
            if Instant::now().duration_since(last_applied).as_secs_f32() < rule.cooldown {
                return Ok(false);
            }
        }

        // Check condition
        Ok(self.evaluate_condition(&rule.condition, metrics)?)
    }

    /// Evaluate optimization condition
    fn evaluate_condition(&self, condition: &OptimizationCondition, metrics: &PerformanceMetrics) -> PerformanceResult<bool> {
        match condition {
            OptimizationCondition::LowFps { threshold } => Ok(metrics.fps < *threshold),
            OptimizationCondition::HighFps { threshold } => Ok(metrics.fps > *threshold),
            OptimizationCondition::HighCpuUsage { threshold } => Ok(metrics.cpu_usage > *threshold),
            OptimizationCondition::HighGpuUsage { threshold } => Ok(metrics.gpu_usage > *threshold),
            OptimizationCondition::HighMemoryUsage { threshold } => Ok(metrics.memory_usage > *threshold),
            OptimizationCondition::HighFrameTime { threshold } => Ok(metrics.frame_time > *threshold),
            OptimizationCondition::HighFrameTimeVariance { threshold } => {
                // Calculate frame time variance (simplified)
                Ok(metrics.frame_time > *threshold)
            },
            OptimizationCondition::And { conditions } => {
                for condition in conditions {
                    if !self.evaluate_condition(condition, metrics)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            },
            OptimizationCondition::Or { conditions } => {
                for condition in conditions {
                    if self.evaluate_condition(condition, metrics)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            },
        }
    }

    /// Apply optimization rule
    fn apply_rule(&mut self, rule: &OptimizationRule, metrics: &PerformanceMetrics) -> PerformanceResult<Option<OptimizationRecord>> {
        let quality_before = self.current_quality.clone();
        let fps_before = metrics.fps;

        // Apply action
        let impact = self.apply_action(&rule.action)?;

        // Update rule last applied time
        if let Some(rule_ref) = self.rules.iter_mut().find(|r| r.name == rule.name) {
            rule_ref.last_applied = Some(Instant::now());
        }

        // Create optimization record
        let record = OptimizationRecord {
            timestamp: Instant::now(),
            optimization: rule.name.clone(),
            impact,
            quality_before: quality_before.clone(),
            quality_after: self.current_quality.clone(),
            fps_before,
            fps_after: metrics.fps, // This would be updated in the next frame
        };

        // Emit event
        self.emit_event(PerformanceEvent::OptimizationApplied {
            optimization: rule.name.clone(),
            impact,
        });

        Ok(Some(record))
    }

    /// Apply optimization action
    fn apply_action(&mut self, action: &OptimizationAction) -> PerformanceResult<f32> {
        match action {
            OptimizationAction::DecreaseQuality => {
                self.decrease_quality_level();
                Ok(0.2) // 20% performance improvement
            },
            OptimizationAction::IncreaseQuality => {
                self.increase_quality_level();
                Ok(-0.1) // 10% performance cost
            },
            OptimizationAction::SetQuality { level } => {
                let old_quality = self.current_quality.clone();
                self.current_quality = level.clone();
                
                // Calculate impact based on quality change
                let impact = self.calculate_quality_impact(&old_quality, level);
                Ok(impact)
            },
            OptimizationAction::ToggleFeature { feature: _, enabled: _ } => {
                // Feature toggling would be implemented here
                Ok(0.1)
            },
            OptimizationAction::AdjustRendering { setting: _, value: _ } => {
                // Rendering adjustment would be implemented here
                Ok(0.05)
            },
            OptimizationAction::AdjustAudio { setting: _, value: _ } => {
                // Audio adjustment would be implemented here
                Ok(0.02)
            },
            OptimizationAction::AdjustPhysics { setting: _, value: _ } => {
                // Physics adjustment would be implemented here
                Ok(0.1)
            },
            OptimizationAction::AdjustVisualEffects { setting: _, value: _ } => {
                // Visual effects adjustment would be implemented here
                Ok(0.15)
            },
            OptimizationAction::RunGarbageCollection => {
                // Garbage collection would be implemented here
                Ok(0.05)
            },
            OptimizationAction::ClearCaches => {
                // Cache clearing would be implemented here
                Ok(0.1)
            },
            OptimizationAction::Custom { action: _, parameters: _ } => {
                // Custom action would be implemented here
                Ok(0.0)
            },
        }
    }

    /// Decrease quality level
    fn decrease_quality_level(&mut self) {
        self.current_quality = match self.current_quality {
            QualityLevel::Ultra => QualityLevel::High,
            QualityLevel::High => QualityLevel::Medium,
            QualityLevel::Medium => QualityLevel::Low,
            QualityLevel::Low => QualityLevel::Low,
            QualityLevel::Custom => QualityLevel::Medium,
        };
    }

    /// Increase quality level
    fn increase_quality_level(&mut self) {
        self.current_quality = match self.current_quality {
            QualityLevel::Low => QualityLevel::Medium,
            QualityLevel::Medium => QualityLevel::High,
            QualityLevel::High => QualityLevel::Ultra,
            QualityLevel::Ultra => QualityLevel::Ultra,
            QualityLevel::Custom => QualityLevel::High,
        };
    }

    /// Calculate quality impact
    fn calculate_quality_impact(&self, from: &QualityLevel, to: &QualityLevel) -> f32 {
        let from_value = match from {
            QualityLevel::Low => 0.1,
            QualityLevel::Medium => 0.5,
            QualityLevel::High => 0.8,
            QualityLevel::Ultra => 1.0,
            QualityLevel::Custom => 0.8,
        };

        let to_value = match to {
            QualityLevel::Low => 0.1,
            QualityLevel::Medium => 0.5,
            QualityLevel::High => 0.8,
            QualityLevel::Ultra => 1.0,
            QualityLevel::Custom => 0.8,
        };

        to_value - from_value
    }

    /// Initialize default optimization rules
    fn initialize_default_rules(&mut self) {
        // Low FPS rule
        self.rules.push(OptimizationRule {
            name: "Low FPS Optimization".to_string(),
            condition: OptimizationCondition::LowFps { threshold: 30.0 },
            action: OptimizationAction::DecreaseQuality,
            priority: 100,
            cooldown: 5.0,
            last_applied: None,
        });

        // High FPS rule
        self.rules.push(OptimizationRule {
            name: "High FPS Optimization".to_string(),
            condition: OptimizationCondition::HighFps { threshold: 80.0 },
            action: OptimizationAction::IncreaseQuality,
            priority: 50,
            cooldown: 10.0,
            last_applied: None,
        });

        // High CPU usage rule
        self.rules.push(OptimizationRule {
            name: "High CPU Usage Optimization".to_string(),
            condition: OptimizationCondition::HighCpuUsage { threshold: 80.0 },
            action: OptimizationAction::DecreaseQuality,
            priority: 90,
            cooldown: 3.0,
            last_applied: None,
        });

        // High GPU usage rule
        self.rules.push(OptimizationRule {
            name: "High GPU Usage Optimization".to_string(),
            condition: OptimizationCondition::HighGpuUsage { threshold: 80.0 },
            action: OptimizationAction::DecreaseQuality,
            priority: 90,
            cooldown: 3.0,
            last_applied: None,
        });

        // High memory usage rule
        self.rules.push(OptimizationRule {
            name: "High Memory Usage Optimization".to_string(),
            condition: OptimizationCondition::HighMemoryUsage { threshold: 800.0 },
            action: OptimizationAction::RunGarbageCollection,
            priority: 80,
            cooldown: 2.0,
            last_applied: None,
        });
    }

    /// Emit performance event
    fn emit_event(&self, event: PerformanceEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for PerformanceTargets {
    fn default() -> Self {
        Self {
            target_fps: 60.0,
            min_fps: 30.0,
            target_frame_time: 16.67,
            max_frame_time: 33.33,
            target_cpu_usage: 60.0,
            max_cpu_usage: 80.0,
            target_gpu_usage: 60.0,
            max_gpu_usage: 80.0,
            target_memory_usage: 512.0,
            max_memory_usage: 1024.0,
        }
    }
}

impl Default for OptimizationManager {
    fn default() -> Self {
        Self::new()
    }
}
