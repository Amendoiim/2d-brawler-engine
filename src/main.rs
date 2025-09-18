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
