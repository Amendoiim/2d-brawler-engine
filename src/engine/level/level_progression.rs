//! Level progression system for managing level selection, checkpoints, and rewards

use crate::engine::level::{Level, LevelType, LevelConfig, SpawnPoint, SpawnType};
use glam::Vec2;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Level progression manager for handling level selection, checkpoints, and rewards
pub struct LevelProgressionManager {
    /// Available levels in the game
    levels: HashMap<String, LevelInfo>,
    /// Player's progress through levels
    player_progress: PlayerProgress,
    /// Checkpoint system
    checkpoint_system: CheckpointSystem,
    /// Reward system
    reward_system: RewardSystem,
    /// Level selection state
    level_selection: LevelSelection,
}

/// Information about a level
#[derive(Debug, Clone)]
pub struct LevelInfo {
    /// Level identifier
    pub id: String,
    /// Level name for display
    pub name: String,
    /// Level description
    pub description: String,
    /// Level type
    pub level_type: LevelType,
    /// Difficulty rating (1-10)
    pub difficulty: u32,
    /// Required level to unlock
    pub required_level: u32,
    /// Biome type
    pub biome: String,
    /// Level configuration
    pub config: LevelConfig,
    /// Whether the level is unlocked
    pub unlocked: bool,
    /// Whether the level has been completed
    pub completed: bool,
    /// Best completion time (in seconds)
    pub best_time: Option<f32>,
    /// Number of times completed
    pub completion_count: u32,
    /// Star rating (0-3)
    pub star_rating: u32,
    /// Level rewards
    pub rewards: Vec<LevelReward>,
}

/// Player's overall progress
#[derive(Debug, Clone)]
pub struct PlayerProgress {
    /// Current player level
    pub level: u32,
    /// Total experience points
    pub experience: u32,
    /// Experience needed for next level
    pub experience_to_next: u32,
    /// Total levels completed
    pub levels_completed: u32,
    /// Total play time (in seconds)
    pub total_play_time: f32,
    /// Unlocked achievements
    pub achievements: Vec<String>,
    /// Player statistics
    pub stats: PlayerStats,
}

/// Player statistics
#[derive(Debug, Clone)]
pub struct PlayerStats {
    /// Total enemies defeated
    pub enemies_defeated: u32,
    /// Total items collected
    pub items_collected: u32,
    /// Total deaths
    pub deaths: u32,
    /// Total checkpoints reached
    pub checkpoints_reached: u32,
    /// Best combo achieved
    pub best_combo: u32,
    /// Total damage dealt
    pub total_damage_dealt: u32,
    /// Total damage taken
    pub total_damage_taken: u32,
}

/// Checkpoint system for saving progress within levels
#[derive(Debug, Clone)]
pub struct CheckpointSystem {
    /// Active checkpoints in current level
    active_checkpoints: Vec<Checkpoint>,
    /// Last checkpoint reached
    last_checkpoint: Option<String>,
    /// Checkpoint data storage
    checkpoint_data: HashMap<String, CheckpointData>,
}

/// A checkpoint in a level
#[derive(Debug, Clone)]
pub struct Checkpoint {
    /// Checkpoint identifier
    pub id: String,
    /// Checkpoint position
    pub position: Vec2,
    /// Checkpoint type
    pub checkpoint_type: CheckpointType,
    /// Whether checkpoint is active
    pub active: bool,
    /// Whether checkpoint has been reached
    pub reached: bool,
    /// Time when checkpoint was reached
    pub reached_time: Option<f32>,
    /// Player state when checkpoint was reached
    pub player_state: Option<PlayerState>,
}

/// Types of checkpoints
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CheckpointType {
    /// Standard checkpoint
    Standard,
    /// Boss checkpoint (before boss fight)
    Boss,
    /// Secret checkpoint (hidden)
    Secret,
    /// Auto-save checkpoint
    AutoSave,
    /// Level start checkpoint
    Start,
    /// Level end checkpoint
    End,
}

/// Data stored at a checkpoint
#[derive(Debug, Clone)]
pub struct CheckpointData {
    /// Player position
    pub player_position: Vec2,
    /// Player health
    pub player_health: f32,
    /// Player experience
    pub player_experience: u32,
    /// Player level
    pub player_level: u32,
    /// Inventory items
    pub inventory: Vec<String>,
    /// Active abilities
    pub abilities: Vec<String>,
    /// Level state
    pub level_state: LevelState,
    /// Timestamp
    pub timestamp: f32,
}

/// Level state at checkpoint
#[derive(Debug, Clone)]
pub struct LevelState {
    /// Destroyed objects
    pub destroyed_objects: Vec<String>,
    /// Activated switches
    pub activated_switches: Vec<String>,
    /// Opened doors
    pub opened_doors: Vec<String>,
    /// Collected items
    pub collected_items: Vec<String>,
    /// Defeated enemies
    pub defeated_enemies: Vec<String>,
}

/// Player state snapshot
#[derive(Debug, Clone)]
pub struct PlayerState {
    /// Player position
    pub position: Vec2,
    /// Player health
    pub health: f32,
    /// Player experience
    pub experience: u32,
    /// Player level
    pub level: u32,
    /// Player inventory
    pub inventory: Vec<String>,
    /// Player abilities
    pub abilities: Vec<String>,
}

/// Reward system for level completion
#[derive(Debug, Clone)]
pub struct RewardSystem {
    /// Available rewards
    rewards: HashMap<String, LevelReward>,
    /// Reward templates
    reward_templates: HashMap<LevelType, Vec<RewardTemplate>>,
}

/// A reward for completing a level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelReward {
    /// Reward identifier
    pub id: String,
    /// Reward type
    pub reward_type: RewardType,
    /// Reward value
    pub value: u32,
    /// Reward description
    pub description: String,
    /// Whether reward has been claimed
    pub claimed: bool,
    /// Requirements to unlock reward
    pub requirements: Vec<RewardRequirement>,
}

/// Types of rewards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RewardType {
    /// Experience points
    Experience,
    /// Currency/coins
    Currency,
    /// Item
    Item,
    /// Ability unlock
    Ability,
    /// Achievement
    Achievement,
    /// Cosmetic unlock
    Cosmetic,
}

/// Requirements for unlocking rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardRequirement {
    /// Requirement type
    pub requirement_type: RequirementType,
    /// Required value
    pub required_value: u32,
    /// Current progress
    pub current_progress: u32,
}

/// Types of requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RequirementType {
    /// Complete level in time
    TimeLimit,
    /// Complete without dying
    NoDeath,
    /// Collect all items
    CollectAll,
    /// Defeat all enemies
    DefeatAll,
    /// Complete with specific rating
    StarRating,
    /// Complete multiple times
    CompletionCount,
}

/// Reward template for level types
#[derive(Debug, Clone)]
pub struct RewardTemplate {
    /// Reward type
    pub reward_type: RewardType,
    /// Base value
    pub base_value: u32,
    /// Difficulty multiplier
    pub difficulty_multiplier: f32,
    /// Probability of reward
    pub probability: f32,
    /// Requirements
    pub requirements: Vec<RewardRequirement>,
}

/// Level selection system
#[derive(Debug, Clone)]
pub struct LevelSelection {
    /// Available level categories
    categories: Vec<LevelCategory>,
    /// Current selected level
    selected_level: Option<String>,
    /// Current selected category
    selected_category: Option<String>,
    /// Level filtering options
    filters: LevelFilters,
}

/// Level category for organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelCategory {
    /// Category identifier
    pub id: String,
    /// Category name
    pub name: String,
    /// Category description
    pub description: String,
    /// Levels in this category
    pub levels: Vec<String>,
    /// Category icon
    pub icon: String,
    /// Category order
    pub order: u32,
}

/// Level filtering options
#[derive(Debug, Clone, Default)]
pub struct LevelFilters {
    /// Filter by difficulty
    pub difficulty_range: Option<(u32, u32)>,
    /// Filter by level type
    pub level_types: Vec<LevelType>,
    /// Filter by biome
    pub biomes: Vec<String>,
    /// Filter by completion status
    pub completion_status: Option<CompletionFilter>,
    /// Search text
    pub search_text: Option<String>,
}

/// Completion status filter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompletionFilter {
    /// All levels
    All,
    /// Completed levels only
    Completed,
    /// Uncompleted levels only
    Uncompleted,
    /// Unlocked levels only
    Unlocked,
    /// Locked levels only
    Locked,
}

impl LevelProgressionManager {
    /// Create a new level progression manager
    pub fn new() -> Self {
        let mut manager = Self {
            levels: HashMap::new(),
            player_progress: PlayerProgress::new(),
            checkpoint_system: CheckpointSystem::new(),
            reward_system: RewardSystem::new(),
            level_selection: LevelSelection::new(),
        };
        manager.initialize_default_levels();
        manager.initialize_reward_templates();
        manager.initialize_level_categories();
        manager
    }

    /// Initialize default levels
    fn initialize_default_levels(&mut self) {
        // Tutorial levels
        self.add_level(LevelInfo {
            id: "tutorial_1".to_string(),
            name: "First Steps".to_string(),
            description: "Learn the basics of movement and combat".to_string(),
            level_type: LevelType::Standard,
            difficulty: 1,
            required_level: 1,
            biome: "forest".to_string(),
            config: LevelConfig::default(),
            unlocked: true,
            completed: false,
            best_time: None,
            completion_count: 0,
            star_rating: 0,
            rewards: vec![],
        });

        // Combat arena levels
        self.add_level(LevelInfo {
            id: "combat_arena_1".to_string(),
            name: "Training Grounds".to_string(),
            description: "Practice your combat skills".to_string(),
            level_type: LevelType::CombatArena,
            difficulty: 2,
            required_level: 2,
            biome: "forest".to_string(),
            config: LevelConfig::default(),
            unlocked: false,
            completed: false,
            best_time: None,
            completion_count: 0,
            star_rating: 0,
            rewards: vec![],
        });

        // Platforming levels
        self.add_level(LevelInfo {
            id: "platforming_1".to_string(),
            name: "Leap of Faith".to_string(),
            description: "Test your jumping skills".to_string(),
            level_type: LevelType::Platforming,
            difficulty: 3,
            required_level: 3,
            biome: "desert".to_string(),
            config: LevelConfig::default(),
            unlocked: false,
            completed: false,
            best_time: None,
            completion_count: 0,
            star_rating: 0,
            rewards: vec![],
        });

        // Puzzle levels
        self.add_level(LevelInfo {
            id: "puzzle_1".to_string(),
            name: "Mind Bender".to_string(),
            description: "Solve the ancient puzzle".to_string(),
            level_type: LevelType::Puzzle,
            difficulty: 4,
            required_level: 4,
            biome: "cave".to_string(),
            config: LevelConfig::default(),
            unlocked: false,
            completed: false,
            best_time: None,
            completion_count: 0,
            star_rating: 0,
            rewards: vec![],
        });

        // Boss arena levels
        self.add_level(LevelInfo {
            id: "boss_1".to_string(),
            name: "Dragon's Lair".to_string(),
            description: "Face the mighty dragon".to_string(),
            level_type: LevelType::BossArena,
            difficulty: 5,
            required_level: 5,
            biome: "lava".to_string(),
            config: LevelConfig::default(),
            unlocked: false,
            completed: false,
            best_time: None,
            completion_count: 0,
            star_rating: 0,
            rewards: vec![],
        });
    }

    /// Initialize reward templates for different level types
    fn initialize_reward_templates(&mut self) {
        // Combat arena rewards
        self.reward_system.reward_templates.insert(
            LevelType::CombatArena,
            vec![
                RewardTemplate {
                    reward_type: RewardType::Experience,
                    base_value: 100,
                    difficulty_multiplier: 1.5,
                    probability: 1.0,
                    requirements: vec![],
                },
                RewardTemplate {
                    reward_type: RewardType::Currency,
                    base_value: 50,
                    difficulty_multiplier: 1.2,
                    probability: 0.8,
                    requirements: vec![],
                },
            ],
        );

        // Platforming rewards
        self.reward_system.reward_templates.insert(
            LevelType::Platforming,
            vec![
                RewardTemplate {
                    reward_type: RewardType::Experience,
                    base_value: 80,
                    difficulty_multiplier: 1.3,
                    probability: 1.0,
                    requirements: vec![],
                },
                RewardTemplate {
                    reward_type: RewardType::Ability,
                    base_value: 1,
                    difficulty_multiplier: 1.0,
                    probability: 0.3,
                    requirements: vec![RewardRequirement {
                        requirement_type: RequirementType::NoDeath,
                        required_value: 1,
                        current_progress: 0,
                    }],
                },
            ],
        );

        // Puzzle rewards
        self.reward_system.reward_templates.insert(
            LevelType::Puzzle,
            vec![
                RewardTemplate {
                    reward_type: RewardType::Experience,
                    base_value: 120,
                    difficulty_multiplier: 1.4,
                    probability: 1.0,
                    requirements: vec![],
                },
                RewardTemplate {
                    reward_type: RewardType::Item,
                    base_value: 1,
                    difficulty_multiplier: 1.0,
                    probability: 0.6,
                    requirements: vec![],
                },
            ],
        );

        // Boss arena rewards
        self.reward_system.reward_templates.insert(
            LevelType::BossArena,
            vec![
                RewardTemplate {
                    reward_type: RewardType::Experience,
                    base_value: 200,
                    difficulty_multiplier: 2.0,
                    probability: 1.0,
                    requirements: vec![],
                },
                RewardTemplate {
                    reward_type: RewardType::Currency,
                    base_value: 100,
                    difficulty_multiplier: 1.5,
                    probability: 1.0,
                    requirements: vec![],
                },
                RewardTemplate {
                    reward_type: RewardType::Achievement,
                    base_value: 1,
                    difficulty_multiplier: 1.0,
                    probability: 1.0,
                    requirements: vec![],
                },
            ],
        );
    }

    /// Initialize level categories
    fn initialize_level_categories(&mut self) {
        self.level_selection.categories = vec![
            LevelCategory {
                id: "tutorial".to_string(),
                name: "Tutorial".to_string(),
                description: "Learn the basics".to_string(),
                levels: vec!["tutorial_1".to_string()],
                icon: "tutorial_icon".to_string(),
                order: 1,
            },
            LevelCategory {
                id: "combat".to_string(),
                name: "Combat Arenas".to_string(),
                description: "Test your fighting skills".to_string(),
                levels: vec!["combat_arena_1".to_string()],
                icon: "combat_icon".to_string(),
                order: 2,
            },
            LevelCategory {
                id: "platforming".to_string(),
                name: "Platforming".to_string(),
                description: "Jump and climb your way through".to_string(),
                levels: vec!["platforming_1".to_string()],
                icon: "platforming_icon".to_string(),
                order: 3,
            },
            LevelCategory {
                id: "puzzle".to_string(),
                name: "Puzzles".to_string(),
                description: "Solve challenging puzzles".to_string(),
                levels: vec!["puzzle_1".to_string()],
                icon: "puzzle_icon".to_string(),
                order: 4,
            },
            LevelCategory {
                id: "boss".to_string(),
                name: "Boss Battles".to_string(),
                description: "Face powerful bosses".to_string(),
                levels: vec!["boss_1".to_string()],
                icon: "boss_icon".to_string(),
                order: 5,
            },
        ];
    }

    /// Add a level to the progression system
    pub fn add_level(&mut self, level_info: LevelInfo) {
        self.levels.insert(level_info.id.clone(), level_info);
    }

    /// Get level information
    pub fn get_level(&self, level_id: &str) -> Option<&LevelInfo> {
        self.levels.get(level_id)
    }

    /// Get all levels
    pub fn get_all_levels(&self) -> &HashMap<String, LevelInfo> {
        &self.levels
    }

    /// Get filtered levels
    pub fn get_filtered_levels(&self) -> Vec<&LevelInfo> {
        let mut filtered_levels: Vec<&LevelInfo> = self.levels.values().collect();

        // Apply filters
        if let Some((min, max)) = self.level_selection.filters.difficulty_range {
            filtered_levels.retain(|level| level.difficulty >= min && level.difficulty <= max);
        }

        if !self.level_selection.filters.level_types.is_empty() {
            filtered_levels.retain(|level| self.level_selection.filters.level_types.contains(&level.level_type));
        }

        if !self.level_selection.filters.biomes.is_empty() {
            filtered_levels.retain(|level| self.level_selection.filters.biomes.contains(&level.biome));
        }

        if let Some(completion_filter) = self.level_selection.filters.completion_status {
            match completion_filter {
                CompletionFilter::Completed => filtered_levels.retain(|level| level.completed),
                CompletionFilter::Uncompleted => filtered_levels.retain(|level| !level.completed),
                CompletionFilter::Unlocked => filtered_levels.retain(|level| level.unlocked),
                CompletionFilter::Locked => filtered_levels.retain(|level| !level.unlocked),
                CompletionFilter::All => {}
            }
        }

        if let Some(search_text) = &self.level_selection.filters.search_text {
            let search_lower = search_text.to_lowercase();
            filtered_levels.retain(|level| {
                level.name.to_lowercase().contains(&search_lower) ||
                level.description.to_lowercase().contains(&search_lower)
            });
        }

        filtered_levels
    }

    /// Complete a level
    pub fn complete_level(&mut self, level_id: &str, completion_time: f32, star_rating: u32) -> Result<Vec<LevelReward>, String> {
        // Generate rewards first to avoid borrowing conflicts
        let rewards = self.generate_level_rewards(level_id, star_rating)?;
        
        let level = self.levels.get_mut(level_id)
            .ok_or_else(|| format!("Level {} not found", level_id))?;

        // Update level completion
        level.completed = true;
        level.completion_count += 1;
        
        if level.best_time.is_none() || completion_time < level.best_time.unwrap() {
            level.best_time = Some(completion_time);
        }

        if star_rating > level.star_rating {
            level.star_rating = star_rating;
        }

        // Add rewards to level
        for reward in &rewards {
            level.rewards.push(reward.clone());
        }

        // Update player progress
        self.player_progress.levels_completed += 1;
        self.player_progress.total_play_time += completion_time;

        // Unlock next levels
        self.unlock_next_levels(level_id);

        Ok(rewards)
    }

    /// Generate rewards for level completion
    fn generate_level_rewards(&mut self, level_id: &str, star_rating: u32) -> Result<Vec<LevelReward>, String> {
        let level = self.levels.get(level_id)
            .ok_or_else(|| format!("Level {} not found", level_id))?;

        let templates = self.reward_system.reward_templates.get(&level.level_type)
            .ok_or_else(|| format!("No reward templates for level type {:?}", level.level_type))?;

        let mut rewards = Vec::new();
        let mut rng = fastrand::Rng::new();

        for template in templates {
            if rng.f32() <= template.probability {
                let value = (template.base_value as f32 * template.difficulty_multiplier * level.difficulty as f32) as u32;
                
                let reward = LevelReward {
                    id: format!("{}_{:?}_{}", level_id, template.reward_type, rng.u32(0..10000)),
                    reward_type: template.reward_type,
                    value,
                    description: format!("{:?} reward for completing {}", template.reward_type, level.name),
                    claimed: false,
                    requirements: template.requirements.clone(),
                };

                rewards.push(reward);
            }
        }

        Ok(rewards)
    }

    /// Unlock next levels based on completion
    fn unlock_next_levels(&mut self, completed_level_id: &str) {
        let completed_level = self.levels.get(completed_level_id).unwrap();
        
        for level in self.levels.values_mut() {
            if !level.unlocked && level.required_level <= self.player_progress.level {
                level.unlocked = true;
            }
        }
    }

    /// Add experience to player
    pub fn add_experience(&mut self, amount: u32) {
        self.player_progress.experience += amount;
        
        // Check for level up
        while self.player_progress.experience >= self.player_progress.experience_to_next {
            self.player_progress.experience -= self.player_progress.experience_to_next;
            self.player_progress.level += 1;
            self.player_progress.experience_to_next = self.calculate_experience_to_next();
            
            // Unlock levels based on new level
            self.unlock_levels_by_player_level();
        }
    }

    /// Calculate experience needed for next level
    fn calculate_experience_to_next(&self) -> u32 {
        // Simple exponential formula: base * level^1.5
        let base = 100;
        let level = self.player_progress.level as f32;
        (base as f32 * level.powf(1.5)) as u32
    }

    /// Unlock levels based on player level
    fn unlock_levels_by_player_level(&mut self) {
        for level in self.levels.values_mut() {
            if !level.unlocked && level.required_level <= self.player_progress.level {
                level.unlocked = true;
            }
        }
    }

    /// Create a checkpoint
    pub fn create_checkpoint(&mut self, checkpoint: Checkpoint) {
        self.checkpoint_system.active_checkpoints.push(checkpoint);
    }

    /// Reach a checkpoint
    pub fn reach_checkpoint(&mut self, checkpoint_id: &str, player_state: PlayerState) -> Result<(), String> {
        let checkpoint = self.checkpoint_system.active_checkpoints.iter_mut()
            .find(|c| c.id == checkpoint_id)
            .ok_or_else(|| format!("Checkpoint {} not found", checkpoint_id))?;

        checkpoint.reached = true;
        checkpoint.reached_time = Some(0.0); // TODO: Get actual time
        checkpoint.player_state = Some(player_state.clone());

        // Store checkpoint data
        let checkpoint_data = CheckpointData {
            player_position: player_state.position,
            player_health: player_state.health,
            player_experience: player_state.experience,
            player_level: player_state.level,
            inventory: player_state.inventory,
            abilities: player_state.abilities,
            level_state: LevelState::default(),
            timestamp: 0.0, // TODO: Get actual timestamp
        };

        self.checkpoint_system.checkpoint_data.insert(checkpoint_id.to_string(), checkpoint_data);
        self.checkpoint_system.last_checkpoint = Some(checkpoint_id.to_string());

        // Update player stats
        self.player_progress.stats.checkpoints_reached += 1;

        Ok(())
    }

    /// Get checkpoint data
    pub fn get_checkpoint_data(&self, checkpoint_id: &str) -> Option<&CheckpointData> {
        self.checkpoint_system.checkpoint_data.get(checkpoint_id)
    }

    /// Get last checkpoint
    pub fn get_last_checkpoint(&self) -> Option<&str> {
        self.checkpoint_system.last_checkpoint.as_deref()
    }

    /// Get player progress
    pub fn get_player_progress(&self) -> &PlayerProgress {
        &self.player_progress
    }

    /// Get level categories
    pub fn get_level_categories(&self) -> &Vec<LevelCategory> {
        &self.level_selection.categories
    }

    /// Set level filters
    pub fn set_level_filters(&mut self, filters: LevelFilters) {
        self.level_selection.filters = filters;
    }

    /// Select a level
    pub fn select_level(&mut self, level_id: &str) -> Result<(), String> {
        if !self.levels.contains_key(level_id) {
            return Err(format!("Level {} not found", level_id));
        }

        let level = self.levels.get(level_id).unwrap();
        if !level.unlocked {
            return Err(format!("Level {} is not unlocked", level_id));
        }

        self.level_selection.selected_level = Some(level_id.to_string());
        Ok(())
    }

    /// Get selected level
    pub fn get_selected_level(&self) -> Option<&str> {
        self.level_selection.selected_level.as_deref()
    }

    /// Claim a reward
    pub fn claim_reward(&mut self, level_id: &str, reward_id: &str) -> Result<(), String> {
        // Find the reward first
        let reward_value = {
            let level = self.levels.get(level_id)
                .ok_or_else(|| format!("Level {} not found", level_id))?;

            let reward = level.rewards.iter()
                .find(|r| r.id == reward_id)
                .ok_or_else(|| format!("Reward {} not found", reward_id))?;

            if reward.claimed {
                return Err("Reward already claimed".to_string());
            }

            // Check requirements
            for requirement in &reward.requirements {
                if requirement.current_progress < requirement.required_value {
                    return Err(format!("Requirement not met: {:?}", requirement.requirement_type));
                }
            }

            (reward.reward_type, reward.value, reward.description.clone())
        };

        // Apply reward
        match reward_value.0 {
            RewardType::Experience => {
                self.add_experience(reward_value.1);
            },
            RewardType::Currency => {
                // TODO: Add currency system
            },
            RewardType::Item => {
                // TODO: Add item to inventory
            },
            RewardType::Ability => {
                // TODO: Unlock ability
            },
            RewardType::Achievement => {
                self.player_progress.achievements.push(reward_value.2);
            },
            RewardType::Cosmetic => {
                // TODO: Unlock cosmetic
            },
        }

        // Mark reward as claimed
        let level = self.levels.get_mut(level_id).unwrap();
        let reward = level.rewards.iter_mut()
            .find(|r| r.id == reward_id).unwrap();
        reward.claimed = true;
        
        Ok(())
    }
}

impl Default for LevelProgressionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerProgress {
    fn new() -> Self {
        Self {
            level: 1,
            experience: 0,
            experience_to_next: 100,
            levels_completed: 0,
            total_play_time: 0.0,
            achievements: Vec::new(),
            stats: PlayerStats::new(),
        }
    }
}

impl PlayerStats {
    fn new() -> Self {
        Self {
            enemies_defeated: 0,
            items_collected: 0,
            deaths: 0,
            checkpoints_reached: 0,
            best_combo: 0,
            total_damage_dealt: 0,
            total_damage_taken: 0,
        }
    }
}

impl CheckpointSystem {
    fn new() -> Self {
        Self {
            active_checkpoints: Vec::new(),
            last_checkpoint: None,
            checkpoint_data: HashMap::new(),
        }
    }
}

impl RewardSystem {
    fn new() -> Self {
        Self {
            rewards: HashMap::new(),
            reward_templates: HashMap::new(),
        }
    }
}

impl LevelSelection {
    fn new() -> Self {
        Self {
            categories: Vec::new(),
            selected_level: None,
            selected_category: None,
            filters: LevelFilters::default(),
        }
    }
}

impl Default for LevelState {
    fn default() -> Self {
        Self {
            destroyed_objects: Vec::new(),
            activated_switches: Vec::new(),
            opened_doors: Vec::new(),
            collected_items: Vec::new(),
            defeated_enemies: Vec::new(),
        }
    }
}
