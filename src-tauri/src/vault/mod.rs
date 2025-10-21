// Vault module - handles vault operations
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub struct VaultManager;

impl VaultManager {
    pub fn new() -> Self {
        Self
    }
}

impl Default for VaultManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Vault metadata structure
#[derive(Serialize, Deserialize)]
pub struct VaultMetadata {
    pub version: u32,
    pub salt: Vec<u8>,
    pub password_hash: String,
    pub created_at: i64,
}

/// Get the default vault directory
pub fn get_vault_directory() -> Result<PathBuf, String> {
    // Use the same directory as the GUI app (Tauri's app data directory)
    // On Windows: %APPDATA%/com.clerk.app
    // On macOS: ~/Library/Application Support/com.clerk.app
    // On Linux: ~/.config/com.clerk.app
    let app_data_dir = dirs::config_dir()
        .ok_or("Failed to get config directory")?;
    
    let vault_dir = app_data_dir.join("com.clerk.app");
    
    // Create directory if it doesn't exist
    std::fs::create_dir_all(&vault_dir)
        .map_err(|e| format!("Failed to create vault directory: {}", e))?;
    
    Ok(vault_dir)
}
