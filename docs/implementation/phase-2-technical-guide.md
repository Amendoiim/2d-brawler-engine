# Phase 2 Technical Implementation Guide
## 2D Brawler Engine - Step-by-Step Implementation

**Phase Status:** 🚧 **PLANNING**  
**Focus:** Technical implementation details and code examples  
**Goal:** Detailed guide for implementing Phase 2 features

---

## 🔧 **Week 1: Core Systems Implementation**

### **Day 1-2: ECS System Completion**

#### **Problem: Borrowing Issues in ECS**
Current issue: Can't borrow `World` mutably and immutably simultaneously.

#### **Solution: Query System Redesign**
```rust
// src/engine/ecs/query.rs
pub struct Query<'a, T: Component> {
    world: &'a World,
    component_type: TypeId,
    entities: Vec<Entity>,
}

impl<'a, T: Component> Query<'a, T> {
    pub fn new(world: &'a World) -> Self {
        let component_type = TypeId::of::<T>();
        let entities = world.get_entities_with_component::<T>();
        
        Self {
            world,
            component_type,
            entities,
        }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.entities.iter().filter_map(|&entity| {
            self.world.get_component::<T>(entity).map(|comp| (entity, comp))
        })
    }
}

pub struct QueryMut<'a, T: Component> {
    world: &'a mut World,
    component_type: TypeId,
    entities: Vec<Entity>,
}

impl<'a, T: Component> QueryMut<'a, T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        let entities = self.entities.clone();
        entities.into_iter().filter_map(|entity| {
            self.world.get_component_mut::<T>(entity).map(|comp| (entity, comp))
        })
    }
}
```

#### **System Execution Pipeline**
```rust
// src/engine/ecs/system_manager.rs
pub struct SystemManager {
    systems: Vec<Box<dyn System>>,
    execution_order: Vec<usize>,
}

impl SystemManager {
    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system));
        self.update_execution_order();
    }
    
    pub fn update(&mut self, world: &mut World, dt: f32) {
        for &system_index in &self.execution_order {
            if let Some(system) = self.systems.get_mut(system_index) {
                system.update(world, dt);
            }
        }
    }
    
    fn update_execution_order(&mut self) {
        let mut indices: Vec<usize> = (0..self.systems.len()).collect();
        indices.sort_by_key(|&i| self.systems[i].priority());
        self.execution_order = indices;
    }
}
```

### **Day 3-4: Real WGPU Rendering**

#### **Fix Raw Window Handle Issues**
```rust
// Update Cargo.toml
[dependencies]
wgpu = "0.18"  # Use latest version
raw-window-handle = "0.6"  # Ensure compatibility

// src/engine/renderer/mod.rs
use raw_window_handle::{HasRawWindowHandle, HasRawDisplayHandle};

impl Renderer {
    pub fn new(window: &Window) -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        
        // Fix raw window handle compatibility
        let surface = unsafe {
            instance.create_surface(&wgpu::SurfaceTarget::from_window(window)?
        }?;
        
        // Rest of implementation...
    }
}
```

#### **Sprite Rendering Implementation**
```rust
// src/engine/renderer/sprite.rs
pub struct SpriteRenderer {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
    texture: wgpu::Texture,
}

impl SpriteRenderer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        // Create vertex buffer
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Vertex Buffer"),
            size: (std::mem::size_of::<Vertex>() * 1000) as u64, // 1000 sprites max
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create index buffer
        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Index Buffer"),
            size: (std::mem::size_of::<u16>() * 6000) as u64, // 1000 sprites * 6 indices
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create uniform buffer
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Uniform Buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Sprite Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        
        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Sprite Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        
        // Create render pipeline
        let render_pipeline = Self::create_render_pipeline(device, config, &bind_group_layout);
        
        Self {
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            bind_group,
            render_pipeline,
            texture_bind_group: Self::create_texture_bind_group(device),
            texture: Self::create_default_texture(device),
        }
    }
    
    pub fn render_sprites(
        &self,
        render_pass: &mut wgpu::RenderPass,
        world: &World,
        camera: &Camera,
    ) {
        // Query all sprites and positions
        let sprites = world.query::<Sprite>();
        let positions = world.query::<Position>();
        
        // Update vertex buffer with sprite data
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        for (entity, sprite) in sprites.iter() {
            if let Some(position) = positions.get(entity) {
                if sprite.visible {
                    let sprite_vertices = Self::create_sprite_vertices(
                        position.x, position.y,
                        sprite.width, sprite.height,
                        sprite.color,
                    );
                    
                    let base_index = vertices.len() as u16;
                    vertices.extend_from_slice(&sprite_vertices);
                    
                    // Add quad indices
                    indices.extend_from_slice(&[
                        base_index, base_index + 1, base_index + 2,
                        base_index + 2, base_index + 3, base_index,
                    ]);
                }
            }
        }
        
        // Update buffers
        // (In real implementation, you'd update the GPU buffers here)
        
        // Render sprites
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_bind_group(1, &self.texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..vertices.len() as u32);
    }
}
```

---

## 🎮 **Week 2: Input & Physics Integration**

### **Day 8-10: Input System Implementation**

#### **Input Event Processing**
```rust
// src/engine/input/input_manager.rs
pub struct InputManager {
    keyboard_state: KeyboardState,
    mouse_state: MouseState,
    gamepad_state: GamepadState,
    input_mappings: HashMap<InputAction, Vec<InputBinding>>,
}

impl InputManager {
    pub fn new() -> Self {
        let mut input_mappings = HashMap::new();
        
        // Default keyboard mappings
        input_mappings.insert(InputAction::MoveLeft, vec![
            InputBinding::keyboard(KeyCode::ArrowLeft),
            InputBinding::keyboard(KeyCode::KeyA),
        ]);
        input_mappings.insert(InputAction::MoveRight, vec![
            InputBinding::keyboard(KeyCode::ArrowRight),
            InputBinding::keyboard(KeyCode::KeyD),
        ]);
        input_mappings.insert(InputAction::MoveUp, vec![
            InputBinding::keyboard(KeyCode::ArrowUp),
            InputBinding::keyboard(KeyCode::KeyW),
        ]);
        input_mappings.insert(InputAction::MoveDown, vec![
            InputBinding::keyboard(KeyCode::ArrowDown),
            InputBinding::keyboard(KeyCode::KeyS),
        ]);
        input_mappings.insert(InputAction::Attack, vec![
            InputBinding::keyboard(KeyCode::Space),
            InputBinding::mouse(MouseButton::Left),
        ]);
        
        Self {
            keyboard_state: KeyboardState::new(),
            mouse_state: MouseState::new(),
            gamepad_state: GamepadState::new(),
            input_mappings,
        }
    }
    
    pub fn process_input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                self.process_keyboard_input(event);
            }
            WindowEvent::MouseInput { button, state, .. } => {
                self.process_mouse_input(*button, *state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.process_cursor_movement(*position);
            }
            _ => {}
        }
    }
    
    pub fn is_action_pressed(&self, action: InputAction) -> bool {
        if let Some(bindings) = self.input_mappings.get(&action) {
            bindings.iter().any(|binding| self.is_binding_pressed(binding))
        } else {
            false
        }
    }
    
    fn is_binding_pressed(&self, binding: &InputBinding) -> bool {
        match binding.input_type {
            InputType::Keyboard => {
                if let Some(key) = binding.key {
                    self.keyboard_state.is_pressed(key)
                } else {
                    false
                }
            }
            InputType::Mouse => {
                if let Some(button) = binding.mouse_button {
                    self.mouse_state.is_pressed(button)
                } else {
                    false
                }
            }
            InputType::Gamepad => {
                if let Some(button) = binding.gamepad_button {
                    self.gamepad_state.is_pressed(button)
                } else {
                    false
                }
            }
        }
    }
}
```

#### **Character Movement System**
```rust
// src/game/systems/movement.rs
pub struct PlayerInputSystem {
    input_manager: InputManager,
}

impl System for PlayerInputSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        let mut query = world.query_mut::<Velocity>();
        
        for (entity, velocity) in query.iter_mut() {
            // Reset horizontal velocity
            velocity.x = 0.0;
            
            // Process input
            if self.input_manager.is_action_pressed(InputAction::MoveLeft) {
                velocity.x = -200.0;
            }
            if self.input_manager.is_action_pressed(InputAction::MoveRight) {
                velocity.x = 200.0;
            }
            if self.input_manager.is_action_pressed(InputAction::MoveUp) {
                velocity.y = -200.0;
            }
            if self.input_manager.is_action_pressed(InputAction::MoveDown) {
                velocity.y = 200.0;
            }
        }
    }
}

pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        let mut query = world.query_mut::<(Position, Velocity)>();
        
        for (entity, (position, velocity)) in query.iter_mut() {
            // Update position based on velocity
            position.x += velocity.x * dt;
            position.y += velocity.y * dt;
            
            // Apply friction
            velocity.x *= 0.9;
            velocity.y *= 0.9;
        }
    }
}
```

### **Day 11-14: Physics Integration**

#### **Physics-ECS Synchronization**
```rust
// src/engine/physics/physics_system.rs
pub struct PhysicsSystem {
    physics_world: PhysicsWorld,
    entity_to_body: HashMap<Entity, RigidBodyHandle>,
    body_to_entity: HashMap<RigidBodyHandle, Entity>,
}

impl PhysicsSystem {
    pub fn step(&mut self, world: &mut World, dt: f32) {
        // Step physics simulation
        self.physics_world.step(dt);
        
        // Sync physics bodies with ECS components
        self.sync_physics_to_ecs(world);
        self.sync_ecs_to_physics(world);
    }
    
    fn sync_physics_to_ecs(&self, world: &mut World) {
        for (body_handle, entity) in &self.body_to_entity {
            if let Some(body) = self.physics_world.rigid_body_set.get(*body_handle) {
                if let Some(position) = world.get_component_mut::<Position>(*entity) {
                    let pos = body.translation();
                    position.x = pos.x;
                    position.y = pos.y;
                }
                
                if let Some(velocity) = world.get_component_mut::<Velocity>(*entity) {
                    let vel = body.linvel();
                    velocity.x = vel.x;
                    velocity.y = vel.y;
                }
            }
        }
    }
    
    fn sync_ecs_to_physics(&mut self, world: &World) {
        for (entity, body_handle) in &self.entity_to_body {
            if let Some(body) = self.physics_world.rigid_body_set.get_mut(*body_handle) {
                if let Some(position) = world.get_component::<Position>(*entity) {
                    body.set_translation(vector![position.x, position.y], true);
                }
                
                if let Some(velocity) = world.get_component::<Velocity>(*entity) {
                    body.set_linvel(vector![velocity.x, velocity.y], true);
                }
            }
        }
    }
}
```

---

## 🔊 **Week 3: Audio & Asset Management**

### **Day 15-17: Audio System Implementation**

#### **Sound File Loading and Playback**
```rust
// src/engine/audio/audio_engine.rs
pub struct AudioEngine {
    _stream: OutputStream,
    sink: Sink,
    music_sink: Sink,
    sounds: HashMap<String, Vec<u8>>,
    current_music: Option<String>,
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let music_sink = Sink::try_new(&stream_handle)?;
        
        Ok(Self {
            _stream,
            sink,
            music_sink,
            sounds: HashMap::new(),
            current_music: None,
        })
    }
    
    pub fn load_sound(&mut self, name: &str, data: Vec<u8>) -> Result<()> {
        self.sounds.insert(name.to_string(), data);
        Ok(())
    }
    
    pub fn play_sound(&self, name: &str) -> Result<()> {
        if let Some(sound_data) = self.sounds.get(name) {
            let cursor = std::io::Cursor::new(sound_data.clone());
            let source = rodio::Decoder::new(cursor)?;
            self.sink.append(source);
        }
        Ok(())
    }
    
    pub fn play_music(&mut self, name: &str) -> Result<()> {
        if let Some(music_data) = self.sounds.get(name) {
            let cursor = std::io::Cursor::new(music_data.clone());
            let source = rodio::Decoder::new(cursor)?;
            self.music_sink.append(source);
            self.current_music = Some(name.to_string());
        }
        Ok(())
    }
    
    pub fn stop_music(&mut self) {
        self.music_sink.stop();
        self.current_music = None;
    }
}
```

### **Day 18-21: Asset Management**

#### **Asset Loading Pipeline**
```rust
// src/engine/asset/asset_manager.rs
pub struct AssetManager {
    textures: HashMap<String, wgpu::Texture>,
    sounds: HashMap<String, Vec<u8>>,
    meshes: HashMap<String, Mesh>,
    asset_paths: HashMap<String, PathBuf>,
}

impl AssetManager {
    pub async fn load_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
    ) -> Result<()> {
        let image = image::open(path)?;
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();
        
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some(path),
        });
        
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );
        
        self.textures.insert(path.to_string(), texture);
        Ok(())
    }
    
    pub fn load_sound(&mut self, path: &str) -> Result<()> {
        let data = std::fs::read(path)?;
        self.sounds.insert(path.to_string(), data);
        Ok(())
    }
}
```

---

## 🎮 **Week 4: Game Logic & Scene Management**

### **Day 22-24: Game Logic Implementation**

#### **Character Movement System**
```rust
// src/game/systems/character_controller.rs
pub struct CharacterController {
    speed: f32,
    jump_force: f32,
    is_grounded: bool,
}

impl System for CharacterController {
    fn update(&mut self, world: &mut World, dt: f32) {
        let mut query = world.query_mut::<(Position, Velocity, CharacterController)>();
        
        for (entity, (position, velocity, controller)) in query.iter_mut() {
            // Handle horizontal movement
            if controller.is_moving_left {
                velocity.x = -controller.speed;
            } else if controller.is_moving_right {
                velocity.x = controller.speed;
            } else {
                velocity.x *= 0.8; // Friction
            }
            
            // Handle jumping
            if controller.should_jump && controller.is_grounded {
                velocity.y = -controller.jump_force;
                controller.is_grounded = false;
            }
            
            // Apply gravity
            velocity.y += 980.0 * dt; // Gravity
            
            // Update position
            position.x += velocity.x * dt;
            position.y += velocity.y * dt;
        }
    }
}
```

#### **Combat System Implementation**
```rust
// src/game/systems/combat_system.rs
pub struct CombatSystem;

impl System for CombatSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        let mut query = world.query_mut::<(Position, Combat, Attack)>();
        
        for (entity, (position, combat, attack)) in query.iter_mut() {
            // Update attack cooldown
            combat.last_attack_time += dt;
            
            // Check if attack should be executed
            if attack.should_attack && combat.last_attack_time >= combat.attack_cooldown {
                self.execute_attack(world, entity, position, attack);
                combat.last_attack_time = 0.0;
            }
        }
    }
    
    fn execute_attack(&self, world: &mut World, attacker: Entity, position: &Position, attack: &Attack) {
        // Find targets in attack range
        let targets = world.query::<Position>();
        let healths = world.query::<Health>();
        
        for (target_entity, target_position) in targets.iter() {
            if target_entity == attacker {
                continue; // Don't attack self
            }
            
            let distance = ((position.x - target_position.x).powi(2) + 
                           (position.y - target_position.y).powi(2)).sqrt();
            
            if distance <= attack.range {
                // Apply damage
                if let Some(target_health) = healths.get(target_entity) {
                    // Deal damage to target
                    world.get_component_mut::<Health>(target_entity)
                        .unwrap()
                        .current -= attack.damage;
                }
            }
        }
    }
}
```

### **Day 25-28: Scene Management**

#### **Scene System Implementation**
```rust
// src/engine/scene/scene_manager.rs
pub struct SceneManager {
    current_scene: Option<String>,
    scenes: HashMap<String, Scene>,
    scene_transitions: Vec<SceneTransition>,
}

impl SceneManager {
    pub fn load_scene(&mut self, name: &str) -> Result<()> {
        if let Some(scene) = self.scenes.get(name) {
            self.current_scene = Some(name.to_string());
            // Load scene entities into ECS world
            self.load_scene_entities(scene);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Scene not found: {}", name))
        }
    }
    
    pub fn create_scene(&mut self, name: &str, entities: Vec<SceneEntity>) -> Result<()> {
        let scene = Scene {
            name: name.to_string(),
            entities,
            background_color: [0.1, 0.2, 0.3, 1.0],
        };
        
        self.scenes.insert(name.to_string(), scene);
        Ok(())
    }
    
    fn load_scene_entities(&self, scene: &Scene) {
        // Load scene entities into ECS world
        for entity in &scene.entities {
            // Create entity and add components
            // This would be implemented based on your ECS system
        }
    }
}
```

---

## 📊 **Phase 2 Testing Strategy**

### **Unit Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ecs_query_system() {
        let mut world = World::new();
        let entity = world.create_entity();
        world.add_component(entity, Position { x: 10.0, y: 20.0 });
        
        let query = world.query::<Position>();
        let positions: Vec<_> = query.iter().collect();
        
        assert_eq!(positions.len(), 1);
        assert_eq!(positions[0].1.x, 10.0);
        assert_eq!(positions[0].1.y, 20.0);
    }
    
    #[test]
    fn test_input_mapping() {
        let mut input_manager = InputManager::new();
        
        // Test default mappings
        assert!(input_manager.is_action_pressed(InputAction::MoveLeft));
        assert!(input_manager.is_action_pressed(InputAction::MoveRight));
    }
}
```

### **Integration Testing**
```rust
#[test]
fn test_physics_ecs_integration() {
    let mut world = World::new();
    let mut physics_system = PhysicsSystem::new();
    
    // Create entity with physics body
    let entity = world.create_entity();
    world.add_component(entity, Position { x: 0.0, y: 0.0 });
    world.add_component(entity, Velocity { x: 10.0, y: 0.0 });
    
    // Step physics
    physics_system.step(&mut world, 0.016); // 60 FPS
    
    // Check position updated
    let position = world.get_component::<Position>(entity).unwrap();
    assert!(position.x > 0.0);
}
```

---

## 🎯 **Phase 2 Success Metrics**

### **Technical Benchmarks**
- **Rendering**: 60+ FPS at 1080p with 100+ sprites
- **Input Latency**: <16ms response time
- **Physics**: 60Hz physics simulation
- **Audio**: <100ms audio latency
- **Memory**: <100MB RAM usage
- **Loading**: <2s scene load time

### **Gameplay Benchmarks**
- **Movement**: Smooth character movement in all directions
- **Combat**: Responsive attack system with hit detection
- **Progression**: Character leveling and skill progression
- **Scenes**: Seamless level transitions
- **Co-op**: Two-player local multiplayer

---

## 🚀 **Phase 2 Conclusion**

Phase 2 will transform the 2D Brawler Engine from a solid foundation into a fully functional game engine. By implementing real rendering, input processing, physics simulation, audio playback, and game logic, the engine will be ready for content creation and gameplay development.

**Key deliverables:**
- ✅ Real WGPU sprite rendering
- ✅ Functional input system
- ✅ Physics simulation with collision
- ✅ Audio playback system
- ✅ Asset loading pipeline
- ✅ Character movement and combat
- ✅ Scene management system

**Phase 2 will create a production-ready 2D game engine!** 🎮
