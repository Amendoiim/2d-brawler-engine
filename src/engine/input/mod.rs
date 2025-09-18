//! Input handling system

use anyhow::Result;
use winit::{
    event::{WindowEvent, ElementState},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};
use std::collections::{HashMap, HashSet};

/// Input manager for handling keyboard, mouse, and gamepad input
pub struct InputManager {
    window_id: WindowId,
    keyboard_state: KeyboardState,
    mouse_state: MouseState,
    gamepad_state: GamepadState,
    input_actions: HashMap<String, InputAction>,
}

/// Keyboard input state
#[derive(Debug, Clone)]
pub struct KeyboardState {
    pressed_keys: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
}

/// Mouse input state
#[derive(Debug, Clone)]
pub struct MouseState {
    position: (f32, f32),
    pressed_buttons: HashSet<winit::event::MouseButton>,
    just_pressed: HashSet<winit::event::MouseButton>,
    just_released: HashSet<winit::event::MouseButton>,
    scroll_delta: f32,
}

/// Gamepad input state
#[derive(Debug, Clone)]
pub struct GamepadState {
    connected: bool,
    buttons: HashSet<gilrs::Button>,
    axes: HashMap<gilrs::Axis, f32>,
    just_pressed: HashSet<gilrs::Button>,
    just_released: HashSet<gilrs::Button>,
}

/// Input action mapping
#[derive(Debug, Clone)]
pub enum InputAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Jump,
    Attack,
    Block,
    Interact,
    Pause,
    Menu,
}

impl InputManager {
    /// Create a new input manager
    pub fn new(window_id: WindowId) -> Result<Self> {
        Ok(Self {
            window_id,
            keyboard_state: KeyboardState::new(),
            mouse_state: MouseState::new(),
            gamepad_state: GamepadState::new(),
            input_actions: Self::create_default_actions(),
        })
    }

    /// Create default input action mappings
    fn create_default_actions() -> HashMap<String, InputAction> {
        let mut actions = HashMap::new();
        
        // Movement actions
        actions.insert("move_up".to_string(), InputAction::MoveUp);
        actions.insert("move_down".to_string(), InputAction::MoveDown);
        actions.insert("move_left".to_string(), InputAction::MoveLeft);
        actions.insert("move_right".to_string(), InputAction::MoveRight);
        
        // Combat actions
        actions.insert("jump".to_string(), InputAction::Jump);
        actions.insert("attack".to_string(), InputAction::Attack);
        actions.insert("block".to_string(), InputAction::Block);
        
        // UI actions
        actions.insert("interact".to_string(), InputAction::Interact);
        actions.insert("pause".to_string(), InputAction::Pause);
        actions.insert("menu".to_string(), InputAction::Menu);
        
        actions
    }

    /// Handle window events
    pub fn handle_window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    match event.state {
                        ElementState::Pressed => {
                            if !self.keyboard_state.pressed_keys.contains(&key_code) {
                                self.keyboard_state.just_pressed.insert(key_code);
                            }
                            self.keyboard_state.pressed_keys.insert(key_code);
                        }
                        ElementState::Released => {
                            if self.keyboard_state.pressed_keys.contains(&key_code) {
                                self.keyboard_state.just_released.insert(key_code);
                            }
                            self.keyboard_state.pressed_keys.remove(&key_code);
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_state.position = (position.x as f32, position.y as f32);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                match state {
                    ElementState::Pressed => {
                        if !self.mouse_state.pressed_buttons.contains(&button) {
                            self.mouse_state.just_pressed.insert(button);
                        }
                        self.mouse_state.pressed_buttons.insert(button);
                    }
                    ElementState::Released => {
                        if self.mouse_state.pressed_buttons.contains(&button) {
                            self.mouse_state.just_released.insert(button);
                        }
                        self.mouse_state.pressed_buttons.remove(&button);
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.mouse_state.scroll_delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };
            }
            _ => {}
        }
    }

    /// Update input state (call at end of frame)
    pub fn update(&mut self) {
        // Clear just pressed/released states
        self.keyboard_state.just_pressed.clear();
        self.keyboard_state.just_released.clear();
        self.mouse_state.just_pressed.clear();
        self.mouse_state.just_released.clear();
        self.gamepad_state.just_pressed.clear();
        self.gamepad_state.just_released.clear();
        
        // Reset scroll delta
        self.mouse_state.scroll_delta = 0.0;
    }

    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keyboard_state.pressed_keys.contains(&key)
    }

    /// Check if a key was just pressed this frame
    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.keyboard_state.just_pressed.contains(&key)
    }

    /// Check if a key was just released this frame
    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.keyboard_state.just_released.contains(&key)
    }

    /// Check if a mouse button is currently pressed
    pub fn is_mouse_button_pressed(&self, button: winit::event::MouseButton) -> bool {
        self.mouse_state.pressed_buttons.contains(&button)
    }

    /// Check if a mouse button was just pressed this frame
    pub fn is_mouse_button_just_pressed(&self, button: winit::event::MouseButton) -> bool {
        self.mouse_state.just_pressed.contains(&button)
    }

    /// Get mouse position
    pub fn mouse_position(&self) -> (f32, f32) {
        self.mouse_state.position
    }

    /// Get mouse scroll delta
    pub fn mouse_scroll_delta(&self) -> f32 {
        self.mouse_state.scroll_delta
    }

    /// Check if an input action is currently active
    pub fn is_action_pressed(&self, action: &str) -> bool {
        if let Some(input_action) = self.input_actions.get(action) {
            match input_action {
                InputAction::MoveUp => self.is_key_pressed(KeyCode::KeyW) || 
                                     self.is_key_pressed(KeyCode::ArrowUp),
                InputAction::MoveDown => self.is_key_pressed(KeyCode::KeyS) || 
                                       self.is_key_pressed(KeyCode::ArrowDown),
                InputAction::MoveLeft => self.is_key_pressed(KeyCode::KeyA) || 
                                       self.is_key_pressed(KeyCode::ArrowLeft),
                InputAction::MoveRight => self.is_key_pressed(KeyCode::KeyD) || 
                                        self.is_key_pressed(KeyCode::ArrowRight),
                InputAction::Jump => self.is_key_pressed(KeyCode::Space),
                InputAction::Attack => self.is_key_pressed(KeyCode::KeyJ) ||
                                     self.is_mouse_button_pressed(winit::event::MouseButton::Left),
                InputAction::Block => self.is_key_pressed(KeyCode::KeyK) ||
                                    self.is_mouse_button_pressed(winit::event::MouseButton::Right),
                InputAction::Interact => self.is_key_pressed(KeyCode::KeyE),
                InputAction::Pause => self.is_key_pressed(KeyCode::Escape),
                InputAction::Menu => self.is_key_pressed(KeyCode::Tab),
            }
        } else {
            false
        }
    }

    /// Check if an input action was just pressed this frame
    pub fn is_action_just_pressed(&self, action: &str) -> bool {
        if let Some(input_action) = self.input_actions.get(action) {
            match input_action {
                InputAction::MoveUp => self.is_key_just_pressed(KeyCode::KeyW) || 
                                     self.is_key_just_pressed(KeyCode::ArrowUp),
                InputAction::MoveDown => self.is_key_just_pressed(KeyCode::KeyS) || 
                                       self.is_key_just_pressed(KeyCode::ArrowDown),
                InputAction::MoveLeft => self.is_key_just_pressed(KeyCode::KeyA) || 
                                       self.is_key_just_pressed(KeyCode::ArrowLeft),
                InputAction::MoveRight => self.is_key_just_pressed(KeyCode::KeyD) || 
                                        self.is_key_just_pressed(KeyCode::ArrowRight),
                InputAction::Jump => self.is_key_just_pressed(KeyCode::Space),
                InputAction::Attack => self.is_key_just_pressed(KeyCode::KeyJ) ||
                                     self.is_mouse_button_just_pressed(winit::event::MouseButton::Left),
                InputAction::Block => self.is_key_just_pressed(KeyCode::KeyK) ||
                                    self.is_mouse_button_just_pressed(winit::event::MouseButton::Right),
                InputAction::Interact => self.is_key_just_pressed(KeyCode::KeyE),
                InputAction::Pause => self.is_key_just_pressed(KeyCode::Escape),
                InputAction::Menu => self.is_key_just_pressed(KeyCode::Tab),
            }
        } else {
            false
        }
    }

    /// Get movement input as a vector
    pub fn get_movement_input(&self) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;

        if self.is_action_pressed("move_left") {
            x -= 1.0;
        }
        if self.is_action_pressed("move_right") {
            x += 1.0;
        }
        if self.is_action_pressed("move_up") {
            y += 1.0;
        }
        if self.is_action_pressed("move_down") {
            y -= 1.0;
        }

        (x, y)
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

impl KeyboardState {
    fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }
}

impl MouseState {
    fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            pressed_buttons: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
            scroll_delta: 0.0,
        }
    }
}

impl GamepadState {
    fn new() -> Self {
        Self {
            connected: false,
            buttons: HashSet::new(),
            axes: HashMap::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }
}
