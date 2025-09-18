//! Entity-Component-System implementation

use std::collections::HashMap;
use std::any::TypeId;

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
    systems: Vec<Box<dyn System>>,
}

impl World {
    /// Create a new ECS world
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_entity_id: 0,
            components: HashMap::new(),
            systems: Vec::new(),
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
        self.systems.push(Box::new(system));
    }

    /// Update all systems
    pub fn update(&mut self, dt: f32) {
        // For Phase 1, we'll implement a simplified system update
        // This avoids the borrowing complexity for now
        // In a real ECS, this would use a more sophisticated approach
        
        // For now, we'll just log that systems would be updated
        // TODO: Implement proper system execution in Phase 2
        log::debug!("Updating {} systems", self.systems.len());
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
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
