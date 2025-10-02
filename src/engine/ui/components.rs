//! UI Components
//! 
//! This module provides comprehensive UI component system with reusable UI elements.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{UIElementType, UIResult, UIError, UIEvent};

/// UI component manager
pub struct UIComponentManager {
    /// Registered components
    pub components: HashMap<String, UIComponent>,
    /// Component instances
    pub instances: HashMap<String, UIComponentInstance>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&UIEvent) + Send + Sync>>,
}

/// UI component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIComponent {
    /// Component name
    pub name: String,
    /// Component type
    pub component_type: UIElementType,
    /// Component properties
    pub properties: ComponentProperties,
    /// Component states
    pub states: HashMap<String, ComponentState>,
    /// Component variants
    pub variants: HashMap<String, ComponentVariant>,
    /// Component sizes
    pub sizes: HashMap<String, ComponentSize>,
    /// Child components
    pub children: Vec<String>,
    /// Event handlers
    pub event_handlers: HashMap<String, String>,
}

/// UI component instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIComponentInstance {
    /// Instance ID
    pub id: String,
    /// Component name
    pub component_name: String,
    /// Instance properties
    pub properties: HashMap<String, ComponentPropertyValue>,
    /// Current state
    pub current_state: String,
    /// Current variant
    pub current_variant: String,
    /// Current size
    pub current_size: String,
    /// Position
    pub position: (f32, f32),
    /// Size
    pub size: (f32, f32),
    /// Visible
    pub visible: bool,
    /// Enabled
    pub enabled: bool,
    /// Focused
    pub focused: bool,
    /// Hovered
    pub hovered: bool,
    /// Pressed
    pub pressed: bool,
    /// Child instances
    pub children: Vec<String>,
}

/// Component properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProperties {
    /// Base properties
    pub base: HashMap<String, ComponentProperty>,
    /// Required properties
    pub required: Vec<String>,
    /// Optional properties
    pub optional: Vec<String>,
    /// Default values
    pub defaults: HashMap<String, ComponentPropertyValue>,
}

/// Component property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProperty {
    /// Property name
    pub name: String,
    /// Property type
    pub property_type: PropertyType,
    /// Property description
    pub description: String,
    /// Default value
    pub default_value: ComponentPropertyValue,
    /// Validation rules
    pub validation: Option<ValidationRules>,
}

/// Component state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentState {
    /// State name
    pub name: String,
    /// State properties
    pub properties: HashMap<String, ComponentPropertyValue>,
    /// State transitions
    pub transitions: HashMap<String, StateTransition>,
}

/// Component variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentVariant {
    /// Variant name
    pub name: String,
    /// Variant properties
    pub properties: HashMap<String, ComponentPropertyValue>,
    /// Variant description
    pub description: String,
}

/// Component size
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSize {
    /// Size name
    pub name: String,
    /// Size properties
    pub properties: HashMap<String, ComponentPropertyValue>,
    /// Size description
    pub description: String,
}

/// State transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    /// From state
    pub from: String,
    /// To state
    pub to: String,
    /// Transition condition
    pub condition: TransitionCondition,
    /// Transition duration
    pub duration: f32,
    /// Transition easing
    pub easing: String,
}

/// Transition condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionCondition {
    /// On click
    OnClick,
    /// On hover
    OnHover,
    /// On focus
    OnFocus,
    /// On press
    OnPress,
    /// On release
    OnRelease,
    /// Custom condition
    Custom(String),
}

/// Component property value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentPropertyValue {
    /// String value
    String(String),
    /// Number value
    Number(f32),
    /// Boolean value
    Boolean(bool),
    /// Color value (RGBA)
    Color((f32, f32, f32, f32)),
    /// Vector value (x, y)
    Vector((f32, f32)),
    /// Rectangle value (x, y, width, height)
    Rectangle((f32, f32, f32, f32)),
    /// Array value
    Array(Vec<ComponentPropertyValue>),
    /// Object value
    Object(HashMap<String, ComponentPropertyValue>),
}

/// Property type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyType {
    /// String type
    String,
    /// Number type
    Number,
    /// Boolean type
    Boolean,
    /// Color type
    Color,
    /// Vector type
    Vector,
    /// Rectangle type
    Rectangle,
    /// Array type
    Array,
    /// Object type
    Object,
    /// Enum type
    Enum(Vec<String>),
}

/// Validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    /// Minimum value (for numbers)
    pub min: Option<f32>,
    /// Maximum value (for numbers)
    pub max: Option<f32>,
    /// Minimum length (for strings/arrays)
    pub min_length: Option<usize>,
    /// Maximum length (for strings/arrays)
    pub max_length: Option<usize>,
    /// Required
    pub required: bool,
    /// Pattern (for strings)
    pub pattern: Option<String>,
    /// Custom validation
    pub custom: Option<String>,
}

impl UIComponentManager {
    /// Create a new UI component manager
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            instances: HashMap::new(),
            event_handlers: Vec::new(),
        }
    }

    /// Register component
    pub fn register_component(&mut self, component: UIComponent) -> UIResult<()> {
        if self.components.contains_key(&component.name) {
            return Err(UIError::InvalidConfig(format!("Component '{}' already registered", component.name)));
        }

        self.components.insert(component.name.clone(), component);
        Ok(())
    }

    /// Unregister component
    pub fn unregister_component(&mut self, name: &str) -> UIResult<()> {
        if !self.components.contains_key(name) {
            return Err(UIError::ElementNotFound(name.to_string()));
        }

        // Remove all instances of this component
        self.instances.retain(|_, instance| instance.component_name != name);
        self.components.remove(name);
        Ok(())
    }

    /// Create component instance
    pub fn create_instance(&mut self, component_name: &str, instance_id: &str, properties: HashMap<String, ComponentPropertyValue>) -> UIResult<()> {
        let component = self.components.get(component_name)
            .ok_or_else(|| UIError::ElementNotFound(component_name.to_string()))?;

        // Validate properties
        self.validate_properties(component, &properties)?;

        // Create instance
        let instance = UIComponentInstance {
            id: instance_id.to_string(),
            component_name: component_name.to_string(),
            properties,
            current_state: "default".to_string(),
            current_variant: "default".to_string(),
            current_size: "medium".to_string(),
            position: (0.0, 0.0),
            size: (100.0, 100.0),
            visible: true,
            enabled: true,
            focused: false,
            hovered: false,
            pressed: false,
            children: Vec::new(),
        };

        self.instances.insert(instance_id.to_string(), instance);
        Ok(())
    }

    /// Destroy component instance
    pub fn destroy_instance(&mut self, instance_id: &str) -> UIResult<()> {
        if !self.instances.contains_key(instance_id) {
            return Err(UIError::ElementNotFound(instance_id.to_string()));
        }

        self.instances.remove(instance_id);
        Ok(())
    }

    /// Get component instance
    pub fn get_instance(&self, instance_id: &str) -> Option<&UIComponentInstance> {
        self.instances.get(instance_id)
    }

    /// Get component instance mutably
    pub fn get_instance_mut(&mut self, instance_id: &str) -> Option<&mut UIComponentInstance> {
        self.instances.get_mut(instance_id)
    }

    /// Update instance property
    pub fn update_instance_property(&mut self, instance_id: &str, property_name: &str, value: ComponentPropertyValue) -> UIResult<()> {
        let instance = self.instances.get_mut(instance_id)
            .ok_or_else(|| UIError::ElementNotFound(instance_id.to_string()))?;

        let component = self.components.get(&instance.component_name)
            .ok_or_else(|| UIError::ElementNotFound(instance.component_name.clone()))?;

        // Validate property
        self.validate_property(component, property_name, &value)?;

        instance.properties.insert(property_name.to_string(), value);
        Ok(())
    }

    /// Set instance state
    pub fn set_instance_state(&mut self, instance_id: &str, state_name: &str) -> UIResult<()> {
        let instance = self.instances.get_mut(instance_id)
            .ok_or_else(|| UIError::ElementNotFound(instance_id.to_string()))?;

        let component = self.components.get(&instance.component_name)
            .ok_or_else(|| UIError::ElementNotFound(instance.component_name.clone()))?;

        if !component.states.contains_key(state_name) {
            return Err(UIError::InvalidConfig(format!("State '{}' not found for component '{}'", state_name, instance.component_name)));
        }

        instance.current_state = state_name.to_string();
        Ok(())
    }

    /// Set instance variant
    pub fn set_instance_variant(&mut self, instance_id: &str, variant_name: &str) -> UIResult<()> {
        let instance = self.instances.get_mut(instance_id)
            .ok_or_else(|| UIError::ElementNotFound(instance_id.to_string()))?;

        let component = self.components.get(&instance.component_name)
            .ok_or_else(|| UIError::ElementNotFound(instance.component_name.clone()))?;

        if !component.variants.contains_key(variant_name) {
            return Err(UIError::InvalidConfig(format!("Variant '{}' not found for component '{}'", variant_name, instance.component_name)));
        }

        instance.current_variant = variant_name.to_string();
        Ok(())
    }

    /// Set instance size
    pub fn set_instance_size(&mut self, instance_id: &str, size_name: &str) -> UIResult<()> {
        let instance = self.instances.get_mut(instance_id)
            .ok_or_else(|| UIError::ElementNotFound(instance_id.to_string()))?;

        let component = self.components.get(&instance.component_name)
            .ok_or_else(|| UIError::ElementNotFound(instance.component_name.clone()))?;

        if !component.sizes.contains_key(size_name) {
            return Err(UIError::InvalidConfig(format!("Size '{}' not found for component '{}'", size_name, instance.component_name)));
        }

        instance.current_size = size_name.to_string();
        Ok(())
    }

    /// Set instance position
    pub fn set_instance_position(&mut self, instance_id: &str, position: (f32, f32)) -> UIResult<()> {
        let instance = self.instances.get_mut(instance_id)
            .ok_or_else(|| UIError::ElementNotFound(instance_id.to_string()))?;

        instance.position = position;
        Ok(())
    }

    /// Set instance size
    pub fn set_instance_size_dimensions(&mut self, instance_id: &str, size: (f32, f32)) -> UIResult<()> {
        let instance = self.instances.get_mut(instance_id)
            .ok_or_else(|| UIError::ElementNotFound(instance_id.to_string()))?;

        instance.size = size;
        Ok(())
    }

    /// Set instance visibility
    pub fn set_instance_visibility(&mut self, instance_id: &str, visible: bool) -> UIResult<()> {
        let instance = self.instances.get_mut(instance_id)
            .ok_or_else(|| UIError::ElementNotFound(instance_id.to_string()))?;

        instance.visible = visible;
        Ok(())
    }

    /// Set instance enabled state
    pub fn set_instance_enabled(&mut self, instance_id: &str, enabled: bool) -> UIResult<()> {
        let instance = self.instances.get_mut(instance_id)
            .ok_or_else(|| UIError::ElementNotFound(instance_id.to_string()))?;

        instance.enabled = enabled;
        Ok(())
    }

    /// Get all instances
    pub fn get_all_instances(&self) -> &HashMap<String, UIComponentInstance> {
        &self.instances
    }

    /// Get instances by component name
    pub fn get_instances_by_component(&self, component_name: &str) -> Vec<&UIComponentInstance> {
        self.instances.values()
            .filter(|instance| instance.component_name == component_name)
            .collect()
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&UIEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Validate properties
    fn validate_properties(&self, component: &UIComponent, properties: &HashMap<String, ComponentPropertyValue>) -> UIResult<()> {
        // Check required properties
        for required_prop in &component.properties.required {
            if !properties.contains_key(required_prop) {
                return Err(UIError::InvalidConfig(format!("Required property '{}' not provided", required_prop)));
            }
        }

        // Validate each property
        for (prop_name, prop_value) in properties {
            self.validate_property(component, prop_name, prop_value)?;
        }

        Ok(())
    }

    /// Validate single property
    fn validate_property(&self, component: &UIComponent, property_name: &str, value: &ComponentPropertyValue) -> UIResult<()> {
        if let Some(property) = component.properties.base.get(property_name) {
            // Check type compatibility
            if !self.is_type_compatible(&property.property_type, value) {
                return Err(UIError::InvalidConfig(format!("Property '{}' has incompatible type", property_name)));
            }

            // Apply validation rules
            if let Some(validation) = &property.validation {
                self.apply_validation_rules(validation, value)?;
            }
        } else {
            return Err(UIError::InvalidConfig(format!("Property '{}' not found in component", property_name)));
        }

        Ok(())
    }

    /// Check type compatibility
    fn is_type_compatible(&self, property_type: &PropertyType, value: &ComponentPropertyValue) -> bool {
        match (property_type, value) {
            (PropertyType::String, ComponentPropertyValue::String(_)) => true,
            (PropertyType::Number, ComponentPropertyValue::Number(_)) => true,
            (PropertyType::Boolean, ComponentPropertyValue::Boolean(_)) => true,
            (PropertyType::Color, ComponentPropertyValue::Color(_)) => true,
            (PropertyType::Vector, ComponentPropertyValue::Vector(_)) => true,
            (PropertyType::Rectangle, ComponentPropertyValue::Rectangle(_)) => true,
            (PropertyType::Array, ComponentPropertyValue::Array(_)) => true,
            (PropertyType::Object, ComponentPropertyValue::Object(_)) => true,
            (PropertyType::Enum(_), ComponentPropertyValue::String(_)) => true,
            _ => false,
        }
    }

    /// Apply validation rules
    fn apply_validation_rules(&self, rules: &ValidationRules, value: &ComponentPropertyValue) -> UIResult<()> {
        if rules.required && matches!(value, ComponentPropertyValue::String(s) if s.is_empty()) {
            return Err(UIError::InvalidConfig("Required property cannot be empty".to_string()));
        }

        match value {
            ComponentPropertyValue::Number(n) => {
                if let Some(min) = rules.min {
                    if *n < min {
                        return Err(UIError::InvalidConfig(format!("Value {} is below minimum {}", n, min)));
                    }
                }
                if let Some(max) = rules.max {
                    if *n > max {
                        return Err(UIError::InvalidConfig(format!("Value {} is above maximum {}", n, max)));
                    }
                }
            },
            ComponentPropertyValue::String(s) => {
                if let Some(min_len) = rules.min_length {
                    if s.len() < min_len {
                        return Err(UIError::InvalidConfig(format!("String length {} is below minimum {}", s.len(), min_len)));
                    }
                }
                if let Some(max_len) = rules.max_length {
                    if s.len() > max_len {
                        return Err(UIError::InvalidConfig(format!("String length {} is above maximum {}", s.len(), max_len)));
                    }
                }
            },
            ComponentPropertyValue::Array(arr) => {
                if let Some(min_len) = rules.min_length {
                    if arr.len() < min_len {
                        return Err(UIError::InvalidConfig(format!("Array length {} is below minimum {}", arr.len(), min_len)));
                    }
                }
                if let Some(max_len) = rules.max_length {
                    if arr.len() > max_len {
                        return Err(UIError::InvalidConfig(format!("Array length {} is above maximum {}", arr.len(), max_len)));
                    }
                }
            },
            _ => {}
        }

        Ok(())
    }

    /// Emit UI event
    fn emit_event(&self, event: UIEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for UIComponentManager {
    fn default() -> Self {
        Self::new()
    }
}
