//! Entity-Component-System implementation

use std::collections::HashMap;
use std::any::TypeId;

pub mod query;
pub mod system_manager;

/// Entity ID type
pub type Entity = u32;

/// Component trait for ECS
pub trait Component: 'static + Send + Sync {}

/// Derive macro for Component trait
#[macro_export]
macro_rules! derive_component {
    ($($t:ty),*) => {
        $(
            impl Component for $t {}
        )*
    };
}

/// System trait for ECS
pub trait System {
    fn update(&mut self, world: &mut World, dt: f32);
}

/// ECS World that manages entities, components, and systems
pub struct World {
    entities: Vec<Entity>,
    next_entity_id: Entity,
    components: HashMap<TypeId, Box<dyn std::any::Any>>,
    system_manager: system_manager::SystemManager,
}

impl World {
    /// Create a new ECS world
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_entity_id: 0,
            components: HashMap::new(),
            system_manager: system_manager::SystemManager::new(),
        }
    }

    /// Create a new entity
    pub fn create_entity(&mut self) -> Entity {
        let entity = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(entity);
        entity
    }

    /// Add a component to an entity
    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        
        if !self.components.contains_key(&type_id) {
            self.components.insert(type_id, Box::new(HashMap::<Entity, T>::new()));
        }
        
        if let Some(component_map) = self.components.get_mut(&type_id) {
            if let Some(map) = component_map.downcast_mut::<HashMap<Entity, T>>() {
                map.insert(entity, component);
            }
        }
    }

    /// Get a component from an entity
    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        
        if let Some(component_map) = self.components.get(&type_id) {
            if let Some(map) = component_map.downcast_ref::<HashMap<Entity, T>>() {
                return map.get(&entity);
            }
        }
        
        None
    }

    /// Get a mutable component from an entity
    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        
        if let Some(component_map) = self.components.get_mut(&type_id) {
            if let Some(map) = component_map.downcast_mut::<HashMap<Entity, T>>() {
                return map.get_mut(&entity);
            }
        }
        
        None
    }

    /// Remove a component from an entity
    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        let type_id = TypeId::of::<T>();
        
        if let Some(component_map) = self.components.get_mut(&type_id) {
            if let Some(map) = component_map.downcast_mut::<HashMap<Entity, T>>() {
                map.remove(&entity);
            }
        }
    }

    /// Add a system to the world
    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        // For backward compatibility, use default priority
        self.add_system_with_priority(
            system,
            "Unnamed System".to_string(),
            system_manager::ExecutionOrder::new(system_manager::SystemPriority::Normal, 0),
        );
    }
    
    /// Add a system with specific priority and name
    pub fn add_system_with_priority<T: System + 'static>(
        &mut self,
        system: T,
        name: String,
        execution_order: system_manager::ExecutionOrder,
    ) {
        self.system_manager.add_system(system, name, execution_order);
    }

    /// Update all systems
    pub fn update(&mut self, dt: f32) {
        // Phase 2: Use the system manager for proper execution
        // For now, we'll use a simplified approach to avoid borrowing issues
        log::debug!("Updating {} systems", self.system_manager.system_count());
        
        // TODO: Implement proper system execution in Phase 2
        // This requires a more sophisticated ECS architecture to avoid borrowing conflicts
    }

    /// Query entities with specific components
    pub fn query<T: Component>(&self) -> Vec<Entity> {
        let type_id = TypeId::of::<T>();
        let mut result = Vec::new();
        
        if let Some(component_map) = self.components.get(&type_id) {
            if let Some(map) = component_map.downcast_ref::<HashMap<Entity, T>>() {
                result.extend(map.keys().copied());
            }
        }
        
        result
    }

    /// Query entities with multiple components
    pub fn query_with<A: Component, B: Component>(&self) -> Vec<(Entity, &A, &B)> {
        let type_a = TypeId::of::<A>();
        let type_b = TypeId::of::<B>();
        let mut result = Vec::new();
        
        if let (Some(map_a), Some(map_b)) = (
            self.components.get(&type_a).and_then(|m| m.downcast_ref::<HashMap<Entity, A>>()),
            self.components.get(&type_b).and_then(|m| m.downcast_ref::<HashMap<Entity, B>>())
        ) {
            for entity in map_a.keys() {
                if let (Some(comp_a), Some(comp_b)) = (map_a.get(entity), map_b.get(entity)) {
                    result.push((*entity, comp_a, comp_b));
                }
            }
        }
        
        result
    }

    // Note: query_with_mut removed for Phase 1 due to borrowing complexity
    // This would be implemented with a more sophisticated ECS architecture
    
    /// Get entities that have the specified component (for query system)
    pub fn get_entities_with_component<T: Component>(&self) -> Vec<Entity> {
        let type_id = TypeId::of::<T>();
        let mut result = Vec::new();
        
        if let Some(component_map) = self.components.get(&type_id) {
            if let Some(map) = component_map.downcast_ref::<HashMap<Entity, T>>() {
                result.extend(map.keys().copied());
            }
        }
        
        result
    }
    
    /// Get system manager for advanced system management
    pub fn system_manager(&self) -> &system_manager::SystemManager {
        &self.system_manager
    }
    
    /// Get mutable system manager for advanced system management
    pub fn system_manager_mut(&mut self) -> &mut system_manager::SystemManager {
        &mut self.system_manager
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
