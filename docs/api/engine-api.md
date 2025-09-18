# Engine API Documentation

## Overview

This document provides detailed API documentation for the 2D Brawler Engine. The engine is built using Rust and follows modern game development patterns with an Entity-Component-System (ECS) architecture.

**Current Status:** Phase 1 Complete ✅ | Phase 2 Active 🚀

## Development Status

### Phase 1: Foundation (Complete ✅)
- **Engine Architecture** - Modular system design with ECS
- **Core Systems** - Basic implementations of all major systems
- **Game Components** - Component definitions for game logic
- **Platform Support** - Cross-platform compatibility

### Phase 2: Feature Implementation (Active 🚀)
- **Real Rendering** - Actual WGPU sprite rendering
- **Functional ECS** - Fix borrowing issues and system execution
- **Input Processing** - Connect input to game actions
- **Physics Simulation** - Implement actual physics with collision
- **Audio Playback** - Load and play actual sound files
- **Asset Loading** - Implement file loading and texture display
- **Game Logic** - Character movement and basic combat
- **Scene Management** - Real scene loading and transitions

## API Status Legend

- ✅ **Implemented** - Fully functional in Phase 1
- 🔄 **Placeholder** - Basic implementation, needs Phase 2 work
- 🚧 **In Progress** - Currently being implemented in Phase 2
- 📋 **Planned** - Scheduled for future phases

## Core Engine

### Engine

The main engine struct that coordinates all systems.

```rust
pub struct Engine {
    platform: Platform,
    renderer: Renderer,
    physics: PhysicsWorld,
    audio: AudioEngine,
    input: InputManager,
    ecs: World,
    scene: SceneManager,
    asset_manager: AssetManager,
}
```

#### Methods

##### `new(window: Window) -> Result<Self>`
Creates a new engine instance with the specified window.

**Parameters:**
- `window`: The window to render to

**Returns:**
- `Result<Self>`: The new engine instance or an error

##### `window() -> &Window`
Gets a reference to the engine's window.

**Returns:**
- `&Window`: Reference to the window

##### `handle_window_event(event: WindowEvent)`
Handles window events from the operating system.

**Parameters:**
- `event`: The window event to handle

##### `resize(size: PhysicalSize<u32>)`
Resizes the rendering surface to the specified size.

**Parameters:**
- `size`: The new size in pixels

##### `render() -> Result<()>`
Renders a single frame.

**Returns:**
- `Result<()>`: Success or error

## Entity-Component-System (ECS)

### World

The ECS world that manages entities, components, and systems.

```rust
pub struct World {
    entities: Vec<Entity>,
    next_entity_id: Entity,
    components: HashMap<TypeId, Box<dyn std::any::Any>>,
    systems: Vec<Box<dyn System>>,
}
```

#### Methods

##### `new() -> Self`
Creates a new ECS world.

**Returns:**
- `Self`: The new world instance

##### `create_entity() -> Entity`
Creates a new entity and returns its ID.

**Returns:**
- `Entity`: The new entity ID

##### `add_component<T: Component>(entity: Entity, component: T)`
Adds a component to an entity.

**Parameters:**
- `entity`: The entity to add the component to
- `component`: The component to add

##### `get_component<T: Component>(entity: Entity) -> Option<&T>`
Gets a component from an entity.

**Parameters:**
- `entity`: The entity to get the component from

**Returns:**
- `Option<&T>`: The component or None if not found

##### `get_component_mut<T: Component>(entity: Entity) -> Option<&mut T>`
Gets a mutable component from an entity.

**Parameters:**
- `entity`: The entity to get the component from

**Returns:**
- `Option<&mut T>`: Mutable reference to the component or None if not found

##### `remove_component<T: Component>(entity: Entity)`
Removes a component from an entity.

**Parameters:**
- `entity`: The entity to remove the component from

##### `add_system<T: System + 'static>(system: T)`
Adds a system to the world.

**Parameters:**
- `system`: The system to add

##### `update(dt: f32)`
Updates all systems in the world.

**Parameters:**
- `dt`: Delta time in seconds

##### `query<T: Component>() -> Vec<Entity>`
Queries all entities that have a specific component.

**Parameters:**
- `T`: The component type to query for

**Returns:**
- `Vec<Entity>`: List of entities with the component

### Component Trait

```rust
pub trait Component: 'static + Send + Sync {}
```

All components must implement this trait.

### System Trait

```rust
pub trait System {
    fn update(&mut self, world: &mut World, dt: f32);
}
```

All systems must implement this trait.

## Rendering System

### Renderer

The 2D renderer using WGPU.

```rust
pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
}
```

#### Methods

##### `new(platform: &Platform) -> Result<Self>`
Creates a new renderer.

**Parameters:**
- `platform`: The platform abstraction

**Returns:**
- `Result<Self>`: The new renderer or an error

##### `resize(size: PhysicalSize<u32>)`
Resizes the rendering surface.

**Parameters:**
- `size`: The new size in pixels

##### `render(world: &World) -> Result<()>`
Renders a frame using the ECS world.

**Parameters:**
- `world`: The ECS world to render

**Returns:**
- `Result<()>`: Success or error

## Physics System

### PhysicsWorld

The 2D physics world using Rapier.

```rust
pub struct PhysicsWorld {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    integration_parameters: IntegrationParameters,
    gravity: Vector<f32>,
}
```

#### Methods

##### `new() -> Result<Self>`
Creates a new physics world.

**Returns:**
- `Result<Self>`: The new physics world or an error

##### `step(dt: f32)`
Steps the physics simulation forward by the specified time.

**Parameters:**
- `dt`: Delta time in seconds

##### `add_rigid_body(body: RigidBody) -> RigidBodyHandle`
Adds a rigid body to the world.

**Parameters:**
- `body`: The rigid body to add

**Returns:**
- `RigidBodyHandle`: Handle to the added rigid body

##### `add_collider(collider: Collider, parent: RigidBodyHandle) -> ColliderHandle`
Adds a collider to the world.

**Parameters:**
- `collider`: The collider to add
- `parent`: The parent rigid body

**Returns:**
- `ColliderHandle`: Handle to the added collider

## Audio System

### AudioEngine

The audio engine for managing sound effects and music.

```rust
pub struct AudioEngine {
    _stream: OutputStream,
    sink: Sink,
    sounds: HashMap<String, Vec<u8>>,
}
```

#### Methods

##### `new() -> Result<Self>`
Creates a new audio engine.

**Returns:**
- `Result<Self>`: The new audio engine or an error

##### `load_sound(name: &str, data: Vec<u8>)`
Loads a sound from memory.

**Parameters:**
- `name`: The name to associate with the sound
- `data`: The sound data

##### `play_sound(name: &str) -> Result<()>`
Plays a sound effect.

**Parameters:**
- `name`: The name of the sound to play

**Returns:**
- `Result<()>`: Success or error

##### `play_music(name: &str) -> Result<()>`
Plays background music.

**Parameters:**
- `name`: The name of the music to play

**Returns:**
- `Result<()>`: Success or error

##### `set_volume(volume: f32)`
Sets the audio volume.

**Parameters:**
- `volume`: Volume level (0.0 to 1.0)

## Input System

### InputManager

The input manager for handling keyboard, mouse, and gamepad input.

```rust
pub struct InputManager {
    window_id: WindowId,
    keyboard_state: std::collections::HashSet<KeyCode>,
    mouse_state: MouseState,
    gamepad_state: GamepadState,
}
```

#### Methods

##### `new(window_id: WindowId) -> Result<Self>`
Creates a new input manager.

**Parameters:**
- `window_id`: The window ID to associate with

**Returns:**
- `Result<Self>`: The new input manager or an error

##### `handle_window_event(event: WindowEvent)`
Handles window events.

**Parameters:**
- `event`: The window event to handle

##### `update()`
Updates the input state.

##### `is_key_pressed(key: KeyCode) -> bool`
Checks if a key is currently pressed.

**Parameters:**
- `key`: The key to check

**Returns:**
- `bool`: True if the key is pressed

##### `is_mouse_button_pressed(button: MouseButton) -> bool`
Checks if a mouse button is currently pressed.

**Parameters:**
- `button`: The mouse button to check

**Returns:**
- `bool`: True if the button is pressed

##### `mouse_position() -> (f32, f32)`
Gets the current mouse position.

**Returns:**
- `(f32, f32)`: The mouse position in pixels

##### `is_gamepad_button_pressed(button: Button) -> bool`
Checks if a gamepad button is currently pressed.

**Parameters:**
- `button`: The gamepad button to check

**Returns:**
- `bool`: True if the button is pressed

##### `gamepad_axis(axis: Axis) -> f32`
Gets the value of a gamepad axis.

**Parameters:**
- `axis`: The axis to query

**Returns:**
- `f32`: The axis value (-1.0 to 1.0)

## Game Components

### Position

```rust
#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
```

### Velocity

```rust
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
```

### Sprite

```rust
#[derive(Component)]
pub struct Sprite {
    pub texture_id: u32,
    pub width: f32,
    pub height: f32,
}
```

### Health

```rust
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
}
```

### Character

```rust
#[derive(Component)]
pub struct Character {
    pub name: String,
    pub character_class: CharacterClass,
    pub level: u32,
    pub experience: u32,
}
```

### Combat

```rust
#[derive(Component)]
pub struct Combat {
    pub attack_power: f32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub last_attack_time: f32,
}
```

## Game Systems

### MovementSystem

Updates entity positions based on velocity.

```rust
pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, world: &mut World, dt: f32);
}
```

### CombatSystem

Handles combat mechanics and attacks.

```rust
pub struct CombatSystem;

impl System for CombatSystem {
    fn update(&mut self, world: &mut World, dt: f32);
}
```

### CharacterProgressionSystem

Manages character leveling and progression.

```rust
pub struct CharacterProgressionSystem;

impl System for CharacterProgressionSystem {
    fn update(&mut self, world: &mut World, dt: f32);
}
```

### LevelManagementSystem

Handles level generation and management.

```rust
pub struct LevelManagementSystem {
    current_level: Option<Level>,
    generator: LevelGenerator,
}

impl System for LevelManagementSystem {
    fn update(&mut self, world: &mut World, dt: f32);
}
```

## Error Handling

The engine uses `anyhow::Result<T>` for error handling throughout the codebase. This provides a consistent error handling pattern with detailed error information and context.

## Threading

The engine is designed to be single-threaded for simplicity, but the architecture allows for easy multi-threading in the future. The ECS system is designed to be thread-safe and can be extended to support parallel system execution.

## Memory Management

The engine uses Rust's ownership system for memory management, eliminating the need for garbage collection. This provides predictable performance and prevents common memory-related bugs.

## Performance Considerations

- The ECS system is designed for cache-friendly data access
- Components are stored in contiguous arrays for optimal performance
- Systems are designed to minimize allocations in hot paths
- The rendering system uses efficient batching for sprites
- Physics simulation is optimized for 2D games
