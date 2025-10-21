use rusqlite::Connection;
use chrono::Utc;
use crate::database::{DatabaseError, schema::*};

/// Run all database migrations
pub fn run_migrations(conn: &Connection) -> Result<(), DatabaseError> {
    // Check current version
    let current_version = get_schema_version(conn)?;
    
    if current_version == 0 {
        // Fresh database - run initial migration
        run_initial_migration(conn)?;
    } else {
        // Run incremental migrations for existing databases
        migrate_add_lock_timeout(conn)?;
        migrate_add_audit_log(conn)?;
    }
    
    Ok(())
}

/// Add lock_timeout_minutes column to vault_metadata (for existing databases)
fn migrate_add_lock_timeout(conn: &Connection) -> Result<(), DatabaseError> {
    // Check if column already exists
    let column_exists: bool = conn
        .prepare("SELECT lock_timeout_minutes FROM vault_metadata LIMIT 1")
        .is_ok();
    
    if !column_exists {
        // Add the column with default value 0 (disabled)
        conn.execute(
            "ALTER TABLE vault_metadata ADD COLUMN lock_timeout_minutes INTEGER DEFAULT 0",
            [],
        )
        .map_err(|e| DatabaseError::MigrationError(format!("Failed to add lock_timeout_minutes column: {}", e)))?;
    }
    
    Ok(())
}

/// Add audit_log table (for existing databases)
fn migrate_add_audit_log(conn: &Connection) -> Result<(), DatabaseError> {
    // Check if table already exists
    let table_exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='audit_log'",
            [],
            |row| row.get::<_, i64>(0).map(|count| count > 0),
        )
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    if !table_exists {
        // Create audit_log table
        conn.execute(CREATE_AUDIT_LOG_TABLE, [])
            .map_err(|e| DatabaseError::MigrationError(format!("Failed to create audit_log table: {}", e)))?;
        
        conn.execute(CREATE_AUDIT_LOG_TIMESTAMP_INDEX, [])
            .map_err(|e| DatabaseError::MigrationError(format!("Failed to create audit_log timestamp index: {}", e)))?;
        
        conn.execute(CREATE_AUDIT_LOG_ENTITY_INDEX, [])
            .map_err(|e| DatabaseError::MigrationError(format!("Failed to create audit_log entity index: {}", e)))?;
    }
    
    Ok(())
}

/// Get current schema version from database
fn get_schema_version(conn: &Connection) -> Result<u32, DatabaseError> {
    // Check if vault_metadata table exists
    let table_exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='vault_metadata'",
            [],
            |row| row.get::<_, i64>(0).map(|count| count > 0),
        )
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    if !table_exists {
        return Ok(0); // No schema yet
    }
    
    // Get version from vault_metadata
    let version: u32 = conn
        .query_row(
            "SELECT version FROM vault_metadata WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);
    
    Ok(version)
}

/// Run initial database migration (version 0 -> 1)
fn run_initial_migration(conn: &Connection) -> Result<(), DatabaseError> {
    // Create all tables
    conn.execute(CREATE_VAULT_METADATA_TABLE, [])
        .map_err(|e| DatabaseError::MigrationError(format!("Failed to create vault_metadata table: {}", e)))?;
    
    conn.execute(CREATE_PROJECTS_TABLE, [])
        .map_err(|e| DatabaseError::MigrationError(format!("Failed to create projects table: {}", e)))?;
    
    conn.execute(CREATE_ENVIRONMENTS_TABLE, [])
        .map_err(|e| DatabaseError::MigrationError(format!("Failed to create environments table: {}", e)))?;
    
    conn.execute(CREATE_VARIABLES_TABLE, [])
        .map_err(|e| DatabaseError::MigrationError(format!("Failed to create variables table: {}", e)))?;
    
    // Create indices
    conn.execute(CREATE_PROJECTS_NAME_INDEX, [])
        .map_err(|e| DatabaseError::MigrationError(format!("Failed to create projects name index: {}", e)))?;
    
    conn.execute(CREATE_ENVIRONMENTS_PROJECT_INDEX, [])
        .map_err(|e| DatabaseError::MigrationError(format!("Failed to create environments project index: {}", e)))?;
    
    conn.execute(CREATE_VARIABLES_ENVIRONMENT_INDEX, [])
        .map_err(|e| DatabaseError::MigrationError(format!("Failed to create variables environment index: {}", e)))?;
    
    conn.execute(CREATE_VARIABLES_KEY_INDEX, [])
        .map_err(|e| DatabaseError::MigrationError(format!("Failed to create variables key index: {}", e)))?;
    
    // Create audit_log table
    conn.execute(CREATE_AUDIT_LOG_TABLE, [])
        .map_err(|e| DatabaseError::MigrationError(format!("Failed to create audit_log table: {}", e)))?;
    
    conn.execute(CREATE_AUDIT_LOG_TIMESTAMP_INDEX, [])
        .map_err(|e| DatabaseError::MigrationError(format!("Failed to create audit_log timestamp index: {}", e)))?;
    
    conn.execute(CREATE_AUDIT_LOG_ENTITY_INDEX, [])
        .map_err(|e| DatabaseError::MigrationError(format!("Failed to create audit_log entity index: {}", e)))?;
    
    // Insert initial metadata
    let now = Utc::now().timestamp();
    conn.execute(
        "INSERT INTO vault_metadata (id, version, created_at, last_accessed, last_modified) VALUES (?, ?, ?, ?, ?)",
        [1, SCHEMA_VERSION as i64, now, now, now],
    )
    .map_err(|e| DatabaseError::MigrationError(format!("Failed to insert vault metadata: {}", e)))?;
    
    Ok(())
}

/// Update last_accessed timestamp in vault_metadata
pub fn update_last_accessed(conn: &Connection) -> Result<(), DatabaseError> {
    let now = Utc::now().timestamp();
    conn.execute(
        "UPDATE vault_metadata SET last_accessed = ? WHERE id = 1",
        [now],
    )
    .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    Ok(())
}

/// Update last_modified timestamp in vault_metadata
pub fn update_last_modified(conn: &Connection) -> Result<(), DatabaseError> {
    let now = Utc::now().timestamp();
    conn.execute(
        "UPDATE vault_metadata SET last_modified = ? WHERE id = 1",
        [now],
    )
    .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    
    #[test]
    fn test_initial_migration() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        
        assert_eq!(get_schema_version(&conn).unwrap(), 0);
        
        run_initial_migration(&conn).unwrap();
        
        assert_eq!(get_schema_version(&conn).unwrap(), SCHEMA_VERSION);
    }
    
    #[test]
    fn test_all_tables_created() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        
        run_initial_migration(&conn).unwrap();
        
        // Check all tables exist
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        
        assert!(tables.contains(&"vault_metadata".to_string()));
        assert!(tables.contains(&"projects".to_string()));
        assert!(tables.contains(&"environments".to_string()));
        assert!(tables.contains(&"variables".to_string()));
    }
    
    #[test]
    fn test_update_timestamps() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        run_initial_migration(&conn).unwrap();
        
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        assert!(update_last_accessed(&conn).is_ok());
        assert!(update_last_modified(&conn).is_ok());
    }
}
