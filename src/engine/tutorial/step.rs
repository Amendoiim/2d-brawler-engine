//! Tutorial step definitions and management
//! 
//! This module defines tutorial steps, their conditions, and reward systems.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::{TutorialResult, TutorialError};

/// Individual tutorial step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialStep {
    /// Step ID
    pub id: String,
    /// Step type
    pub step_type: TutorialStepType,
    /// Step title
    pub title: String,
    /// Step description
    pub description: String,
    /// Detailed instructions
    pub instructions: Vec<String>,
    /// Completion condition
    pub completion_condition: TutorialCondition,
    /// Visual highlights
    pub visual_highlights: Vec<VisualHighlight>,
    /// Audio cues
    pub audio_cues: Vec<AudioCue>,
    /// Step duration (0 = no timeout)
    pub duration: f32,
    /// Required input
    pub required_input: Option<RequiredInput>,
    /// Step order
    pub order: u32,
    /// Optional step
    pub optional: bool,
    /// Skip condition
    pub skip_condition: Option<TutorialCondition>,
}

impl TutorialStep {
    /// Create a new tutorial step
    pub fn new(id: String, step_type: TutorialStepType, title: String, description: String) -> Self {
        Self {
            id,
            step_type,
            title,
            description,
            instructions: Vec::new(),
            completion_condition: TutorialCondition::default(),
            visual_highlights: Vec::new(),
            audio_cues: Vec::new(),
            duration: 0.0,
            required_input: None,
            order: 0,
            optional: false,
            skip_condition: None,
        }
    }

    /// Add instruction
    pub fn add_instruction(&mut self, instruction: String) {
        self.instructions.push(instruction);
    }

    /// Add visual highlight
    pub fn add_highlight(&mut self, highlight: VisualHighlight) {
        self.visual_highlights.push(highlight);
    }

    /// Add audio cue
    pub fn add_audio_cue(&mut self, audio_cue: AudioCue) {
        self.audio_cues.push(audio_cue);
    }

    /// Set completion condition
    pub fn set_completion_condition(&mut self, condition: TutorialCondition) {
        self.completion_condition = condition;
    }

    /// Set required input
    pub fn set_required_input(&mut self, input: RequiredInput) {
        self.required_input = Some(input);
    }

    /// Check if step can be skipped
    pub fn can_skip(&self, context: &TutorialContext) -> bool {
        if let Some(skip_condition) = &self.skip_condition {
            skip_condition.evaluate(context)
        } else {
            self.optional
        }
    }
}

/// Tutorial step types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TutorialStepType {
    /// Information step (just display text)
    Information,
    /// Interactive step (requires player action)
    Interactive,
    /// Demonstration step (show example)
    Demonstration,
    /// Practice step (let player try)
    Practice,
    /// Quiz step (test knowledge)
    Quiz,
    /// Navigation step (guide to location)
    Navigation,
    /// Combat step (combat tutorial)
    Combat,
    /// Inventory step (inventory tutorial)
    Inventory,
    /// Settings step (settings tutorial)
    Settings,
    /// Custom step (custom behavior)
    Custom(String),
}

/// Tutorial condition for step completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialCondition {
    /// Condition type
    pub condition_type: TutorialConditionType,
    /// Condition data
    pub data: TutorialConditionData,
    /// Logical operator for multiple conditions
    pub operator: LogicalOperator,
    /// Sub-conditions
    pub sub_conditions: Vec<TutorialCondition>,
}

impl Default for TutorialCondition {
    fn default() -> Self {
        Self {
            condition_type: TutorialConditionType::Always,
            data: TutorialConditionData::default(),
            operator: LogicalOperator::And,
            sub_conditions: Vec::new(),
        }
    }
}

impl TutorialCondition {
    /// Evaluate the condition
    pub fn evaluate(&self, context: &TutorialContext) -> bool {
        match self.operator {
            LogicalOperator::And => {
                self.evaluate_self(context) && self.sub_conditions.iter().all(|c| c.evaluate(context))
            }
            LogicalOperator::Or => {
                self.evaluate_self(context) || self.sub_conditions.iter().any(|c| c.evaluate(context))
            }
            LogicalOperator::Not => {
                !self.evaluate_self(context)
            }
        }
    }

    /// Evaluate this condition only
    fn evaluate_self(&self, context: &TutorialContext) -> bool {
        match &self.condition_type {
            TutorialConditionType::Always => true,
            TutorialConditionType::Never => false,
            TutorialConditionType::KeyPressed => {
                if let TutorialConditionData::KeyData { key, modifiers } = &self.data {
                    context.input_state.is_key_pressed(key) && 
                    context.input_state.has_modifiers(modifiers.clone())
                } else {
                    false
                }
            }
            TutorialConditionType::MouseClicked => {
                if let TutorialConditionData::MouseData { button, position } = &self.data {
                    context.input_state.is_mouse_clicked(button) &&
                    context.input_state.is_mouse_at_position(*position)
                } else {
                    false
                }
            }
            TutorialConditionType::PlayerMoved => {
                if let TutorialConditionData::MovementData { distance, direction } = &self.data {
                    context.player_state.has_moved_distance(*distance) &&
                    context.player_state.moved_in_direction(direction)
                } else {
                    false
                }
            }
            TutorialConditionType::ItemUsed => {
                if let TutorialConditionData::ItemData { item_id, quantity } = &self.data {
                    context.player_state.has_used_item(item_id) &&
                    context.player_state.used_item_quantity(item_id) >= *quantity
                } else {
                    false
                }
            }
            TutorialConditionType::EnemyDefeated => {
                if let TutorialConditionData::EnemyData { enemy_type, count } = &self.data {
                    context.player_state.defeated_enemies_of_type(enemy_type) >= *count
                } else {
                    false
                }
            }
            TutorialConditionType::LevelReached => {
                if let TutorialConditionData::LevelData { level_id, position } = &self.data {
                    context.player_state.is_at_level(level_id) &&
                    context.player_state.is_at_position(*position)
                } else {
                    false
                }
            }
            TutorialConditionType::TimeElapsed => {
                if let TutorialConditionData::TimeData { duration } = &self.data {
                    context.tutorial_state.step_duration >= *duration
                } else {
                    false
                }
            }
            TutorialConditionType::Custom => {
                // Custom conditions would be evaluated by the game
                false
            }
        }
    }
}

/// Logical operators for conditions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

/// Tutorial condition types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TutorialConditionType {
    /// Always true
    Always,
    /// Never true
    Never,
    /// Key pressed
    KeyPressed,
    /// Mouse clicked
    MouseClicked,
    /// Player moved
    PlayerMoved,
    /// Item used
    ItemUsed,
    /// Enemy defeated
    EnemyDefeated,
    /// Level reached
    LevelReached,
    /// Time elapsed
    TimeElapsed,
    /// Custom condition
    Custom,
}

/// Tutorial condition data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TutorialConditionData {
    /// No data
    None,
    /// Key data
    KeyData {
        key: String,
        modifiers: Vec<String>,
    },
    /// Mouse data
    MouseData {
        button: String,
        position: (f32, f32),
    },
    /// Movement data
    MovementData {
        distance: f32,
        direction: String,
    },
    /// Item data
    ItemData {
        item_id: String,
        quantity: u32,
    },
    /// Enemy data
    EnemyData {
        enemy_type: String,
        count: u32,
    },
    /// Level data
    LevelData {
        level_id: String,
        position: (f32, f32),
    },
    /// Time data
    TimeData {
        duration: f32,
    },
    /// Custom data
    CustomData {
        data_type: String,
        value: String,
    },
}

impl Default for TutorialConditionData {
    fn default() -> Self {
        Self::None
    }
}

/// Visual highlight for tutorial steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualHighlight {
    /// Highlight ID
    pub id: String,
    /// Highlight type
    pub highlight_type: HighlightType,
    /// Target element
    pub target: HighlightTarget,
    /// Highlight properties
    pub properties: HighlightProperties,
    /// Animation
    pub animation: Option<HighlightAnimation>,
    /// Duration
    pub duration: f32,
}

/// Highlight types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HighlightType {
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

/// Highlight target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HighlightTarget {
    /// UI element
    UIElement(String),
    /// Game object
    GameObject(String),
    /// Position
    Position((f32, f32)),
    /// Area
    Area((f32, f32, f32, f32)),
    /// Screen region
    ScreenRegion((f32, f32, f32, f32)),
}

/// Highlight properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightProperties {
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
}

impl Default for HighlightProperties {
    fn default() -> Self {
        Self {
            color: (1.0, 1.0, 0.0, 1.0), // Yellow
            intensity: 1.0,
            size: 1.0,
            thickness: 2.0,
            opacity: 0.8,
        }
    }
}

/// Highlight animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightAnimation {
    /// Animation type
    pub animation_type: AnimationType,
    /// Duration
    pub duration: f32,
    /// Loop
    pub loop_animation: bool,
    /// Easing
    pub easing: EasingType,
}

/// Animation types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationType {
    FadeIn,
    FadeOut,
    ScaleIn,
    ScaleOut,
    Pulse,
    Rotate,
    Move,
    Custom(String),
}

/// Easing types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EasingType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
    Custom(String),
}

/// Audio cue for tutorial steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioCue {
    /// Cue ID
    pub id: String,
    /// Audio file
    pub audio_file: String,
    /// Cue type
    pub cue_type: AudioCueType,
    /// Volume
    pub volume: f32,
    /// Pitch
    pub pitch: f32,
    /// Loop
    pub loop_audio: bool,
    /// Delay
    pub delay: f32,
}

/// Audio cue types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioCueType {
    /// Sound effect
    SoundEffect,
    /// Voice over
    VoiceOver,
    /// Music
    Music,
    /// Ambient
    Ambient,
    /// Custom
    Custom(String),
}

/// Required input for tutorial steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredInput {
    /// Input type
    pub input_type: InputType,
    /// Input data
    pub input_data: InputData,
    /// Timeout
    pub timeout: f32,
    /// Retry count
    pub max_retries: u32,
}

/// Input types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputType {
    /// Key press
    KeyPress,
    /// Key combination
    KeyCombination,
    /// Mouse click
    MouseClick,
    /// Mouse movement
    MouseMovement,
    /// Gamepad button
    GamepadButton,
    /// Gamepad stick
    GamepadStick,
    /// Custom input
    Custom(String),
}

/// Input data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputData {
    /// Key data
    KeyData {
        key: String,
        modifiers: Vec<String>,
    },
    /// Mouse data
    MouseData {
        button: String,
        position: (f32, f32),
    },
    /// Gamepad data
    GamepadData {
        button: String,
        stick: String,
        threshold: f32,
    },
    /// Custom data
    CustomData {
        data_type: String,
        value: String,
    },
}

/// Tutorial context for condition evaluation
#[derive(Debug, Clone)]
pub struct TutorialContext {
    /// Input state
    pub input_state: InputState,
    /// Player state
    pub player_state: PlayerState,
    /// Tutorial state
    pub tutorial_state: TutorialState,
    /// Game state
    pub game_state: GameState,
}

/// Input state
#[derive(Debug, Clone)]
pub struct InputState {
    /// Pressed keys
    pub pressed_keys: Vec<String>,
    /// Mouse position
    pub mouse_position: (f32, f32),
    /// Mouse buttons
    pub mouse_buttons: Vec<String>,
    /// Modifier keys
    pub modifiers: Vec<String>,
}

impl InputState {
    /// Check if key is pressed
    pub fn is_key_pressed(&self, key: &str) -> bool {
        self.pressed_keys.contains(&key.to_string())
    }

    /// Check if mouse is clicked
    pub fn is_mouse_clicked(&self, button: &str) -> bool {
        self.mouse_buttons.contains(&button.to_string())
    }

    /// Check if mouse is at position
    pub fn is_mouse_at_position(&self, position: (f32, f32)) -> bool {
        let (x, y) = position;
        let (mx, my) = self.mouse_position;
        (mx - x).abs() < 10.0 && (my - y).abs() < 10.0
    }

    /// Check if modifiers are pressed
    pub fn has_modifiers(&self, modifiers: Vec<String>) -> bool {
        modifiers.iter().all(|m| self.modifiers.contains(m))
    }
}

/// Player state
#[derive(Debug, Clone)]
pub struct PlayerState {
    /// Player position
    pub position: (f32, f32),
    /// Player level
    pub level: u32,
    /// Player health
    pub health: f32,
    /// Player mana
    pub mana: f32,
    /// Player stamina
    pub stamina: f32,
    /// Used items
    pub used_items: HashMap<String, u32>,
    /// Defeated enemies
    pub defeated_enemies: HashMap<String, u32>,
    /// Current level
    pub current_level: String,
}

impl PlayerState {
    /// Check if player has moved distance
    pub fn has_moved_distance(&self, distance: f32) -> bool {
        // This would be calculated based on movement tracking
        true // Placeholder
    }

    /// Check if player moved in direction
    pub fn moved_in_direction(&self, direction: &str) -> bool {
        // This would be calculated based on movement tracking
        true // Placeholder
    }

    /// Check if player has used item
    pub fn has_used_item(&self, item_id: &str) -> bool {
        self.used_items.contains_key(item_id)
    }

    /// Get used item quantity
    pub fn used_item_quantity(&self, item_id: &str) -> u32 {
        self.used_items.get(item_id).copied().unwrap_or(0)
    }

    /// Get defeated enemies of type
    pub fn defeated_enemies_of_type(&self, enemy_type: &str) -> u32 {
        self.defeated_enemies.get(enemy_type).copied().unwrap_or(0)
    }

    /// Check if player is at level
    pub fn is_at_level(&self, level_id: &str) -> bool {
        self.current_level == level_id
    }

    /// Check if player is at position
    pub fn is_at_position(&self, position: (f32, f32)) -> bool {
        let (x, y) = position;
        let (px, py) = self.position;
        (px - x).abs() < 10.0 && (py - y).abs() < 10.0
    }
}

/// Tutorial state
#[derive(Debug, Clone)]
pub struct TutorialState {
    /// Current tutorial
    pub current_tutorial: Option<String>,
    /// Current step
    pub current_step: Option<usize>,
    /// Step start time
    pub step_start_time: f32,
    /// Step duration
    pub step_duration: f32,
    /// Completed steps
    pub completed_steps: Vec<usize>,
}

/// Game state
#[derive(Debug, Clone)]
pub struct GameState {
    /// Current time
    pub current_time: f32,
    /// Game paused
    pub paused: bool,
    /// Current scene
    pub current_scene: String,
    /// Available tutorials
    pub available_tutorials: Vec<String>,
}

/// Tutorial reward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialReward {
    /// Reward type
    pub reward_type: RewardType,
    /// Reward data
    pub reward_data: RewardData,
    /// Quantity
    pub quantity: u32,
    /// Description
    pub description: String,
}

/// Reward types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RewardType {
    /// Experience points
    Experience,
    /// Gold/currency
    Currency,
    /// Item
    Item,
    /// Skill point
    SkillPoint,
    /// Unlock
    Unlock,
    /// Custom
    Custom(String),
}

/// Reward data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardData {
    /// No data
    None,
    /// Item data
    ItemData {
        item_id: String,
        item_type: String,
    },
    /// Unlock data
    UnlockData {
        unlock_type: String,
        unlock_id: String,
    },
    /// Custom data
    CustomData {
        data_type: String,
        value: String,
    },
}

impl Default for RewardData {
    fn default() -> Self {
        Self::None
    }
}
