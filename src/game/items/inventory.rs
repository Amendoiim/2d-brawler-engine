//! Inventory Management System
//! 
//! This module provides inventory management for items, including storage, organization,
//! filtering, sorting, and item operations like stacking, splitting, and transferring.

use crate::engine::ecs::{Component, Entity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Item, ItemType, ItemRarity};

/// Inventory component for entities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    pub items: HashMap<u32, InventorySlot>,
    pub max_slots: u32,
    pub used_slots: u32,
    pub total_weight: f32,
    pub max_weight: f32,
    pub filters: InventoryFilters,
    pub sort_order: SortOrder,
}

/// Individual inventory slot
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventorySlot {
    pub item_id: String,
    pub quantity: u32,
    pub slot_index: u32,
    pub locked: bool,
    pub favorite: bool,
}

/// Inventory filters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryFilters {
    pub item_type_filter: Option<ItemType>,
    pub rarity_filter: Option<ItemRarity>,
    pub name_filter: Option<String>,
    pub show_equipped: bool,
    pub show_consumables: bool,
    pub show_materials: bool,
    pub show_quest_items: bool,
}

/// Sort order for inventory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SortOrder {
    Name,
    Type,
    Rarity,
    Value,
    Weight,
    Quantity,
    DateAcquired,
    Custom,
}

/// Inventory manager for handling inventory operations
#[derive(Debug, Clone)]
pub struct InventoryManager {
    pub inventories: HashMap<Entity, Inventory>,
    pub item_database: HashMap<String, Item>,
}

/// Inventory operation result
#[derive(Debug, Clone, PartialEq)]
pub enum InventoryResult {
    Success,
    InventoryFull,
    ItemNotFound,
    InsufficientQuantity,
    WeightExceeded,
    InvalidSlot,
    SlotLocked,
    CannotStack,
}

impl Inventory {
    /// Create new inventory
    pub fn new(max_slots: u32, max_weight: f32) -> Self {
        Self {
            items: HashMap::new(),
            max_slots,
            used_slots: 0,
            total_weight: 0.0,
            max_weight,
            filters: InventoryFilters::default(),
            sort_order: SortOrder::Name,
        }
    }
    
    /// Add item to inventory
    pub fn add_item(&mut self, item_id: String, quantity: u32, items: &HashMap<String, Item>) -> InventoryResult {
        // Check if item exists
        let item = match items.get(&item_id) {
            Some(item) => item,
            None => return InventoryResult::ItemNotFound,
        };
        
        // Check weight limit
        let item_weight = self.calculate_item_weight(item, quantity);
        if self.total_weight + item_weight > self.max_weight {
            return InventoryResult::WeightExceeded;
        }
        
        // Try to stack with existing items first
        if item.max_stack > 1 {
            for (slot_index, slot) in &mut self.items {
                if slot.item_id == item_id && !slot.locked {
                    let can_add = (slot.quantity + quantity).min(item.max_stack) - slot.quantity;
                    if can_add > 0 {
                        slot.quantity += can_add;
                        self.total_weight += self.calculate_item_weight(item, can_add);
                        
                        if can_add < quantity {
                            return self.add_item(item_id, quantity - can_add, items);
                        }
                        return InventoryResult::Success;
                    }
                }
            }
        }
        
        // Find empty slot
        if self.used_slots >= self.max_slots {
            return InventoryResult::InventoryFull;
        }
        
        let slot_index = self.find_empty_slot();
        self.items.insert(slot_index, InventorySlot {
            item_id: item_id.clone(),
            quantity,
            slot_index,
            locked: false,
            favorite: false,
        });
        
        self.used_slots += 1;
        self.total_weight += item_weight;
        
        InventoryResult::Success
    }
    
    /// Remove item from inventory
    pub fn remove_item(&mut self, slot_index: u32, quantity: u32, items: &HashMap<String, Item>) -> InventoryResult {
        let item_id = if let Some(slot) = self.items.get(&slot_index) {
            if slot.locked {
                return InventoryResult::SlotLocked;
            }
            if slot.quantity < quantity {
                return InventoryResult::InsufficientQuantity;
            }
            slot.item_id.clone()
        } else {
            return InventoryResult::InvalidSlot;
        };
        
        let item = match items.get(&item_id) {
            Some(item) => item,
            None => return InventoryResult::ItemNotFound,
        };
        
        if let Some(slot) = self.items.get_mut(&slot_index) {
            slot.quantity -= quantity;
        }
        
        self.total_weight -= self.calculate_item_weight(item, quantity);
        
        if let Some(slot) = self.items.get(&slot_index) {
            if slot.quantity == 0 {
                self.items.remove(&slot_index);
                self.used_slots -= 1;
            }
        }
        
        InventoryResult::Success
    }
    
    /// Move item between slots
    pub fn move_item(&mut self, from_slot: u32, to_slot: u32, items: &HashMap<String, Item>) -> InventoryResult {
        let from_item = match self.items.get(&from_slot) {
            Some(item) => item.clone(),
            None => return InventoryResult::InvalidSlot,
        };
        
        if from_item.locked {
            return InventoryResult::SlotLocked;
        }
        
        // Check if destination slot is empty
        if !self.items.contains_key(&to_slot) {
            // Move to empty slot
            self.items.remove(&from_slot);
            self.items.insert(to_slot, InventorySlot {
                slot_index: to_slot,
                ..from_item
            });
            return InventoryResult::Success;
        }
        
        let to_item = self.items.get(&to_slot).unwrap();
        if to_item.locked {
            return InventoryResult::SlotLocked;
        }
        
        // Check if items can be stacked
        if from_item.item_id == to_item.item_id {
            let item = match items.get(&from_item.item_id) {
                Some(item) => item,
                None => return InventoryResult::ItemNotFound,
            };
            
            let can_stack = (to_item.quantity + from_item.quantity).min(item.max_stack) - to_item.quantity;
            if can_stack > 0 {
                // Stack items
                self.items.get_mut(&to_slot).unwrap().quantity += can_stack;
                
                if can_stack < from_item.quantity {
                    // Partial stack, update from slot
                    self.items.get_mut(&from_slot).unwrap().quantity -= can_stack;
                } else {
                    // Full stack, remove from slot
                    self.items.remove(&from_slot);
                    self.used_slots -= 1;
                }
                
                return InventoryResult::Success;
            }
        }
        
        // Swap items
        let mut to_item = self.items.remove(&to_slot).unwrap();
        let mut from_item = self.items.remove(&from_slot).unwrap();
        
        std::mem::swap(&mut to_item.slot_index, &mut from_item.slot_index);
        
        self.items.insert(to_slot, to_item);
        self.items.insert(from_slot, from_item);
        
        InventoryResult::Success
    }
    
    /// Split item stack
    pub fn split_item(&mut self, slot_index: u32, quantity: u32, items: &HashMap<String, Item>) -> InventoryResult {
        let slot = match self.items.get(&slot_index) {
            Some(slot) => slot,
            None => return InventoryResult::InvalidSlot,
        };
        
        if slot.locked {
            return InventoryResult::SlotLocked;
        }
        
        if slot.quantity <= quantity {
            return InventoryResult::InsufficientQuantity;
        }
        
        if self.used_slots >= self.max_slots {
            return InventoryResult::InventoryFull;
        }
        
        let new_slot_index = self.find_empty_slot();
        let mut new_slot = slot.clone();
        new_slot.quantity = quantity;
        new_slot.slot_index = new_slot_index;
        
        self.items.insert(new_slot_index, new_slot);
        self.items.get_mut(&slot_index).unwrap().quantity -= quantity;
        self.used_slots += 1;
        
        InventoryResult::Success
    }
    
    /// Lock/unlock a slot
    pub fn toggle_slot_lock(&mut self, slot_index: u32) -> InventoryResult {
        match self.items.get_mut(&slot_index) {
            Some(slot) => {
                slot.locked = !slot.locked;
                InventoryResult::Success
            }
            None => InventoryResult::InvalidSlot,
        }
    }
    
    /// Toggle favorite status
    pub fn toggle_favorite(&mut self, slot_index: u32) -> InventoryResult {
        match self.items.get_mut(&slot_index) {
            Some(slot) => {
                slot.favorite = !slot.favorite;
                InventoryResult::Success
            }
            None => InventoryResult::InvalidSlot,
        }
    }
    
    /// Get item in slot
    pub fn get_item(&self, slot_index: u32) -> Option<&InventorySlot> {
        self.items.get(&slot_index)
    }
    
    /// Get all items matching filters
    pub fn get_filtered_items(&self, items: &HashMap<String, Item>) -> Vec<&InventorySlot> {
        let mut filtered_items: Vec<&InventorySlot> = self.items.values().collect();
        
        // Apply filters
        if let Some(item_type) = &self.filters.item_type_filter {
            filtered_items.retain(|slot| {
                items.get(&slot.item_id)
                    .map(|item| &item.item_type == item_type)
                    .unwrap_or(false)
            });
        }
        
        if let Some(rarity) = &self.filters.rarity_filter {
            filtered_items.retain(|slot| {
                items.get(&slot.item_id)
                    .map(|item| &item.rarity == rarity)
                    .unwrap_or(false)
            });
        }
        
        if let Some(name_filter) = &self.filters.name_filter {
            filtered_items.retain(|slot| {
                items.get(&slot.item_id)
                    .map(|item| item.name.to_lowercase().contains(&name_filter.to_lowercase()))
                    .unwrap_or(false)
            });
        }
        
        // Sort items
        self.sort_items(&mut filtered_items, items);
        
        filtered_items
    }
    
    /// Sort items based on current sort order
    fn sort_items(&self, items: &mut Vec<&InventorySlot>, item_database: &HashMap<String, Item>) {
        match self.sort_order {
            SortOrder::Name => {
                items.sort_by(|a, b| {
                    let empty_string = String::new();
                    let name_a = item_database.get(&a.item_id).map(|i| &i.name).unwrap_or(&empty_string);
                    let name_b = item_database.get(&b.item_id).map(|i| &i.name).unwrap_or(&empty_string);
                    name_a.cmp(name_b)
                });
            }
            SortOrder::Type => {
                items.sort_by(|a, b| {
                    let type_a = item_database.get(&a.item_id).map(|i| &i.item_type);
                    let type_b = item_database.get(&b.item_id).map(|i| &i.item_type);
                    match (type_a, type_b) {
                        (Some(ta), Some(tb)) => format!("{:?}", ta).cmp(&format!("{:?}", tb)),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                });
            }
            SortOrder::Rarity => {
                items.sort_by(|a, b| {
                    let rarity_a = item_database.get(&a.item_id).map(|i| &i.rarity);
                    let rarity_b = item_database.get(&b.item_id).map(|i| &i.rarity);
                    rarity_a.cmp(&rarity_b)
                });
            }
            SortOrder::Value => {
                items.sort_by(|a, b| {
                    let value_a = item_database.get(&a.item_id).map(|i| i.value).unwrap_or(0);
                    let value_b = item_database.get(&b.item_id).map(|i| i.value).unwrap_or(0);
                    value_b.cmp(&value_a) // Descending order
                });
            }
            SortOrder::Quantity => {
                items.sort_by(|a, b| b.quantity.cmp(&a.quantity));
            }
            _ => {} // Custom sorting or no sorting
        }
    }
    
    /// Find empty slot index
    fn find_empty_slot(&self) -> u32 {
        for i in 0..self.max_slots {
            if !self.items.contains_key(&i) {
                return i;
            }
        }
        self.max_slots // Should never reach here if inventory is not full
    }
    
    /// Calculate item weight
    fn calculate_item_weight(&self, item: &Item, quantity: u32) -> f32 {
        // Simple weight calculation - can be enhanced with actual weight values
        let base_weight = match item.item_type {
            ItemType::Weapon(_) => 2.0,
            ItemType::Armor(_) => 1.5,
            ItemType::Accessory(_) => 0.5,
            ItemType::Consumable(_) => 0.2,
            ItemType::Material(_) => 0.1,
            ItemType::Quest(_) => 0.3,
            ItemType::Misc(_) => 0.1,
        };
        
        base_weight * quantity as f32
    }
    
    /// Get available space
    pub fn get_available_slots(&self) -> u32 {
        self.max_slots - self.used_slots
    }
    
    /// Get available weight
    pub fn get_available_weight(&self) -> f32 {
        self.max_weight - self.total_weight
    }
    
    /// Check if inventory is full
    pub fn is_full(&self) -> bool {
        self.used_slots >= self.max_slots
    }
    
    /// Check if weight limit is exceeded
    pub fn is_overweight(&self) -> bool {
        self.total_weight > self.max_weight
    }
    
    /// Get inventory statistics for this inventory
    pub fn get_stats(&self) -> InventoryStats {
        let mut stats = InventoryStats {
            total_items: 0,
            total_value: 0,
            total_weight: self.total_weight,
            max_weight: self.max_weight,
            used_slots: self.used_slots,
            max_slots: self.max_slots,
            rarity_counts: HashMap::new(),
            type_counts: HashMap::new(),
        };
        
        for slot in self.items.values() {
            stats.total_items += slot.quantity;
            // Note: value calculation would need item database
        }
        
        stats
    }
}

impl Default for InventoryFilters {
    fn default() -> Self {
        Self {
            item_type_filter: None,
            rarity_filter: None,
            name_filter: None,
            show_equipped: true,
            show_consumables: true,
            show_materials: true,
            show_quest_items: true,
        }
    }
}

impl InventoryManager {
    /// Create new inventory manager
    pub fn new() -> Self {
        Self {
            inventories: HashMap::new(),
            item_database: HashMap::new(),
        }
    }
    
    /// Add inventory for entity
    pub fn add_inventory(&mut self, entity: Entity, inventory: Inventory) {
        self.inventories.insert(entity, inventory);
    }
    
    /// Get inventory for entity
    pub fn get_inventory(&self, entity: Entity) -> Option<&Inventory> {
        self.inventories.get(&entity)
    }
    
    /// Get mutable inventory for entity
    pub fn get_inventory_mut(&mut self, entity: Entity) -> Option<&mut Inventory> {
        self.inventories.get_mut(&entity)
    }
    
    /// Add item to entity's inventory
    pub fn add_item_to_inventory(&mut self, entity: Entity, item_id: String, quantity: u32) -> InventoryResult {
        if let Some(inventory) = self.inventories.get_mut(&entity) {
            inventory.add_item(item_id, quantity, &self.item_database)
        } else {
            InventoryResult::InvalidSlot
        }
    }
    
    /// Remove item from entity's inventory
    pub fn remove_item_from_inventory(&mut self, entity: Entity, slot_index: u32, quantity: u32) -> InventoryResult {
        if let Some(inventory) = self.inventories.get_mut(&entity) {
            inventory.remove_item(slot_index, quantity, &self.item_database)
        } else {
            InventoryResult::InvalidSlot
        }
    }
    
    /// Transfer item between inventories
    pub fn transfer_item(&mut self, from_entity: Entity, to_entity: Entity, slot_index: u32, quantity: u32) -> InventoryResult {
        // Get item from source inventory
        let item = if let Some(inventory) = self.inventories.get(&from_entity) {
            if let Some(slot) = inventory.get_item(slot_index) {
                slot.clone()
            } else {
                return InventoryResult::InvalidSlot;
            }
        } else {
            return InventoryResult::InvalidSlot;
        };
        
        // Remove from source
        let remove_result = self.remove_item_from_inventory(from_entity, slot_index, quantity);
        if remove_result != InventoryResult::Success {
            return remove_result;
        }
        
        // Add to destination
        self.add_item_to_inventory(to_entity, item.item_id, quantity)
    }
    
    /// Update item database
    pub fn update_item_database(&mut self, items: HashMap<String, Item>) {
        self.item_database = items;
    }
    
    /// Get item from database
    pub fn get_item(&self, item_id: &str) -> Option<&Item> {
        self.item_database.get(item_id)
    }
    
    /// Search items in inventory
    pub fn search_items(&self, entity: Entity, query: &str) -> Vec<&InventorySlot> {
        if let Some(inventory) = self.inventories.get(&entity) {
            inventory.items.values()
                .filter(|slot| {
                    self.item_database.get(&slot.item_id)
                        .map(|item| item.name.to_lowercase().contains(&query.to_lowercase()) ||
                                   item.description.to_lowercase().contains(&query.to_lowercase()))
                        .unwrap_or(false)
                })
                .collect()
        } else {
            vec![]
        }
    }
    

    /// Get inventory statistics
    pub fn get_inventory_stats(&self, entity: Entity) -> Option<InventoryStats> {
        if let Some(inventory) = self.inventories.get(&entity) {
            let mut stats = InventoryStats {
                total_items: 0,
                total_value: 0,
                total_weight: inventory.total_weight,
                max_weight: inventory.max_weight,
                used_slots: inventory.used_slots,
                max_slots: inventory.max_slots,
                rarity_counts: HashMap::new(),
                type_counts: HashMap::new(),
            };
            
            for slot in inventory.items.values() {
                stats.total_items += slot.quantity;
                
                if let Some(item) = self.item_database.get(&slot.item_id) {
                    stats.total_value += item.value * slot.quantity;
                    
                    *stats.rarity_counts.entry(item.rarity.clone()).or_insert(0) += slot.quantity;
                    *stats.type_counts.entry(item.item_type.clone()).or_insert(0) += slot.quantity;
                }
            }
            
            Some(stats)
        } else {
            None
        }
    }
}

/// Inventory statistics
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryStats {
    pub total_items: u32,
    pub total_value: u32,
    pub total_weight: f32,
    pub max_weight: f32,
    pub used_slots: u32,
    pub max_slots: u32,
    pub rarity_counts: HashMap<ItemRarity, u32>,
    pub type_counts: HashMap<ItemType, u32>,
}

impl Default for InventoryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Inventory {}
