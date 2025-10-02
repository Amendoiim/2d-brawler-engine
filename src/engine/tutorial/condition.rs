//! Tutorial condition system
//! 
//! This module provides the condition evaluation system for tutorial steps.

use super::step::{TutorialConditionType, TutorialConditionData, LogicalOperator};
use super::step::{TutorialContext, InputState, PlayerState, TutorialState, GameState};
use std::collections::HashMap;

/// Condition evaluator for tutorial steps
pub struct ConditionEvaluator {
    /// Registered custom conditions
    custom_conditions: HashMap<String, Box<dyn Fn(&TutorialContext) -> bool + Send + Sync>>,
}

impl ConditionEvaluator {
    /// Create a new condition evaluator
    pub fn new() -> Self {
        Self {
            custom_conditions: HashMap::new(),
        }
    }

    /// Register a custom condition
    pub fn register_custom_condition<F>(&mut self, name: String, condition: F)
    where
        F: Fn(&TutorialContext) -> bool + Send + Sync + 'static,
    {
        self.custom_conditions.insert(name, Box::new(condition));
    }

    /// Evaluate a condition
    pub fn evaluate_condition(&self, condition_type: &TutorialConditionType, data: &TutorialConditionData, context: &TutorialContext) -> bool {
        match condition_type {
            TutorialConditionType::Always => true,
            TutorialConditionType::Never => false,
            TutorialConditionType::KeyPressed => self.evaluate_key_pressed(data, context),
            TutorialConditionType::MouseClicked => self.evaluate_mouse_clicked(data, context),
            TutorialConditionType::PlayerMoved => self.evaluate_player_moved(data, context),
            TutorialConditionType::ItemUsed => self.evaluate_item_used(data, context),
            TutorialConditionType::EnemyDefeated => self.evaluate_enemy_defeated(data, context),
            TutorialConditionType::LevelReached => self.evaluate_level_reached(data, context),
            TutorialConditionType::TimeElapsed => self.evaluate_time_elapsed(data, context),
            TutorialConditionType::Custom => self.evaluate_custom_condition(data, context),
        }
    }

    /// Evaluate key pressed condition
    fn evaluate_key_pressed(&self, data: &TutorialConditionData, context: &TutorialContext) -> bool {
        if let TutorialConditionData::KeyData { key, modifiers } = data {
            context.input_state.is_key_pressed(key) && 
            context.input_state.has_modifiers(modifiers.clone())
        } else {
            false
        }
    }

    /// Evaluate mouse clicked condition
    fn evaluate_mouse_clicked(&self, data: &TutorialConditionData, context: &TutorialContext) -> bool {
        if let TutorialConditionData::MouseData { button, position } = data {
            context.input_state.is_mouse_clicked(button) &&
            context.input_state.is_mouse_at_position(*position)
        } else {
            false
        }
    }

    /// Evaluate player moved condition
    fn evaluate_player_moved(&self, data: &TutorialConditionData, context: &TutorialContext) -> bool {
        if let TutorialConditionData::MovementData { distance, direction } = data {
            context.player_state.has_moved_distance(*distance) &&
            context.player_state.moved_in_direction(direction)
        } else {
            false
        }
    }

    /// Evaluate item used condition
    fn evaluate_item_used(&self, data: &TutorialConditionData, context: &TutorialContext) -> bool {
        if let TutorialConditionData::ItemData { item_id, quantity } = data {
            context.player_state.has_used_item(item_id) &&
            context.player_state.used_item_quantity(item_id) >= *quantity
        } else {
            false
        }
    }

    /// Evaluate enemy defeated condition
    fn evaluate_enemy_defeated(&self, data: &TutorialConditionData, context: &TutorialContext) -> bool {
        if let TutorialConditionData::EnemyData { enemy_type, count } = data {
            context.player_state.defeated_enemies_of_type(enemy_type) >= *count
        } else {
            false
        }
    }

    /// Evaluate level reached condition
    fn evaluate_level_reached(&self, data: &TutorialConditionData, context: &TutorialContext) -> bool {
        if let TutorialConditionData::LevelData { level_id, position } = data {
            context.player_state.is_at_level(level_id) &&
            context.player_state.is_at_position(*position)
        } else {
            false
        }
    }

    /// Evaluate time elapsed condition
    fn evaluate_time_elapsed(&self, data: &TutorialConditionData, context: &TutorialContext) -> bool {
        if let TutorialConditionData::TimeData { duration } = data {
            context.tutorial_state.step_duration >= *duration
        } else {
            false
        }
    }

    /// Evaluate custom condition
    fn evaluate_custom_condition(&self, data: &TutorialConditionData, context: &TutorialContext) -> bool {
        if let TutorialConditionData::CustomData { data_type, value } = data {
            if let Some(condition) = self.custom_conditions.get(data_type) {
                condition(context)
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl Default for ConditionEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// Condition builder for creating complex conditions
pub struct ConditionBuilder {
    conditions: Vec<(TutorialConditionType, TutorialConditionData, LogicalOperator)>,
}

impl ConditionBuilder {
    /// Create a new condition builder
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    /// Add a condition
    pub fn add_condition(mut self, condition_type: TutorialConditionType, data: TutorialConditionData, operator: LogicalOperator) -> Self {
        self.conditions.push((condition_type, data, operator));
        self
    }

    /// Add key pressed condition
    pub fn key_pressed(mut self, key: &str, modifiers: Vec<String>) -> Self {
        self.conditions.push((
            TutorialConditionType::KeyPressed,
            TutorialConditionData::KeyData {
                key: key.to_string(),
                modifiers,
            },
            LogicalOperator::And,
        ));
        self
    }

    /// Add mouse clicked condition
    pub fn mouse_clicked(mut self, button: &str, position: (f32, f32)) -> Self {
        self.conditions.push((
            TutorialConditionType::MouseClicked,
            TutorialConditionData::MouseData {
                button: button.to_string(),
                position,
            },
            LogicalOperator::And,
        ));
        self
    }

    /// Add player moved condition
    pub fn player_moved(mut self, distance: f32, direction: &str) -> Self {
        self.conditions.push((
            TutorialConditionType::PlayerMoved,
            TutorialConditionData::MovementData {
                distance,
                direction: direction.to_string(),
            },
            LogicalOperator::And,
        ));
        self
    }

    /// Add item used condition
    pub fn item_used(mut self, item_id: &str, quantity: u32) -> Self {
        self.conditions.push((
            TutorialConditionType::ItemUsed,
            TutorialConditionData::ItemData {
                item_id: item_id.to_string(),
                quantity,
            },
            LogicalOperator::And,
        ));
        self
    }

    /// Add enemy defeated condition
    pub fn enemy_defeated(mut self, enemy_type: &str, count: u32) -> Self {
        self.conditions.push((
            TutorialConditionType::EnemyDefeated,
            TutorialConditionData::EnemyData {
                enemy_type: enemy_type.to_string(),
                count,
            },
            LogicalOperator::And,
        ));
        self
    }

    /// Add level reached condition
    pub fn level_reached(mut self, level_id: &str, position: (f32, f32)) -> Self {
        self.conditions.push((
            TutorialConditionType::LevelReached,
            TutorialConditionData::LevelData {
                level_id: level_id.to_string(),
                position,
            },
            LogicalOperator::And,
        ));
        self
    }

    /// Add time elapsed condition
    pub fn time_elapsed(mut self, duration: f32) -> Self {
        self.conditions.push((
            TutorialConditionType::TimeElapsed,
            TutorialConditionData::TimeData { duration },
            LogicalOperator::And,
        ));
        self
    }

    /// Add OR operator
    pub fn or(mut self) -> Self {
        if let Some((_, _, op)) = self.conditions.last_mut() {
            *op = LogicalOperator::Or;
        }
        self
    }

    /// Add NOT operator
    pub fn not(mut self) -> Self {
        if let Some((_, _, op)) = self.conditions.last_mut() {
            *op = LogicalOperator::Not;
        }
        self
    }

    /// Build the condition
    pub fn build(self) -> TutorialCondition {
        if self.conditions.is_empty() {
            return TutorialCondition::default();
        }

        let mut condition = TutorialCondition {
            condition_type: self.conditions[0].0.clone(),
            data: self.conditions[0].1.clone(),
            operator: LogicalOperator::And,
            sub_conditions: Vec::new(),
        };

        for (condition_type, data, operator) in self.conditions.into_iter().skip(1) {
            let sub_condition = TutorialCondition {
                condition_type,
                data,
                operator: LogicalOperator::And,
                sub_conditions: Vec::new(),
            };

            match operator {
                LogicalOperator::And => {
                    condition.sub_conditions.push(sub_condition);
                }
                LogicalOperator::Or => {
                    condition.operator = LogicalOperator::Or;
                    condition.sub_conditions.push(sub_condition);
                }
                LogicalOperator::Not => {
                    let not_condition = TutorialCondition {
                        condition_type: sub_condition.condition_type,
                        data: sub_condition.data,
                        operator: LogicalOperator::Not,
                        sub_conditions: Vec::new(),
                    };
                    condition.sub_conditions.push(not_condition);
                }
            }
        }

        condition
    }
}

impl Default for ConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Tutorial condition implementation
#[derive(Debug, Clone)]
pub struct TutorialCondition {
    /// Condition type
    pub condition_type: TutorialConditionType,
    /// Condition data
    pub data: TutorialConditionData,
    /// Logical operator
    pub operator: LogicalOperator,
    /// Sub-conditions
    pub sub_conditions: Vec<TutorialCondition>,
}

impl TutorialCondition {
    /// Create a new condition
    pub fn new(condition_type: TutorialConditionType, data: TutorialConditionData) -> Self {
        Self {
            condition_type,
            data,
            operator: LogicalOperator::And,
            sub_conditions: Vec::new(),
        }
    }

    /// Create a key pressed condition
    pub fn key_pressed(key: &str, modifiers: Vec<String>) -> Self {
        Self::new(
            TutorialConditionType::KeyPressed,
            TutorialConditionData::KeyData {
                key: key.to_string(),
                modifiers,
            },
        )
    }

    /// Create a mouse clicked condition
    pub fn mouse_clicked(button: &str, position: (f32, f32)) -> Self {
        Self::new(
            TutorialConditionType::MouseClicked,
            TutorialConditionData::MouseData {
                button: button.to_string(),
                position,
            },
        )
    }

    /// Create a player moved condition
    pub fn player_moved(distance: f32, direction: &str) -> Self {
        Self::new(
            TutorialConditionType::PlayerMoved,
            TutorialConditionData::MovementData {
                distance,
                direction: direction.to_string(),
            },
        )
    }

    /// Create an item used condition
    pub fn item_used(item_id: &str, quantity: u32) -> Self {
        Self::new(
            TutorialConditionType::ItemUsed,
            TutorialConditionData::ItemData {
                item_id: item_id.to_string(),
                quantity,
            },
        )
    }

    /// Create an enemy defeated condition
    pub fn enemy_defeated(enemy_type: &str, count: u32) -> Self {
        Self::new(
            TutorialConditionType::EnemyDefeated,
            TutorialConditionData::EnemyData {
                enemy_type: enemy_type.to_string(),
                count,
            },
        )
    }

    /// Create a level reached condition
    pub fn level_reached(level_id: &str, position: (f32, f32)) -> Self {
        Self::new(
            TutorialConditionType::LevelReached,
            TutorialConditionData::LevelData {
                level_id: level_id.to_string(),
                position,
            },
        )
    }

    /// Create a time elapsed condition
    pub fn time_elapsed(duration: f32) -> Self {
        Self::new(
            TutorialConditionType::TimeElapsed,
            TutorialConditionData::TimeData { duration },
        )
    }

    /// Create an always true condition
    pub fn always() -> Self {
        Self::new(
            TutorialConditionType::Always,
            TutorialConditionData::None,
        )
    }

    /// Create a never true condition
    pub fn never() -> Self {
        Self::new(
            TutorialConditionType::Never,
            TutorialConditionData::None,
        )
    }

    /// Add a sub-condition
    pub fn add_sub_condition(mut self, condition: TutorialCondition) -> Self {
        self.sub_conditions.push(condition);
        self
    }

    /// Set the logical operator
    pub fn with_operator(mut self, operator: LogicalOperator) -> Self {
        self.operator = operator;
        self
    }

    /// Evaluate the condition
    pub fn evaluate(&self, context: &TutorialContext) -> bool {
        let evaluator = ConditionEvaluator::new();
        let self_result = evaluator.evaluate_condition(&self.condition_type, &self.data, context);

        match self.operator {
            LogicalOperator::And => {
                self_result && self.sub_conditions.iter().all(|c| c.evaluate(context))
            }
            LogicalOperator::Or => {
                self_result || self.sub_conditions.iter().any(|c| c.evaluate(context))
            }
            LogicalOperator::Not => {
                !self_result
            }
        }
    }
}

impl Default for TutorialCondition {
    fn default() -> Self {
        Self::always()
    }
}
