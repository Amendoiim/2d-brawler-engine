//! Audio system

use anyhow::Result;
use rodio::{Decoder, OutputStream, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

/// Audio engine for managing sound effects and music
pub struct AudioEngine {
    _stream: OutputStream,
    sink: Sink,
    sounds: HashMap<String, Vec<u8>>,
}

impl AudioEngine {
    /// Create a new audio engine
    pub fn new() -> Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        Ok(Self {
            _stream,
            sink,
            sounds: HashMap::new(),
        })
    }

    /// Load a sound file
    pub fn load_sound(&mut self, name: &str, data: Vec<u8>) {
        self.sounds.insert(name.to_string(), data);
    }

    /// Play a sound effect
    pub fn play_sound(&self, name: &str) -> Result<()> {
        if let Some(sound_data) = self.sounds.get(name) {
            let cursor = std::io::Cursor::new(sound_data.clone());
            let source = Decoder::new(cursor)?;
            self.sink.append(source);
        }
        Ok(())
    }

    /// Play background music
    pub fn play_music(&self, name: &str) -> Result<()> {
        if let Some(sound_data) = self.sounds.get(name) {
            let cursor = std::io::Cursor::new(sound_data.clone());
            let source = Decoder::new(cursor)?;
            self.sink.append(source);
        }
        Ok(())
    }

    /// Set volume (0.0 to 1.0)
    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume.clamp(0.0, 1.0));
    }

    /// Update audio system
    pub fn update(&mut self, _dt: f32) {
        // Audio system updates are handled by the sink
    }
}
