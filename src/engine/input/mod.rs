//! Input handling system

use anyhow::Result;
use winit::{
    event::WindowEvent,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

/// Input manager for handling keyboard, mouse, and gamepad input
pub struct InputManager {
    window_id: WindowId,
    keyboard_state: std::collections::HashSet<KeyCode>,
    mouse_state: MouseState,
    gamepad_state: GamepadState,
}

#[derive(Default)]
struct MouseState {
    position: (f32, f32),
    buttons: std::collections::HashSet<winit::event::MouseButton>,
}

#[derive(Default)]
struct GamepadState {
    connected: bool,
    buttons: std::collections::HashSet<gilrs::Button>,
    axes: std::collections::HashMap<gilrs::Axis, f32>,
}

impl InputManager {
    /// Create a new input manager
    pub fn new(window_id: WindowId) -> Result<Self> {
        Ok(Self {
            window_id,
            keyboard_state: std::collections::HashSet::new(),
            mouse_state: MouseState::default(),
            gamepad_state: GamepadState::default(),
        })
    }

    /// Handle window events
    pub fn handle_window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    match event.state {
                        winit::event::ElementState::Pressed => {
                            self.keyboard_state.insert(key_code);
                        }
                        winit::event::ElementState::Released => {
                            self.keyboard_state.remove(&key_code);
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_state.position = (position.x as f32, position.y as f32);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                match state {
                    winit::event::ElementState::Pressed => {
                        self.mouse_state.buttons.insert(button);
                    }
                    winit::event::ElementState::Released => {
                        self.mouse_state.buttons.remove(&button);
                    }
                }
            }
            _ => {}
        }
    }

    /// Update input state
    pub fn update(&mut self) {
        // Update gamepad state if needed
        // This would typically involve polling gamepad input
    }

    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keyboard_state.contains(&key)
    }

    /// Check if a mouse button is currently pressed
    pub fn is_mouse_button_pressed(&self, button: winit::event::MouseButton) -> bool {
        self.mouse_state.buttons.contains(&button)
    }

    /// Get mouse position
    pub fn mouse_position(&self) -> (f32, f32) {
        self.mouse_state.position
    }

    /// Check if a gamepad button is currently pressed
    pub fn is_gamepad_button_pressed(&self, button: gilrs::Button) -> bool {
        self.gamepad_state.connected && self.gamepad_state.buttons.contains(&button)
    }

    /// Get gamepad axis value
    pub fn gamepad_axis(&self, axis: gilrs::Axis) -> f32 {
        self.gamepad_state.axes.get(&axis).copied().unwrap_or(0.0)
    }
}
