//! UI Themes
//! 
//! This module provides comprehensive theming system for UI elements.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{UIResult, UIError, UIEvent};

/// UI theme manager
pub struct UIThemeManager {
    /// Current theme
    pub current_theme: String,
    /// Available themes
    pub themes: HashMap<String, UITheme>,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&UIEvent) + Send + Sync>>,
}

/// UI theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UITheme {
    /// Theme name
    pub name: String,
    /// Theme description
    pub description: String,
    /// Theme version
    pub version: String,
    /// Theme author
    pub author: String,
    /// Color palette
    pub colors: ColorPalette,
    /// Typography settings
    pub typography: TypographySettings,
    /// Spacing settings
    pub spacing: SpacingSettings,
    /// Border settings
    pub borders: BorderSettings,
    /// Shadow settings
    pub shadows: ShadowSettings,
    /// Animation settings
    pub animations: AnimationSettings,
    /// Component styles
    pub components: HashMap<String, ComponentStyle>,
}

/// Color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Primary colors
    pub primary: ColorSet,
    /// Secondary colors
    pub secondary: ColorSet,
    /// Accent colors
    pub accent: ColorSet,
    /// Neutral colors
    pub neutral: ColorSet,
    /// Success colors
    pub success: ColorSet,
    /// Warning colors
    pub warning: ColorSet,
    /// Error colors
    pub error: ColorSet,
    /// Info colors
    pub info: ColorSet,
    /// Background colors
    pub background: BackgroundColors,
    /// Text colors
    pub text: TextColors,
    /// Border colors
    pub border: BorderColors,
}

/// Color set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSet {
    /// Main color
    pub main: (f32, f32, f32, f32),
    /// Light variant
    pub light: (f32, f32, f32, f32),
    /// Dark variant
    pub dark: (f32, f32, f32, f32),
    /// Contrast color
    pub contrast: (f32, f32, f32, f32),
}

/// Background colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundColors {
    /// Primary background
    pub primary: (f32, f32, f32, f32),
    /// Secondary background
    pub secondary: (f32, f32, f32, f32),
    /// Tertiary background
    pub tertiary: (f32, f32, f32, f32),
    /// Overlay background
    pub overlay: (f32, f32, f32, f32),
    /// Modal background
    pub modal: (f32, f32, f32, f32),
    /// Card background
    pub card: (f32, f32, f32, f32),
}

/// Text colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextColors {
    /// Primary text
    pub primary: (f32, f32, f32, f32),
    /// Secondary text
    pub secondary: (f32, f32, f32, f32),
    /// Tertiary text
    pub tertiary: (f32, f32, f32, f32),
    /// Disabled text
    pub disabled: (f32, f32, f32, f32),
    /// Link text
    pub link: (f32, f32, f32, f32),
    /// Error text
    pub error: (f32, f32, f32, f32),
    /// Success text
    pub success: (f32, f32, f32, f32),
    /// Warning text
    pub warning: (f32, f32, f32, f32),
}

/// Border colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderColors {
    /// Primary border
    pub primary: (f32, f32, f32, f32),
    /// Secondary border
    pub secondary: (f32, f32, f32, f32),
    /// Focus border
    pub focus: (f32, f32, f32, f32),
    /// Error border
    pub error: (f32, f32, f32, f32),
    /// Success border
    pub success: (f32, f32, f32, f32),
    /// Warning border
    pub warning: (f32, f32, f32, f32),
}

/// Typography settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographySettings {
    /// Font families
    pub font_families: FontFamilies,
    /// Font sizes
    pub font_sizes: FontSizes,
    /// Font weights
    pub font_weights: FontWeights,
    /// Line heights
    pub line_heights: LineHeights,
    /// Letter spacing
    pub letter_spacing: LetterSpacing,
    /// Text styles
    pub text_styles: HashMap<String, TextStyle>,
}

/// Font families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontFamilies {
    /// Primary font family
    pub primary: String,
    /// Secondary font family
    pub secondary: String,
    /// Monospace font family
    pub monospace: String,
    /// Display font family
    pub display: String,
}

/// Font sizes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSizes {
    /// Extra small
    pub xs: f32,
    /// Small
    pub sm: f32,
    /// Medium
    pub md: f32,
    /// Large
    pub lg: f32,
    /// Extra large
    pub xl: f32,
    /// Extra extra large
    pub xxl: f32,
    /// Huge
    pub huge: f32,
}

/// Font weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontWeights {
    /// Light
    pub light: u32,
    /// Regular
    pub regular: u32,
    /// Medium
    pub medium: u32,
    /// Semi bold
    pub semibold: u32,
    /// Bold
    pub bold: u32,
    /// Extra bold
    pub extrabold: u32,
}

/// Line heights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineHeights {
    /// Tight
    pub tight: f32,
    /// Normal
    pub normal: f32,
    /// Relaxed
    pub relaxed: f32,
    /// Loose
    pub loose: f32,
}

/// Letter spacing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetterSpacing {
    /// Tight
    pub tight: f32,
    /// Normal
    pub normal: f32,
    /// Wide
    pub wide: f32,
    /// Wider
    pub wider: f32,
}

/// Text style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    /// Font family
    pub font_family: String,
    /// Font size
    pub font_size: f32,
    /// Font weight
    pub font_weight: u32,
    /// Line height
    pub line_height: f32,
    /// Letter spacing
    pub letter_spacing: f32,
    /// Text color
    pub color: (f32, f32, f32, f32),
    /// Text decoration
    pub decoration: TextDecoration,
    /// Text transform
    pub transform: TextTransform,
}

/// Text decoration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextDecoration {
    /// No decoration
    None,
    /// Underline
    Underline,
    /// Overline
    Overline,
    /// Line through
    LineThrough,
}

/// Text transform
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextTransform {
    /// No transform
    None,
    /// Uppercase
    Uppercase,
    /// Lowercase
    Lowercase,
    /// Capitalize
    Capitalize,
}

/// Spacing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingSettings {
    /// Spacing scale
    pub scale: f32,
    /// Spacing values
    pub values: SpacingValues,
    /// Component spacing
    pub components: HashMap<String, ComponentSpacing>,
}

/// Spacing values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingValues {
    /// Extra small
    pub xs: f32,
    /// Small
    pub sm: f32,
    /// Medium
    pub md: f32,
    /// Large
    pub lg: f32,
    /// Extra large
    pub xl: f32,
    /// Extra extra large
    pub xxl: f32,
}

/// Component spacing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSpacing {
    /// Padding
    pub padding: SpacingRect,
    /// Margin
    pub margin: SpacingRect,
    /// Gap
    pub gap: f32,
}

/// Spacing rectangle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingRect {
    /// Top
    pub top: f32,
    /// Right
    pub right: f32,
    /// Bottom
    pub bottom: f32,
    /// Left
    pub left: f32,
}

/// Border settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderSettings {
    /// Border widths
    pub widths: BorderWidths,
    /// Border styles
    pub styles: BorderStyles,
    /// Border radius
    pub radius: BorderRadius,
}

/// Border widths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderWidths {
    /// None
    pub none: f32,
    /// Thin
    pub thin: f32,
    /// Medium
    pub medium: f32,
    /// Thick
    pub thick: f32,
}

/// Border styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderStyles {
    /// Solid
    pub solid: String,
    /// Dashed
    pub dashed: String,
    /// Dotted
    pub dotted: String,
    /// Double
    pub double: String,
}

/// Border radius
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderRadius {
    /// None
    pub none: f32,
    /// Small
    pub small: f32,
    /// Medium
    pub medium: f32,
    /// Large
    pub large: f32,
    /// Extra large
    pub extra_large: f32,
    /// Full
    pub full: f32,
}

/// Shadow settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowSettings {
    /// Shadow definitions
    pub shadows: HashMap<String, ShadowDefinition>,
    /// Default shadow
    pub default: String,
}

/// Shadow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowDefinition {
    /// Shadow name
    pub name: String,
    /// Shadow color
    pub color: (f32, f32, f32, f32),
    /// Shadow offset X
    pub offset_x: f32,
    /// Shadow offset Y
    pub offset_y: f32,
    /// Shadow blur radius
    pub blur_radius: f32,
    /// Shadow spread
    pub spread: f32,
}

/// Animation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSettings {
    /// Animation durations
    pub durations: AnimationDurations,
    /// Animation easings
    pub easings: AnimationEasings,
    /// Animation delays
    pub delays: AnimationDelays,
}

/// Animation durations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationDurations {
    /// Fast
    pub fast: f32,
    /// Normal
    pub normal: f32,
    /// Slow
    pub slow: f32,
    /// Very slow
    pub very_slow: f32,
}

/// Animation easings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationEasings {
    /// Default easing
    pub default: String,
    /// Ease in
    pub ease_in: String,
    /// Ease out
    pub ease_out: String,
    /// Ease in-out
    pub ease_in_out: String,
}

/// Animation delays
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationDelays {
    /// No delay
    pub none: f32,
    /// Short delay
    pub short: f32,
    /// Medium delay
    pub medium: f32,
    /// Long delay
    pub long: f32,
}

/// Component style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStyle {
    /// Component name
    pub name: String,
    /// Base styles
    pub base: HashMap<String, String>,
    /// State styles
    pub states: HashMap<String, HashMap<String, String>>,
    /// Variant styles
    pub variants: HashMap<String, HashMap<String, String>>,
    /// Size styles
    pub sizes: HashMap<String, HashMap<String, String>>,
}

impl UIThemeManager {
    /// Create a new UI theme manager
    pub fn new() -> Self {
        let mut manager = Self {
            current_theme: "default".to_string(),
            themes: HashMap::new(),
            event_handlers: Vec::new(),
        };

        // Initialize default theme
        manager.initialize_default_theme();
        manager
    }

    /// Set current theme
    pub fn set_theme(&mut self, theme_name: &str) -> UIResult<()> {
        if !self.themes.contains_key(theme_name) {
            return Err(UIError::ThemeNotFound(theme_name.to_string()));
        }

        let old_theme = self.current_theme.clone();
        self.current_theme = theme_name.to_string();

        // Emit theme changed event
        self.emit_event(UIEvent::ThemeChanged {
            old_theme,
            new_theme: theme_name.to_string(),
        });

        Ok(())
    }

    /// Get current theme
    pub fn get_current_theme(&self) -> &UITheme {
        self.themes.get(&self.current_theme)
            .expect("Current theme should always exist")
    }

    /// Add theme
    pub fn add_theme(&mut self, theme: UITheme) {
        self.themes.insert(theme.name.clone(), theme);
    }

    /// Remove theme
    pub fn remove_theme(&mut self, theme_name: &str) -> UIResult<()> {
        if theme_name == "default" {
            return Err(UIError::InvalidConfig("Cannot remove default theme".to_string()));
        }

        if self.current_theme == theme_name {
            self.current_theme = "default".to_string();
        }

        self.themes.remove(theme_name);
        Ok(())
    }

    /// Get theme
    pub fn get_theme(&self, theme_name: &str) -> Option<&UITheme> {
        self.themes.get(theme_name)
    }

    /// Get all theme names
    pub fn get_theme_names(&self) -> Vec<&String> {
        self.themes.keys().collect()
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&UIEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Initialize default theme
    fn initialize_default_theme(&mut self) {
        let default_theme = UITheme {
            name: "default".to_string(),
            description: "Default UI theme".to_string(),
            version: "1.0.0".to_string(),
            author: "Game Engine".to_string(),
            colors: ColorPalette {
                primary: ColorSet {
                    main: (0.2, 0.6, 1.0, 1.0),
                    light: (0.4, 0.7, 1.0, 1.0),
                    dark: (0.1, 0.4, 0.8, 1.0),
                    contrast: (1.0, 1.0, 1.0, 1.0),
                },
                secondary: ColorSet {
                    main: (0.6, 0.6, 0.6, 1.0),
                    light: (0.8, 0.8, 0.8, 1.0),
                    dark: (0.4, 0.4, 0.4, 1.0),
                    contrast: (0.0, 0.0, 0.0, 1.0),
                },
                accent: ColorSet {
                    main: (1.0, 0.6, 0.2, 1.0),
                    light: (1.0, 0.8, 0.4, 1.0),
                    dark: (0.8, 0.4, 0.0, 1.0),
                    contrast: (1.0, 1.0, 1.0, 1.0),
                },
                neutral: ColorSet {
                    main: (0.5, 0.5, 0.5, 1.0),
                    light: (0.7, 0.7, 0.7, 1.0),
                    dark: (0.3, 0.3, 0.3, 1.0),
                    contrast: (1.0, 1.0, 1.0, 1.0),
                },
                success: ColorSet {
                    main: (0.2, 0.8, 0.2, 1.0),
                    light: (0.4, 0.9, 0.4, 1.0),
                    dark: (0.1, 0.6, 0.1, 1.0),
                    contrast: (1.0, 1.0, 1.0, 1.0),
                },
                warning: ColorSet {
                    main: (1.0, 0.8, 0.2, 1.0),
                    light: (1.0, 0.9, 0.4, 1.0),
                    dark: (0.8, 0.6, 0.0, 1.0),
                    contrast: (0.0, 0.0, 0.0, 1.0),
                },
                error: ColorSet {
                    main: (0.8, 0.2, 0.2, 1.0),
                    light: (0.9, 0.4, 0.4, 1.0),
                    dark: (0.6, 0.1, 0.1, 1.0),
                    contrast: (1.0, 1.0, 1.0, 1.0),
                },
                info: ColorSet {
                    main: (0.2, 0.6, 0.8, 1.0),
                    light: (0.4, 0.7, 0.9, 1.0),
                    dark: (0.1, 0.4, 0.6, 1.0),
                    contrast: (1.0, 1.0, 1.0, 1.0),
                },
                background: BackgroundColors {
                    primary: (0.95, 0.95, 0.95, 1.0),
                    secondary: (0.9, 0.9, 0.9, 1.0),
                    tertiary: (0.85, 0.85, 0.85, 1.0),
                    overlay: (0.0, 0.0, 0.0, 0.5),
                    modal: (1.0, 1.0, 1.0, 0.95),
                    card: (1.0, 1.0, 1.0, 1.0),
                },
                text: TextColors {
                    primary: (0.1, 0.1, 0.1, 1.0),
                    secondary: (0.4, 0.4, 0.4, 1.0),
                    tertiary: (0.6, 0.6, 0.6, 1.0),
                    disabled: (0.8, 0.8, 0.8, 1.0),
                    link: (0.2, 0.6, 1.0, 1.0),
                    error: (0.8, 0.2, 0.2, 1.0),
                    success: (0.2, 0.8, 0.2, 1.0),
                    warning: (1.0, 0.8, 0.2, 1.0),
                },
                border: BorderColors {
                    primary: (0.8, 0.8, 0.8, 1.0),
                    secondary: (0.9, 0.9, 0.9, 1.0),
                    focus: (0.2, 0.6, 1.0, 1.0),
                    error: (0.8, 0.2, 0.2, 1.0),
                    success: (0.2, 0.8, 0.2, 1.0),
                    warning: (1.0, 0.8, 0.2, 1.0),
                },
            },
            typography: TypographySettings {
                font_families: FontFamilies {
                    primary: "Arial".to_string(),
                    secondary: "Helvetica".to_string(),
                    monospace: "Courier New".to_string(),
                    display: "Impact".to_string(),
                },
                font_sizes: FontSizes {
                    xs: 12.0,
                    sm: 14.0,
                    md: 16.0,
                    lg: 18.0,
                    xl: 20.0,
                    xxl: 24.0,
                    huge: 32.0,
                },
                font_weights: FontWeights {
                    light: 300,
                    regular: 400,
                    medium: 500,
                    semibold: 600,
                    bold: 700,
                    extrabold: 800,
                },
                line_heights: LineHeights {
                    tight: 1.2,
                    normal: 1.5,
                    relaxed: 1.75,
                    loose: 2.0,
                },
                letter_spacing: LetterSpacing {
                    tight: -0.5,
                    normal: 0.0,
                    wide: 0.5,
                    wider: 1.0,
                },
                text_styles: HashMap::new(),
            },
            spacing: SpacingSettings {
                scale: 1.0,
                values: SpacingValues {
                    xs: 4.0,
                    sm: 8.0,
                    md: 16.0,
                    lg: 24.0,
                    xl: 32.0,
                    xxl: 48.0,
                },
                components: HashMap::new(),
            },
            borders: BorderSettings {
                widths: BorderWidths {
                    none: 0.0,
                    thin: 1.0,
                    medium: 2.0,
                    thick: 4.0,
                },
                styles: BorderStyles {
                    solid: "solid".to_string(),
                    dashed: "dashed".to_string(),
                    dotted: "dotted".to_string(),
                    double: "double".to_string(),
                },
                radius: BorderRadius {
                    none: 0.0,
                    small: 4.0,
                    medium: 8.0,
                    large: 12.0,
                    extra_large: 16.0,
                    full: 999.0,
                },
            },
            shadows: ShadowSettings {
                shadows: HashMap::new(),
                default: "none".to_string(),
            },
            animations: AnimationSettings {
                durations: AnimationDurations {
                    fast: 0.15,
                    normal: 0.3,
                    slow: 0.5,
                    very_slow: 1.0,
                },
                easings: AnimationEasings {
                    default: "ease-in-out".to_string(),
                    ease_in: "ease-in".to_string(),
                    ease_out: "ease-out".to_string(),
                    ease_in_out: "ease-in-out".to_string(),
                },
                delays: AnimationDelays {
                    none: 0.0,
                    short: 0.1,
                    medium: 0.2,
                    long: 0.5,
                },
            },
            components: HashMap::new(),
        };

        self.themes.insert("default".to_string(), default_theme);
    }

    /// Emit UI event
    fn emit_event(&self, event: UIEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for UIThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
