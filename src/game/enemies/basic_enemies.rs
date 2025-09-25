//! Basic enemy types for the 2D Brawler Engine
//! 
//! This module defines basic enemy types with simple AI behaviors:
//! - Goblin: Fast, weak, swarm behavior
//! - Orc: Strong, slow, aggressive behavior
//! - Archer: Ranged, tactical, positioning behavior

use super::{Enemy, EnemyType, AIState};
use crate::engine::ecs::Entity;
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Goblin enemy - Fast, weak, swarm behavior
#[derive(Debug, Clone)]
pub struct Goblin {
    /// Base enemy data
    pub enemy: Enemy,
    /// Swarm coordination
    pub swarm_id: Option<u32>,
    /// Pack behavior
    pub pack_size: u32,
    /// Aggression level
    pub aggression: f32,
    /// Last attack time
    pub last_attack_time: f32,
    /// Attack cooldown
    pub attack_cooldown: f32,
}

impl Goblin {
    /// Create a new goblin
    pub fn new(level: u32, swarm_id: Option<u32>) -> Self {
        let mut enemy = Enemy::new(EnemyType::Goblin, level);
        enemy.movement_speed = 120.0;
        enemy.attack_range = 60.0;
        enemy.detection_range = 80.0;

        Self {
            enemy,
            swarm_id,
            pack_size: 3,
            aggression: 0.8,
            last_attack_time: 0.0,
            attack_cooldown: 1.0,
        }
    }

    /// Update goblin behavior
    pub fn update(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        self.last_attack_time += dt;
        
        // Update AI state
        self.enemy.update_ai_state(Some(player_position), current_position);
        
        // Goblin-specific behavior
        match self.enemy.ai_state {
            AIState::Chasing => {
                // Goblins are more aggressive in groups
                if self.pack_size > 1 {
                    self.aggression = 1.0;
                } else {
                    self.aggression = 0.8;
                }
            }
            AIState::Attacking => {
                // Quick attacks with short cooldown
                if self.last_attack_time >= self.attack_cooldown {
                    self.perform_attack();
                    self.last_attack_time = 0.0;
                }
            }
            AIState::Fleeing => {
                // Goblins flee when health is low
                if self.enemy.health_percentage() > 0.3 {
                    self.enemy.ai_state = AIState::Chasing;
                }
            }
            _ => {}
        }
    }

    /// Perform goblin attack
    fn perform_attack(&mut self) {
        // Goblin swarm attack - multiple quick strikes
        println!("Goblin performs swarm attack!");
    }

    /// Get swarm behavior modifier
    pub fn get_swarm_modifier(&self) -> f32 {
        match self.pack_size {
            1 => 1.0,
            2 => 1.2,
            3 => 1.5,
            4..=5 => 1.8,
            _ => 2.0,
        }
    }
}

/// Orc enemy - Strong, slow, aggressive behavior
#[derive(Debug, Clone)]
pub struct Orc {
    /// Base enemy data
    pub enemy: Enemy,
    /// Rage level
    pub rage_level: f32,
    /// Heavy attack charge time
    pub charge_time: f32,
    /// Is charging heavy attack
    pub is_charging: bool,
    /// Last heavy attack time
    pub last_heavy_attack: f32,
    /// Heavy attack cooldown
    pub heavy_attack_cooldown: f32,
}

impl Orc {
    /// Create a new orc
    pub fn new(level: u32) -> Self {
        let mut enemy = Enemy::new(EnemyType::Orc, level);
        enemy.movement_speed = 80.0;
        enemy.attack_range = 100.0;
        enemy.detection_range = 120.0;

        Self {
            enemy,
            rage_level: 0.0,
            charge_time: 0.0,
            is_charging: false,
            last_heavy_attack: 0.0,
            heavy_attack_cooldown: 3.0,
        }
    }

    /// Update orc behavior
    pub fn update(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        self.last_heavy_attack += dt;
        
        // Update AI state
        self.enemy.update_ai_state(Some(player_position), current_position);
        
        // Orc-specific behavior
        match self.enemy.ai_state {
            AIState::Chasing => {
                // Build rage while chasing
                self.rage_level = (self.rage_level + dt * 0.1).min(1.0);
            }
            AIState::Attacking => {
                // Orcs prefer heavy attacks when possible
                if self.last_heavy_attack >= self.heavy_attack_cooldown && !self.is_charging {
                    self.start_heavy_attack();
                } else if self.is_charging {
                    self.charge_time += dt;
                    if self.charge_time >= 1.5 {
                        self.perform_heavy_attack();
                    }
                } else {
                    self.perform_normal_attack();
                }
            }
            AIState::Fleeing => {
                // Orcs rarely flee, only when very low health
                if self.enemy.health_percentage() > 0.1 {
                    self.enemy.ai_state = AIState::Chasing;
                }
            }
            _ => {}
        }
    }

    /// Start charging heavy attack
    fn start_heavy_attack(&mut self) {
        self.is_charging = true;
        self.charge_time = 0.0;
        println!("Orc starts charging heavy attack!");
    }

    /// Perform heavy attack
    fn perform_heavy_attack(&mut self) {
        self.is_charging = false;
        self.charge_time = 0.0;
        self.last_heavy_attack = 0.0;
        self.rage_level = 0.0; // Reset rage after heavy attack
        
        let damage_multiplier = 1.0 + self.rage_level;
        println!("Orc performs heavy attack with {}x damage!", damage_multiplier);
    }

    /// Perform normal attack
    fn perform_normal_attack(&mut self) {
        println!("Orc performs normal attack!");
    }

    /// Get rage damage modifier
    pub fn get_rage_modifier(&self) -> f32 {
        1.0 + self.rage_level * 0.5
    }
}

/// Archer enemy - Ranged, tactical, positioning behavior
#[derive(Debug, Clone)]
pub struct Archer {
    /// Base enemy data
    pub enemy: Enemy,
    /// Aiming time
    pub aim_time: f32,
    /// Is aiming
    pub is_aiming: bool,
    /// Last shot time
    pub last_shot_time: f32,
    /// Shot cooldown
    pub shot_cooldown: f32,
    /// Accuracy
    pub accuracy: f32,
    /// Preferred distance from target
    pub preferred_distance: f32,
}

impl Archer {
    /// Create a new archer
    pub fn new(level: u32) -> Self {
        let mut enemy = Enemy::new(EnemyType::Archer, level);
        enemy.movement_speed = 100.0;
        enemy.attack_range = 200.0;
        enemy.detection_range = 250.0;

        Self {
            enemy,
            aim_time: 0.0,
            is_aiming: false,
            last_shot_time: 0.0,
            shot_cooldown: 2.0,
            accuracy: 0.8,
            preferred_distance: 150.0,
        }
    }

    /// Update archer behavior
    pub fn update(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        self.last_shot_time += dt;
        
        // Update AI state
        self.enemy.update_ai_state(Some(player_position), current_position);
        
        let distance_to_player = current_position.distance(player_position);
        
        // Archer-specific behavior
        match self.enemy.ai_state {
            AIState::Chasing => {
                // Archers try to maintain preferred distance
                if distance_to_player < self.preferred_distance * 0.8 {
                    // Too close, move away
                    self.enemy.ai_state = AIState::Fleeing;
                } else if distance_to_player > self.preferred_distance * 1.2 {
                    // Too far, move closer
                    self.enemy.ai_state = AIState::Chasing;
                } else {
                    // Good distance, start aiming
                    self.enemy.ai_state = AIState::Attacking;
                }
            }
            AIState::Attacking => {
                if !self.is_aiming && self.last_shot_time >= self.shot_cooldown {
                    self.start_aiming();
                } else if self.is_aiming {
                    self.aim_time += dt;
                    if self.aim_time >= 1.0 {
                        self.take_shot(player_position, current_position);
                    }
                }
            }
            AIState::Fleeing => {
                // Archers flee to maintain distance
                if distance_to_player >= self.preferred_distance {
                    self.enemy.ai_state = AIState::Attacking;
                }
            }
            _ => {}
        }
    }

    /// Start aiming
    fn start_aiming(&mut self) {
        self.is_aiming = true;
        self.aim_time = 0.0;
        println!("Archer starts aiming!");
    }

    /// Take a shot
    fn take_shot(&mut self, target_position: Vec2, current_position: Vec2) {
        self.is_aiming = false;
        self.aim_time = 0.0;
        self.last_shot_time = 0.0;
        
        // Calculate accuracy based on distance and aim time
        let distance = current_position.distance(target_position);
        let distance_accuracy = (1.0 - (distance / self.enemy.attack_range).min(1.0)) * 0.3;
        let aim_accuracy = (self.aim_time / 1.0).min(1.0) * 0.7;
        let total_accuracy = self.accuracy + distance_accuracy + aim_accuracy;
        
        println!("Archer shoots with {:.1}% accuracy!", total_accuracy * 100.0);
    }

    /// Get accuracy modifier based on conditions
    pub fn get_accuracy_modifier(&self, distance: f32, is_moving: bool) -> f32 {
        let distance_modifier = if distance <= self.preferred_distance {
            1.0
        } else {
            1.0 - ((distance - self.preferred_distance) / self.preferred_distance * 0.5).min(0.5)
        };
        
        let movement_modifier = if is_moving { 0.8 } else { 1.0 };
        
        distance_modifier * movement_modifier
    }
}

/// Basic enemy factory for creating enemies
pub struct BasicEnemyFactory;

impl BasicEnemyFactory {
    /// Create a goblin
    pub fn create_goblin(level: u32, swarm_id: Option<u32>) -> Goblin {
        Goblin::new(level, swarm_id)
    }

    /// Create an orc
    pub fn create_orc(level: u32) -> Orc {
        Orc::new(level)
    }

    /// Create an archer
    pub fn create_archer(level: u32) -> Archer {
        Archer::new(level)
    }

    /// Create a basic enemy by type
    pub fn create_enemy(enemy_type: EnemyType, level: u32) -> Enemy {
        match enemy_type {
            EnemyType::Goblin => {
                let goblin = Self::create_goblin(level, None);
                goblin.enemy
            }
            EnemyType::Orc => {
                let orc = Self::create_orc(level);
                orc.enemy
            }
            EnemyType::Archer => {
                let archer = Self::create_archer(level);
                archer.enemy
            }
            _ => Enemy::new(enemy_type, level),
        }
    }
}
