//! UI Animations
//! 
//! This module provides comprehensive UI animation system with various animation types and easing functions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::{UIAnimationType, UIResult, UIError, UIEvent};

/// UI animation manager
pub struct UIAnimationManager {
    /// Active animations
    pub active_animations: HashMap<String, UIAnimation>,
    /// Animation definitions
    pub animation_definitions: HashMap<String, UIAnimationDefinition>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&UIEvent) + Send + Sync>>,
}

/// UI animation
#[derive(Debug, Clone)]
pub struct UIAnimation {
    /// Animation ID
    pub id: String,
    /// Element ID
    pub element_id: String,
    /// Animation type
    pub animation_type: UIAnimationType,
    /// Animation definition
    pub definition: UIAnimationDefinition,
    /// Start time
    pub start_time: Instant,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Is completed
    pub completed: bool,
    /// Is paused
    pub paused: bool,
    /// Loop count (0 = infinite)
    pub loop_count: u32,
    /// Current loop
    pub current_loop: u32,
    /// Reverse direction
    pub reverse: bool,
}

/// UI animation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIAnimationDefinition {
    /// Animation name
    pub name: String,
    /// Animation type
    pub animation_type: UIAnimationType,
    /// Duration in seconds
    pub duration: f32,
    /// Easing function
    pub easing: EasingFunction,
    /// Delay in seconds
    pub delay: f32,
    /// Loop count (0 = infinite)
    pub loop_count: u32,
    /// Auto reverse
    pub auto_reverse: bool,
    /// Animation properties
    pub properties: AnimationProperties,
}

/// Animation properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationProperties {
    /// Position animation
    pub position: Option<PositionAnimation>,
    /// Scale animation
    pub scale: Option<ScaleAnimation>,
    /// Rotation animation
    pub rotation: Option<RotationAnimation>,
    /// Color animation
    pub color: Option<ColorAnimation>,
    /// Alpha animation
    pub alpha: Option<AlphaAnimation>,
    /// Size animation
    pub size: Option<SizeAnimation>,
    /// Custom properties
    pub custom: HashMap<String, CustomAnimationProperty>,
}

/// Position animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionAnimation {
    /// Start position
    pub start: (f32, f32),
    /// End position
    pub end: (f32, f32),
    /// Relative animation
    pub relative: bool,
}

/// Scale animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleAnimation {
    /// Start scale
    pub start: (f32, f32),
    /// End scale
    pub end: (f32, f32),
    /// Scale from center
    pub from_center: bool,
}

/// Rotation animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationAnimation {
    /// Start rotation in degrees
    pub start: f32,
    /// End rotation in degrees
    pub end: f32,
    /// Rotate from center
    pub from_center: bool,
}

/// Color animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorAnimation {
    /// Start color (RGBA)
    pub start: (f32, f32, f32, f32),
    /// End color (RGBA)
    pub end: (f32, f32, f32, f32),
}

/// Alpha animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlphaAnimation {
    /// Start alpha
    pub start: f32,
    /// End alpha
    pub end: f32,
}

/// Size animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeAnimation {
    /// Start size
    pub start: (f32, f32),
    /// End size
    pub end: (f32, f32),
    /// Size from center
    pub from_center: bool,
}

/// Custom animation property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAnimationProperty {
    /// Start value
    pub start: f32,
    /// End value
    pub end: f32,
    /// Property name
    pub property_name: String,
}

/// Easing function
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

impl UIAnimationManager {
    /// Create a new UI animation manager
    pub fn new() -> Self {
        Self {
            active_animations: HashMap::new(),
            animation_definitions: HashMap::new(),
            event_handlers: Vec::new(),
        }
    }

    /// Update animation manager
    pub fn update(&mut self, delta_time: f32) -> UIResult<()> {
        let mut completed_animations = Vec::new();

        for (id, animation) in self.active_animations.iter_mut() {
            if animation.paused {
                continue;
            }

            // Update progress
            let elapsed = animation.start_time.elapsed().as_secs_f32();
            let total_duration = animation.definition.duration + animation.definition.delay;
            
            if elapsed < animation.definition.delay {
                continue;
            }

            let animation_elapsed = elapsed - animation.definition.delay;
            animation.progress = (animation_elapsed / animation.definition.duration).min(1.0);

            // Apply easing
            let eased_progress = self.apply_easing(animation.progress, &animation.definition.easing);

            // Apply animation properties
            self.apply_animation_properties(animation, eased_progress)?;

            // Check if animation is completed
            if animation.progress >= 1.0 {
                if animation.loop_count == 0 || animation.current_loop < animation.loop_count - 1 {
                    // Continue looping
                    animation.current_loop += 1;
                    animation.progress = 0.0;
                    animation.start_time = Instant::now();
                    
                    if animation.definition.auto_reverse {
                        animation.reverse = !animation.reverse;
                    }
                } else {
                    // Animation completed
                    animation.completed = true;
                    completed_animations.push(id.clone());
                }
            }
        }

        // Remove completed animations and emit events
        for id in completed_animations {
            if let Some(animation) = self.active_animations.remove(&id) {
                self.emit_event(UIEvent::AnimationCompleted {
                    element_id: animation.element_id,
                    animation_type: animation.animation_type,
                });
            }
        }

        Ok(())
    }

    /// Start animation
    pub fn start_animation(&mut self, element_id: &str, animation_name: &str) -> UIResult<()> {
        let definition = self.animation_definitions.get(animation_name)
            .ok_or_else(|| UIError::AnimationNotFound(animation_name.to_string()))?;

        // Check if animation is already running
        let animation_id = format!("{}_{}", element_id, animation_name);
        if self.active_animations.contains_key(&animation_id) {
            return Err(UIError::AnimationAlreadyRunning(animation_id));
        }

        // Create animation
        let animation = UIAnimation {
            id: animation_id.clone(),
            element_id: element_id.to_string(),
            animation_type: definition.animation_type.clone(),
            definition: definition.clone(),
            start_time: Instant::now(),
            progress: 0.0,
            completed: false,
            paused: false,
            loop_count: definition.loop_count,
            current_loop: 0,
            reverse: false,
        };

        self.active_animations.insert(animation_id, animation);

        // Emit start event
        self.emit_event(UIEvent::AnimationStarted {
            element_id: element_id.to_string(),
            animation_type: definition.animation_type.clone(),
        });

        Ok(())
    }

    /// Stop animation
    pub fn stop_animation(&mut self, element_id: &str, animation_name: &str) -> UIResult<()> {
        let animation_id = format!("{}_{}", element_id, animation_name);
        self.active_animations.remove(&animation_id);
        Ok(())
    }

    /// Pause animation
    pub fn pause_animation(&mut self, element_id: &str, animation_name: &str) -> UIResult<()> {
        let animation_id = format!("{}_{}", element_id, animation_name);
        if let Some(animation) = self.active_animations.get_mut(&animation_id) {
            animation.paused = true;
        }
        Ok(())
    }

    /// Resume animation
    pub fn resume_animation(&mut self, element_id: &str, animation_name: &str) -> UIResult<()> {
        let animation_id = format!("{}_{}", element_id, animation_name);
        if let Some(animation) = self.active_animations.get_mut(&animation_id) {
            animation.paused = false;
        }
        Ok(())
    }

    /// Add animation definition
    pub fn add_animation_definition(&mut self, definition: UIAnimationDefinition) {
        self.animation_definitions.insert(definition.name.clone(), definition);
    }

    /// Remove animation definition
    pub fn remove_animation_definition(&mut self, name: &str) {
        self.animation_definitions.remove(name);
    }

    /// Get active animations for element
    pub fn get_element_animations(&self, element_id: &str) -> Vec<&UIAnimation> {
        self.active_animations.values()
            .filter(|anim| anim.element_id == element_id)
            .collect()
    }

    /// Check if element has active animations
    pub fn has_active_animations(&self, element_id: &str) -> bool {
        self.active_animations.values()
            .any(|anim| anim.element_id == element_id && !anim.completed)
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&UIEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Apply easing function
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
            EasingFunction::Custom(_) => t, // Custom easing would be implemented here
        }
    }

    /// Bounce out easing
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

    /// Elastic in easing
    fn elastic_in(&self, t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            t
        } else {
            let c4 = (2.0 * std::f32::consts::PI) / 3.0;
            -2.0_f32.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin()
        }
    }

    /// Elastic out easing
    fn elastic_out(&self, t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            t
        } else {
            let c4 = (2.0 * std::f32::consts::PI) / 3.0;
            2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
        }
    }

    /// Back in easing
    fn back_in(&self, t: f32) -> f32 {
        let c1 = 1.70158;
        let c3 = c1 + 1.0;
        c3 * t * t * t - c1 * t * t
    }

    /// Back out easing
    fn back_out(&self, t: f32) -> f32 {
        let c1 = 1.70158;
        let c3 = c1 + 1.0;
        1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
    }

    /// Apply animation properties
    fn apply_animation_properties(&self, animation: &UIAnimation, progress: f32) -> UIResult<()> {
        // In a real implementation, this would apply the animation properties to the UI element
        // For now, we'll just track the progress
        
        // Position animation
        if let Some(pos_anim) = &animation.definition.properties.position {
            let (start_x, start_y) = pos_anim.start;
            let (end_x, end_y) = pos_anim.end;
            let current_x = start_x + (end_x - start_x) * progress;
            let current_y = start_y + (end_y - start_y) * progress;
            // Apply position to element
        }

        // Scale animation
        if let Some(scale_anim) = &animation.definition.properties.scale {
            let (start_x, start_y) = scale_anim.start;
            let (end_x, end_y) = scale_anim.end;
            let current_x = start_x + (end_x - start_x) * progress;
            let current_y = start_y + (end_y - start_y) * progress;
            // Apply scale to element
        }

        // Rotation animation
        if let Some(rot_anim) = &animation.definition.properties.rotation {
            let start_rot = rot_anim.start;
            let end_rot = rot_anim.end;
            let current_rot = start_rot + (end_rot - start_rot) * progress;
            // Apply rotation to element
        }

        // Color animation
        if let Some(color_anim) = &animation.definition.properties.color {
            let (start_r, start_g, start_b, start_a) = color_anim.start;
            let (end_r, end_g, end_b, end_a) = color_anim.end;
            let current_r = start_r + (end_r - start_r) * progress;
            let current_g = start_g + (end_g - start_g) * progress;
            let current_b = start_b + (end_b - start_b) * progress;
            let current_a = start_a + (end_a - start_a) * progress;
            // Apply color to element
        }

        // Alpha animation
        if let Some(alpha_anim) = &animation.definition.properties.alpha {
            let start_alpha = alpha_anim.start;
            let end_alpha = alpha_anim.end;
            let current_alpha = start_alpha + (end_alpha - start_alpha) * progress;
            // Apply alpha to element
        }

        // Size animation
        if let Some(size_anim) = &animation.definition.properties.size {
            let (start_w, start_h) = size_anim.start;
            let (end_w, end_h) = size_anim.end;
            let current_w = start_w + (end_w - start_w) * progress;
            let current_h = start_h + (end_h - start_h) * progress;
            // Apply size to element
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

impl Default for UIAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}
