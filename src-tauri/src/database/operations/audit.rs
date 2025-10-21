use rusqlite::Connection;
use chrono::Utc;

/// Log an audit entry to the audit_log table
pub fn log_audit(
    conn: &Connection,
    operation_type: &str,
    entity_type: &str,
    entity_id: Option<i64>,
    entity_name: Option<&str>,
    details: Option<serde_json::Value>,
) -> Result<(), String> {
    let now = Utc::now().timestamp();
    let details_str = details.map(|d| d.to_string());
    
    conn.execute(
        "INSERT INTO audit_log (timestamp, operation_type, entity_type, entity_id, entity_name, details, created_at) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            &now,
            operation_type,
            entity_type,
            &entity_id,
            &entity_name,
            &details_str,
            &now,
        ),
    )
    .map_err(|e| format!("Failed to log audit entry: {}", e))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    #[test]
    fn test_log_audit() {
        let db = Database::new_in_memory().unwrap();
        let conn = db.connection();
        
        // Create audit_log table
        conn.execute(
            "CREATE TABLE audit_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                operation_type TEXT NOT NULL,
                entity_type TEXT NOT NULL,
                entity_id INTEGER,
                entity_name TEXT,
                details TEXT,
                created_at INTEGER NOT NULL
            )",
            [],
        ).unwrap();
        
        // Test logging
        let result = log_audit(
            conn,
            "create",
            "project",
            Some(1),
            Some("Test Project"),
            Some(json!({"description": "A test project"})),
        );
        
        assert!(result.is_ok());
        
        // Verify entry was created
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM audit_log", [], |row| row.get(0))
            .unwrap();
        
        assert_eq!(count, 1);
    }
}
