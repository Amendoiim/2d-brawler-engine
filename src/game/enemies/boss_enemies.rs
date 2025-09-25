//! Boss enemy types for the 2D Brawler Engine
//! 
//! This module defines boss enemy types with complex AI behaviors and multiple phases:
//! - Goblin King: Swarm summoning, area attacks
//! - Orc Warlord: Heavy attacks, ground pounds
//! - Dark Mage: Teleportation, spell combinations
//! - Dragon Lord: Flight phases, breath attacks

use super::{Enemy, EnemyType, AIState};
use crate::engine::ecs::Entity;
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Boss phase system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BossPhase {
    Phase1,
    Phase2,
    Phase3,
    Phase4,
    Final,
}

/// Boss state for complex behaviors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossState {
    /// Current phase
    pub current_phase: BossPhase,
    /// Phase health thresholds
    pub phase_thresholds: Vec<f32>,
    /// Phase transition time
    pub phase_transition_time: f32,
    /// Is transitioning phases
    pub is_transitioning: bool,
    /// Phase-specific abilities
    pub phase_abilities: Vec<String>,
    /// Phase cooldowns
    pub phase_cooldowns: std::collections::HashMap<String, f32>,
}

/// Goblin King boss - Swarm summoning, area attacks
#[derive(Debug, Clone)]
pub struct GoblinKing {
    /// Base enemy data
    pub enemy: Enemy,
    /// Boss state
    pub boss_state: BossState,
    /// Summoned goblins
    pub summoned_goblins: Vec<Entity>,
    /// Swarm attack cooldown
    pub swarm_attack_cooldown: f32,
    /// Last swarm attack time
    pub last_swarm_attack_time: f32,
    /// Area attack cooldown
    pub area_attack_cooldown: f32,
    /// Last area attack time
    pub last_area_attack_time: f32,
    /// Summon cooldown
    pub summon_cooldown: f32,
    /// Last summon time
    pub last_summon_time: f32,
    /// Maximum goblins to summon
    pub max_goblins: u32,
}

impl GoblinKing {
    /// Create a new Goblin King
    pub fn new(level: u32) -> Self {
        let mut enemy = Enemy::new(EnemyType::GoblinKing, level);
        enemy.movement_speed = 80.0;
        enemy.attack_range = 120.0;
        enemy.detection_range = 200.0;

        let boss_state = BossState {
            current_phase: BossPhase::Phase1,
            phase_thresholds: vec![0.8, 0.6, 0.4, 0.2],
            phase_transition_time: 0.0,
            is_transitioning: false,
            phase_abilities: vec!["summon_goblins".to_string(), "swarm_attack".to_string()],
            phase_cooldowns: std::collections::HashMap::new(),
        };

        Self {
            enemy,
            boss_state,
            summoned_goblins: Vec::new(),
            swarm_attack_cooldown: 3.0,
            last_swarm_attack_time: 0.0,
            area_attack_cooldown: 5.0,
            last_area_attack_time: 0.0,
            summon_cooldown: 8.0,
            last_summon_time: 0.0,
            max_goblins: 5,
        }
    }

    /// Update Goblin King behavior
    pub fn update(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        self.last_swarm_attack_time += dt;
        self.last_area_attack_time += dt;
        self.last_summon_time += dt;
        
        // Update AI state
        self.enemy.update_ai_state(Some(player_position), current_position);
        
        // Check for phase transitions
        self.check_phase_transition();
        
        // Update phase-specific behavior
        match self.boss_state.current_phase {
            BossPhase::Phase1 => self.update_phase1_behavior(dt, player_position, current_position),
            BossPhase::Phase2 => self.update_phase2_behavior(dt, player_position, current_position),
            BossPhase::Phase3 => self.update_phase3_behavior(dt, player_position, current_position),
            BossPhase::Phase4 => self.update_phase4_behavior(dt, player_position, current_position),
            BossPhase::Final => self.update_final_phase_behavior(dt, player_position, current_position),
        }
    }

    /// Check for phase transitions
    fn check_phase_transition(&mut self) {
        let health_percentage = self.enemy.health_percentage();
        
        for (i, threshold) in self.boss_state.phase_thresholds.iter().enumerate() {
            if health_percentage <= *threshold {
                let new_phase = match i {
                    0 => BossPhase::Phase2,
                    1 => BossPhase::Phase3,
                    2 => BossPhase::Phase4,
                    3 => BossPhase::Final,
                    _ => BossPhase::Final,
                };
                
                if self.boss_state.current_phase != new_phase {
                    self.transition_to_phase(new_phase);
                }
                break;
            }
        }
    }

    /// Transition to a new phase
    fn transition_to_phase(&mut self, new_phase: BossPhase) {
        self.boss_state.current_phase = new_phase;
        self.boss_state.is_transitioning = true;
        self.boss_state.phase_transition_time = 0.0;
        
        // Update abilities for new phase
        Self::update_phase_abilities();
        
        println!("Goblin King transitions to {:?}!", new_phase);
    }

    /// Update phase abilities
    fn update_phase_abilities() {
        // Phase-specific abilities would be updated here
    }

    /// Update Phase 1 behavior
    fn update_phase1_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // Basic attacks and occasional goblin summoning
        if self.last_summon_time >= self.summon_cooldown {
            self.summon_goblins(current_position);
        }
    }

    /// Update Phase 2 behavior
    fn update_phase2_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // More frequent summoning and swarm attacks
        if self.last_swarm_attack_time >= self.swarm_attack_cooldown {
            self.perform_swarm_attack(player_position);
        }
    }

    /// Update Phase 3 behavior
    fn update_phase3_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // Area attacks and increased aggression
        if self.last_area_attack_time >= self.area_attack_cooldown {
            self.perform_area_attack(current_position);
        }
    }

    /// Update Phase 4 behavior
    fn update_phase4_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // Desperate attacks and maximum goblin summoning
        self.max_goblins = 8;
        self.summon_cooldown = 4.0;
    }

    /// Update Final phase behavior
    fn update_final_phase_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // All abilities on reduced cooldown
        self.swarm_attack_cooldown = 1.5;
        self.area_attack_cooldown = 2.0;
        self.summon_cooldown = 2.0;
    }

    /// Summon goblins
    fn summon_goblins(&mut self, position: Vec2) {
        self.last_summon_time = 0.0;
        println!("Goblin King summons goblins!");
    }

    /// Perform swarm attack
    fn perform_swarm_attack(&mut self, target_position: Vec2) {
        self.last_swarm_attack_time = 0.0;
        println!("Goblin King performs swarm attack!");
    }

    /// Perform area attack
    fn perform_area_attack(&mut self, position: Vec2) {
        self.last_area_attack_time = 0.0;
        println!("Goblin King performs area attack!");
    }
}

/// Orc Warlord boss - Heavy attacks, ground pounds
#[derive(Debug, Clone)]
pub struct OrcWarlord {
    /// Base enemy data
    pub enemy: Enemy,
    /// Boss state
    pub boss_state: BossState,
    /// Ground pound cooldown
    pub ground_pound_cooldown: f32,
    /// Last ground pound time
    pub last_ground_pound_time: f32,
    /// Heavy attack cooldown
    pub heavy_attack_cooldown: f32,
    /// Last heavy attack time
    pub last_heavy_attack_time: f32,
    /// Rage mode
    pub rage_mode: bool,
    /// Rage duration
    pub rage_duration: f32,
    /// Rage cooldown
    pub rage_cooldown: f32,
    /// Last rage time
    pub last_rage_time: f32,
}

impl OrcWarlord {
    /// Create a new Orc Warlord
    pub fn new(level: u32) -> Self {
        let mut enemy = Enemy::new(EnemyType::OrcWarlord, level);
        enemy.movement_speed = 70.0;
        enemy.attack_range = 150.0;
        enemy.detection_range = 180.0;

        let boss_state = BossState {
            current_phase: BossPhase::Phase1,
            phase_thresholds: vec![0.75, 0.5, 0.25],
            phase_transition_time: 0.0,
            is_transitioning: false,
            phase_abilities: vec!["ground_pound".to_string(), "heavy_attack".to_string(), "rage_mode".to_string()],
            phase_cooldowns: std::collections::HashMap::new(),
        };

        Self {
            enemy,
            boss_state,
            ground_pound_cooldown: 6.0,
            last_ground_pound_time: 0.0,
            heavy_attack_cooldown: 4.0,
            last_heavy_attack_time: 0.0,
            rage_mode: false,
            rage_duration: 0.0,
            rage_cooldown: 15.0,
            last_rage_time: 0.0,
        }
    }

    /// Update Orc Warlord behavior
    pub fn update(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        self.last_ground_pound_time += dt;
        self.last_heavy_attack_time += dt;
        self.last_rage_time += dt;
        
        // Update AI state
        self.enemy.update_ai_state(Some(player_position), current_position);
        
        // Update rage mode
        if self.rage_mode {
            self.rage_duration -= dt;
            if self.rage_duration <= 0.0 {
                self.end_rage_mode();
            }
        }
        
        // Check for phase transitions
        self.check_phase_transition();
        
        // Update phase-specific behavior
        match self.boss_state.current_phase {
            BossPhase::Phase1 => self.update_phase1_behavior(dt, player_position, current_position),
            BossPhase::Phase2 => self.update_phase2_behavior(dt, player_position, current_position),
            BossPhase::Phase3 => self.update_phase3_behavior(dt, player_position, current_position),
            _ => self.update_final_phase_behavior(dt, player_position, current_position),
        }
    }

    /// Check for phase transitions
    fn check_phase_transition(&mut self) {
        let health_percentage = self.enemy.health_percentage();
        
        for (i, threshold) in self.boss_state.phase_thresholds.iter().enumerate() {
            if health_percentage <= *threshold {
                let new_phase = match i {
                    0 => BossPhase::Phase2,
                    1 => BossPhase::Phase3,
                    2 => BossPhase::Final,
                    _ => BossPhase::Final,
                };
                
                if self.boss_state.current_phase != new_phase {
                    self.transition_to_phase(new_phase);
                }
                break;
            }
        }
    }

    /// Transition to a new phase
    fn transition_to_phase(&mut self, new_phase: BossPhase) {
        self.boss_state.current_phase = new_phase;
        self.boss_state.is_transitioning = true;
        self.boss_state.phase_transition_time = 0.0;
        
        println!("Orc Warlord transitions to {:?}!", new_phase);
    }

    /// Update Phase 1 behavior
    fn update_phase1_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // Basic heavy attacks
        if self.last_heavy_attack_time >= self.heavy_attack_cooldown {
            self.perform_heavy_attack();
        }
    }

    /// Update Phase 2 behavior
    fn update_phase2_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // Ground pounds and heavy attacks
        if self.last_ground_pound_time >= self.ground_pound_cooldown {
            self.perform_ground_pound(current_position);
        }
    }

    /// Update Phase 3 behavior
    fn update_phase3_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // Rage mode activation
        if !self.rage_mode && self.last_rage_time >= self.rage_cooldown {
            self.enter_rage_mode();
        }
    }

    /// Update Final phase behavior
    fn update_final_phase_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // All abilities on reduced cooldown
        self.ground_pound_cooldown = 3.0;
        self.heavy_attack_cooldown = 2.0;
        self.rage_cooldown = 8.0;
    }

    /// Perform heavy attack
    fn perform_heavy_attack(&mut self) {
        self.last_heavy_attack_time = 0.0;
        println!("Orc Warlord performs heavy attack!");
    }

    /// Perform ground pound
    fn perform_ground_pound(&mut self, position: Vec2) {
        self.last_ground_pound_time = 0.0;
        println!("Orc Warlord performs ground pound!");
    }

    /// Enter rage mode
    fn enter_rage_mode(&mut self) {
        self.rage_mode = true;
        self.rage_duration = 10.0;
        self.last_rage_time = 0.0;
        
        // Rage bonuses
        self.enemy.movement_speed *= 1.5;
        self.enemy.attack_range *= 1.2;
        
        println!("Orc Warlord enters rage mode!");
    }

    /// End rage mode
    fn end_rage_mode(&mut self) {
        self.rage_mode = false;
        self.rage_duration = 0.0;
        
        // Reset bonuses
        self.enemy.movement_speed = 70.0;
        self.enemy.attack_range = 150.0;
        
        println!("Orc Warlord exits rage mode!");
    }
}

/// Dark Mage boss - Teleportation, spell combinations
#[derive(Debug, Clone)]
pub struct DarkMage {
    /// Base enemy data
    pub enemy: Enemy,
    /// Boss state
    pub boss_state: BossState,
    /// Mana pool
    pub mana: f32,
    /// Maximum mana
    pub max_mana: f32,
    /// Mana regeneration
    pub mana_regen: f32,
    /// Teleport cooldown
    pub teleport_cooldown: f32,
    /// Last teleport time
    pub last_teleport_time: f32,
    /// Spell combination cooldown
    pub spell_combo_cooldown: f32,
    /// Last spell combo time
    pub last_spell_combo_time: f32,
    /// Current spell sequence
    pub current_spell_sequence: Vec<String>,
    /// Spell casting time
    pub spell_casting_time: f32,
    /// Is casting
    pub is_casting: bool,
}

impl DarkMage {
    /// Create a new Dark Mage
    pub fn new(level: u32) -> Self {
        let mut enemy = Enemy::new(EnemyType::DarkMage, level);
        enemy.movement_speed = 90.0;
        enemy.attack_range = 200.0;
        enemy.detection_range = 250.0;

        let boss_state = BossState {
            current_phase: BossPhase::Phase1,
            phase_thresholds: vec![0.8, 0.6, 0.4, 0.2],
            phase_transition_time: 0.0,
            is_transitioning: false,
            phase_abilities: vec!["teleport".to_string(), "spell_combo".to_string(), "area_spell".to_string()],
            phase_cooldowns: std::collections::HashMap::new(),
        };

        Self {
            enemy,
            boss_state,
            mana: 150.0,
            max_mana: 150.0,
            mana_regen: 15.0,
            teleport_cooldown: 4.0,
            last_teleport_time: 0.0,
            spell_combo_cooldown: 8.0,
            last_spell_combo_time: 0.0,
            current_spell_sequence: Vec::new(),
            spell_casting_time: 0.0,
            is_casting: false,
        }
    }

    /// Update Dark Mage behavior
    pub fn update(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        self.last_teleport_time += dt;
        self.last_spell_combo_time += dt;
        
        // Regenerate mana
        self.mana = (self.mana + self.mana_regen * dt).min(self.max_mana);
        
        // Update AI state
        self.enemy.update_ai_state(Some(player_position), current_position);
        
        // Update spell casting
        if self.is_casting {
            self.spell_casting_time += dt;
            if self.spell_casting_time >= 2.0 {
                self.complete_spell_cast();
            }
        }
        
        // Check for phase transitions
        self.check_phase_transition();
        
        // Update phase-specific behavior
        match self.boss_state.current_phase {
            BossPhase::Phase1 => self.update_phase1_behavior(dt, player_position, current_position),
            BossPhase::Phase2 => self.update_phase2_behavior(dt, player_position, current_position),
            BossPhase::Phase3 => self.update_phase3_behavior(dt, player_position, current_position),
            BossPhase::Phase4 => self.update_phase4_behavior(dt, player_position, current_position),
            BossPhase::Final => self.update_final_phase_behavior(dt, player_position, current_position),
        }
    }

    /// Check for phase transitions
    fn check_phase_transition(&mut self) {
        let health_percentage = self.enemy.health_percentage();
        
        for (i, threshold) in self.boss_state.phase_thresholds.iter().enumerate() {
            if health_percentage <= *threshold {
                let new_phase = match i {
                    0 => BossPhase::Phase2,
                    1 => BossPhase::Phase3,
                    2 => BossPhase::Phase4,
                    3 => BossPhase::Final,
                    _ => BossPhase::Final,
                };
                
                if self.boss_state.current_phase != new_phase {
                    self.transition_to_phase(new_phase);
                }
                break;
            }
        }
    }

    /// Transition to a new phase
    fn transition_to_phase(&mut self, new_phase: BossPhase) {
        self.boss_state.current_phase = new_phase;
        self.boss_state.is_transitioning = true;
        self.boss_state.phase_transition_time = 0.0;
        
        println!("Dark Mage transitions to {:?}!", new_phase);
    }

    /// Update Phase 1 behavior
    fn update_phase1_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // Basic teleportation and single spells
        if self.last_teleport_time >= self.teleport_cooldown {
            self.teleport_away(current_position);
        }
    }

    /// Update Phase 2 behavior
    fn update_phase2_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // Spell combinations
        if self.last_spell_combo_time >= self.spell_combo_cooldown && !self.is_casting {
            self.start_spell_combo();
        }
    }

    /// Update Phase 3 behavior
    fn update_phase3_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // More frequent teleportation and complex spell sequences
        self.teleport_cooldown = 2.0;
        self.spell_combo_cooldown = 5.0;
    }

    /// Update Phase 4 behavior
    fn update_phase4_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // Maximum spell power and frequency
        self.mana_regen = 25.0;
        self.max_mana = 200.0;
    }

    /// Update Final phase behavior
    fn update_final_phase_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // All abilities on minimal cooldown
        self.teleport_cooldown = 1.0;
        self.spell_combo_cooldown = 2.0;
    }

    /// Teleport away from danger
    fn teleport_away(&mut self, current_position: Vec2) {
        self.last_teleport_time = 0.0;
        println!("Dark Mage teleports away!");
    }

    /// Start spell combination
    fn start_spell_combo(&mut self) {
        self.last_spell_combo_time = 0.0;
        self.is_casting = true;
        self.spell_casting_time = 0.0;
        self.current_spell_sequence = vec!["fireball".to_string(), "ice_shard".to_string(), "lightning".to_string()];
        println!("Dark Mage starts spell combination!");
    }

    /// Complete spell cast
    fn complete_spell_cast(&mut self) {
        self.is_casting = false;
        self.spell_casting_time = 0.0;
        println!("Dark Mage completes spell combination!");
    }
}

/// Dragon Lord boss - Flight phases, breath attacks
#[derive(Debug, Clone)]
pub struct DragonLord {
    /// Base enemy data
    pub enemy: Enemy,
    /// Boss state
    pub boss_state: BossState,
    /// Flight phase
    pub flight_phase: FlightPhase,
    /// Flight duration
    pub flight_duration: f32,
    /// Ground duration
    pub ground_duration: f32,
    /// Breath attack cooldown
    pub breath_attack_cooldown: f32,
    /// Last breath attack time
    pub last_breath_attack_time: f32,
    /// Roar cooldown
    pub roar_cooldown: f32,
    /// Last roar time
    pub last_roar_time: f32,
    /// Flight height
    pub flight_height: f32,
    /// Is flying
    pub is_flying: bool,
}

/// Flight phases for Dragon Lord
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlightPhase {
    Ground,
    LowFlight,
    HighFlight,
    Dive,
}

impl DragonLord {
    /// Create a new Dragon Lord
    pub fn new(level: u32) -> Self {
        let mut enemy = Enemy::new(EnemyType::DragonLord, level);
        enemy.movement_speed = 100.0;
        enemy.attack_range = 250.0;
        enemy.detection_range = 300.0;
        enemy.can_fly = true;

        let boss_state = BossState {
            current_phase: BossPhase::Phase1,
            phase_thresholds: vec![0.8, 0.6, 0.4, 0.2],
            phase_transition_time: 0.0,
            is_transitioning: false,
            phase_abilities: vec!["fire_breath".to_string(), "flight".to_string(), "roar".to_string(), "dive_attack".to_string()],
            phase_cooldowns: std::collections::HashMap::new(),
        };

        Self {
            enemy,
            boss_state,
            flight_phase: FlightPhase::Ground,
            flight_duration: 0.0,
            ground_duration: 0.0,
            breath_attack_cooldown: 5.0,
            last_breath_attack_time: 0.0,
            roar_cooldown: 8.0,
            last_roar_time: 0.0,
            flight_height: 0.0,
            is_flying: false,
        }
    }

    /// Update Dragon Lord behavior
    pub fn update(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        self.last_breath_attack_time += dt;
        self.last_roar_time += dt;
        
        // Update AI state
        self.enemy.update_ai_state(Some(player_position), current_position);
        
        // Update flight phases
        self.update_flight_phase(dt);
        
        // Check for phase transitions
        self.check_phase_transition();
        
        // Update phase-specific behavior
        match self.boss_state.current_phase {
            BossPhase::Phase1 => self.update_phase1_behavior(dt, player_position, current_position),
            BossPhase::Phase2 => self.update_phase2_behavior(dt, player_position, current_position),
            BossPhase::Phase3 => self.update_phase3_behavior(dt, player_position, current_position),
            BossPhase::Phase4 => self.update_phase4_behavior(dt, player_position, current_position),
            BossPhase::Final => self.update_final_phase_behavior(dt, player_position, current_position),
        }
    }

    /// Update flight phase
    fn update_flight_phase(&mut self, dt: f32) {
        match self.flight_phase {
            FlightPhase::Ground => {
                self.ground_duration += dt;
                if self.ground_duration >= 8.0 {
                    self.transition_to_flight(FlightPhase::LowFlight);
                }
            }
            FlightPhase::LowFlight => {
                self.flight_duration += dt;
                if self.flight_duration >= 6.0 {
                    self.transition_to_flight(FlightPhase::HighFlight);
                }
            }
            FlightPhase::HighFlight => {
                self.flight_duration += dt;
                if self.flight_duration >= 4.0 {
                    self.transition_to_flight(FlightPhase::Dive);
                }
            }
            FlightPhase::Dive => {
                self.flight_duration += dt;
                if self.flight_duration >= 2.0 {
                    self.transition_to_flight(FlightPhase::Ground);
                }
            }
        }
    }

    /// Transition to flight phase
    fn transition_to_flight(&mut self, new_phase: FlightPhase) {
        self.flight_phase = new_phase;
        self.flight_duration = 0.0;
        self.ground_duration = 0.0;
        
        match new_phase {
            FlightPhase::Ground => {
                self.is_flying = false;
                self.flight_height = 0.0;
            }
            FlightPhase::LowFlight => {
                self.is_flying = true;
                self.flight_height = 50.0;
            }
            FlightPhase::HighFlight => {
                self.is_flying = true;
                self.flight_height = 100.0;
            }
            FlightPhase::Dive => {
                self.is_flying = true;
                self.flight_height = 20.0;
            }
        }
        
        println!("Dragon Lord transitions to {:?}!", new_phase);
    }

    /// Check for phase transitions
    fn check_phase_transition(&mut self) {
        let health_percentage = self.enemy.health_percentage();
        
        for (i, threshold) in self.boss_state.phase_thresholds.iter().enumerate() {
            if health_percentage <= *threshold {
                let new_phase = match i {
                    0 => BossPhase::Phase2,
                    1 => BossPhase::Phase3,
                    2 => BossPhase::Phase4,
                    3 => BossPhase::Final,
                    _ => BossPhase::Final,
                };
                
                if self.boss_state.current_phase != new_phase {
                    self.transition_to_phase(new_phase);
                }
                break;
            }
        }
    }

    /// Transition to a new phase
    fn transition_to_phase(&mut self, new_phase: BossPhase) {
        self.boss_state.current_phase = new_phase;
        self.boss_state.is_transitioning = true;
        self.boss_state.phase_transition_time = 0.0;
        
        println!("Dragon Lord transitions to {:?}!", new_phase);
    }

    /// Update Phase 1 behavior
    fn update_phase1_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // Basic ground attacks and occasional flight
        if self.last_breath_attack_time >= self.breath_attack_cooldown {
            self.perform_breath_attack();
        }
    }

    /// Update Phase 2 behavior
    fn update_phase2_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // More frequent flight and breath attacks
        self.breath_attack_cooldown = 3.0;
    }

    /// Update Phase 3 behavior
    fn update_phase3_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // Roar attacks and dive attacks
        if self.last_roar_time >= self.roar_cooldown {
            self.perform_roar();
        }
    }

    /// Update Phase 4 behavior
    fn update_phase4_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // Increased flight frequency and power
        self.breath_attack_cooldown = 2.0;
        self.roar_cooldown = 4.0;
    }

    /// Update Final phase behavior
    fn update_final_phase_behavior(&mut self, dt: f32, player_position: Vec2, current_position: Vec2) {
        // All abilities on minimal cooldown
        self.breath_attack_cooldown = 1.0;
        self.roar_cooldown = 2.0;
    }

    /// Perform breath attack
    fn perform_breath_attack(&mut self) {
        self.last_breath_attack_time = 0.0;
        println!("Dragon Lord performs fire breath attack!");
    }

    /// Perform roar
    fn perform_roar(&mut self) {
        self.last_roar_time = 0.0;
        println!("Dragon Lord roars with terrifying power!");
    }
}

/// Boss enemy factory for creating boss enemies
pub struct BossEnemyFactory;

impl BossEnemyFactory {
    /// Create a Goblin King
    pub fn create_goblin_king(level: u32) -> GoblinKing {
        GoblinKing::new(level)
    }

    /// Create an Orc Warlord
    pub fn create_orc_warlord(level: u32) -> OrcWarlord {
        OrcWarlord::new(level)
    }

    /// Create a Dark Mage
    pub fn create_dark_mage(level: u32) -> DarkMage {
        DarkMage::new(level)
    }

    /// Create a Dragon Lord
    pub fn create_dragon_lord(level: u32) -> DragonLord {
        DragonLord::new(level)
    }

    /// Create a boss enemy by type
    pub fn create_enemy(enemy_type: EnemyType, level: u32) -> Enemy {
        match enemy_type {
            EnemyType::GoblinKing => {
                let goblin_king = Self::create_goblin_king(level);
                goblin_king.enemy
            }
            EnemyType::OrcWarlord => {
                let orc_warlord = Self::create_orc_warlord(level);
                orc_warlord.enemy
            }
            EnemyType::DarkMage => {
                let dark_mage = Self::create_dark_mage(level);
                dark_mage.enemy
            }
            EnemyType::DragonLord => {
                let dragon_lord = Self::create_dragon_lord(level);
                dragon_lord.enemy
            }
            _ => Enemy::new(enemy_type, level),
        }
    }
}
