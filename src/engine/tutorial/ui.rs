//! Tutorial UI system
//! 
//! This module provides the user interface components for the tutorial system.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Tutorial UI manager
#[derive(Debug, Clone)]
pub struct TutorialUI {
    /// UI elements
    pub elements: HashMap<String, TutorialUIElement>,
    /// Active overlays
    pub active_overlays: Vec<TutorialOverlay>,
    /// UI settings
    pub settings: TutorialUISettings,
    /// Animation state
    pub animation_state: AnimationState,
}

impl TutorialUI {
    /// Create a new tutorial UI
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            active_overlays: Vec::new(),
            settings: TutorialUISettings::default(),
            animation_state: AnimationState::default(),
        }
    }

    /// Add UI element
    pub fn add_element(&mut self, id: String, element: TutorialUIElement) {
        self.elements.insert(id, element);
    }

    /// Show overlay
    pub fn show_overlay(&mut self, overlay: TutorialOverlay) {
        self.active_overlays.push(overlay);
    }

    /// Hide overlay
    pub fn hide_overlay(&mut self, overlay_id: &str) {
        self.active_overlays.retain(|o| o.id != overlay_id);
    }

    /// Update UI
    pub fn update(&mut self, delta_time: f32) {
        // Update animations
        self.animation_state.update(delta_time);
        
        // Update overlays
        for overlay in &mut self.active_overlays {
            overlay.update(delta_time);
        }
    }

    /// Render UI
    pub fn render(&self) {
        // Render overlays
        for overlay in &self.active_overlays {
            overlay.render();
        }
    }
}

impl Default for TutorialUI {
    fn default() -> Self {
        Self::new()
    }
}

/// Tutorial UI element
#[derive(Debug, Clone)]
pub struct TutorialUIElement {
    /// Element ID
    pub id: String,
    /// Element type
    pub element_type: TutorialUIElementType,
    /// Position
    pub position: (f32, f32),
    /// Size
    pub size: (f32, f32),
    /// Visible
    pub visible: bool,
    /// Properties
    pub properties: TutorialUIProperties,
    /// Animation
    pub animation: Option<UIAnimation>,
}

/// Tutorial UI element types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TutorialUIElementType {
    /// Text element
    Text,
    /// Button element
    Button,
    /// Image element
    Image,
    /// Progress bar
    ProgressBar,
    /// Highlight box
    HighlightBox,
    /// Arrow
    Arrow,
    /// Tooltip
    Tooltip,
    /// Custom element
    Custom(String),
}

/// Tutorial UI properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialUIProperties {
    /// Text content
    pub text: Option<String>,
    /// Font size
    pub font_size: f32,
    /// Color
    pub color: (f32, f32, f32, f32),
    /// Background color
    pub background_color: Option<(f32, f32, f32, f32)>,
    /// Border color
    pub border_color: Option<(f32, f32, f32, f32)>,
    /// Border width
    pub border_width: f32,
    /// Opacity
    pub opacity: f32,
    /// Scale
    pub scale: f32,
    /// Rotation
    pub rotation: f32,
}

impl Default for TutorialUIProperties {
    fn default() -> Self {
        Self {
            text: None,
            font_size: 16.0,
            color: (1.0, 1.0, 1.0, 1.0),
            background_color: None,
            border_color: None,
            border_width: 0.0,
            opacity: 1.0,
            scale: 1.0,
            rotation: 0.0,
        }
    }
}

/// Tutorial overlay
#[derive(Debug, Clone)]
pub struct TutorialOverlay {
    /// Overlay ID
    pub id: String,
    /// Overlay type
    pub overlay_type: TutorialOverlayType,
    /// Position
    pub position: (f32, f32),
    /// Size
    pub size: (f32, f32),
    /// Visible
    pub visible: bool,
    /// Opacity
    pub opacity: f32,
    /// Animation
    pub animation: Option<UIAnimation>,
    /// Content
    pub content: TutorialOverlayContent,
    /// Duration
    pub duration: f32,
    /// Age
    pub age: f32,
}

impl TutorialOverlay {
    /// Create a new overlay
    pub fn new(id: String, overlay_type: TutorialOverlayType) -> Self {
        Self {
            id,
            overlay_type,
            position: (0.0, 0.0),
            size: (100.0, 100.0),
            visible: true,
            opacity: 1.0,
            animation: None,
            content: TutorialOverlayContent::Text(String::new()),
            duration: 0.0,
            age: 0.0,
        }
    }

    /// Update overlay
    pub fn update(&mut self, delta_time: f32) {
        self.age += delta_time;
        
        if let Some(animation) = &mut self.animation {
            animation.update(delta_time);
        }
    }

    /// Render overlay
    pub fn render(&self) {
        if !self.visible {
            return;
        }
        
        // Render based on content type
        match &self.content {
            TutorialOverlayContent::Text(text) => {
                // Render text
            }
            TutorialOverlayContent::Image(image_path) => {
                // Render image
            }
            TutorialOverlayContent::Highlight(highlight) => {
                // Render highlight
            }
        }
    }
}

/// Tutorial overlay types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TutorialOverlayType {
    /// Tooltip overlay
    Tooltip,
    /// Highlight overlay
    Highlight,
    /// Modal overlay
    Modal,
    /// Banner overlay
    Banner,
    /// Custom overlay
    Custom(String),
}

/// Tutorial overlay content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TutorialOverlayContent {
    /// Text content
    Text(String),
    /// Image content
    Image(String),
    /// Highlight content
    Highlight(TutorialHighlight),
}

/// Tutorial highlight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialHighlight {
    /// Highlight ID
    pub id: String,
    /// Target element
    pub target: String,
    /// Highlight type
    pub highlight_type: TutorialHighlightType,
    /// Properties
    pub properties: TutorialHighlightProperties,
    /// Animation
    pub animation: Option<TutorialHighlightAnimation>,
}

/// Tutorial highlight types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TutorialHighlightType {
    /// Glow effect
    Glow,
    /// Outline effect
    Outline,
    /// Pulse effect
    Pulse,
    /// Arrow pointing
    Arrow,
    /// Circle around
    Circle,
    /// Rectangle around
    Rectangle,
    /// Custom effect
    Custom(String),
}

/// Tutorial highlight properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialHighlightProperties {
    /// Color
    pub color: (f32, f32, f32, f32),
    /// Intensity
    pub intensity: f32,
    /// Size
    pub size: f32,
    /// Thickness
    pub thickness: f32,
    /// Opacity
    pub opacity: f32,
    /// Duration
    pub duration: f32,
}

impl Default for TutorialHighlightProperties {
    fn default() -> Self {
        Self {
            color: (1.0, 1.0, 0.0, 1.0), // Yellow
            intensity: 1.0,
            size: 1.0,
            thickness: 2.0,
            opacity: 0.8,
            duration: 2.0,
        }
    }
}

/// Tutorial highlight animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialHighlightAnimation {
    /// Animation type
    pub animation_type: TutorialHighlightAnimationType,
    /// Duration
    pub duration: f32,
    /// Loop
    pub loop_animation: bool,
    /// Easing
    pub easing: TutorialEasingType,
}

/// Tutorial highlight animation types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TutorialHighlightAnimationType {
    /// Fade in
    FadeIn,
    /// Fade out
    FadeOut,
    /// Scale in
    ScaleIn,
    /// Scale out
    ScaleOut,
    /// Pulse
    Pulse,
    /// Rotate
    Rotate,
    /// Move
    Move,
    /// Custom animation
    Custom(String),
}

/// Tutorial easing types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TutorialEasingType {
    /// Linear
    Linear,
    /// Ease in
    EaseIn,
    /// Ease out
    EaseOut,
    /// Ease in out
    EaseInOut,
    /// Bounce
    Bounce,
    /// Elastic
    Elastic,
    /// Custom easing
    Custom(String),
}

/// UI animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIAnimation {
    /// Animation type
    pub animation_type: UIAnimationType,
    /// Duration
    pub duration: f32,
    /// Start time
    pub start_time: f32,
    /// Current time
    pub current_time: f32,
    /// Loop
    pub loop_animation: bool,
    /// Easing
    pub easing: TutorialEasingType,
    /// Properties
    pub properties: UIAnimationProperties,
}

impl UIAnimation {
    /// Create a new animation
    pub fn new(animation_type: UIAnimationType, duration: f32) -> Self {
        Self {
            animation_type,
            duration,
            start_time: 0.0,
            current_time: 0.0,
            loop_animation: false,
            easing: TutorialEasingType::Linear,
            properties: UIAnimationProperties::default(),
        }
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
        
        if self.current_time >= self.duration {
            if self.loop_animation {
                self.current_time = 0.0;
            } else {
                self.current_time = self.duration;
            }
        }
    }

    /// Get animation progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        self.current_time / self.duration
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        self.current_time >= self.duration && !self.loop_animation
    }
}

/// UI animation types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIAnimationType {
    /// Fade in
    FadeIn,
    /// Fade out
    FadeOut,
    /// Scale in
    ScaleIn,
    /// Scale out
    ScaleOut,
    /// Move
    Move,
    /// Rotate
    Rotate,
    /// Pulse
    Pulse,
    /// Custom animation
    Custom(String),
}

/// UI animation properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIAnimationProperties {
    /// Start position
    pub start_position: (f32, f32),
    /// End position
    pub end_position: (f32, f32),
    /// Start scale
    pub start_scale: f32,
    /// End scale
    pub end_scale: f32,
    /// Start opacity
    pub start_opacity: f32,
    /// End opacity
    pub end_opacity: f32,
    /// Start rotation
    pub start_rotation: f32,
    /// End rotation
    pub end_rotation: f32,
}

impl Default for UIAnimationProperties {
    fn default() -> Self {
        Self {
            start_position: (0.0, 0.0),
            end_position: (0.0, 0.0),
            start_scale: 1.0,
            end_scale: 1.0,
            start_opacity: 1.0,
            end_opacity: 1.0,
            start_rotation: 0.0,
            end_rotation: 0.0,
        }
    }
}

/// Animation state
#[derive(Debug, Clone, Default)]
pub struct AnimationState {
    /// Active animations
    pub active_animations: Vec<UIAnimation>,
    /// Animation time
    pub animation_time: f32,
}

impl AnimationState {
    /// Update animations
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
        
        for animation in &mut self.active_animations {
            animation.update(delta_time);
        }
        
        // Remove completed animations
        self.active_animations.retain(|a| !a.is_complete());
    }

    /// Add animation
    pub fn add_animation(&mut self, animation: UIAnimation) {
        self.active_animations.push(animation);
    }
}

/// Tutorial UI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialUISettings {
    /// Show UI
    pub show_ui: bool,
    /// Show highlights
    pub show_highlights: bool,
    /// Show tooltips
    pub show_tooltips: bool,
    /// Show progress
    pub show_progress: bool,
    /// UI scale
    pub ui_scale: f32,
    /// Animation speed
    pub animation_speed: f32,
    /// Fade duration
    pub fade_duration: f32,
    /// Highlight intensity
    pub highlight_intensity: f32,
    /// Font size
    pub font_size: f32,
    /// Color scheme
    pub color_scheme: TutorialColorScheme,
}

impl Default for TutorialUISettings {
    fn default() -> Self {
        Self {
            show_ui: true,
            show_highlights: true,
            show_tooltips: true,
            show_progress: true,
            ui_scale: 1.0,
            animation_speed: 1.0,
            fade_duration: 0.5,
            highlight_intensity: 0.8,
            font_size: 16.0,
            color_scheme: TutorialColorScheme::default(),
        }
    }
}

/// Tutorial color scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialColorScheme {
    /// Primary color
    pub primary: (f32, f32, f32, f32),
    /// Secondary color
    pub secondary: (f32, f32, f32, f32),
    /// Accent color
    pub accent: (f32, f32, f32, f32),
    /// Background color
    pub background: (f32, f32, f32, f32),
    /// Text color
    pub text: (f32, f32, f32, f32),
    /// Highlight color
    pub highlight: (f32, f32, f32, f32),
    /// Success color
    pub success: (f32, f32, f32, f32),
    /// Warning color
    pub warning: (f32, f32, f32, f32),
    /// Error color
    pub error: (f32, f32, f32, f32),
}

impl Default for TutorialColorScheme {
    fn default() -> Self {
        Self {
            primary: (0.2, 0.4, 0.8, 1.0),      // Blue
            secondary: (0.6, 0.6, 0.6, 1.0),    // Gray
            accent: (1.0, 0.8, 0.0, 1.0),       // Yellow
            background: (0.1, 0.1, 0.1, 0.9),   // Dark gray
            text: (1.0, 1.0, 1.0, 1.0),         // White
            highlight: (1.0, 1.0, 0.0, 1.0),    // Yellow
            success: (0.0, 0.8, 0.0, 1.0),      // Green
            warning: (1.0, 0.6, 0.0, 1.0),      // Orange
            error: (0.8, 0.0, 0.0, 1.0),        // Red
        }
    }
}
