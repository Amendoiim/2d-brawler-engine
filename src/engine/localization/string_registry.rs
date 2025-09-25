//! Comprehensive String ID Registry
//! 
//! This module contains all string IDs used throughout the game with their
//! character limits and metadata for proper localization.

use super::StringId;
use super::string_id::{StringCategory, StringIdMetadata};
use std::collections::HashMap;

/// Comprehensive string ID registry with character limits
pub struct GameStringRegistry {
    /// All registered string IDs with metadata
    registry: HashMap<StringId, StringIdMetadata>,
}

impl GameStringRegistry {
    /// Create a new game string registry with all string IDs
    pub fn new() -> Self {
        let mut registry = HashMap::new();
        
        // Register all UI strings
        Self::register_ui_strings(&mut registry);
        
        // Register all gameplay strings
        Self::register_gameplay_strings(&mut registry);
        
        // Register all character strings
        Self::register_character_strings(&mut registry);
        
        // Register all item strings
        Self::register_item_strings(&mut registry);
        
        // Register all combat strings
        Self::register_combat_strings(&mut registry);
        
        // Register all enemy strings
        Self::register_enemy_strings(&mut registry);
        
        // Register all level strings
        Self::register_level_strings(&mut registry);
        
        // Register all tutorial strings
        Self::register_tutorial_strings(&mut registry);
        
        // Register all error strings
        Self::register_error_strings(&mut registry);
        
        // Register all achievement strings
        Self::register_achievement_strings(&mut registry);
        
        // Register all settings strings
        Self::register_settings_strings(&mut registry);
        
        Self { registry }
    }
    
    /// Get string metadata by ID
    pub fn get_metadata(&self, id: &StringId) -> Option<&StringIdMetadata> {
        self.registry.get(id)
    }
    
    /// Get all string IDs by category
    pub fn get_strings_by_category(&self, category: &StringCategory) -> Vec<&StringId> {
        self.registry
            .iter()
            .filter(|(_, metadata)| &metadata.category == category)
            .map(|(id, _)| id)
            .collect()
    }
    
    /// Get all registered string IDs
    pub fn get_all_strings(&self) -> Vec<&StringId> {
        self.registry.keys().collect()
    }
    
    /// Register UI strings
    fn register_ui_strings(registry: &mut HashMap<StringId, StringIdMetadata>) {
        let ui_strings = vec![
            // Main Menu
            ("ui.main_menu", "Main Menu", 20, "Main menu title"),
            ("ui.play", "Play", 10, "Play button"),
            ("ui.new_game", "New Game", 15, "New game button"),
            ("ui.continue", "Continue", 15, "Continue button"),
            ("ui.load_game", "Load Game", 15, "Load game button"),
            ("ui.settings", "Settings", 15, "Settings button"),
            ("ui.quit", "Quit", 10, "Quit button"),
            ("ui.exit", "Exit", 10, "Exit button"),
            
            // Game Menu
            ("ui.pause", "Pause", 10, "Pause button"),
            ("ui.resume", "Resume", 15, "Resume button"),
            ("ui.restart", "Restart", 15, "Restart button"),
            ("ui.save_game", "Save Game", 15, "Save game button"),
            ("ui.return_to_menu", "Return to Menu", 25, "Return to main menu"),
            ("ui.return_to_game", "Return to Game", 25, "Return to game"),
            
            // Navigation
            ("ui.back", "Back", 10, "Back button"),
            ("ui.next", "Next", 10, "Next button"),
            ("ui.previous", "Previous", 15, "Previous button"),
            ("ui.cancel", "Cancel", 15, "Cancel button"),
            ("ui.confirm", "Confirm", 15, "Confirm button"),
            ("ui.save", "Save", 10, "Save button"),
            ("ui.load", "Load", 10, "Load button"),
            ("ui.delete", "Delete", 10, "Delete button"),
            ("ui.edit", "Edit", 10, "Edit button"),
            ("ui.create", "Create", 10, "Create button"),
            
            // Character Selection
            ("ui.character_selection", "Character Selection", 30, "Character selection title"),
            ("ui.create_character", "Create Character", 25, "Create character button"),
            ("ui.select_character", "Select Character", 25, "Select character button"),
            ("ui.delete_character", "Delete Character", 25, "Delete character button"),
            ("ui.character_name", "Character Name", 20, "Character name label"),
            ("ui.character_class", "Character Class", 20, "Character class label"),
            ("ui.character_level", "Level", 10, "Character level label"),
            ("ui.character_stats", "Stats", 10, "Character stats label"),
            ("ui.character_equipment", "Equipment", 15, "Character equipment label"),
            ("ui.character_abilities", "Abilities", 15, "Character abilities label"),
            
            // Inventory & Equipment
            ("ui.inventory", "Inventory", 15, "Inventory title"),
            ("ui.equipment", "Equipment", 15, "Equipment title"),
            ("ui.items", "Items", 10, "Items label"),
            ("ui.weapons", "Weapons", 15, "Weapons label"),
            ("ui.armor", "Armor", 10, "Armor label"),
            ("ui.accessories", "Accessories", 20, "Accessories label"),
            ("ui.consumables", "Consumables", 20, "Consumables label"),
            ("ui.equip", "Equip", 10, "Equip button"),
            ("ui.unequip", "Unequip", 15, "Unequip button"),
            ("ui.use_item", "Use Item", 15, "Use item button"),
            ("ui.drop_item", "Drop Item", 15, "Drop item button"),
            ("ui.item_info", "Item Info", 15, "Item info label"),
            ("ui.item_stats", "Item Stats", 15, "Item stats label"),
            ("ui.item_effects", "Item Effects", 20, "Item effects label"),
            ("ui.item_requirements", "Requirements", 20, "Item requirements label"),
        ];
        
        for (id, default_text, max_length, description) in ui_strings {
            let string_id = StringId::new(id);
            let metadata = StringIdMetadata {
                id: string_id.clone(),
                category: StringCategory::UI,
                description: Some(description.to_string()),
                context: Some("UI Element".to_string()),
                has_plural: false,
                max_length: Some(max_length),
                can_be_empty: false,
            };
            registry.insert(string_id, metadata);
        }
    }
    
    /// Register gameplay strings
    fn register_gameplay_strings(registry: &mut HashMap<StringId, StringIdMetadata>) {
        let gameplay_strings = vec![
            // Core Stats
            ("gameplay.level", "Level", 10, "Level label"),
            ("gameplay.score", "Score", 10, "Score label"),
            ("gameplay.time", "Time", 10, "Time label"),
            ("gameplay.health", "Health", 10, "Health label"),
            ("gameplay.mana", "Mana", 10, "Mana label"),
            ("gameplay.stamina", "Stamina", 15, "Stamina label"),
            ("gameplay.experience", "Experience", 15, "Experience label"),
            ("gameplay.gold", "Gold", 10, "Gold label"),
            ("gameplay.silver", "Silver", 10, "Silver label"),
            ("gameplay.copper", "Copper", 10, "Copper label"),
            
            // Character Stats
            ("gameplay.strength", "Strength", 15, "Strength stat"),
            ("gameplay.agility", "Agility", 15, "Agility stat"),
            ("gameplay.intelligence", "Intelligence", 20, "Intelligence stat"),
            ("gameplay.vitality", "Vitality", 15, "Vitality stat"),
            ("gameplay.wisdom", "Wisdom", 15, "Wisdom stat"),
            ("gameplay.charisma", "Charisma", 15, "Charisma stat"),
            ("gameplay.luck", "Luck", 10, "Luck stat"),
            ("gameplay.attack_power", "Attack Power", 20, "Attack power stat"),
            ("gameplay.defense", "Defense", 15, "Defense stat"),
            ("gameplay.magic_power", "Magic Power", 20, "Magic power stat"),
            ("gameplay.magic_resistance", "Magic Resistance", 25, "Magic resistance stat"),
            ("gameplay.critical_chance", "Critical Chance", 25, "Critical chance stat"),
            ("gameplay.critical_damage", "Critical Damage", 25, "Critical damage stat"),
            ("gameplay.movement_speed", "Movement Speed", 25, "Movement speed stat"),
            ("gameplay.attack_speed", "Attack Speed", 20, "Attack speed stat"),
        ];
        
        for (id, default_text, max_length, description) in gameplay_strings {
            let string_id = StringId::new(id);
            let metadata = StringIdMetadata {
                id: string_id.clone(),
                category: StringCategory::Gameplay,
                description: Some(description.to_string()),
                context: Some("Gameplay Element".to_string()),
                has_plural: false,
                max_length: Some(max_length),
                can_be_empty: false,
            };
            registry.insert(string_id, metadata);
        }
    }
    
    /// Register character strings
    fn register_character_strings(registry: &mut HashMap<StringId, StringIdMetadata>) {
        let character_strings = vec![
            // Character Classes
            ("characters.fighter", "Fighter", 15, "Fighter class name"),
            ("characters.ranger", "Ranger", 15, "Ranger class name"),
            ("characters.mage", "Mage", 10, "Mage class name"),
            ("characters.tank", "Tank", 10, "Tank class name"),
            ("characters.assassin", "Assassin", 20, "Assassin class name"),
            
            // Character Descriptions
            ("characters.fighter_desc", "A skilled warrior trained in combat", 50, "Fighter description"),
            ("characters.ranger_desc", "A master of the bow and wilderness", 50, "Ranger description"),
            ("characters.mage_desc", "A scholar of the arcane arts", 40, "Mage description"),
            ("characters.tank_desc", "A massive warrior clad in heavy armor", 50, "Tank description"),
            ("characters.assassin_desc", "A stealthy killer from the shadows", 50, "Assassin description"),
            
            // Character Templates
            ("characters.warrior_template", "Warrior", 15, "Warrior template name"),
            ("characters.forest_guardian", "Forest Guardian", 25, "Forest Guardian template"),
            ("characters.arcane_scholar", "Arcane Scholar", 25, "Arcane Scholar template"),
            ("characters.iron_guardian", "Iron Guardian", 25, "Iron Guardian template"),
            ("characters.shadow_blade", "Shadow Blade", 25, "Shadow Blade template"),
            
            // Character Customization
            ("characters.customization", "Character Customization", 30, "Character customization title"),
            ("characters.appearance", "Appearance", 15, "Appearance section"),
            ("characters.equipment", "Equipment", 15, "Equipment section"),
            ("characters.stats", "Stats", 10, "Stats section"),
            ("characters.abilities", "Abilities", 15, "Abilities section"),
        ];
        
        for (id, default_text, max_length, description) in character_strings {
            let string_id = StringId::new(id);
            let metadata = StringIdMetadata {
                id: string_id.clone(),
                category: StringCategory::Characters,
                description: Some(description.to_string()),
                context: Some("Character Element".to_string()),
                has_plural: false,
                max_length: Some(max_length),
                can_be_empty: false,
            };
            registry.insert(string_id, metadata);
        }
    }
    
    /// Register item strings
    fn register_item_strings(registry: &mut HashMap<StringId, StringIdMetadata>) {
        let item_strings = vec![
            // Item Types
            ("items.weapon", "Weapon", 15, "Weapon type"),
            ("items.armor", "Armor", 10, "Armor type"),
            ("items.accessory", "Accessory", 20, "Accessory type"),
            ("items.consumable", "Consumable", 20, "Consumable type"),
            ("items.material", "Material", 15, "Material type"),
            ("items.quest", "Quest Item", 20, "Quest item type"),
            ("items.misc", "Miscellaneous", 20, "Miscellaneous type"),
            
            // Item Rarity
            ("items.common", "Common", 10, "Common rarity"),
            ("items.uncommon", "Uncommon", 15, "Uncommon rarity"),
            ("items.rare", "Rare", 10, "Rare rarity"),
            ("items.epic", "Epic", 10, "Epic rarity"),
            ("items.legendary", "Legendary", 15, "Legendary rarity"),
            ("items.artifact", "Artifact", 15, "Artifact rarity"),
            
            // Weapon Types
            ("items.sword", "Sword", 10, "Sword weapon type"),
            ("items.axe", "Axe", 10, "Axe weapon type"),
            ("items.mace", "Mace", 10, "Mace weapon type"),
            ("items.dagger", "Dagger", 15, "Dagger weapon type"),
            ("items.bow", "Bow", 10, "Bow weapon type"),
            ("items.crossbow", "Crossbow", 15, "Crossbow weapon type"),
            ("items.staff", "Staff", 10, "Staff weapon type"),
            ("items.wand", "Wand", 10, "Wand weapon type"),
            ("items.spear", "Spear", 10, "Spear weapon type"),
            ("items.hammer", "Hammer", 15, "Hammer weapon type"),
            
            // Armor Types
            ("items.helmet", "Helmet", 15, "Helmet armor type"),
            ("items.chestplate", "Chestplate", 20, "Chestplate armor type"),
            ("items.leggings", "Leggings", 15, "Leggings armor type"),
            ("items.boots", "Boots", 10, "Boots armor type"),
            ("items.gloves", "Gloves", 10, "Gloves armor type"),
            ("items.shield", "Shield", 15, "Shield armor type"),
            ("items.cape", "Cape", 10, "Cape armor type"),
            ("items.belt", "Belt", 10, "Belt armor type"),
            
            // Consumable Items
            ("items.minor_health_potion", "Minor Health Potion", 30, "Minor health potion"),
            ("items.health_potion", "Health Potion", 25, "Health potion"),
            ("items.greater_health_potion", "Greater Health Potion", 35, "Greater health potion"),
            ("items.mana_potion", "Mana Potion", 25, "Mana potion"),
            ("items.stamina_potion", "Stamina Potion", 25, "Stamina potion"),
            ("items.strength_potion", "Potion of Strength", 30, "Strength potion"),
            ("items.speed_potion", "Potion of Speed", 30, "Speed potion"),
            ("items.bread", "Bread", 10, "Bread consumable"),
            ("items.teleport_scroll", "Scroll of Teleportation", 35, "Teleport scroll"),
            
            // Item Properties
            ("items.damage", "Damage", 10, "Damage property"),
            ("items.armor_value", "Armor Value", 20, "Armor value property"),
            ("items.durability", "Durability", 15, "Durability property"),
            ("items.weight", "Weight", 10, "Weight property"),
            ("items.value", "Value", 10, "Value property"),
            ("items.quality", "Quality", 10, "Quality property"),
        ];
        
        for (id, default_text, max_length, description) in item_strings {
            let string_id = StringId::new(id);
            let metadata = StringIdMetadata {
                id: string_id.clone(),
                category: StringCategory::Items,
                description: Some(description.to_string()),
                context: Some("Item Element".to_string()),
                has_plural: false,
                max_length: Some(max_length),
                can_be_empty: false,
            };
            registry.insert(string_id, metadata);
        }
    }
    
    /// Register combat strings
    fn register_combat_strings(registry: &mut HashMap<StringId, StringIdMetadata>) {
        let combat_strings = vec![
            // Combat Actions
            ("combat.attack", "Attack", 10, "Attack action"),
            ("combat.defend", "Defend", 10, "Defend action"),
            ("combat.dodge", "Dodge", 10, "Dodge action"),
            ("combat.block", "Block", 10, "Block action"),
            ("combat.parry", "Parry", 10, "Parry action"),
            ("combat.cast_spell", "Cast Spell", 20, "Cast spell action"),
            ("combat.use_item", "Use Item", 15, "Use item action"),
            ("combat.special_move", "Special Move", 20, "Special move action"),
            ("combat.ultimate", "Ultimate", 15, "Ultimate ability"),
            
            // Combat Results
            ("combat.critical_hit", "Critical Hit!", 20, "Critical hit message"),
            ("combat.miss", "Miss", 10, "Miss message"),
            ("combat.damage", "Damage", 10, "Damage label"),
            ("combat.heal", "Heal", 10, "Heal label"),
            ("combat.death", "Death", 10, "Death message"),
            ("combat.victory", "Victory!", 15, "Victory message"),
            ("combat.defeat", "Defeat", 10, "Defeat message"),
            ("combat.level_up", "Level Up!", 15, "Level up message"),
            ("combat.combo", "Combo!", 10, "Combo message"),
            ("combat.perfect", "Perfect!", 15, "Perfect timing message"),
            
            // Combat Stats
            ("combat.attack_power", "Attack Power", 20, "Attack power stat"),
            ("combat.defense", "Defense", 15, "Defense stat"),
            ("combat.critical_chance", "Critical Chance", 25, "Critical chance stat"),
            ("combat.critical_damage", "Critical Damage", 25, "Critical damage stat"),
            ("combat.accuracy", "Accuracy", 15, "Accuracy stat"),
            ("combat.evasion", "Evasion", 15, "Evasion stat"),
        ];
        
        for (id, default_text, max_length, description) in combat_strings {
            let string_id = StringId::new(id);
            let metadata = StringIdMetadata {
                id: string_id.clone(),
                category: StringCategory::Combat,
                description: Some(description.to_string()),
                context: Some("Combat Element".to_string()),
                has_plural: false,
                max_length: Some(max_length),
                can_be_empty: false,
            };
            registry.insert(string_id, metadata);
        }
    }
    
    /// Register enemy strings
    fn register_enemy_strings(registry: &mut HashMap<StringId, StringIdMetadata>) {
        let enemy_strings = vec![
            // Basic Enemies
            ("enemies.goblin", "Goblin", 15, "Goblin enemy"),
            ("enemies.orc", "Orc", 10, "Orc enemy"),
            ("enemies.skeleton", "Skeleton", 15, "Skeleton enemy"),
            ("enemies.zombie", "Zombie", 15, "Zombie enemy"),
            ("enemies.spider", "Giant Spider", 20, "Giant spider enemy"),
            ("enemies.wolf", "Dire Wolf", 15, "Dire wolf enemy"),
            ("enemies.bandit", "Bandit", 15, "Bandit enemy"),
            ("enemies.cultist", "Cultist", 15, "Cultist enemy"),
            
            // Special Enemies
            ("enemies.goblin_mage", "Goblin Mage", 20, "Goblin mage enemy"),
            ("enemies.orc_berserker", "Orc Berserker", 25, "Orc berserker enemy"),
            ("enemies.shield_bearer", "Shield Bearer", 25, "Shield bearer enemy"),
            ("enemies.dark_mage", "Dark Mage", 20, "Dark mage enemy"),
            ("enemies.necromancer", "Necromancer", 25, "Necromancer enemy"),
            
            // Boss Enemies
            ("enemies.goblin_king", "Goblin King", 20, "Goblin King boss"),
            ("enemies.orc_warlord", "Orc Warlord", 25, "Orc Warlord boss"),
            ("enemies.dragon_lord", "Dragon Lord", 25, "Dragon Lord boss"),
            ("enemies.ancient_lich", "Ancient Lich", 25, "Ancient Lich boss"),
            ("enemies.demon_prince", "Demon Prince", 25, "Demon Prince boss"),
        ];
        
        for (id, default_text, max_length, description) in enemy_strings {
            let string_id = StringId::new(id);
            let metadata = StringIdMetadata {
                id: string_id.clone(),
                category: StringCategory::Enemies,
                description: Some(description.to_string()),
                context: Some("Enemy Element".to_string()),
                has_plural: false,
                max_length: Some(max_length),
                can_be_empty: false,
            };
            registry.insert(string_id, metadata);
        }
    }
    
    /// Register level strings
    fn register_level_strings(registry: &mut HashMap<StringId, StringIdMetadata>) {
        let level_strings = vec![
            // Level Types
            ("levels.forest_clearing", "Forest Clearing", 25, "Forest clearing level"),
            ("levels.dark_cave", "Dark Cave", 20, "Dark cave level"),
            ("levels.mountain_pass", "Mountain Pass", 25, "Mountain pass level"),
            ("levels.ancient_ruins", "Ancient Ruins", 25, "Ancient ruins level"),
            ("levels.castle_courtyard", "Castle Courtyard", 30, "Castle courtyard level"),
            ("levels.dungeon_depths", "Dungeon Depths", 25, "Dungeon depths level"),
            ("levels.volcanic_chamber", "Volcanic Chamber", 30, "Volcanic chamber level"),
            ("levels.ice_cavern", "Ice Cavern", 20, "Ice cavern level"),
            
            // Level Properties
            ("levels.difficulty", "Difficulty", 15, "Level difficulty"),
            ("levels.biome", "Biome", 10, "Level biome"),
            ("levels.rooms", "Rooms", 10, "Number of rooms"),
            ("levels.connections", "Connections", 20, "Number of connections"),
            ("levels.spawn_points", "Spawn Points", 25, "Number of spawn points"),
        ];
        
        for (id, default_text, max_length, description) in level_strings {
            let string_id = StringId::new(id);
            let metadata = StringIdMetadata {
                id: string_id.clone(),
                category: StringCategory::Levels,
                description: Some(description.to_string()),
                context: Some("Level Element".to_string()),
                has_plural: false,
                max_length: Some(max_length),
                can_be_empty: false,
            };
            registry.insert(string_id, metadata);
        }
    }
    
    /// Register tutorial strings
    fn register_tutorial_strings(registry: &mut HashMap<StringId, StringIdMetadata>) {
        let tutorial_strings = vec![
            // Tutorial Steps
            ("tutorial.welcome", "Welcome to the 2D Brawler!", 35, "Welcome message"),
            ("tutorial.movement", "Use WASD to move your character", 45, "Movement tutorial"),
            ("tutorial.combat", "Click to attack enemies", 35, "Combat tutorial"),
            ("tutorial.inventory", "Press I to open your inventory", 40, "Inventory tutorial"),
            ("tutorial.skills", "Press S to view your skills", 35, "Skills tutorial"),
            ("tutorial.abilities", "Press A to view your abilities", 40, "Abilities tutorial"),
            ("tutorial.leveling", "Gain experience to level up", 40, "Leveling tutorial"),
            ("tutorial.equipment", "Equip items to improve your stats", 45, "Equipment tutorial"),
            ("tutorial.saving", "Press F5 to save your game", 35, "Saving tutorial"),
            ("tutorial.loading", "Press F9 to load your game", 35, "Loading tutorial"),
        ];
        
        for (id, default_text, max_length, description) in tutorial_strings {
            let string_id = StringId::new(id);
            let metadata = StringIdMetadata {
                id: string_id.clone(),
                category: StringCategory::Tutorial,
                description: Some(description.to_string()),
                context: Some("Tutorial Element".to_string()),
                has_plural: false,
                max_length: Some(max_length),
                can_be_empty: false,
            };
            registry.insert(string_id, metadata);
        }
    }
    
    /// Register error strings
    fn register_error_strings(registry: &mut HashMap<StringId, StringIdMetadata>) {
        let error_strings = vec![
            // System Errors
            ("errors.generic", "An error occurred", 25, "Generic error message"),
            ("errors.save_failed", "Failed to save game", 30, "Save failed error"),
            ("errors.load_failed", "Failed to load game", 30, "Load failed error"),
            ("errors.network", "Network connection failed", 35, "Network error"),
            ("errors.invalid_input", "Invalid input provided", 35, "Invalid input error"),
            ("errors.file_not_found", "File not found", 25, "File not found error"),
            ("errors.permission_denied", "Permission denied", 25, "Permission denied error"),
            ("errors.out_of_memory", "Out of memory", 20, "Out of memory error"),
            
            // Game Errors
            ("errors.inventory_full", "Inventory is full", 25, "Inventory full error"),
            ("errors.insufficient_funds", "Insufficient funds", 25, "Insufficient funds error"),
            ("errors.item_not_found", "Item not found", 25, "Item not found error"),
            ("errors.character_not_found", "Character not found", 30, "Character not found error"),
            ("errors.level_not_found", "Level not found", 25, "Level not found error"),
            ("errors.ability_not_found", "Ability not found", 25, "Ability not found error"),
            ("errors.equipment_slot_occupied", "Equipment slot is occupied", 35, "Equipment slot occupied error"),
            ("errors.insufficient_level", "Insufficient level", 25, "Insufficient level error"),
        ];
        
        for (id, default_text, max_length, description) in error_strings {
            let string_id = StringId::new(id);
            let metadata = StringIdMetadata {
                id: string_id.clone(),
                category: StringCategory::Errors,
                description: Some(description.to_string()),
                context: Some("Error Message".to_string()),
                has_plural: false,
                max_length: Some(max_length),
                can_be_empty: false,
            };
            registry.insert(string_id, metadata);
        }
    }
    
    /// Register achievement strings
    fn register_achievement_strings(registry: &mut HashMap<StringId, StringIdMetadata>) {
        let achievement_strings = vec![
            // Achievement Names
            ("achievements.first_kill", "First Blood", 20, "First enemy kill"),
            ("achievements.level_10", "Rising Star", 20, "Reach level 10"),
            ("achievements.level_25", "Veteran", 15, "Reach level 25"),
            ("achievements.level_50", "Master", 15, "Reach level 50"),
            ("achievements.combo_10", "Combo Master", 25, "10-hit combo"),
            ("achievements.perfect_dodge", "Dodge Master", 25, "Perfect dodge"),
            ("achievements.collector", "Item Collector", 25, "Collect 100 items"),
            ("achievements.explorer", "Explorer", 15, "Visit all levels"),
            ("achievements.boss_killer", "Boss Slayer", 25, "Defeat all bosses"),
            ("achievements.perfectionist", "Perfectionist", 25, "Complete without dying"),
        ];
        
        for (id, default_text, max_length, description) in achievement_strings {
            let string_id = StringId::new(id);
            let metadata = StringIdMetadata {
                id: string_id.clone(),
                category: StringCategory::Achievements,
                description: Some(description.to_string()),
                context: Some("Achievement Element".to_string()),
                has_plural: false,
                max_length: Some(max_length),
                can_be_empty: false,
            };
            registry.insert(string_id, metadata);
        }
    }
    
    /// Register settings strings
    fn register_settings_strings(registry: &mut HashMap<StringId, StringIdMetadata>) {
        let settings_strings = vec![
            // Settings Categories
            ("settings.graphics", "Graphics Settings", 25, "Graphics settings title"),
            ("settings.audio", "Audio Settings", 20, "Audio settings title"),
            ("settings.controls", "Control Settings", 25, "Control settings title"),
            ("settings.language", "Language", 15, "Language setting"),
            ("settings.resolution", "Resolution", 15, "Resolution setting"),
            ("settings.fullscreen", "Fullscreen", 15, "Fullscreen setting"),
            ("settings.vsync", "VSync", 10, "VSync setting"),
            ("settings.quality", "Quality", 10, "Quality setting"),
            
            // Volume Settings
            ("settings.volume_master", "Master Volume", 20, "Master volume setting"),
            ("settings.volume_music", "Music Volume", 20, "Music volume setting"),
            ("settings.volume_sfx", "SFX Volume", 20, "SFX volume setting"),
            ("settings.volume_voice", "Voice Volume", 20, "Voice volume setting"),
            
            // Graphics Settings
            ("settings.anti_aliasing", "Anti-Aliasing", 20, "Anti-aliasing setting"),
            ("settings.shadows", "Shadows", 15, "Shadows setting"),
            ("settings.particles", "Particles", 15, "Particles setting"),
            ("settings.texture_quality", "Texture Quality", 25, "Texture quality setting"),
            ("settings.effects_quality", "Effects Quality", 25, "Effects quality setting"),
        ];
        
        for (id, default_text, max_length, description) in settings_strings {
            let string_id = StringId::new(id);
            let metadata = StringIdMetadata {
                id: string_id.clone(),
                category: StringCategory::Settings,
                description: Some(description.to_string()),
                context: Some("Settings Element".to_string()),
                has_plural: false,
                max_length: Some(max_length),
                can_be_empty: false,
            };
            registry.insert(string_id, metadata);
        }
    }
}

impl Default for GameStringRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_registry_creation() {
        let registry = GameStringRegistry::new();
        assert!(!registry.get_all_strings().is_empty());
    }
    
    #[test]
    fn test_string_metadata() {
        let registry = GameStringRegistry::new();
        let ui_strings = registry.get_strings_by_category(&StringCategory::UI);
        assert!(!ui_strings.is_empty());
        
        let main_menu_id = StringId::new("ui.main_menu");
        let metadata = registry.get_metadata(&main_menu_id);
        assert!(metadata.is_some());
        
        let metadata = metadata.unwrap();
        assert_eq!(metadata.category, StringCategory::UI);
        assert_eq!(metadata.max_length, Some(20));
    }
    
    #[test]
    fn test_character_limits() {
        let registry = GameStringRegistry::new();
        
        // Test UI string character limits
        let ui_strings = registry.get_strings_by_category(&StringCategory::UI);
        for string_id in ui_strings {
            let metadata = registry.get_metadata(string_id).unwrap();
            assert!(metadata.max_length.is_some());
            assert!(metadata.max_length.unwrap() <= 50); // UI strings should be reasonable length
        }
    }
}
