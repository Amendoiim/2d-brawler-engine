//! Animation controller for managing entity animations

use crate::engine::animation::{AnimationStateMachine, AnimationTransition, TransitionCondition};
use crate::engine::ecs::Entity;
use std::collections::HashMap;

/// Animation controller for managing entity-specific animation logic
pub struct AnimationController {
    /// Entity this controller belongs to
    pub entity: Entity,
    /// Animation state machine
    pub state_machine: AnimationStateMachine,
    /// Pending animation transition
    pub pending_transition: Option<String>,
    /// Animation event callbacks
    pub event_callbacks: HashMap<String, Vec<AnimationEventCallback>>,
    /// Controller-specific parameters
    pub parameters: HashMap<String, f32>,
    /// Controller-specific flags
    pub flags: HashMap<String, bool>,
}

/// Animation event callback function type
pub type AnimationEventCallback = Box<dyn Fn(&str, &AnimationController) + Send + Sync>;

impl AnimationController {
    /// Create a new animation controller
    pub fn new(entity: Entity, initial_state: String) -> Self {
        Self {
            entity,
            state_machine: AnimationStateMachine::new(initial_state),
            pending_transition: None,
            event_callbacks: HashMap::new(),
            parameters: HashMap::new(),
            flags: HashMap::new(),
        }
    }

    /// Update the animation controller
    pub fn update(&mut self, dt: f32) {
        // Update state machine
        if let Some(new_animation) = self.state_machine.update(dt) {
            self.pending_transition = Some(new_animation);
        }

        // Update controller-specific logic
        self.update_controller_logic(dt);
    }

    /// Update controller-specific logic
    fn update_controller_logic(&mut self, dt: f32) {
        // This method can be overridden for custom controller behavior
        // For now, we'll implement some basic logic
        
        // Example: Update movement-based animations
        if let Some(velocity_x) = self.get_parameter("velocity_x") {
            if velocity_x.abs() > 0.1 {
                self.set_flag("is_moving".to_string(), true);
            } else {
                self.set_flag("is_moving".to_string(), false);
            }
        }

        // Example: Update combat state
        if let Some(attacking) = self.get_flag("is_attacking") {
            if attacking {
                self.set_flag("in_combat".to_string(), true);
            }
        }
    }

    /// Get the initial animation for this controller
    pub fn get_initial_animation(&self) -> Option<String> {
        self.state_machine.get_current_animation()
    }

    /// Get pending transition
    pub fn get_pending_transition(&self) -> Option<String> {
        self.pending_transition.clone()
    }

    /// Clear pending transition
    pub fn clear_pending_transition(&mut self) {
        self.pending_transition = None;
    }

    /// Set a parameter
    pub fn set_parameter(&mut self, name: String, value: f32) {
        self.parameters.insert(name.clone(), value);
        self.state_machine.set_parameter(name, value);
    }

    /// Get a parameter
    pub fn get_parameter(&self, name: &str) -> Option<f32> {
        self.parameters.get(name).copied()
    }

    /// Set a flag
    pub fn set_flag(&mut self, name: String, value: bool) {
        self.flags.insert(name.clone(), value);
        self.state_machine.set_flag(name, value);
    }

    /// Get a flag
    pub fn get_flag(&self, name: &str) -> Option<bool> {
        self.flags.get(name).copied()
    }

    /// Force transition to a state
    pub fn force_transition(&mut self, state_name: String) {
        self.state_machine.force_transition(state_name);
        if let Some(animation) = self.state_machine.get_current_animation() {
            self.pending_transition = Some(animation);
        }
    }

    /// Add an event callback
    pub fn add_event_callback(&mut self, event: String, callback: AnimationEventCallback) {
        self.event_callbacks.entry(event).or_insert_with(Vec::new).push(callback);
    }

    /// Trigger an animation event
    pub fn trigger_event(&self, event: &str) {
        if let Some(callbacks) = self.event_callbacks.get(event) {
            for callback in callbacks {
                callback(event, self);
            }
        }
    }

    /// Handle animation completion
    pub fn on_animation_complete(&mut self, animation_name: &str) {
        // Set animation complete flag
        self.set_flag("animation_complete".to_string(), true);
        
        // Trigger completion event
        self.trigger_event("animation_complete");
        
        // Reset flag after a frame
        self.set_flag("animation_complete".to_string(), false);
    }

    /// Get current animation name
    pub fn get_current_animation(&self) -> Option<String> {
        self.state_machine.get_current_animation()
    }

    /// Get current state
    pub fn get_current_state(&self) -> &String {
        self.state_machine.get_current_state()
    }

    /// Check if currently blending
    pub fn is_blending(&self) -> bool {
        self.state_machine.is_blending()
    }

    /// Get blend weights
    pub fn get_blend_weights(&self) -> Option<(f32, f32)> {
        self.state_machine.get_blend_weights()
    }
}

/// Predefined animation controller types
pub struct AnimationControllerBuilder {
    entity: Entity,
    initial_state: String,
    transitions: Vec<AnimationTransition>,
    parameters: HashMap<String, f32>,
    flags: HashMap<String, bool>,
}

impl AnimationControllerBuilder {
    /// Create a new animation controller builder
    pub fn new(entity: Entity, initial_state: String) -> Self {
        Self {
            entity,
            initial_state,
            transitions: Vec::new(),
            parameters: HashMap::new(),
            flags: HashMap::new(),
        }
    }

    /// Add a transition
    pub fn add_transition(mut self, transition: AnimationTransition) -> Self {
        self.transitions.push(transition);
        self
    }

    /// Add a parameter
    pub fn add_parameter(mut self, name: String, value: f32) -> Self {
        self.parameters.insert(name, value);
        self
    }

    /// Add a flag
    pub fn add_flag(mut self, name: String, value: bool) -> Self {
        self.flags.insert(name, value);
        self
    }

    /// Build the animation controller
    pub fn build(self) -> AnimationController {
        let mut controller = AnimationController::new(self.entity, self.initial_state);
        
        // Add transitions
        for transition in self.transitions {
            controller.state_machine.add_transition(transition);
        }
        
        // Add parameters
        for (name, value) in self.parameters {
            controller.set_parameter(name, value);
        }
        
        // Add flags
        for (name, value) in self.flags {
            controller.set_flag(name, value);
        }
        
        controller
    }
}

/// Common animation controller presets
impl AnimationController {
    /// Create a basic character controller
    pub fn create_character_controller(entity: Entity) -> Self {
        AnimationControllerBuilder::new(entity, "idle".to_string())
            .add_transition(AnimationTransition {
                from: "idle".to_string(),
                to: "walk".to_string(),
                condition: TransitionCondition::OnCondition { condition: "flag_is_moving".to_string() },
                priority: 1,
                blend: true,
                blend_duration: 0.1,
            })
            .add_transition(AnimationTransition {
                from: "walk".to_string(),
                to: "idle".to_string(),
                condition: TransitionCondition::OnCondition { condition: "flag_is_moving".to_string() },
                priority: 1,
                blend: true,
                blend_duration: 0.1,
            })
            .add_transition(AnimationTransition {
                from: "idle".to_string(),
                to: "attack".to_string(),
                condition: TransitionCondition::OnInput { input: "attack".to_string() },
                priority: 2,
                blend: false,
                blend_duration: 0.0,
            })
            .add_transition(AnimationTransition {
                from: "attack".to_string(),
                to: "idle".to_string(),
                condition: TransitionCondition::OnComplete,
                priority: 1,
                blend: false,
                blend_duration: 0.0,
            })
            .add_parameter("velocity_x".to_string(), 0.0)
            .add_parameter("velocity_y".to_string(), 0.0)
            .add_flag("is_moving".to_string(), false)
            .add_flag("is_attacking".to_string(), false)
            .build()
    }

    /// Create a combat controller
    pub fn create_combat_controller(entity: Entity) -> Self {
        AnimationControllerBuilder::new(entity, "combat_idle".to_string())
            .add_transition(AnimationTransition {
                from: "combat_idle".to_string(),
                to: "light_attack".to_string(),
                condition: TransitionCondition::OnInput { input: "light_attack".to_string() },
                priority: 2,
                blend: false,
                blend_duration: 0.0,
            })
            .add_transition(AnimationTransition {
                from: "combat_idle".to_string(),
                to: "heavy_attack".to_string(),
                condition: TransitionCondition::OnInput { input: "heavy_attack".to_string() },
                priority: 2,
                blend: false,
                blend_duration: 0.0,
            })
            .add_transition(AnimationTransition {
                from: "light_attack".to_string(),
                to: "combat_idle".to_string(),
                condition: TransitionCondition::OnComplete,
                priority: 1,
                blend: false,
                blend_duration: 0.0,
            })
            .add_transition(AnimationTransition {
                from: "heavy_attack".to_string(),
                to: "combat_idle".to_string(),
                condition: TransitionCondition::OnComplete,
                priority: 1,
                blend: false,
                blend_duration: 0.0,
            })
            .add_flag("in_combat".to_string(), true)
            .add_flag("can_attack".to_string(), true)
            .build()
    }
}
