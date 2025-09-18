//! Entity-Component-System implementation

use std::collections::HashMap;
use std::any::TypeId;

/// Entity ID type
pub type Entity = u32;

/// Component trait for ECS
pub trait Component: 'static + Send + Sync {}

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
        for system in &mut self.systems {
            system.update(self, dt);
        }
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
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
