//! Animation data structures and definitions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single frame in an animation sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationFrame {
    /// Texture ID for this frame
    pub texture_id: u32,
    /// Frame duration in seconds
    pub duration: f32,
    /// Frame offset from sprite sheet
    pub offset_x: f32,
    pub offset_y: f32,
    /// Frame size
    pub width: f32,
    pub height: f32,
    /// Color tint for this frame
    pub color: [f32; 4],
    /// Frame-specific effects
    pub effects: Vec<FrameEffect>,
}

/// Effects that can be applied to individual frames
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrameEffect {
    /// Screen shake effect
    ScreenShake { intensity: f32, duration: f32 },
    /// Particle effect
    ParticleEffect { effect_name: String, position: [f32; 2] },
    /// Sound effect
    SoundEffect { sound_name: String, volume: f32 },
    /// Camera zoom
    CameraZoom { zoom: f32, duration: f32 },
}

/// Complete animation sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    /// Animation name/identifier
    pub name: String,
    /// All frames in this animation
    pub frames: Vec<AnimationFrame>,
    /// Total animation duration
    pub duration: f32,
    /// Whether this animation loops
    pub looping: bool,
    /// Animation playback speed multiplier
    pub speed: f32,
    /// Animation priority (higher = more important)
    pub priority: u32,
    /// Animation tags for categorization
    pub tags: Vec<String>,
    /// Animation metadata
    pub metadata: HashMap<String, String>,
}

impl Animation {
    /// Create a new animation
    pub fn new(name: String) -> Self {
        Self {
            name,
            frames: Vec::new(),
            duration: 0.0,
            looping: false,
            speed: 1.0,
            priority: 0,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a frame to this animation
    pub fn add_frame(&mut self, frame: AnimationFrame) {
        self.duration += frame.duration;
        self.frames.push(frame);
    }

    /// Set animation looping
    pub fn set_looping(&mut self, looping: bool) -> &mut Self {
        self.looping = looping;
        self
    }

    /// Set animation speed
    pub fn set_speed(&mut self, speed: f32) -> &mut Self {
        self.speed = speed;
        self
    }

    /// Set animation priority
    pub fn set_priority(&mut self, priority: u32) -> &mut Self {
        self.priority = priority;
        self
    }

    /// Add a tag to this animation
    pub fn add_tag(&mut self, tag: String) -> &mut Self {
        self.tags.push(tag);
        self
    }

    /// Set metadata for this animation
    pub fn set_metadata(&mut self, key: String, value: String) -> &mut Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get frame at specific time
    pub fn get_frame_at_time(&self, time: f32) -> Option<&AnimationFrame> {
        let mut accumulated_time = 0.0;
        
        for frame in &self.frames {
            if time >= accumulated_time && time < accumulated_time + frame.duration {
                return Some(frame);
            }
            accumulated_time += frame.duration;
        }
        
        // If looping and we've passed the end, loop back
        if self.looping && time >= self.duration {
            let looped_time = time % self.duration;
            return self.get_frame_at_time(looped_time);
        }
        
        None
    }

    /// Get animation progress (0.0 to 1.0)
    pub fn get_progress(&self, time: f32) -> f32 {
        if self.duration == 0.0 {
            return 0.0;
        }
        
        if self.looping {
            (time % self.duration) / self.duration
        } else {
            (time / self.duration).min(1.0)
        }
    }

    /// Check if animation is complete at given time
    pub fn is_complete(&self, time: f32) -> bool {
        !self.looping && time >= self.duration
    }
}

/// Animation state for an entity
#[derive(Debug, Clone)]
pub struct AnimationState {
    /// Current animation name
    pub current_animation: String,
    /// Current frame index
    pub current_frame: usize,
    /// Elapsed time in current frame
    pub elapsed_time: f32,
    /// Whether animation is currently playing
    pub is_playing: bool,
    /// Speed multiplier for this animation
    pub speed_multiplier: f32,
}

impl AnimationState {
    /// Create a new animation state
    pub fn new(animation_name: String) -> Self {
        Self {
            current_animation: animation_name,
            current_frame: 0,
            elapsed_time: 0.0,
            is_playing: true,
            speed_multiplier: 1.0,
        }
    }

    /// Reset animation state
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.elapsed_time = 0.0;
        self.is_playing = true;
    }
}

/// Animation transition definition
#[derive(Debug, Clone)]
pub struct AnimationTransition {
    /// Source animation name
    pub from: String,
    /// Target animation name
    pub to: String,
    /// Transition condition
    pub condition: TransitionCondition,
    /// Transition priority
    pub priority: u32,
    /// Whether to blend between animations
    pub blend: bool,
    /// Blend duration in seconds
    pub blend_duration: f32,
}

/// Conditions for animation transitions
#[derive(Debug, Clone)]
pub enum TransitionCondition {
    /// Transition when animation completes
    OnComplete,
    /// Transition when specific input is pressed
    OnInput { input: String },
    /// Transition when specific state is reached
    OnState { state: String },
    /// Transition when condition function returns true
    OnCondition { condition: String },
    /// Transition immediately
    Immediate,
}

/// Animation blend state for smooth transitions
#[derive(Debug, Clone)]
pub struct AnimationBlend {
    /// Source animation
    pub from_animation: String,
    /// Target animation
    pub to_animation: String,
    /// Blend progress (0.0 to 1.0)
    pub progress: f32,
    /// Blend duration
    pub duration: f32,
    /// Elapsed blend time
    pub elapsed_time: f32,
}

impl AnimationBlend {
    /// Create a new animation blend
    pub fn new(from: String, to: String, duration: f32) -> Self {
        Self {
            from_animation: from,
            to_animation: to,
            progress: 0.0,
            duration,
            elapsed_time: 0.0,
        }
    }

    /// Update blend progress
    pub fn update(&mut self, dt: f32) -> bool {
        self.elapsed_time += dt;
        self.progress = (self.elapsed_time / self.duration).min(1.0);
        
        // Return true if blend is complete
        self.progress >= 1.0
    }

    /// Get blend weight for source animation
    pub fn get_from_weight(&self) -> f32 {
        1.0 - self.progress
    }

    /// Get blend weight for target animation
    pub fn get_to_weight(&self) -> f32 {
        self.progress
    }
}
