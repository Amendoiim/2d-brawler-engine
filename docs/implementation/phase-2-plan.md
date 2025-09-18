# Phase 2 Implementation Plan
## 2D Brawler Engine - Feature Implementation & Functionality

**Status:** 🚧 **PLANNING**  
**Duration:** 4-6 weeks  
**Goal:** Transform foundation into functional game engine with real features

---

## 📋 **Phase 2 Overview**

Phase 2 focuses on implementing actual functionality on top of the solid foundation established in Phase 1. This phase transforms placeholder implementations into working systems that can render, process input, simulate physics, and run actual game logic.

---

## 🎯 **Phase 2 Objectives**

### **Primary Goals**
1. **Real Rendering** - Implement actual WGPU sprite rendering
2. **Functional ECS** - Fix borrowing issues and implement real system execution
3. **Input Processing** - Connect input events to game actions
4. **Physics Simulation** - Implement actual physics with collision detection
5. **Audio Playback** - Load and play actual sound files
6. **Asset Loading** - Implement file loading and texture display
7. **Game Logic** - Implement character movement and basic combat
8. **Scene Management** - Real scene loading and transitions

### **Success Criteria**
- ✅ Engine renders actual sprites on screen
- ✅ Input controls character movement
- ✅ Physics simulation works with collision detection
- ✅ Audio plays sound effects and music
- ✅ Assets load from files and display correctly
- ✅ Basic game logic runs (movement, combat, progression)
- ✅ Scene system loads and switches between levels
- ✅ Performance targets met (60+ FPS at 1080p minimum), should target 120 FPS in 4K.

---

## 🏗️ **Phase 2 Implementation Roadmap**

### **Week 1: Core Systems Foundation**
**Focus:** Fix ECS and implement real rendering

#### **Day 1-2: ECS System Completion**
- [ ] **Fix Borrowing Issues**
  - Implement proper query system with borrowing workarounds
  - Create component iteration without conflicts
  - Add efficient entity management
  - Implement system execution pipeline

- [ ] **ECS Query System**
  - Create `Query<T>` for single component queries
  - Implement `Query<(T1, T2)>` for multiple components
  - Add `QueryMut<T>` for mutable component access
  - Create `QueryFilter` for conditional queries

#### **Day 3-4: Real WGPU Rendering**
- [ ] **Fix Raw Window Handle Issues**
  - Update to compatible WGPU version
  - Fix window handle trait implementations
  - Resolve surface creation issues
  - Test on multiple platforms

- [ ] **Implement Sprite Rendering**
  - Create actual WGPU render pipeline
  - Implement sprite batching system
  - Add texture loading and binding
  - Create vertex buffer management

#### **Day 5-7: Rendering Pipeline**
- [ ] **Render Pipeline Completion**
  - Implement proper render passes
  - Add depth testing and blending
  - Create uniform buffer management
  - Add camera system integration

- [ ] **Texture System**
  - Implement texture loading from files
  - Add texture atlas support
  - Create texture caching system
  - Add texture filtering and sampling

### **Week 2: Input & Physics Integration**
**Focus:** Connect input to game actions and implement physics

#### **Day 8-10: Input System Implementation**
- [ ] **Input Event Processing**
  - Connect keyboard events to game actions
  - Implement mouse input handling
  - Add gamepad input processing
  - Create input mapping system

- [ ] **Character Movement**
  - Implement player character movement
  - Add movement constraints and boundaries
  - Create movement animation system
  - Add collision-based movement

#### **Day 11-14: Physics Integration**
- [ ] **Physics Simulation**
  - Implement actual physics step execution
  - Add rigid body creation and management
  - Create collision detection system
  - Add physics constraints and joints

- [ ] **Physics-ECS Integration**
  - Sync physics bodies with ECS components
  - Implement physics-based movement
  - Add collision response system
  - Create physics event handling

### **Week 3: Audio & Asset Management**
**Focus:** Implement audio playback and asset loading

#### **Day 15-17: Audio System Implementation**
- [ ] **Sound File Loading**
  - Implement audio file loading (WAV, OGG, MP3)
  - Add audio asset management
  - Create sound effect system
  - Implement background music playback

- [ ] **Audio Integration**
  - Connect audio to game events
  - Add spatial audio support
  - Implement audio volume control
  - Create audio event system

#### **Day 18-21: Asset Management**
- [ ] **Asset Loading Pipeline**
  - Implement texture loading from files
  - Add mesh and model loading
  - Create asset caching system
  - Add asset hot-reloading

- [ ] **Asset Integration**
  - Connect assets to rendering system
  - Implement asset streaming
  - Add asset validation
  - Create asset dependency management

### **Week 4: Game Logic & Scene Management**
**Focus:** Implement actual game logic and scene system

#### **Day 22-24: Game Logic Implementation**
- [ ] **Character Systems**
  - Implement character movement logic
  - Add character animation system
  - Create character state management
  - Add character interaction system

- [ ] **Combat System**
  - Implement attack logic and damage
  - Add combat animation system
  - Create hit detection and response
  - Add special move system

#### **Day 25-28: Scene Management**
- [ ] **Scene System Implementation**
  - Implement actual scene loading
  - Add scene transition system
  - Create scene serialization
  - Add scene management UI

- [ ] **Level System**
  - Implement level generation
  - Add enemy spawning system
  - Create obstacle and environment system
  - Add level progression logic

---

## 🔧 **Technical Implementation Details**

### **1. ECS System Completion**

#### **Query System Implementation**
```rust
pub struct Query<'a, T: Component> {
    world: &'a World,
    component_type: TypeId,
    entities: Vec<Entity>,
}

impl<'a, T: Component> Query<'a, T> {
    pub fn new(world: &'a World) -> Self {
        // Implementation for single component queries
    }
    
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        // Iterator over entities with component T
    }
}

pub struct QueryMut<'a, T: Component> {
    world: &'a mut World,
    component_type: TypeId,
    entities: Vec<Entity>,
}

impl<'a, T: Component> QueryMut<'a, T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        // Mutable iterator over entities with component T
    }
}
```

#### **System Execution Pipeline**
```rust
pub trait System {
    fn update(&mut self, world: &mut World, dt: f32);
    fn name(&self) -> &str;
    fn priority(&self) -> i32 { 0 }
}

pub struct SystemManager {
    systems: Vec<Box<dyn System>>,
    execution_order: Vec<usize>,
}

impl SystemManager {
    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        // Add system with proper ordering
    }
    
    pub fn update(&mut self, world: &mut World, dt: f32) {
        // Execute systems in proper order
    }
}
```

### **2. Real WGPU Rendering**

#### **Render Pipeline Implementation**
```rust
pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    sprite_renderer: SpriteRenderer,
    camera: Camera,
}

impl Renderer {
    pub fn new(window: &Window) -> Result<Self> {
        // Initialize WGPU context
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window)? };
        
        // Create device and queue
        let (device, queue) = pollster::block_on(instance.request_device(
            &wgpu::DeviceDescriptor::default(),
            None,
        ))?;
        
        // Configure surface
        let surface_caps = surface.get_capabilities(&device);
        let surface_format = surface_caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        
        // Create render pipeline
        let render_pipeline = Self::create_render_pipeline(&device, &config);
        
        Ok(Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            sprite_renderer: SpriteRenderer::new(&device, &config),
            camera: Camera::new(),
        })
    }
    
    pub fn render(&mut self, world: &World) -> Result<()> {
        // Actual WGPU rendering implementation
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
            });
            
            render_pass.set_pipeline(&self.render_pipeline);
            
            // Render sprites
            self.sprite_renderer.render_sprites(&mut render_pass, world, &self.camera);
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
}
```

#### **Sprite Rendering System**
```rust
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
        for (entity, sprite) in sprites.iter() {
            if let Some(position) = positions.get(entity) {
                // Add sprite vertices to buffer
                vertices.extend_from_slice(&Self::create_sprite_vertices(
                    position.x, position.y,
                    sprite.width, sprite.height,
                    sprite.color,
                ));
            }
        }
        
        // Render sprites
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_bind_group(1, &self.texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..vertices.len() as u32);
    }
}
```

### **3. Input System Implementation**

#### **Input Event Processing**
```rust
pub struct InputManager {
    keyboard_state: KeyboardState,
    mouse_state: MouseState,
    gamepad_state: GamepadState,
    input_mappings: HashMap<InputAction, Vec<InputBinding>>,
}

pub enum InputAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Attack,
    Jump,
    Special,
    Pause,
}

pub struct InputBinding {
    pub input_type: InputType,
    pub key: Option<KeyCode>,
    pub mouse_button: Option<MouseButton>,
    pub gamepad_button: Option<gilrs::Button>,
}

impl InputManager {
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
        // Check if any binding for this action is pressed
        if let Some(bindings) = self.input_mappings.get(&action) {
            bindings.iter().any(|binding| self.is_binding_pressed(binding))
        } else {
            false
        }
    }
}
```

#### **Character Movement System**
```rust
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

pub struct PlayerInputSystem {
    input_manager: InputManager,
}

impl System for PlayerInputSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        let mut query = world.query_mut::<Velocity>();
        
        for (entity, velocity) in query.iter_mut() {
            // Process input and update velocity
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
```

### **4. Physics Integration**

#### **Physics-ECS Synchronization**
```rust
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

### **5. Audio System Implementation**

#### **Sound File Loading and Playback**
```rust
pub struct AudioEngine {
    _stream: OutputStream,
    sink: Sink,
    sounds: HashMap<String, Vec<u8>>,
    music_sink: Sink,
    current_music: Option<String>,
}

impl AudioEngine {
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
}
```

### **6. Asset Management System**

#### **Asset Loading Pipeline**
```rust
pub struct AssetManager {
    textures: HashMap<String, wgpu::Texture>,
    sounds: HashMap<String, Vec<u8>>,
    meshes: HashMap<String, Mesh>,
    asset_paths: HashMap<String, PathBuf>,
}

impl AssetManager {
    pub async fn load_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, path: &str) -> Result<()> {
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
}
```

---

## 🎮 **Game Logic Implementation**

### **1. Character Movement System**
```rust
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

### **2. Combat System Implementation**
```rust
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

---

## 📊 **Phase 2 Success Metrics**

### **Technical Requirements**
- [ ] **Rendering**: Engine displays actual sprites on screen
- [ ] **Input**: Character responds to keyboard/gamepad input
- [ ] **Physics**: Objects fall, collide, and respond to forces
- [ ] **Audio**: Sound effects and music play correctly
- [ ] **Assets**: Textures and sounds load from files
- [ ] **Performance**: 60+ FPS at 1080p resolution
- [ ] **Stability**: No crashes during extended gameplay

### **Gameplay Requirements**
- [ ] **Movement**: Character moves smoothly in all directions
- [ ] **Combat**: Basic attack system with damage and hit detection
- [ ] **Progression**: Character can gain experience and level up
- [ ] **Scenes**: Multiple levels can be loaded and played
- [ ] **Co-op**: Two players can play simultaneously (local)

### **Quality Requirements**
- [ ] **Code Quality**: Clean, maintainable, well-documented code
- [ ] **Performance**: Smooth gameplay with consistent frame rate
- [ ] **Stability**: Robust error handling and recovery
- [ ] **Extensibility**: Easy to add new features and content

---

## 🚀 **Phase 2 Deliverables**

### **Week 1 Deliverables**
- ✅ Working ECS query system
- ✅ Real WGPU sprite rendering
- ✅ Basic texture loading and display
- ✅ Fixed raw window handle issues

### **Week 2 Deliverables**
- ✅ Functional input system
- ✅ Character movement with input
- ✅ Physics simulation with collision
- ✅ Physics-ECS synchronization

### **Week 3 Deliverables**
- ✅ Audio file loading and playback
- ✅ Sound effects and music system
- ✅ Asset management pipeline
- ✅ Texture and sound caching

### **Week 4 Deliverables**
- ✅ Character movement and animation
- ✅ Basic combat system
- ✅ Scene loading and transitions
- ✅ Level generation and enemy spawning

---

## 🎯 **Phase 2 Success Criteria**

### **Must Have (Critical)**
1. **Real Rendering** - Engine displays actual graphics
2. **Input Response** - Character moves with input
3. **Physics Simulation** - Objects interact with physics
4. **Audio Playback** - Sounds and music play
5. **Asset Loading** - Files load and display correctly

### **Should Have (Important)**
1. **Combat System** - Basic attack and damage
2. **Scene Management** - Level loading and transitions
3. **Character Progression** - Experience and leveling
4. **Performance** - 60+ FPS at 1080p
5. **Stability** - No crashes during gameplay

### **Nice to Have (Optional)**
1. **Animation System** - Character and sprite animations
2. **Particle Effects** - Visual effects for combat
3. **UI System** - Health bars, menus, HUD
4. **Save System** - Game state persistence
5. **Configuration** - Settings and options

---

## 🔄 **Phase 2 Risk Mitigation**

### **Technical Risks**
1. **WGPU Compatibility** - Test on multiple platforms early
2. **ECS Borrowing** - Implement proper query system design
3. **Performance Issues** - Profile and optimize early
4. **Asset Loading** - Implement robust error handling
5. **Physics Integration** - Test collision detection thoroughly

### **Mitigation Strategies**
1. **Early Testing** - Test each system as it's implemented
2. **Incremental Development** - Build and test small features
3. **Performance Monitoring** - Profile code regularly
4. **Error Handling** - Implement comprehensive error recovery
5. **Documentation** - Document all APIs and systems

---

## 📈 **Phase 2 Timeline**

### **Week 1: Core Systems** (Days 1-7)
- ECS completion and rendering implementation
- Focus on getting basic graphics working

### **Week 2: Input & Physics** (Days 8-14)
- Input processing and physics integration
- Focus on interactive gameplay

### **Week 3: Audio & Assets** (Days 15-21)
- Audio system and asset management
- Focus on content and polish

### **Week 4: Game Logic** (Days 22-28)
- Character systems and combat
- Focus on actual gameplay

---

## 🎮 **Phase 2 Conclusion**

Phase 2 will transform the 2D Brawler Engine from a solid foundation into a functional game engine capable of running actual gameplay. By the end of Phase 2, the engine will have:

- 🎨 **Real Graphics** - Actual sprite rendering and display
- 🎮 **Interactive Input** - Responsive character control
- ⚡ **Physics Simulation** - Realistic object interaction
- 🔊 **Audio System** - Sound effects and music
- 📁 **Asset Management** - File loading and content
- 🎯 **Game Logic** - Character movement and combat
- 🏗️ **Scene System** - Level loading and management

**Phase 2 will create a fully functional 2D game engine ready for content creation and gameplay development!** 🚀
