//! UI Transitions
//! 
//! This module provides comprehensive UI transition system for smooth transitions between UI states.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::{UITransitionType, UIResult, UIError, UIEvent};

/// UI transition manager
pub struct UITransitionManager {
    /// Active transitions
    pub active_transitions: HashMap<String, UITransition>,
    /// Transition definitions
    pub transition_definitions: HashMap<String, UITransitionDefinition>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&UIEvent) + Send + Sync>>,
}

/// UI transition
#[derive(Debug, Clone)]
pub struct UITransition {
    /// Transition ID
    pub id: String,
    /// From element ID
    pub from_element: String,
    /// To element ID
    pub to_element: String,
    /// Transition type
    pub transition_type: UITransitionType,
    /// Transition definition
    pub definition: UITransitionDefinition,
    /// Start time
    pub start_time: Instant,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Is completed
    pub completed: bool,
    /// Is paused
    pub paused: bool,
    /// Reverse direction
    pub reverse: bool,
}

/// UI transition definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UITransitionDefinition {
    /// Transition name
    pub name: String,
    /// Transition type
    pub transition_type: UITransitionType,
    /// Duration in seconds
    pub duration: f32,
    /// Easing function
    pub easing: EasingFunction,
    /// Delay in seconds
    pub delay: f32,
    /// Transition properties
    pub properties: TransitionProperties,
}

/// Transition properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionProperties {
    /// Fade properties
    pub fade: Option<FadeTransition>,
    /// Slide properties
    pub slide: Option<SlideTransition>,
    /// Scale properties
    pub scale: Option<ScaleTransition>,
    /// Flip properties
    pub flip: Option<FlipTransition>,
    /// Wipe properties
    pub wipe: Option<WipeTransition>,
    /// Dissolve properties
    pub dissolve: Option<DissolveTransition>,
    /// Custom properties
    pub custom: HashMap<String, CustomTransitionProperty>,
}

/// Fade transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FadeTransition {
    /// Fade out from element
    pub fade_out_from: bool,
    /// Fade in to element
    pub fade_in_to: bool,
    /// Fade duration ratio (0.0 to 1.0)
    pub fade_duration_ratio: f32,
}

/// Slide transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideTransition {
    /// Slide direction
    pub direction: SlideDirection,
    /// Slide distance
    pub distance: f32,
    /// Slide from element
    pub slide_from: bool,
    /// Slide to element
    pub slide_to: bool,
}

/// Scale transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleTransition {
    /// Scale from center
    pub from_center: bool,
    /// Scale from element
    pub scale_from: bool,
    /// Scale to element
    pub scale_to: bool,
    /// Scale factor
    pub scale_factor: f32,
}

/// Flip transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlipTransition {
    /// Flip axis
    pub axis: FlipAxis,
    /// Flip from element
    pub flip_from: bool,
    /// Flip to element
    pub flip_to: bool,
}

/// Wipe transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipeTransition {
    /// Wipe direction
    pub direction: WipeDirection,
    /// Wipe from element
    pub wipe_from: bool,
    /// Wipe to element
    pub wipe_to: bool,
    /// Wipe angle
    pub angle: f32,
}

/// Dissolve transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DissolveTransition {
    /// Dissolve pattern
    pub pattern: DissolvePattern,
    /// Dissolve from element
    pub dissolve_from: bool,
    /// Dissolve to element
    pub dissolve_to: bool,
    /// Pattern size
    pub pattern_size: f32,
}

/// Custom transition property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTransitionProperty {
    /// Property name
    pub property_name: String,
    /// Start value
    pub start_value: f32,
    /// End value
    pub end_value: f32,
    /// Apply to from element
    pub apply_to_from: bool,
    /// Apply to to element
    pub apply_to_to: bool,
}

/// Slide direction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SlideDirection {
    /// Slide left
    Left,
    /// Slide right
    Right,
    /// Slide up
    Up,
    /// Slide down
    Down,
    /// Slide diagonal up-left
    UpLeft,
    /// Slide diagonal up-right
    UpRight,
    /// Slide diagonal down-left
    DownLeft,
    /// Slide diagonal down-right
    DownRight,
}

/// Flip axis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlipAxis {
    /// Flip horizontally
    Horizontal,
    /// Flip vertically
    Vertical,
    /// Flip diagonally
    Diagonal,
}

/// Wipe direction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WipeDirection {
    /// Wipe left to right
    LeftToRight,
    /// Wipe right to left
    RightToLeft,
    /// Wipe top to bottom
    TopToBottom,
    /// Wipe bottom to top
    BottomToTop,
    /// Wipe diagonal
    Diagonal,
    /// Wipe circular
    Circular,
}

/// Dissolve pattern
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DissolvePattern {
    /// Random dissolve
    Random,
    /// Checkerboard dissolve
    Checkerboard,
    /// Spiral dissolve
    Spiral,
    /// Wave dissolve
    Wave,
    /// Custom pattern
    Custom(String),
}

/// Easing function (reused from animations)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EasingFunction {
    /// Linear easing
    Linear,
    /// Ease in
    EaseIn,
    /// Ease out
    EaseOut,
    /// Ease in-out
    EaseInOut,
    /// Bounce in
    BounceIn,
    /// Bounce out
    BounceOut,
    /// Bounce in-out
    BounceInOut,
    /// Elastic in
    ElasticIn,
    /// Elastic out
    ElasticOut,
    /// Elastic in-out
    ElasticInOut,
    /// Back in
    BackIn,
    /// Back out
    BackOut,
    /// Back in-out
    BackInOut,
    /// Custom easing
    Custom(String),
}

impl UITransitionManager {
    /// Create a new UI transition manager
    pub fn new() -> Self {
        Self {
            active_transitions: HashMap::new(),
            transition_definitions: HashMap::new(),
            event_handlers: Vec::new(),
        }
    }

    /// Update transition manager
    pub fn update(&mut self, delta_time: f32) -> UIResult<()> {
        let mut completed_transitions = Vec::new();

        for (id, transition) in self.active_transitions.iter_mut() {
            if transition.paused {
                continue;
            }

            // Update progress
            let elapsed = transition.start_time.elapsed().as_secs_f32();
            let total_duration = transition.definition.duration + transition.definition.delay;
            
            if elapsed < transition.definition.delay {
                continue;
            }

            let transition_elapsed = elapsed - transition.definition.delay;
            transition.progress = (transition_elapsed / transition.definition.duration).min(1.0);

            // Apply easing
            let eased_progress = self.apply_easing(transition.progress, &transition.definition.easing);

            // Apply transition properties
            self.apply_transition_properties(transition, eased_progress)?;

            // Check if transition is completed
            if transition.progress >= 1.0 {
                transition.completed = true;
                completed_transitions.push(id.clone());
            }
        }

        // Remove completed transitions and emit events
        for id in completed_transitions {
            if let Some(transition) = self.active_transitions.remove(&id) {
                self.emit_event(UIEvent::TransitionCompleted {
                    from_element: transition.from_element,
                    to_element: transition.to_element,
                    transition_type: transition.transition_type,
                });
            }
        }

        Ok(())
    }

    /// Start transition
    pub fn start_transition(&mut self, from_element: &str, to_element: &str, transition_name: &str) -> UIResult<()> {
        let definition = self.transition_definitions.get(transition_name)
            .ok_or_else(|| UIError::TransitionNotFound(transition_name.to_string()))?;

        // Check if transition is already running
        let transition_id = format!("{}_{}_{}", from_element, to_element, transition_name);
        if self.active_transitions.contains_key(&transition_id) {
            return Err(UIError::TransitionAlreadyRunning(transition_id));
        }

        // Create transition
        let transition = UITransition {
            id: transition_id.clone(),
            from_element: from_element.to_string(),
            to_element: to_element.to_string(),
            transition_type: definition.transition_type.clone(),
            definition: definition.clone(),
            start_time: Instant::now(),
            progress: 0.0,
            completed: false,
            paused: false,
            reverse: false,
        };

        self.active_transitions.insert(transition_id, transition);

        // Emit start event
        self.emit_event(UIEvent::TransitionStarted {
            from_element: from_element.to_string(),
            to_element: to_element.to_string(),
            transition_type: definition.transition_type.clone(),
        });

        Ok(())
    }

    /// Stop transition
    pub fn stop_transition(&mut self, from_element: &str, to_element: &str, transition_name: &str) -> UIResult<()> {
        let transition_id = format!("{}_{}_{}", from_element, to_element, transition_name);
        self.active_transitions.remove(&transition_id);
        Ok(())
    }

    /// Pause transition
    pub fn pause_transition(&mut self, from_element: &str, to_element: &str, transition_name: &str) -> UIResult<()> {
        let transition_id = format!("{}_{}_{}", from_element, to_element, transition_name);
        if let Some(transition) = self.active_transitions.get_mut(&transition_id) {
            transition.paused = true;
        }
        Ok(())
    }

    /// Resume transition
    pub fn resume_transition(&mut self, from_element: &str, to_element: &str, transition_name: &str) -> UIResult<()> {
        let transition_id = format!("{}_{}_{}", from_element, to_element, transition_name);
        if let Some(transition) = self.active_transitions.get_mut(&transition_id) {
            transition.paused = false;
        }
        Ok(())
    }

    /// Add transition definition
    pub fn add_transition_definition(&mut self, definition: UITransitionDefinition) {
        self.transition_definitions.insert(definition.name.clone(), definition);
    }

    /// Remove transition definition
    pub fn remove_transition_definition(&mut self, name: &str) {
        self.transition_definitions.remove(name);
    }

    /// Get active transitions for element
    pub fn get_element_transitions(&self, element_id: &str) -> Vec<&UITransition> {
        self.active_transitions.values()
            .filter(|trans| trans.from_element == element_id || trans.to_element == element_id)
            .collect()
    }

    /// Check if element has active transitions
    pub fn has_active_transitions(&self, element_id: &str) -> bool {
        self.active_transitions.values()
            .any(|trans| (trans.from_element == element_id || trans.to_element == element_id) && !trans.completed)
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&UIEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Apply easing function (same as animations)
    fn apply_easing(&self, t: f32, easing: &EasingFunction) -> f32 {
        match easing {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            },
            EasingFunction::BounceIn => 1.0 - self.bounce_out(1.0 - t),
            EasingFunction::BounceOut => self.bounce_out(t),
            EasingFunction::BounceInOut => {
                if t < 0.5 {
                    0.5 * (1.0 - self.bounce_out(1.0 - 2.0 * t))
                } else {
                    0.5 * self.bounce_out(2.0 * t - 1.0) + 0.5
                }
            },
            EasingFunction::ElasticIn => self.elastic_in(t),
            EasingFunction::ElasticOut => self.elastic_out(t),
            EasingFunction::ElasticInOut => {
                if t < 0.5 {
                    0.5 * self.elastic_in(2.0 * t)
                } else {
                    0.5 * self.elastic_out(2.0 * t - 1.0) + 0.5
                }
            },
            EasingFunction::BackIn => self.back_in(t),
            EasingFunction::BackOut => self.back_out(t),
            EasingFunction::BackInOut => {
                if t < 0.5 {
                    0.5 * self.back_in(2.0 * t)
                } else {
                    0.5 * self.back_out(2.0 * t - 1.0) + 0.5
                }
            },
            EasingFunction::Custom(_) => t,
        }
    }

    /// Apply transition properties
    fn apply_transition_properties(&self, transition: &UITransition, progress: f32) -> UIResult<()> {
        // Fade transition
        if let Some(fade) = &transition.definition.properties.fade {
            if fade.fade_out_from {
                let alpha = 1.0 - progress;
                // Apply alpha to from element
            }
            if fade.fade_in_to {
                let alpha = progress;
                // Apply alpha to to element
            }
        }

        // Slide transition
        if let Some(slide) = &transition.definition.properties.slide {
            let distance = slide.distance * progress;
            // Apply slide transformation based on direction
        }

        // Scale transition
        if let Some(scale) = &transition.definition.properties.scale {
            let scale_factor = 1.0 + (scale.scale_factor - 1.0) * progress;
            // Apply scale transformation
        }

        // Flip transition
        if let Some(flip) = &transition.definition.properties.flip {
            let rotation = 180.0 * progress;
            // Apply flip transformation based on axis
        }

        // Wipe transition
        if let Some(wipe) = &transition.definition.properties.wipe {
            let wipe_progress = progress;
            // Apply wipe effect based on direction
        }

        // Dissolve transition
        if let Some(dissolve) = &transition.definition.properties.dissolve {
            let dissolve_progress = progress;
            // Apply dissolve effect based on pattern
        }

        Ok(())
    }

    /// Bounce out easing (same as animations)
    fn bounce_out(&self, t: f32) -> f32 {
        if t < 1.0 / 2.75 {
            7.5625 * t * t
        } else if t < 2.0 / 2.75 {
            let t = t - 1.5 / 2.75;
            7.5625 * t * t + 0.75
        } else if t < 2.5 / 2.75 {
            let t = t - 2.25 / 2.75;
            7.5625 * t * t + 0.9375
        } else {
            let t = t - 2.625 / 2.75;
            7.5625 * t * t + 0.984375
        }
    }

    /// Elastic in easing (same as animations)
    fn elastic_in(&self, t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            t
        } else {
            let c4 = (2.0 * std::f32::consts::PI) / 3.0;
            -2.0_f32.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin()
        }
    }

    /// Elastic out easing (same as animations)
    fn elastic_out(&self, t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            t
        } else {
            let c4 = (2.0 * std::f32::consts::PI) / 3.0;
            2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
        }
    }

    /// Back in easing (same as animations)
    fn back_in(&self, t: f32) -> f32 {
        let c1 = 1.70158;
        let c3 = c1 + 1.0;
        c3 * t * t * t - c1 * t * t
    }

    /// Back out easing (same as animations)
    fn back_out(&self, t: f32) -> f32 {
        let c1 = 1.70158;
        let c3 = c1 + 1.0;
        1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
    }

    /// Emit UI event
    fn emit_event(&self, event: UIEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for UITransitionManager {
    fn default() -> Self {
        Self::new()
    }
}
