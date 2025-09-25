//! Asset management system for loading and caching resources

use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, Duration};

/// Asset types supported by the engine
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum AssetType {
    Texture,
    Sound,
    Mesh,
    Font,
    Shader,
    Data,
}

/// Asset loading priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssetPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Asset loading state
#[derive(Debug, Clone, PartialEq)]
pub enum AssetState {
    NotLoaded,
    Loading,
    Loaded,
    Failed,
    Unloading,
}

/// Asset metadata
#[derive(Debug, Clone)]
pub struct AssetInfo {
    pub name: String,
    pub path: PathBuf,
    pub asset_type: AssetType,
    pub size: usize,
    pub last_modified: SystemTime,
    pub loaded: bool,
    pub state: AssetState,
    pub priority: AssetPriority,
    pub dependencies: Vec<String>,
    pub load_time: Duration,
    pub access_count: u32,
    pub last_accessed: SystemTime,
}

/// Asset validation result
#[derive(Debug, Clone)]
pub struct AssetValidation {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Asset streaming configuration
#[derive(Debug, Clone)]
pub struct StreamingConfig {
    pub max_memory_usage: usize,
    pub streaming_threads: usize,
    pub prefetch_distance: f32,
    pub cache_timeout: Duration,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            max_memory_usage: 512 * 1024 * 1024, // 512MB
            streaming_threads: 2,
            prefetch_distance: 1000.0,
            cache_timeout: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Asset manager for loading and caching game assets
pub struct AssetManager {
    textures: HashMap<String, u32>,
    sounds: HashMap<String, Vec<u8>>,
    meshes: HashMap<String, Vec<u8>>,
    fonts: HashMap<String, Vec<u8>>,
    shaders: HashMap<String, String>,
    data: HashMap<String, Vec<u8>>,
    asset_info: HashMap<String, AssetInfo>,
    asset_paths: HashMap<String, PathBuf>,
    loading_queue: VecDeque<String>,
    streaming_config: StreamingConfig,
    memory_usage: usize,
    asset_cache: HashMap<String, SystemTime>,
}

impl AssetManager {
    /// Create a new asset manager
    pub fn new() -> Result<Self> {
        Ok(Self {
            textures: HashMap::new(),
            sounds: HashMap::new(),
            meshes: HashMap::new(),
            fonts: HashMap::new(),
            shaders: HashMap::new(),
            data: HashMap::new(),
            asset_info: HashMap::new(),
            asset_paths: HashMap::new(),
            loading_queue: VecDeque::new(),
            streaming_config: StreamingConfig::default(),
            memory_usage: 0,
            asset_cache: HashMap::new(),
        })
    }

    /// Set streaming configuration
    pub fn set_streaming_config(&mut self, config: StreamingConfig) {
        self.streaming_config = config;
        log::info!("Updated streaming configuration: {:?}", self.streaming_config);
    }

    /// Load a texture asset with validation
    pub fn load_texture(&mut self, name: &str, path: &Path) -> Result<()> {
        let start_time = SystemTime::now();
        
        // Validate asset before loading
        let validation = self.validate_asset(path, &AssetType::Texture)?;
        if !validation.valid {
            return Err(anyhow::anyhow!("Asset validation failed: {:?}", validation.errors));
        }

        let data = fs::read(path)?;
        let metadata = fs::metadata(path)?;
        
        // For now, we'll just store a placeholder ID
        // In a real implementation, this would load the texture into GPU memory
        let texture_id = self.textures.len() as u32 + 1;
        self.textures.insert(name.to_string(), texture_id);
        
        let load_time = start_time.elapsed().unwrap_or_default();
        self.memory_usage += data.len();
        
        self.asset_info.insert(name.to_string(), AssetInfo {
            name: name.to_string(),
            path: path.to_path_buf(),
            asset_type: AssetType::Texture,
            size: data.len(),
            last_modified: metadata.modified()?,
            loaded: true,
            state: AssetState::Loaded,
            priority: AssetPriority::Normal,
            dependencies: Vec::new(),
            load_time,
            access_count: 0,
            last_accessed: SystemTime::now(),
        });
        
        self.asset_paths.insert(name.to_string(), path.to_path_buf());
        self.asset_cache.insert(name.to_string(), SystemTime::now());
        
        log::info!("Loaded texture: {} from {:?} ({} bytes, {}ms)", 
                  name, path, data.len(), load_time.as_millis());
        Ok(())
    }

    /// Validate an asset before loading
    pub fn validate_asset(&self, path: &Path, asset_type: &AssetType) -> Result<AssetValidation> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if file exists
        if !path.exists() {
            errors.push(format!("File does not exist: {:?}", path));
            return Ok(AssetValidation { valid: false, errors, warnings });
        }

        // Check file size
        if let Ok(metadata) = fs::metadata(path) {
            let size = metadata.len();
            match asset_type {
                AssetType::Texture => {
                    if size > 100 * 1024 * 1024 { // 100MB
                        warnings.push("Texture file is very large, consider optimization".to_string());
                    }
                }
                AssetType::Sound => {
                    if size > 50 * 1024 * 1024 { // 50MB
                        warnings.push("Sound file is very large, consider compression".to_string());
                    }
                }
                AssetType::Mesh => {
                    if size > 200 * 1024 * 1024 { // 200MB
                        warnings.push("Mesh file is very large, consider LOD optimization".to_string());
                    }
                }
                _ => {}
            }
        }

        // Check file extension
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            match asset_type {
                AssetType::Texture => {
                    if !["png", "jpg", "jpeg", "bmp", "tga", "dds"].contains(&ext.as_str()) {
                        warnings.push(format!("Unusual texture format: {}", ext));
                    }
                }
                AssetType::Sound => {
                    if !["wav", "ogg", "mp3", "flac"].contains(&ext.as_str()) {
                        warnings.push(format!("Unusual sound format: {}", ext));
                    }
                }
                AssetType::Mesh => {
                    if !["obj", "fbx", "gltf", "glb", "dae"].contains(&ext.as_str()) {
                        warnings.push(format!("Unusual mesh format: {}", ext));
                    }
                }
                AssetType::Font => {
                    if !["ttf", "otf", "woff", "woff2"].contains(&ext.as_str()) {
                        warnings.push(format!("Unusual font format: {}", ext));
                    }
                }
                AssetType::Shader => {
                    if !["wgsl", "glsl", "hlsl", "vert", "frag"].contains(&ext.as_str()) {
                        warnings.push(format!("Unusual shader format: {}", ext));
                    }
                }
                _ => {}
            }
        }

        Ok(AssetValidation { 
            valid: errors.is_empty(), 
            errors, 
            warnings 
        })
    }

    /// Load a sound asset with validation
    pub fn load_sound(&mut self, name: &str, path: &Path) -> Result<()> {
        let start_time = SystemTime::now();
        
        let validation = self.validate_asset(path, &AssetType::Sound)?;
        if !validation.valid {
            return Err(anyhow::anyhow!("Asset validation failed: {:?}", validation.errors));
        }

        let data = fs::read(path)?;
        let metadata = fs::metadata(path)?;
        
        self.sounds.insert(name.to_string(), data.clone());
        
        let load_time = start_time.elapsed().unwrap_or_default();
        self.memory_usage += data.len();
        
        self.asset_info.insert(name.to_string(), AssetInfo {
            name: name.to_string(),
            path: path.to_path_buf(),
            asset_type: AssetType::Sound,
            size: data.len(),
            last_modified: metadata.modified()?,
            loaded: true,
            state: AssetState::Loaded,
            priority: AssetPriority::Normal,
            dependencies: Vec::new(),
            load_time,
            access_count: 0,
            last_accessed: SystemTime::now(),
        });
        
        self.asset_paths.insert(name.to_string(), path.to_path_buf());
        self.asset_cache.insert(name.to_string(), SystemTime::now());
        
        log::info!("Loaded sound: {} from {:?} ({} bytes, {}ms)", 
                  name, path, data.len(), load_time.as_millis());
        Ok(())
    }

    /// Load a mesh asset
    pub fn load_mesh(&mut self, name: &str, path: &Path) -> Result<()> {
        let data = fs::read(path)?;
        let metadata = fs::metadata(path)?;
        
        self.meshes.insert(name.to_string(), data.clone());
        
        self.asset_info.insert(name.to_string(), AssetInfo {
            name: name.to_string(),
            path: path.to_path_buf(),
            asset_type: AssetType::Mesh,
            size: data.len(),
            last_modified: metadata.modified()?,
            loaded: true,
            state: AssetState::Loaded,
            priority: AssetPriority::Normal,
            dependencies: Vec::new(),
            load_time: Duration::default(),
            access_count: 0,
            last_accessed: SystemTime::now(),
        });
        
        self.asset_paths.insert(name.to_string(), path.to_path_buf());
        log::info!("Loaded mesh: {} from {:?} ({} bytes)", name, path, data.len());
        Ok(())
    }

    /// Load a font asset
    pub fn load_font(&mut self, name: &str, path: &Path) -> Result<()> {
        let data = fs::read(path)?;
        let metadata = fs::metadata(path)?;
        
        self.fonts.insert(name.to_string(), data.clone());
        
        self.asset_info.insert(name.to_string(), AssetInfo {
            name: name.to_string(),
            path: path.to_path_buf(),
            asset_type: AssetType::Font,
            size: data.len(),
            last_modified: metadata.modified()?,
            loaded: true,
            state: AssetState::Loaded,
            priority: AssetPriority::Normal,
            dependencies: Vec::new(),
            load_time: Duration::default(),
            access_count: 0,
            last_accessed: SystemTime::now(),
        });
        
        self.asset_paths.insert(name.to_string(), path.to_path_buf());
        log::info!("Loaded font: {} from {:?} ({} bytes)", name, path, data.len());
        Ok(())
    }

    /// Load a shader asset
    pub fn load_shader(&mut self, name: &str, path: &Path) -> Result<()> {
        let data = fs::read_to_string(path)?;
        let metadata = fs::metadata(path)?;
        
        self.shaders.insert(name.to_string(), data.clone());
        
        self.asset_info.insert(name.to_string(), AssetInfo {
            name: name.to_string(),
            path: path.to_path_buf(),
            asset_type: AssetType::Shader,
            size: data.len(),
            last_modified: metadata.modified()?,
            loaded: true,
            state: AssetState::Loaded,
            priority: AssetPriority::High,
            dependencies: Vec::new(),
            load_time: Duration::default(),
            access_count: 0,
            last_accessed: SystemTime::now(),
        });
        
        self.asset_paths.insert(name.to_string(), path.to_path_buf());
        log::info!("Loaded shader: {} from {:?} ({} bytes)", name, path, data.len());
        Ok(())
    }

    /// Load generic data asset
    pub fn load_data(&mut self, name: &str, path: &Path) -> Result<()> {
        let data = fs::read(path)?;
        let metadata = fs::metadata(path)?;
        
        self.data.insert(name.to_string(), data.clone());
        
        self.asset_info.insert(name.to_string(), AssetInfo {
            name: name.to_string(),
            path: path.to_path_buf(),
            asset_type: AssetType::Data,
            size: data.len(),
            last_modified: metadata.modified()?,
            loaded: true,
            state: AssetState::Loaded,
            priority: AssetPriority::Normal,
            dependencies: Vec::new(),
            load_time: Duration::default(),
            access_count: 0,
            last_accessed: SystemTime::now(),
        });
        
        self.asset_paths.insert(name.to_string(), path.to_path_buf());
        log::info!("Loaded data: {} from {:?} ({} bytes)", name, path, data.len());
        Ok(())
    }

    /// Get texture ID with access tracking
    pub fn get_texture(&mut self, name: &str) -> Option<u32> {
        if let Some(info) = self.asset_info.get_mut(name) {
            info.access_count += 1;
            info.last_accessed = SystemTime::now();
        }
        self.textures.get(name).copied()
    }

    /// Get sound data with access tracking
    pub fn get_sound(&mut self, name: &str) -> Option<&Vec<u8>> {
        if let Some(info) = self.asset_info.get_mut(name) {
            info.access_count += 1;
            info.last_accessed = SystemTime::now();
        }
        self.sounds.get(name)
    }

    /// Get mesh data with access tracking
    pub fn get_mesh(&mut self, name: &str) -> Option<&Vec<u8>> {
        if let Some(info) = self.asset_info.get_mut(name) {
            info.access_count += 1;
            info.last_accessed = SystemTime::now();
        }
        self.meshes.get(name)
    }

    /// Get font data with access tracking
    pub fn get_font(&mut self, name: &str) -> Option<&Vec<u8>> {
        if let Some(info) = self.asset_info.get_mut(name) {
            info.access_count += 1;
            info.last_accessed = SystemTime::now();
        }
        self.fonts.get(name)
    }

    /// Get shader data with access tracking
    pub fn get_shader(&mut self, name: &str) -> Option<&String> {
        if let Some(info) = self.asset_info.get_mut(name) {
            info.access_count += 1;
            info.last_accessed = SystemTime::now();
        }
        self.shaders.get(name)
    }

    /// Get data asset with access tracking
    pub fn get_data(&mut self, name: &str) -> Option<&Vec<u8>> {
        if let Some(info) = self.asset_info.get_mut(name) {
            info.access_count += 1;
            info.last_accessed = SystemTime::now();
        }
        self.data.get(name)
    }

    /// Get asset information
    pub fn get_asset_info(&self, name: &str) -> Option<&AssetInfo> {
        self.asset_info.get(name)
    }

    /// Check if an asset is loaded
    pub fn is_asset_loaded(&self, name: &str) -> bool {
        self.asset_info.get(name).map(|info| info.loaded).unwrap_or(false)
    }

    /// Get all loaded asset names
    pub fn get_loaded_assets(&self) -> Vec<String> {
        self.asset_info.keys().cloned().collect()
    }

    /// Get total memory usage
    pub fn get_memory_usage(&self) -> usize {
        self.asset_info.values().map(|info| info.size).sum()
    }

    /// Get asset count by type
    pub fn get_asset_count_by_type(&self, asset_type: AssetType) -> usize {
        self.asset_info.values()
            .filter(|info| info.asset_type == asset_type)
            .count()
    }

    /// Queue an asset for loading
    pub fn queue_asset(&mut self, name: &str, priority: AssetPriority) {
        if !self.asset_info.contains_key(name) {
            self.asset_info.insert(name.to_string(), AssetInfo {
                name: name.to_string(),
                path: PathBuf::new(),
                asset_type: AssetType::Data,
                size: 0,
                last_modified: SystemTime::now(),
                loaded: false,
                state: AssetState::NotLoaded,
                priority,
                dependencies: Vec::new(),
                load_time: Duration::default(),
                access_count: 0,
                last_accessed: SystemTime::now(),
            });
        }

        if !self.loading_queue.contains(&name.to_string()) {
            self.loading_queue.push_back(name.to_string());
            log::debug!("Queued asset for loading: {} (priority: {:?})", name, priority);
        }
    }

    /// Process loading queue
    pub fn process_loading_queue(&mut self) -> Result<()> {
        let mut loaded_count = 0;
        let max_loads_per_frame = 5; // Limit loading per frame for performance
        let mut to_load = Vec::new();

        // Collect assets to load this frame
        while let Some(asset_name) = self.loading_queue.pop_front() {
            if loaded_count >= max_loads_per_frame {
                self.loading_queue.push_front(asset_name);
                break;
            }

            if let Some(asset_info) = self.asset_info.get(&asset_name) {
                if asset_info.state == AssetState::NotLoaded {
                    if let Some(path) = self.asset_paths.get(&asset_name) {
                        to_load.push((asset_name.clone(), asset_info.asset_type.clone(), path.clone()));
                        loaded_count += 1;
                    }
                }
            }
        }

        // Load the collected assets
        for (asset_name, asset_type, path) in to_load {
            match asset_type {
                AssetType::Texture => {
                    if let Err(e) = self.load_texture(&asset_name, &path) {
                        log::error!("Failed to load texture {}: {}", asset_name, e);
                    }
                }
                AssetType::Sound => {
                    if let Err(e) = self.load_sound(&asset_name, &path) {
                        log::error!("Failed to load sound {}: {}", asset_name, e);
                    }
                }
                AssetType::Mesh => {
                    if let Err(e) = self.load_mesh(&asset_name, &path) {
                        log::error!("Failed to load mesh {}: {}", asset_name, e);
                    }
                }
                AssetType::Font => {
                    if let Err(e) = self.load_font(&asset_name, &path) {
                        log::error!("Failed to load font {}: {}", asset_name, e);
                    }
                }
                AssetType::Shader => {
                    if let Err(e) = self.load_shader(&asset_name, &path) {
                        log::error!("Failed to load shader {}: {}", asset_name, e);
                    }
                }
                AssetType::Data => {
                    if let Err(e) = self.load_data(&asset_name, &path) {
                        log::error!("Failed to load data {}: {}", asset_name, e);
                    }
                }
            }
        }

        Ok(())
    }

    /// Clean up unused assets
    pub fn cleanup_unused_assets(&mut self) {
        let now = SystemTime::now();
        let mut to_remove = Vec::new();

        for (name, info) in &self.asset_info {
            if info.loaded && info.priority != AssetPriority::Critical {
                if let Ok(duration) = now.duration_since(info.last_accessed) {
                    if duration > self.streaming_config.cache_timeout {
                        to_remove.push(name.clone());
                    }
                }
            }
        }

        for name in to_remove {
            self.unload_asset(&name);
        }
    }

    /// Unload an asset
    pub fn unload_asset(&mut self, name: &str) {
        if let Some(info) = self.asset_info.get(name) {
            self.memory_usage = self.memory_usage.saturating_sub(info.size);
        }

        self.textures.remove(name);
        self.sounds.remove(name);
        self.meshes.remove(name);
        self.fonts.remove(name);
        self.shaders.remove(name);
        self.data.remove(name);
        self.asset_cache.remove(name);

        if let Some(info) = self.asset_info.get_mut(name) {
            info.loaded = false;
            info.state = AssetState::NotLoaded;
        }

        log::debug!("Unloaded asset: {}", name);
    }

    /// Get memory usage percentage
    pub fn get_memory_usage_percentage(&self) -> f32 {
        (self.memory_usage as f32 / self.streaming_config.max_memory_usage as f32) * 100.0
    }

    /// Get loading queue status
    pub fn get_loading_queue_status(&self) -> (usize, Vec<String>) {
        (self.loading_queue.len(), self.loading_queue.iter().cloned().collect())
    }

    /// Get asset statistics
    pub fn get_asset_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("total_assets".to_string(), self.asset_info.len());
        stats.insert("loaded_assets".to_string(), self.asset_info.values().filter(|info| info.loaded).count());
        stats.insert("memory_usage_mb".to_string(), self.memory_usage / (1024 * 1024));
        stats.insert("loading_queue".to_string(), self.loading_queue.len());
        
        for asset_type in [AssetType::Texture, AssetType::Sound, AssetType::Mesh, AssetType::Font, AssetType::Shader, AssetType::Data] {
            let count = self.get_asset_count_by_type(asset_type.clone());
            stats.insert(format!("{:?}_count", asset_type).to_lowercase(), count);
        }
        
        stats
    }

    /// Create a test texture (colored rectangle)
    pub fn create_test_texture(&mut self, name: &str, width: u32, height: u32, color: [u8; 3]) -> Result<()> {
        let texture_id = self.textures.len() as u32 + 1;
        self.textures.insert(name.to_string(), texture_id);
        
        let size = (width * height * 3) as usize;
        self.memory_usage += size;
        
        self.asset_info.insert(name.to_string(), AssetInfo {
            name: name.to_string(),
            path: PathBuf::from(format!("test_{}.png", name)),
            asset_type: AssetType::Texture,
            size,
            last_modified: SystemTime::now(),
            loaded: true,
            state: AssetState::Loaded,
            priority: AssetPriority::Normal,
            dependencies: Vec::new(),
            load_time: Duration::default(),
            access_count: 0,
            last_accessed: SystemTime::now(),
        });
        
        self.asset_cache.insert(name.to_string(), SystemTime::now());
        
        log::info!("Created test texture: {} ({}x{}, color: {:?})", name, width, height, color);
        Ok(())
    }

    /// Update asset manager (call each frame)
    pub fn update(&mut self, dt: f32) -> Result<()> {
        // Process loading queue
        self.process_loading_queue()?;
        
        // Clean up unused assets if memory usage is high
        if self.get_memory_usage_percentage() > 80.0 {
            self.cleanup_unused_assets();
        }
        
        log::debug!("Asset manager update: {} assets, {}MB memory, {} in queue", 
                   self.asset_info.len(), 
                   self.memory_usage / (1024 * 1024),
                   self.loading_queue.len());
        
        Ok(())
    }
}
