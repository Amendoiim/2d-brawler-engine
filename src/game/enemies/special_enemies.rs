//! Specialized enemy types for the 2D Brawler Engine
//! 
//! This module defines specialized enemy types with advanced AI behaviors:
//! - Mage: Magic attacks, teleportation, area effects
//! - Berserker: Rage mode, increased damage, reduced defense
//! - Shield Bearer: Defensive, group protection, shield mechanics

use super::{Enemy, EnemyType, AIState};
use crate::engine::ecs::Entity;
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Mage enemy - Magic attacks, teleportation, area effects
#[derive(Debug, Clone)]
pub struct Mage {
    /// Base enemy data
    pub enemy: Enemy,
    /// Mana pool
    pub mana: f32,
    /// Maximum mana
    pub max_mana: f32,
    /// Mana regeneration rate
    pub mana_regen: f32,
    /// Current spell being cast
    pub current_spell: Option<SpellType>,
    /// Casting time
    pub casting_time: f32,
    /// Is casting
    pub is_casting: bool,
    /// Teleport cooldown
    pub teleport_cooldown: f32,
    /// Last teleport time
    pub last_teleport_time: f32,
    /// Spell range
    pub spell_range: f32,
}

/// Spell types for mage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellType {
    Fireball,
    Teleport,
    AreaExplosion,
    Shield,
    Heal,
}

impl Mage {
    /// Create a new mage
    pub fn new(level: u32) -> Self {
        let mut enemy = Enemy::new(EnemyType::Mage, level);
        enemy.movement_speed = 100.0;
        enemy.attack_range = 180.0;
        enemy.detection_range = 200.0;

        Self {
            enemy,
            mana: 100.0,
            max_mana: 100.0,
            mana_regen: 10.0,
            current_spell: None,
            casting_time: 0.0,
            is_casting: false,
            teleport_cooldown: 5.0,
            last_teleport_time: 0.0,
            spell_range: 180.0,
        }
    }

    /// Update mage behavior
    pub fn update(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        self.last_teleport_time += dt;
        
        // Regenerate mana
        self.mana = (self.mana + self.mana_regen * dt).min(self.max_mana);
        
        // Update AI state
        self.enemy.update_ai_state(Some(player_position), current_position);
        
        let distance_to_player = current_position.distance(player_position);
        
        // Mage-specific behavior
        match self.enemy.ai_state {
            AIState::Chasing => {
                // Mages prefer to maintain distance
                if distance_to_player < 100.0 {
                    self.enemy.ai_state = AIState::Fleeing;
                } else if distance_to_player > self.spell_range {
                    self.enemy.ai_state = AIState::Chasing;
                } else {
                    self.enemy.ai_state = AIState::Attacking;
                }
            }
            AIState::Attacking => {
                if !self.is_casting {
                    self.choose_spell(distance_to_player);
                } else {
                    self.casting_time += dt;
                    if self.casting_time >= self.get_spell_cast_time() {
                        self.cast_spell(player_position, current_position);
                    }
                }
            }
            AIState::Fleeing => {
                // Use teleport to escape if available
                if self.can_teleport() && distance_to_player < 80.0 {
                    self.teleport_away(current_position);
                }
            }
            _ => {}
        }
    }

    /// Choose appropriate spell
    fn choose_spell(&mut self, distance: f32) {
        if self.mana < 20.0 {
            return; // Not enough mana
        }

        if distance > 150.0 {
            self.current_spell = Some(SpellType::Fireball);
        } else if distance < 80.0 && self.can_teleport() {
            self.current_spell = Some(SpellType::Teleport);
        } else if distance < 120.0 {
            self.current_spell = Some(SpellType::AreaExplosion);
        } else {
            self.current_spell = Some(SpellType::Fireball);
        }

        self.is_casting = true;
        self.casting_time = 0.0;
    }

    /// Cast the current spell
    fn cast_spell(&mut self, target_position: Vec2, current_position: Vec2) {
        if let Some(spell) = self.current_spell {
            let mana_cost = self.get_spell_mana_cost(spell);
            if self.mana >= mana_cost {
                self.mana -= mana_cost;
                
                match spell {
                    SpellType::Fireball => {
                        println!("Mage casts fireball at target!");
                    }
                    SpellType::Teleport => {
                        self.teleport_away(current_position);
                    }
                    SpellType::AreaExplosion => {
                        println!("Mage casts area explosion!");
                    }
                    SpellType::Shield => {
                        println!("Mage casts protective shield!");
                    }
                    SpellType::Heal => {
                        self.enemy.heal(50.0);
                        println!("Mage heals self!");
                    }
                }
            }
        }
        
        self.is_casting = false;
        self.casting_time = 0.0;
        self.current_spell = None;
    }

    /// Teleport away from danger
    fn teleport_away(&mut self, current_position: Vec2) {
        self.last_teleport_time = 0.0;
        println!("Mage teleports away!");
    }

    /// Check if teleport is available
    fn can_teleport(&self) -> bool {
        self.last_teleport_time >= self.teleport_cooldown && self.mana >= 30.0
    }

    /// Get spell cast time
    fn get_spell_cast_time(&self) -> f32 {
        match self.current_spell {
            Some(SpellType::Fireball) => 1.0,
            Some(SpellType::Teleport) => 0.5,
            Some(SpellType::AreaExplosion) => 2.0,
            Some(SpellType::Shield) => 1.5,
            Some(SpellType::Heal) => 2.5,
            None => 0.0,
        }
    }

    /// Get spell mana cost
    fn get_spell_mana_cost(&self, spell: SpellType) -> f32 {
        match spell {
            SpellType::Fireball => 20.0,
            SpellType::Teleport => 30.0,
            SpellType::AreaExplosion => 40.0,
            SpellType::Shield => 25.0,
            SpellType::Heal => 35.0,
        }
    }
}

/// Berserker enemy - Rage mode, increased damage, reduced defense
#[derive(Debug, Clone)]
pub struct Berserker {
    /// Base enemy data
    pub enemy: Enemy,
    /// Rage level (0.0 to 1.0)
    pub rage_level: f32,
    /// Rage build rate
    pub rage_build_rate: f32,
    /// Rage decay rate
    pub rage_decay_rate: f32,
    /// Is in rage mode
    pub is_raging: bool,
    /// Rage duration
    pub rage_duration: f32,
    /// Rage cooldown
    pub rage_cooldown: f32,
    /// Last rage time
    pub last_rage_time: f32,
    /// Combo counter
    pub combo_count: u32,
    /// Last combo time
    pub last_combo_time: f32,
}

impl Berserker {
    /// Create a new berserker
    pub fn new(level: u32) -> Self {
        let mut enemy = Enemy::new(EnemyType::Berserker, level);
        enemy.movement_speed = 90.0;
        enemy.attack_range = 90.0;
        enemy.detection_range = 120.0;

        Self {
            enemy,
            rage_level: 0.0,
            rage_build_rate: 0.3,
            rage_decay_rate: 0.1,
            is_raging: false,
            rage_duration: 10.0,
            rage_cooldown: 15.0,
            last_rage_time: 0.0,
            combo_count: 0,
            last_combo_time: 0.0,
        }
    }

    /// Update berserker behavior
    pub fn update(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        self.last_rage_time += dt;
        self.last_combo_time += dt;
        
        // Update AI state
        self.enemy.update_ai_state(Some(player_position), current_position);
        
        // Update rage
        if self.is_raging {
            self.rage_duration -= dt;
            if self.rage_duration <= 0.0 {
                self.end_rage();
            }
        } else {
            // Build rage when in combat
            if self.enemy.ai_state == AIState::Attacking || self.enemy.ai_state == AIState::Chasing {
                self.rage_level = (self.rage_level + self.rage_build_rate * dt).min(1.0);
            } else {
                self.rage_level = (self.rage_level - self.rage_decay_rate * dt).max(0.0);
            }
            
            // Enter rage mode when rage is full
            if self.rage_level >= 1.0 && self.can_enter_rage() {
                self.enter_rage();
            }
        }
        
        // Berserker-specific behavior
        match self.enemy.ai_state {
            AIState::Attacking => {
                if self.is_raging {
                    self.perform_rage_attack();
                } else {
                    self.perform_normal_attack();
                }
            }
            AIState::Chasing => {
                // Berserkers are more aggressive when raging
                if self.is_raging {
                    self.enemy.movement_speed = 120.0;
                } else {
                    self.enemy.movement_speed = 90.0;
                }
            }
            _ => {}
        }
    }

    /// Enter rage mode
    fn enter_rage(&mut self) {
        self.is_raging = true;
        self.rage_duration = 10.0;
        self.last_rage_time = 0.0;
        self.rage_level = 1.0;
        
        // Rage bonuses
        self.enemy.movement_speed *= 1.3;
        self.enemy.attack_range *= 1.1;
        
        println!("Berserker enters rage mode!");
    }

    /// End rage mode
    fn end_rage(&mut self) {
        self.is_raging = false;
        self.rage_level = 0.0;
        self.combo_count = 0;
        
        // Reset bonuses
        self.enemy.movement_speed = 90.0;
        self.enemy.attack_range = 90.0;
        
        println!("Berserker exits rage mode!");
    }

    /// Check if can enter rage
    fn can_enter_rage(&self) -> bool {
        self.last_rage_time >= self.rage_cooldown
    }

    /// Perform rage attack
    fn perform_rage_attack(&mut self) {
        self.combo_count += 1;
        self.last_combo_time = 0.0;
        
        let damage_multiplier = 1.0 + (self.combo_count as f32 * 0.2);
        println!("Berserker performs rage attack #{} with {}x damage!", 
                self.combo_count, damage_multiplier);
    }

    /// Perform normal attack
    fn perform_normal_attack(&mut self) {
        println!("Berserker performs normal attack!");
    }

    /// Get rage damage modifier
    pub fn get_rage_damage_modifier(&self) -> f32 {
        if self.is_raging {
            1.0 + self.rage_level + (self.combo_count as f32 * 0.2)
        } else {
            1.0 + self.rage_level * 0.3
        }
    }
}

/// Shield Bearer enemy - Defensive, group protection, shield mechanics
#[derive(Debug, Clone)]
pub struct ShieldBearer {
    /// Base enemy data
    pub enemy: Enemy,
    /// Shield health
    pub shield_health: f32,
    /// Maximum shield health
    pub max_shield_health: f32,
    /// Shield regeneration rate
    pub shield_regen: f32,
    /// Shield regeneration delay
    pub shield_regen_delay: f32,
    /// Last damage time
    pub last_damage_time: f32,
    /// Is in defensive stance
    pub is_defensive: bool,
    /// Defensive stance duration
    pub defensive_duration: f32,
    /// Shield bash cooldown
    pub shield_bash_cooldown: f32,
    /// Last shield bash time
    pub last_shield_bash_time: f32,
    /// Protected allies
    pub protected_allies: Vec<Entity>,
}

impl ShieldBearer {
    /// Create a new shield bearer
    pub fn new(level: u32) -> Self {
        let mut enemy = Enemy::new(EnemyType::ShieldBearer, level);
        enemy.movement_speed = 70.0;
        enemy.attack_range = 80.0;
        enemy.detection_range = 150.0;

        Self {
            enemy,
            shield_health: 100.0,
            max_shield_health: 100.0,
            shield_regen: 5.0,
            shield_regen_delay: 3.0,
            last_damage_time: 0.0,
            is_defensive: false,
            defensive_duration: 0.0,
            shield_bash_cooldown: 4.0,
            last_shield_bash_time: 0.0,
            protected_allies: Vec::new(),
        }
    }

    /// Update shield bearer behavior
    pub fn update(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        self.last_shield_bash_time += dt;
        self.last_damage_time += dt;
        
        // Update AI state
        self.enemy.update_ai_state(Some(player_position), current_position);
        
        // Regenerate shield
        if self.last_damage_time >= self.shield_regen_delay {
            self.shield_health = (self.shield_health + self.shield_regen * dt).min(self.max_shield_health);
        }
        
        // Update defensive stance
        if self.is_defensive {
            self.defensive_duration -= dt;
            if self.defensive_duration <= 0.0 {
                self.end_defensive_stance();
            }
        }
        
        // Shield bearer-specific behavior
        match self.enemy.ai_state {
            AIState::Chasing => {
                // Enter defensive stance when approaching
                let distance = current_position.distance(player_position);
                if distance < 120.0 && !self.is_defensive {
                    self.enter_defensive_stance();
                }
            }
            AIState::Attacking => {
                if self.last_shield_bash_time >= self.shield_bash_cooldown {
                    self.perform_shield_bash();
                } else {
                    self.perform_normal_attack();
                }
            }
            AIState::Fleeing => {
                // Shield bearers rarely flee, only when shield is broken
                if self.shield_health > 0.0 {
                    self.enemy.ai_state = AIState::Chasing;
                }
            }
            _ => {}
        }
    }

    /// Enter defensive stance
    fn enter_defensive_stance(&mut self) {
        self.is_defensive = true;
        self.defensive_duration = 5.0;
        
        // Defensive bonuses
        self.enemy.movement_speed *= 0.7;
        
        println!("Shield Bearer enters defensive stance!");
    }

    /// End defensive stance
    fn end_defensive_stance(&mut self) {
        self.is_defensive = false;
        self.defensive_duration = 0.0;
        
        // Reset movement speed
        self.enemy.movement_speed = 70.0;
        
        println!("Shield Bearer exits defensive stance!");
    }

    /// Perform shield bash
    fn perform_shield_bash(&mut self) {
        self.last_shield_bash_time = 0.0;
        println!("Shield Bearer performs shield bash!");
    }

    /// Perform normal attack
    fn perform_normal_attack(&mut self) {
        println!("Shield Bearer performs normal attack!");
    }

    /// Take damage with shield protection
    pub fn take_damage_with_shield(&mut self, damage: f32) -> f32 {
        self.last_damage_time = 0.0;
        
        if self.shield_health > 0.0 {
            let shield_damage = damage.min(self.shield_health);
            self.shield_health -= shield_damage;
            let remaining_damage = damage - shield_damage;
            
            if remaining_damage > 0.0 {
                self.enemy.take_damage(remaining_damage);
            }
            
            remaining_damage
        } else {
            self.enemy.take_damage(damage);
            damage
        }
    }

    /// Get shield protection modifier
    pub fn get_shield_protection(&self) -> f32 {
        if self.is_defensive {
            0.5 // 50% damage reduction
        } else {
            0.8 // 20% damage reduction
        }
    }

    /// Check if shield is active
    pub fn has_shield(&self) -> bool {
        self.shield_health > 0.0
    }
}

/// Special enemy factory for creating specialized enemies
pub struct SpecialEnemyFactory;

impl SpecialEnemyFactory {
    /// Create a mage
    pub fn create_mage(level: u32) -> Mage {
        Mage::new(level)
    }

    /// Create a berserker
    pub fn create_berserker(level: u32) -> Berserker {
        Berserker::new(level)
    }

    /// Create a shield bearer
    pub fn create_shield_bearer(level: u32) -> ShieldBearer {
        ShieldBearer::new(level)
    }

    /// Create a special enemy by type
    pub fn create_enemy(enemy_type: EnemyType, level: u32) -> Enemy {
        match enemy_type {
            EnemyType::Mage => {
                let mage = Self::create_mage(level);
                mage.enemy
            }
            EnemyType::Berserker => {
                let berserker = Self::create_berserker(level);
                berserker.enemy
            }
            EnemyType::ShieldBearer => {
                let shield_bearer = Self::create_shield_bearer(level);
                shield_bearer.enemy
            }
            _ => Enemy::new(enemy_type, level),
        }
    }
}
