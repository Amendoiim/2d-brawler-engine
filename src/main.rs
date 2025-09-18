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
use game::{Position, Velocity, Sprite, MovementSystem, InputMovementSystem};

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
    // Create a test entity with position, velocity, and sprite
    let entity = engine.ecs_mut().create_entity();
    
    engine.ecs_mut().add_component(entity, Position { x: 100.0, y: 100.0 });
    engine.ecs_mut().add_component(entity, Velocity { x: 50.0, y: 25.0 });
    engine.ecs_mut().add_component(entity, Sprite {
        texture_id: 0,
        width: 32.0,
        height: 32.0,
        color: [1.0, 0.0, 0.0, 1.0], // Red
        visible: true,
    });
    
    // Add movement systems
    engine.ecs_mut().add_system(MovementSystem);
    engine.ecs_mut().add_system(InputMovementSystem::new());
    
    info!("Created test scene with entity {}", entity);
}
