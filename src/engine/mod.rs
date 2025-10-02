//! Core engine systems and modules

pub mod renderer;
pub mod physics;
pub mod audio;
pub mod input;
pub mod ecs;
pub mod scene;
pub mod asset;
pub mod animation;
pub mod particles;
pub mod level;
pub mod localization;
pub mod visual;
pub mod tutorial;
pub mod achievements;
pub mod save;
pub mod completion;
pub mod performance;
pub mod ui;
pub mod sound_test;
pub mod dynamic_music;
pub mod sfx_pitch;
pub mod music_transition;

use anyhow::Result;
use winit::{
    event::WindowEvent,
    window::{Window, WindowId},
};

use crate::platform::Platform;

/// Main engine struct that coordinates all systems
pub struct Engine {
    platform: Platform,
    renderer: renderer::Renderer,
    physics: physics::PhysicsWorld,
    audio: audio::AudioManager,
    input: input::InputManager,
    ecs: ecs::World,
    scene: scene::SceneManager,
    asset_manager: asset::AssetManager,
    animation: animation::AnimationSystem,
    particles: particles::ParticleSystem,
    level_generator: level::LevelGenerator,
    localization: localization::manager::GameLocalizationManager,
    visual_effects: visual::VisualEffectsManager,
    tutorial: tutorial::TutorialManager,
    achievements: achievements::AchievementManager,
    save: save::SaveManager,
    completion: completion::CompletionTracker,
    pub performance: performance::PerformanceMonitor,
    pub ui: ui::UIManager,
    pub sound_test: sound_test::SoundTestManager,
    pub dynamic_music: dynamic_music::DynamicMusicManager,
    pub sfx_pitch: sfx_pitch::SFXPitchManager,
    pub music_transition: music_transition::MusicTransitionManager,
}

impl Engine {
    /// Create a new engine instance
    pub fn new(window: Window) -> Result<Self> {
        let window_id = window.id();
        
        // Initialize platform abstraction
        let platform = Platform::new(window)?;
        
        // Initialize core systems
        let renderer = renderer::Renderer::new(&platform)?;
        let physics = physics::PhysicsWorld::new()?;
        let audio = audio::AudioManager::new(audio::AudioConfig::default());
        let input = input::InputManager::new(window_id)?;
        let ecs = ecs::World::new();
        let scene = scene::SceneManager::new();
        let asset_manager = asset::AssetManager::new()?;
        let animation = animation::AnimationSystem::new();
        let particles = particles::ParticleSystem::new();
        let level_generator = level::LevelGenerator::new();
        let localization = localization::manager::GameLocalizationManager::new();
        let visual_effects = visual::VisualEffectsManager::new();
        let tutorial = tutorial::TutorialManager::new();
        let achievements = achievements::AchievementManager::new();
        let save = save::SaveManager::new(save::SaveConfig::default());
        let completion = completion::CompletionTracker::new(completion::CompletionConfig::default());
        let performance = performance::PerformanceMonitor::new();
        let ui = ui::UIManager::new(ui::UIConfig::default());
        let audio = audio::AudioManager::new(audio::AudioConfig::default());
        let sound_test = sound_test::SoundTestManager::new(sound_test::SoundTestConfig::default());
        let dynamic_music = dynamic_music::DynamicMusicManager::new(dynamic_music::DynamicMusicConfig::default());
        let sfx_pitch = sfx_pitch::SFXPitchManager::new(sfx_pitch::SFXPitchConfig::default());
        let music_transition = music_transition::MusicTransitionManager::new(music_transition::MusicTransitionConfig::default());

        Ok(Self {
            platform,
            renderer,
            physics,
            audio,
            input,
            ecs,
            scene,
            asset_manager,
            animation,
            particles,
            level_generator,
            localization,
            visual_effects,
            tutorial,
            achievements,
            save,
            completion,
            performance,
            ui,
            sound_test,
            dynamic_music,
            sfx_pitch,
            music_transition,
        })
    }

    /// Get reference to the window
    pub fn window(&self) -> &Window {
        self.platform.window()
    }

    /// Handle window events
    pub fn handle_window_event(&mut self, event: WindowEvent) {
        self.input.handle_window_event(event);
    }

    /// Resize the rendering surface
    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(size);
    }

    /// Main render function
    pub fn render(&mut self) -> Result<()> {
        // Update timing
        self.platform.update_timing();
        
        // Update systems
        self.update_internal()?;
        
        // Render frame
        self.renderer.render(&self.ecs)?;
        
        Ok(())
    }

    /// Get mutable reference to ECS world
    pub fn ecs_mut(&mut self) -> &mut ecs::World {
        &mut self.ecs
    }

    /// Get mutable reference to audio engine
    pub fn audio_mut(&mut self) -> &mut audio::AudioManager {
        &mut self.audio
    }

    /// Get mutable reference to asset manager
    pub fn asset_manager_mut(&mut self) -> &mut asset::AssetManager {
        &mut self.asset_manager
    }

    /// Get mutable reference to scene manager
    pub fn scene_mut(&mut self) -> &mut scene::SceneManager {
        &mut self.scene
    }

    /// Get mutable reference to animation system
    pub fn animation_mut(&mut self) -> &mut animation::AnimationSystem {
        &mut self.animation
    }

    /// Get mutable reference to particle system
    pub fn particles_mut(&mut self) -> &mut particles::ParticleSystem {
        &mut self.particles
    }

    /// Get mutable reference to level generator
    pub fn level_generator_mut(&mut self) -> &mut level::LevelGenerator {
        &mut self.level_generator
    }

    /// Get mutable reference to localization manager
    pub fn localization_mut(&mut self) -> &mut localization::manager::GameLocalizationManager {
        &mut self.localization
    }

    /// Get reference to localization manager
    pub fn localization(&self) -> &localization::manager::GameLocalizationManager {
        &self.localization
    }

    /// Get mutable reference to visual effects manager
    pub fn visual_effects_mut(&mut self) -> &mut visual::VisualEffectsManager {
        &mut self.visual_effects
    }

    /// Get reference to visual effects manager
    pub fn visual_effects(&self) -> &visual::VisualEffectsManager {
        &self.visual_effects
    }

    /// Get mutable reference to tutorial manager
    pub fn tutorial_mut(&mut self) -> &mut tutorial::TutorialManager {
        &mut self.tutorial
    }

    /// Get reference to tutorial manager
    pub fn tutorial(&self) -> &tutorial::TutorialManager {
        &self.tutorial
    }

    /// Get mutable reference to achievement manager
    pub fn achievements_mut(&mut self) -> &mut achievements::AchievementManager {
        &mut self.achievements
    }

    /// Get reference to achievement manager
    pub fn achievements(&self) -> &achievements::AchievementManager {
        &self.achievements
    }

    /// Get mutable reference to save manager
    pub fn save_mut(&mut self) -> &mut save::SaveManager {
        &mut self.save
    }

    /// Get reference to save manager
    pub fn save(&self) -> &save::SaveManager {
        &self.save
    }

    /// Get mutable reference to completion tracker
    pub fn completion_mut(&mut self) -> &mut completion::CompletionTracker {
        &mut self.completion
    }

    /// Get reference to completion tracker
    pub fn completion(&self) -> &completion::CompletionTracker {
        &self.completion
    }

    /// Get mutable reference to performance monitor
    pub fn performance_mut(&mut self) -> &mut performance::PerformanceMonitor {
        &mut self.performance
    }

    /// Get reference to performance monitor
    pub fn performance(&self) -> &performance::PerformanceMonitor {
        &self.performance
    }

    /// Get mutable reference to UI manager
    pub fn ui_mut(&mut self) -> &mut ui::UIManager {
        &mut self.ui
    }

    /// Get reference to UI manager
    pub fn ui(&self) -> &ui::UIManager {
        &self.ui
    }

    /// Get mutable reference to sound test manager
    pub fn sound_test_mut(&mut self) -> &mut sound_test::SoundTestManager {
        &mut self.sound_test
    }

    /// Get reference to sound test manager
    pub fn sound_test(&self) -> &sound_test::SoundTestManager {
        &self.sound_test
    }

    /// Get mutable reference to dynamic music manager
    pub fn dynamic_music_mut(&mut self) -> &mut dynamic_music::DynamicMusicManager {
        &mut self.dynamic_music
    }

    /// Get reference to dynamic music manager
    pub fn dynamic_music(&self) -> &dynamic_music::DynamicMusicManager {
        &self.dynamic_music
    }

    /// Get mutable reference to SFX pitch manager
    pub fn sfx_pitch_mut(&mut self) -> &mut sfx_pitch::SFXPitchManager {
        &mut self.sfx_pitch
    }

    /// Get reference to SFX pitch manager
    pub fn sfx_pitch(&self) -> &sfx_pitch::SFXPitchManager {
        &self.sfx_pitch
    }

    /// Get mutable reference to music transition manager
    pub fn music_transition_mut(&mut self) -> &mut music_transition::MusicTransitionManager {
        &mut self.music_transition
    }

    /// Get reference to music transition manager
    pub fn music_transition(&self) -> &music_transition::MusicTransitionManager {
        &self.music_transition
    }


    /// Load a scene
    pub fn load_scene(&mut self, name: &str) -> Result<()> {
        self.scene.load_scene(name, &mut self.ecs)
    }

    /// Update all engine systems with given delta time
    pub fn update(&mut self, dt: f32) -> Result<()> {
        // Update input
        self.input.update();
        
        // Update physics
        self.physics.step(dt);
        
        // Update audio
        self.audio.update(dt);
        
        // Update asset manager
        self.asset_manager.update(dt)?;
        
        // Update ECS systems
        self.ecs.update(dt);
        
        // Update animation system
        self.animation.update(&mut self.ecs, dt);
        
        // Update particle system (commented out for now)
        // self.particles.update(&self.ecs, dt);
        
        // Update visual effects
        self.visual_effects.update(0.016); // 60 FPS delta time
        
        // Update tutorial system
        let context = tutorial::step::TutorialContext {
            input_state: tutorial::step::InputState {
                pressed_keys: Vec::new(),
                mouse_position: (0.0, 0.0),
                mouse_buttons: Vec::new(),
                modifiers: Vec::new(),
            },
            player_state: tutorial::step::PlayerState {
                position: (0.0, 0.0),
                level: 1,
                health: 100.0,
                mana: 100.0,
                stamina: 100.0,
                used_items: std::collections::HashMap::new(),
                defeated_enemies: std::collections::HashMap::new(),
                current_level: "tutorial".to_string(),
            },
            tutorial_state: tutorial::step::TutorialState {
                current_tutorial: None,
                current_step: None,
                step_start_time: 0.0,
                step_duration: 0.0,
                completed_steps: Vec::new(),
            },
            game_state: tutorial::step::GameState {
                current_time: 0.0,
                paused: false,
                current_scene: "main".to_string(),
                available_tutorials: Vec::new(),
            },
        };
        self.tutorial.update(dt, &context);
        
        // Update achievement system
        self.achievements.update(dt);
        
        // Update save system
        self.save.update(dt).unwrap_or_else(|e| {
            eprintln!("Save system update error: {}", e);
        });
        
        // Update completion system
        self.completion.update(dt).unwrap_or_else(|e| {
            eprintln!("Completion system update error: {}", e);
        });
        
        // Update performance system
        self.performance.update(dt).unwrap_or_else(|e| {
            eprintln!("Performance system update error: {}", e);
        });
        
        // Update UI system
        self.ui.update(dt).unwrap_or_else(|e| {
            eprintln!("UI system update error: {}", e);
        });
        
        // Update audio system
        self.audio.update(dt).unwrap_or_else(|e| {
            eprintln!("Audio system update error: {}", e);
        });
        
        // Update sound test system
        self.sound_test.update(dt).unwrap_or_else(|e| {
            eprintln!("Sound test system update error: {}", e);
        });
        
        // Update dynamic music system
        self.dynamic_music.update(dt, &[]).unwrap_or_else(|e| {
            eprintln!("Dynamic music system update error: {}", e);
        });
        
        // Update SFX pitch system
        self.sfx_pitch.update(dt, &[]).unwrap_or_else(|e| {
            eprintln!("SFX pitch system update error: {}", e);
        });
        
        // Update music transition system
        self.music_transition.update(dt).unwrap_or_else(|e| {
            eprintln!("Music transition system update error: {}", e);
        });
        
        // Update scene
        self.scene.update(dt, &mut self.ecs);
        
        Ok(())
    }

    /// Update all engine systems (internal method)
    fn update_internal(&mut self) -> Result<()> {
        let dt = self.platform.delta_time();
        self.update(dt)
    }
}
