//! Memory Management
//! 
//! This module manages memory optimization, garbage collection, and memory monitoring.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::{PerformanceResult, PerformanceError, PerformanceEvent};

/// Memory manager
pub struct MemoryManager {
    /// Memory budget in MB
    pub memory_budget: u64,
    /// Current memory usage in MB
    pub current_usage: f64,
    /// Peak memory usage in MB
    pub peak_usage: f64,
    /// Memory usage history
    pub usage_history: Vec<f64>,
    /// Memory allocations
    pub allocations: HashMap<String, MemoryAllocation>,
    /// Garbage collection enabled
    pub gc_enabled: bool,
    /// Last garbage collection time
    pub last_gc_time: Instant,
    /// GC interval in seconds
    pub gc_interval: f32,
    /// Memory pressure threshold
    pub pressure_threshold: f32,
    /// Memory cleanup threshold
    pub cleanup_threshold: f32,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&PerformanceEvent) + Send + Sync>>,
}

/// Memory allocation
#[derive(Debug, Clone)]
pub struct MemoryAllocation {
    /// Allocation ID
    pub id: String,
    /// Size in bytes
    pub size: usize,
    /// Allocation time
    pub allocation_time: Instant,
    /// Last access time
    pub last_access_time: Instant,
    /// Access count
    pub access_count: u64,
    /// Allocation type
    pub allocation_type: AllocationType,
    /// Priority
    pub priority: AllocationPriority,
    /// Can be freed
    pub can_be_freed: bool,
}

/// Allocation type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AllocationType {
    /// Texture memory
    Texture,
    /// Buffer memory
    Buffer,
    /// Shader memory
    Shader,
    /// Audio memory
    Audio,
    /// Asset memory
    Asset,
    /// System memory
    System,
    /// Temporary memory
    Temporary,
}

/// Allocation priority
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum AllocationPriority {
    /// Critical - never freed
    Critical,
    /// High - freed only under extreme pressure
    High,
    /// Medium - freed under moderate pressure
    Medium,
    /// Low - freed under light pressure
    Low,
    /// Temporary - freed immediately when not needed
    Temporary,
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new(memory_budget: u64) -> Self {
        Self {
            memory_budget,
            current_usage: 0.0,
            peak_usage: 0.0,
            usage_history: Vec::new(),
            allocations: HashMap::new(),
            gc_enabled: true,
            last_gc_time: Instant::now(),
            gc_interval: 5.0, // 5 seconds
            pressure_threshold: 0.8, // 80%
            cleanup_threshold: 0.9, // 90%
            event_handlers: Vec::new(),
        }
    }

    /// Update memory manager
    pub fn update(&mut self) -> PerformanceResult<()> {
        // Update current memory usage
        self.update_memory_usage()?;

        // Check for memory pressure
        self.check_memory_pressure()?;

        // Run garbage collection if needed
        if self.gc_enabled && self.should_run_gc() {
            self.run_garbage_collection()?;
        }

        // Clean up old allocations
        self.cleanup_old_allocations()?;

        Ok(())
    }

    /// Allocate memory
    pub fn allocate(&mut self, id: String, size: usize, allocation_type: AllocationType, priority: AllocationPriority) -> PerformanceResult<()> {
        // Check if allocation would exceed budget
        let size_mb = size as f64 / (1024.0 * 1024.0);
        if self.current_usage + size_mb > self.memory_budget as f64 {
            // Try to free some memory
            self.free_memory_by_priority(priority)?;
        }

        // Create allocation
        let allocation = MemoryAllocation {
            id: id.clone(),
            size,
            allocation_time: Instant::now(),
            last_access_time: Instant::now(),
            access_count: 0,
            allocation_type,
            priority,
            can_be_freed: priority != AllocationPriority::Critical,
        };

        // Add to allocations
        self.allocations.insert(id, allocation);

        // Update usage
        self.current_usage += size_mb;
        self.peak_usage = self.peak_usage.max(self.current_usage);

        Ok(())
    }

    /// Deallocate memory
    pub fn deallocate(&mut self, id: &str) -> PerformanceResult<()> {
        if let Some(allocation) = self.allocations.remove(id) {
            let size_mb = allocation.size as f64 / (1024.0 * 1024.0);
            self.current_usage -= size_mb;
        }
        Ok(())
    }

    /// Access memory allocation
    pub fn access(&mut self, id: &str) -> PerformanceResult<()> {
        if let Some(allocation) = self.allocations.get_mut(id) {
            allocation.last_access_time = Instant::now();
            allocation.access_count += 1;
        }
        Ok(())
    }

    /// Get memory usage percentage
    pub fn get_usage_percentage(&self) -> f32 {
        (self.current_usage / self.memory_budget as f64 * 100.0) as f32
    }

    /// Check if memory is under pressure
    pub fn is_under_pressure(&self) -> bool {
        self.get_usage_percentage() > self.pressure_threshold * 100.0
    }

    /// Check if memory needs cleanup
    pub fn needs_cleanup(&self) -> bool {
        self.get_usage_percentage() > self.cleanup_threshold * 100.0
    }

    /// Get memory statistics
    pub fn get_statistics(&self) -> MemoryStatistics {
        let mut stats = MemoryStatistics::default();

        for allocation in self.allocations.values() {
            let size_mb = allocation.size as f64 / (1024.0 * 1024.0);
            
            match allocation.allocation_type {
                AllocationType::Texture => stats.texture_memory += size_mb,
                AllocationType::Buffer => stats.buffer_memory += size_mb,
                AllocationType::Shader => stats.shader_memory += size_mb,
                AllocationType::Audio => stats.audio_memory += size_mb,
                AllocationType::Asset => stats.asset_memory += size_mb,
                AllocationType::System => stats.system_memory += size_mb,
                AllocationType::Temporary => stats.temporary_memory += size_mb,
            }

            stats.total_allocations += 1;
            stats.total_size += allocation.size;
        }

        stats.current_usage = self.current_usage;
        stats.peak_usage = self.peak_usage;
        stats.memory_budget = self.memory_budget as f64;
        stats.usage_percentage = self.get_usage_percentage();

        stats
    }

    /// Set memory budget
    pub fn set_memory_budget(&mut self, budget: u64) {
        self.memory_budget = budget;
    }

    /// Enable/disable garbage collection
    pub fn set_gc_enabled(&mut self, enabled: bool) {
        self.gc_enabled = enabled;
    }

    /// Set GC interval
    pub fn set_gc_interval(&mut self, interval: f32) -> PerformanceResult<()> {
        if interval <= 0.0 {
            return Err(PerformanceError::InvalidConfig("GC interval must be positive".to_string()));
        }
        self.gc_interval = interval;
        Ok(())
    }

    /// Set memory thresholds
    pub fn set_thresholds(&mut self, pressure: f32, cleanup: f32) -> PerformanceResult<()> {
        if pressure >= cleanup || pressure < 0.0 || cleanup > 1.0 {
            return Err(PerformanceError::InvalidConfig("Invalid threshold values".to_string()));
        }
        self.pressure_threshold = pressure;
        self.cleanup_threshold = cleanup;
        Ok(())
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&PerformanceEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Update memory usage
    fn update_memory_usage(&mut self) -> PerformanceResult<()> {
        // In a real implementation, this would query the system for actual memory usage
        // For now, we'll use our tracked allocations
        let mut total_usage = 0.0;
        for allocation in self.allocations.values() {
            total_usage += allocation.size as f64 / (1024.0 * 1024.0);
        }
        self.current_usage = total_usage;

        // Update usage history
        self.usage_history.push(self.current_usage);
        if self.usage_history.len() > 100 {
            self.usage_history.remove(0);
        }

        Ok(())
    }

    /// Check memory pressure
    fn check_memory_pressure(&self) -> PerformanceResult<()> {
        if self.is_under_pressure() {
            self.emit_event(PerformanceEvent::HighMemoryUsage {
                usage: self.current_usage,
                threshold: self.memory_budget as f64 * self.pressure_threshold as f64,
            });
        }
        Ok(())
    }

    /// Check if garbage collection should run
    fn should_run_gc(&self) -> bool {
        Instant::now().duration_since(self.last_gc_time).as_secs_f32() >= self.gc_interval
    }

    /// Run garbage collection
    fn run_garbage_collection(&mut self) -> PerformanceResult<()> {
        let mut freed_mb = 0.0;
        let mut to_remove = Vec::new();

        // Find allocations to free
        for (id, allocation) in &self.allocations {
            if allocation.can_be_freed && self.should_free_allocation(allocation) {
                to_remove.push(id.clone());
                freed_mb += allocation.size as f64 / (1024.0 * 1024.0);
            }
        }

        // Remove allocations
        for id in to_remove {
            self.allocations.remove(&id);
        }

        // Update usage
        self.current_usage -= freed_mb;
        self.last_gc_time = Instant::now();

        if freed_mb > 0.0 {
            self.emit_event(PerformanceEvent::MemoryCleanup { freed_mb });
        }

        Ok(())
    }

    /// Check if allocation should be freed
    fn should_free_allocation(&self, allocation: &MemoryAllocation) -> bool {
        let age = Instant::now().duration_since(allocation.allocation_time).as_secs_f32();
        let last_access = Instant::now().duration_since(allocation.last_access_time).as_secs_f32();

        match allocation.priority {
            AllocationPriority::Critical => false,
            AllocationPriority::High => age > 300.0 && last_access > 60.0, // 5 minutes age, 1 minute unused
            AllocationPriority::Medium => age > 120.0 && last_access > 30.0, // 2 minutes age, 30 seconds unused
            AllocationPriority::Low => age > 60.0 && last_access > 10.0, // 1 minute age, 10 seconds unused
            AllocationPriority::Temporary => last_access > 5.0, // 5 seconds unused
        }
    }

    /// Free memory by priority
    fn free_memory_by_priority(&mut self, max_priority: AllocationPriority) -> PerformanceResult<()> {
        let mut freed_mb = 0.0;
        let mut to_remove = Vec::new();

        // Find allocations to free based on priority
        for (id, allocation) in &self.allocations {
            if allocation.can_be_freed && 
               allocation.priority <= max_priority && 
               self.should_free_allocation(allocation) {
                to_remove.push(id.clone());
                freed_mb += allocation.size as f64 / (1024.0 * 1024.0);
            }
        }

        // Remove allocations
        for id in to_remove {
            self.allocations.remove(&id);
        }

        // Update usage
        self.current_usage -= freed_mb;

        Ok(())
    }

    /// Clean up old allocations
    fn cleanup_old_allocations(&mut self) -> PerformanceResult<()> {
        let mut freed_mb = 0.0;
        let mut to_remove = Vec::new();

        // Find very old allocations
        for (id, allocation) in &self.allocations {
            let age = Instant::now().duration_since(allocation.allocation_time).as_secs_f32();
            if allocation.can_be_freed && age > 600.0 { // 10 minutes
                to_remove.push(id.clone());
                freed_mb += allocation.size as f64 / (1024.0 * 1024.0);
            }
        }

        // Remove allocations
        for id in to_remove {
            self.allocations.remove(&id);
        }

        // Update usage
        self.current_usage -= freed_mb;

        Ok(())
    }

    /// Emit performance event
    fn emit_event(&self, event: PerformanceEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatistics {
    /// Current memory usage in MB
    pub current_usage: f64,
    /// Peak memory usage in MB
    pub peak_usage: f64,
    /// Memory budget in MB
    pub memory_budget: f64,
    /// Usage percentage
    pub usage_percentage: f32,
    /// Total allocations
    pub total_allocations: usize,
    /// Total size in bytes
    pub total_size: usize,
    /// Texture memory in MB
    pub texture_memory: f64,
    /// Buffer memory in MB
    pub buffer_memory: f64,
    /// Shader memory in MB
    pub shader_memory: f64,
    /// Audio memory in MB
    pub audio_memory: f64,
    /// Asset memory in MB
    pub asset_memory: f64,
    /// System memory in MB
    pub system_memory: f64,
    /// Temporary memory in MB
    pub temporary_memory: f64,
}

impl Default for MemoryStatistics {
    fn default() -> Self {
        Self {
            current_usage: 0.0,
            peak_usage: 0.0,
            memory_budget: 0.0,
            usage_percentage: 0.0,
            total_allocations: 0,
            total_size: 0,
            texture_memory: 0.0,
            buffer_memory: 0.0,
            shader_memory: 0.0,
            audio_memory: 0.0,
            asset_memory: 0.0,
            system_memory: 0.0,
            temporary_memory: 0.0,
        }
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new(1024) // 1GB default budget
    }
}
