use crate::commands::database::DatabaseState;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{Manager, State};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupMetadata {
    version: String,
    #[serde(alias = "created_at")]
    created_at: String,
    #[serde(alias = "vault_name")]
    vault_name: String,
    #[serde(alias = "project_count")]
    project_count: usize,
    #[serde(alias = "environment_count")]
    environment_count: usize,
    #[serde(alias = "variable_count")]
    variable_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupFile {
    metadata: BackupMetadata,
    #[serde(alias = "vault_data")]
    vault_data: String,      // Base64 encoded vault file
    #[serde(alias = "database_data")]
    database_data: String,   // Base64 encoded database file
}

// Reserved for future use - backup metadata display
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupInfo {
    filename: String,
    metadata: BackupMetadata,
    #[serde(alias = "file_size")]
    file_size: u64,
}

/// Creates a backup of the current vault and database
#[tauri::command]
pub fn create_backup(
    app: tauri::AppHandle,
    state: State<DatabaseState>,
) -> Result<BackupFile, String> {
    // Get vault directory
    let vault_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let vault_path = vault_dir.join("vault.clerk");
    let database_path = vault_dir.join("vault.db");
    
    // Read vault file
    let vault_content = fs::read(&vault_path)
        .map_err(|e| format!("Failed to read vault file: {}", e))?;
    
    // Read database file
    let database_content = fs::read(&database_path)
        .map_err(|e| format!("Failed to read database file: {}", e))?;
    
    // Encode to Base64
    let vault_data = BASE64.encode(&vault_content);
    let database_data = BASE64.encode(&database_content);
    
    // Get statistics from database
    let (project_count, environment_count, variable_count) = {
        let db_lock = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
        
        // If database not initialized, initialize it temporarily to read stats
        if db_lock.is_none() {
            drop(db_lock); // Drop the lock before reinitializing
            
            // Initialize database temporarily
            let db = crate::database::Database::new(&database_path)
                .map_err(|e| format!("Failed to open database: {}", e))?;
            
            let conn = db.connection();
            
            let proj_count: usize = conn.query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))
                .unwrap_or(0);
            let env_count: usize = conn.query_row("SELECT COUNT(*) FROM environments", [], |row| row.get(0))
                .unwrap_or(0);
            let var_count: usize = conn.query_row("SELECT COUNT(*) FROM variables", [], |row| row.get(0))
                .unwrap_or(0);
            
            (proj_count, env_count, var_count)
        } else {
            // Database already initialized, use it
            let db = db_lock.as_ref().unwrap();
            let conn = db.connection();
            
            let proj_count: usize = conn.query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))
                .unwrap_or(0);
            let env_count: usize = conn.query_row("SELECT COUNT(*) FROM environments", [], |row| row.get(0))
                .unwrap_or(0);
            let var_count: usize = conn.query_row("SELECT COUNT(*) FROM variables", [], |row| row.get(0))
                .unwrap_or(0);
            
            (proj_count, env_count, var_count)
        }
    };
    
    // Extract vault name from path
    let vault_name = PathBuf::from(&vault_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    // Create metadata
    let metadata = BackupMetadata {
        version: "1.0.0".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        vault_name,
        project_count,
        environment_count,
        variable_count,
    };
    
    Ok(BackupFile {
        metadata,
        vault_data,
        database_data,
    })
}

/// Restores a backup to the specified vault and database paths
#[tauri::command]
pub fn restore_backup(
    app: tauri::AppHandle,
    backup_json: String,
) -> Result<String, String> {
    // Get vault directory
    let vault_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let target_vault_path = vault_dir.join("vault.clerk");
    let target_database_path = vault_dir.join("vault.db");
    // Parse backup JSON
    let backup: BackupFile = serde_json::from_str(&backup_json)
        .map_err(|e| format!("Invalid backup file format: {}", e))?;
    
    // Validate backup version
    if backup.metadata.version != "1.0.0" {
        return Err(format!(
            "Unsupported backup version: {}. Expected 1.0.0",
            backup.metadata.version
        ));
    }
    
    // Decode Base64 data
    let vault_content = BASE64.decode(&backup.vault_data)
        .map_err(|e| format!("Failed to decode vault data: {}", e))?;
    
    let database_content = BASE64.decode(&backup.database_data)
        .map_err(|e| format!("Failed to decode database data: {}", e))?;
    
    // Create backup of existing files if they exist
    if target_vault_path.exists() {
        let backup_vault = target_vault_path.with_extension("clerk.backup");
        fs::copy(&target_vault_path, &backup_vault)
            .map_err(|e| format!("Failed to backup existing vault: {}", e))?;
    }
    
    if target_database_path.exists() {
        let backup_db = target_database_path.with_extension("db.backup");
        fs::copy(&target_database_path, &backup_db)
            .map_err(|e| format!("Failed to backup existing database: {}", e))?;
    }
    
    // Write restored files
    fs::write(&target_vault_path, vault_content)
        .map_err(|e| format!("Failed to write vault file: {}", e))?;
    
    fs::write(&target_database_path, database_content)
        .map_err(|e| format!("Failed to write database file: {}", e))?;
    
    Ok(format!(
        "Successfully restored backup. Projects: {}, Environments: {}, Variables: {}",
        backup.metadata.project_count,
        backup.metadata.environment_count,
        backup.metadata.variable_count
    ))
}

/// Extracts metadata from a backup file without fully restoring it
#[tauri::command]
pub fn get_backup_info(backup_json: String) -> Result<BackupMetadata, String> {
    let backup: BackupFile = serde_json::from_str(&backup_json)
        .map_err(|e| format!("Invalid backup file format: {}", e))?;
    
    Ok(backup.metadata)
}

/// Validates a backup file structure
#[tauri::command]
pub fn validate_backup_file(backup_json: String) -> Result<bool, String> {
    // Try to parse as BackupFile
    let backup: BackupFile = serde_json::from_str(&backup_json)
        .map_err(|e| format!("Invalid backup file format: {}", e))?;
    
    // Validate version
    if backup.metadata.version != "1.0.0" {
        return Ok(false);
    }
    
    // Validate Base64 data can be decoded
    BASE64.decode(&backup.vault_data)
        .map_err(|_| "Invalid vault data encoding".to_string())?;
    
    BASE64.decode(&backup.database_data)
        .map_err(|_| "Invalid database data encoding".to_string())?;
    
    Ok(true)
}
