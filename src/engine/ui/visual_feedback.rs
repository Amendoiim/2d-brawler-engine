//! UI Visual Feedback
//! 
//! This module provides comprehensive visual feedback system for UI interactions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::{UIVisualFeedbackType, UIResult, UIError, UIEvent};

/// UI visual feedback manager
pub struct UIVisualFeedbackManager {
    /// Active feedback effects
    pub active_feedback: HashMap<String, UIVisualFeedback>,
    /// Feedback definitions
    pub feedback_definitions: HashMap<String, UIVisualFeedbackDefinition>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&UIEvent) + Send + Sync>>,
}

/// UI visual feedback
#[derive(Debug, Clone)]
pub struct UIVisualFeedback {
    /// Feedback ID
    pub id: String,
    /// Element ID
    pub element_id: String,
    /// Feedback type
    pub feedback_type: UIVisualFeedbackType,
    /// Feedback definition
    pub definition: UIVisualFeedbackDefinition,
    /// Start time
    pub start_time: Instant,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Is completed
    pub completed: bool,
    /// Is paused
    pub paused: bool,
    /// Intensity multiplier
    pub intensity: f32,
}

/// UI visual feedback definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIVisualFeedbackDefinition {
    /// Feedback name
    pub name: String,
    /// Feedback type
    pub feedback_type: UIVisualFeedbackType,
    /// Duration in seconds
    pub duration: f32,
    /// Easing function
    pub easing: EasingFunction,
    /// Delay in seconds
    pub delay: f32,
    /// Feedback properties
    pub properties: FeedbackProperties,
}

/// Feedback properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackProperties {
    /// Color feedback
    pub color: Option<ColorFeedback>,
    /// Glow feedback
    pub glow: Option<GlowFeedback>,
    /// Shake feedback
    pub shake: Option<ShakeFeedback>,
    /// Pulse feedback
    pub pulse: Option<PulseFeedback>,
    /// Scale feedback
    pub scale: Option<ScaleFeedback>,
    /// Border feedback
    pub border: Option<BorderFeedback>,
    /// Particle feedback
    pub particle: Option<ParticleFeedback>,
    /// Custom properties
    pub custom: HashMap<String, CustomFeedbackProperty>,
}

/// Color feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorFeedback {
    /// Start color (RGBA)
    pub start_color: (f32, f32, f32, f32),
    /// End color (RGBA)
    pub end_color: (f32, f32, f32, f32),
    /// Apply to background
    pub apply_to_background: bool,
    /// Apply to text
    pub apply_to_text: bool,
    /// Apply to border
    pub apply_to_border: bool,
}

/// Glow feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlowFeedback {
    /// Glow color (RGBA)
    pub glow_color: (f32, f32, f32, f32),
    /// Glow intensity
    pub intensity: f32,
    /// Glow radius
    pub radius: f32,
    /// Glow spread
    pub spread: f32,
    /// Glow quality
    pub quality: GlowQuality,
}

/// Shake feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShakeFeedback {
    /// Shake intensity
    pub intensity: f32,
    /// Shake frequency
    pub frequency: f32,
    /// Shake direction
    pub direction: ShakeDirection,
    /// Shake decay
    pub decay: f32,
}

/// Pulse feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulseFeedback {
    /// Pulse scale factor
    pub scale_factor: f32,
    /// Pulse frequency
    pub frequency: f32,
    /// Pulse intensity
    pub intensity: f32,
    /// Pulse from center
    pub from_center: bool,
}

/// Scale feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleFeedback {
    /// Scale factor
    pub scale_factor: f32,
    /// Scale from center
    pub from_center: bool,
    /// Scale duration
    pub scale_duration: f32,
    /// Return to original scale
    pub return_to_original: bool,
}

/// Border feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderFeedback {
    /// Border color (RGBA)
    pub border_color: (f32, f32, f32, f32),
    /// Border width
    pub border_width: f32,
    /// Border style
    pub border_style: BorderStyle,
    /// Border radius
    pub border_radius: f32,
}

/// Particle feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleFeedback {
    /// Particle count
    pub particle_count: u32,
    /// Particle lifetime
    pub particle_lifetime: f32,
    /// Particle velocity
    pub particle_velocity: f32,
    /// Particle color (RGBA)
    pub particle_color: (f32, f32, f32, f32),
    /// Particle size
    pub particle_size: f32,
    /// Particle direction
    pub particle_direction: ParticleDirection,
}

/// Custom feedback property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFeedbackProperty {
    /// Property name
    pub property_name: String,
    /// Start value
    pub start_value: f32,
    /// End value
    pub end_value: f32,
    /// Property type
    pub property_type: CustomPropertyType,
}

/// Glow quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GlowQuality {
    /// Low quality glow
    Low,
    /// Medium quality glow
    Medium,
    /// High quality glow
    High,
    /// Ultra quality glow
    Ultra,
}

/// Shake direction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShakeDirection {
    /// Horizontal shake
    Horizontal,
    /// Vertical shake
    Vertical,
    /// Diagonal shake
    Diagonal,
    /// Random shake
    Random,
}

/// Border style
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BorderStyle {
    /// Solid border
    Solid,
    /// Dashed border
    Dashed,
    /// Dotted border
    Dotted,
    /// Double border
    Double,
    /// Groove border
    Groove,
    /// Ridge border
    Ridge,
    /// Inset border
    Inset,
    /// Outset border
    Outset,
}

/// Particle direction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParticleDirection {
    /// Upward particles
    Up,
    /// Downward particles
    Down,
    /// Leftward particles
    Left,
    /// Rightward particles
    Right,
    /// Outward particles
    Outward,
    /// Inward particles
    Inward,
    /// Random particles
    Random,
}

/// Custom property type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CustomPropertyType {
    /// Float property
    Float,
    /// Color property
    Color,
    /// Vector property
    Vector,
    /// Boolean property
    Boolean,
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

impl UIVisualFeedbackManager {
    /// Create a new UI visual feedback manager
    pub fn new() -> Self {
        Self {
            active_feedback: HashMap::new(),
            feedback_definitions: HashMap::new(),
            event_handlers: Vec::new(),
        }
    }

    /// Update visual feedback manager
    pub fn update(&mut self, delta_time: f32) -> UIResult<()> {
        let mut completed_feedback = Vec::new();

        for (id, feedback) in self.active_feedback.iter_mut() {
            if feedback.paused {
                continue;
            }

            // Update progress
            let elapsed = feedback.start_time.elapsed().as_secs_f32();
            let total_duration = feedback.definition.duration + feedback.definition.delay;
            
            if elapsed < feedback.definition.delay {
                continue;
            }

            let feedback_elapsed = elapsed - feedback.definition.delay;
            feedback.progress = (feedback_elapsed / feedback.definition.duration).min(1.0);

            // Apply easing
            let eased_progress = self.apply_easing(feedback.progress, &feedback.definition.easing);

            // Apply feedback properties
            self.apply_feedback_properties(feedback, eased_progress)?;

            // Check if feedback is completed
            if feedback.progress >= 1.0 {
                feedback.completed = true;
                completed_feedback.push(id.clone());
            }
        }

        // Remove completed feedback and emit events
        for id in completed_feedback {
            if let Some(feedback) = self.active_feedback.remove(&id) {
                self.emit_event(UIEvent::VisualFeedbackTriggered {
                    element_id: feedback.element_id,
                    feedback_type: feedback.feedback_type,
                });
            }
        }

        Ok(())
    }

    /// Trigger visual feedback
    pub fn trigger_feedback(&mut self, element_id: &str, feedback_name: &str, intensity: f32) -> UIResult<()> {
        let definition = self.feedback_definitions.get(feedback_name)
            .ok_or_else(|| UIError::Unknown(format!("Feedback definition not found: {}", feedback_name)))?;

        // Create feedback
        let feedback_id = format!("{}_{}_{}", element_id, feedback_name, Instant::now().elapsed().as_millis());
        let feedback = UIVisualFeedback {
            id: feedback_id.clone(),
            element_id: element_id.to_string(),
            feedback_type: definition.feedback_type.clone(),
            definition: definition.clone(),
            start_time: Instant::now(),
            progress: 0.0,
            completed: false,
            paused: false,
            intensity,
        };

        self.active_feedback.insert(feedback_id, feedback);

        Ok(())
    }

    /// Stop feedback
    pub fn stop_feedback(&mut self, element_id: &str, feedback_name: &str) -> UIResult<()> {
        let feedback_id = format!("{}_{}", element_id, feedback_name);
        self.active_feedback.remove(&feedback_id);
        Ok(())
    }

    /// Pause feedback
    pub fn pause_feedback(&mut self, element_id: &str, feedback_name: &str) -> UIResult<()> {
        let feedback_id = format!("{}_{}", element_id, feedback_name);
        if let Some(feedback) = self.active_feedback.get_mut(&feedback_id) {
            feedback.paused = true;
        }
        Ok(())
    }

    /// Resume feedback
    pub fn resume_feedback(&mut self, element_id: &str, feedback_name: &str) -> UIResult<()> {
        let feedback_id = format!("{}_{}", element_id, feedback_name);
        if let Some(feedback) = self.active_feedback.get_mut(&feedback_id) {
            feedback.paused = false;
        }
        Ok(())
    }

    /// Add feedback definition
    pub fn add_feedback_definition(&mut self, definition: UIVisualFeedbackDefinition) {
        self.feedback_definitions.insert(definition.name.clone(), definition);
    }

    /// Remove feedback definition
    pub fn remove_feedback_definition(&mut self, name: &str) {
        self.feedback_definitions.remove(name);
    }

    /// Get active feedback for element
    pub fn get_element_feedback(&self, element_id: &str) -> Vec<&UIVisualFeedback> {
        self.active_feedback.values()
            .filter(|feedback| feedback.element_id == element_id)
            .collect()
    }

    /// Check if element has active feedback
    pub fn has_active_feedback(&self, element_id: &str) -> bool {
        self.active_feedback.values()
            .any(|feedback| feedback.element_id == element_id && !feedback.completed)
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

    /// Apply feedback properties
    fn apply_feedback_properties(&self, feedback: &UIVisualFeedback, progress: f32) -> UIResult<()> {
        let intensity = feedback.intensity;

        // Color feedback
        if let Some(color) = &feedback.definition.properties.color {
            let (start_r, start_g, start_b, start_a) = color.start_color;
            let (end_r, end_g, end_b, end_a) = color.end_color;
            let current_r = start_r + (end_r - start_r) * progress * intensity;
            let current_g = start_g + (end_g - start_g) * progress * intensity;
            let current_b = start_b + (end_b - start_b) * progress * intensity;
            let current_a = start_a + (end_a - start_a) * progress * intensity;
            // Apply color to element
        }

        // Glow feedback
        if let Some(glow) = &feedback.definition.properties.glow {
            let glow_intensity = glow.intensity * progress * intensity;
            let glow_radius = glow.radius * progress * intensity;
            // Apply glow effect to element
        }

        // Shake feedback
        if let Some(shake) = &feedback.definition.properties.shake {
            let shake_intensity = shake.intensity * progress * intensity;
            let shake_frequency = shake.frequency;
            // Apply shake effect to element
        }

        // Pulse feedback
        if let Some(pulse) = &feedback.definition.properties.pulse {
            let pulse_scale = 1.0 + (pulse.scale_factor - 1.0) * progress * intensity;
            let pulse_frequency = pulse.frequency;
            // Apply pulse effect to element
        }

        // Scale feedback
        if let Some(scale) = &feedback.definition.properties.scale {
            let scale_factor = 1.0 + (scale.scale_factor - 1.0) * progress * intensity;
            // Apply scale effect to element
        }

        // Border feedback
        if let Some(border) = &feedback.definition.properties.border {
            let border_width = border.border_width * progress * intensity;
            let (border_r, border_g, border_b, border_a) = border.border_color;
            // Apply border effect to element
        }

        // Particle feedback
        if let Some(particle) = &feedback.definition.properties.particle {
            let particle_count = (particle.particle_count as f32 * progress * intensity) as u32;
            let particle_lifetime = particle.particle_lifetime * progress * intensity;
            // Apply particle effect to element
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

impl Default for UIVisualFeedbackManager {
    fn default() -> Self {
        Self::new()
    }
}
