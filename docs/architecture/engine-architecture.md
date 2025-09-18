# Engine Architecture Documentation

## Overview

The 2D Brawler Engine is built using a modular architecture designed for high performance, maintainability, and cross-platform compatibility. The engine follows modern game development patterns with a focus on the Entity-Component-System (ECS) paradigm.

**Current Status:** Phase 1 Complete ✅ | Phase 2 Active 🚀

## Development Phases

### Phase 1: Foundation (Complete ✅)
- **Engine Architecture** - Modular system design established
- **ECS Framework** - Basic entity-component-system implementation
- **Core Systems** - Rendering, physics, audio, input, scene, asset management
- **Game Components** - Combat, character, level, progression systems
- **Platform Support** - Cross-platform compatibility foundation

### Phase 2: Feature Implementation (Active 🚀)
- [x] **Real Rendering** - WGPU sprite rendering foundation with ECS integration
- [x] **Functional ECS** - Query system and system manager with borrowing fixes
- [x] **Input Processing** - Full keyboard, mouse, and action mapping system
- [x] **Physics Simulation** - Rapier2D physics integration with collision setup
- [ ] **Audio Playback** - Load and play actual sound files
- [ ] **Asset Loading** - Implement file loading and texture display
- [ ] **Game Logic** - Character movement and basic combat
- [ ] **Scene Management** - Real scene loading and transitions

## Core Architecture Principles

### 1. Modular Design
- Each system is self-contained with clear interfaces
- Loose coupling between modules
- High cohesion within modules

### 2. Performance First
- Zero-allocation hot paths where possible
- SIMD-optimized math operations
- Efficient memory layout and cache-friendly data structures

### 3. Cross-Platform Compatibility
- Abstract platform-specific functionality
- Consistent behavior across all target platforms
- Platform-specific optimizations where beneficial

## System Architecture

### Core Engine Layer

```
┌─────────────────────────────────────────────────────────┐
│                    Application Layer                    │
├─────────────────────────────────────────────────────────┤
│                    Game Logic Layer                     │
├─────────────────────────────────────────────────────────┤
│                    Engine Core Layer                    │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐      │
│  │   Renderer  │ │   Physics   │ │    Audio    │      │
│  └─────────────┘ └─────────────┘ └─────────────┘      │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐      │
│  │    Input    │ │  Networking │ │    ECS      │      │
│  └─────────────┘ └─────────────┘ └─────────────┘      │
├─────────────────────────────────────────────────────────┤
│                  Platform Abstraction Layer            │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐      │
│  │   WGPU      │ │   Winit     │ │   Rodio     │      │
│  └─────────────┘ └─────────────┘ └─────────────┘      │
└─────────────────────────────────────────────────────────┘
```

### Entity-Component-System (ECS)

The engine uses an ECS architecture for optimal performance and flexibility:

#### Entities
- Unique identifiers for game objects
- No data storage, just handles to components

#### Components
- Pure data containers
- No logic or behavior
- Examples: Position, Velocity, Sprite, Health

#### Systems
- Logic that operates on entities with specific component combinations
- Examples: MovementSystem, RenderSystem, CollisionSystem

#### Example ECS Usage
```rust
// Components
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Sprite {
    texture_id: u32,
    width: f32,
    height: f32,
}

// System
struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        for (pos, vel) in world.query::<(&mut Position, &Velocity)>() {
            pos.x += vel.x * dt;
            pos.y += vel.y * dt;
        }
    }
}
```

## Rendering System

### Architecture
- **Renderer Core:** Manages GPU resources and rendering pipeline
- **Sprite Batcher:** Efficiently batches sprites for GPU rendering
- **Shader System:** Manages vertex and fragment shaders
- **Texture Manager:** Handles texture loading and caching

### Performance Optimizations
- **Sprite Batching:** Groups sprites by texture for efficient rendering
- **Frustum Culling:** Only renders sprites within the camera view
- **Texture Atlasing:** Reduces draw calls by combining textures
- **Instanced Rendering:** Renders multiple similar objects in one draw call

### Rendering Pipeline
1. **Culling:** Determine visible objects
2. **Batching:** Group sprites by material/texture
3. **Upload:** Send vertex data to GPU
4. **Draw:** Execute draw calls
5. **Present:** Display final frame

## Physics System

### 2D Physics Engine
- **Collision Detection:** Broad phase and narrow phase collision detection
- **Collision Response:** Impulse-based collision resolution
- **Spatial Partitioning:** Optimized collision queries using spatial hashing
- **Constraint Solving:** Joint and constraint systems

### Physics Components
```rust
#[derive(Component)]
struct RigidBody {
    mass: f32,
    inertia: f32,
    velocity: Vec2,
    angular_velocity: f32,
}

#[derive(Component)]
struct Collider {
    shape: CollisionShape,
    is_trigger: bool,
    material: PhysicsMaterial,
}
```

## Audio System

### Architecture
- **Audio Engine:** Manages audio context and resources
- **Spatial Audio:** 3D positional audio for immersive experience
- **Music System:** Handles background music and transitions
- **SFX Manager:** Manages sound effects and audio pools

### Features
- **Spatial Audio:** Position-based audio with distance attenuation
- **Audio Streaming:** Efficient loading of large audio files
- **Audio Compression:** Support for various audio formats
- **Dynamic Range:** Automatic volume adjustment based on game state

## Input System

### Multi-Device Support
- **Keyboard/Mouse:** Standard PC input
- **Gamepad:** Xbox, PlayStation, and generic controllers
- **Touch:** Mobile and tablet input (future)

### Input Mapping
- **Action-Based Input:** Map physical inputs to game actions
- **Input Buffering:** Frame-perfect input timing
- **Customizable Controls:** Player-defined control schemes

## Networking System

### Multiplayer Architecture
- **Client-Server Model:** Authoritative server with client prediction
- **Rollback Netcode:** Smooth gameplay with network compensation
- **State Synchronization:** Efficient delta compression
- **Lobby System:** Matchmaking and room management

### Network Protocol
- **QUIC Protocol:** Reliable, low-latency communication
- **Delta Compression:** Only send changed data
- **Input Prediction:** Client-side prediction for responsiveness
- **Lag Compensation:** Server-side rollback for fairness

## Memory Management

### Allocation Strategy
- **Arena Allocators:** Fast allocation for temporary data
- **Object Pools:** Reuse objects to avoid allocation
- **Memory Pools:** Pre-allocated memory for specific types
- **Garbage Collection:** None (Rust's ownership system)

### Performance Considerations
- **Cache-Friendly Layout:** Data structures optimized for CPU cache
- **SIMD Operations:** Vectorized math operations where possible
- **Memory Alignment:** Proper alignment for SIMD operations
- **Zero-Copy Operations:** Minimize data copying

## Threading Model

### Thread Architecture
- **Main Thread:** Rendering, input, and game logic
- **Audio Thread:** Audio processing and playback
- **Network Thread:** Network communication
- **Worker Threads:** Background tasks (asset loading, etc.)

### Synchronization
- **Lock-Free Data Structures:** Where possible for performance
- **Message Passing:** Communication between threads
- **Atomic Operations:** Thread-safe shared data
- **Mutexes:** When shared mutable state is necessary

## Asset Management

### Asset Pipeline
- **Asset Loading:** Asynchronous asset loading
- **Asset Caching:** Memory and disk caching
- **Asset Compression:** Efficient storage and loading
- **Hot Reloading:** Development-time asset updates

### Supported Formats
- **Images:** PNG, JPEG, WebP
- **Audio:** OGG, WAV, MP3
- **Data:** JSON, TOML, Binary
- **Shaders:** WGSL (WebGPU Shading Language)

## Performance Targets

### Frame Rate
- **Target:** 120 FPS stable
- **Minimum:** 60 FPS on low-end hardware
- **Variable Refresh Rate:** Support for G-Sync/FreeSync

### Resolution Support
- **4K (3840x2160):** Primary target
- **1440p (2560x1440):** High-end target
- **1080p (1920x1080):** Standard target
- **720p (1280x720):** Minimum target

### Memory Usage
- **Target:** < 2GB RAM
- **Peak:** < 4GB RAM
- **Streaming:** Efficient asset streaming

## Platform-Specific Considerations

### macOS
- **Metal API:** Native graphics API
- **Core Audio:** Native audio system
- **App Store:** Compliance requirements

### Windows
- **DirectX 12:** High-performance graphics
- **XAudio2:** Windows audio system
- **Steam:** Integration support

### Consoles
- **Xbox:** DirectX 12 Ultimate
- **PlayStation 5:** GNMX graphics API
- **Nintendo Switch 2:** NVN API (speculated)

## Development Tools

### Debugging
- **Render Debugger:** Visual debugging of rendering pipeline
- **Physics Debugger:** Collision visualization
- **Performance Profiler:** Frame time analysis
- **Memory Profiler:** Memory usage tracking

### Testing
- **Unit Tests:** Individual system testing
- **Integration Tests:** System interaction testing
- **Performance Tests:** Benchmarking and regression testing
- **Platform Tests:** Cross-platform compatibility testing
