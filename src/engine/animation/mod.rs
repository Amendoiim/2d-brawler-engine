//! Animation system for 2D sprite animations

pub mod animation;
pub mod state_machine;
pub mod renderer;
pub mod controller;

pub use animation::{Animation, AnimationFrame, AnimationState as AnimationStateData, AnimationTransition, TransitionCondition, AnimationBlend, FrameEffect};
pub use state_machine::{AnimationStateMachine, AnimationState as StateMachineState};
pub use renderer::*;
pub use controller::*;

use crate::engine::ecs::{Component, Entity, World};
use std::collections::HashMap;

/// Animation system for managing sprite animations
pub struct AnimationSystem {
    pub animations: HashMap<String, Animation>,
    pub states: HashMap<Entity, AnimationStateData>,
    pub controllers: HashMap<Entity, AnimationController>,
}

impl AnimationSystem {
    /// Create a new animation system
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            states: HashMap::new(),
            controllers: HashMap::new(),
        }
    }

    /// Register an animation
    pub fn register_animation(&mut self, name: String, mut animation: Animation) {
        self.animations.insert(name, animation);
    }

    /// Add animation controller to an entity
    pub fn add_controller(&mut self, entity: Entity, mut controller: AnimationController) {
        // Get initial animation before moving controller
        let initial_animation = controller.get_initial_animation();
        
        self.controllers.insert(entity, controller);
        
        // Initialize animation state
        if let Some(animation_name) = initial_animation {
            self.states.insert(entity, AnimationStateData {
                current_animation: animation_name,
                current_frame: 0,
                elapsed_time: 0.0,
                is_playing: true,
                speed_multiplier: 1.0,
            });
        }
    }

    /// Update animation system
    pub fn update(&mut self, world: &mut World, dt: f32) {
        // Update all animation states
        for (entity, state) in self.states.iter_mut() {
            if !state.is_playing {
                continue;
            }

            // Get animation data
            if let Some(animation) = self.animations.get(&state.current_animation) {
                // Update elapsed time
                state.elapsed_time += dt * state.speed_multiplier;

                // Check if we need to advance to next frame
                let frame_duration = animation.duration / animation.frames.len() as f32;
                if state.elapsed_time >= frame_duration {
                    state.elapsed_time = 0.0;
                    state.current_frame += 1;

                    // Check if animation is complete
                    if state.current_frame >= animation.frames.len() {
                        if animation.looping {
                            state.current_frame = 0;
                        } else {
                            state.is_playing = false;
                            // Store animation name to avoid borrowing issues
                            let animation_name = state.current_animation.clone();
                            // Trigger animation complete event after the loop
                            // We'll handle this separately to avoid borrowing conflicts
                        }
                    }
                }
            }
        }

        // Update animation controllers
        for (entity, controller) in self.controllers.iter_mut() {
            controller.update(dt);
            
            // Check for state transitions
            if let Some(new_animation) = controller.get_pending_transition() {
                if let Some(state) = self.states.get_mut(entity) {
                    state.current_animation = new_animation.clone();
                    state.current_frame = 0;
                    state.elapsed_time = 0.0;
                    state.is_playing = true;
                }
                controller.clear_pending_transition();
            }
        }
    }

    /// Handle animation completion
    fn handle_animation_complete(&mut self, entity: Entity, animation_name: &str) {
        // Notify controller of animation completion
        if let Some(controller) = self.controllers.get_mut(&entity) {
            controller.on_animation_complete(animation_name);
        }
    }

    /// Get current animation frame for an entity
    pub fn get_current_frame(&self, entity: Entity) -> Option<&AnimationFrame> {
        if let Some(state) = self.states.get(&entity) {
            if let Some(animation) = self.animations.get(&state.current_animation) {
                return animation.frames.get(state.current_frame);
            }
        }
        None
    }

    /// Play animation on entity
    pub fn play_animation(&mut self, entity: Entity, animation_name: &str) {
        if let Some(state) = self.states.get_mut(&entity) {
            if self.animations.contains_key(animation_name) {
                state.current_animation = animation_name.to_string();
                state.current_frame = 0;
                state.elapsed_time = 0.0;
                state.is_playing = true;
            }
        }
    }

    /// Set animation speed multiplier
    pub fn set_speed_multiplier(&mut self, entity: Entity, multiplier: f32) {
        if let Some(state) = self.states.get_mut(&entity) {
            state.speed_multiplier = multiplier;
        }
    }

    /// Pause/resume animation
    pub fn set_playing(&mut self, entity: Entity, playing: bool) {
        if let Some(state) = self.states.get_mut(&entity) {
            state.is_playing = playing;
        }
    }

    /// Get animation progress (0.0 to 1.0)
    pub fn get_animation_progress(&self, entity: Entity) -> Option<f32> {
        if let Some(state) = self.states.get(&entity) {
            if let Some(animation) = self.animations.get(&state.current_animation) {
                let total_duration = animation.duration;
                let current_time = state.current_frame as f32 * (total_duration / animation.frames.len() as f32) + state.elapsed_time;
                return Some((current_time / total_duration).min(1.0));
            }
        }
        None
    }
}
