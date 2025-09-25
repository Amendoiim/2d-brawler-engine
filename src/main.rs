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
use engine::level::{LevelGenerator, LevelConfig, GenerationAlgorithm, AlgorithmParams};

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
        match engine.level_generator_mut().generate_level_advanced(config.clone(), algorithm) {
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
            let interactive_objects = engine.level_generator().get_interactive_objects();
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
                    match engine.level_generator_mut().damage_interactive_object(&first_object.id, 25.0) {
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
                let background_layers = engine.level_generator().get_background_layers();
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
                let visible_layers = engine.level_generator().get_visible_background_layers();
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
                let atmospheric_effects = engine.level_generator().get_atmospheric_effects();
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
                let active_effects = engine.level_generator().get_active_atmospheric_effects();
                log::info!("  Active effects count: {}", active_effects.len());
                
                // Test global lighting
                let global_lighting = engine.level_generator().get_global_lighting();
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
                let weather_system = engine.level_generator().get_weather_system();
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
