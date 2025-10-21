/// Database schema definitions for Clerk vault
/// 
/// Schema Structure:
/// - vault_metadata: Stores vault configuration and metadata
/// - projects: Top-level organization unit (e.g., "MyApp", "Backend API")
/// - environments: Belongs to a project (e.g., "development", "production")
/// - variables: Belongs to an environment (e.g., "DATABASE_URL", "API_KEY")
///   * Values are encrypted using AES-256-GCM before storage
///   * AAD (Additional Authenticated Data) includes project_id, env_id, key name
pub const SCHEMA_VERSION: u32 = 1;

/// SQL to create the vault_metadata table
pub const CREATE_VAULT_METADATA_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS vault_metadata (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    version INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    last_accessed INTEGER NOT NULL,
    last_modified INTEGER NOT NULL,
    lock_timeout_minutes INTEGER DEFAULT 0
);
"#;

/// SQL to create the projects table
pub const CREATE_PROJECTS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
"#;

/// SQL to create index on projects.name
pub const CREATE_PROJECTS_NAME_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_projects_name ON projects(name);
"#;

/// SQL to create the environments table
pub const CREATE_ENVIRONMENTS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS environments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    UNIQUE(project_id, name)
);
"#;

/// SQL to create index on environments.project_id
pub const CREATE_ENVIRONMENTS_PROJECT_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_environments_project ON environments(project_id);
"#;

/// SQL to create the variables table
pub const CREATE_VARIABLES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS variables (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    environment_id INTEGER NOT NULL,
    key TEXT NOT NULL,
    encrypted_value BLOB NOT NULL,
    description TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE,
    UNIQUE(environment_id, key)
);
"#;

/// SQL to create index on variables.environment_id
pub const CREATE_VARIABLES_ENVIRONMENT_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_variables_environment ON variables(environment_id);
"#;

/// SQL to create index on variables.key
pub const CREATE_VARIABLES_KEY_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_variables_key ON variables(key);
"#;

/// SQL to create the audit_log table
pub const CREATE_AUDIT_LOG_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS audit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    operation_type TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id INTEGER,
    entity_name TEXT,
    details TEXT,
    created_at INTEGER NOT NULL
);
"#;

/// SQL to create index on audit_log.timestamp
pub const CREATE_AUDIT_LOG_TIMESTAMP_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_audit_log_timestamp ON audit_log(timestamp DESC);
"#;

/// SQL to create index on audit_log.entity_type
pub const CREATE_AUDIT_LOG_ENTITY_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_audit_log_entity ON audit_log(entity_type, entity_id);
"#;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_schema_version_is_positive() {
        assert!(SCHEMA_VERSION > 0);
    }
    
    #[test]
    fn test_sql_statements_not_empty() {
        assert!(!CREATE_VAULT_METADATA_TABLE.is_empty());
        assert!(!CREATE_PROJECTS_TABLE.is_empty());
        assert!(!CREATE_ENVIRONMENTS_TABLE.is_empty());
        assert!(!CREATE_VARIABLES_TABLE.is_empty());
    }
}
