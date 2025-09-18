//! ECS System Manager - Phase 2 Implementation
//! 
//! This module provides a sophisticated system management system that handles
//! system execution order, dependencies, and borrowing issues.

use std::collections::HashMap;
use crate::engine::ecs::{System, World};

/// System priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SystemPriority {
    /// Highest priority - runs first (e.g., input processing)
    Critical = 0,
    /// High priority - runs early (e.g., physics)
    High = 1,
    /// Normal priority - runs in the middle (e.g., game logic)
    Normal = 2,
    /// Low priority - runs late (e.g., rendering)
    Low = 3,
    /// Lowest priority - runs last (e.g., cleanup)
    Cleanup = 4,
}

/// System execution order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExecutionOrder {
    priority: SystemPriority,
    sub_order: u32,
}

impl ExecutionOrder {
    /// Create a new execution order
    pub fn new(priority: SystemPriority, sub_order: u32) -> Self {
        Self { priority, sub_order }
    }
    
    /// Get the priority
    pub fn priority(&self) -> SystemPriority {
        self.priority
    }
    
    /// Get the sub-order
    pub fn sub_order(&self) -> u32 {
        self.sub_order
    }
}

/// System wrapper with metadata
pub struct SystemWrapper {
    system: Box<dyn System>,
    name: String,
    execution_order: ExecutionOrder,
    enabled: bool,
}

impl SystemWrapper {
    /// Create a new system wrapper
    pub fn new<T: System + 'static>(
        system: T,
        name: String,
        execution_order: ExecutionOrder,
    ) -> Self {
        Self {
            system: Box::new(system),
            name,
            execution_order,
            enabled: true,
        }
    }
    
    /// Get the system name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get the execution order
    pub fn execution_order(&self) -> ExecutionOrder {
        self.execution_order
    }
    
    /// Check if the system is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Enable or disable the system
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Update the system
    pub fn update(&mut self, world: &mut World, dt: f32) {
        if self.enabled {
            self.system.update(world, dt);
        }
    }
}

/// System manager that handles system execution
pub struct SystemManager {
    systems: Vec<SystemWrapper>,
    execution_order: Vec<usize>,
    system_indices: HashMap<String, usize>,
}

impl SystemManager {
    /// Create a new system manager
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
            execution_order: Vec::new(),
            system_indices: HashMap::new(),
        }
    }
    
    /// Add a system to the manager
    pub fn add_system<T: System + 'static>(
        &mut self,
        system: T,
        name: String,
        execution_order: ExecutionOrder,
    ) {
        let index = self.systems.len();
        self.systems.push(SystemWrapper::new(system, name.clone(), execution_order));
        self.system_indices.insert(name, index);
        self.update_execution_order();
    }
    
    /// Remove a system by name
    pub fn remove_system(&mut self, name: &str) -> bool {
        if let Some(&index) = self.system_indices.get(name) {
            self.systems.remove(index);
            self.system_indices.clear();
            
            // Rebuild indices
            for (i, system) in self.systems.iter().enumerate() {
                self.system_indices.insert(system.name().to_string(), i);
            }
            
            self.update_execution_order();
            true
        } else {
            false
        }
    }
    
    /// Enable or disable a system
    pub fn set_system_enabled(&mut self, name: &str, enabled: bool) -> bool {
        if let Some(&index) = self.system_indices.get(name) {
            self.systems[index].set_enabled(enabled);
            true
        } else {
            false
        }
    }
    
    /// Get system information
    pub fn get_system_info(&self, name: &str) -> Option<(ExecutionOrder, bool)> {
        if let Some(&index) = self.system_indices.get(name) {
            let system = &self.systems[index];
            Some((system.execution_order(), system.is_enabled()))
        } else {
            None
        }
    }
    
    /// Update all systems in the correct order
    pub fn update(&mut self, world: &mut World, dt: f32) {
        for &system_index in &self.execution_order {
            if system_index < self.systems.len() {
                self.systems[system_index].update(world, dt);
            }
        }
    }
    
    /// Get the number of systems
    pub fn system_count(&self) -> usize {
        self.systems.len()
    }
    
    /// Get the number of enabled systems
    pub fn enabled_system_count(&self) -> usize {
        self.systems.iter().filter(|s| s.is_enabled()).count()
    }
    
    /// Update the execution order based on system priorities
    fn update_execution_order(&mut self) {
        let mut indices: Vec<usize> = (0..self.systems.len()).collect();
        
        // Sort by execution order (priority first, then sub-order)
        indices.sort_by_key(|&i| {
            let system = &self.systems[i];
            (system.execution_order().priority, system.execution_order().sub_order)
        });
        
        self.execution_order = indices;
    }
    
    /// Get system names in execution order
    pub fn system_names(&self) -> Vec<String> {
        self.execution_order
            .iter()
            .filter_map(|&i| {
                if i < self.systems.len() {
                    Some(self.systems[i].name().to_string())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for SystemManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience macros for common system priorities
#[macro_export]
macro_rules! system_priority {
    (critical) => { $crate::engine::ecs::system_manager::SystemPriority::Critical };
    (high) => { $crate::engine::ecs::system_manager::SystemPriority::High };
    (normal) => { $crate::engine::ecs::system_manager::SystemPriority::Normal };
    (low) => { $crate::engine::ecs::system_manager::SystemPriority::Low };
    (cleanup) => { $crate::engine::ecs::system_manager::SystemPriority::Cleanup };
}

#[macro_export]
macro_rules! execution_order {
    ($priority:ident, $sub_order:expr) => {
        $crate::engine::ecs::system_manager::ExecutionOrder::new(
            system_priority!($priority),
            $sub_order
        )
    };
}
