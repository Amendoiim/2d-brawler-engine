//! Enemy system for the 2D Brawler Engine
//! 
//! This module provides a comprehensive enemy system with:
//! - Basic enemy types (Goblin, Orc, Archer)
//! - Specialized enemies (Mage, Berserker, Shield Bearer)
//! - Flying enemies (Bat, Demon, Dragon)
//! - Boss enemies with multiple phases
//! - AI behavior system with state machines
//! - Enemy spawning and management

pub mod basic_enemies;
pub mod special_enemies;
pub mod boss_enemies;
pub mod ai;

pub use basic_enemies::*;
pub use special_enemies::*;
pub use boss_enemies::*;
pub use ai::*;

use crate::engine::ecs::{Component, Entity};
use crate::game::combat::Combat;
use crate::game::characters::Character;
use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core enemy component
#[derive(Debug, Clone)]
pub struct Enemy {
    /// Enemy type identifier
    pub enemy_type: EnemyType,
    /// Current health
    pub health: f32,
    /// Maximum health
    pub max_health: f32,
    /// Current AI state
    pub ai_state: AIState,
    /// Enemy level (affects stats and abilities)
    pub level: u32,
    /// Experience value when defeated
    pub experience_value: u32,
    /// Whether the enemy is alive
    pub is_alive: bool,
    /// Enemy-specific abilities
    pub abilities: Vec<String>,
    /// Current target (player entity)
    pub target: Option<Entity>,
    /// Last known player position
    pub last_known_position: Option<Vec2>,
    /// Detection range
    pub detection_range: f32,
    /// Attack range
    pub attack_range: f32,
    /// Movement speed
    pub movement_speed: f32,
    /// Whether enemy can fly
    pub can_fly: bool,
    /// Whether enemy is currently flying
    pub is_flying: bool,
}

impl Component for Enemy {}

impl Enemy {
    /// Create a new enemy
    pub fn new(enemy_type: EnemyType, level: u32) -> Self {
        let (health, experience_value, abilities) = match enemy_type {
            EnemyType::Goblin => (50.0, 25, vec!["swarm_attack".to_string()]),
            EnemyType::Orc => (100.0, 50, vec!["heavy_attack".to_string()]),
            EnemyType::Archer => (75.0, 40, vec!["ranged_attack".to_string()]),
            EnemyType::Mage => (80.0, 60, vec!["fireball".to_string(), "teleport".to_string()]),
            EnemyType::Berserker => (120.0, 70, vec!["rage_mode".to_string(), "frenzy_attack".to_string()]),
            EnemyType::ShieldBearer => (150.0, 80, vec!["shield_bash".to_string(), "defensive_stance".to_string()]),
            EnemyType::Bat => (30.0, 15, vec!["dive_attack".to_string()]),
            EnemyType::Demon => (200.0, 150, vec!["fire_breath".to_string(), "flight".to_string()]),
            EnemyType::Dragon => (500.0, 500, vec!["fire_breath".to_string(), "flight".to_string(), "roar".to_string()]),
            EnemyType::GoblinKing => (300.0, 300, vec!["summon_goblins".to_string(), "area_attack".to_string()]),
            EnemyType::OrcWarlord => (400.0, 400, vec!["ground_pound".to_string(), "heavy_attack".to_string()]),
            EnemyType::DarkMage => (250.0, 350, vec!["teleport".to_string(), "spell_combination".to_string()]),
            EnemyType::DragonLord => (800.0, 1000, vec!["fire_breath".to_string(), "flight_phases".to_string(), "roar".to_string()]),
        };

        let level_multiplier = 1.0 + (level as f32 * 0.1);
        let scaled_health = health * level_multiplier;
        let scaled_experience = (experience_value as f32 * level_multiplier) as u32;

        Self {
            enemy_type,
            health: scaled_health,
            max_health: scaled_health,
            ai_state: AIState::Idle,
            level,
            experience_value: scaled_experience,
            is_alive: true,
            abilities,
            target: None,
            last_known_position: None,
            detection_range: match enemy_type {
                EnemyType::Archer | EnemyType::Mage => 200.0,
                EnemyType::Bat | EnemyType::Demon | EnemyType::Dragon => 150.0,
                _ => 100.0,
            },
            attack_range: match enemy_type {
                EnemyType::Archer | EnemyType::Mage => 150.0,
                EnemyType::Bat => 50.0,
                EnemyType::Demon | EnemyType::Dragon => 200.0,
                _ => 80.0,
            },
            movement_speed: match enemy_type {
                EnemyType::Goblin | EnemyType::Bat => 120.0,
                EnemyType::Orc | EnemyType::Berserker => 80.0,
                EnemyType::Archer | EnemyType::Mage => 100.0,
                EnemyType::ShieldBearer => 60.0,
                EnemyType::Demon | EnemyType::Dragon => 150.0,
                _ => 100.0,
            },
            can_fly: matches!(enemy_type, EnemyType::Bat | EnemyType::Demon | EnemyType::Dragon | EnemyType::DragonLord),
            is_flying: false,
        }
    }

    /// Take damage and return if enemy is still alive
    pub fn take_damage(&mut self, damage: f32) -> bool {
        self.health = (self.health - damage).max(0.0);
        self.is_alive = self.health > 0.0;
        self.is_alive
    }

    /// Heal the enemy
    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    /// Get health percentage
    pub fn health_percentage(&self) -> f32 {
        self.health / self.max_health
    }

    /// Check if enemy can see target
    pub fn can_see_target(&self, target_position: Vec2, current_position: Vec2) -> bool {
        let distance = current_position.distance(target_position);
        distance <= self.detection_range
    }

    /// Check if enemy can attack target
    pub fn can_attack_target(&self, target_position: Vec2, current_position: Vec2) -> bool {
        let distance = current_position.distance(target_position);
        distance <= self.attack_range
    }

    /// Update AI state based on current situation
    pub fn update_ai_state(&mut self, target_position: Option<Vec2>, current_position: Vec2) {
        if let Some(target_pos) = target_position {
            if self.can_attack_target(target_pos, current_position) {
                self.ai_state = AIState::Attacking;
            } else if self.can_see_target(target_pos, current_position) {
                self.ai_state = AIState::Chasing;
                self.last_known_position = Some(target_pos);
            } else {
                self.ai_state = AIState::Searching;
            }
        } else {
            self.ai_state = AIState::Idle;
        }
    }
}

/// Enemy type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyType {
    // Basic enemies
    Goblin,
    Orc,
    Archer,
    
    // Specialized enemies
    Mage,
    Berserker,
    ShieldBearer,
    
    // Flying enemies
    Bat,
    Demon,
    Dragon,
    
    // Boss enemies
    GoblinKing,
    OrcWarlord,
    DarkMage,
    DragonLord,
}

/// AI state for enemy behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AIState {
    /// Enemy is idle and not aware of player
    Idle,
    /// Enemy is chasing the player
    Chasing,
    /// Enemy is attacking the player
    Attacking,
    /// Enemy is searching for the player
    Searching,
    /// Enemy is fleeing from the player
    Fleeing,
    /// Enemy is in a special state (e.g., casting spell)
    Special,
    /// Enemy is dead
    Dead,
}

/// Enemy spawn configuration
#[derive(Debug, Clone)]
pub struct EnemySpawn {
    /// Enemy type to spawn
    pub enemy_type: EnemyType,
    /// Spawn position
    pub position: Vec2,
    /// Enemy level
    pub level: u32,
    /// Spawn delay in seconds
    pub delay: f32,
    /// Whether this is a boss spawn
    pub is_boss: bool,
    /// Spawn conditions
    pub conditions: Vec<SpawnCondition>,
}

/// Spawn condition for enemies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnCondition {
    /// Spawn when player reaches certain level
    PlayerLevel(u32),
    /// Spawn when certain area is entered
    AreaEntered(String),
    /// Spawn when certain enemy is defeated
    EnemyDefeated(EnemyType),
    /// Spawn at specific time
    TimeOfDay(f32),
    /// Spawn randomly with probability
    Random(f32),
}

/// Enemy manager for handling enemy spawning and behavior
#[derive(Debug, Clone)]
pub struct EnemyManager {
    /// Active enemies
    pub enemies: HashMap<Entity, Enemy>,
    /// Enemy spawn points
    pub spawn_points: Vec<EnemySpawn>,
    /// Spawned enemies count
    pub spawned_count: u32,
    /// Maximum enemies allowed
    pub max_enemies: u32,
    /// Current difficulty level
    pub difficulty: f32,
    /// Enemy scaling factor
    pub scaling_factor: f32,
}

impl EnemyManager {
    /// Create a new enemy manager
    pub fn new() -> Self {
        Self {
            enemies: HashMap::new(),
            spawn_points: Vec::new(),
            spawned_count: 0,
            max_enemies: 50,
            difficulty: 1.0,
            scaling_factor: 1.0,
        }
    }

    /// Add an enemy spawn point
    pub fn add_spawn_point(&mut self, spawn: EnemySpawn) {
        self.spawn_points.push(spawn);
    }

    /// Spawn an enemy at a specific position
    pub fn spawn_enemy(&mut self, enemy_type: EnemyType, position: Vec2, level: u32) -> Option<Entity> {
        if self.spawned_count >= self.max_enemies {
            return None;
        }

        let mut enemy = Enemy::new(enemy_type, level);
        
        // Apply difficulty scaling
        enemy.health *= self.difficulty;
        enemy.max_health *= self.difficulty;
        enemy.movement_speed *= self.difficulty;
        enemy.attack_range *= self.difficulty;
        enemy.detection_range *= self.difficulty;

        // Create entity (this would be handled by the ECS system)
        // For now, we'll use a simple counter as entity ID
        let entity = self.spawned_count;
        
        self.enemies.insert(entity, enemy);
        self.spawned_count += 1;
        
        Some(entity)
    }

    /// Remove an enemy
    pub fn remove_enemy(&mut self, entity: Entity) -> Option<Enemy> {
        if let Some(enemy) = self.enemies.remove(&entity) {
            self.spawned_count = self.spawned_count.saturating_sub(1);
            Some(enemy)
        } else {
            None
        }
    }

    /// Get enemy by entity
    pub fn get_enemy(&self, entity: Entity) -> Option<&Enemy> {
        self.enemies.get(&entity)
    }

    /// Get mutable enemy by entity
    pub fn get_enemy_mut(&mut self, entity: Entity) -> Option<&mut Enemy> {
        self.enemies.get_mut(&entity)
    }

    /// Update all enemies
    pub fn update_enemies(&mut self, dt: f32, player_position: Vec2) {
        for (entity, enemy) in self.enemies.iter_mut() {
            enemy.update_ai_state(Some(player_position), Vec2::ZERO); // Position would come from transform component
        }
    }

    /// Set difficulty level
    pub fn set_difficulty(&mut self, difficulty: f32) {
        self.difficulty = difficulty.max(0.1);
        self.scaling_factor = 1.0 + (self.difficulty - 1.0) * 0.5;
    }

    /// Get total enemies count
    pub fn enemy_count(&self) -> usize {
        self.enemies.len()
    }

    /// Get alive enemies count
    pub fn alive_enemy_count(&self) -> usize {
        self.enemies.values().filter(|enemy| enemy.is_alive).count()
    }
}

impl Default for EnemyManager {
    fn default() -> Self {
        Self::new()
    }
}
