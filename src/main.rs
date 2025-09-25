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
    let idle_animation = Animation::new("idle".to_string())
        .set_looping(true)
        .set_speed(1.0)
        .set_priority(1);

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

    let walk_animation = Animation::new("walk".to_string())
        .set_looping(true)
        .set_speed(1.5)
        .set_priority(2);

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

    let attack_animation = Animation::new("attack".to_string())
        .set_looping(false)
        .set_speed(2.0)
        .set_priority(3);

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
