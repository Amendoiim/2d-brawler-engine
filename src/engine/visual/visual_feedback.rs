//! Visual feedback system
//! 
//! This module provides comprehensive visual feedback for user interactions:
//! - UI feedback (hover, click, focus states)
//! - Gameplay feedback (damage numbers, status effects, notifications)
//! - Character feedback (health bars, mana bars, buffs/debuffs)
//! - Environmental feedback (interactive objects, collectibles)

use std::collections::HashMap;

/// Visual feedback manager
#[derive(Default)]
pub struct VisualFeedbackManager {
    /// Active feedback elements
    pub elements: HashMap<String, VisualFeedbackElement>,
    /// UI feedback system
    pub ui_feedback: UIFeedbackSystem,
    /// Gameplay feedback system
    pub gameplay_feedback: GameplayFeedbackSystem,
    /// Character feedback system
    pub character_feedback: CharacterFeedbackSystem,
    /// Environmental feedback system
    pub environmental_feedback: EnvironmentalFeedbackSystem,
    /// Performance settings
    pub performance: VisualFeedbackPerformanceSettings,
    /// Metrics
    pub metrics: VisualFeedbackMetrics,
}

/// Visual feedback element types
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum VisualFeedbackElementType {
    // UI feedback
    UIHover,
    UIClick,
    UIFocus,
    UISelect,
    UIDisable,
    UIEnable,
    
    // Gameplay feedback
    DamageNumber,
    HealNumber,
    CriticalHit,
    StatusEffect,
    Notification,
    Achievement,
    QuestUpdate,
    LevelUp,
    
    // Character feedback
    HealthBar,
    ManaBar,
    StaminaBar,
    ExperienceBar,
    BuffIcon,
    DebuffIcon,
    ShieldIcon,
    AuraIcon,
    
    // Environmental feedback
    InteractiveObject,
    Collectible,
    Door,
    Chest,
    Switch,
    Lever,
    Button,
    PressurePlate,
}

/// Individual visual feedback element
#[derive(Clone)]
pub struct VisualFeedbackElement {
    /// Element ID
    pub id: String,
    /// Element type
    pub element_type: VisualFeedbackElementType,
    /// Element enabled
    pub enabled: bool,
    /// Element position
    pub position: Vec3,
    /// Element size
    pub size: Vec2,
    /// Element color
    pub color: Color,
    /// Element alpha
    pub alpha: f32,
    /// Element duration
    pub duration: f32,
    /// Element age
    pub age: f32,
    /// Element parameters
    pub parameters: VisualFeedbackParameters,
    /// Element performance settings
    pub performance: VisualFeedbackElementPerformanceSettings,
}

/// Visual feedback parameters
#[derive(Clone)]
pub struct VisualFeedbackParameters {
    // UI feedback parameters
    pub ui_scale: f32,
    pub ui_glow_intensity: f32,
    pub ui_pulse_speed: f32,
    pub ui_fade_speed: f32,
    
    // Damage number parameters
    pub damage_number_size: f32,
    pub damage_number_color: Color,
    pub damage_number_duration: f32,
    pub damage_number_speed: f32,
    pub damage_number_direction: Vec2,
    
    // Status effect parameters
    pub status_effect_icon_size: f32,
    pub status_effect_duration: f32,
    pub status_effect_pulse: bool,
    pub status_effect_glow: bool,
    
    // Health bar parameters
    pub health_bar_width: f32,
    pub health_bar_height: f32,
    pub health_bar_background_color: Color,
    pub health_bar_foreground_color: Color,
    pub health_bar_border_color: Color,
    pub health_bar_show_text: bool,
    pub health_bar_show_percentage: bool,
    
    // Interactive object parameters
    pub interactive_object_glow: bool,
    pub interactive_object_glow_color: Color,
    pub interactive_object_glow_intensity: f32,
    pub interactive_object_pulse: bool,
    pub interactive_object_pulse_speed: f32,
}

/// UI feedback system
#[derive(Clone)]
pub struct UIFeedbackSystem {
    /// Hover effects
    pub hover_effects: HashMap<String, UIHoverEffect>,
    /// Click effects
    pub click_effects: HashMap<String, UIClickEffect>,
    /// Focus effects
    pub focus_effects: HashMap<String, UIFocusEffect>,
    /// Performance settings
    pub performance: UIFeedbackPerformanceSettings,
}

/// UI hover effect
#[derive(Clone)]
pub struct UIHoverEffect {
    /// Effect ID
    pub id: String,
    /// Effect enabled
    pub enabled: bool,
    /// Effect scale
    pub scale: f32,
    /// Effect color
    pub color: Color,
    /// Effect glow
    pub glow: bool,
    /// Effect pulse
    pub pulse: bool,
    /// Effect duration
    pub duration: f32,
}

/// UI click effect
#[derive(Clone)]
pub struct UIClickEffect {
    /// Effect ID
    pub id: String,
    /// Effect enabled
    pub enabled: bool,
    /// Effect scale
    pub scale: f32,
    /// Effect color
    pub color: Color,
    /// Effect ripple
    pub ripple: bool,
    /// Effect duration
    pub duration: f32,
}

/// UI focus effect
#[derive(Clone)]
pub struct UIFocusEffect {
    /// Effect ID
    pub id: String,
    /// Effect enabled
    pub enabled: bool,
    /// Effect border
    pub border: bool,
    /// Effect border color
    pub border_color: Color,
    /// Effect border width
    pub border_width: f32,
    /// Effect glow
    pub glow: bool,
    /// Effect duration
    pub duration: f32,
}

/// Gameplay feedback system
#[derive(Clone)]
pub struct GameplayFeedbackSystem {
    /// Damage numbers
    pub damage_numbers: HashMap<String, DamageNumber>,
    /// Status effects
    pub status_effects: HashMap<String, StatusEffect>,
    /// Notifications
    pub notifications: HashMap<String, Notification>,
    /// Performance settings
    pub performance: GameplayFeedbackPerformanceSettings,
}

/// Damage number
#[derive(Clone)]
pub struct DamageNumber {
    /// Number ID
    pub id: String,
    /// Number value
    pub value: f32,
    /// Number position
    pub position: Vec3,
    /// Number color
    pub color: Color,
    /// Number size
    pub size: f32,
    /// Number duration
    pub duration: f32,
    /// Number age
    pub age: f32,
    /// Number velocity
    pub velocity: Vec2,
    /// Number gravity
    pub gravity: f32,
    /// Number enabled
    pub enabled: bool,
}

/// Status effect
#[derive(Clone)]
pub struct StatusEffect {
    /// Effect ID
    pub effect_id: String,
    /// Effect type
    pub effect_type: StatusEffectType,
    /// Effect position
    pub position: Vec3,
    /// Effect icon
    pub icon: String,
    /// Effect color
    pub color: Color,
    /// Effect duration
    pub duration: f32,
    /// Effect age
    pub age: f32,
    /// Effect stack count
    pub stack_count: u32,
    /// Effect enabled
    pub enabled: bool,
}

/// Status effect types
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum StatusEffectType {
    Buff,
    Debuff,
    Shield,
    Aura,
    DoT,
    HoT,
    Stun,
    Slow,
    Haste,
    Invisible,
    Invulnerable,
    Regeneration,
    Poison,
    Burn,
    Freeze,
    Shock,
    Curse,
    Blessing,
}

/// Notification
#[derive(Clone)]
pub struct Notification {
    /// Notification ID
    pub id: String,
    /// Notification text
    pub text: String,
    /// Notification position
    pub position: Vec3,
    /// Notification color
    pub color: Color,
    /// Notification background color
    pub background_color: Color,
    /// Notification duration
    pub duration: f32,
    /// Notification age
    pub age: f32,
    /// Notification enabled
    pub enabled: bool,
}

/// Character feedback system
#[derive(Clone)]
pub struct CharacterFeedbackSystem {
    /// Health bars
    pub health_bars: HashMap<String, HealthBar>,
    /// Mana bars
    pub mana_bars: HashMap<String, ManaBar>,
    /// Stamina bars
    pub stamina_bars: HashMap<String, StaminaBar>,
    /// Experience bars
    pub experience_bars: HashMap<String, ExperienceBar>,
    /// Buff icons
    pub buff_icons: HashMap<String, BuffIcon>,
    /// Debuff icons
    pub debuff_icons: HashMap<String, DebuffIcon>,
    /// Performance settings
    pub performance: CharacterFeedbackPerformanceSettings,
}

/// Health bar
#[derive(Clone)]
pub struct HealthBar {
    /// Bar ID
    pub id: String,
    /// Bar position
    pub position: Vec3,
    /// Bar size
    pub size: Vec2,
    /// Current health
    pub current_health: f32,
    /// Maximum health
    pub maximum_health: f32,
    /// Bar color
    pub color: Color,
    /// Bar background color
    pub background_color: Color,
    /// Bar border color
    pub border_color: Color,
    /// Bar enabled
    pub enabled: bool,
}

/// Mana bar
#[derive(Clone)]
pub struct ManaBar {
    /// Bar ID
    pub id: String,
    /// Bar position
    pub position: Vec3,
    /// Bar size
    pub size: Vec2,
    /// Current mana
    pub current_mana: f32,
    /// Maximum mana
    pub maximum_mana: f32,
    /// Bar color
    pub color: Color,
    /// Bar background color
    pub background_color: Color,
    /// Bar border color
    pub border_color: Color,
    /// Bar enabled
    pub enabled: bool,
}

/// Stamina bar
#[derive(Clone)]
pub struct StaminaBar {
    /// Bar ID
    pub id: String,
    /// Bar position
    pub position: Vec3,
    /// Bar size
    pub size: Vec2,
    /// Current stamina
    pub current_stamina: f32,
    /// Maximum stamina
    pub maximum_stamina: f32,
    /// Bar color
    pub color: Color,
    /// Bar background color
    pub background_color: Color,
    /// Bar border color
    pub border_color: Color,
    /// Bar enabled
    pub enabled: bool,
}

/// Experience bar
#[derive(Clone)]
pub struct ExperienceBar {
    /// Bar ID
    pub id: String,
    /// Bar position
    pub position: Vec3,
    /// Bar size
    pub size: Vec2,
    /// Current experience
    pub current_experience: f32,
    /// Maximum experience
    pub maximum_experience: f32,
    /// Bar color
    pub color: Color,
    /// Bar background color
    pub background_color: Color,
    /// Bar border color
    pub border_color: Color,
    /// Bar enabled
    pub enabled: bool,
}

/// Buff icon
#[derive(Clone)]
pub struct BuffIcon {
    /// Icon ID
    pub id: String,
    /// Icon position
    pub position: Vec3,
    /// Icon size
    pub size: Vec2,
    /// Icon texture
    pub texture: String,
    /// Icon color
    pub color: Color,
    /// Icon duration
    pub duration: f32,
    /// Icon age
    pub age: f32,
    /// Icon enabled
    pub enabled: bool,
}

/// Debuff icon
#[derive(Clone)]
pub struct DebuffIcon {
    /// Icon ID
    pub id: String,
    /// Icon position
    pub position: Vec3,
    /// Icon size
    pub size: Vec2,
    /// Icon texture
    pub texture: String,
    /// Icon color
    pub color: Color,
    /// Icon duration
    pub duration: f32,
    /// Icon age
    pub age: f32,
    /// Icon enabled
    pub enabled: bool,
}

/// Environmental feedback system
#[derive(Clone)]
pub struct EnvironmentalFeedbackSystem {
    /// Interactive objects
    pub interactive_objects: HashMap<String, InteractiveObject>,
    /// Collectibles
    pub collectibles: HashMap<String, Collectible>,
    /// Doors
    pub doors: HashMap<String, Door>,
    /// Chests
    pub chests: HashMap<String, Chest>,
    /// Switches
    pub switches: HashMap<String, Switch>,
    /// Performance settings
    pub performance: EnvironmentalFeedbackPerformanceSettings,
}

/// Interactive object
#[derive(Clone)]
pub struct InteractiveObject {
    /// Object ID
    pub id: String,
    /// Object position
    pub position: Vec3,
    /// Object size
    pub size: Vec2,
    /// Object type
    pub object_type: InteractiveObjectType,
    /// Object glow
    pub glow: bool,
    /// Object glow color
    pub glow_color: Color,
    /// Object glow intensity
    pub glow_intensity: f32,
    /// Object pulse
    pub pulse: bool,
    /// Object pulse speed
    pub pulse_speed: f32,
    /// Object enabled
    pub enabled: bool,
}

/// Interactive object types
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum InteractiveObjectType {
    Door,
    Chest,
    Switch,
    Lever,
    Button,
    PressurePlate,
    NPC,
    Shop,
    QuestGiver,
    Teleporter,
    SavePoint,
    Checkpoint,
}

/// Collectible
#[derive(Clone)]
pub struct Collectible {
    /// Collectible ID
    pub id: String,
    /// Collectible position
    pub position: Vec3,
    /// Collectible size
    pub size: Vec2,
    /// Collectible type
    pub collectible_type: CollectibleType,
    /// Collectible glow
    pub glow: bool,
    /// Collectible glow color
    pub glow_color: Color,
    /// Collectible pulse
    pub pulse: bool,
    /// Collectible pulse speed
    pub pulse_speed: f32,
    /// Collectible enabled
    pub enabled: bool,
}

/// Collectible types
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CollectibleType {
    Coin,
    Gem,
    Key,
    Potion,
    Scroll,
    Book,
    Artifact,
    Relic,
    Trophy,
    Achievement,
}

/// Door
#[derive(Clone)]
pub struct Door {
    /// Door ID
    pub id: String,
    /// Door position
    pub position: Vec3,
    /// Door size
    pub size: Vec2,
    /// Door open
    pub open: bool,
    /// Door locked
    pub locked: bool,
    /// Door glow
    pub glow: bool,
    /// Door glow color
    pub glow_color: Color,
    /// Door enabled
    pub enabled: bool,
}

/// Chest
#[derive(Clone)]
pub struct Chest {
    /// Chest ID
    pub id: String,
    /// Chest position
    pub position: Vec3,
    /// Chest size
    pub size: Vec2,
    /// Chest open
    pub open: bool,
    /// Chest locked
    pub locked: bool,
    /// Chest glow
    pub glow: bool,
    /// Chest glow color
    pub glow_color: Color,
    /// Chest enabled
    pub enabled: bool,
}

/// Switch
#[derive(Clone)]
pub struct Switch {
    /// Switch ID
    pub id: String,
    /// Switch position
    pub position: Vec3,
    /// Switch size
    pub size: Vec2,
    /// Switch active
    pub active: bool,
    /// Switch glow
    pub glow: bool,
    /// Switch glow color
    pub glow_color: Color,
    /// Switch enabled
    pub enabled: bool,
}

/// Performance settings for visual feedback
#[derive(Clone)]
pub struct VisualFeedbackPerformanceSettings {
    /// Feedback enabled
    pub enabled: bool,
    /// Update frequency
    pub update_frequency: f32,
    /// Last update
    pub last_update: f32,
    /// Max elements
    pub max_elements: usize,
    /// Max damage numbers
    pub max_damage_numbers: usize,
    /// Max status effects
    pub max_status_effects: usize,
    /// Max notifications
    pub max_notifications: usize,
}

/// UI feedback performance settings
#[derive(Clone)]
pub struct UIFeedbackPerformanceSettings {
    /// UI feedback enabled
    pub enabled: bool,
    /// Update frequency
    pub update_frequency: f32,
    /// Last update
    pub last_update: f32,
    /// Max hover effects
    pub max_hover_effects: usize,
    /// Max click effects
    pub max_click_effects: usize,
    /// Max focus effects
    pub max_focus_effects: usize,
}

/// Gameplay feedback performance settings
#[derive(Clone)]
pub struct GameplayFeedbackPerformanceSettings {
    /// Gameplay feedback enabled
    pub enabled: bool,
    /// Update frequency
    pub update_frequency: f32,
    /// Last update
    pub last_update: f32,
    /// Max damage numbers
    pub max_damage_numbers: usize,
    /// Max status effects
    pub max_status_effects: usize,
    /// Max notifications
    pub max_notifications: usize,
}

/// Character feedback performance settings
#[derive(Clone)]
pub struct CharacterFeedbackPerformanceSettings {
    /// Character feedback enabled
    pub enabled: bool,
    /// Update frequency
    pub update_frequency: f32,
    /// Last update
    pub last_update: f32,
    /// Max health bars
    pub max_health_bars: usize,
    /// Max mana bars
    pub max_mana_bars: usize,
    /// Max stamina bars
    pub max_stamina_bars: usize,
    /// Max experience bars
    pub max_experience_bars: usize,
    /// Max buff icons
    pub max_buff_icons: usize,
    /// Max debuff icons
    pub max_debuff_icons: usize,
}

/// Environmental feedback performance settings
#[derive(Clone)]
pub struct EnvironmentalFeedbackPerformanceSettings {
    /// Environmental feedback enabled
    pub enabled: bool,
    /// Update frequency
    pub update_frequency: f32,
    /// Last update
    pub last_update: f32,
    /// Max interactive objects
    pub max_interactive_objects: usize,
    /// Max collectibles
    pub max_collectibles: usize,
    /// Max doors
    pub max_doors: usize,
    /// Max chests
    pub max_chests: usize,
    /// Max switches
    pub max_switches: usize,
}

/// Visual feedback element performance settings
#[derive(Clone)]
pub struct VisualFeedbackElementPerformanceSettings {
    /// Element enabled
    pub enabled: bool,
    /// Update frequency
    pub update_frequency: f32,
    /// Last update
    pub last_update: f32,
    /// Quality level
    pub quality: crate::engine::visual::QualityLevel,
}

/// Visual feedback metrics
#[derive(Clone, Default)]
pub struct VisualFeedbackMetrics {
    /// Active elements
    pub active_elements: usize,
    /// Active damage numbers
    pub active_damage_numbers: usize,
    /// Active status effects
    pub active_status_effects: usize,
    /// Active notifications
    pub active_notifications: usize,
    /// Update time
    pub update_time: f32,
    /// Memory usage
    pub memory_usage: usize,
}

/// Simple 2D vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Simple 3D vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Simple color
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const DARK_GRAY: Color = Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 };
}

impl VisualFeedbackManager {
    /// Create a new visual feedback manager
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            ui_feedback: UIFeedbackSystem::default(),
            gameplay_feedback: GameplayFeedbackSystem::default(),
            character_feedback: CharacterFeedbackSystem::default(),
            environmental_feedback: EnvironmentalFeedbackSystem::default(),
            performance: VisualFeedbackPerformanceSettings::default(),
            metrics: VisualFeedbackMetrics::default(),
        }
    }

    /// Add a visual feedback element
    pub fn add_element(&mut self, element: VisualFeedbackElement) -> Result<(), String> {
        if self.elements.contains_key(&element.id) {
            return Err(format!("Visual feedback element '{}' already exists", element.id));
        }

        self.elements.insert(element.id.clone(), element);
        Ok(())
    }

    /// Remove a visual feedback element
    pub fn remove_element(&mut self, id: &str) -> bool {
        self.elements.remove(id).is_some()
    }

    /// Update visual feedback system
    pub fn update(&mut self, delta_time: f32) {
        // Update elements
        for element in self.elements.values_mut() {
            element.update(delta_time);
        }
        
        // Update subsystems
        self.ui_feedback.update(delta_time);
        self.gameplay_feedback.update(delta_time);
        self.character_feedback.update(delta_time);
        self.environmental_feedback.update(delta_time);
        
        self.update_metrics();
    }

    /// Update performance metrics
    fn update_metrics(&mut self) {
        self.metrics.active_elements = self.elements.values().filter(|e| e.enabled).count();
        self.metrics.active_damage_numbers = self.gameplay_feedback.damage_numbers.values().filter(|d| d.enabled).count();
        self.metrics.active_status_effects = self.gameplay_feedback.status_effects.values().filter(|s| s.enabled).count();
        self.metrics.active_notifications = self.gameplay_feedback.notifications.values().filter(|n| n.enabled).count();
    }

    /// Create damage number
    pub fn create_damage_number(&mut self, id: String, value: f32, position: Vec3, color: Color) -> Result<(), String> {
        let damage_number = DamageNumber {
            id: id.clone(),
            value,
            position,
            color,
            size: 1.0,
            duration: 2.0,
            age: 0.0,
            velocity: Vec2::new(0.0, 1.0),
            gravity: -9.81,
            enabled: true,
        };
        
        self.gameplay_feedback.damage_numbers.insert(id, damage_number);
        Ok(())
    }

    /// Create status effect
    pub fn create_status_effect(&mut self, id: String, effect_type: StatusEffectType, position: Vec3, duration: f32) -> Result<(), String> {
        let status_effect = StatusEffect {
            effect_id: id.clone(),
            effect_type,
            position,
            icon: format!("{:?}", effect_type),
            color: Color::WHITE,
            duration,
            age: 0.0,
            stack_count: 1,
            enabled: true,
        };
        
        self.gameplay_feedback.status_effects.insert(id, status_effect);
        Ok(())
    }

    /// Create notification
    pub fn create_notification(&mut self, id: String, text: String, position: Vec3, duration: f32) -> Result<(), String> {
        let notification = Notification {
            id: id.clone(),
            text,
            position,
            color: Color::WHITE,
            background_color: Color::BLACK,
            duration,
            age: 0.0,
            enabled: true,
        };
        
        self.gameplay_feedback.notifications.insert(id, notification);
        Ok(())
    }
}

impl Default for VisualFeedbackParameters {
    fn default() -> Self {
        Self {
            ui_scale: 1.0,
            ui_glow_intensity: 1.0,
            ui_pulse_speed: 1.0,
            ui_fade_speed: 1.0,
            damage_number_size: 1.0,
            damage_number_color: Color::RED,
            damage_number_duration: 2.0,
            damage_number_speed: 1.0,
            damage_number_direction: Vec2::new(0.0, 1.0),
            status_effect_icon_size: 1.0,
            status_effect_duration: 10.0,
            status_effect_pulse: true,
            status_effect_glow: true,
            health_bar_width: 100.0,
            health_bar_height: 10.0,
            health_bar_background_color: Color::DARK_GRAY,
            health_bar_foreground_color: Color::RED,
            health_bar_border_color: Color::WHITE,
            health_bar_show_text: true,
            health_bar_show_percentage: true,
            interactive_object_glow: true,
            interactive_object_glow_color: Color::YELLOW,
            interactive_object_glow_intensity: 1.0,
            interactive_object_pulse: true,
            interactive_object_pulse_speed: 1.0,
        }
    }
}

impl Default for UIFeedbackSystem {
    fn default() -> Self {
        Self {
            hover_effects: HashMap::new(),
            click_effects: HashMap::new(),
            focus_effects: HashMap::new(),
            performance: UIFeedbackPerformanceSettings::default(),
        }
    }
}

impl Default for GameplayFeedbackSystem {
    fn default() -> Self {
        Self {
            damage_numbers: HashMap::new(),
            status_effects: HashMap::new(),
            notifications: HashMap::new(),
            performance: GameplayFeedbackPerformanceSettings::default(),
        }
    }
}

impl Default for CharacterFeedbackSystem {
    fn default() -> Self {
        Self {
            health_bars: HashMap::new(),
            mana_bars: HashMap::new(),
            stamina_bars: HashMap::new(),
            experience_bars: HashMap::new(),
            buff_icons: HashMap::new(),
            debuff_icons: HashMap::new(),
            performance: CharacterFeedbackPerformanceSettings::default(),
        }
    }
}

impl Default for EnvironmentalFeedbackSystem {
    fn default() -> Self {
        Self {
            interactive_objects: HashMap::new(),
            collectibles: HashMap::new(),
            doors: HashMap::new(),
            chests: HashMap::new(),
            switches: HashMap::new(),
            performance: EnvironmentalFeedbackPerformanceSettings::default(),
        }
    }
}

impl Default for VisualFeedbackPerformanceSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            update_frequency: 60.0,
            last_update: 0.0,
            max_elements: 100,
            max_damage_numbers: 50,
            max_status_effects: 20,
            max_notifications: 10,
        }
    }
}

impl Default for UIFeedbackPerformanceSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            update_frequency: 60.0,
            last_update: 0.0,
            max_hover_effects: 10,
            max_click_effects: 5,
            max_focus_effects: 5,
        }
    }
}

impl Default for GameplayFeedbackPerformanceSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            update_frequency: 60.0,
            last_update: 0.0,
            max_damage_numbers: 50,
            max_status_effects: 20,
            max_notifications: 10,
        }
    }
}

impl Default for CharacterFeedbackPerformanceSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            update_frequency: 60.0,
            last_update: 0.0,
            max_health_bars: 10,
            max_mana_bars: 10,
            max_stamina_bars: 10,
            max_experience_bars: 10,
            max_buff_icons: 20,
            max_debuff_icons: 20,
        }
    }
}

impl Default for EnvironmentalFeedbackPerformanceSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            update_frequency: 60.0,
            last_update: 0.0,
            max_interactive_objects: 50,
            max_collectibles: 100,
            max_doors: 20,
            max_chests: 20,
            max_switches: 20,
        }
    }
}

impl Default for VisualFeedbackElementPerformanceSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            update_frequency: 60.0,
            last_update: 0.0,
            quality: crate::engine::visual::QualityLevel::High,
        }
    }
}

impl VisualFeedbackElement {
    /// Update element
    pub fn update(&mut self, delta_time: f32) {
        if !self.enabled {
            return;
        }

        self.age += delta_time;
        
        if self.age >= self.duration {
            self.enabled = false;
            return;
        }

        self.performance.last_update += delta_time;
        
        if self.performance.last_update >= self.performance.update_frequency {
            self.update_element();
            self.performance.last_update = 0.0;
        }
    }

    /// Update element based on type
    fn update_element(&mut self) {
        match self.element_type {
            VisualFeedbackElementType::UIHover => {
                // Update UI hover effect
            }
            VisualFeedbackElementType::DamageNumber => {
                // Update damage number
            }
            _ => {
                // Update other elements
            }
        }
    }
}

impl UIFeedbackSystem {
    /// Update UI feedback system
    pub fn update(&mut self, delta_time: f32) {
        // Update hover effects
        for effect in self.hover_effects.values_mut() {
            effect.update(delta_time);
        }
        
        // Update click effects
        for effect in self.click_effects.values_mut() {
            effect.update(delta_time);
        }
        
        // Update focus effects
        for effect in self.focus_effects.values_mut() {
            effect.update(delta_time);
        }
    }
}

impl GameplayFeedbackSystem {
    /// Update gameplay feedback system
    pub fn update(&mut self, delta_time: f32) {
        // Update damage numbers
        for damage_number in self.damage_numbers.values_mut() {
            damage_number.update(delta_time);
        }
        
        // Update status effects
        for status_effect in self.status_effects.values_mut() {
            status_effect.update(delta_time);
        }
        
        // Update notifications
        for notification in self.notifications.values_mut() {
            notification.update(delta_time);
        }
    }
}

impl CharacterFeedbackSystem {
    /// Update character feedback system
    pub fn update(&mut self, delta_time: f32) {
        // Update health bars
        for health_bar in self.health_bars.values_mut() {
            health_bar.update(delta_time);
        }
        
        // Update mana bars
        for mana_bar in self.mana_bars.values_mut() {
            mana_bar.update(delta_time);
        }
        
        // Update stamina bars
        for stamina_bar in self.stamina_bars.values_mut() {
            stamina_bar.update(delta_time);
        }
        
        // Update experience bars
        for experience_bar in self.experience_bars.values_mut() {
            experience_bar.update(delta_time);
        }
        
        // Update buff icons
        for buff_icon in self.buff_icons.values_mut() {
            buff_icon.update(delta_time);
        }
        
        // Update debuff icons
        for debuff_icon in self.debuff_icons.values_mut() {
            debuff_icon.update(delta_time);
        }
    }
}

impl EnvironmentalFeedbackSystem {
    /// Update environmental feedback system
    pub fn update(&mut self, delta_time: f32) {
        // Update interactive objects
        for interactive_object in self.interactive_objects.values_mut() {
            interactive_object.update(delta_time);
        }
        
        // Update collectibles
        for collectible in self.collectibles.values_mut() {
            collectible.update(delta_time);
        }
        
        // Update doors
        for door in self.doors.values_mut() {
            door.update(delta_time);
        }
        
        // Update chests
        for chest in self.chests.values_mut() {
            chest.update(delta_time);
        }
        
        // Update switches
        for switch in self.switches.values_mut() {
            switch.update(delta_time);
        }
    }
}

// Implement update methods for all feedback types
impl UIHoverEffect {
    pub fn update(&mut self, delta_time: f32) {
        // Update hover effect
    }
}

impl UIClickEffect {
    pub fn update(&mut self, delta_time: f32) {
        // Update click effect
    }
}

impl UIFocusEffect {
    pub fn update(&mut self, delta_time: f32) {
        // Update focus effect
    }
}

impl DamageNumber {
    pub fn update(&mut self, delta_time: f32) {
        if !self.enabled {
            return;
        }

        self.age += delta_time;
        
        if self.age >= self.duration {
            self.enabled = false;
            return;
        }

        // Update position based on velocity and gravity
        self.position = Vec3::new(
            self.position.x + self.velocity.x * delta_time,
            self.position.y + self.velocity.y * delta_time,
            self.position.z,
        );
        self.velocity = Vec2::new(
            self.velocity.x,
            self.velocity.y + self.gravity * delta_time,
        );
    }
}

impl StatusEffect {
    pub fn update(&mut self, delta_time: f32) {
        if !self.enabled {
            return;
        }

        self.age += delta_time;
        
        if self.age >= self.duration {
            self.enabled = false;
            return;
        }
    }
}

impl Notification {
    pub fn update(&mut self, delta_time: f32) {
        if !self.enabled {
            return;
        }

        self.age += delta_time;
        
        if self.age >= self.duration {
            self.enabled = false;
            return;
        }
    }
}

impl HealthBar {
    pub fn update(&mut self, delta_time: f32) {
        // Update health bar
    }
}

impl ManaBar {
    pub fn update(&mut self, delta_time: f32) {
        // Update mana bar
    }
}

impl StaminaBar {
    pub fn update(&mut self, delta_time: f32) {
        // Update stamina bar
    }
}

impl ExperienceBar {
    pub fn update(&mut self, delta_time: f32) {
        // Update experience bar
    }
}

impl BuffIcon {
    pub fn update(&mut self, delta_time: f32) {
        if !self.enabled {
            return;
        }

        self.age += delta_time;
        
        if self.age >= self.duration {
            self.enabled = false;
            return;
        }
    }
}

impl DebuffIcon {
    pub fn update(&mut self, delta_time: f32) {
        if !self.enabled {
            return;
        }

        self.age += delta_time;
        
        if self.age >= self.duration {
            self.enabled = false;
            return;
        }
    }
}

impl InteractiveObject {
    pub fn update(&mut self, delta_time: f32) {
        // Update interactive object
    }
}

impl Collectible {
    pub fn update(&mut self, delta_time: f32) {
        // Update collectible
    }
}

impl Door {
    pub fn update(&mut self, delta_time: f32) {
        // Update door
    }
}

impl Chest {
    pub fn update(&mut self, delta_time: f32) {
        // Update chest
    }
}

impl Switch {
    pub fn update(&mut self, delta_time: f32) {
        // Update switch
    }
}