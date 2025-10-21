use serde::{Deserialize, Serialize};
use chrono::Utc;

pub mod projects;
pub mod environments;
pub mod variables;
pub mod audit;

/// Project model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Project {
    pub fn new(name: String, description: Option<String>) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: None,
            name,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Environment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub id: Option<i64>,
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Environment {
    pub fn new(project_id: i64, name: String, description: Option<String>) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: None,
            project_id,
            name,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Variable model (encrypted value)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub id: Option<i64>,
    pub environment_id: i64,
    pub key: String,
    #[serde(skip)] // Don't serialize the encrypted bytes
    pub encrypted_value: Vec<u8>,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Variable {
    pub fn new(
        environment_id: i64,
        key: String,
        encrypted_value: Vec<u8>,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: None,
            environment_id,
            key,
            encrypted_value,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Variable with decrypted value (for API responses)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDecrypted {
    pub id: i64,
    pub environment_id: i64,
    pub key: String,
    pub value: String, // Decrypted value
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_project_creation() {
        let project = Project::new("TestProject".to_string(), Some("Description".to_string()));
        assert_eq!(project.name, "TestProject");
        assert_eq!(project.description, Some("Description".to_string()));
        assert!(project.id.is_none());
        assert!(project.created_at > 0);
    }
    
    #[test]
    fn test_environment_creation() {
        let env = Environment::new(1, "production".to_string(), None);
        assert_eq!(env.project_id, 1);
        assert_eq!(env.name, "production");
        assert!(env.id.is_none());
    }
    
    #[test]
    fn test_variable_creation() {
        let var = Variable::new(
            1,
            "API_KEY".to_string(),
            vec![1, 2, 3, 4],
            Some("API Key".to_string()),
        );
        assert_eq!(var.environment_id, 1);
        assert_eq!(var.key, "API_KEY");
        assert_eq!(var.encrypted_value, vec![1, 2, 3, 4]);
    }
}
