# Localization Implementation Plan

**Created:** September 25, 2025  
**Last Updated:** September 25, 2025  
**Status:** Comprehensive localization system with advanced audio features

## Overview
This document outlines the comprehensive localization implementation plan for the 2D Brawler Engine, including all string IDs, character limits for UI elements, and implementation guidelines.

## Target Languages
- **FIGS**: French, Italian, German, Spanish
- **BRPT**: Brazilian Portuguese
- **KO**: Korean
- **JP**: Japanese
- **Chinese Simplified**: 简体中文
- **Chinese Traditional**: 繁體中文
- **Turkish**: Türkçe
- **Arabic**: العربية

## String ID Categories and Character Limits

### 1. UI Elements (`ui.*`)

#### Main Menu (Character Limit: 20)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `ui.main_menu` | "Main Menu" | 20 | Main menu title |
| `ui.play` | "Play" | 10 | Play button |
| `ui.new_game` | "New Game" | 15 | New game button |
| `ui.continue` | "Continue" | 15 | Continue button |
| `ui.load_game` | "Load Game" | 15 | Load game button |
| `ui.settings` | "Settings" | 15 | Settings button |
| `ui.quit` | "Quit" | 10 | Quit button |
| `ui.exit` | "Exit" | 10 | Exit button |

#### Game Menu (Character Limit: 25)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `ui.pause` | "Pause" | 10 | Pause button |
| `ui.resume` | "Resume" | 15 | Resume button |
| `ui.restart` | "Restart" | 15 | Restart button |
| `ui.save_game` | "Save Game" | 15 | Save game button |
| `ui.load_game` | "Load Game" | 15 | Load game button |
| `ui.return_to_menu` | "Return to Menu" | 25 | Return to main menu |
| `ui.return_to_game` | "Return to Game" | 25 | Return to game |

#### Navigation (Character Limit: 15)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `ui.back` | "Back" | 10 | Back button |
| `ui.next` | "Next" | 10 | Next button |
| `ui.previous` | "Previous" | 15 | Previous button |
| `ui.cancel` | "Cancel" | 15 | Cancel button |
| `ui.confirm` | "Confirm" | 15 | Confirm button |
| `ui.save` | "Save" | 10 | Save button |
| `ui.load` | "Load" | 10 | Load button |
| `ui.delete` | "Delete" | 10 | Delete button |
| `ui.edit` | "Edit" | 10 | Edit button |
| `ui.create` | "Create" | 10 | Create button |

#### Character Selection (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `ui.character_selection` | "Character Selection" | 30 | Character selection title |
| `ui.create_character` | "Create Character" | 25 | Create character button |
| `ui.select_character` | "Select Character" | 25 | Select character button |
| `ui.delete_character` | "Delete Character" | 25 | Delete character button |
| `ui.character_name` | "Character Name" | 20 | Character name label |
| `ui.character_class` | "Character Class" | 20 | Character class label |
| `ui.character_level` | "Level" | 10 | Character level label |
| `ui.character_stats` | "Stats" | 10 | Character stats label |
| `ui.character_equipment` | "Equipment" | 15 | Character equipment label |
| `ui.character_abilities` | "Abilities" | 15 | Character abilities label |

#### Inventory & Equipment (Character Limit: 25)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `ui.inventory` | "Inventory" | 15 | Inventory title |
| `ui.equipment` | "Equipment" | 15 | Equipment title |
| `ui.items` | "Items" | 10 | Items label |
| `ui.weapons` | "Weapons" | 15 | Weapons label |
| `ui.armor` | "Armor" | 10 | Armor label |
| `ui.accessories` | "Accessories" | 20 | Accessories label |
| `ui.consumables` | "Consumables" | 20 | Consumables label |
| `ui.equip` | "Equip" | 10 | Equip button |
| `ui.unequip` | "Unequip" | 15 | Unequip button |
| `ui.use_item` | "Use Item" | 15 | Use item button |
| `ui.drop_item` | "Drop Item" | 15 | Drop item button |
| `ui.item_info` | "Item Info" | 15 | Item info label |
| `ui.item_stats` | "Item Stats" | 15 | Item stats label |
| `ui.item_effects` | "Item Effects" | 20 | Item effects label |
| `ui.item_requirements` | "Requirements" | 20 | Item requirements label |

#### Settings (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `ui.settings_graphics` | "Graphics Settings" | 25 | Graphics settings title |
| `ui.settings_audio` | "Audio Settings" | 20 | Audio settings title |
| `ui.settings_controls` | "Control Settings" | 25 | Control settings title |
| `ui.settings_language` | "Language" | 15 | Language setting |
| `ui.settings_resolution` | "Resolution" | 15 | Resolution setting |
| `ui.settings_fullscreen` | "Fullscreen" | 15 | Fullscreen setting |
| `ui.settings_vsync` | "VSync" | 10 | VSync setting |
| `ui.settings_quality` | "Quality" | 10 | Quality setting |
| `ui.settings_volume_master` | "Master Volume" | 20 | Master volume setting |
| `ui.settings_volume_music` | "Music Volume" | 20 | Music volume setting |
| `ui.settings_volume_sfx` | "SFX Volume" | 20 | SFX volume setting |
| `ui.settings_volume_voice` | "Voice Volume" | 20 | Voice volume setting |

#### Sound Test (Character Limit: 40)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `ui.sound_test` | "Sound Test" | 20 | Sound test menu title |
| `ui.sound_test_sfx` | "Sound Effects" | 25 | SFX category title |
| `ui.sound_test_music` | "Background Music" | 30 | BGM category title |
| `ui.sound_test_voice` | "Voice Lines" | 25 | Voice category title |
| `ui.sound_test_ambient` | "Ambient Sounds" | 30 | Ambient category title |
| `ui.sound_test_play` | "Play" | 10 | Play sound button |
| `ui.sound_test_stop` | "Stop" | 10 | Stop sound button |
| `ui.sound_test_loop` | "Loop" | 10 | Loop sound button |
| `ui.sound_test_volume` | "Volume" | 15 | Volume control label |
| `ui.sound_test_category` | "Category" | 15 | Category selection label |
| `ui.sound_test_current` | "Currently Playing" | 25 | Currently playing label |
| `ui.sound_test_duration` | "Duration" | 15 | Duration label |
| `ui.sound_test_frequency` | "Frequency" | 15 | Frequency label |
| `ui.sound_test_format` | "Format" | 15 | Audio format label |
| `ui.sound_test_channels` | "Channels" | 15 | Audio channels label |
| `ui.sound_test_bitrate` | "Bitrate" | 15 | Audio bitrate label |
| `ui.sound_test_sample_rate` | "Sample Rate" | 20 | Sample rate label |
| `ui.sound_test_export` | "Export Audio" | 20 | Export audio button |
| `ui.sound_test_import` | "Import Audio" | 20 | Import audio button |
| `ui.sound_test_reset` | "Reset to Default" | 25 | Reset audio settings button |

### 2. Gameplay Elements (`gameplay.*`)

#### Core Stats (Character Limit: 15)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `gameplay.level` | "Level" | 10 | Level label |
| `gameplay.score` | "Score" | 10 | Score label |
| `gameplay.time` | "Time" | 10 | Time label |
| `gameplay.health` | "Health" | 10 | Health label |
| `gameplay.mana` | "Mana" | 10 | Mana label |
| `gameplay.stamina` | "Stamina" | 15 | Stamina label |
| `gameplay.experience` | "Experience" | 15 | Experience label |
| `gameplay.gold` | "Gold" | 10 | Gold label |
| `gameplay.silver` | "Silver" | 10 | Silver label |
| `gameplay.copper` | "Copper" | 10 | Copper label |

#### Character Stats (Character Limit: 20)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `gameplay.strength` | "Strength" | 15 | Strength stat |
| `gameplay.agility` | "Agility" | 15 | Agility stat |
| `gameplay.intelligence` | "Intelligence" | 20 | Intelligence stat |
| `gameplay.vitality` | "Vitality" | 15 | Vitality stat |
| `gameplay.wisdom` | "Wisdom" | 15 | Wisdom stat |
| `gameplay.charisma` | "Charisma" | 15 | Charisma stat |
| `gameplay.luck` | "Luck" | 10 | Luck stat |
| `gameplay.attack_power` | "Attack Power" | 20 | Attack power stat |
| `gameplay.defense` | "Defense" | 15 | Defense stat |
| `gameplay.magic_power` | "Magic Power" | 20 | Magic power stat |
| `gameplay.magic_resistance` | "Magic Resistance" | 25 | Magic resistance stat |
| `gameplay.critical_chance` | "Critical Chance" | 25 | Critical chance stat |
| `gameplay.critical_damage` | "Critical Damage" | 25 | Critical damage stat |
| `gameplay.movement_speed` | "Movement Speed" | 25 | Movement speed stat |
| `gameplay.attack_speed` | "Attack Speed" | 20 | Attack speed stat |

### 3. Character Classes (`characters.*`)

#### Character Classes (Character Limit: 25)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `characters.fighter` | "Fighter" | 15 | Fighter class name |
| `characters.ranger` | "Ranger" | 15 | Ranger class name |
| `characters.mage` | "Mage" | 10 | Mage class name |
| `characters.tank` | "Tank" | 10 | Tank class name |
| `characters.assassin` | "Assassin" | 20 | Assassin class name |
| `characters.fighter_desc` | "A skilled warrior trained in combat" | 50 | Fighter description |
| `characters.ranger_desc` | "A master of the bow and wilderness" | 50 | Ranger description |
| `characters.mage_desc` | "A scholar of the arcane arts" | 40 | Mage description |
| `characters.tank_desc` | "A massive warrior clad in heavy armor" | 50 | Tank description |
| `characters.assassin_desc` | "A stealthy killer from the shadows" | 50 | Assassin description |

#### Character Templates (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `characters.warrior_template` | "Warrior" | 15 | Warrior template name |
| `characters.forest_guardian` | "Forest Guardian" | 25 | Forest Guardian template |
| `characters.arcane_scholar` | "Arcane Scholar" | 25 | Arcane Scholar template |
| `characters.iron_guardian` | "Iron Guardian" | 25 | Iron Guardian template |
| `characters.shadow_blade` | "Shadow Blade" | 25 | Shadow Blade template |

### 4. Items & Equipment (`items.*`)

#### Item Types (Character Limit: 20)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `items.weapon` | "Weapon" | 15 | Weapon type |
| `items.armor` | "Armor" | 10 | Armor type |
| `items.accessory` | "Accessory" | 20 | Accessory type |
| `items.consumable` | "Consumable" | 20 | Consumable type |
| `items.material` | "Material" | 15 | Material type |
| `items.quest` | "Quest Item" | 20 | Quest item type |
| `items.misc` | "Miscellaneous" | 20 | Miscellaneous type |

#### Item Rarity (Character Limit: 15)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `items.common` | "Common" | 10 | Common rarity |
| `items.uncommon` | "Uncommon" | 15 | Uncommon rarity |
| `items.rare` | "Rare" | 10 | Rare rarity |
| `items.epic` | "Epic" | 10 | Epic rarity |
| `items.legendary` | "Legendary" | 15 | Legendary rarity |
| `items.artifact` | "Artifact" | 15 | Artifact rarity |

#### Weapon Types (Character Limit: 25)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `items.sword` | "Sword" | 10 | Sword weapon type |
| `items.axe` | "Axe" | 10 | Axe weapon type |
| `items.mace` | "Mace" | 10 | Mace weapon type |
| `items.dagger` | "Dagger" | 15 | Dagger weapon type |
| `items.bow` | "Bow" | 10 | Bow weapon type |
| `items.crossbow` | "Crossbow" | 15 | Crossbow weapon type |
| `items.staff` | "Staff" | 10 | Staff weapon type |
| `items.wand` | "Wand" | 10 | Wand weapon type |
| `items.spear` | "Spear" | 10 | Spear weapon type |
| `items.hammer` | "Hammer" | 15 | Hammer weapon type |

#### Armor Types (Character Limit: 20)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `items.helmet` | "Helmet" | 15 | Helmet armor type |
| `items.chestplate` | "Chestplate" | 20 | Chestplate armor type |
| `items.leggings` | "Leggings" | 15 | Leggings armor type |
| `items.boots` | "Boots" | 10 | Boots armor type |
| `items.gloves` | "Gloves" | 10 | Gloves armor type |
| `items.shield` | "Shield" | 15 | Shield armor type |
| `items.cape` | "Cape" | 10 | Cape armor type |
| `items.belt` | "Belt" | 10 | Belt armor type |

#### Consumable Items (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `items.minor_health_potion` | "Minor Health Potion" | 30 | Minor health potion |
| `items.health_potion` | "Health Potion" | 25 | Health potion |
| `items.greater_health_potion` | "Greater Health Potion" | 35 | Greater health potion |
| `items.mana_potion` | "Mana Potion" | 25 | Mana potion |
| `items.stamina_potion` | "Stamina Potion" | 25 | Stamina potion |
| `items.strength_potion` | "Potion of Strength" | 30 | Strength potion |
| `items.speed_potion` | "Potion of Speed" | 30 | Speed potion |
| `items.bread` | "Bread" | 10 | Bread consumable |
| `items.teleport_scroll` | "Scroll of Teleportation" | 35 | Teleport scroll |

### 5. Combat System (`combat.*`)

#### Combat Actions (Character Limit: 20)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `combat.attack` | "Attack" | 10 | Attack action |
| `combat.defend` | "Defend" | 10 | Defend action |
| `combat.dodge` | "Dodge" | 10 | Dodge action |
| `combat.block` | "Block" | 10 | Block action |
| `combat.parry` | "Parry" | 10 | Parry action |
| `combat.cast_spell` | "Cast Spell" | 20 | Cast spell action |
| `combat.use_item` | "Use Item" | 15 | Use item action |
| `combat.special_move` | "Special Move" | 20 | Special move action |
| `combat.ultimate` | "Ultimate" | 15 | Ultimate ability |

#### Combat Results (Character Limit: 25)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `combat.critical_hit` | "Critical Hit!" | 20 | Critical hit message |
| `combat.miss` | "Miss" | 10 | Miss message |
| `combat.damage` | "Damage" | 10 | Damage label |
| `combat.heal` | "Heal" | 10 | Heal label |
| `combat.death` | "Death" | 10 | Death message |
| `combat.victory` | "Victory!" | 15 | Victory message |
| `combat.defeat` | "Defeat" | 10 | Defeat message |
| `combat.level_up` | "Level Up!" | 15 | Level up message |
| `combat.combo` | "Combo!" | 10 | Combo message |
| `combat.perfect` | "Perfect!" | 15 | Perfect timing message |

### 6. Enemy Types (`enemies.*`)

#### Basic Enemies (Character Limit: 25)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `enemies.goblin` | "Goblin" | 15 | Goblin enemy |
| `enemies.orc` | "Orc" | 10 | Orc enemy |
| `enemies.skeleton` | "Skeleton" | 15 | Skeleton enemy |
| `enemies.zombie` | "Zombie" | 15 | Zombie enemy |
| `enemies.spider` | "Giant Spider" | 20 | Giant spider enemy |
| `enemies.wolf` | "Dire Wolf" | 15 | Dire wolf enemy |
| `enemies.bandit` | "Bandit" | 15 | Bandit enemy |
| `enemies.cultist` | "Cultist" | 15 | Cultist enemy |

#### Special Enemies (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `enemies.goblin_mage` | "Goblin Mage" | 20 | Goblin mage enemy |
| `enemies.orc_berserker` | "Orc Berserker" | 25 | Orc berserker enemy |
| `enemies.shield_bearer` | "Shield Bearer" | 25 | Shield bearer enemy |
| `enemies.dark_mage` | "Dark Mage" | 20 | Dark mage enemy |
| `enemies.necromancer` | "Necromancer" | 25 | Necromancer enemy |

#### Boss Enemies (Character Limit: 35)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `enemies.goblin_king` | "Goblin King" | 20 | Goblin King boss |
| `enemies.orc_warlord` | "Orc Warlord" | 25 | Orc Warlord boss |
| `enemies.dragon_lord` | "Dragon Lord" | 25 | Dragon Lord boss |
| `enemies.ancient_lich` | "Ancient Lich" | 25 | Ancient Lich boss |
| `enemies.demon_prince` | "Demon Prince" | 25 | Demon Prince boss |

### 7. Level Names (`levels.*`)

#### Level Types (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `levels.forest_clearing` | "Forest Clearing" | 25 | Forest clearing level |
| `levels.dark_cave` | "Dark Cave" | 20 | Dark cave level |
| `levels.mountain_pass` | "Mountain Pass" | 25 | Mountain pass level |
| `levels.ancient_ruins` | "Ancient Ruins" | 25 | Ancient ruins level |
| `levels.castle_courtyard` | "Castle Courtyard" | 30 | Castle courtyard level |
| `levels.dungeon_depths` | "Dungeon Depths" | 25 | Dungeon depths level |
| `levels.volcanic_chamber` | "Volcanic Chamber" | 30 | Volcanic chamber level |
| `levels.ice_cavern` | "Ice Cavern" | 20 | Ice cavern level |

### 8. Tutorial System (`tutorial.*`)

#### Tutorial Steps (Character Limit: 50)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `tutorial.welcome` | "Welcome to the 2D Brawler!" | 35 | Welcome message |
| `tutorial.movement` | "Use WASD to move your character" | 45 | Movement tutorial |
| `tutorial.combat` | "Click to attack enemies" | 35 | Combat tutorial |
| `tutorial.inventory` | "Press I to open your inventory" | 40 | Inventory tutorial |
| `tutorial.skills` | "Press S to view your skills" | 35 | Skills tutorial |
| `tutorial.abilities` | "Press A to view your abilities" | 40 | Abilities tutorial |
| `tutorial.leveling` | "Gain experience to level up" | 40 | Leveling tutorial |
| `tutorial.equipment` | "Equip items to improve your stats" | 45 | Equipment tutorial |

### 9. Error Messages (`errors.*`)

#### System Errors (Character Limit: 100)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `errors.generic` | "An error occurred" | 25 | Generic error message |
| `errors.save_failed` | "Failed to save game" | 30 | Save failed error |
| `errors.load_failed` | "Failed to load game" | 30 | Load failed error |
| `errors.network` | "Network connection failed" | 35 | Network error |
| `errors.invalid_input` | "Invalid input provided" | 35 | Invalid input error |
| `errors.file_not_found` | "File not found" | 25 | File not found error |
| `errors.permission_denied` | "Permission denied" | 25 | Permission denied error |
| `errors.out_of_memory` | "Out of memory" | 20 | Out of memory error |
| `errors.inventory_full` | "Inventory is full" | 25 | Inventory full error |
| `errors.insufficient_funds` | "Insufficient funds" | 25 | Insufficient funds error |
| `errors.item_not_found` | "Item not found" | 25 | Item not found error |
| `errors.character_not_found` | "Character not found" | 30 | Character not found error |

### 10. Achievement System (`achievements.*`)

#### Achievement Names (Character Limit: 50)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `achievements.first_kill` | "First Blood" | 20 | First enemy kill |
| `achievements.level_10` | "Rising Star" | 20 | Reach level 10 |
| `achievements.level_25` | "Veteran" | 15 | Reach level 25 |
| `achievements.level_50` | "Master" | 15 | Reach level 50 |
| `achievements.combo_10` | "Combo Master" | 25 | 10-hit combo |
| `achievements.perfect_dodge` | "Dodge Master" | 25 | Perfect dodge |
| `achievements.collector` | "Item Collector" | 25 | Collect 100 items |
| `achievements.explorer` | "Explorer" | 15 | Visit all levels |
| `achievements.boss_killer` | "Boss Slayer" | 25 | Defeat all bosses |
| `achievements.perfectionist` | "Perfectionist" | 25 | Complete without dying |

### 11. Sound Test System (`ui.sound_test.*`)

#### Sound Test UI (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `ui.sound_test` | "Sound Test" | 15 | Sound test menu title |
| `ui.sound_test_sfx` | "Sound Effects" | 20 | SFX category title |
| `ui.sound_test_music` | "Background Music" | 20 | BGM category title |
| `ui.sound_test_voice` | "Voice Lines" | 20 | Voice category title |
| `ui.sound_test_ambient` | "Ambient Sounds" | 20 | Ambient category title |
| `ui.sound_test_play` | "Play" | 10 | Play sound button |
| `ui.sound_test_stop` | "Stop" | 10 | Stop sound button |
| `ui.sound_test_loop` | "Loop" | 10 | Loop sound button |
| `ui.sound_test_volume` | "Volume" | 10 | Volume control label |
| `ui.sound_test_category` | "Category" | 10 | Category selection label |
| `ui.sound_test_current` | "Currently Playing" | 20 | Currently playing label |
| `ui.sound_test_duration` | "Duration" | 10 | Duration label |
| `ui.sound_test_frequency` | "Frequency" | 10 | Frequency label |
| `ui.sound_test_format` | "Format" | 10 | Audio format label |
| `ui.sound_test_channels` | "Channels" | 10 | Audio channels label |
| `ui.sound_test_bitrate` | "Bitrate" | 10 | Audio bitrate label |
| `ui.sound_test_sample_rate` | "Sample Rate" | 15 | Sample rate label |
| `ui.sound_test_export` | "Export Audio" | 15 | Export audio button |
| `ui.sound_test_import` | "Import Audio" | 15 | Import audio button |
| `ui.sound_test_reset` | "Reset to Default" | 20 | Reset audio settings button |

#### Background Music Categories (Character Limit: 35)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `ui.sound_test_music_main_menu` | "Main Menu Theme" | 25 | Main menu background music |
| `ui.sound_test_music_character_select` | "Character Selection" | 30 | Character selection music |
| `ui.sound_test_music_forest` | "Forest Theme" | 20 | Forest level music |
| `ui.sound_test_music_cave` | "Cave Theme" | 20 | Cave level music |
| `ui.sound_test_music_mountain` | "Mountain Theme" | 25 | Mountain level music |
| `ui.sound_test_music_ruins` | "Ancient Ruins" | 25 | Ruins level music |
| `ui.sound_test_music_castle` | "Castle Theme" | 20 | Castle level music |
| `ui.sound_test_music_dungeon` | "Dungeon Theme" | 25 | Dungeon level music |
| `ui.sound_test_music_volcano` | "Volcanic Theme" | 25 | Volcanic level music |
| `ui.sound_test_music_ice` | "Ice Cavern" | 20 | Ice cavern music |
| `ui.sound_test_music_combat` | "Combat Theme" | 20 | Combat background music |
| `ui.sound_test_music_boss` | "Boss Battle" | 20 | Boss battle music |
| `ui.sound_test_music_victory` | "Victory Theme" | 20 | Victory music |
| `ui.sound_test_music_defeat` | "Defeat Theme" | 20 | Defeat music |
| `ui.sound_test_music_credits` | "Credits Theme" | 20 | Credits music |

#### Voice Line Categories (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `ui.sound_test_voice_attack` | "Attack Voice" | 20 | Character attack voice |
| `ui.sound_test_voice_hurt` | "Hurt Voice" | 20 | Character hurt voice |
| `ui.sound_test_voice_death` | "Death Voice" | 20 | Character death voice |
| `ui.sound_test_voice_victory` | "Victory Voice" | 20 | Victory voice |
| `ui.sound_test_voice_level_up` | "Level Up Voice" | 25 | Level up voice |
| `ui.sound_test_voice_taunt` | "Taunt Voice" | 20 | Taunt voice |
| `ui.sound_test_voice_encouragement` | "Encouragement" | 25 | Encouragement voice |
| `ui.sound_test_voice_warning` | "Warning Voice" | 20 | Warning voice |
| `ui.sound_test_voice_grunt` | "Grunt Voice" | 20 | Grunt voice |
| `ui.sound_test_voice_sigh` | "Sigh Voice" | 20 | Sigh voice |

#### Ambient Sound Categories (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `ui.sound_test_ambient_wind` | "Wind" | 10 | Wind ambient sound |
| `ui.sound_test_ambient_rain` | "Rain" | 10 | Rain ambient sound |
| `ui.sound_test_ambient_thunder` | "Thunder" | 15 | Thunder ambient sound |
| `ui.sound_test_ambient_fire` | "Fire" | 10 | Fire ambient sound |
| `ui.sound_test_ambient_water` | "Water" | 10 | Water ambient sound |
| `ui.sound_test_ambient_cave` | "Cave Echo" | 20 | Cave echo ambient sound |
| `ui.sound_test_ambient_forest` | "Forest Sounds" | 25 | Forest ambient sounds |
| `ui.sound_test_ambient_city` | "City Sounds" | 20 | City ambient sounds |
| `ui.sound_test_ambient_machinery` | "Machinery" | 20 | Machinery ambient sound |
| `ui.sound_test_ambient_magic` | "Magic Aura" | 20 | Magic aura ambient sound |

### 12. SFX Pitch Variation System (`sfx_pitch.*`)

#### Pitch Control (Character Limit: 25)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `sfx_pitch.original` | "Original Pitch" | 20 | Original sound pitch |
| `sfx_pitch.higher` | "Higher Pitch" | 20 | Higher pitch variation |
| `sfx_pitch.lower` | "Lower Pitch" | 20 | Lower pitch variation |
| `sfx_pitch.random` | "Random Pitch" | 20 | Random pitch variation |
| `sfx_pitch.semitone_up` | "Semitone Up" | 20 | One semitone higher |
| `sfx_pitch.semitone_down` | "Semitone Down" | 25 | One semitone lower |
| `sfx_pitch.octave_up` | "Octave Up" | 20 | One octave higher |
| `sfx_pitch.octave_down` | "Octave Down" | 20 | One octave lower |
| `sfx_pitch.pitch_shift` | "Pitch Shift" | 20 | Pitch shift control |
| `sfx_pitch.pitch_bend` | "Pitch Bend" | 20 | Pitch bend control |
| `sfx_pitch.vibrato` | "Vibrato" | 15 | Vibrato effect |
| `sfx_pitch.tremolo` | "Tremolo" | 15 | Tremolo effect |
| `sfx_pitch.wah_wah` | "Wah-Wah" | 15 | Wah-wah effect |
| `sfx_pitch.chorus` | "Chorus" | 15 | Chorus effect |
| `sfx_pitch.flanger` | "Flanger" | 15 | Flanger effect |

#### Pitch Variation Types (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `sfx_pitch.variation_1` | "Variation 1" | 20 | First pitch variation |
| `sfx_pitch.variation_2` | "Variation 2" | 20 | Second pitch variation |
| `sfx_pitch.variation_3` | "Variation 3" | 20 | Third pitch variation |
| `sfx_pitch.variation_4` | "Variation 4" | 20 | Fourth pitch variation |
| `sfx_pitch.variation_5` | "Variation 5" | 20 | Fifth pitch variation |
| `sfx_pitch.variation_6` | "Variation 6" | 20 | Sixth pitch variation |
| `sfx_pitch.variation_7` | "Variation 7" | 20 | Seventh pitch variation |
| `sfx_pitch.variation_8` | "Variation 8" | 20 | Eighth pitch variation |
| `sfx_pitch.variation_9` | "Variation 9" | 20 | Ninth pitch variation |
| `sfx_pitch.variation_10` | "Variation 10" | 20 | Tenth pitch variation |
| `sfx_pitch.variation_custom` | "Custom Variation" | 25 | Custom pitch variation |
| `sfx_pitch.variation_preset` | "Preset Variation" | 25 | Preset pitch variation |

#### Pitch Range Controls (Character Limit: 25)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `sfx_pitch.range_min` | "Minimum Range" | 20 | Minimum pitch range |
| `sfx_pitch.range_max` | "Maximum Range" | 20 | Maximum pitch range |
| `sfx_pitch.range_center` | "Center Range" | 20 | Center pitch range |
| `sfx_pitch.range_narrow` | "Narrow Range" | 20 | Narrow pitch range |
| `sfx_pitch.range_wide` | "Wide Range" | 20 | Wide pitch range |
| `sfx_pitch.range_auto` | "Auto Range" | 20 | Automatic range detection |
| `sfx_pitch.range_manual` | "Manual Range" | 20 | Manual range setting |
| `sfx_pitch.range_preset` | "Preset Range" | 20 | Preset range selection |

#### Pitch Effects (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `sfx_pitch.effect_none` | "No Effect" | 15 | No pitch effect |
| `sfx_pitch.effect_robot` | "Robot Effect" | 20 | Robot voice effect |
| `sfx_pitch.effect_helium` | "Helium Effect" | 20 | Helium voice effect |
| `sfx_pitch.effect_deep` | "Deep Voice" | 20 | Deep voice effect |
| `sfx_pitch.effect_echo` | "Echo Effect" | 20 | Echo pitch effect |
| `sfx_pitch.effect_reverb` | "Reverb Effect" | 20 | Reverb pitch effect |
| `sfx_pitch.effect_distortion` | "Distortion" | 20 | Distortion effect |
| `sfx_pitch.effect_bitcrush` | "Bitcrush" | 20 | Bitcrush effect |
| `sfx_pitch.effect_lowpass` | "Low Pass" | 20 | Low pass filter |
| `sfx_pitch.effect_highpass` | "High Pass" | 20 | High pass filter |
| `sfx_pitch.effect_bandpass` | "Band Pass" | 20 | Band pass filter |
| `sfx_pitch.effect_notch` | "Notch Filter" | 20 | Notch filter |

#### Pitch Variation Presets (Character Limit: 35)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `sfx_pitch.preset_combat_light` | "Light Combat" | 20 | Light combat variation |
| `sfx_pitch.preset_combat_heavy` | "Heavy Combat" | 20 | Heavy combat variation |
| `sfx_pitch.preset_combat_critical` | "Critical Hit" | 20 | Critical hit variation |
| `sfx_pitch.preset_weapon_swing` | "Weapon Swing" | 20 | Weapon swing variation |
| `sfx_pitch.preset_weapon_impact` | "Weapon Impact" | 20 | Weapon impact variation |
| `sfx_pitch.preset_footstep_light` | "Light Footstep" | 25 | Light footstep variation |
| `sfx_pitch.preset_footstep_heavy` | "Heavy Footstep" | 25 | Heavy footstep variation |
| `sfx_pitch.preset_ui_click` | "UI Click" | 15 | UI click variation |
| `sfx_pitch.preset_ui_hover` | "UI Hover" | 15 | UI hover variation |
| `sfx_pitch.preset_item_pickup` | "Item Pickup" | 20 | Item pickup variation |
| `sfx_pitch.preset_item_use` | "Item Use" | 15 | Item use variation |
| `sfx_pitch.preset_spell_cast` | "Spell Cast" | 20 | Spell cast variation |
| `sfx_pitch.preset_spell_impact` | "Spell Impact" | 20 | Spell impact variation |
| `sfx_pitch.preset_environmental` | "Environmental" | 25 | Environmental variation |
| `sfx_pitch.preset_character_voice` | "Character Voice" | 25 | Character voice variation |

### 13. Dynamic Music System (`music.*`)

#### Music Stems (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `music.stem_1` | "Stem 1 - Rhythm" | 20 | Rhythm section (bass, drums) |
| `music.stem_2` | "Stem 2 - Harmony" | 20 | Harmonic instruments (guitars, keys) |
| `music.stem_3` | "Stem 3 - Melody" | 20 | Melodic instruments (lead, vocals) |
| `music.stem_4` | "Stem 4 - Effects" | 20 | Atmospheric effects and textures |
| `music.stem_bass` | "Bass & Drums" | 20 | Bass and drum stem |
| `music.stem_guitars` | "Guitars" | 15 | Guitar stem |
| `music.stem_keyboards` | "Keyboards" | 20 | Keyboard stem |
| `music.stem_lead` | "Lead Instruments" | 25 | Lead instrument stem |
| `music.stem_vocals` | "Vocals" | 15 | Vocal stem |
| `music.stem_strings` | "Strings" | 15 | String section stem |
| `music.stem_brass` | "Brass" | 15 | Brass section stem |
| `music.stem_woodwinds` | "Woodwinds" | 20 | Woodwind section stem |
| `music.stem_percussion` | "Percussion" | 20 | Percussion stem |
| `music.stem_ambient` | "Ambient" | 15 | Ambient texture stem |
| `music.stem_effects` | "Effects" | 15 | Sound effects stem |

#### Dynamic Music States (Character Limit: 35)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `music.state_calm` | "Calm State" | 20 | Calm gameplay state |
| `music.state_tension` | "Tension State" | 20 | Building tension state |
| `music.state_combat` | "Combat State" | 20 | Active combat state |
| `music.state_intense` | "Intense State" | 20 | High intensity state |
| `music.state_boss` | "Boss State" | 20 | Boss battle state |
| `music.state_victory` | "Victory State" | 20 | Victory celebration state |
| `music.state_defeat` | "Defeat State" | 20 | Defeat state |
| `music.state_exploration` | "Exploration State" | 25 | Exploration state |
| `music.state_danger` | "Danger State" | 20 | Danger warning state |
| `music.state_mystery` | "Mystery State" | 20 | Mystery/suspense state |
| `music.state_celebration` | "Celebration State" | 25 | Celebration state |
| `music.state_sadness` | "Sadness State" | 20 | Emotional/sad state |

#### Music Transitions (Character Limit: 35)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `music.transition_fade_in` | "Fade In" | 15 | Fade in from silence |
| `music.transition_fade_out` | "Fade Out" | 15 | Fade out to silence |
| `music.transition_crossfade` | "Crossfade" | 20 | Crossfade between tracks |
| `music.transition_cut` | "Cut Transition" | 20 | Instant cut transition |
| `music.transition_beat_match` | "Beat Match" | 25 | Beat-synchronized transition |
| `music.transition_phrase` | "Phrase Transition" | 25 | Phrase-synchronized transition |
| `music.transition_measure` | "Measure Transition" | 25 | Measure-synchronized transition |
| `music.transition_gradual` | "Gradual Transition" | 25 | Gradual stem addition |
| `music.transition_instant` | "Instant Transition" | 25 | Instant stem change |
| `music.transition_ramp` | "Ramp Transition" | 20 | Volume ramp transition |
| `music.transition_stop` | "Stop Transition" | 20 | Stop all stems |
| `music.transition_silence` | "To Silence" | 20 | Transition to silence |
| `music.transition_from_silence` | "From Silence" | 25 | Transition from silence |
| `music.transition_loop` | "Loop Transition" | 25 | Seamless loop transition |
| `music.transition_reverse` | "Reverse Transition" | 25 | Reverse playback transition |

#### Music Fade Types (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `music.fade_linear` | "Linear Fade" | 20 | Linear volume fade |
| `music.fade_exponential` | "Exponential Fade" | 25 | Exponential volume fade |
| `music.fade_logarithmic` | "Logarithmic Fade" | 25 | Logarithmic volume fade |
| `music.fade_s_curve` | "S-Curve Fade" | 20 | S-curve volume fade |
| `music.fade_cosine` | "Cosine Fade" | 20 | Cosine volume fade |
| `music.fade_sine` | "Sine Fade" | 15 | Sine volume fade |
| `music.fade_custom` | "Custom Fade" | 20 | Custom fade curve |
| `music.fade_preset` | "Preset Fade" | 20 | Preset fade curve |

#### Music Transition Timing (Character Limit: 25)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `music.timing_instant` | "Instant" | 10 | Instant transition |
| `music.timing_fast` | "Fast" | 10 | Fast transition |
| `music.timing_medium` | "Medium" | 10 | Medium transition |
| `music.timing_slow` | "Slow" | 10 | Slow transition |
| `music.timing_custom` | "Custom" | 15 | Custom timing |
| `music.timing_auto` | "Auto" | 10 | Automatic timing |
| `music.timing_beat` | "Beat Sync" | 15 | Beat synchronized |
| `music.timing_phrase` | "Phrase Sync" | 20 | Phrase synchronized |

#### Music Transition Effects (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `music.effect_none` | "No Effect" | 15 | No transition effect |
| `music.effect_reverb` | "Reverb" | 15 | Reverb transition effect |
| `music.effect_echo` | "Echo" | 10 | Echo transition effect |
| `music.effect_filter` | "Filter" | 15 | Filter transition effect |
| `music.effect_distortion` | "Distortion" | 20 | Distortion transition effect |
| `music.effect_chorus` | "Chorus" | 15 | Chorus transition effect |
| `music.effect_flanger` | "Flanger" | 15 | Flanger transition effect |
| `music.effect_phaser` | "Phaser" | 15 | Phaser transition effect |
| `music.effect_compressor` | "Compressor" | 20 | Compressor transition effect |
| `music.effect_limiter` | "Limiter" | 15 | Limiter transition effect |
| `music.effect_gate` | "Gate" | 10 | Gate transition effect |
| `music.effect_expander` | "Expander" | 20 | Expander transition effect |

#### Music Controls (Character Limit: 25)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `music.control_volume` | "Volume" | 10 | Volume control |
| `music.control_pan` | "Pan" | 10 | Pan control |
| `music.control_pitch` | "Pitch" | 10 | Pitch control |
| `music.control_tempo` | "Tempo" | 10 | Tempo control |
| `music.control_key` | "Key" | 10 | Musical key control |
| `music.control_scale` | "Scale" | 10 | Musical scale control |
| `music.control_loop` | "Loop" | 10 | Loop control |
| `music.control_sync` | "Sync" | 10 | Synchronization control |
| `music.control_quantize` | "Quantize" | 15 | Quantization control |
| `music.control_swing` | "Swing" | 10 | Swing control |

#### Music Analysis (Character Limit: 30)
| String ID | English | Max Chars | Description |
|-----------|---------|-----------|-------------|
| `music.analysis_bpm` | "BPM" | 10 | Beats per minute |
| `music.analysis_key` | "Musical Key" | 20 | Detected musical key |
| `music.analysis_scale` | "Scale" | 15 | Musical scale |
| `music.analysis_energy` | "Energy Level" | 20 | Musical energy level |
| `music.analysis_valence` | "Valence" | 15 | Musical mood valence |
| `music.analysis_danceability` | "Danceability" | 25 | Danceability score |
| `music.analysis_acousticness` | "Acousticness" | 25 | Acoustic quality score |
| `music.analysis_instrumentalness` | "Instrumentalness" | 30 | Instrumental quality score |
| `music.analysis_liveness` | "Liveness" | 15 | Live performance quality |
| `music.analysis_speechiness` | "Speechiness" | 20 | Speech content score |

## SFX Pitch Variation System Implementation

### Advanced Real-Time Pitch Shifting System

The SFX pitch variation system will implement a sophisticated real-time pitch shifting engine that can generate multiple variations from a single source audio file, creating diverse audio experiences without requiring separate files for each variation.

#### Core Pitch Shifting Features

1. **Real-Time Pitch Processing**
   - **Pitch Shifting**: Real-time pitch modification without tempo change
   - **Pitch Bending**: Smooth pitch transitions and bends
   - **Semitone Control**: Precise semitone adjustments (±12 semitones)
   - **Octave Control**: Full octave shifts (±3 octaves)
   - **Microtone Control**: Fine pitch adjustments (cents)

2. **Pitch Variation Generation**
   - **Random Variations**: Generate random pitch variations within specified ranges
   - **Preset Variations**: Pre-defined pitch variations for common use cases
   - **Custom Variations**: User-defined pitch variations
   - **Batch Processing**: Generate multiple variations from single source
   - **Variation Caching**: Cache generated variations for performance

3. **Advanced Pitch Effects**
   - **Vibrato**: Pitch modulation for musical expression
   - **Tremolo**: Amplitude modulation for rhythmic effects
   - **Wah-Wah**: Filter sweep effects
   - **Chorus**: Multiple pitch-shifted copies for richness
   - **Flanger**: Comb filtering with pitch modulation

4. **Pitch Range Management**
   - **Range Detection**: Automatic pitch range analysis
   - **Range Limiting**: Prevent pitch shifting beyond safe ranges
   - **Range Presets**: Common pitch ranges for different sound types
   - **Range Validation**: Ensure pitch variations remain musical

5. **Quality Control**
   - **Algorithm Selection**: Choose optimal pitch shifting algorithm
   - **Quality Settings**: Balance between quality and performance
   - **Artifact Reduction**: Minimize pitch shifting artifacts
   - **Formant Preservation**: Maintain vocal characteristics

#### Technical Implementation

1. **Pitch Shifting Algorithms**
   - **PSOLA (Pitch Synchronous Overlap and Add)**: High-quality vocal pitch shifting
   - **Phase Vocoder**: Spectral domain pitch shifting
   - **Granular Synthesis**: Granular pitch shifting
   - **Time-Domain Pitch Shifting**: Fast pitch shifting for real-time use
   - **Hybrid Algorithms**: Combine multiple techniques for optimal results

2. **Real-Time Processing**
   - **Low-Latency Processing**: Minimal delay for real-time applications
   - **Buffer Management**: Efficient audio buffer handling
   - **Thread Safety**: Multi-threaded pitch processing
   - **Memory Optimization**: Efficient memory usage for pitch processing

3. **Pitch Analysis Engine**
   - **Fundamental Frequency Detection**: Accurate pitch detection
   - **Harmonic Analysis**: Analyze harmonic content
   - **Formant Tracking**: Track vocal formants
   - **Pitch Contour Analysis**: Analyze pitch changes over time

4. **Variation Management**
   - **Variation Database**: Store and manage pitch variations
   - **Variation Metadata**: Track variation properties and usage
   - **Variation Search**: Find variations by criteria
   - **Variation Export**: Export variations for external use

#### SFX Pitch Variation UI

```
┌─────────────────────────────────────────────────────────────┐
│                    SFX PITCH VARIATION                      │
├─────────────────────────────────────────────────────────────┤
│ Source: combat_hit_01.wav | Original: 440Hz | Key: A4      │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────────────────────────────────┐ │
│ │   Pitch     │ │              Variations                 │ │
│ │  Controls   │ │                                         │ │
│ │             │ │  Variation 1: [A4] [440Hz] [Original]   │ │
│ │ [Original]  │ │  Variation 2: [A#4] [466Hz] [+1 Semi]   │ │
│ │ [+1 Semi]   │ │  Variation 3: [B4] [494Hz] [+2 Semi]    │ │
│ │ [-1 Semi]   │ │  Variation 4: [G#4] [415Hz] [-1 Semi]   │ │
│ │ [+1 Octave] │ │  Variation 5: [A5] [880Hz] [+1 Octave]  │ │
│ │ [-1 Octave] │ │  Variation 6: [A3] [220Hz] [-1 Octave]  │ │
│ │ [Random]    │ │  Variation 7: [C5] [523Hz] [Random]     │ │
│ │ [Custom]    │ │  Variation 8: [F4] [349Hz] [Custom]     │ │
│ └─────────────┘ └─────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│ Effects: [Vibrato] [Tremolo] [Chorus] [Flanger] [None]     │
│ Range: [Auto] [Manual] [Preset] | Quality: [High] [Medium] │
├─────────────────────────────────────────────────────────────┤
│ [Generate] [Play] [Stop] [Save] [Load] [Export] [Settings] │
└─────────────────────────────────────────────────────────────┘
```

#### Pitch Variation Categories

1. **Combat Sounds**
   - **Light Combat**: Higher pitch for light attacks
   - **Heavy Combat**: Lower pitch for heavy attacks
   - **Critical Hits**: Dramatic pitch variations
   - **Weapon Swings**: Pitch variations based on weapon type
   - **Weapon Impacts**: Pitch variations based on impact force

2. **Character Sounds**
   - **Footsteps**: Pitch variations based on character weight
   - **Voice Lines**: Pitch variations for different characters
   - **Character Actions**: Pitch variations for different actions
   - **Character States**: Pitch variations based on health/status

3. **UI Sounds**
   - **Button Clicks**: Pitch variations for different button types
   - **Menu Sounds**: Pitch variations for menu navigation
   - **Notification Sounds**: Pitch variations for different notifications
   - **System Sounds**: Pitch variations for system events

4. **Environmental Sounds**
   - **Door Sounds**: Pitch variations for different door types
   - **Chest Sounds**: Pitch variations for different chest types
   - **Environmental Effects**: Pitch variations for weather/atmosphere
   - **Interactive Objects**: Pitch variations for different objects

#### File Organization

```
assets/sfx/
├── source/
│   ├── combat_hit_01.wav
│   ├── weapon_swing_01.wav
│   ├── footstep_01.wav
│   └── ui_click_01.wav
├── variations/
│   ├── combat_hit_01/
│   │   ├── variation_1.wav
│   │   ├── variation_2.wav
│   │   └── variation_3.wav
│   ├── weapon_swing_01/
│   │   ├── variation_1.wav
│   │   ├── variation_2.wav
│   │   └── variation_3.wav
│   └── footstep_01/
│       ├── variation_1.wav
│       ├── variation_2.wav
│       └── variation_3.wav
├── presets/
│   ├── combat_presets.json
│   ├── ui_presets.json
│   └── environmental_presets.json
└── metadata/
    ├── pitch_analysis.json
    ├── variation_database.json
    └── quality_metrics.json
```

#### Performance Optimization

1. **Caching System**
   - **Variation Caching**: Cache frequently used variations
   - **Precomputed Variations**: Generate variations at build time
   - **Lazy Loading**: Load variations on demand
   - **Memory Management**: Efficient memory usage for variations

2. **Real-Time Processing**
   - **Streaming**: Stream pitch-shifted audio in real-time
   - **Buffer Optimization**: Optimize audio buffers for pitch processing
   - **Thread Pool**: Use thread pool for parallel pitch processing
   - **Quality Scaling**: Adjust quality based on performance

3. **Algorithm Optimization**
   - **Fast Algorithms**: Use fast algorithms for real-time processing
   - **Quality Modes**: Different quality modes for different use cases
   - **Hardware Acceleration**: Use hardware acceleration when available
   - **SIMD Optimization**: Use SIMD instructions for vectorized processing

## Dynamic Music System Implementation

### World-Class 4-Stem Adaptive Music System

The dynamic music system will implement a sophisticated 4-stem adaptive audio system that can seamlessly transition between different musical intensities based on gameplay events, creating an immersive and responsive audio experience.

#### Core 4-Stem Architecture

1. **Stem 1 - Rhythm Foundation (Bass & Drums)**
   - Always present as the musical foundation
   - Provides consistent tempo and rhythm
   - Can be solo or combined with other stems
   - Used during calm exploration and intense combat

2. **Stem 2 - Harmonic Layer (Guitars & Keyboards)**
   - Adds harmonic complexity and texture
   - Introduces chord progressions and harmonies
   - Activates during moderate tension and combat
   - Provides musical depth and richness

3. **Stem 3 - Melodic Layer (Lead Instruments & Vocals)**
   - Contains main melodies and vocal lines
   - Adds emotional depth and character
   - Activates during high-intensity moments
   - Provides memorable musical hooks

4. **Stem 4 - Atmospheric Effects (Textures & Ambience)**
   - Adds atmospheric depth and immersion
   - Includes sound effects and ambient textures
   - Enhances environmental storytelling
   - Provides dynamic range and contrast

#### Advanced Features

1. **Real-Time Music Analysis**
   - **BPM Detection**: Automatic tempo detection and synchronization
   - **Key Detection**: Musical key analysis for seamless transitions
   - **Energy Analysis**: Real-time energy level calculation
   - **Mood Analysis**: Valence and arousal detection
   - **Instrumental Analysis**: Detection of instrumental vs. vocal content

2. **Intelligent Transition System**
   - **Beat-Synchronized Transitions**: Transitions aligned to musical beats
   - **Phrase-Synchronized Transitions**: Transitions aligned to musical phrases
   - **Crossfade Transitions**: Smooth volume-based transitions
   - **Gradual Stem Addition**: Progressive stem activation
   - **Instant State Changes**: Immediate response to critical events

3. **Dynamic Music States**
   - **Calm State**: Only Stem 1 (bass & drums)
   - **Tension State**: Stems 1 + 2 (adds harmonic layer)
   - **Combat State**: Stems 1 + 2 + 3 (adds melodic layer)
   - **Intense State**: All 4 stems (full orchestration)
   - **Boss State**: Custom stem combinations for boss battles
   - **Victory State**: Celebratory stem combinations
   - **Defeat State**: Somber stem combinations

4. **Advanced Audio Processing**
   - **Real-Time Pitch Shifting**: Maintain musical key across tempo changes
   - **Dynamic Range Compression**: Automatic volume balancing
   - **Spatial Audio**: 3D positioning of musical elements
   - **Reverb and Effects**: Dynamic application of audio effects
   - **EQ and Filtering**: Real-time frequency shaping

5. **Gameplay Integration**
   - **Event-Driven Triggers**: Music responds to specific game events
   - **Health-Based Intensity**: Music intensity based on player health
   - **Enemy Proximity**: Music intensity based on enemy distance
   - **Time-Based Progression**: Music evolves over time
   - **Player Action Response**: Music responds to player actions

#### Music Transition System Implementation

### Advanced Music Transition Engine

The music transition system will implement a sophisticated transition engine that can seamlessly fade in, fade out, and crossfade between different music files, stems, and silence, creating professional-quality music transitions.

#### Core Transition Features

1. **Fade In/Out Transitions**
   - **Fade In**: Smooth volume increase from silence to full volume
   - **Fade Out**: Smooth volume decrease from full volume to silence
   - **Fade Curves**: Multiple fade curve types (linear, exponential, logarithmic, S-curve, cosine, sine)
   - **Fade Timing**: Customizable fade duration and timing
   - **Fade Presets**: Pre-defined fade settings for common use cases

2. **Crossfade Transitions**
   - **Track Crossfade**: Seamless transition between different music tracks
   - **Stem Crossfade**: Crossfade between different music stems
   - **State Crossfade**: Crossfade between different music states
   - **Loop Crossfade**: Seamless looping with crossfade
   - **Reverse Crossfade**: Crossfade with reverse playback

3. **Synchronized Transitions**
   - **Beat Synchronization**: Transitions aligned to musical beats
   - **Phrase Synchronization**: Transitions aligned to musical phrases
   - **Measure Synchronization**: Transitions aligned to musical measures
   - **Tempo Matching**: Automatic tempo matching for smooth transitions
   - **Key Matching**: Automatic key matching for harmonic transitions

4. **Advanced Transition Effects**
   - **Reverb Transitions**: Reverb effects during transitions
   - **Filter Transitions**: Filter sweeps during transitions
   - **Echo Transitions**: Echo effects during transitions
   - **Distortion Transitions**: Distortion effects during transitions
   - **Chorus/Flanger**: Modulation effects during transitions

5. **Transition Timing Control**
   - **Instant Transitions**: Immediate cut between tracks
   - **Fast Transitions**: Quick transitions (0.1-0.5 seconds)
   - **Medium Transitions**: Standard transitions (0.5-2.0 seconds)
   - **Slow Transitions**: Slow transitions (2.0-5.0 seconds)
   - **Custom Timing**: User-defined transition duration
   - **Auto Timing**: Automatic timing based on music analysis

#### Technical Implementation

1. **Transition Engine Architecture**
   - **Multi-Track Mixer**: Mix multiple audio tracks simultaneously
   - **Volume Automation**: Real-time volume control for smooth fades
   - **Crossfade Buffer**: Buffer management for crossfade transitions
   - **Synchronization Engine**: Beat and phrase synchronization
   - **Effect Processing**: Real-time effect processing during transitions

2. **Fade Curve Algorithms**
   - **Linear Fade**: Constant rate volume change
   - **Exponential Fade**: Exponential volume change (natural fade)
   - **Logarithmic Fade**: Logarithmic volume change
   - **S-Curve Fade**: S-shaped volume curve for smooth transitions
   - **Cosine Fade**: Cosine-based volume curve
   - **Sine Fade**: Sine-based volume curve
   - **Custom Curves**: User-defined fade curves

3. **Synchronization System**
   - **Beat Detection**: Real-time beat detection and analysis
   - **Tempo Tracking**: Continuous tempo tracking and adjustment
   - **Key Detection**: Musical key detection for harmonic matching
   - **Phrase Analysis**: Musical phrase structure analysis
   - **Loop Point Detection**: Automatic loop point detection

4. **Transition State Management**
   - **Transition Queue**: Queue multiple transitions for smooth chaining
   - **Transition Priority**: Priority system for transition conflicts
   - **Transition Interruption**: Graceful handling of transition interruptions
   - **Transition Rollback**: Rollback capability for failed transitions
   - **Transition History**: Track transition history for analysis

#### Music Transition UI

```
┌─────────────────────────────────────────────────────────────┐
│                    MUSIC TRANSITION SYSTEM                  │
├─────────────────────────────────────────────────────────────┤
│ Current: forest_theme.wav | Next: combat_theme.wav         │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────────────────────────────────┐ │
│ │ Transition  │ │              Fade Controls               │ │
│ │    Type     │ │                                         │ │
│ │             │ │  Fade In:  [████████] 2.0s [Linear]     │ │
│ │ [Fade In]   │ │  Fade Out: [████████] 1.5s [Exponential]│ │
│ │ [Fade Out]  │ │  Crossfade: [████████] 3.0s [S-Curve]   │ │
│ │ [Crossfade] │ │  To Silence: [████████] 1.0s [Linear]   │ │
│ │ [Cut]       │ │  From Silence: [████████] 2.5s [Cosine] │ │
│ │ [Loop]      │ │                                         │ │
│ │ [Reverse]   │ │  Effects: [Reverb] [Filter] [Echo]      │ │
│ └─────────────┘ └─────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│ Timing: [Instant] [Fast] [Medium] [Slow] [Custom] [Auto]   │
│ Sync: [Beat] [Phrase] [Measure] [None] | Curve: [Custom]   │
├─────────────────────────────────────────────────────────────┤
│ [Preview] [Apply] [Save] [Load] [Reset] [Settings]         │
└─────────────────────────────────────────────┘
```

#### Transition Categories

1. **Track Transitions**
   - **Level Transitions**: Between different level themes
   - **State Transitions**: Between different gameplay states
   - **Boss Transitions**: Special transitions for boss battles
   - **Menu Transitions**: Transitions between menu and gameplay
   - **Cutscene Transitions**: Transitions for cutscenes

2. **Stem Transitions**
   - **Stem Addition**: Adding stems to existing music
   - **Stem Removal**: Removing stems from existing music
   - **Stem Replacement**: Replacing one stem with another
   - **Stem Crossfade**: Crossfading between different stems
   - **Stem Fade**: Fading individual stems in/out

3. **Silence Transitions**
   - **To Silence**: Fading music to complete silence
   - **From Silence**: Fading music in from silence
   - **Silence Duration**: Configurable silence duration
   - **Silence Effects**: Effects during silence periods
   - **Silence Loops**: Looping silence periods

4. **Special Transitions**
   - **Loop Transitions**: Seamless looping with crossfade
   - **Reverse Transitions**: Reverse playback transitions
   - **Pitch Transitions**: Pitch-shifted transitions
   - **Tempo Transitions**: Tempo-changed transitions
   - **Key Transitions**: Key-changed transitions

#### File Organization

```
assets/music/
├── transitions/
│   ├── fade_curves/
│   │   ├── linear.json
│   │   ├── exponential.json
│   │   ├── logarithmic.json
│   │   ├── s_curve.json
│   │   ├── cosine.json
│   │   └── sine.json
│   ├── presets/
│   │   ├── combat_transitions.json
│   │   ├── menu_transitions.json
│   │   ├── boss_transitions.json
│   │   └── cutscene_transitions.json
│   └── effects/
│       ├── reverb_transitions.json
│       ├── filter_transitions.json
│       └── echo_transitions.json
├── stems/
│   ├── stem_1_rhythm/
│   ├── stem_2_harmony/
│   ├── stem_3_melody/
│   └── stem_4_effects/
└── compositions/
    ├── forest_theme.json
    ├── combat_theme.json
    └── boss_theme.json
```

#### Performance Optimization

1. **Transition Caching**
   - **Precomputed Transitions**: Pre-compute common transitions
   - **Transition Buffers**: Buffer management for smooth transitions
   - **Memory Management**: Efficient memory usage for transitions
   - **Lazy Loading**: Load transitions on demand

2. **Real-Time Processing**
   - **Low-Latency Transitions**: Minimal delay for real-time transitions
   - **Smooth Fades**: Smooth volume changes without artifacts
   - **Effect Processing**: Real-time effect processing during transitions
   - **Synchronization**: Accurate beat and phrase synchronization

3. **Quality Control**
   - **Artifact Reduction**: Minimize transition artifacts
   - **Volume Matching**: Automatic volume matching between tracks
   - **Tempo Matching**: Automatic tempo matching for smooth transitions
   - **Key Matching**: Automatic key matching for harmonic transitions

#### Technical Implementation

1. **Audio Engine Integration**
   - **Multi-Channel Audio**: Support for 4+ simultaneous audio streams
   - **Low-Latency Processing**: Real-time audio processing
   - **Memory Management**: Efficient audio buffer management
   - **Format Support**: Multiple audio format support (WAV, OGG, FLAC)

2. **Music Analysis Engine**
   - **FFT Analysis**: Fast Fourier Transform for frequency analysis
   - **Onset Detection**: Beat and note onset detection
   - **Pitch Detection**: Musical pitch and key detection
   - **Tempo Tracking**: Real-time tempo analysis
   - **Harmonic Analysis**: Chord and scale detection

3. **Transition Engine**
   - **Synchronization**: Beat and phrase synchronization
   - **Crossfading**: Smooth volume transitions
   - **Pitch Matching**: Key and scale matching
   - **Tempo Matching**: BPM synchronization
   - **Loop Points**: Seamless loop transitions

4. **State Management**
   - **State Machine**: Music state management system
   - **Priority System**: Event priority handling
   - **Fade Curves**: Customizable transition curves
   - **Queue System**: Music transition queuing
   - **Interruption Handling**: Graceful interruption management

#### Music System UI

```
┌─────────────────────────────────────────────────────────────┐
│                    DYNAMIC MUSIC SYSTEM                     │
├─────────────────────────────────────────────────────────────┤
│ Current State: [Combat] | BPM: 120 | Key: C Major         │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────────────────────────────────┐ │
│ │   Active    │ │              Stem Controls               │ │
│ │   Stems     │ │                                         │ │
│ │             │ │  Stem 1 (Rhythm)    [████████] 100%    │ │
│ │ ✓ Stem 1    │ │  Stem 2 (Harmony)   [████████] 100%    │ │
│ │ ✓ Stem 2    │ │  Stem 3 (Melody)    [████████] 100%    │ │
│ │ ✓ Stem 3    │ │  Stem 4 (Effects)   [████░░░░] 50%     │ │
│ │ ○ Stem 4    │ │                                         │ │
│ │             │ │  Transition: [Beat Sync] [Crossfade]    │ │
│ │ Next State: │ │  Duration: [2.0s] [4.0s] [8.0s]        │ │
│ │ [Intense]   │ │                                         │ │
│ └─────────────┘ └─────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│ [Play] [Pause] [Stop] [Loop] [Random] [Export] [Settings]  │
└─────────────────────────────────────────────────────────────┘
```

#### Music File Organization

```
assets/music/
├── stems/
│   ├── stem_1_rhythm/
│   │   ├── forest_calm.wav
│   │   ├── forest_combat.wav
│   │   └── forest_intense.wav
│   ├── stem_2_harmony/
│   │   ├── forest_calm.wav
│   │   ├── forest_combat.wav
│   │   └── forest_intense.wav
│   ├── stem_3_melody/
│   │   ├── forest_calm.wav
│   │   ├── forest_combat.wav
│   │   └── forest_intense.wav
│   └── stem_4_effects/
│       ├── forest_calm.wav
│       ├── forest_combat.wav
│       └── forest_intense.wav
├── compositions/
│   ├── forest_theme.json
│   ├── cave_theme.json
│   └── boss_theme.json
└── analysis/
    ├── forest_theme_analysis.json
    ├── cave_theme_analysis.json
    └── boss_theme_analysis.json
```

## Sound Test Implementation

### Classic 16-Bit Brawler Sound Test Features

The sound test system will replicate the classic sound test functionality found in 16-bit brawler games like Final Fight, Streets of Rage, and Double Dragon, providing players with a comprehensive audio testing and preview system.

#### Core Sound Test Features

1. **Audio Category Navigation**
   - Browse through SFX, BGM, Voice, and Ambient sound categories
   - Hierarchical organization with subcategories
   - Quick category switching with keyboard shortcuts

2. **Audio Playback Controls**
   - Play/Stop/Pause functionality for each sound
   - Loop mode for continuous playback
   - Volume control per category and individual sounds
   - Seek/scrub through longer audio files

3. **Audio Information Display**
   - File name and path
   - Duration and current position
   - Audio format (WAV, OGG, MP3, etc.)
   - Sample rate, bitrate, and channel information
   - File size and compression ratio

4. **Classic 16-Bit Features**
   - **Sound Waveform Display**: Visual representation of audio waveforms
   - **Frequency Analysis**: Real-time frequency spectrum display
   - **Audio Mixer**: Individual volume sliders for each category
   - **Sound Library**: Browse all available sounds with preview
   - **Favorites System**: Mark favorite sounds for quick access
   - **Playlist Creation**: Create custom sound playlists

5. **Advanced Features**
   - **Audio Export**: Export individual sounds or playlists
   - **Audio Import**: Import custom sounds (mod support)
   - **Audio Recording**: Record gameplay audio for analysis
   - **Audio Comparison**: A/B testing between different sounds
   - **Audio Statistics**: Usage statistics and play counts

#### Sound Test UI Layout

```
┌─────────────────────────────────────────────────────────────┐
│                    SOUND TEST                              │
├─────────────────────────────────────────────────────────────┤
│ Categories: [SFX] [Music] [Voice] [Ambient] [All]         │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────────────────────────────────┐ │
│ │   Sound     │ │              Waveform                   │ │
│ │   List      │ │              Display                    │ │
│ │             │ │                                         │ │
│ │ • Combat    │ │  ████████████████████████████████      │ │
│ │ • Weapons   │ │  ████████████████████████████████      │ │
│ │ • Spells    │ │  ████████████████████████████████      │ │
│ │ • Items     │ │  ████████████████████████████████      │ │
│ │ • UI        │ │  ████████████████████████████████      │ │
│ │ • Ambient   │ │  ████████████████████████████████      │ │
│ └─────────────┘ └─────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│ [Play] [Stop] [Loop] [Volume: ████████░░] [Export] [Info] │
├─────────────────────────────────────────────────────────────┤
│ Currently Playing: combat_hit_01.wav (0:02/0:02)          │
│ Format: WAV | Sample Rate: 44.1kHz | Channels: Stereo     │
└─────────────────────────────────────────────────────────────┘
```

#### Technical Implementation

1. **Audio Engine Integration**
   - Integrate with existing audio system
   - Support for multiple audio formats
   - Real-time audio processing and visualization
   - Low-latency audio playback

2. **File Management**
   - Scan and catalog all audio files
   - Generate metadata for each sound
   - Support for compressed and uncompressed formats
   - Automatic file organization and categorization

3. **Performance Optimization**
   - Lazy loading of audio files
   - Audio streaming for large files
   - Memory management for audio buffers
   - Efficient audio mixing and processing

4. **User Experience**
   - Intuitive navigation with keyboard and mouse
   - Responsive UI with smooth animations
   - Accessibility features for audio testing
   - Customizable interface layout

#### Sound Test Categories

1. **Sound Effects (SFX)**
   - Combat sounds (hits, misses, blocks, parries)
   - Weapon sounds (swings, impacts, reloads)
   - Spell sounds (casting, impacts, effects)
   - Item sounds (pickups, uses, drops)
   - UI sounds (clicks, hovers, notifications)
   - Environmental sounds (doors, chests, switches)

2. **Background Music (BGM)**
   - Main menu and character selection themes
   - Level-specific background music
   - Combat and boss battle themes
   - Victory and defeat themes
   - Ambient and atmospheric music

3. **Voice Lines**
   - Character attack and combat voices
   - Hurt, death, and victory voices
   - Taunt and encouragement lines
   - Warning and alert voices
   - Character-specific voice sets

4. **Ambient Sounds**
   - Environmental audio (wind, rain, fire)
   - Atmospheric sounds (cave echoes, city noise)
   - Weather effects (thunder, storms)
   - Magical and supernatural sounds

## Implementation Guidelines

### Character Limit Guidelines

1. **UI Elements**: Keep under 20 characters for buttons, 30 for titles
2. **Item Names**: Keep under 30 characters for readability
3. **Character Names**: Keep under 25 characters for display
4. **Descriptions**: Keep under 50 characters for tooltips
5. **Error Messages**: Keep under 100 characters for popups
6. **Sound Test**: Keep under 40 characters for sound names and categories

### Language-Specific Considerations

#### RTL Languages (Arabic)
- Reserve 20% extra space for RTL text expansion
- Consider text direction in UI layout
- Test with longer Arabic text

#### Asian Languages (Korean, Japanese, Chinese)
- Reserve 30% extra space for character expansion
- Consider vertical text layout options
- Test with complex characters

#### European Languages (FIGS, BRPT)
- Reserve 15% extra space for text expansion
- Consider compound words in German
- Test with accented characters

### String ID Naming Convention

```
category.subcategory.specific_element
```

Examples:
- `ui.menu.play` - UI menu play button
- `items.weapon.sword` - Items weapon sword
- `characters.class.fighter` - Characters class fighter
- `combat.action.attack` - Combat action attack

### Translation File Structure

```json
{
  "ui": {
    "main_menu": "Main Menu",
    "play": "Play",
    "settings": "Settings"
  },
  "gameplay": {
    "level": "Level",
    "health": "Health",
    "mana": "Mana"
  }
}
```

### Testing Requirements

1. **Character Limit Testing**: Verify all strings fit within UI elements
2. **RTL Testing**: Test Arabic text layout and direction
3. **Font Testing**: Test with different font sizes and styles
4. **Context Testing**: Verify strings make sense in context
5. **Pluralization Testing**: Test plural forms in all languages
6. **Interpolation Testing**: Test string interpolation with variables

## Implementation Priority

### Phase 1: Core Localization System (Week 1) ✅ COMPLETE
- [x] String ID system and registry
- [x] Translation file format and loading
- [x] Language switching and fallback
- [x] Basic UI integration

### Phase 2: Advanced Features (Week 2) ✅ COMPLETE
- [x] Pluralization system
- [x] String interpolation
- [x] UI mirroring for RTL languages
- [x] Character limits and validation

### Phase 3: Content Translation (Week 3) ✅ COMPLETE
- [x] UI strings translation
- [x] Gameplay strings translation
- [x] Combat strings translation
- [x] Character strings translation

### Phase 4: Audio Systems (Week 4) ✅ COMPLETE
- [x] Sound test system strings
- [x] Dynamic music system strings
- [x] SFX pitch variation strings
- [x] Music transition strings

### Phase 5: Sound Test Implementation (Week 5) 🔄 IN PROGRESS
- [ ] Sound test UI implementation
- [ ] Audio playback controls
- [ ] Waveform display
- [ ] Export/import functionality

### Phase 6: SFX Pitch Variation System Implementation (Week 6) 📋 PLANNED
- [ ] Pitch shifting algorithms
- [ ] Real-time processing
- [ ] Variation management
- [ ] UI controls

### Phase 7: Dynamic Music System Implementation (Week 7) 📋 PLANNED
- [ ] 4-stem architecture
- [ ] Real-time analysis
- [ ] Intelligent transitions
- [ ] Gameplay integration

## File Organization

```
assets/translations/
├── en.json          # English (base language)
├── fr.json          # French
├── it.json          # Italian
├── de.json          # German
├── es.json          # Spanish
├── pt-BR.json       # Brazilian Portuguese
├── ko.json          # Korean
├── ja.json          # Japanese
├── zh-CN.json       # Chinese Simplified
├── zh-TW.json       # Chinese Traditional
├── tr.json          # Turkish
└── ar.json          # Arabic
```

## Quality Assurance

### Translation Review Process
1. **Native Speaker Review**: Each language reviewed by native speakers
2. **Context Review**: Verify strings work in game context
3. **UI Review**: Verify strings fit UI elements
4. **Cultural Review**: Ensure cultural appropriateness
5. **Technical Review**: Verify string interpolation and pluralization

### Automated Testing
1. **Character Limit Validation**: Automated check for character limits
2. **Missing Translation Detection**: Automated check for missing strings
3. **String Interpolation Testing**: Automated test of variable substitution
4. **Pluralization Testing**: Automated test of plural forms

This comprehensive plan ensures all game strings are properly localized with appropriate character limits for UI elements across all target languages.
