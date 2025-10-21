use rusqlite::{Connection, params};
use chrono::Utc;
use serde_json::json;
use crate::database::{DatabaseError, operations::{Variable, VariableDecrypted, audit::log_audit}};
use crate::crypto::encryption;

/// Create a new variable (value must already be encrypted)
pub fn create_variable(conn: &Connection, var: &Variable) -> Result<i64, DatabaseError> {
    conn.execute(
        "INSERT INTO variables (environment_id, key, encrypted_value, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
        params![
            var.environment_id,
            &var.key,
            &var.encrypted_value,
            &var.description,
            var.created_at,
            var.updated_at,
        ],
    )?;
    
    let var_id = conn.last_insert_rowid();
    
    // Log the audit entry
    let _ = log_audit(
        conn,
        "create",
        "variable",
        Some(var_id),
        Some(&var.key),
        Some(json!({
            "environment_id": var.environment_id,
            "description": &var.description,
        })),
    );
    
    Ok(var_id)
}

/// Get a variable by ID (returns encrypted value)
pub fn get_variable(conn: &Connection, id: i64) -> Result<Variable, DatabaseError> {
    let mut stmt = conn.prepare(
        "SELECT id, environment_id, key, encrypted_value, description, created_at, updated_at FROM variables WHERE id = ?"
    )?;
    
    let var = stmt.query_row(params![id], |row| {
        Ok(Variable {
            id: Some(row.get(0)?),
            environment_id: row.get(1)?,
            key: row.get(2)?,
            encrypted_value: row.get(3)?,
            description: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    
    Ok(var)
}

/// Get all variables for an environment (returns encrypted values)
pub fn get_variables_by_environment(conn: &Connection, environment_id: i64) -> Result<Vec<Variable>, DatabaseError> {
    let mut stmt = conn.prepare(
        "SELECT id, environment_id, key, encrypted_value, description, created_at, updated_at FROM variables WHERE environment_id = ? ORDER BY key"
    )?;
    
    let variables = stmt.query_map(params![environment_id], |row| {
        Ok(Variable {
            id: Some(row.get(0)?),
            environment_id: row.get(1)?,
            key: row.get(2)?,
            encrypted_value: row.get(3)?,
            description: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(variables)
}

/// Update a variable (value must already be encrypted)
pub fn update_variable(conn: &Connection, id: i64, var: &Variable) -> Result<(), DatabaseError> {
    let now = Utc::now().timestamp();
    let rows_affected = conn.execute(
        "UPDATE variables SET key = ?, encrypted_value = ?, description = ?, updated_at = ? WHERE id = ?",
        params![&var.key, &var.encrypted_value, &var.description, now, id],
    )?;
    
    if rows_affected == 0 {
        return Err(DatabaseError::NotFound(format!("Variable with id {} not found", id)));
    }
    
    // Log the audit entry
    let _ = log_audit(
        conn,
        "update",
        "variable",
        Some(id),
        Some(&var.key),
        Some(json!({
            "environment_id": var.environment_id,
            "description": &var.description,
        })),
    );
    
    Ok(())
}

/// Delete a variable
pub fn delete_variable(conn: &Connection, id: i64) -> Result<(), DatabaseError> {
    // Get variable key before deleting for audit log
    let var_key: Option<String> = conn.query_row(
        "SELECT key FROM variables WHERE id = ?",
        params![id],
        |row| row.get(0),
    ).ok();
    
    let rows_affected = conn.execute("DELETE FROM variables WHERE id = ?", params![id])?;
    
    if rows_affected == 0 {
        return Err(DatabaseError::NotFound(format!("Variable with id {} not found", id)));
    }
    
    // Log the audit entry
    let _ = log_audit(
        conn,
        "delete",
        "variable",
        Some(id),
        var_key.as_deref(),
        None,
    );
    
    Ok(())
}

/// Check if a variable exists by key within an environment
pub fn variable_exists(conn: &Connection, environment_id: i64, key: &str) -> Result<bool, DatabaseError> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM variables WHERE environment_id = ? AND key = ?",
        params![environment_id, key],
        |row| row.get(0),
    )?;
    
    Ok(count > 0)
}

/// Encrypt and create a variable (high-level helper)
pub fn create_variable_encrypted(
    conn: &Connection,
    environment_id: i64,
    key: String,
    value: String,
    description: Option<String>,
    encryption_key: &[u8; 32],
) -> Result<i64, DatabaseError> {
    // Create AAD (Additional Authenticated Data) from context
    let aad = format!("env:{};key:{}", environment_id, key);
    
    // Encrypt the value
    let encrypted_value = encryption::encrypt(encryption_key, value.as_bytes(), aad.as_bytes())
        .map_err(|e| DatabaseError::EncryptionError(e.to_string()))?;
    
    let var = Variable::new(environment_id, key, encrypted_value, description);
    create_variable(conn, &var)
}

/// Get and decrypt a variable (high-level helper)
pub fn get_variable_decrypted(
    conn: &Connection,
    id: i64,
    encryption_key: &[u8; 32],
) -> Result<VariableDecrypted, DatabaseError> {
    let var = get_variable(conn, id)?;
    
    // Create AAD from context
    let aad = format!("env:{};key:{}", var.environment_id, var.key);
    
    // Decrypt the value
    let decrypted_bytes = encryption::decrypt(encryption_key, &var.encrypted_value, aad.as_bytes())
        .map_err(|e| DatabaseError::EncryptionError(e.to_string()))?;
    
    let decrypted_value = String::from_utf8(decrypted_bytes.to_vec())
        .map_err(|e| DatabaseError::SerializationError(format!("Invalid UTF-8: {}", e)))?;
    
    Ok(VariableDecrypted {
        id: var.id.unwrap(),
        environment_id: var.environment_id,
        key: var.key,
        value: decrypted_value,
        description: var.description,
        created_at: var.created_at,
        updated_at: var.updated_at,
    })
}

/// Get all variables for an environment with decryption (high-level helper)
pub fn get_variables_by_environment_decrypted(
    conn: &Connection,
    environment_id: i64,
    encryption_key: &[u8; 32],
) -> Result<Vec<VariableDecrypted>, DatabaseError> {
    let variables = get_variables_by_environment(conn, environment_id)?;
    
    let mut decrypted_vars = Vec::new();
    for var in variables {
        let aad = format!("env:{};key:{}", var.environment_id, var.key);
        
        let decrypted_bytes = encryption::decrypt(encryption_key, &var.encrypted_value, aad.as_bytes())
            .map_err(|e| DatabaseError::EncryptionError(e.to_string()))?;
        
        let decrypted_value = String::from_utf8(decrypted_bytes.to_vec())
            .map_err(|e| DatabaseError::SerializationError(format!("Invalid UTF-8: {}", e)))?;
        
        decrypted_vars.push(VariableDecrypted {
            id: var.id.unwrap(),
            environment_id: var.environment_id,
            key: var.key,
            value: decrypted_value,
            description: var.description,
            created_at: var.created_at,
            updated_at: var.updated_at,
        });
    }
    
    Ok(decrypted_vars)
}

/// Update a variable with encryption (high-level helper)
pub fn update_variable_encrypted(
    conn: &Connection,
    id: i64,
    key: String,
    value: String,
    description: Option<String>,
    encryption_key: &[u8; 32],
) -> Result<(), DatabaseError> {
    // Get the existing variable to know the environment_id
    let existing = get_variable(conn, id)?;
    
    // Create AAD from context
    let aad = format!("env:{};key:{}", existing.environment_id, key);
    
    // Encrypt the new value
    let encrypted_value = encryption::encrypt(encryption_key, value.as_bytes(), aad.as_bytes())
        .map_err(|e| DatabaseError::EncryptionError(e.to_string()))?;
    
    let var = Variable::new(existing.environment_id, key, encrypted_value, description);
    update_variable(conn, id, &var)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{Database, operations::{Project, Environment, projects, environments}};
    use crate::crypto::key_derivation;
    
    fn setup_test_db() -> (Database, i64, [u8; 32]) {
        let db = Database::new_in_memory().unwrap();
        db.initialize().unwrap();
        
        let project = Project::new("TestProject".to_string(), None);
        let project_id = projects::create_project(db.connection(), &project).unwrap();
        
        let env = Environment::new(project_id, "test-env".to_string(), None);
        let env_id = environments::create_environment(db.connection(), &env).unwrap();
        
        // Create a test encryption key
        let salt = [1u8; 16];
        let key = key_derivation::derive_key("test_password", &salt).unwrap();
        
        (db, env_id, key)
    }
    
    #[test]
    fn test_create_and_get_variable_encrypted() {
        let (db, env_id, key) = setup_test_db();
        
        let var_id = create_variable_encrypted(
            db.connection(),
            env_id,
            "API_KEY".to_string(),
            "secret_value_123".to_string(),
            Some("API Key".to_string()),
            &key,
        ).unwrap();
        
        let decrypted = get_variable_decrypted(db.connection(), var_id, &key).unwrap();
        
        assert_eq!(decrypted.key, "API_KEY");
        assert_eq!(decrypted.value, "secret_value_123");
        assert_eq!(decrypted.description, Some("API Key".to_string()));
    }
    
    #[test]
    fn test_encryption_with_wrong_key_fails() {
        let (db, env_id, key) = setup_test_db();
        
        let var_id = create_variable_encrypted(
            db.connection(),
            env_id,
            "SECRET".to_string(),
            "my_secret".to_string(),
            None,
            &key,
        ).unwrap();
        
        // Try to decrypt with wrong key
        let wrong_key = [0u8; 32];
        assert!(get_variable_decrypted(db.connection(), var_id, &wrong_key).is_err());
    }
    
    #[test]
    fn test_get_variables_by_environment_decrypted() {
        let (db, env_id, key) = setup_test_db();
        
        create_variable_encrypted(db.connection(), env_id, "VAR1".to_string(), "value1".to_string(), None, &key).unwrap();
        create_variable_encrypted(db.connection(), env_id, "VAR2".to_string(), "value2".to_string(), None, &key).unwrap();
        
        let vars = get_variables_by_environment_decrypted(db.connection(), env_id, &key).unwrap();
        
        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0].value, "value1");
        assert_eq!(vars[1].value, "value2");
    }
    
    #[test]
    fn test_update_variable_encrypted() {
        let (db, env_id, key) = setup_test_db();
        
        let var_id = create_variable_encrypted(
            db.connection(),
            env_id,
            "OLD_KEY".to_string(),
            "old_value".to_string(),
            None,
            &key,
        ).unwrap();
        
        update_variable_encrypted(
            db.connection(),
            var_id,
            "NEW_KEY".to_string(),
            "new_value".to_string(),
            Some("Updated".to_string()),
            &key,
        ).unwrap();
        
        let decrypted = get_variable_decrypted(db.connection(), var_id, &key).unwrap();
        assert_eq!(decrypted.key, "NEW_KEY");
        assert_eq!(decrypted.value, "new_value");
    }
    
    #[test]
    fn test_delete_variable() {
        let (db, env_id, key) = setup_test_db();
        
        let var_id = create_variable_encrypted(
            db.connection(),
            env_id,
            "TO_DELETE".to_string(),
            "value".to_string(),
            None,
            &key,
        ).unwrap();
        
        delete_variable(db.connection(), var_id).unwrap();
        
        assert!(get_variable(db.connection(), var_id).is_err());
    }
    
    #[test]
    fn test_unique_key_per_environment() {
        let (db, env_id, key) = setup_test_db();
        
        create_variable_encrypted(db.connection(), env_id, "SAME_KEY".to_string(), "value1".to_string(), None, &key).unwrap();
        
        assert!(create_variable_encrypted(db.connection(), env_id, "SAME_KEY".to_string(), "value2".to_string(), None, &key).is_err());
    }
    
    #[test]
    fn test_cascade_delete_from_environment() {
        let (db, env_id, key) = setup_test_db();
        
        let var_id = create_variable_encrypted(
            db.connection(),
            env_id,
            "TEST".to_string(),
            "value".to_string(),
            None,
            &key,
        ).unwrap();
        
        environments::delete_environment(db.connection(), env_id).unwrap();
        
        assert!(get_variable(db.connection(), var_id).is_err());
    }
}
