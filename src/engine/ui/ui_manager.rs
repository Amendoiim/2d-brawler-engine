//! UI Manager
//! 
//! This module provides the main UI management system that coordinates all UI subsystems.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

use super::{
    UIConfig, UIElementType, UIAnimationType, UITransitionType, UIVisualFeedbackType,
    UIEvent, UIResult, UIError, MouseButton
};

use crate::engine::ui::{
    animations::UIAnimationManager,
    transitions::UITransitionManager,
    visual_feedback::UIVisualFeedbackManager,
    themes::UIThemeManager,
    components::UIComponentManager,
};

/// UI manager
pub struct UIManager {
    /// UI configuration
    pub config: UIConfig,
    /// Animation manager
    pub animations: UIAnimationManager,
    /// Transition manager
    pub transitions: UITransitionManager,
    /// Visual feedback manager
    pub visual_feedback: UIVisualFeedbackManager,
    /// Theme manager
    pub themes: UIThemeManager,
    /// Component manager
    pub components: UIComponentManager,
    /// UI elements
    pub elements: HashMap<String, UIElement>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&UIEvent) + Send + Sync>>,
    /// Input state
    pub input_state: UIInputState,
    /// Render state
    pub render_state: UIRenderState,
}

/// UI element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElement {
    /// Element ID
    pub id: String,
    /// Element type
    pub element_type: UIElementType,
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
    /// Z-index
    pub z_index: i32,
    /// Parent element ID
    pub parent: Option<String>,
    /// Child element IDs
    pub children: Vec<String>,
    /// Element properties
    pub properties: HashMap<String, String>,
    /// Element styles
    pub styles: HashMap<String, String>,
    /// Element data
    pub data: HashMap<String, String>,
}

/// UI input state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIInputState {
    /// Mouse position
    pub mouse_position: (f32, f32),
    /// Mouse buttons pressed
    pub mouse_buttons_pressed: Vec<MouseButton>,
    /// Mouse buttons just pressed
    pub mouse_buttons_just_pressed: Vec<MouseButton>,
    /// Mouse buttons just released
    pub mouse_buttons_just_released: Vec<MouseButton>,
    /// Keyboard keys pressed
    pub keyboard_keys_pressed: Vec<String>,
    /// Keyboard keys just pressed
    pub keyboard_keys_just_pressed: Vec<String>,
    /// Keyboard keys just released
    pub keyboard_keys_just_released: Vec<String>,
    /// Text input
    pub text_input: String,
    /// Scroll delta
    pub scroll_delta: f32,
    /// Touch points
    pub touch_points: Vec<TouchPoint>,
}

/// Touch point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TouchPoint {
    /// Touch ID
    pub id: u32,
    /// Position
    pub position: (f32, f32),
    /// Pressure
    pub pressure: f32,
    /// Active
    pub active: bool,
}

/// UI render state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIRenderState {
    /// Viewport size
    pub viewport_size: (f32, f32),
    /// UI scale
    pub ui_scale: f32,
    /// DPI scale
    pub dpi_scale: f32,
    /// Render target
    pub render_target: String,
    /// Render quality
    pub render_quality: RenderQuality,
    /// Anti-aliasing enabled
    pub anti_aliasing: bool,
    /// VSync enabled
    pub vsync: bool,
}

/// Render quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RenderQuality {
    /// Low quality
    Low,
    /// Medium quality
    Medium,
    /// High quality
    High,
    /// Ultra quality
    Ultra,
}

impl UIManager {
    /// Create a new UI manager
    pub fn new(config: UIConfig) -> Self {
        Self {
            config,
            animations: UIAnimationManager::new(),
            transitions: UITransitionManager::new(),
            visual_feedback: UIVisualFeedbackManager::new(),
            themes: UIThemeManager::new(),
            components: UIComponentManager::new(),
            elements: HashMap::new(),
            event_handlers: Vec::new(),
            input_state: UIInputState::default(),
            render_state: UIRenderState::default(),
        }
    }

    /// Update UI manager
    pub fn update(&mut self, delta_time: f32) -> UIResult<()> {
        // Update subsystems
        self.animations.update(delta_time)?;
        self.transitions.update(delta_time)?;
        self.visual_feedback.update(delta_time)?;

        // Update UI elements
        self.update_elements(delta_time)?;

        // Process input
        self.process_input()?;

        // Update render state
        self.update_render_state()?;

        Ok(())
    }

    /// Create UI element
    pub fn create_element(&mut self, id: &str, element_type: UIElementType, position: (f32, f32), size: (f32, f32)) -> UIResult<()> {
        if self.elements.contains_key(id) {
            return Err(UIError::InvalidConfig(format!("Element '{}' already exists", id)));
        }

        let element = UIElement {
            id: id.to_string(),
            element_type,
            position,
            size,
            visible: true,
            enabled: true,
            focused: false,
            hovered: false,
            pressed: false,
            z_index: 0,
            parent: None,
            children: Vec::new(),
            properties: HashMap::new(),
            styles: HashMap::new(),
            data: HashMap::new(),
        };

        self.elements.insert(id.to_string(), element);
        Ok(())
    }

    /// Remove UI element
    pub fn remove_element(&mut self, id: &str) -> UIResult<()> {
        if !self.elements.contains_key(id) {
            return Err(UIError::ElementNotFound(id.to_string()));
        }

        // Remove from parent
        if let Some(element) = self.elements.get(id) {
            if let Some(parent_id) = &element.parent {
                if let Some(parent) = self.elements.get_mut(parent_id) {
                    parent.children.retain(|child_id| child_id != id);
                }
            }
        }

        // Remove children
        if let Some(element) = self.elements.get(id) {
            for child_id in &element.children {
                self.remove_element(child_id)?;
            }
        }

        self.elements.remove(id);
        Ok(())
    }

    /// Get UI element
    pub fn get_element(&self, id: &str) -> Option<&UIElement> {
        self.elements.get(id)
    }

    /// Get UI element mutably
    pub fn get_element_mut(&mut self, id: &str) -> Option<&mut UIElement> {
        self.elements.get_mut(id)
    }

    /// Set element position
    pub fn set_element_position(&mut self, id: &str, position: (f32, f32)) -> UIResult<()> {
        let element = self.elements.get_mut(id)
            .ok_or_else(|| UIError::ElementNotFound(id.to_string()))?;

        element.position = position;
        Ok(())
    }

    /// Set element size
    pub fn set_element_size(&mut self, id: &str, size: (f32, f32)) -> UIResult<()> {
        let element = self.elements.get_mut(id)
            .ok_or_else(|| UIError::ElementNotFound(id.to_string()))?;

        element.size = size;
        Ok(())
    }

    /// Set element visibility
    pub fn set_element_visibility(&mut self, id: &str, visible: bool) -> UIResult<()> {
        let element = self.elements.get_mut(id)
            .ok_or_else(|| UIError::ElementNotFound(id.to_string()))?;

        element.visible = visible;
        Ok(())
    }

    /// Set element enabled state
    pub fn set_element_enabled(&mut self, id: &str, enabled: bool) -> UIResult<()> {
        let element = self.elements.get_mut(id)
            .ok_or_else(|| UIError::ElementNotFound(id.to_string()))?;

        element.enabled = enabled;
        Ok(())
    }

    /// Set element focus
    pub fn set_element_focus(&mut self, id: &str, focused: bool) -> UIResult<()> {
        let element = self.elements.get_mut(id)
            .ok_or_else(|| UIError::ElementNotFound(id.to_string()))?;

        element.focused = focused;
        Ok(())
    }

    /// Set element hover state
    pub fn set_element_hover(&mut self, id: &str, hovered: bool) -> UIResult<()> {
        let element = self.elements.get_mut(id)
            .ok_or_else(|| UIError::ElementNotFound(id.to_string()))?;

        element.hovered = hovered;
        Ok(())
    }

    /// Set element pressed state
    pub fn set_element_pressed(&mut self, id: &str, pressed: bool) -> UIResult<()> {
        let element = self.elements.get_mut(id)
            .ok_or_else(|| UIError::ElementNotFound(id.to_string()))?;

        element.pressed = pressed;
        Ok(())
    }

    /// Set element z-index
    pub fn set_element_z_index(&mut self, id: &str, z_index: i32) -> UIResult<()> {
        let element = self.elements.get_mut(id)
            .ok_or_else(|| UIError::ElementNotFound(id.to_string()))?;

        element.z_index = z_index;
        Ok(())
    }

    /// Set element property
    pub fn set_element_property(&mut self, id: &str, key: &str, value: &str) -> UIResult<()> {
        let element = self.elements.get_mut(id)
            .ok_or_else(|| UIError::ElementNotFound(id.to_string()))?;

        element.properties.insert(key.to_string(), value.to_string());
        Ok(())
    }

    /// Set element style
    pub fn set_element_style(&mut self, id: &str, key: &str, value: &str) -> UIResult<()> {
        let element = self.elements.get_mut(id)
            .ok_or_else(|| UIError::ElementNotFound(id.to_string()))?;

        element.styles.insert(key.to_string(), value.to_string());
        Ok(())
    }

    /// Set element data
    pub fn set_element_data(&mut self, id: &str, key: &str, value: &str) -> UIResult<()> {
        let element = self.elements.get_mut(id)
            .ok_or_else(|| UIError::ElementNotFound(id.to_string()))?;

        element.data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    /// Add child element
    pub fn add_child_element(&mut self, parent_id: &str, child_id: &str) -> UIResult<()> {
        let parent = self.elements.get_mut(parent_id)
            .ok_or_else(|| UIError::ElementNotFound(parent_id.to_string()))?;

        let child = self.elements.get_mut(child_id)
            .ok_or_else(|| UIError::ElementNotFound(child_id.to_string()))?;

        parent.children.push(child_id.to_string());
        child.parent = Some(parent_id.to_string());
        Ok(())
    }

    /// Remove child element
    pub fn remove_child_element(&mut self, parent_id: &str, child_id: &str) -> UIResult<()> {
        let parent = self.elements.get_mut(parent_id)
            .ok_or_else(|| UIError::ElementNotFound(parent_id.to_string()))?;

        let child = self.elements.get_mut(child_id)
            .ok_or_else(|| UIError::ElementNotFound(child_id.to_string()))?;

        parent.children.retain(|id| id != child_id);
        child.parent = None;
        Ok(())
    }

    /// Start animation
    pub fn start_animation(&mut self, element_id: &str, animation_name: &str) -> UIResult<()> {
        self.animations.start_animation(element_id, animation_name)
    }

    /// Start transition
    pub fn start_transition(&mut self, from_element: &str, to_element: &str, transition_name: &str) -> UIResult<()> {
        self.transitions.start_transition(from_element, to_element, transition_name)
    }

    /// Trigger visual feedback
    pub fn trigger_visual_feedback(&mut self, element_id: &str, feedback_name: &str, intensity: f32) -> UIResult<()> {
        self.visual_feedback.trigger_feedback(element_id, feedback_name, intensity)
    }

    /// Set theme
    pub fn set_theme(&mut self, theme_name: &str) -> UIResult<()> {
        self.themes.set_theme(theme_name)
    }

    /// Get current theme
    pub fn get_current_theme(&self) -> &crate::engine::ui::themes::UITheme {
        self.themes.get_current_theme()
    }

    /// Set UI scale
    pub fn set_ui_scale(&mut self, scale: f32) -> UIResult<()> {
        if scale <= 0.0 {
            return Err(UIError::InvalidConfig("UI scale must be positive".to_string()));
        }

        let old_scale = self.config.ui_scale;
        self.config.ui_scale = scale;
        self.render_state.ui_scale = scale;

        // Emit scale changed event
        self.emit_event(UIEvent::UIScaleChanged {
            old_scale,
            new_scale: scale,
        });

        Ok(())
    }

    /// Get UI scale
    pub fn get_ui_scale(&self) -> f32 {
        self.config.ui_scale
    }

    /// Update elements
    fn update_elements(&mut self, delta_time: f32) -> UIResult<()> {
        // Update element states based on input
        for (id, element) in self.elements.iter_mut() {
            // Update hover state
            let is_hovered = self.is_point_in_element(self.input_state.mouse_position, element);
            element.hovered = is_hovered;

            // Update pressed state
            if is_hovered && self.input_state.mouse_buttons_pressed.contains(&MouseButton::Left) {
                element.pressed = true;
            } else if !self.input_state.mouse_buttons_pressed.contains(&MouseButton::Left) {
                element.pressed = false;
            }

            // Emit events
            if is_hovered && !element.hovered {
                self.emit_event(UIEvent::ElementHovered {
                    element_id: id.clone(),
                });
            }

            if element.pressed && self.input_state.mouse_buttons_just_pressed.contains(&MouseButton::Left) {
                self.emit_event(UIEvent::ElementClicked {
                    element_id: id.clone(),
                    button: MouseButton::Left,
                });
            }
        }

        Ok(())
    }

    /// Process input
    fn process_input(&mut self) -> UIResult<()> {
        // Clear just pressed/released states
        self.input_state.mouse_buttons_just_pressed.clear();
        self.input_state.mouse_buttons_just_released.clear();
        self.input_state.keyboard_keys_just_pressed.clear();
        self.input_state.keyboard_keys_just_released.clear();

        // In a real implementation, this would process actual input events
        // For now, we'll just maintain the current state

        Ok(())
    }

    /// Update render state
    fn update_render_state(&mut self) -> UIResult<()> {
        // Update render state based on configuration
        self.render_state.ui_scale = self.config.ui_scale;
        self.render_state.anti_aliasing = self.config.animations_enabled;
        self.render_state.vsync = self.config.transitions_enabled;

        Ok(())
    }

    /// Check if point is in element
    fn is_point_in_element(&self, point: (f32, f32), element: &UIElement) -> bool {
        let (px, py) = point;
        let (ex, ey) = element.position;
        let (ew, eh) = element.size;

        px >= ex && px <= ex + ew && py >= ey && py <= ey + eh
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&UIEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Emit UI event
    fn emit_event(&self, event: UIEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for UIInputState {
    fn default() -> Self {
        Self {
            mouse_position: (0.0, 0.0),
            mouse_buttons_pressed: Vec::new(),
            mouse_buttons_just_pressed: Vec::new(),
            mouse_buttons_just_released: Vec::new(),
            keyboard_keys_pressed: Vec::new(),
            keyboard_keys_just_pressed: Vec::new(),
            keyboard_keys_just_released: Vec::new(),
            text_input: String::new(),
            scroll_delta: 0.0,
            touch_points: Vec::new(),
        }
    }
}

impl Default for UIRenderState {
    fn default() -> Self {
        Self {
            viewport_size: (1920.0, 1080.0),
            ui_scale: 1.0,
            dpi_scale: 1.0,
            render_target: "default".to_string(),
            render_quality: RenderQuality::High,
            anti_aliasing: true,
            vsync: true,
        }
    }
}

impl Default for UIManager {
    fn default() -> Self {
        Self::new(UIConfig::default())
    }
}
