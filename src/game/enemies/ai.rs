//! AI system for enemy behavior in the 2D Brawler Engine
//! 
//! This module provides a comprehensive AI system with:
//! - State machine for enemy behaviors
//! - Pathfinding integration
//! - Group coordination and tactics
//! - Difficulty-based AI scaling
//! - Environmental interaction and awareness

use super::{Enemy, EnemyType, AIState, EnemyManager};
use crate::engine::ecs::Entity;
use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI behavior state machine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIStateMachine {
    /// Current state
    pub current_state: AIState,
    /// Previous state
    pub previous_state: AIState,
    /// State transition conditions
    pub transitions: HashMap<AIState, Vec<StateTransition>>,
    /// State entry time
    pub state_entry_time: f32,
    /// State duration
    pub state_duration: f32,
    /// AI difficulty level
    pub difficulty: f32,
    /// AI aggressiveness
    pub aggressiveness: f32,
    /// AI intelligence
    pub intelligence: f32,
}

/// State transition condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    /// Target state
    pub target_state: AIState,
    /// Condition function
    pub condition: TransitionCondition,
    /// Priority (higher = more important)
    pub priority: u32,
}

/// Transition condition types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionCondition {
    /// Health below threshold
    HealthBelow(f32),
    /// Health above threshold
    HealthAbove(f32),
    /// Distance to target
    DistanceToTarget(f32, DistanceCondition),
    /// Time in current state
    TimeInState(f32),
    /// Target lost
    TargetLost,
    /// Target found
    TargetFound,
    /// Custom condition
    Custom(String),
}

/// Distance condition types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistanceCondition {
    LessThan,
    GreaterThan,
    Equal,
}

impl AIStateMachine {
    /// Create a new AI state machine
    pub fn new(difficulty: f32) -> Self {
        let mut transitions = HashMap::new();
        
        // Define state transitions for each state
        transitions.insert(AIState::Idle, vec![
            StateTransition {
                target_state: AIState::Chasing,
                condition: TransitionCondition::TargetFound,
                priority: 10,
            },
        ]);
        
        transitions.insert(AIState::Chasing, vec![
            StateTransition {
                target_state: AIState::Attacking,
                condition: TransitionCondition::DistanceToTarget(80.0, DistanceCondition::LessThan),
                priority: 10,
            },
            StateTransition {
                target_state: AIState::Searching,
                condition: TransitionCondition::TargetLost,
                priority: 8,
            },
            StateTransition {
                target_state: AIState::Fleeing,
                condition: TransitionCondition::HealthBelow(0.3),
                priority: 5,
            },
        ]);
        
        transitions.insert(AIState::Attacking, vec![
            StateTransition {
                target_state: AIState::Chasing,
                condition: TransitionCondition::DistanceToTarget(100.0, DistanceCondition::GreaterThan),
                priority: 8,
            },
            StateTransition {
                target_state: AIState::Fleeing,
                condition: TransitionCondition::HealthBelow(0.2),
                priority: 6,
            },
        ]);
        
        transitions.insert(AIState::Searching, vec![
            StateTransition {
                target_state: AIState::Chasing,
                condition: TransitionCondition::TargetFound,
                priority: 10,
            },
            StateTransition {
                target_state: AIState::Idle,
                condition: TransitionCondition::TimeInState(5.0),
                priority: 3,
            },
        ]);
        
        transitions.insert(AIState::Fleeing, vec![
            StateTransition {
                target_state: AIState::Chasing,
                condition: TransitionCondition::HealthAbove(0.5),
                priority: 7,
            },
            StateTransition {
                target_state: AIState::Idle,
                condition: TransitionCondition::TimeInState(3.0),
                priority: 4,
            },
        ]);

        Self {
            current_state: AIState::Idle,
            previous_state: AIState::Idle,
            transitions,
            state_entry_time: 0.0,
            state_duration: 0.0,
            difficulty,
            aggressiveness: difficulty,
            intelligence: difficulty,
        }
    }

    /// Update the state machine
    pub fn update(&mut self, dt: f32, enemy: &Enemy, target_position: Option<Vec2>, current_position: Vec2) {
        self.state_duration += dt;
        
        // Check for state transitions
        if let Some(transitions) = self.transitions.get(&self.current_state) {
            for transition in transitions {
                if self.check_condition(&transition.condition, enemy, target_position, current_position) {
                    self.transition_to(transition.target_state);
                    break;
                }
            }
        }
    }

    /// Check if a transition condition is met
    fn check_condition(&self, condition: &TransitionCondition, enemy: &Enemy, target_position: Option<Vec2>, current_position: Vec2) -> bool {
        match condition {
            TransitionCondition::HealthBelow(threshold) => {
                enemy.health_percentage() < *threshold
            }
            TransitionCondition::HealthAbove(threshold) => {
                enemy.health_percentage() > *threshold
            }
            TransitionCondition::DistanceToTarget(distance, condition_type) => {
                if let Some(target_pos) = target_position {
                    let dist = current_position.distance(target_pos);
                    match condition_type {
                        DistanceCondition::LessThan => dist < *distance,
                        DistanceCondition::GreaterThan => dist > *distance,
                        DistanceCondition::Equal => (dist - *distance).abs() < 10.0,
                    }
                } else {
                    false
                }
            }
            TransitionCondition::TimeInState(time) => {
                self.state_duration >= *time
            }
            TransitionCondition::TargetLost => {
                target_position.is_none()
            }
            TransitionCondition::TargetFound => {
                target_position.is_some()
            }
            TransitionCondition::Custom(_) => {
                // Custom conditions would be implemented here
                false
            }
        }
    }

    /// Transition to a new state
    fn transition_to(&mut self, new_state: AIState) {
        self.previous_state = self.current_state;
        self.current_state = new_state;
        self.state_entry_time = 0.0;
        self.state_duration = 0.0;
    }

    /// Get current state
    pub fn get_current_state(&self) -> AIState {
        self.current_state
    }

    /// Get state duration
    pub fn get_state_duration(&self) -> f32 {
        self.state_duration
    }
}

/// AI behavior controller
#[derive(Debug, Clone)]
pub struct AIBehaviorController {
    /// State machine
    pub state_machine: AIStateMachine,
    /// Pathfinding data
    pub pathfinding: PathfindingData,
    /// Group coordination
    pub group_coordination: GroupCoordination,
    /// Environmental awareness
    pub environmental_awareness: EnvironmentalAwareness,
    /// Decision making
    pub decision_making: DecisionMaking,
}

/// Pathfinding data for AI
#[derive(Debug, Clone)]
pub struct PathfindingData {
    /// Current path
    pub current_path: Vec<Vec2>,
    /// Current path index
    pub path_index: usize,
    /// Pathfinding target
    pub pathfinding_target: Option<Vec2>,
    /// Pathfinding update interval
    pub pathfinding_interval: f32,
    /// Last pathfinding update
    pub last_pathfinding_update: f32,
    /// Pathfinding range
    pub pathfinding_range: f32,
}

/// Group coordination for AI
#[derive(Debug, Clone)]
pub struct GroupCoordination {
    /// Group ID
    pub group_id: Option<u32>,
    /// Group role
    pub group_role: GroupRole,
    /// Coordination target
    pub coordination_target: Option<Vec2>,
    /// Group formation
    pub formation: FormationType,
    /// Communication range
    pub communication_range: f32,
}

/// Group roles for AI coordination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupRole {
    Leader,
    Follower,
    Scout,
    Support,
    Tank,
}

/// Formation types for group coordination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormationType {
    Line,
    Circle,
    V,
    Wedge,
    Loose,
}

/// Environmental awareness for AI
#[derive(Debug, Clone)]
pub struct EnvironmentalAwareness {
    /// Known obstacles
    pub obstacles: Vec<Vec2>,
    /// Known cover positions
    pub cover_positions: Vec<Vec2>,
    /// Known danger zones
    pub danger_zones: Vec<Vec2>,
    /// Environmental hazards
    pub hazards: Vec<EnvironmentalHazard>,
    /// Awareness radius
    pub awareness_radius: f32,
}

/// Environmental hazard types
#[derive(Debug, Clone)]
pub struct EnvironmentalHazard {
    /// Hazard position
    pub position: Vec2,
    /// Hazard type
    pub hazard_type: HazardType,
    /// Hazard radius
    pub radius: f32,
    /// Hazard damage
    pub damage: f32,
}

/// Hazard types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HazardType {
    Fire,
    Poison,
    Electric,
    Explosive,
    Spikes,
}

/// Decision making for AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionMaking {
    /// Decision history
    pub decision_history: Vec<AIDecision>,
    /// Learning rate
    pub learning_rate: f32,
    /// Risk tolerance
    pub risk_tolerance: f32,
    /// Aggression level
    pub aggression_level: f32,
    /// Intelligence level
    pub intelligence_level: f32,
}

/// AI decision types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIDecision {
    /// Decision type
    pub decision_type: DecisionType,
    /// Decision outcome
    pub outcome: DecisionOutcome,
    /// Decision time
    pub timestamp: f32,
    /// Decision context
    pub context: String,
}

/// Decision types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionType {
    Attack,
    Retreat,
    Flank,
    Ambush,
    Defend,
    Patrol,
}

/// Decision outcomes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionOutcome {
    Success,
    Failure,
    Partial,
    Unknown,
}

impl AIBehaviorController {
    /// Create a new AI behavior controller
    pub fn new(difficulty: f32) -> Self {
        Self {
            state_machine: AIStateMachine::new(difficulty),
            pathfinding: PathfindingData {
                current_path: Vec::new(),
                path_index: 0,
                pathfinding_target: None,
                pathfinding_interval: 0.5,
                last_pathfinding_update: 0.0,
                pathfinding_range: 200.0,
            },
            group_coordination: GroupCoordination {
                group_id: None,
                group_role: GroupRole::Follower,
                coordination_target: None,
                formation: FormationType::Loose,
                communication_range: 150.0,
            },
            environmental_awareness: EnvironmentalAwareness {
                obstacles: Vec::new(),
                cover_positions: Vec::new(),
                danger_zones: Vec::new(),
                hazards: Vec::new(),
                awareness_radius: 100.0,
            },
            decision_making: DecisionMaking {
                decision_history: Vec::new(),
                learning_rate: 0.1,
                risk_tolerance: 0.5,
                aggression_level: difficulty,
                intelligence_level: difficulty,
            },
        }
    }

    /// Update AI behavior
    pub fn update(&mut self, dt: f32, enemy: &Enemy, target_position: Option<Vec2>, current_position: Vec2) {
        // Update state machine
        self.state_machine.update(dt, enemy, target_position, current_position);
        
        // Update pathfinding
        self.update_pathfinding(dt, target_position, current_position);
        
        // Update group coordination
        self.update_group_coordination(dt, current_position);
        
        // Update environmental awareness
        self.update_environmental_awareness(dt, current_position);
        
        // Make decisions
        self.make_decision(dt, enemy, target_position, current_position);
    }

    /// Update pathfinding
    fn update_pathfinding(&mut self, dt: f32, target_position: Option<Vec2>, current_position: Vec2) {
        self.pathfinding.last_pathfinding_update += dt;
        
        if self.pathfinding.last_pathfinding_update >= self.pathfinding.pathfinding_interval {
            if let Some(target) = target_position {
                // Update pathfinding target
                self.pathfinding.pathfinding_target = Some(target);
                
                // Simple pathfinding - direct line for now
                // In a real implementation, this would use A* or similar
                self.pathfinding.current_path = vec![current_position, target];
                self.pathfinding.path_index = 0;
            }
            
            self.pathfinding.last_pathfinding_update = 0.0;
        }
    }

    /// Update group coordination
    fn update_group_coordination(&mut self, dt: f32, current_position: Vec2) {
        // Group coordination logic would be implemented here
        // This would handle formation movement, group tactics, etc.
    }

    /// Update environmental awareness
    fn update_environmental_awareness(&mut self, dt: f32, current_position: Vec2) {
        // Environmental awareness logic would be implemented here
        // This would handle obstacle detection, cover identification, etc.
    }

    /// Make AI decisions
    fn make_decision(&mut self, dt: f32, enemy: &Enemy, target_position: Option<Vec2>, current_position: Vec2) {
        let decision = match self.state_machine.get_current_state() {
            AIState::Idle => self.decide_idle_behavior(enemy, target_position, current_position),
            AIState::Chasing => self.decide_chasing_behavior(enemy, target_position, current_position),
            AIState::Attacking => self.decide_attacking_behavior(enemy, target_position, current_position),
            AIState::Searching => self.decide_searching_behavior(enemy, target_position, current_position),
            AIState::Fleeing => self.decide_fleeing_behavior(enemy, target_position, current_position),
            AIState::Special => self.decide_special_behavior(enemy, target_position, current_position),
            AIState::Dead => DecisionType::Defend,
        };
        
        // Record decision
        self.record_decision(decision, current_position);
    }

    /// Decide idle behavior
    fn decide_idle_behavior(&self, enemy: &Enemy, target_position: Option<Vec2>, current_position: Vec2) -> DecisionType {
        if target_position.is_some() {
            DecisionType::Attack
        } else {
            DecisionType::Patrol
        }
    }

    /// Decide chasing behavior
    fn decide_chasing_behavior(&self, enemy: &Enemy, target_position: Option<Vec2>, current_position: Vec2) -> DecisionType {
        if let Some(target) = target_position {
            let distance = current_position.distance(target);
            if distance < enemy.attack_range {
                DecisionType::Attack
            } else if distance > enemy.detection_range {
                DecisionType::Patrol
            } else {
                DecisionType::Attack
            }
        } else {
            DecisionType::Patrol
        }
    }

    /// Decide attacking behavior
    fn decide_attacking_behavior(&self, enemy: &Enemy, target_position: Option<Vec2>, current_position: Vec2) -> DecisionType {
        if enemy.health_percentage() < 0.3 {
            DecisionType::Retreat
        } else if let Some(target) = target_position {
            let distance = current_position.distance(target);
            if distance > enemy.attack_range {
                DecisionType::Attack
            } else {
                DecisionType::Attack
            }
        } else {
            DecisionType::Patrol
        }
    }

    /// Decide searching behavior
    fn decide_searching_behavior(&self, enemy: &Enemy, target_position: Option<Vec2>, current_position: Vec2) -> DecisionType {
        if target_position.is_some() {
            DecisionType::Attack
        } else {
            DecisionType::Patrol
        }
    }

    /// Decide fleeing behavior
    fn decide_fleeing_behavior(&self, enemy: &Enemy, target_position: Option<Vec2>, current_position: Vec2) -> DecisionType {
        if enemy.health_percentage() > 0.5 {
            DecisionType::Attack
        } else {
            DecisionType::Retreat
        }
    }

    /// Decide special behavior
    fn decide_special_behavior(&self, enemy: &Enemy, target_position: Option<Vec2>, current_position: Vec2) -> DecisionType {
        DecisionType::Defend
    }

    /// Record a decision
    fn record_decision(&mut self, decision: DecisionType, current_position: Vec2) {
        let ai_decision = AIDecision {
            decision_type: decision,
            outcome: DecisionOutcome::Unknown,
            timestamp: 0.0, // Would be set to current time
            context: format!("Position: {:?}", current_position),
        };
        
        self.decision_making.decision_history.push(ai_decision);
        
        // Keep only recent decisions
        if self.decision_making.decision_history.len() > 100 {
            self.decision_making.decision_history.remove(0);
        }
    }

    /// Get next movement direction
    pub fn get_movement_direction(&self, current_position: Vec2) -> Vec2 {
        if self.pathfinding.current_path.is_empty() {
            return Vec2::ZERO;
        }
        
        if self.pathfinding.path_index >= self.pathfinding.current_path.len() {
            return Vec2::ZERO;
        }
        
        let target = self.pathfinding.current_path[self.pathfinding.path_index];
        let direction = (target - current_position).normalize();
        
        direction
    }

    /// Get current AI state
    pub fn get_current_state(&self) -> AIState {
        self.state_machine.get_current_state()
    }
}

/// AI system manager
#[derive(Debug, Clone)]
pub struct AISystem {
    /// AI controllers for each enemy
    pub controllers: HashMap<Entity, AIBehaviorController>,
    /// Global AI settings
    pub global_settings: AIGlobalSettings,
    /// AI performance metrics
    pub performance_metrics: AIPerformanceMetrics,
}

/// Global AI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGlobalSettings {
    /// Global difficulty
    pub global_difficulty: f32,
    /// AI update frequency
    pub update_frequency: f32,
    /// Maximum AI complexity
    pub max_complexity: u32,
    /// AI learning enabled
    pub learning_enabled: bool,
}

/// AI performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIPerformanceMetrics {
    /// Average decision time
    pub average_decision_time: f32,
    /// AI update count
    pub update_count: u32,
    /// Performance issues
    pub performance_issues: Vec<String>,
}

impl AISystem {
    /// Create a new AI system
    pub fn new() -> Self {
        Self {
            controllers: HashMap::new(),
            global_settings: AIGlobalSettings {
                global_difficulty: 1.0,
                update_frequency: 60.0,
                max_complexity: 100,
                learning_enabled: true,
            },
            performance_metrics: AIPerformanceMetrics {
                average_decision_time: 0.0,
                update_count: 0,
                performance_issues: Vec::new(),
            },
        }
    }

    /// Add an AI controller for an enemy
    pub fn add_controller(&mut self, entity: Entity, difficulty: f32) {
        let controller = AIBehaviorController::new(difficulty);
        self.controllers.insert(entity, controller);
    }

    /// Remove an AI controller
    pub fn remove_controller(&mut self, entity: Entity) {
        self.controllers.remove(&entity);
    }

    /// Update all AI controllers
    pub fn update(&mut self, dt: f32, enemy_manager: &EnemyManager, player_position: Vec2) {
        for (entity, controller) in self.controllers.iter_mut() {
            if let Some(enemy) = enemy_manager.get_enemy(*entity) {
                controller.update(dt, enemy, Some(player_position), Vec2::ZERO); // Position would come from transform
            }
        }
        
        self.performance_metrics.update_count += 1;
    }

    /// Get AI controller for entity
    pub fn get_controller(&self, entity: Entity) -> Option<&AIBehaviorController> {
        self.controllers.get(&entity)
    }

    /// Get mutable AI controller for entity
    pub fn get_controller_mut(&mut self, entity: Entity) -> Option<&mut AIBehaviorController> {
        self.controllers.get_mut(&entity)
    }
}

impl Default for AISystem {
    fn default() -> Self {
        Self::new()
    }
}
