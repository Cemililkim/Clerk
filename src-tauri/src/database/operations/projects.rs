use rusqlite::{Connection, params};
use chrono::Utc;
use serde_json::json;
use crate::database::{DatabaseError, operations::{Project, audit::log_audit}};

/// Create a new project
pub fn create_project(conn: &Connection, project: &Project) -> Result<i64, DatabaseError> {
    conn.execute(
        "INSERT INTO projects (name, description, created_at, updated_at) VALUES (?, ?, ?, ?)",
        params![
            &project.name,
            &project.description,
            project.created_at,
            project.updated_at,
        ],
    )?;
    
    let project_id = conn.last_insert_rowid();
    
    // Log the audit entry
    let _ = log_audit(
        conn,
        "create",
        "project",
        Some(project_id),
        Some(&project.name),
        Some(json!({
            "description": &project.description,
        })),
    );
    
    Ok(project_id)
}

/// Get a project by ID
pub fn get_project(conn: &Connection, id: i64) -> Result<Project, DatabaseError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, created_at, updated_at FROM projects WHERE id = ?"
    )?;
    
    let project = stmt.query_row(params![id], |row| {
        Ok(Project {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;
    
    Ok(project)
}

/// Get all projects
pub fn get_all_projects(conn: &Connection) -> Result<Vec<Project>, DatabaseError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, created_at, updated_at FROM projects ORDER BY name"
    )?;
    
    let projects = stmt.query_map([], |row| {
        Ok(Project {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(projects)
}

/// Update a project
pub fn update_project(conn: &Connection, id: i64, project: &Project) -> Result<(), DatabaseError> {
    let now = Utc::now().timestamp();
    let rows_affected = conn.execute(
        "UPDATE projects SET name = ?, description = ?, updated_at = ? WHERE id = ?",
        params![&project.name, &project.description, now, id],
    )?;
    
    if rows_affected == 0 {
        return Err(DatabaseError::NotFound(format!("Project with id {} not found", id)));
    }
    
    // Log the audit entry
    let _ = log_audit(
        conn,
        "update",
        "project",
        Some(id),
        Some(&project.name),
        Some(json!({
            "description": &project.description,
        })),
    );
    
    Ok(())
}

/// Delete a project (cascades to environments and variables)
pub fn delete_project(conn: &Connection, id: i64) -> Result<(), DatabaseError> {
    // Get project name before deleting for audit log
    let project_name: Option<String> = conn.query_row(
        "SELECT name FROM projects WHERE id = ?",
        params![id],
        |row| row.get(0),
    ).ok();
    
    let rows_affected = conn.execute("DELETE FROM projects WHERE id = ?", params![id])?;
    
    if rows_affected == 0 {
        return Err(DatabaseError::NotFound(format!("Project with id {} not found", id)));
    }
    
    // Log the audit entry
    let _ = log_audit(
        conn,
        "delete",
        "project",
        Some(id),
        project_name.as_deref(),
        None,
    );
    
    Ok(())
}

/// Check if a project exists by name
pub fn project_exists_by_name(conn: &Connection, name: &str) -> Result<bool, DatabaseError> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM projects WHERE name = ?",
        params![name],
        |row| row.get(0),
    )?;
    
    Ok(count > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;
    
    #[test]
    fn test_create_and_get_project() {
        let db = Database::new_in_memory().unwrap();
        db.initialize().unwrap();
        
        let project = Project::new("TestProject".to_string(), Some("Test Description".to_string()));
        let id = create_project(db.connection(), &project).unwrap();
        
        assert!(id > 0);
        
        let retrieved = get_project(db.connection(), id).unwrap();
        assert_eq!(retrieved.name, "TestProject");
        assert_eq!(retrieved.description, Some("Test Description".to_string()));
    }
    
    #[test]
    fn test_get_all_projects() {
        let db = Database::new_in_memory().unwrap();
        db.initialize().unwrap();
        
        let project1 = Project::new("Project1".to_string(), None);
        let project2 = Project::new("Project2".to_string(), None);
        
        create_project(db.connection(), &project1).unwrap();
        create_project(db.connection(), &project2).unwrap();
        
        let projects = get_all_projects(db.connection()).unwrap();
        assert_eq!(projects.len(), 2);
    }
    
    #[test]
    fn test_update_project() {
        let db = Database::new_in_memory().unwrap();
        db.initialize().unwrap();
        
        let project = Project::new("OldName".to_string(), None);
        let id = create_project(db.connection(), &project).unwrap();
        
        let mut updated_project = Project::new("NewName".to_string(), Some("New Description".to_string()));
        update_project(db.connection(), id, &updated_project).unwrap();
        
        let retrieved = get_project(db.connection(), id).unwrap();
        assert_eq!(retrieved.name, "NewName");
        assert_eq!(retrieved.description, Some("New Description".to_string()));
    }
    
    #[test]
    fn test_delete_project() {
        let db = Database::new_in_memory().unwrap();
        db.initialize().unwrap();
        
        let project = Project::new("ToDelete".to_string(), None);
        let id = create_project(db.connection(), &project).unwrap();
        
        delete_project(db.connection(), id).unwrap();
        
        assert!(get_project(db.connection(), id).is_err());
    }
    
    #[test]
    fn test_unique_name_constraint() {
        let db = Database::new_in_memory().unwrap();
        db.initialize().unwrap();
        
        let project1 = Project::new("SameName".to_string(), None);
        let project2 = Project::new("SameName".to_string(), None);
        
        create_project(db.connection(), &project1).unwrap();
        assert!(create_project(db.connection(), &project2).is_err());
    }
}
