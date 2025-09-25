//! UI Mirroring System for RTL Languages
//! 
//! This module provides UI mirroring functionality for right-to-left (RTL) languages
//! like Arabic, ensuring proper cultural presentation of UI elements.

use super::{Language, TextAlignment};
use glam::Vec2;

/// UI element types that need mirroring for RTL languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UIElementType {
    /// Button elements
    Button,
    /// Menu items
    MenuItem,
    /// Dialog boxes
    Dialog,
    /// Tooltips
    Tooltip,
    /// Progress bars
    ProgressBar,
    /// Sliders
    Slider,
    /// Checkboxes
    Checkbox,
    /// Radio buttons
    RadioButton,
    /// Text input fields
    TextInput,
    /// Dropdown menus
    Dropdown,
    /// Tab panels
    Tab,
    /// Status bars
    StatusBar,
    /// Navigation elements
    Navigation,
    /// Icon buttons
    IconButton,
    /// List items
    ListItem,
    /// Card elements
    Card,
    /// Panel elements
    Panel,
    /// Generic elements
    Generic,
}

/// UI mirroring configuration for different element types
#[derive(Debug, Clone)]
pub struct UIMirroringConfig {
    /// Whether to mirror the element horizontally
    pub mirror_horizontal: bool,
    /// Whether to mirror the element vertically
    pub mirror_vertical: bool,
    /// Text alignment for the element
    pub text_alignment: TextAlignment,
    /// Whether to reverse the order of child elements
    pub reverse_children: bool,
    /// Whether to flip icon direction
    pub flip_icon: bool,
    /// Whether to reverse text direction
    pub reverse_text: bool,
}

/// Statistics about UI mirroring
#[derive(Debug, Clone)]
pub struct UIMirroringStats {
    /// Total number of registered UI elements
    pub total_elements: usize,
    /// Number of elements that are mirrored
    pub mirrored_elements: usize,
    /// List of element types
    pub element_types: Vec<UIElementType>,
}

/// UI mirroring manager for handling RTL language support
pub struct UIMirroringManager {
    /// Current language
    current_language: Language,
    /// Mirroring configurations for different UI element types
    element_configs: std::collections::HashMap<UIElementType, UIMirroringConfig>,
}

impl UIMirroringManager {
    /// Create a new UI mirroring manager
    pub fn new() -> Self {
        let mut manager = Self {
            current_language: Language::English,
            element_configs: std::collections::HashMap::new(),
        };
        
        // Initialize default configurations
        manager.initialize_default_configs();
        manager
    }

    /// Set the current language
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    /// Get the current language
    pub fn current_language(&self) -> Language {
        self.current_language
    }

    /// Check if UI mirroring is required for the current language
    pub fn requires_mirroring(&self) -> bool {
        self.current_language.requires_ui_mirroring()
    }

    /// Get mirroring configuration for a UI element type
    pub fn get_mirroring_config(&self, element_type: UIElementType) -> UIMirroringConfig {
        if !self.requires_mirroring() {
            // Return default (no mirroring) configuration for LTR languages
            UIMirroringConfig {
                mirror_horizontal: false,
                mirror_vertical: false,
                text_alignment: TextAlignment::Left,
                reverse_children: false,
                flip_icon: false,
                reverse_text: false,
            }
        } else {
            // Return RTL-specific configuration
            self.element_configs
                .get(&element_type)
                .cloned()
                .unwrap_or_else(|| self.get_default_rtl_config())
        }
    }

    /// Apply mirroring transformation to a position
    pub fn mirror_position(&self, position: Vec2, element_bounds: (f32, f32), element_type: UIElementType) -> Vec2 {
        if !self.requires_mirroring() {
            return position;
        }

        let config = self.get_mirroring_config(element_type);
        let mut mirrored_position = position;

        if config.mirror_horizontal {
            mirrored_position.x = element_bounds.0 - position.x;
        }

        if config.mirror_vertical {
            mirrored_position.y = element_bounds.1 - position.y;
        }

        mirrored_position
    }

    /// Apply mirroring transformation to a size
    pub fn mirror_size(&self, size: Vec2, element_type: UIElementType) -> Vec2 {
        if !self.requires_mirroring() {
            return size;
        }

        let config = self.get_mirroring_config(element_type);
        let mut mirrored_size = size;

        if config.mirror_horizontal {
            mirrored_size.x = -size.x;
        }

        if config.mirror_vertical {
            mirrored_size.y = -size.y;
        }

        mirrored_size
    }

    /// Get text alignment for a UI element type
    pub fn get_text_alignment(&self, element_type: UIElementType) -> TextAlignment {
        if !self.requires_mirroring() {
            return TextAlignment::Left;
        }

        self.get_mirroring_config(element_type).text_alignment
    }

    /// Check if an element should have its children reversed
    pub fn should_reverse_children(&self, element_type: UIElementType) -> bool {
        if !self.requires_mirroring() {
            return false;
        }

        self.get_mirroring_config(element_type).reverse_children
    }

    /// Check if an icon should be flipped
    pub fn should_flip_icon(&self, element_type: UIElementType) -> bool {
        if !self.requires_mirroring() {
            return false;
        }

        self.get_mirroring_config(element_type).flip_icon
    }

    /// Check if text should be reversed
    pub fn should_reverse_text(&self, element_type: UIElementType) -> bool {
        if !self.requires_mirroring() {
            return false;
        }

        self.get_mirroring_config(element_type).reverse_text
    }

    /// Initialize default mirroring configurations for RTL languages
    fn initialize_default_configs(&mut self) {
        // Button elements - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::Button, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: false,
            flip_icon: true,
            reverse_text: false,
        });

        // Menu items - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::MenuItem, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: true,
            flip_icon: true,
            reverse_text: false,
        });

        // Dialog boxes - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::Dialog, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: true,
            flip_icon: true,
            reverse_text: false,
        });

        // Tooltips - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::Tooltip, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: false,
            flip_icon: true,
            reverse_text: false,
        });

        // Progress bars - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::ProgressBar, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: false,
            flip_icon: false,
            reverse_text: false,
        });

        // Sliders - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::Slider, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: false,
            flip_icon: false,
            reverse_text: false,
        });

        // Checkboxes - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::Checkbox, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: false,
            flip_icon: false,
            reverse_text: false,
        });

        // Radio buttons - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::RadioButton, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: false,
            flip_icon: false,
            reverse_text: false,
        });

        // Text input fields - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::TextInput, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: false,
            flip_icon: false,
            reverse_text: true,
        });

        // Dropdown menus - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::Dropdown, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: true,
            flip_icon: true,
            reverse_text: false,
        });

        // Tab panels - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::Tab, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: true,
            flip_icon: false,
            reverse_text: false,
        });

        // Status bars - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::StatusBar, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: true,
            flip_icon: true,
            reverse_text: false,
        });

        // Navigation elements - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::Navigation, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: true,
            flip_icon: true,
            reverse_text: false,
        });

        // Icon buttons - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::IconButton, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: false,
            flip_icon: true,
            reverse_text: false,
        });

        // List items - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::ListItem, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: false,
            flip_icon: true,
            reverse_text: false,
        });

        // Card elements - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::Card, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: true,
            flip_icon: true,
            reverse_text: false,
        });

        // Panel elements - mirror horizontally, right-align text
        self.element_configs.insert(UIElementType::Panel, UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: true,
            flip_icon: true,
            reverse_text: false,
        });
    }

    /// Get default RTL configuration
    fn get_default_rtl_config(&self) -> UIMirroringConfig {
        UIMirroringConfig {
            mirror_horizontal: true,
            mirror_vertical: false,
            text_alignment: TextAlignment::Right,
            reverse_children: false,
            flip_icon: true,
            reverse_text: false,
        }
    }

    /// Get element type for a given element name
    pub fn get_element_type(&self, element_name: &str) -> UIElementType {
        // For now, return Generic - in a real implementation, this would look up
        // the element in a registry or infer from the name
        UIElementType::Generic
    }

    /// Check if an element should be mirrored
    pub fn should_mirror_element(&self, element_name: &str) -> bool {
        if !self.requires_mirroring() {
            return false;
        }
        
        // For now, mirror all elements in RTL languages
        // In a real implementation, this would check specific element configurations
        true
    }

    /// Register a UI element
    pub fn register_element(&mut self, element_name: &str, element_type: UIElementType) {
        // For now, this is a no-op as we don't have a registry
        // In a real implementation, this would store element metadata
    }

    /// Get statistics about UI mirroring
    pub fn get_stats(&self) -> UIMirroringStats {
        UIMirroringStats {
            total_elements: 0, // Would be tracked in a real implementation
            mirrored_elements: 0,
            element_types: vec![],
        }
    }
}

impl Default for UIMirroringManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for UI mirroring
pub mod helpers {
    use super::*;

    /// Apply RTL text direction to a string
    pub fn apply_rtl_text_direction(text: &str) -> String {
        // For Arabic text, we might need to apply Unicode bidirectional algorithm
        // This is a simplified version - in production, you'd use a proper RTL text library
        if text.chars().any(|c| c.is_ascii_alphabetic() || c.is_numeric()) {
            // Mixed content - might need special handling
            text.to_string()
        } else {
            // Pure Arabic text - should be handled by the font renderer
            text.to_string()
        }
    }

    /// Check if a string contains RTL characters
    pub fn contains_rtl_characters(text: &str) -> bool {
        text.chars().any(|c| {
            // Check for Arabic characters (U+0600-U+06FF)
            matches!(c as u32, 0x0600..=0x06FF) ||
            // Check for other RTL scripts
            matches!(c as u32, 0x0590..=0x05FF) || // Hebrew
            matches!(c as u32, 0x0700..=0x074F) || // Syriac
            matches!(c as u32, 0x0750..=0x077F) || // Arabic Supplement
            matches!(c as u32, 0x0780..=0x07BF) || // Thaana
            matches!(c as u32, 0x07C0..=0x07FF) || // NKo
            matches!(c as u32, 0x0800..=0x083F) || // Samaritan
            matches!(c as u32, 0x0840..=0x085F) || // Mandaic
            matches!(c as u32, 0x0860..=0x086F) || // Syriac Supplement
            matches!(c as u32, 0x08A0..=0x08FF) || // Arabic Extended-A
            matches!(c as u32, 0xFB1D..=0xFDFF) || // Arabic Presentation Forms-A
            matches!(c as u32, 0xFE70..=0xFEFF) || // Arabic Presentation Forms-B
            matches!(c as u32, 0x1EE00..=0x1EEFF) // Arabic Mathematical Alphabetic Symbols
        })
    }

    /// Get the dominant text direction of a string
    pub fn get_text_direction(text: &str) -> TextAlignment {
        if contains_rtl_characters(text) {
            TextAlignment::Right
        } else {
            TextAlignment::Left
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtl_detection() {
        let manager = UIMirroringManager::new();
        
        // Test with English (LTR)
        let mut manager = manager;
        manager.set_language(Language::English);
        assert!(!manager.requires_mirroring());
        
        // Test with Arabic (RTL)
        manager.set_language(Language::Arabic);
        assert!(manager.requires_mirroring());
    }

    #[test]
    fn test_mirroring_config() {
        let mut manager = UIMirroringManager::new();
        manager.set_language(Language::Arabic);
        
        let config = manager.get_mirroring_config(UIElementType::Button);
        assert!(config.mirror_horizontal);
        assert_eq!(config.text_alignment, TextAlignment::Right);
        assert!(config.flip_icon);
    }

    #[test]
    fn test_position_mirroring() {
        let mut manager = UIMirroringManager::new();
        manager.set_language(Language::Arabic);
        
        let position = Vec2::new(100.0, 50.0);
        let bounds = (200.0, 100.0);
        let mirrored = manager.mirror_position(position, bounds, UIElementType::Button);
        
        // Should be mirrored horizontally
        assert_eq!(mirrored.x, 100.0); // 200 - 100
        assert_eq!(mirrored.y, 50.0); // No vertical mirroring
    }

    #[test]
    fn test_rtl_character_detection() {
        assert!(helpers::contains_rtl_characters("مرحبا")); // Arabic
        assert!(helpers::contains_rtl_characters("שלום")); // Hebrew
        assert!(!helpers::contains_rtl_characters("Hello")); // English
        assert!(!helpers::contains_rtl_characters("123")); // Numbers
    }

    #[test]
    fn test_text_direction() {
        assert_eq!(helpers::get_text_direction("مرحبا"), TextAlignment::Right);
        assert_eq!(helpers::get_text_direction("Hello"), TextAlignment::Left);
        assert_eq!(helpers::get_text_direction("123"), TextAlignment::Left);
    }
}
