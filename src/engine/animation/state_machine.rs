//! Animation state machine for managing animation transitions

use crate::engine::animation::{Animation, AnimationTransition, TransitionCondition, AnimationBlend};
use std::collections::HashMap;

/// Animation state machine for managing complex animation logic
#[derive(Debug)]
pub struct AnimationStateMachine {
    /// Current state
    pub current_state: String,
    /// Available states
    pub states: HashMap<String, AnimationState>,
    /// State transitions
    pub transitions: Vec<AnimationTransition>,
    /// Current blend state
    pub current_blend: Option<AnimationBlend>,
    /// State machine parameters
    pub parameters: HashMap<String, f32>,
    /// State machine flags
    pub flags: HashMap<String, bool>,
}

/// Individual animation state
#[derive(Debug, Clone)]
pub struct AnimationState {
    /// State name
    pub name: String,
    /// Default animation for this state
    pub default_animation: String,
    /// State entry animation
    pub entry_animation: Option<String>,
    /// State exit animation
    pub exit_animation: Option<String>,
    /// State-specific parameters
    pub parameters: HashMap<String, f32>,
    /// State-specific flags
    pub flags: HashMap<String, bool>,
}

impl AnimationStateMachine {
    /// Create a new animation state machine
    pub fn new(initial_state: String) -> Self {
        Self {
            current_state: initial_state,
            states: HashMap::new(),
            transitions: Vec::new(),
            current_blend: None,
            parameters: HashMap::new(),
            flags: HashMap::new(),
        }
    }

    /// Add a state to the state machine
    pub fn add_state(&mut self, state: AnimationState) {
        self.states.insert(state.name.clone(), state);
    }

    /// Add a transition between states
    pub fn add_transition(&mut self, transition: AnimationTransition) {
        self.transitions.push(transition);
    }

    /// Set a parameter value
    pub fn set_parameter(&mut self, name: String, value: f32) {
        self.parameters.insert(name, value);
    }

    /// Get a parameter value
    pub fn get_parameter(&self, name: &str) -> Option<f32> {
        self.parameters.get(name).copied()
    }

    /// Set a flag value
    pub fn set_flag(&mut self, name: String, value: bool) {
        self.flags.insert(name, value);
    }

    /// Get a flag value
    pub fn get_flag(&self, name: &str) -> Option<bool> {
        self.flags.get(name).copied()
    }

    /// Update the state machine
    pub fn update(&mut self, dt: f32) -> Option<String> {
        // Update current blend if active
        if let Some(blend) = &mut self.current_blend {
            if blend.update(dt) {
                // Blend complete, transition to new state
                let new_state = blend.to_animation.clone();
                self.current_blend = None;
                self.current_state = new_state.clone();
                return Some(new_state);
            }
        }

        // Check for state transitions
        for transition in &self.transitions {
            if transition.from == self.current_state {
                if self.check_transition_condition(&transition.condition) {
                    // Transition condition met
                    if transition.blend {
                        // Start blend transition
                        self.current_blend = Some(AnimationBlend::new(
                            transition.from.clone(),
                            transition.to.clone(),
                            transition.blend_duration,
                        ));
                    } else {
                        // Immediate transition
                        self.current_state = transition.to.clone();
                        return Some(transition.to.clone());
                    }
                }
            }
        }

        None
    }

    /// Check if a transition condition is met
    fn check_transition_condition(&self, condition: &TransitionCondition) -> bool {
        match condition {
            TransitionCondition::OnComplete => {
                // This would be set by the animation system when an animation completes
                self.get_flag("animation_complete").unwrap_or(false)
            }
            TransitionCondition::OnInput { input } => {
                // This would be checked against input system
                self.get_flag(&format!("input_{}", input)).unwrap_or(false)
            }
            TransitionCondition::OnState { state } => {
                // Check if we're in the specified state
                self.current_state == *state
            }
            TransitionCondition::OnCondition { condition } => {
                // Check custom condition
                self.evaluate_condition(condition)
            }
            TransitionCondition::Immediate => true,
        }
    }

    /// Evaluate a custom condition string
    fn evaluate_condition(&self, condition: &str) -> bool {
        // Simple condition evaluation - in a real implementation,
        // this would be more sophisticated
        if condition.starts_with("parameter_") {
            let param_name = &condition[10..];
            if let Some(value) = self.get_parameter(param_name) {
                return value > 0.0;
            }
        } else if condition.starts_with("flag_") {
            let flag_name = &condition[5..];
            return self.get_flag(flag_name).unwrap_or(false);
        }
        
        false
    }

    /// Get current animation name
    pub fn get_current_animation(&self) -> Option<String> {
        if let Some(blend) = &self.current_blend {
            // During blend, return target animation
            Some(blend.to_animation.clone())
        } else if let Some(state) = self.states.get(&self.current_state) {
            Some(state.default_animation.clone())
        } else {
            None
        }
    }

    /// Get blend weights if currently blending
    pub fn get_blend_weights(&self) -> Option<(f32, f32)> {
        if let Some(blend) = &self.current_blend {
            Some((blend.get_from_weight(), blend.get_to_weight()))
        } else {
            None
        }
    }

    /// Force transition to a state
    pub fn force_transition(&mut self, new_state: String) {
        self.current_state = new_state;
        self.current_blend = None;
    }

    /// Check if currently blending
    pub fn is_blending(&self) -> bool {
        self.current_blend.is_some()
    }

    /// Get current state
    pub fn get_current_state(&self) -> &String {
        &self.current_state
    }

    /// Get state by name
    pub fn get_state(&self, name: &str) -> Option<&AnimationState> {
        self.states.get(name)
    }

    /// Get all available states
    pub fn get_states(&self) -> &HashMap<String, AnimationState> {
        &self.states
    }

    /// Get all transitions
    pub fn get_transitions(&self) -> &Vec<AnimationTransition> {
        &self.transitions
    }
}

impl AnimationState {
    /// Create a new animation state
    pub fn new(name: String, default_animation: String) -> Self {
        Self {
            name,
            default_animation,
            entry_animation: None,
            exit_animation: None,
            parameters: HashMap::new(),
            flags: HashMap::new(),
        }
    }

    /// Set entry animation
    pub fn set_entry_animation(&mut self, animation: String) -> &mut Self {
        self.entry_animation = Some(animation);
        self
    }

    /// Set exit animation
    pub fn set_exit_animation(&mut self, animation: String) -> &mut Self {
        self.exit_animation = Some(animation);
        self
    }

    /// Set a parameter
    pub fn set_parameter(&mut self, name: String, value: f32) -> &mut Self {
        self.parameters.insert(name, value);
        self
    }

    /// Get a parameter
    pub fn get_parameter(&self, name: &str) -> Option<f32> {
        self.parameters.get(name).copied()
    }

    /// Set a flag
    pub fn set_flag(&mut self, name: String, value: bool) -> &mut Self {
        self.flags.insert(name, value);
        self
    }

    /// Get a flag
    pub fn get_flag(&self, name: &str) -> Option<bool> {
        self.flags.get(name).copied()
    }
}
