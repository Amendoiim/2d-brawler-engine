//! Platform abstraction layer

use anyhow::Result;
use std::time::{Duration, Instant};
use winit::window::Window;

/// Platform abstraction for cross-platform functionality
pub struct Platform {
    window: Window,
    last_frame_time: Instant,
    delta_time: Duration,
}

impl Platform {
    /// Create a new platform instance
    pub fn new(window: Window) -> Result<Self> {
        Ok(Self {
            window,
            last_frame_time: Instant::now(),
            delta_time: Duration::ZERO,
        })
    }

    /// Get reference to the window
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Get the time delta since last frame
    pub fn delta_time(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    /// Update frame timing
    pub fn update_timing(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;
    }
}
