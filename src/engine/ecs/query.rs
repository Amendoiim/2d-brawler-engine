//! ECS Query System - Phase 2 Implementation
//! 
//! This module provides a practical query system that solves borrowing issues
//! by using a different approach to component access and iteration.

use std::collections::HashMap;
use std::any::TypeId;
use crate::engine::ecs::{Component, Entity, World};

/// Query for single component type - Phase 2 simplified approach
pub struct Query<'a, T: Component> {
    world: &'a World,
    entities: Vec<Entity>,
    _phantom: std::marker::PhantomData<T>,
}

/// Mutable query for single component type - Phase 2 simplified approach
pub struct QueryMut<'a, T: Component> {
    world: &'a mut World,
    entities: Vec<Entity>,
    _phantom: std::marker::PhantomData<T>,
}


impl<'a, T: Component> Query<'a, T> {
    /// Create a new query for the specified component type
    pub fn new(world: &'a World) -> Self {
        let entities = world.get_entities_with_component::<T>();
        
        Self {
            world,
            entities,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Get the first entity with this component
    pub fn first(&self) -> Option<Entity> {
        self.entities.first().copied()
    }
    
    /// Get the number of entities with this component
    pub fn count(&self) -> usize {
        self.entities.len()
    }
    
    /// Get all entities with this component
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
    
    /// Get a component for a specific entity
    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.world.get_component::<T>(entity)
    }
}

impl<'a, T: Component> QueryMut<'a, T> {
    /// Create a new mutable query for the specified component type
    pub fn new(world: &'a mut World) -> Self {
        let entities = world.get_entities_with_component::<T>();
        
        Self {
            world,
            entities,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Get a mutable reference to a component for a specific entity
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.world.get_component_mut::<T>(entity)
    }
    
    /// Get the first entity with this component
    pub fn first(&self) -> Option<Entity> {
        self.entities.first().copied()
    }
    
    /// Get the number of entities with this component
    pub fn count(&self) -> usize {
        self.entities.len()
    }
    
    /// Get all entities with this component
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
}

// Extension trait for World to support queries
pub trait WorldQueryExt {
    /// Get entities that have the specified component
    fn get_entities_with_component<T: Component>(&self) -> Vec<Entity>;
    
    /// Create a query for the specified component type
    fn query<T: Component>(&self) -> Query<T>;
    
    /// Create a mutable query for the specified component type
    fn query_mut<T: Component>(&mut self) -> QueryMut<T>;
}

impl WorldQueryExt for World {
    fn get_entities_with_component<T: Component>(&self) -> Vec<Entity> {
        let type_id = TypeId::of::<T>();
        let mut result = Vec::new();
        
        if let Some(component_map) = self.components.get(&type_id) {
            if let Some(map) = component_map.downcast_ref::<HashMap<Entity, T>>() {
                result.extend(map.keys().copied());
            }
        }
        
        result
    }
    
    fn query<T: Component>(&self) -> Query<T> {
        Query::new(self)
    }
    
    fn query_mut<T: Component>(&mut self) -> QueryMut<T> {
        QueryMut::new(self)
    }
}
