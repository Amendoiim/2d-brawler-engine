//! Audio system for sound effects and music

use anyhow::Result;
use rodio::{Decoder, OutputStream, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Audio engine for managing sound effects and music
pub struct AudioEngine {
    _stream: OutputStream,
    sink: Sink,
    music_sink: Sink,
    sounds: HashMap<String, Vec<u8>>,
    volume: f32,
    music_volume: f32,
}

impl AudioEngine {
    /// Create a new audio engine
    pub fn new() -> Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let music_sink = Sink::try_new(&stream_handle)?;

        Ok(Self {
            _stream,
            sink,
            music_sink,
            sounds: HashMap::new(),
            volume: 1.0,
            music_volume: 0.7,
        })
    }

    /// Load a sound file from disk
    pub fn load_sound_file(&mut self, name: &str, path: &Path) -> Result<()> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut data = Vec::new();
        std::io::Read::read_to_end(&mut reader, &mut data)?;
        
        self.sounds.insert(name.to_string(), data);
        log::info!("Loaded sound: {} from {:?}", name, path);
        Ok(())
    }

    /// Load a sound from memory
    pub fn load_sound(&mut self, name: &str, data: Vec<u8>) {
        let data_len = data.len();
        self.sounds.insert(name.to_string(), data);
        log::debug!("Loaded sound: {} ({} bytes)", name, data_len);
    }

    /// Play a sound effect
    pub fn play_sound(&self, name: &str) -> Result<()> {
        if let Some(sound_data) = self.sounds.get(name) {
            let cursor = std::io::Cursor::new(sound_data.clone());
            let source = Decoder::new(cursor)?;
            self.sink.append(source);
            log::debug!("Playing sound: {}", name);
        } else {
            log::warn!("Sound not found: {}", name);
        }
        Ok(())
    }

    /// Play background music
    pub fn play_music(&self, name: &str) -> Result<()> {
        if let Some(sound_data) = self.sounds.get(name) {
            let cursor = std::io::Cursor::new(sound_data.clone());
            let source = Decoder::new(cursor)?;
            self.music_sink.append(source);
            log::info!("Playing music: {}", name);
        } else {
            log::warn!("Music not found: {}", name);
        }
        Ok(())
    }

    /// Stop all music
    pub fn stop_music(&self) {
        self.music_sink.stop();
        log::debug!("Stopped music");
    }

    /// Stop all sound effects
    pub fn stop_sounds(&self) {
        self.sink.stop();
        log::debug!("Stopped sound effects");
    }

    /// Set master volume (0.0 to 1.0)
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        self.sink.set_volume(self.volume);
        log::debug!("Set volume to: {}", self.volume);
    }

    /// Set music volume (0.0 to 1.0)
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
        self.music_sink.set_volume(self.music_volume);
        log::debug!("Set music volume to: {}", self.music_volume);
    }

    /// Check if a sound is loaded
    pub fn has_sound(&self, name: &str) -> bool {
        self.sounds.contains_key(name)
    }

    /// Get loaded sound count
    pub fn sound_count(&self) -> usize {
        self.sounds.len()
    }

    /// Get sound names
    pub fn sound_names(&self) -> Vec<String> {
        self.sounds.keys().cloned().collect()
    }

    /// Create a test sound (sine wave)
    pub fn create_test_sound(&mut self, name: &str, frequency: f32, duration: f32) -> Result<()> {
        let sample_rate = 44100;
        let samples = (sample_rate as f32 * duration) as usize;
        let mut data = Vec::with_capacity(samples * 2);
        
        for i in 0..samples {
            let t = i as f32 / sample_rate as f32;
            let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin() * 0.3;
            let sample_i16 = (sample * i16::MAX as f32) as i16;
            data.extend_from_slice(&sample_i16.to_le_bytes());
            data.extend_from_slice(&sample_i16.to_le_bytes()); // Stereo
        }
        
        self.load_sound(name, data);
        Ok(())
    }

    /// Update audio system
    pub fn update(&mut self, dt: f32) {
        // Update audio system
        // This could include fading, crossfading, etc.
        log::debug!("Audio system update: dt={}, sounds={}", dt, self.sound_count());
    }
}
