//! UI Polish System
//! 
//! This module provides comprehensive UI polish, animations, transitions, and visual feedback
//! for the 2D brawler game.

pub mod animations;
pub mod transitions;
pub mod visual_feedback;
pub mod ui_manager;
pub mod themes;
pub mod components;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Re-export main components
pub use animations::UIAnimationManager;
pub use transitions::UITransitionManager;
pub use visual_feedback::UIVisualFeedbackManager;
pub use ui_manager::UIManager;
pub use themes::UIThemeManager;
pub use components::UIComponentManager;

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    /// Enable UI animations
    pub animations_enabled: bool,
    /// Enable UI transitions
    pub transitions_enabled: bool,
    /// Enable visual feedback
    pub visual_feedback_enabled: bool,
    /// Default animation duration in seconds
    pub default_animation_duration: f32,
    /// Default transition duration in seconds
    pub default_transition_duration: f32,
    /// UI scale factor
    pub ui_scale: f32,
    /// Enable UI sound effects
    pub ui_sounds_enabled: bool,
    /// Enable UI haptic feedback
    pub haptic_feedback_enabled: bool,
    /// Theme name
    pub theme_name: String,
    /// Enable accessibility features
    pub accessibility_enabled: bool,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            animations_enabled: true,
            transitions_enabled: true,
            visual_feedback_enabled: true,
            default_animation_duration: 0.3,
            default_transition_duration: 0.5,
            ui_scale: 1.0,
            ui_sounds_enabled: true,
            haptic_feedback_enabled: false,
            theme_name: "default".to_string(),
            accessibility_enabled: true,
        }
    }
}

/// UI element types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIElementType {
    /// Button
    Button,
    /// Label
    Label,
    /// Panel
    Panel,
    /// Menu
    Menu,
    /// Dialog
    Dialog,
    /// Progress bar
    ProgressBar,
    /// Slider
    Slider,
    /// Checkbox
    Checkbox,
    /// Text input
    TextInput,
    /// Image
    Image,
    /// Container
    Container,
    /// List
    List,
    /// Grid
    Grid,
    /// Tab
    Tab,
    /// Tooltip
    Tooltip,
    /// Notification
    Notification,
}

/// UI animation types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UIAnimationType {
    /// Fade in/out
    Fade,
    /// Scale up/down
    Scale,
    /// Slide in/out
    Slide,
    /// Rotate
    Rotate,
    /// Bounce
    Bounce,
    /// Shake
    Shake,
    /// Pulse
    Pulse,
    /// Glow
    Glow,
    /// Color change
    ColorChange,
    /// Custom animation
    Custom(String),
}

/// UI transition types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UITransitionType {
    /// Fade transition
    Fade,
    /// Slide transition
    Slide,
    /// Scale transition
    Scale,
    /// Flip transition
    Flip,
    /// Wipe transition
    Wipe,
    /// Dissolve transition
    Dissolve,
    /// Custom transition
    Custom(String),
}

/// UI visual feedback types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UIVisualFeedbackType {
    /// Button press feedback
    ButtonPress,
    /// Hover feedback
    Hover,
    /// Focus feedback
    Focus,
    /// Success feedback
    Success,
    /// Error feedback
    Error,
    /// Warning feedback
    Warning,
    /// Info feedback
    Info,
    /// Loading feedback
    Loading,
    /// Custom feedback
    Custom(String),
}

/// UI events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIEvent {
    /// Element clicked
    ElementClicked { element_id: String, button: MouseButton },
    /// Element hovered
    ElementHovered { element_id: String },
    /// Element focused
    ElementFocused { element_id: String },
    /// Element unfocused
    ElementUnfocused { element_id: String },
    /// Animation started
    AnimationStarted { element_id: String, animation_type: UIAnimationType },
    /// Animation completed
    AnimationCompleted { element_id: String, animation_type: UIAnimationType },
    /// Transition started
    TransitionStarted { from_element: String, to_element: String, transition_type: UITransitionType },
    /// Transition completed
    TransitionCompleted { from_element: String, to_element: String, transition_type: UITransitionType },
    /// Visual feedback triggered
    VisualFeedbackTriggered { element_id: String, feedback_type: UIVisualFeedbackType },
    /// Theme changed
    ThemeChanged { old_theme: String, new_theme: String },
    /// UI scale changed
    UIScaleChanged { old_scale: f32, new_scale: f32 },
}

/// Mouse button types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button
    Middle,
    /// Other mouse button
    Other(u8),
}

/// UI result type
pub type UIResult<T> = Result<T, UIError>;

/// UI error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIError {
    /// Element not found
    ElementNotFound(String),
    /// Animation not found
    AnimationNotFound(String),
    /// Transition not found
    TransitionNotFound(String),
    /// Theme not found
    ThemeNotFound(String),
    /// Invalid configuration
    InvalidConfig(String),
    /// Animation already running
    AnimationAlreadyRunning(String),
    /// Transition already running
    TransitionAlreadyRunning(String),
    /// UI system not initialized
    SystemNotInitialized,
    /// Unknown error
    Unknown(String),
}

impl std::fmt::Display for UIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UIError::ElementNotFound(id) => write!(f, "UI element not found: {}", id),
            UIError::AnimationNotFound(name) => write!(f, "UI animation not found: {}", name),
            UIError::TransitionNotFound(name) => write!(f, "UI transition not found: {}", name),
            UIError::ThemeNotFound(name) => write!(f, "UI theme not found: {}", name),
            UIError::InvalidConfig(msg) => write!(f, "Invalid UI configuration: {}", msg),
            UIError::AnimationAlreadyRunning(id) => write!(f, "Animation already running for element: {}", id),
            UIError::TransitionAlreadyRunning(id) => write!(f, "Transition already running for element: {}", id),
            UIError::SystemNotInitialized => write!(f, "UI system not initialized"),
            UIError::Unknown(msg) => write!(f, "Unknown UI error: {}", msg),
        }
    }
}

impl std::error::Error for UIError {}
