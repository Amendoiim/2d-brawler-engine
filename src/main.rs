//! 2D Brawler Engine - Main Entry Point
//! 
//! A custom 2D brawler engine inspired by Streets of Rage 4.
//! Built with Rust for high performance and cross-platform compatibility.

use anyhow::Result;
use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod engine;
mod game;
mod platform;

use engine::Engine;
use game::{Position, Velocity, Sprite, MovementSystem, InputMovementSystem, 
          PlayerInputSystem, CharacterControllerSystem, CombatSystem, DamageSystem};
use engine::animation::{Animation, AnimationFrame, AnimationController, FrameEffect};
use engine::particles::ParticleEmitter;
use engine::level::{LevelGenerator, LevelConfig, GenerationAlgorithm, AlgorithmParams, LevelType, LevelFilters, Checkpoint, CheckpointType, PlayerState};
use glam::Vec2;
use game::abilities::ResourceType;
use game::combat::{SpecialMove, ResourceManager, ComboResult};

/// Main application entry point
fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting 2D Brawler Engine v{}", env!("CARGO_PKG_VERSION"));

    // Create event loop
    let event_loop = EventLoop::new()?;
    
    // Create window
    let window = WindowBuilder::new()
        .with_title("2D Brawler Engine")
        .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
        .with_resizable(true)
        .build(&event_loop)?;

    // Initialize engine
    let mut engine = Engine::new(window)?;
    
    // Create test scene
    create_test_scene(&mut engine);

    // Run main game loop
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        engine.resize(physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        match engine.render() {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("Render error: {}", e);
                                elwt.exit();
                            }
                        }
                    }
                    _ => {
                        engine.handle_window_event(event);
                    }
                }
            }
            Event::AboutToWait => {
                // Update engine systems
                if let Err(e) = engine.update(0.016) { // ~60 FPS
                    log::error!("Engine update error: {}", e);
                }
                
                // Request a redraw
                engine.window().request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}

/// Create a test scene with some basic entities
fn create_test_scene(engine: &mut Engine) {
    // Create test scenes
    engine.scene_mut().create_test_scene("level_1");
    engine.scene_mut().create_test_scene("level_2");
    
    // Load the first scene
    if let Err(e) = engine.load_scene("level_1") {
        log::error!("Failed to load scene: {}", e);
    }
    
    // Add game systems
    engine.ecs_mut().add_system(MovementSystem);
    engine.ecs_mut().add_system(InputMovementSystem::new());
    engine.ecs_mut().add_system(PlayerInputSystem::new());
    engine.ecs_mut().add_system(CharacterControllerSystem::new());
    engine.ecs_mut().add_system(CombatSystem);
    engine.ecs_mut().add_system(DamageSystem);
    
    // Test audio system
    test_audio_system(engine);
    
    // Test asset system
    test_asset_system(engine);
    
    // Test game systems
    test_game_systems(engine);
    
    // Test animation system
    test_animation_system(engine);
    
    // Test particle system (commented out for now)
    // test_particle_system(engine);
    
    // Test level generation system
    test_level_generation_system(engine);
    
    // Test level types generation
    test_level_types_generation(engine);

    // Test level progression system
    test_level_progression_system(engine);
    
    // Test advanced combat and abilities system
    test_advanced_combat_system(engine);
    
    // Test character system
    test_character_system(engine);
    test_enemy_system(engine);
    test_items_system(engine);
    
    // Test localization system
    test_localization_system(engine);
    test_comprehensive_string_registry(engine);
    
    info!("Created test scene with game systems");
}

/// Test the audio system
fn test_audio_system(engine: &mut Engine) {
    // Create test sounds
    if let Err(e) = engine.audio_mut().create_test_sound("test_tone", 440.0, 1.0) {
        log::error!("Failed to create test sound: {}", e);
    }
    
    if let Err(e) = engine.audio_mut().create_test_sound("beep", 800.0, 0.5) {
        log::error!("Failed to create beep sound: {}", e);
    }
    
    // Set volume
    engine.audio_mut().set_volume(0.5);
    engine.audio_mut().set_music_volume(0.3);
    
    log::info!("Audio system test completed. Loaded {} sounds", engine.audio_mut().sound_count());
}

/// Test the asset system
fn test_asset_system(engine: &mut Engine) {
    // Create test textures
    if let Err(e) = engine.asset_manager_mut().create_test_texture("red_square", 64, 64, [255, 0, 0]) {
        log::error!("Failed to create test texture: {}", e);
    }

    if let Err(e) = engine.asset_manager_mut().create_test_texture("blue_circle", 32, 32, [0, 0, 255]) {
        log::error!("Failed to create blue texture: {}", e);
    }

    // Test asset statistics
    let stats = engine.asset_manager_mut().get_asset_statistics();
    log::info!("Asset Statistics:");
    for (key, value) in stats {
        log::info!("  {}: {}", key, value);
    }

    // Test memory usage
    let memory_usage = engine.asset_manager_mut().get_memory_usage();
    let memory_percentage = engine.asset_manager_mut().get_memory_usage_percentage();
    log::info!("Memory usage: {} bytes ({:.1}%)", memory_usage, memory_percentage);

    // Test loading queue
    let (queue_size, queue_items) = engine.asset_manager_mut().get_loading_queue_status();
    log::info!("Loading queue: {} items", queue_size);
    if !queue_items.is_empty() {
        log::info!("  Queue items: {:?}", queue_items);
    }

    // Test asset access tracking
    if let Some(texture_id) = engine.asset_manager_mut().get_texture("red_square") {
        log::info!("Retrieved red_square texture with ID: {}", texture_id);
    }

    log::info!("Asset system test completed. Loaded {} assets", engine.asset_manager_mut().get_loaded_assets().len());
}

/// Test the game systems
fn test_game_systems(engine: &mut Engine) {
    // Test scene management
    if let Some(current_scene) = engine.scene_mut().current_scene() {
        log::info!("Current scene: {}", current_scene);
    }
    
    // Test entity count
    let entity_count = engine.ecs_mut().entity_count();
    log::info!("ECS entity count: {}", entity_count);
    
    // Test player entities
    let player_entities = engine.ecs_mut().query::<game::Player>();
    log::info!("Player entities: {}", player_entities.len());
    
    // Test combat entities
    let combat_entities = engine.ecs_mut().query::<game::Combat>();
    log::info!("Combat entities: {}", combat_entities.len());
    
    // Test scene transition
    engine.scene_mut().start_transition("level_2");
    log::info!("Started scene transition to level_2");
    
    log::info!("Game systems test completed");
}

/// Test the animation system
fn test_animation_system(engine: &mut Engine) {
    // Create test animations
    let mut idle_animation = Animation::new("idle".to_string());
    idle_animation.set_looping(true);
    idle_animation.set_speed(1.0);
    idle_animation.set_priority(1);

    // Add frames to idle animation
    for i in 0..4 {
        let frame = AnimationFrame {
            texture_id: 1000 + i,
            duration: 0.25,
            offset_x: i as f32 * 32.0,
            offset_y: 0.0,
            width: 32.0,
            height: 32.0,
            color: [1.0, 1.0, 1.0, 1.0],
            effects: Vec::new(),
        };
        idle_animation.add_frame(frame);
    }

    let mut walk_animation = Animation::new("walk".to_string());
    walk_animation.set_looping(true);
    walk_animation.set_speed(1.5);
    walk_animation.set_priority(2);

    // Add frames to walk animation
    for i in 0..6 {
        let frame = AnimationFrame {
            texture_id: 2000 + i,
            duration: 0.15,
            offset_x: i as f32 * 32.0,
            offset_y: 32.0,
            width: 32.0,
            height: 32.0,
            color: [1.0, 1.0, 1.0, 1.0],
            effects: Vec::new(),
        };
        walk_animation.add_frame(frame);
    }

    let mut attack_animation = Animation::new("attack".to_string());
    attack_animation.set_looping(false);
    attack_animation.set_speed(2.0);
    attack_animation.set_priority(3);

    // Add frames to attack animation
    for i in 0..3 {
        let frame = AnimationFrame {
            texture_id: 3000 + i,
            duration: 0.1,
            offset_x: i as f32 * 32.0,
            offset_y: 64.0,
            width: 32.0,
            height: 32.0,
            color: [1.0, 1.0, 1.0, 1.0],
            effects: vec![
                if i == 1 {
                    FrameEffect::ScreenShake { intensity: 0.1, duration: 0.1 }
                } else {
                    FrameEffect::SoundEffect { sound_name: "attack".to_string(), volume: 0.8 }
                }
            ],
        };
        attack_animation.add_frame(frame);
    }

    // Register animations
    engine.animation_mut().register_animation("idle".to_string(), idle_animation);
    engine.animation_mut().register_animation("walk".to_string(), walk_animation);
    engine.animation_mut().register_animation("attack".to_string(), attack_animation);

    // Create animation controllers for player entities
    let player_entities = engine.ecs_mut().query::<game::Player>();
    for entity in player_entities {
        let controller = AnimationController::create_character_controller(entity);
        engine.animation_mut().add_controller(entity, controller);
    }

    log::info!("Animation system test completed. Registered {} animations", 3);
}

/// Test the particle system
fn test_particle_system(engine: &mut Engine) {
    // Create test particle effects
    let player_entities = engine.ecs_mut().query::<game::Player>();
    
    for (i, entity) in player_entities.iter().enumerate() {
        // Create explosion effect for first player
        if i == 0 {
            engine.particles_mut().create_explosion_effect(*entity, glam::Vec2::new(100.0, 100.0), 1.0);
        }
        
        // Create trail effect for all players
        engine.particles_mut().create_trail_effect(*entity, glam::Vec2::new(50.0 + i as f32 * 100.0, 200.0));
    }

    // Create spark effects
    let spark_entity = engine.ecs_mut().create_entity();
    engine.particles_mut().create_spark_effect(
        spark_entity,
        glam::Vec2::new(300.0, 300.0),
        glam::Vec2::new(1.0, 0.0)
    );

    log::info!("Particle system test completed. Created particle effects for {} entities", player_entities.len());
}

/// Test the level generation system
fn test_level_generation_system(engine: &mut Engine) {
    log::info!("Testing level generation system...");
    
    // Test basic level generation
    let config = LevelConfig {
        room_count_range: (3, 6),
        room_size_range: (5, 12),
        level_width: 40,
        level_height: 40,
        difficulty: 1,
        biome_type: "forest".to_string(),
        seed: Some(12345),
    };
    
    // Test basic level generation
    match engine.level_generator_mut().generate_level(config.clone()) {
        Ok(level) => {
            log::info!("Successfully generated basic level: {}", level.id);
            log::info!("Level dimensions: {}x{}", level.width, level.height);
            log::info!("Number of rooms: {}", level.rooms.len());
            log::info!("Number of connections: {}", level.connections.len());
            log::info!("Number of spawn points: {}", level.spawn_points.len());
            log::info!("Biome: {}", level.biome);
            log::info!("Difficulty: {}", level.difficulty);
            
            // Count different tile types
            let mut tile_counts = std::collections::HashMap::new();
            for x in 0..level.width as usize {
                for y in 0..level.height as usize {
                    let tile = level.tiles[x][y];
                    *tile_counts.entry(tile).or_insert(0) += 1;
                }
            }
            
            log::info!("Tile distribution:");
            for (tile, count) in tile_counts {
                log::info!("  {:?}: {}", tile, count);
            }
        },
        Err(e) => {
            log::error!("Failed to generate basic level: {}", e);
        }
    }
    
    // Test advanced level generation
    match engine.level_generator_mut().generate_level_advanced(config, GenerationAlgorithm::RoomBased) {
        Ok(level) => {
            log::info!("Successfully generated advanced level: {}", level.id);
            log::info!("Advanced level dimensions: {}x{}", level.width, level.height);
            log::info!("Advanced level rooms: {}", level.rooms.len());
            log::info!("Advanced level connections: {}", level.connections.len());
            log::info!("Advanced level spawn points: {}", level.spawn_points.len());
            log::info!("Advanced level biome: {}", level.biome);
            log::info!("Advanced level difficulty: {}", level.difficulty);
            
            // Count different tile types in advanced level
            let mut tile_counts = std::collections::HashMap::new();
            for x in 0..level.width as usize {
                for y in 0..level.height as usize {
                    let tile = level.tiles[x][y];
                    *tile_counts.entry(tile).or_insert(0) += 1;
                }
            }
            
            log::info!("Advanced level tile distribution:");
            for (tile, count) in tile_counts {
                log::info!("  {:?}: {}", tile, count);
            }
            
            // Test different generation algorithms
            test_different_algorithms(engine);
            
            // Test multi-biome level generation
            test_multi_biome_generation(engine);
            
            // Test interactive objects generation
            test_interactive_objects_generation(engine);
            
            // Test background layers generation
            test_background_layers_generation(engine);
            
            // Test atmospheric effects generation
            test_atmospheric_effects_generation(engine);
        },
        Err(e) => {
            log::error!("Failed to generate advanced level: {}", e);
        }
    }
}

/// Test different level generation algorithms
fn test_different_algorithms(engine: &mut Engine) {
    let config = LevelConfig {
        room_count_range: (2, 4),
        room_size_range: (4, 8),
        level_width: 30,
        level_height: 30,
        difficulty: 1,
        biome_type: "desert".to_string(),
        seed: Some(54321),
    };
    
    let algorithms = vec![
        GenerationAlgorithm::RoomBased,
        // Note: Other algorithms are placeholders for now
    ];
    
    for algorithm in algorithms {
        match engine.level_generator_mut().generate_level_advanced(config.clone(), algorithm.clone()) {
            Ok(level) => {
                log::info!("Successfully generated level with {:?} algorithm", algorithm);
                log::info!("  Level ID: {}", level.id);
                log::info!("  Rooms: {}, Connections: {}, Spawn Points: {}", 
                          level.rooms.len(), level.connections.len(), level.spawn_points.len());
            },
            Err(e) => {
                log::warn!("Failed to generate level with {:?} algorithm: {}", algorithm, e);
            }
        }
    }
}

/// Test multi-biome level generation
fn test_multi_biome_generation(engine: &mut Engine) {
    let config = LevelConfig {
        room_count_range: (4, 8),
        room_size_range: (6, 12),
        level_width: 60,
        level_height: 40,
        difficulty: 2,
        biome_type: "forest".to_string(), // This will be overridden
        seed: Some(98765),
    };
    
    // Test multi-biome level with forest -> desert -> arctic transition
    let biomes = vec![
        "forest".to_string(),
        "desert".to_string(),
        "arctic".to_string(),
    ];
    
    match engine.level_generator_mut().generate_multi_biome_level(config, biomes) {
        Ok(level) => {
            log::info!("Successfully generated multi-biome level: {}", level.id);
            log::info!("Multi-biome level dimensions: {}x{}", level.width, level.height);
            log::info!("Multi-biome level rooms: {}", level.rooms.len());
            log::info!("Multi-biome level connections: {}", level.connections.len());
            log::info!("Multi-biome level spawn points: {}", level.spawn_points.len());
            log::info!("Multi-biome level biome: {}", level.biome);
            log::info!("Multi-biome level difficulty: {}", level.difficulty);
            
            // Count different tile types in multi-biome level
            let mut tile_counts = std::collections::HashMap::new();
            for x in 0..level.width as usize {
                for y in 0..level.height as usize {
                    let tile = level.tiles[x][y];
                    *tile_counts.entry(tile).or_insert(0) += 1;
                }
            }
            
            log::info!("Multi-biome level tile distribution:");
            for (tile, count) in tile_counts {
                log::info!("  {:?}: {}", tile, count);
            }
        },
        Err(e) => {
            log::error!("Failed to generate multi-biome level: {}", e);
        }
    }
}

/// Test interactive objects generation
fn test_interactive_objects_generation(engine: &mut Engine) {
    let config = LevelConfig {
        room_count_range: (5, 10),
        room_size_range: (8, 15),
        level_width: 50,
        level_height: 50,
        difficulty: 3,
        biome_type: "cave".to_string(),
        seed: Some(11111),
    };
    
    match engine.level_generator_mut().generate_level_with_objects(config, GenerationAlgorithm::RoomBased) {
        Ok(level) => {
            log::info!("Successfully generated level with interactive objects: {}", level.id);
            log::info!("Level with objects dimensions: {}x{}", level.width, level.height);
            log::info!("Level with objects rooms: {}", level.rooms.len());
            log::info!("Level with objects connections: {}", level.connections.len());
            log::info!("Level with objects spawn points: {}", level.spawn_points.len());
            log::info!("Level with objects biome: {}", level.biome);
            log::info!("Level with objects difficulty: {}", level.difficulty);
            
            // Get interactive objects
            let interactive_objects = engine.level_generator_mut().get_interactive_objects();
            log::info!("Interactive objects count: {}", interactive_objects.len());
            
            // Count objects by type
            let mut object_counts = std::collections::HashMap::new();
            for obj in interactive_objects {
                *object_counts.entry(obj.object_type).or_insert(0) += 1;
            }
            
            log::info!("Interactive objects by type:");
            for (object_type, count) in object_counts {
                log::info!("  {:?}: {}", object_type, count);
            }
            
            // Test object interaction
            if let Some(first_object) = interactive_objects.first() {
                log::info!("Testing interaction with object: {}", first_object.id);
                log::info!("  Object type: {:?}", first_object.object_type);
                log::info!("  Object position: ({}, {})", first_object.x, first_object.y);
                log::info!("  Object state: {:?}", first_object.state);
                log::info!("  Object health: {}/{}", first_object.health, first_object.max_health);
                log::info!("  Object active: {}", first_object.active);
                
                // Test damaging the object
                if first_object.properties.destructible {
                    let object_id = first_object.id.clone();
                    match engine.level_generator_mut().damage_interactive_object(&object_id, 25.0) {
                        Ok(destroyed) => {
                            if destroyed {
                                log::info!("  Object destroyed!");
                            } else {
                                log::info!("  Object damaged but not destroyed");
                            }
                        },
                        Err(e) => {
                            log::error!("  Failed to damage object: {}", e);
                        }
                    }
                }
            }
        },
        Err(e) => {
            log::error!("Failed to generate level with interactive objects: {}", e);
        }
    }
}

/// Test background layers generation
fn test_background_layers_generation(engine: &mut Engine) {
    let screen_width = 1920.0;
    let screen_height = 1080.0;
    
    // Test different biomes
    let biomes = vec!["forest", "desert", "arctic", "cave", "lava"];
    
    for biome in biomes {
        match engine.level_generator_mut().create_background_layers(biome, screen_width, screen_height) {
            Ok(_) => {
                log::info!("Successfully created background layers for biome: {}", biome);
                
                // Get background layers
                let background_layers = engine.level_generator_mut().get_background_layers();
                log::info!("  Background layers count: {}", background_layers.len());
                
                // Count layers by type
                let mut layer_counts = std::collections::HashMap::new();
                for layer in background_layers {
                    *layer_counts.entry(layer.layer_type).or_insert(0) += 1;
                }
                
                log::info!("  Background layers by type:");
                for (layer_type, count) in layer_counts {
                    log::info!("    {:?}: {}", layer_type, count);
                }
                
                // Test layer properties
                for layer in background_layers {
                    log::info!("  Layer: {}", layer.id);
                    log::info!("    Depth: {:.2}", layer.depth);
                    log::info!("    Position: ({:.2}, {:.2})", layer.position.x, layer.position.y);
                    log::info!("    Velocity: ({:.2}, {:.2})", layer.velocity.x, layer.velocity.y);
                    log::info!("    Opacity: {:.2}", layer.opacity);
                    log::info!("    Texture ID: {}", layer.texture_id);
                    log::info!("    Color: [{:.2}, {:.2}, {:.2}, {:.2}]", 
                              layer.color[0], layer.color[1], layer.color[2], layer.color[3]);
                    log::info!("    Visible: {}", layer.visible);
                    log::info!("    Repeat X: {}, Repeat Y: {}", layer.repeat_x, layer.repeat_y);
                }
                
                // Test layer updates
                engine.level_generator_mut().set_camera_position(glam::Vec2::new(100.0, 50.0));
                engine.level_generator_mut().set_camera_velocity(glam::Vec2::new(10.0, 0.0));
                engine.level_generator_mut().update_background_layers(0.016); // ~60 FPS
                
                log::info!("  Updated background layers with camera movement");
                
                // Get visible layers
                let visible_layers = engine.level_generator_mut().get_visible_background_layers();
                log::info!("  Visible layers count: {}", visible_layers.len());
            },
            Err(e) => {
                log::error!("Failed to create background layers for biome {}: {}", biome, e);
            }
        }
    }
}

/// Test atmospheric effects generation
fn test_atmospheric_effects_generation(engine: &mut Engine) {
    let level_width = 100.0;
    let level_height = 100.0;
    
    // Test different biomes
    let biomes = vec!["forest", "desert", "arctic", "cave", "lava"];
    
    for biome in biomes {
        match engine.level_generator_mut().create_atmospheric_effects(biome, level_width, level_height) {
            Ok(_) => {
                log::info!("Successfully created atmospheric effects for biome: {}", biome);
                
                // Get atmospheric effects
                let atmospheric_effects = engine.level_generator_mut().get_atmospheric_effects();
                log::info!("  Atmospheric effects count: {}", atmospheric_effects.len());
                
                // Count effects by type
                let mut effect_counts = std::collections::HashMap::new();
                for effect in atmospheric_effects {
                    *effect_counts.entry(effect.effect_type).or_insert(0) += 1;
                }
                
                log::info!("  Atmospheric effects by type:");
                for (effect_type, count) in effect_counts {
                    log::info!("    {:?}: {}", effect_type, count);
                }
                
                // Test effect properties
                for effect in atmospheric_effects {
                    log::info!("  Effect: {}", effect.id);
                    log::info!("    Type: {:?}", effect.effect_type);
                    log::info!("    Position: ({:.2}, {:.2})", effect.position.x, effect.position.y);
                    log::info!("    Intensity: {:.2}", effect.intensity);
                    log::info!("    Duration: {:.2}", effect.duration);
                    log::info!("    Age: {:.2}", effect.age);
                    log::info!("    Size: {:.2}", effect.size);
                    log::info!("    Color: [{:.2}, {:.2}, {:.2}, {:.2}]", 
                              effect.color[0], effect.color[1], effect.color[2], effect.color[3]);
                    log::info!("    Opacity: {:.2}", effect.opacity);
                    log::info!("    Active: {}", effect.active);
                    log::info!("    Moving: {}, Rotating: {}, Scaling: {}, Fading: {}, Pulsing: {}", 
                              effect.properties.moving, effect.properties.rotating, 
                              effect.properties.scaling, effect.properties.fading, effect.properties.pulsing);
                }
                
                // Test effect updates
                engine.level_generator_mut().update_atmospheric_effects(0.016); // ~60 FPS
                log::info!("  Updated atmospheric effects");
                
                // Get active effects
                let active_effects = engine.level_generator_mut().get_active_atmospheric_effects();
                log::info!("  Active effects count: {}", active_effects.len());
                
                // Test global lighting
                let global_lighting = engine.level_generator_mut().get_global_lighting();
                log::info!("  Global lighting:");
                log::info!("    Ambient color: [{:.2}, {:.2}, {:.2}, {:.2}]", 
                          global_lighting.ambient_color[0], global_lighting.ambient_color[1], 
                          global_lighting.ambient_color[2], global_lighting.ambient_color[3]);
                log::info!("    Ambient intensity: {:.2}", global_lighting.ambient_intensity);
                log::info!("    Directional color: [{:.2}, {:.2}, {:.2}, {:.2}]", 
                          global_lighting.directional_color[0], global_lighting.directional_color[1], 
                          global_lighting.directional_color[2], global_lighting.directional_color[3]);
                log::info!("    Directional intensity: {:.2}", global_lighting.directional_intensity);
                log::info!("    Fog color: [{:.2}, {:.2}, {:.2}, {:.2}]", 
                          global_lighting.fog_color[0], global_lighting.fog_color[1], 
                          global_lighting.fog_color[2], global_lighting.fog_color[3]);
                log::info!("    Fog density: {:.2}", global_lighting.fog_density);
                
                // Test weather system
                let weather_system = engine.level_generator_mut().get_weather_system();
                log::info!("  Weather system:");
                log::info!("    Current weather: {:?}", weather_system.current_weather);
                log::info!("    Weather intensity: {:.2}", weather_system.weather_intensity);
                log::info!("    Weather duration: {:.2}", weather_system.weather_duration);
                log::info!("    Weather age: {:.2}", weather_system.weather_age);
                log::info!("    Weather probability: {:.2}", weather_system.weather_probability);
            },
            Err(e) => {
                log::error!("Failed to create atmospheric effects for biome {}: {}", biome, e);
            }
        }
    }
}

/// Test level types generation
fn test_level_types_generation(engine: &mut Engine) {
    log::info!("=== Testing Level Types Generation ===");
    
    let base_config = LevelConfig {
        room_count_range: (3, 6),
        room_size_range: (8, 12),
        level_width: 40,
        level_height: 30,
        difficulty: 2,
        biome_type: "forest".to_string(),
        seed: Some(12345),
    };

    // Test Combat Arena
    match engine.level_generator_mut().generate_combat_arena(base_config.clone(), 8, 12) {
        Ok(level) => {
            log::info!("✅ Combat Arena generated successfully: {}", level.id);
            log::info!("  Dimensions: {}x{}", level.width, level.height);
            log::info!("  Rooms: {}", level.rooms.len());
            log::info!("  Spawn points: {}", level.spawn_points.len());
            log::info!("  Biome: {}", level.biome);
            log::info!("  Difficulty: {}", level.difficulty);
        },
        Err(e) => {
            log::error!("❌ Failed to generate combat arena: {}", e);
        }
    }

    // Test Platforming Level
    match engine.level_generator_mut().generate_platforming_level(base_config.clone(), 6, 10, 8) {
        Ok(level) => {
            log::info!("✅ Platforming Level generated successfully: {}", level.id);
            log::info!("  Dimensions: {}x{}", level.width, level.height);
            log::info!("  Rooms: {}", level.rooms.len());
            log::info!("  Spawn points: {}", level.spawn_points.len());
            log::info!("  Biome: {}", level.biome);
            log::info!("  Difficulty: {}", level.difficulty);
        },
        Err(e) => {
            log::error!("❌ Failed to generate platforming level: {}", e);
        }
    }

    // Test Puzzle Level
    match engine.level_generator_mut().generate_puzzle_level(base_config.clone(), 4, 6, 3, 3) {
        Ok(level) => {
            log::info!("✅ Puzzle Level generated successfully: {}", level.id);
            log::info!("  Dimensions: {}x{}", level.width, level.height);
            log::info!("  Rooms: {}", level.rooms.len());
            log::info!("  Spawn points: {}", level.spawn_points.len());
            log::info!("  Biome: {}", level.biome);
            log::info!("  Difficulty: {}", level.difficulty);
        },
        Err(e) => {
            log::error!("❌ Failed to generate puzzle level: {}", e);
        }
    }

    // Test Boss Arena
    match engine.level_generator_mut().generate_boss_arena(base_config.clone(), "dragon".to_string(), (25, 18), 3) {
        Ok(level) => {
            log::info!("✅ Boss Arena generated successfully: {}", level.id);
            log::info!("  Dimensions: {}x{}", level.width, level.height);
            log::info!("  Rooms: {}", level.rooms.len());
            log::info!("  Spawn points: {}", level.spawn_points.len());
            log::info!("  Biome: {}", level.biome);
            log::info!("  Difficulty: {}", level.difficulty);
        },
        Err(e) => {
            log::error!("❌ Failed to generate boss arena: {}", e);
        }
    }

    // Test Standard Level
    match engine.level_generator_mut().generate_standard_level(base_config.clone(), 0.6, 0.4) {
        Ok(level) => {
            log::info!("✅ Standard Level generated successfully: {}", level.id);
            log::info!("  Dimensions: {}x{}", level.width, level.height);
            log::info!("  Rooms: {}", level.rooms.len());
            log::info!("  Spawn points: {}", level.spawn_points.len());
            log::info!("  Biome: {}", level.biome);
            log::info!("  Difficulty: {}", level.difficulty);
        },
        Err(e) => {
            log::error!("❌ Failed to generate standard level: {}", e);
        }
    }

    // Test available level types
    let available_types = engine.level_generator_mut().get_available_level_types();
    log::info!("Available level types: {:?}", available_types);

    // Test level type templates
    for level_type in &available_types {
        if let Some(template) = engine.level_generator_mut().get_level_type_template(level_type) {
            log::info!("Template for {:?}:", level_type);
            log::info!("  Room patterns: {}", template.room_patterns.len());
            log::info!("  Tile rules: {}", template.tile_rules.len());
            log::info!("  Object rules: {}", template.object_rules.len());
            log::info!("  Spawn rules: {}", template.spawn_rules.len());
        }
    }
}

/// Test the level progression system
fn test_level_progression_system(engine: &mut Engine) {
    info!("Testing level progression system...");

    // Test level selection and display
    {
        let level_progression = engine.level_generator_mut().get_level_progression_manager_mut();
        let all_levels = level_progression.get_all_levels();
        info!("Available levels: {}", all_levels.len());
        
        for (level_id, level_info) in all_levels {
            info!("Level: {} - {} (Type: {:?}, Difficulty: {}, Unlocked: {})", 
                  level_id, level_info.name, level_info.level_type, level_info.difficulty, level_info.unlocked);
        }
    }

    // Test level filtering
    {
        let mut filters = LevelFilters::default();
        filters.difficulty_range = Some((1, 3));
        filters.level_types = vec![LevelType::CombatArena, LevelType::Standard];
        
        engine.level_generator_mut().set_level_filters(filters);
        let filtered_levels = engine.level_generator_mut().get_filtered_levels();
        info!("Filtered levels (difficulty 1-3, combat/standard): {}", filtered_levels.len());
    }

    // Test level selection
    {
        let level_id = {
            let level_progression = engine.level_generator_mut().get_level_progression_manager_mut();
            let all_levels = level_progression.get_all_levels();
            all_levels.keys().next().map(|s| s.clone())
        };
        
        if let Some(level_id) = level_id {
            match engine.level_generator_mut().select_level(&level_id) {
                Ok(_) => info!("Successfully selected level: {}", level_id),
                Err(e) => info!("Failed to select level {}: {}", level_id, e),
            }
        }
    }

    // Test player progress
    {
        let player_progress = engine.level_generator_mut().get_player_progress();
        info!("Player level: {}, Experience: {}/{}", 
              player_progress.level, player_progress.experience, player_progress.experience_to_next);

        // Test adding experience
        engine.level_generator_mut().add_experience(150);
        let updated_progress = engine.level_generator_mut().get_player_progress();
        info!("After adding 150 XP - Level: {}, Experience: {}/{}", 
              updated_progress.level, updated_progress.experience, updated_progress.experience_to_next);
    }

    // Test checkpoint system
    {
        let checkpoint = Checkpoint {
            id: "test_checkpoint_1".to_string(),
            position: Vec2::new(25.0, 25.0),
            checkpoint_type: CheckpointType::Standard,
            active: true,
            reached: false,
            reached_time: None,
            player_state: None,
        };

        engine.level_generator_mut().create_checkpoint(checkpoint);

        let player_state = PlayerState {
            position: Vec2::new(25.0, 25.0),
            health: 100.0,
            experience: 200,
            level: 2,
            inventory: vec!["sword".to_string(), "potion".to_string()],
            abilities: vec!["dash".to_string()],
        };

        match engine.level_generator_mut().reach_checkpoint("test_checkpoint_1", player_state) {
            Ok(_) => info!("Successfully reached checkpoint: test_checkpoint_1"),
            Err(e) => info!("Failed to reach checkpoint: {}", e),
        }
    }

    // Test level completion and rewards
    {
        let level_id = {
            let level_progression = engine.level_generator_mut().get_level_progression_manager_mut();
            let all_levels = level_progression.get_all_levels();
            all_levels.keys().next().map(|s| s.clone())
        };
        
        if let Some(level_id) = level_id {
            match engine.level_generator_mut().complete_level(&level_id, 120.5, 3) {
                Ok(rewards) => {
                    info!("Level {} completed! Received {} rewards:", level_id, rewards.len());
                    for reward in rewards {
                        info!("  - {:?}: {} ({})", reward.reward_type, reward.value, reward.description);
                    }
                },
                Err(e) => info!("Failed to complete level {}: {}", level_id, e),
            }
        }
    }

    // Test level categories
    {
        let categories = engine.level_generator_mut().get_level_progression_manager().get_level_categories();
        info!("Level categories: {}", categories.len());
        for category in categories {
            info!("Category: {} - {} ({} levels)", category.name, category.description, category.levels.len());
        }
    }

    info!("Level progression system test completed successfully");
}

/// Test the advanced combat and abilities systems
fn test_advanced_combat_system(engine: &mut Engine) {
    info!("Testing Advanced Combat and Abilities System...");

    // Test combo system
    {
        use game::combat::*;
        
        let mut combo = Combo::default();
        let mut combo_processor = ComboInputProcessor::new();
        let current_time = 0.0;

        // Test basic combo
        let result = combo_processor.process_input(&mut combo, ComboInput::LightAttack, current_time);
        info!("Combo input result: {:?}", result);

        let result = combo_processor.process_input(&mut combo, ComboInput::LightAttack, current_time + 0.1);
        info!("Second combo input result: {:?}", result);

        let result = combo_processor.process_input(&mut combo, ComboInput::HeavyAttack, current_time + 0.2);
        info!("Third combo input result: {:?}", result);
    }

    // Test special moves system
    {
        use game::combat::*;
        
        let mut special_move = SpecialMove::new(
            "fireball".to_string(),
            "Fireball".to_string(),
            2.0,
            20.0,
            ResourceType::Mana,
        );
        
        let mut resource_manager = ResourceManager::new();
        resource_manager.resources.insert(ResourceType::Mana, 100.0);
        
        info!("Special move '{}' can execute: {}", special_move.name, special_move.can_execute(&resource_manager.resources));
        
        if special_move.can_execute(&resource_manager.resources) {
            special_move.start_execution();
            resource_manager.consume_resource(ResourceType::Mana, special_move.resource_cost);
            info!("Special move '{}' executed successfully", special_move.name);
        }
    }

    // Test defense system
    {
        use game::combat::*;
        
        let mut defense = Defense::default();
        let mut defense_system = DefenseSystem::new();
        
        // Test blocking
        if defense_system.start_block(&mut defense, BlockType::Heavy) {
            info!("Heavy block activated successfully");
        }
        
        // Test parrying
        if defense_system.start_parry(&mut defense, ParryType::Perfect) {
            info!("Perfect parry activated successfully");
        }
        
        // Test dodging
        if defense_system.start_dodge(&mut defense, DodgeDirection::Backward) {
            info!("Backward dodge activated successfully");
        }
    }

    // Test character abilities system
    {
        use game::abilities::*;
        
        let character_creation = CharacterCreationSystem::new();
        let available_classes = character_creation.get_available_classes();
        
        info!("Available character classes: {}", available_classes.len());
        for class in &available_classes {
            info!("  - {}: {}", class.name(), class.description());
        }
        
        // Create a Fighter character
        let fighter = character_creation.create_character(CharacterClass::Fighter, "Test Fighter".to_string());
        info!("Created {} character: {} (Level {})", fighter.class.name(), fighter.name, fighter.level);
        info!("  Stats: STR={}, AGI={}, INT={}, VIT={}", 
               fighter.stats.strength, fighter.stats.agility, 
               fighter.stats.intelligence, fighter.stats.vitality);
        info!("  Health: {}, Mana: {}, Stamina: {}", 
               fighter.stats.calculate_health(), 
               fighter.stats.calculate_mana(), 
               fighter.stats.calculate_stamina());
        
        // Test ability tree
        if let Some(tree) = fighter.abilities.get_ability_tree("Fighter") {
            let unlocked_abilities = tree.get_unlocked_abilities();
            info!("Fighter has {} unlocked abilities:", unlocked_abilities.len());
            for ability in unlocked_abilities {
                info!("  - {} (Level {})", ability.name, ability.level);
            }
            
            let available_abilities = tree.get_available_abilities();
            info!("Fighter has {} available abilities to unlock:", available_abilities.len());
            for ability in available_abilities {
                info!("  - {} (Cost: {})", ability.name, ability.cost);
            }
        }
    }

    // Test ability effects
    {
        use game::abilities::*;
        
        let mut effect_system = AbilityEffectSystem::new();
        let target_entity = 1;
        
        // Test applying effects
        let effect = AbilityEffect::DamageBonus { multiplier: 0.5 };
        let effect_id = effect_system.processor.apply_effect(effect, target_entity, 10.0);
        info!("Applied damage bonus effect: {}", effect_id);
        
        let damage_bonus = effect_system.processor.calculate_damage_bonus(target_entity);
        info!("Damage bonus for entity {}: {:.2}x", target_entity, damage_bonus);
        
        let defense_bonus = effect_system.processor.calculate_defense_bonus(target_entity);
        info!("Defense bonus for entity {}: {:.2}x", target_entity, defense_bonus);
        
        let speed_bonus = effect_system.processor.calculate_speed_bonus(target_entity);
        info!("Speed bonus for entity {}: {:.2}x", target_entity, speed_bonus);
    }

    info!("Advanced combat and abilities system test completed successfully");
}

/// Test the character system
fn test_character_system(engine: &mut Engine) {
    info!("Testing Character System...");
    
    use game::characters::*;
    use game::items::ItemManager;
    
    // Test character roster
    let mut roster = CharacterRoster::new();
    info!("Character roster created with {} templates", roster.get_template_count());
    
    // Create item manager for character equipment
    let item_manager = ItemManager::new();
    
    // Test character creation from template
    let fighter_template = roster.get_template("fighter_template").unwrap();
    let mut fighter = roster.create_character_from_template("fighter_template", "Test Fighter".to_string(), &item_manager).unwrap();
    info!("Created Fighter character: {} (Level {})", fighter.name, fighter.level);
    info!("  Stats: STR={}, AGI={}, INT={}, VIT={}", 
        fighter.stats.strength, fighter.stats.agility, fighter.stats.intelligence, fighter.stats.vitality);
    info!("  Health: {}, Mana: {}, Stamina: {}", 
        fighter.status.max_health, fighter.status.max_mana, fighter.status.max_stamina);
    
    // Test character leveling
    fighter.add_experience(150);
    if fighter.can_level_up() {
        fighter.level_up();
        info!("Fighter leveled up to level {}!", fighter.level);
        info!("  New Stats: STR={}, AGI={}, INT={}, VIT={}", 
            fighter.stats.strength, fighter.stats.agility, fighter.stats.intelligence, fighter.stats.vitality);
    }
    
    // Test character customization
    let mut customization = CharacterCustomization::new();
    let mut appearance = fighter.appearance.clone();
    appearance.hair_color = [0.8, 0.6, 0.2]; // Blonde hair
    appearance.eye_color = [0.2, 0.8, 0.2]; // Green eyes
    customization.customize_appearance(&mut fighter, appearance);
    info!("Customized fighter appearance");
    
    // Test character progression
    let mut progression = CharacterProgression::new();
    progression.add_character(fighter.id.clone(), &fighter);
    let events = progression.add_experience(&fighter.id, 500).unwrap();
    info!("Added 500 XP to fighter, generated {} progression events", events.len());
    
    // Test character selection
    let mut selection = CharacterSelection::new();
    selection.add_character(fighter);
    
    // Create additional characters for testing
    let mage = roster.create_character_from_template("mage_template", "Test Mage".to_string(), &item_manager).unwrap();
    let ranger = roster.create_character_from_template("ranger_template", "Test Ranger".to_string(), &item_manager).unwrap();
    
    selection.add_character(mage);
    selection.add_character(ranger);
    
    info!("Character selection has {} characters", selection.get_character_count());
    
    // Test character filtering and sorting
    let filtered_characters = selection.get_filtered_characters();
    info!("Filtered characters: {}", filtered_characters.len());
    
    // Test character comparison
    let character_ids: Vec<String> = selection.characters.keys().cloned().collect();
    if character_ids.len() >= 2 {
        selection.start_character_comparison(character_ids[0..2].to_vec()).unwrap();
        let comparison_stats = selection.get_comparison_stats();
        info!("Character comparison stats: {}", comparison_stats.len());
        for (stat_name, stat) in comparison_stats {
            info!("  {}: {} values", stat.name, stat.values.len());
        }
        selection.stop_character_comparison();
    }
    
    // Test character statistics
    let stats = selection.get_character_statistics();
    info!("Character statistics:");
    info!("  Total characters: {}", stats.total_characters);
    info!("  Average level: {:.1}", stats.average_level);
    info!("  Total experience: {}", stats.total_experience);
    info!("  Characters by class: {:?}", stats.characters_by_class);
    
    // Test character damage and abilities
    {
        let fighter = selection.get_character(&character_ids[0]).unwrap();
        info!("Fighter combat stats:");
        info!("  Total damage: {:.1}", fighter.get_total_damage());
        info!("  Total defense: {:.1}", fighter.get_total_defense());
        info!("  Movement speed: {:.1}", fighter.get_movement_speed());
        info!("  Health percentage: {:.1}%", fighter.get_health_percentage() * 100.0);
        info!("  Mana percentage: {:.1}%", fighter.get_mana_percentage() * 100.0);
        info!("  Stamina percentage: {:.1}%", fighter.get_stamina_percentage() * 100.0);
    }
    
    // Test character status effects
    let mut fighter_mut = selection.get_character_mut(&character_ids[0]).unwrap();
    fighter_mut.add_status_effect(StatusEffect {
        id: "test_effect".to_string(),
        effect_type: "poison".to_string(),
        duration: 10.0,
        intensity: 5.0,
        source: "test".to_string(),
    });
    info!("Added status effect to fighter");
    
    fighter_mut.update_status_effects(1.0);
    info!("Updated status effects, {} active effects", fighter_mut.status.status_effects.len());
    
    // Test character damage
    fighter_mut.take_damage(50.0, Some("test_enemy".to_string()));
    info!("Fighter took 50 damage, health: {:.1}/{:.1}", 
        fighter_mut.status.health, fighter_mut.status.max_health);
    
    fighter_mut.heal(25.0);
    info!("Fighter healed 25, health: {:.1}/{:.1}", 
        fighter_mut.status.health, fighter_mut.status.max_health);
    
    // Test equipment and inventory integration
    info!("Testing equipment and inventory integration...");
    
    // Test equipping items
    if let Ok(_) = fighter_mut.equip_item("iron_sword", &item_manager) {
        info!("Equipped Iron Sword to fighter");
        let total_stats = fighter_mut.get_total_stats(&item_manager);
        info!("Fighter total stats with equipment: STR={:.1}, AGI={:.1}, INT={:.1}, VIT={:.1}", 
            total_stats.strength, total_stats.agility, total_stats.intelligence, total_stats.vitality);
    }
    
    // Test inventory operations
    let inventory_result = fighter_mut.add_item_to_inventory("iron_sword", 1, &item_manager);
    info!("Added Iron Sword to inventory: {:?}", inventory_result);
    
    let inventory_stats = fighter_mut.get_inventory_stats();
    info!("Fighter inventory stats: Used slots: {}/{}, Weight: {:.1}/{:.1}", 
        inventory_stats.used_slots, inventory_stats.max_slots, 
        inventory_stats.total_weight, inventory_stats.max_weight);
    
    // Test total health calculation with equipment
    let total_health = fighter_mut.get_total_max_health(&item_manager);
    let total_mana = fighter_mut.get_total_max_mana(&item_manager);
    let total_stamina = fighter_mut.get_total_max_stamina(&item_manager);
    info!("Fighter total stats with equipment: Health: {:.1}, Mana: {:.1}, Stamina: {:.1}", 
        total_health, total_mana, total_stamina);
    
    info!("Character system test completed successfully");
}

/// Test the enemy system
fn test_enemy_system(engine: &mut Engine) {
    info!("Testing enemy system...");
    
    // Test basic enemy creation
    let goblin = game::enemies::Enemy::new(game::enemies::EnemyType::Goblin, 1);
    info!("Created Goblin: Health {:.1}/{:.1}, Level {}, Experience {}", 
        goblin.health, goblin.max_health, goblin.level, goblin.experience_value);
    
    let orc = game::enemies::Enemy::new(game::enemies::EnemyType::Orc, 2);
    info!("Created Orc: Health {:.1}/{:.1}, Level {}, Experience {}", 
        orc.health, orc.max_health, orc.level, orc.experience_value);
    
    let archer = game::enemies::Enemy::new(game::enemies::EnemyType::Archer, 3);
    info!("Created Archer: Health {:.1}/{:.1}, Level {}, Experience {}", 
        archer.health, archer.max_health, archer.level, archer.experience_value);
    
    // Test specialized enemies
    let mage = game::enemies::Enemy::new(game::enemies::EnemyType::Mage, 4);
    info!("Created Mage: Health {:.1}/{:.1}, Level {}, Experience {}", 
        mage.health, mage.max_health, mage.level, mage.experience_value);
    
    let berserker = game::enemies::Enemy::new(game::enemies::EnemyType::Berserker, 5);
    info!("Created Berserker: Health {:.1}/{:.1}, Level {}, Experience {}", 
        berserker.health, berserker.max_health, berserker.level, berserker.experience_value);
    
    let shield_bearer = game::enemies::Enemy::new(game::enemies::EnemyType::ShieldBearer, 6);
    info!("Created Shield Bearer: Health {:.1}/{:.1}, Level {}, Experience {}", 
        shield_bearer.health, shield_bearer.max_health, shield_bearer.level, shield_bearer.experience_value);
    
    // Test flying enemies
    let bat = game::enemies::Enemy::new(game::enemies::EnemyType::Bat, 1);
    info!("Created Bat: Health {:.1}/{:.1}, Can Fly: {}, Level {}", 
        bat.health, bat.max_health, bat.can_fly, bat.level);
    
    let demon = game::enemies::Enemy::new(game::enemies::EnemyType::Demon, 7);
    info!("Created Demon: Health {:.1}/{:.1}, Can Fly: {}, Level {}", 
        demon.health, demon.max_health, demon.can_fly, demon.level);
    
    let dragon = game::enemies::Enemy::new(game::enemies::EnemyType::Dragon, 8);
    info!("Created Dragon: Health {:.1}/{:.1}, Can Fly: {}, Level {}", 
        dragon.health, dragon.max_health, dragon.can_fly, dragon.level);
    
    // Test boss enemies
    let goblin_king = game::enemies::Enemy::new(game::enemies::EnemyType::GoblinKing, 10);
    info!("Created Goblin King: Health {:.1}/{:.1}, Level {}, Experience {}", 
        goblin_king.health, goblin_king.max_health, goblin_king.level, goblin_king.experience_value);
    
    let orc_warlord = game::enemies::Enemy::new(game::enemies::EnemyType::OrcWarlord, 12);
    info!("Created Orc Warlord: Health {:.1}/{:.1}, Level {}, Experience {}", 
        orc_warlord.health, orc_warlord.max_health, orc_warlord.level, orc_warlord.experience_value);
    
    let dark_mage = game::enemies::Enemy::new(game::enemies::EnemyType::DarkMage, 15);
    info!("Created Dark Mage: Health {:.1}/{:.1}, Level {}, Experience {}", 
        dark_mage.health, dark_mage.max_health, dark_mage.level, dark_mage.experience_value);
    
    let dragon_lord = game::enemies::Enemy::new(game::enemies::EnemyType::DragonLord, 20);
    info!("Created Dragon Lord: Health {:.1}/{:.1}, Can Fly: {}, Level {}", 
        dragon_lord.health, dragon_lord.max_health, dragon_lord.can_fly, dragon_lord.level);
    
    // Test enemy manager
    let mut enemy_manager = game::enemies::EnemyManager::new();
    enemy_manager.set_difficulty(1.5);
    info!("Created enemy manager with difficulty: {}", enemy_manager.difficulty);
    
    // Test enemy spawning
    let spawn_position = Vec2::new(100.0, 100.0);
    if let Some(entity) = enemy_manager.spawn_enemy(game::enemies::EnemyType::Goblin, spawn_position, 1) {
        info!("Spawned Goblin at {:?} with entity ID: {}", spawn_position, entity);
    }
    
    if let Some(entity) = enemy_manager.spawn_enemy(game::enemies::EnemyType::Orc, Vec2::new(200.0, 100.0), 2) {
        info!("Spawned Orc at {:?} with entity ID: {}", Vec2::new(200.0, 100.0), entity);
    }
    
    if let Some(entity) = enemy_manager.spawn_enemy(game::enemies::EnemyType::GoblinKing, Vec2::new(300.0, 100.0), 10) {
        info!("Spawned Goblin King at {:?} with entity ID: {}", Vec2::new(300.0, 100.0), entity);
    }
    
    info!("Enemy manager has {} enemies spawned", enemy_manager.enemy_count());
    info!("Enemy manager has {} alive enemies", enemy_manager.alive_enemy_count());
    
    // Test AI system
    let mut ai_system = game::enemies::AISystem::new();
    ai_system.global_settings.global_difficulty = 1.2;
    info!("Created AI system with global difficulty: {}", ai_system.global_settings.global_difficulty);
    
    // Test damage system
    let mut test_goblin = goblin.clone();
    info!("Goblin initial health: {:.1}/{:.1}", test_goblin.health, test_goblin.max_health);
    
    let damage = 15.0;
    let is_alive = test_goblin.take_damage(damage);
    info!("Goblin took {} damage, health: {:.1}/{:.1}, alive: {}", 
        damage, test_goblin.health, test_goblin.max_health, is_alive);
    
    test_goblin.heal(10.0);
    info!("Goblin healed 10, health: {:.1}/{:.1}", test_goblin.health, test_goblin.max_health);
    
    // Test AI state transitions
    let player_position = Vec2::new(150.0, 150.0);
    let enemy_position = Vec2::new(100.0, 100.0);
    
    test_goblin.update_ai_state(Some(player_position), enemy_position);
    info!("Goblin AI state: {:?}", test_goblin.ai_state);
    
    // Test detection and attack ranges
    let can_see = test_goblin.can_see_target(player_position, enemy_position);
    let can_attack = test_goblin.can_attack_target(player_position, enemy_position);
    info!("Goblin can see player: {}, can attack player: {}", can_see, can_attack);
    
    info!("Enemy system test completed successfully");
}

/// Test the items and equipment system
fn test_items_system(engine: &mut Engine) {
    info!("=== Testing Items & Equipment System ===");
    
    // Test item manager
    let mut item_manager = game::items::ItemManager::new();
    item_manager.initialize_default_templates();
    info!("Created item manager with {} templates", item_manager.item_templates.len());
    
    // Test item creation
    if let Some(iron_sword) = item_manager.create_item("basic_sword", 5) {
        info!("Created Iron Sword: Level {}, Value {}, Attack Damage +{}", 
            iron_sword.level_requirement, iron_sword.value, iron_sword.stats.attack_damage_bonus);
    }
    
    if let Some(magic_staff) = item_manager.create_item("magic_staff", 8) {
        info!("Created Magic Staff: Level {}, Value {}, Magic Damage +{}, Mana +{}", 
            magic_staff.level_requirement, magic_staff.value, 
            magic_staff.stats.magic_damage_bonus, magic_staff.stats.mana_bonus);
    }
    
    if let Some(health_potion) = item_manager.create_item("health_potion", 1) {
        info!("Created Health Potion: Value {}, Effects: {}", 
            health_potion.value, health_potion.effects.len());
    }
    
    // Test equipment system
    let mut equipment = game::items::Equipment::new();
    let mut equipment_manager = game::items::EquipmentManager::new();
    equipment_manager.initialize_default_sets();
    info!("Created equipment system with {} equipment sets", equipment_manager.equipment_sets.len());
    
    // Test equipping items
    if let Some(iron_sword) = item_manager.get_item("basic_sword_5") {
        if equipment_manager.can_equip_item(iron_sword, &game::items::EquipmentSlot::MainHand) {
            let old_item = equipment.equip_item(game::items::EquipmentSlot::MainHand, iron_sword.id.clone());
            info!("Equipped Iron Sword, old item: {:?}", old_item);
        }
    }
    
    if let Some(magic_staff) = item_manager.get_item("magic_staff_8") {
        if equipment_manager.can_equip_item(magic_staff, &game::items::EquipmentSlot::OffHand) {
            let old_item = equipment.equip_item(game::items::EquipmentSlot::OffHand, magic_staff.id.clone());
            info!("Equipped Magic Staff, old item: {:?}", old_item);
        }
    }
    
    // Update equipment stats
    equipment.update_equipment(&item_manager.items);
    info!("Equipment total stats - Attack Damage: +{}, Magic Damage: +{}, Mana: +{}", 
        equipment.total_stats.attack_damage_bonus,
        equipment.total_stats.magic_damage_bonus,
        equipment.total_stats.mana_bonus);
    
    // Test consumable system
    let mut consumable_manager = game::items::ConsumableManager::new();
    consumable_manager.initialize_default_templates();
    info!("Created consumable manager with {} templates", consumable_manager.consumable_templates.len());
    
    // Test consumable creation
    if let Some(template) = consumable_manager.consumable_templates.get("health_potion") {
        let health_potion = game::items::Consumable::from_template(template);
        info!("Created Health Potion consumable: Cooldown {:.1}s, Stackable: {}", 
            health_potion.cooldown, health_potion.stackable);
    }
    
    if let Some(template) = consumable_manager.consumable_templates.get("strength_potion") {
        let strength_potion = game::items::Consumable::from_template(template);
        info!("Created Strength Potion: Duration {:.1}s, Cooldown {:.1}s", 
            strength_potion.duration.unwrap_or(0.0), strength_potion.cooldown);
    }
    
    // Test inventory system
    let mut inventory = game::items::Inventory::new(50, 1000.0);
    let mut inventory_manager = game::items::InventoryManager::new();
    inventory_manager.update_item_database(item_manager.items.clone());
    
    // Test adding items to inventory
    let add_result = inventory.add_item("basic_sword_5".to_string(), 1, &inventory_manager.item_database);
    info!("Added Iron Sword to inventory: {:?}", add_result);
    
    let add_result = inventory.add_item("health_potion".to_string(), 5, &inventory_manager.item_database);
    info!("Added 5 Health Potions to inventory: {:?}", add_result);
    
    let add_result = inventory.add_item("mana_potion".to_string(), 3, &inventory_manager.item_database);
    info!("Added 3 Mana Potions to inventory: {:?}", add_result);
    
    // Test inventory statistics
    info!("Inventory stats - Used slots: {}/{}, Weight: {:.1}/{:.1}", 
        inventory.used_slots, inventory.max_slots, inventory.total_weight, inventory.max_weight);
    
    // Test inventory filtering
    inventory.filters.item_type_filter = Some(game::items::ItemType::Consumable(game::items::ConsumableType::Potion));
    let filtered_items = inventory.get_filtered_items(&inventory_manager.item_database);
    info!("Filtered items (potions only): {} items", filtered_items.len());
    
    // Test inventory sorting
    inventory.sort_order = game::items::SortOrder::Value;
    let sorted_items = inventory.get_filtered_items(&inventory_manager.item_database);
    info!("Sorted items by value: {} items", sorted_items.len());
    
    // Test item rarity system
    let common_items = item_manager.get_items_by_rarity(&game::items::ItemRarity::Common);
    let uncommon_items = item_manager.get_items_by_rarity(&game::items::ItemRarity::Uncommon);
    let rare_items = item_manager.get_items_by_rarity(&game::items::ItemRarity::Rare);
    
    info!("Items by rarity - Common: {}, Uncommon: {}, Rare: {}", 
        common_items.len(), uncommon_items.len(), rare_items.len());
    
    // Test item effects
    if let Some(iron_sword) = item_manager.get_item("basic_sword_5") {
        info!("Iron Sword effects: {} effects", iron_sword.effects.len());
        for effect in &iron_sword.effects {
            info!("  Effect: {:?} - {}", effect.effect_type, effect.description);
        }
    }
    
    // Test equipment set bonuses
    let equipped_items: Vec<&String> = equipment.get_all_equipped_items();
    let set_bonuses = equipment_manager.calculate_set_bonuses(&equipped_items, &item_manager.items);
    info!("Active set bonuses: {} bonuses", set_bonuses.len());
    
    for bonus in &set_bonuses {
        info!("  Set Bonus: {} - {} pieces equipped", bonus.set_name, bonus.pieces_equipped);
    }
    
    // Test consumable usage simulation
    let test_entity = 1; // Mock entity ID
    if let Some(template) = consumable_manager.consumable_templates.get("health_potion") {
        let health_potion = game::items::Consumable::from_template(template);
        
        // Simulate using the potion
        match consumable_manager.use_consumable(test_entity, &health_potion) {
            Ok(_) => info!("Successfully used Health Potion on entity {}", test_entity),
            Err(e) => info!("Failed to use Health Potion: {}", e),
        }
        
        // Check cooldown
        let cooldown = consumable_manager.get_remaining_cooldown(test_entity, "health_potion");
        info!("Health Potion cooldown remaining: {:.1}s", cooldown);
    }
    
    // Test inventory operations
    let move_result = inventory.move_item(0, 1, &inventory_manager.item_database);
    info!("Moved item from slot 0 to slot 1: {:?}", move_result);
    
    let split_result = inventory.split_item(1, 2, &inventory_manager.item_database);
    info!("Split item in slot 1 (quantity 2): {:?}", split_result);
    
    // Test inventory statistics
    if let Some(stats) = inventory_manager.get_inventory_stats(test_entity) {
        info!("Inventory statistics - Total items: {}, Total value: {}, Weight: {:.1}/{:.1}", 
            stats.total_items, stats.total_value, stats.total_weight, stats.max_weight);
    }
    
    info!("Items & Equipment system test completed successfully");
}

/// Test the localization system
fn test_localization_system(engine: &mut Engine) {
    use engine::localization::{Language, StringId, string_id};
    use std::collections::HashMap;
    
    info!("Testing Localization System...");
    
    // Test basic localization functionality
    let localization = engine.localization();
    
    // Test string ID creation
    let main_menu_id = string_id!("ui.main_menu");
    let play_id = string_id!("ui.play");
    let settings_id = string_id!("ui.settings");
    
    info!("Created string IDs: {}, {}, {}", 
        main_menu_id.as_str(), play_id.as_str(), settings_id.as_str());
    
    // Test fallback text (should work even without translation files)
    let main_menu_text = localization.get_string(&main_menu_id);
    let play_text = localization.get_string(&play_id);
    let settings_text = localization.get_string(&settings_id);
    
    info!("Fallback texts - Main Menu: '{}', Play: '{}', Settings: '{}'", 
        main_menu_text, play_text, settings_text);
    
    // Test language switching
    let mut localization_mut = engine.localization_mut();
    
    // Test available languages
    let available_languages = localization_mut.available_languages();
    info!("Available languages: {:?}", available_languages);
    
    // Test language switching
    if let Err(e) = localization_mut.set_language(Language::French) {
        info!("Failed to set French language: {}", e);
    } else {
        info!("Switched to French language");
        let french_main_menu = localization_mut.get_string(&main_menu_id);
        info!("French Main Menu: '{}'", french_main_menu);
    }
    
    // Test Korean
    if let Err(e) = localization_mut.set_language(Language::Korean) {
        info!("Failed to set Korean language: {}", e);
    } else {
        info!("Switched to Korean language");
        let korean_main_menu = localization_mut.get_string(&main_menu_id);
        info!("Korean Main Menu: '{}'", korean_main_menu);
    }
    
    // Test Japanese
    if let Err(e) = localization_mut.set_language(Language::Japanese) {
        info!("Failed to set Japanese language: {}", e);
    } else {
        info!("Switched to Japanese language");
        let japanese_main_menu = localization_mut.get_string(&main_menu_id);
        info!("Japanese Main Menu: '{}'", japanese_main_menu);
    }
    
    // Test Chinese Simplified
    if let Err(e) = localization_mut.set_language(Language::ChineseSimplified) {
        info!("Failed to set Chinese Simplified language: {}", e);
    } else {
        info!("Switched to Chinese Simplified language");
        let chinese_main_menu = localization_mut.get_string(&main_menu_id);
        info!("Chinese Simplified Main Menu: '{}'", chinese_main_menu);
    }
    
    // Test Chinese Traditional
    if let Err(e) = localization_mut.set_language(Language::ChineseTraditional) {
        info!("Failed to set Chinese Traditional language: {}", e);
    } else {
        info!("Switched to Chinese Traditional language");
        let chinese_main_menu = localization_mut.get_string(&main_menu_id);
        info!("Chinese Traditional Main Menu: '{}'", chinese_main_menu);
    }
    
    // Test Turkish
    if let Err(e) = localization_mut.set_language(Language::Turkish) {
        info!("Failed to set Turkish language: {}", e);
    } else {
        info!("Switched to Turkish language");
        let turkish_main_menu = localization_mut.get_string(&main_menu_id);
        info!("Turkish Main Menu: '{}'", turkish_main_menu);
    }
    
    // Test Arabic (RTL language)
    if let Err(e) = localization_mut.set_language(Language::Arabic) {
        info!("Failed to set Arabic language: {}", e);
    } else {
        info!("Switched to Arabic language");
        let arabic_main_menu = localization_mut.get_string(&main_menu_id);
        let is_rtl = localization_mut.is_rtl();
        info!("Arabic Main Menu: '{}' (RTL: {})", arabic_main_menu, is_rtl);
    }
    
    // Test string interpolation
    let mut variables = HashMap::new();
    variables.insert("player_name".to_string(), "TestPlayer".to_string());
    variables.insert("score".to_string(), "1000".to_string());
    
    let welcome_id = string_id!("ui.welcome");
    let welcome_text = localization_mut.get_string_with_vars(&welcome_id, &variables);
    info!("Welcome text with variables: '{}'", welcome_text);
    
    // Test pluralization
    let item_count = 5;
    let items_id = string_id!("gameplay.items");
    let items_text = localization_mut.get_plural_string(&items_id, item_count);
    info!("Plural text for {} items: '{}'", item_count, items_text);
    
    // Test font information
    let font_info = localization_mut.get_font_info();
    info!("Font info - Primary: {}, Size multiplier: {:.1}, Supports language: {}", 
        font_info.primary_font, font_info.size_multiplier, font_info.supports_language);
    
    // Test text direction
    let text_direction = localization_mut.get_text_direction();
    info!("Text direction: {:?}", text_direction);
    
    // Test translation coverage
    let coverage = localization_mut.get_all_translation_coverage();
    for (language, coverage_percent) in coverage {
        info!("Translation coverage for {:?}: {:.1}%", language, coverage_percent * 100.0);
    }
    
    // Test string ID registration
    let test_id = string_id!("test.new_string");
    localization_mut.register_string_id(
        test_id.clone(), 
        engine::localization::string_id::StringCategory::UI, 
        Some("Test string for localization".to_string())
    );
    
    let stats = localization_mut.get_string_id_stats();
    info!("String ID registry stats - Total IDs: {}", stats.total_ids);
    
    // Test helper functions
    use engine::localization::manager::helpers;
    let ui_text = helpers::ui(localization_mut, "menu", "play");
    let gameplay_text = helpers::gameplay(localization_mut, "level", "complete");
    let combat_text = helpers::combat(localization_mut, "attack", "heavy");
    
    info!("Helper functions - UI: '{}', Gameplay: '{}', Combat: '{}'", 
        ui_text, gameplay_text, combat_text);
    
    // Switch back to English
    if let Err(e) = localization_mut.set_language(Language::English) {
        info!("Failed to switch back to English: {}", e);
    } else {
        info!("Switched back to English language");
    }
    
    info!("Localization system test completed successfully");
}

/// Test the comprehensive string registry
fn test_comprehensive_string_registry(engine: &mut Engine) {
    println!("\n=== Testing Comprehensive String Registry ===");
    
    use crate::engine::localization::string_registry::GameStringRegistry;
    use crate::engine::localization::StringId;
    use crate::engine::localization::string_id::StringCategory;
    
    // Create the comprehensive string registry
    let registry = GameStringRegistry::new();
    
    // Test getting all strings
    let all_strings = registry.get_all_strings();
    println!("Total registered strings: {}", all_strings.len());
    
    // Test getting strings by category
    let ui_strings = registry.get_strings_by_category(&StringCategory::UI);
    println!("UI strings: {}", ui_strings.len());
    
    let gameplay_strings = registry.get_strings_by_category(&StringCategory::Gameplay);
    println!("Gameplay strings: {}", gameplay_strings.len());
    
    let character_strings = registry.get_strings_by_category(&StringCategory::Characters);
    println!("Character strings: {}", character_strings.len());
    
    let item_strings = registry.get_strings_by_category(&StringCategory::Items);
    println!("Item strings: {}", item_strings.len());
    
    let combat_strings = registry.get_strings_by_category(&StringCategory::Combat);
    println!("Combat strings: {}", combat_strings.len());
    
    let enemy_strings = registry.get_strings_by_category(&StringCategory::Enemies);
    println!("Enemy strings: {}", enemy_strings.len());
    
    let level_strings = registry.get_strings_by_category(&StringCategory::Levels);
    println!("Level strings: {}", level_strings.len());
    
    let tutorial_strings = registry.get_strings_by_category(&StringCategory::Tutorial);
    println!("Tutorial strings: {}", tutorial_strings.len());
    
    let error_strings = registry.get_strings_by_category(&StringCategory::Errors);
    println!("Error strings: {}", error_strings.len());
    
    let achievement_strings = registry.get_strings_by_category(&StringCategory::Achievements);
    println!("Achievement strings: {}", achievement_strings.len());
    
    let settings_strings = registry.get_strings_by_category(&StringCategory::Settings);
    println!("Settings strings: {}", settings_strings.len());
    
    // Test specific string metadata
    let main_menu_id = StringId::new("ui.main_menu");
    if let Some(metadata) = registry.get_metadata(&main_menu_id) {
        println!("Main menu string metadata:");
        println!("  Category: {:?}", metadata.category);
        println!("  Max length: {:?}", metadata.max_length);
        println!("  Description: {:?}", metadata.description);
        println!("  Context: {:?}", metadata.context);
    }
    
    // Test character limits
    let mut max_ui_length = 0;
    let mut max_gameplay_length = 0;
    let mut max_item_length = 0;
    
    for string_id in &ui_strings {
        if let Some(metadata) = registry.get_metadata(string_id) {
            if let Some(length) = metadata.max_length {
                max_ui_length = max_ui_length.max(length);
            }
        }
    }
    
    for string_id in &gameplay_strings {
        if let Some(metadata) = registry.get_metadata(string_id) {
            if let Some(length) = metadata.max_length {
                max_gameplay_length = max_gameplay_length.max(length);
            }
        }
    }
    
    for string_id in &item_strings {
        if let Some(metadata) = registry.get_metadata(string_id) {
            if let Some(length) = metadata.max_length {
                max_item_length = max_item_length.max(length);
            }
        }
    }
    
    println!("Character limits:");
    println!("  Max UI string length: {}", max_ui_length);
    println!("  Max gameplay string length: {}", max_gameplay_length);
    println!("  Max item string length: {}", max_item_length);
    
    // Test some specific string IDs
    let test_strings = vec![
        "ui.main_menu",
        "gameplay.health",
        "characters.fighter",
        "items.sword",
        "combat.attack",
        "enemies.goblin",
        "levels.forest_clearing",
        "tutorial.welcome",
        "errors.generic",
        "achievements.first_kill",
        "settings.graphics",
    ];
    
    println!("\nTesting specific string IDs:");
    for string_id_str in test_strings {
        let string_id = StringId::new(string_id_str);
        if let Some(metadata) = registry.get_metadata(&string_id) {
            println!("  {}: max_length={:?}, category={:?}", 
                string_id_str, 
                metadata.max_length, 
                metadata.category
            );
        } else {
            println!("  {}: NOT FOUND", string_id_str);
        }
    }
    
    println!("Comprehensive string registry test completed successfully!");
}
