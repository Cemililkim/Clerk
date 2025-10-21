use rusqlite::{Connection, params};
use chrono::Utc;
use serde_json::json;
use crate::database::{DatabaseError, operations::{Environment, audit::log_audit}};

/// Create a new environment
pub fn create_environment(conn: &Connection, env: &Environment) -> Result<i64, DatabaseError> {
    conn.execute(
        "INSERT INTO environments (project_id, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        params![
            env.project_id,
            &env.name,
            &env.description,
            env.created_at,
            env.updated_at,
        ],
    )?;
    
    let env_id = conn.last_insert_rowid();
    
    // Log the audit entry
    let _ = log_audit(
        conn,
        "create",
        "environment",
        Some(env_id),
        Some(&env.name),
        Some(json!({
            "project_id": env.project_id,
            "description": &env.description,
        })),
    );
    
    Ok(env_id)
}

/// Get an environment by ID
pub fn get_environment(conn: &Connection, id: i64) -> Result<Environment, DatabaseError> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, name, description, created_at, updated_at FROM environments WHERE id = ?"
    )?;
    
    let env = stmt.query_row(params![id], |row| {
        Ok(Environment {
            id: Some(row.get(0)?),
            project_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })?;
    
    Ok(env)
}

/// Get all environments for a project
pub fn get_environments_by_project(conn: &Connection, project_id: i64) -> Result<Vec<Environment>, DatabaseError> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, name, description, created_at, updated_at FROM environments WHERE project_id = ? ORDER BY name"
    )?;
    
    let environments = stmt.query_map(params![project_id], |row| {
        Ok(Environment {
            id: Some(row.get(0)?),
            project_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(environments)
}

/// Get all environments
pub fn get_all_environments(conn: &Connection) -> Result<Vec<Environment>, DatabaseError> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, name, description, created_at, updated_at FROM environments ORDER BY project_id, name"
    )?;
    
    let environments = stmt.query_map([], |row| {
        Ok(Environment {
            id: Some(row.get(0)?),
            project_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(environments)
}

/// Update an environment
pub fn update_environment(conn: &Connection, id: i64, env: &Environment) -> Result<(), DatabaseError> {
    let now = Utc::now().timestamp();
    let rows_affected = conn.execute(
        "UPDATE environments SET name = ?, description = ?, updated_at = ? WHERE id = ?",
        params![&env.name, &env.description, now, id],
    )?;
    
    if rows_affected == 0 {
        return Err(DatabaseError::NotFound(format!("Environment with id {} not found", id)));
    }
    
    // Log the audit entry
    let _ = log_audit(
        conn,
        "update",
        "environment",
        Some(id),
        Some(&env.name),
        Some(json!({
            "project_id": env.project_id,
            "description": &env.description,
        })),
    );
    
    Ok(())
}

/// Delete an environment (cascades to variables)
pub fn delete_environment(conn: &Connection, id: i64) -> Result<(), DatabaseError> {
    // Get environment name before deleting for audit log
    let env_name: Option<String> = conn.query_row(
        "SELECT name FROM environments WHERE id = ?",
        params![id],
        |row| row.get(0),
    ).ok();
    
    let rows_affected = conn.execute("DELETE FROM environments WHERE id = ?", params![id])?;
    
    if rows_affected == 0 {
        return Err(DatabaseError::NotFound(format!("Environment with id {} not found", id)));
    }
    
    // Log the audit entry
    let _ = log_audit(
        conn,
        "delete",
        "environment",
        Some(id),
        env_name.as_deref(),
        None,
    );
    
    Ok(())
}

/// Check if an environment exists by name within a project
pub fn environment_exists(conn: &Connection, project_id: i64, name: &str) -> Result<bool, DatabaseError> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM environments WHERE project_id = ? AND name = ?",
        params![project_id, name],
        |row| row.get(0),
    )?;
    
    Ok(count > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{Database, operations::{Project, projects}};
    
    fn setup_test_db() -> (Database, i64) {
        let db = Database::new_in_memory().unwrap();
        db.initialize().unwrap();
        
        let project = Project::new("TestProject".to_string(), None);
        let project_id = projects::create_project(db.connection(), &project).unwrap();
        
        (db, project_id)
    }
    
    #[test]
    fn test_create_and_get_environment() {
        let (db, project_id) = setup_test_db();
        
        let env = Environment::new(project_id, "development".to_string(), Some("Dev environment".to_string()));
        let id = create_environment(db.connection(), &env).unwrap();
        
        assert!(id > 0);
        
        let retrieved = get_environment(db.connection(), id).unwrap();
        assert_eq!(retrieved.name, "development");
        assert_eq!(retrieved.project_id, project_id);
    }
    
    #[test]
    fn test_get_environments_by_project() {
        let (db, project_id) = setup_test_db();
        
        let env1 = Environment::new(project_id, "development".to_string(), None);
        let env2 = Environment::new(project_id, "production".to_string(), None);
        
        create_environment(db.connection(), &env1).unwrap();
        create_environment(db.connection(), &env2).unwrap();
        
        let environments = get_environments_by_project(db.connection(), project_id).unwrap();
        assert_eq!(environments.len(), 2);
    }
    
    #[test]
    fn test_update_environment() {
        let (db, project_id) = setup_test_db();
        
        let env = Environment::new(project_id, "old-name".to_string(), None);
        let id = create_environment(db.connection(), &env).unwrap();
        
        let updated_env = Environment::new(project_id, "new-name".to_string(), Some("Updated".to_string()));
        update_environment(db.connection(), id, &updated_env).unwrap();
        
        let retrieved = get_environment(db.connection(), id).unwrap();
        assert_eq!(retrieved.name, "new-name");
    }
    
    #[test]
    fn test_delete_environment() {
        let (db, project_id) = setup_test_db();
        
        let env = Environment::new(project_id, "to-delete".to_string(), None);
        let id = create_environment(db.connection(), &env).unwrap();
        
        delete_environment(db.connection(), id).unwrap();
        
        assert!(get_environment(db.connection(), id).is_err());
    }
    
    #[test]
    fn test_unique_name_per_project() {
        let (db, project_id) = setup_test_db();
        
        let env1 = Environment::new(project_id, "production".to_string(), None);
        let env2 = Environment::new(project_id, "production".to_string(), None);
        
        create_environment(db.connection(), &env1).unwrap();
        assert!(create_environment(db.connection(), &env2).is_err());
    }
    
    #[test]
    fn test_cascade_delete_from_project() {
        let (db, project_id) = setup_test_db();
        
        let env = Environment::new(project_id, "test-env".to_string(), None);
        let env_id = create_environment(db.connection(), &env).unwrap();
        
        // Delete the project
        projects::delete_project(db.connection(), project_id).unwrap();
        
        // Environment should be deleted too
        assert!(get_environment(db.connection(), env_id).is_err());
    }
}
