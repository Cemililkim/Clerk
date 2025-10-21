use crate::crypto::{derive_key, generate_salt, hash_password, verify_password};
use crate::database::Database;
use crate::commands::database::DatabaseState;
use crate::keychain::KeychainManager;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

/// Response for vault creation
#[derive(Serialize)]
pub struct CreateVaultResponse {
    pub success: bool,
    pub message: String,
}

/// Response for vault unlock
#[derive(Serialize)]
pub struct UnlockVaultResponse {
    pub success: bool,
    pub message: String,
}

/// Creates a new encrypted vault
/// 
/// # Arguments
/// * `password` - Master password for the vault
/// * `vault_path` - Optional custom path for vault file (defaults to app data dir)
#[tauri::command]
pub async fn create_vault(
    app: AppHandle,
    state: State<'_, DatabaseState>,
    password: String,
) -> Result<CreateVaultResponse, String> {
    // Validate password strength
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long".to_string());
    }

    // Get vault path in app data directory
    let vault_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    // Create directory if it doesn't exist
    std::fs::create_dir_all(&vault_dir)
        .map_err(|e| format!("Failed to create vault directory: {}", e))?;
    
    let vault_path = vault_dir.join("vault.clerk");
    
    // Check if vault already exists
    if vault_path.exists() {
        return Err("Vault already exists. Please unlock it instead.".to_string());
    }

    // Generate salt for key derivation
    let salt = generate_salt()
        .map_err(|_| "Failed to generate salt".to_string())?;
    
    // Hash password for verification
    let password_hash = hash_password(&password)
        .map_err(|e| format!("Failed to hash password: {}", e))?;

    // Create vault metadata
    let metadata = VaultMetadata {
        version: 1,
        salt: salt.to_vec(),
        password_hash,
        created_at: chrono::Utc::now().timestamp(),
    };

    // Save metadata to file
    let metadata_json = serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
    
    std::fs::write(&vault_path, metadata_json)
        .map_err(|e| format!("Failed to write vault file: {}", e))?;

    // Derive encryption key
    let encryption_key = derive_key(&password, &salt)
        .map_err(|e| format!("Failed to derive key: {}", e))?;

    // Initialize database
    let db_path = vault_dir.join("vault.db");
    let db = Database::new(&db_path)
        .map_err(|e| format!("Failed to create database: {}", e))?;
    
    // Run migrations
    db.initialize()
        .map_err(|e| format!("Failed to initialize database: {}", e))?;

    // Store database and encryption key in app state
    {
        let mut db_guard = state.db.lock().map_err(|e| e.to_string())?;
        *db_guard = Some(db);
    }
    
    {
        let mut key_guard = state.encryption_key.lock().map_err(|e| e.to_string())?;
        *key_guard = Some(encryption_key);
    }

    Ok(CreateVaultResponse {
        success: true,
        message: format!("Vault created successfully at: {}", vault_path.display()),
    })
}

/// Unlocks an existing vault
#[tauri::command]
pub async fn unlock_vault(
    app: AppHandle,
    state: State<'_, DatabaseState>,
    password: String,
    remember_me: Option<bool>,
) -> Result<UnlockVaultResponse, String> {
    // Get vault path
    let vault_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let vault_path = vault_dir.join("vault.clerk");
    
    // Check if vault exists
    if !vault_path.exists() {
        return Err("Vault does not exist. Please create one first.".to_string());
    }

    // Read vault metadata
    let metadata_json = std::fs::read_to_string(&vault_path)
        .map_err(|e| format!("Failed to read vault file: {}", e))?;
    
    let metadata: VaultMetadata = serde_json::from_str(&metadata_json)
        .map_err(|e| format!("Failed to parse vault metadata: {}", e))?;

    // Verify password
    let is_valid = verify_password(&password, &metadata.password_hash)
        .map_err(|e| format!("Failed to verify password: {}", e))?;

    if !is_valid {
        return Err("Invalid password".to_string());
    }

    // Derive encryption key
    let salt_array: [u8; 16] = metadata.salt
        .try_into()
        .map_err(|_| "Invalid salt length".to_string())?;
    
    let encryption_key = derive_key(&password, &salt_array)
        .map_err(|e| format!("Failed to derive key: {}", e))?;

    // Initialize database
    let db_path = vault_dir.join("vault.db");
    let db = Database::new(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    // Run migrations
    db.initialize()
        .map_err(|e| format!("Failed to initialize database: {}", e))?;

    // Store database and encryption key in app state
    {
        let mut db_guard = state.db.lock().map_err(|e| e.to_string())?;
        *db_guard = Some(db);
    }
    
    {
        let mut key_guard = state.encryption_key.lock().map_err(|e| e.to_string())?;
        *key_guard = Some(encryption_key);
    }

    // If remember_me is true, save key to OS keychain
    if remember_me.unwrap_or(false) {
        let keychain = KeychainManager::new();
        keychain.save_key(&encryption_key)
            .map_err(|e| format!("Failed to save key to keychain: {}", e))?;
    }

    Ok(UnlockVaultResponse {
        success: true,
        message: "Vault unlocked successfully".to_string(),
    })
}

/// Attempts to automatically unlock vault using stored key from OS keychain
/// 
/// Called on app startup to provide seamless experience when "Remember Me" was used.
#[tauri::command]
pub async fn auto_unlock(
    app: AppHandle,
    state: State<'_, DatabaseState>,
) -> Result<UnlockVaultResponse, String> {
    // Get vault path
    let vault_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let vault_path = vault_dir.join("vault.clerk");
    
    // Check if vault exists
    if !vault_path.exists() {
        return Err("Vault does not exist".to_string());
    }

    // Try to get encryption key from keychain
    let keychain = KeychainManager::new();
    let encryption_key = match keychain.get_key() {
        Ok(Some(key)) => key,
        Ok(None) => {
            return Err("No stored key found. Please unlock manually.".to_string());
        },
        Err(e) => {
            return Err(format!("Failed to access keychain: {}", e));
        }
    };

    // Initialize database with stored key
    let db_path = vault_dir.join("vault.db");
    let db = Database::new(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    // Run migrations
    db.initialize()
        .map_err(|e| format!("Failed to initialize database: {}", e))?;

    // Store database and encryption key in app state
    {
        let mut db_guard = state.db.lock().map_err(|e| e.to_string())?;
        *db_guard = Some(db);
    }
    
    {
        let mut key_guard = state.encryption_key.lock().map_err(|e| e.to_string())?;
        *key_guard = Some(encryption_key);
    }

    Ok(UnlockVaultResponse {
        success: true,
        message: "Vault auto-unlocked successfully".to_string(),
    })
}

/// Checks if a vault exists
#[tauri::command]
pub async fn check_vault_exists(app: AppHandle) -> Result<bool, String> {
    let vault_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let vault_path = vault_dir.join("vault.clerk");
    Ok(vault_path.exists())
}

/// Locks the vault by clearing in-memory state and keychain
#[tauri::command]
pub async fn lock_vault(
    state: State<'_, DatabaseState>,
) -> Result<(), String> {
    // Clear database and encryption key from app state
    {
        let mut db_guard = state.db.lock().map_err(|e| e.to_string())?;
        *db_guard = None;
    }
    
    {
        let mut key_guard = state.encryption_key.lock().map_err(|e| e.to_string())?;
        *key_guard = None;
    }

    // Delete stored key from OS keychain for security
    let keychain = KeychainManager::new();
    keychain.delete_key()
        .map_err(|e| format!("Failed to clear keychain: {}", e))?;

    Ok(())
}

/// Get the configured lock timeout in minutes (0 = disabled)
#[tauri::command]
pub async fn get_lock_timeout(
    state: State<'_, DatabaseState>,
) -> Result<i64, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref()
        .ok_or("Database not initialized. Please unlock vault first.")?;

    let timeout: i64 = db.connection()
        .query_row(
            "SELECT COALESCE(lock_timeout_minutes, 0) FROM vault_metadata WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .map_err(|e| format!("Failed to get lock timeout: {}", e))?;

    Ok(timeout)
}

/// Set the lock timeout in minutes (0 = disabled, max 1440 = 24 hours)
#[tauri::command]
pub async fn set_lock_timeout(
    state: State<'_, DatabaseState>,
    timeout_minutes: i64,
) -> Result<(), String> {
    // Validate timeout value
    if !(0..=1440).contains(&timeout_minutes) {
        return Err("Timeout must be between 0 (disabled) and 1440 minutes (24 hours)".to_string());
    }

    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref()
        .ok_or("Database not initialized. Please unlock vault first.")?;

    db.connection().execute(
        "UPDATE vault_metadata SET lock_timeout_minutes = ?1, last_modified = ?2 WHERE id = 1",
        [timeout_minutes, chrono::Utc::now().timestamp()],
    )
    .map_err(|e| format!("Failed to set lock timeout: {}", e))?;

    Ok(())
}

/// Vault metadata structure
#[derive(Serialize, Deserialize)]
struct VaultMetadata {
    version: u32,
    salt: Vec<u8>,
    password_hash: String,
    created_at: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_validation() {
        // Password too short should fail
        assert!(validate_password_strength("short").is_err());
        
        // Valid password should pass
        assert!(validate_password_strength("LongPassword123!").is_ok());
    }

    fn validate_password_strength(password: &str) -> Result<(), String> {
        if password.len() < 8 {
            return Err("Password too short".to_string());
        }
        Ok(())
    }
}
