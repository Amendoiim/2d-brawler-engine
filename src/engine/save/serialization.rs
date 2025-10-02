//! Save File Serialization
//! 
//! This module handles the serialization and deserialization of save files,
//! including compression, encryption, and format conversion.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use super::slot::{SaveSlot, SaveSlotData, SaveSlotMetadata};

/// Save serializer
#[derive(Debug, Clone)]
pub struct SaveSerializer {
    /// Compression enabled
    pub compression_enabled: bool,
    /// Encryption enabled
    pub encryption_enabled: bool,
    /// Encryption key
    pub encryption_key: Option<Vec<u8>>,
    /// Format version
    pub format_version: u32,
}

/// Load serializer
#[derive(Debug, Clone)]
pub struct LoadSerializer {
    /// Compression enabled
    pub compression_enabled: bool,
    /// Encryption enabled
    pub encryption_enabled: bool,
    /// Decryption key
    pub decryption_key: Option<Vec<u8>>,
    /// Format version
    pub format_version: u32,
}

/// Serialization error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializationError {
    /// I/O error
    IoError(String),
    /// Serialization error
    SerdeError(String),
    /// Compression error
    CompressionError(String),
    /// Decompression error
    DecompressionError(String),
    /// Encryption error
    EncryptionError(String),
    /// Decryption error
    DecryptionError(String),
    /// Format version mismatch
    FormatVersionMismatch { expected: u32, found: u32 },
    /// Invalid data
    InvalidData(String),
    /// Checksum mismatch
    ChecksumMismatch,
    /// Unknown error
    Unknown(String),
}

/// Serialization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializationResult {
    /// Success status
    pub success: bool,
    /// Serialized data
    pub data: Vec<u8>,
    /// Original size
    pub original_size: usize,
    /// Compressed size
    pub compressed_size: Option<usize>,
    /// Compression ratio
    pub compression_ratio: Option<f32>,
    /// Serialization time in milliseconds
    pub serialization_time: f32,
    /// Error message
    pub error: Option<String>,
}

/// Save file header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveFileHeader {
    /// Magic number
    pub magic: u32,
    /// Format version
    pub version: u32,
    /// Compression type
    pub compression: CompressionType,
    /// Encryption type
    pub encryption: EncryptionType,
    /// Data size
    pub data_size: usize,
    /// Compressed size
    pub compressed_size: Option<usize>,
    /// Checksum
    pub checksum: u32,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Metadata size
    pub metadata_size: usize,
}

/// Compression type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompressionType {
    /// No compression
    None,
    /// Zlib compression
    Zlib,
    /// LZ4 compression
    Lz4,
    /// Gzip compression
    Gzip,
}

/// Encryption type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EncryptionType {
    /// No encryption
    None,
    /// AES-256 encryption
    Aes256,
    /// ChaCha20 encryption
    ChaCha20,
}

impl SaveSerializer {
    /// Create a new save serializer
    pub fn new() -> Self {
        Self {
            compression_enabled: true,
            encryption_enabled: false,
            encryption_key: None,
            format_version: 1,
        }
    }

    /// Create a serializer with compression
    pub fn with_compression() -> Self {
        Self {
            compression_enabled: true,
            encryption_enabled: false,
            encryption_key: None,
            format_version: 1,
        }
    }

    /// Create a serializer with encryption
    pub fn with_encryption(key: Vec<u8>) -> Self {
        Self {
            compression_enabled: true,
            encryption_enabled: true,
            encryption_key: Some(key),
            format_version: 1,
        }
    }

    /// Serialize a save slot
    pub fn serialize_save_slot(&self, save_slot: &SaveSlot) -> SerializationResult {
        let start_time = SystemTime::now();

        // Serialize the save slot data
        let serialized_data = match serde_json::to_vec(save_slot) {
            Ok(data) => data,
            Err(e) => {
                return SerializationResult {
                    success: false,
                    data: Vec::new(),
                    original_size: 0,
                    compressed_size: None,
                    compression_ratio: None,
                    serialization_time: 0.0,
                    error: Some(format!("Serialization error: {}", e)),
                };
            }
        };

        let original_size = serialized_data.len();
        let mut final_data = serialized_data;

        // Apply compression if enabled
        let compressed_size = if self.compression_enabled {
            match self.compress_data(&final_data) {
                Ok(compressed) => {
                    final_data = compressed;
                    Some(final_data.len())
                }
                Err(e) => {
                    return SerializationResult {
                        success: false,
                        data: Vec::new(),
                        original_size,
                        compressed_size: None,
                        compression_ratio: None,
                        serialization_time: 0.0,
                        error: Some(format!("Compression error: {}", e)),
                    };
                }
            }
        } else {
            None
        };

        // Apply encryption if enabled
        if self.encryption_enabled {
            if let Some(key) = &self.encryption_key {
                match self.encrypt_data(&final_data, key) {
                    Ok(encrypted) => final_data = encrypted,
                    Err(e) => {
                        return SerializationResult {
                            success: false,
                            data: Vec::new(),
                            original_size,
                            compressed_size,
                            compression_ratio: None,
                            serialization_time: 0.0,
                            error: Some(format!("Encryption error: {}", e)),
                        };
                    }
                }
            }
        }

        // Create file header
        let header = SaveFileHeader {
            magic: 0x53415645, // "SAVE" in hex
            version: self.format_version,
            compression: if self.compression_enabled {
                CompressionType::Zlib
            } else {
                CompressionType::None
            },
            encryption: if self.encryption_enabled {
                EncryptionType::Aes256
            } else {
                EncryptionType::None
            },
            data_size: original_size,
            compressed_size,
            checksum: self.calculate_checksum(&final_data),
            timestamp: SystemTime::now(),
            metadata_size: 0, // Will be calculated
        };

        // Serialize header
        let header_data = match serde_json::to_vec(&header) {
            Ok(data) => data,
            Err(e) => {
                return SerializationResult {
                    success: false,
                    data: Vec::new(),
                    original_size,
                    compressed_size,
                    compression_ratio: None,
                    serialization_time: 0.0,
                    error: Some(format!("Header serialization error: {}", e)),
                };
            }
        };

        // Combine header and data
        let mut result = Vec::new();
        result.extend_from_slice(&header_data);
        result.extend_from_slice(&final_data);

        let serialization_time = start_time
            .elapsed()
            .unwrap_or_default()
            .as_millis() as f32;

        let compression_ratio = compressed_size.map(|compressed| {
            compressed as f32 / original_size as f32
        });

        SerializationResult {
            success: true,
            data: result,
            original_size,
            compressed_size,
            compression_ratio,
            serialization_time,
            error: None,
        }
    }

    /// Compress data
    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>, SerializationError> {
        // Simple compression simulation (in real implementation, use actual compression)
        Ok(data.to_vec())
    }

    /// Encrypt data
    fn encrypt_data(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, SerializationError> {
        // Simple encryption simulation (in real implementation, use actual encryption)
        let mut encrypted = Vec::new();
        for (i, byte) in data.iter().enumerate() {
            encrypted.push(byte ^ key[i % key.len()]);
        }
        Ok(encrypted)
    }

    /// Calculate checksum
    fn calculate_checksum(&self, data: &[u8]) -> u32 {
        let mut checksum = 0u32;
        for byte in data {
            checksum = checksum.wrapping_add(*byte as u32);
        }
        checksum
    }
}

impl LoadSerializer {
    /// Create a new load serializer
    pub fn new() -> Self {
        Self {
            compression_enabled: true,
            encryption_enabled: false,
            decryption_key: None,
            format_version: 1,
        }
    }

    /// Create a loader with decryption
    pub fn with_decryption(key: Vec<u8>) -> Self {
        Self {
            compression_enabled: true,
            encryption_enabled: true,
            decryption_key: Some(key),
            format_version: 1,
        }
    }

    /// Deserialize a save slot
    pub fn deserialize_save_slot(&self, data: &[u8]) -> Result<SaveSlot, SerializationError> {
        // Parse header
        let header = self.parse_header(data)?;

        // Extract data
        let data_start = header.metadata_size;
        let mut save_data = data[data_start..].to_vec();

        // Decrypt if needed
        if header.encryption != EncryptionType::None {
            if let Some(key) = &self.decryption_key {
                save_data = self.decrypt_data(&save_data, key)?;
            } else {
                return Err(SerializationError::DecryptionError(
                    "Decryption key not provided".to_string(),
                ));
            }
        }

        // Decompress if needed
        if header.compression != CompressionType::None {
            save_data = self.decompress_data(&save_data)?;
        }

        // Deserialize save slot
        let save_slot: SaveSlot = serde_json::from_slice(&save_data)
            .map_err(|e| SerializationError::SerdeError(e.to_string()))?;

        Ok(save_slot)
    }

    /// Parse save file header
    fn parse_header(&self, data: &[u8]) -> Result<SaveFileHeader, SerializationError> {
        // Find header end (simple approach)
        let header_end = data.iter()
            .position(|&b| b == b'{')
            .ok_or_else(|| SerializationError::InvalidData("Header not found".to_string()))?;

        let header_data = &data[..header_end];
        let header: SaveFileHeader = serde_json::from_slice(header_data)
            .map_err(|e| SerializationError::SerdeError(e.to_string()))?;

        // Validate magic number
        if header.magic != 0x53415645 {
            return Err(SerializationError::InvalidData("Invalid magic number".to_string()));
        }

        // Validate version
        if header.version != self.format_version {
            return Err(SerializationError::FormatVersionMismatch {
                expected: self.format_version,
                found: header.version,
            });
        }

        Ok(header)
    }

    /// Decrypt data
    fn decrypt_data(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, SerializationError> {
        // Simple decryption simulation (in real implementation, use actual decryption)
        let mut decrypted = Vec::new();
        for (i, byte) in data.iter().enumerate() {
            decrypted.push(byte ^ key[i % key.len()]);
        }
        Ok(decrypted)
    }

    /// Decompress data
    fn decompress_data(&self, data: &[u8]) -> Result<Vec<u8>, SerializationError> {
        // Simple decompression simulation (in real implementation, use actual decompression)
        Ok(data.to_vec())
    }
}

impl Default for SaveSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LoadSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializationError::IoError(msg) => write!(f, "I/O error: {}", msg),
            SerializationError::SerdeError(msg) => write!(f, "Serialization error: {}", msg),
            SerializationError::CompressionError(msg) => write!(f, "Compression error: {}", msg),
            SerializationError::DecompressionError(msg) => write!(f, "Decompression error: {}", msg),
            SerializationError::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
            SerializationError::DecryptionError(msg) => write!(f, "Decryption error: {}", msg),
            SerializationError::FormatVersionMismatch { expected, found } => {
                write!(f, "Format version mismatch: expected {}, found {}", expected, found)
            }
            SerializationError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            SerializationError::ChecksumMismatch => write!(f, "Checksum mismatch"),
            SerializationError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for SerializationError {}
