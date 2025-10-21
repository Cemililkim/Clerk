use tauri::State;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use crate::database::{Database, operations};

/// Shared database state
pub struct DatabaseState {
    pub db: Mutex<Option<Database>>,
    pub encryption_key: Mutex<Option<[u8; 32]>>,
}

impl DatabaseState {
    pub fn new() -> Self {
        Self {
            db: Mutex::new(None),
            encryption_key: Mutex::new(None),
        }
    }
}

// ============================================================================
// PROJECT COMMANDS
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectResponse {
    pub success: bool,
    pub project_id: Option<i64>,
    pub message: String,
}

#[tauri::command]
pub async fn create_project(
    state: State<'_, DatabaseState>,
    request: CreateProjectRequest,
) -> Result<CreateProjectResponse, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let project = operations::Project::new(request.name.clone(), request.description);
    
    match operations::projects::create_project(db.connection(), &project) {
        Ok(id) => Ok(CreateProjectResponse {
            success: true,
            project_id: Some(id),
            message: format!("Project '{}' created successfully", request.name),
        }),
        Err(e) => Ok(CreateProjectResponse {
            success: false,
            project_id: None,
            message: format!("Failed to create project: {}", e),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProjectsResponse {
    pub success: bool,
    pub projects: Vec<operations::Project>,
    pub message: String,
}

#[tauri::command]
pub async fn get_projects(
    state: State<'_, DatabaseState>,
) -> Result<GetProjectsResponse, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    match operations::projects::get_all_projects(db.connection()) {
        Ok(projects) => Ok(GetProjectsResponse {
            success: true,
            projects,
            message: "Projects retrieved successfully".to_string(),
        }),
        Err(e) => Ok(GetProjectsResponse {
            success: false,
            projects: vec![],
            message: format!("Failed to retrieve projects: {}", e),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProjectRequest {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProjectResponse {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub async fn update_project(
    state: State<'_, DatabaseState>,
    request: UpdateProjectRequest,
) -> Result<UpdateProjectResponse, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let project = operations::Project::new(request.name.clone(), request.description);
    
    match operations::projects::update_project(db.connection(), request.id, &project) {
        Ok(_) => Ok(UpdateProjectResponse {
            success: true,
            message: "Project updated successfully".to_string(),
        }),
        Err(e) => Ok(UpdateProjectResponse {
            success: false,
            message: format!("Failed to update project: {}", e),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteProjectRequest {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteProjectResponse {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub async fn delete_project(
    state: State<'_, DatabaseState>,
    request: DeleteProjectRequest,
) -> Result<DeleteProjectResponse, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    match operations::projects::delete_project(db.connection(), request.id) {
        Ok(_) => Ok(DeleteProjectResponse {
            success: true,
            message: "Project deleted successfully".to_string(),
        }),
        Err(e) => Ok(DeleteProjectResponse {
            success: false,
            message: format!("Failed to delete project: {}", e),
        }),
    }
}

// ============================================================================
// ENVIRONMENT COMMANDS
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEnvironmentRequest {
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEnvironmentResponse {
    pub success: bool,
    pub environment_id: Option<i64>,
    pub message: String,
}

#[tauri::command]
pub async fn create_environment(
    state: State<'_, DatabaseState>,
    request: CreateEnvironmentRequest,
) -> Result<CreateEnvironmentResponse, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let env = operations::Environment::new(
        request.project_id,
        request.name.clone(),
        request.description,
    );
    
    match operations::environments::create_environment(db.connection(), &env) {
        Ok(id) => Ok(CreateEnvironmentResponse {
            success: true,
            environment_id: Some(id),
            message: format!("Environment '{}' created successfully", request.name),
        }),
        Err(e) => Ok(CreateEnvironmentResponse {
            success: false,
            environment_id: None,
            message: format!("Failed to create environment: {}", e),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEnvironmentsRequest {
    pub project_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEnvironmentsResponse {
    pub success: bool,
    pub environments: Vec<operations::Environment>,
    pub message: String,
}

#[tauri::command]
pub async fn get_environments(
    state: State<'_, DatabaseState>,
    request: GetEnvironmentsRequest,
) -> Result<GetEnvironmentsResponse, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    match operations::environments::get_environments_by_project(db.connection(), request.project_id) {
        Ok(environments) => Ok(GetEnvironmentsResponse {
            success: true,
            environments,
            message: "Environments retrieved successfully".to_string(),
        }),
        Err(e) => Ok(GetEnvironmentsResponse {
            success: false,
            environments: vec![],
            message: format!("Failed to retrieve environments: {}", e),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEnvironmentRequest {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEnvironmentResponse {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub async fn update_environment(
    state: State<'_, DatabaseState>,
    request: UpdateEnvironmentRequest,
) -> Result<UpdateEnvironmentResponse, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let env = operations::Environment::new(
        request.project_id,
        request.name.clone(),
        request.description,
    );
    
    match operations::environments::update_environment(db.connection(), request.id, &env) {
        Ok(_) => Ok(UpdateEnvironmentResponse {
            success: true,
            message: "Environment updated successfully".to_string(),
        }),
        Err(e) => Ok(UpdateEnvironmentResponse {
            success: false,
            message: format!("Failed to update environment: {}", e),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteEnvironmentRequest {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteEnvironmentResponse {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub async fn delete_environment(
    state: State<'_, DatabaseState>,
    request: DeleteEnvironmentRequest,
) -> Result<DeleteEnvironmentResponse, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    match operations::environments::delete_environment(db.connection(), request.id) {
        Ok(_) => Ok(DeleteEnvironmentResponse {
            success: true,
            message: "Environment deleted successfully".to_string(),
        }),
        Err(e) => Ok(DeleteEnvironmentResponse {
            success: false,
            message: format!("Failed to delete environment: {}", e),
        }),
    }
}

// ============================================================================
// VARIABLE COMMANDS (with encryption)
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVariableRequest {
    pub environment_id: i64,
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVariableResponse {
    pub success: bool,
    pub variable_id: Option<i64>,
    pub message: String,
}

#[tauri::command]
pub async fn create_variable(
    state: State<'_, DatabaseState>,
    request: CreateVariableRequest,
) -> Result<CreateVariableResponse, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let key_guard = state.encryption_key.lock().map_err(|e| e.to_string())?;
    let encryption_key = key_guard.as_ref().ok_or("Encryption key not available")?;
    
    match operations::variables::create_variable_encrypted(
        db.connection(),
        request.environment_id,
        request.key.clone(),
        request.value,
        request.description,
        encryption_key,
    ) {
        Ok(id) => Ok(CreateVariableResponse {
            success: true,
            variable_id: Some(id),
            message: format!("Variable '{}' created successfully", request.key),
        }),
        Err(e) => Ok(CreateVariableResponse {
            success: false,
            variable_id: None,
            message: format!("Failed to create variable: {}", e),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVariablesRequest {
    pub environment_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVariablesResponse {
    pub success: bool,
    pub variables: Vec<operations::VariableDecrypted>,
    pub message: String,
}

#[tauri::command]
pub async fn get_variables(
    state: State<'_, DatabaseState>,
    request: GetVariablesRequest,
) -> Result<GetVariablesResponse, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let key_guard = state.encryption_key.lock().map_err(|e| e.to_string())?;
    let encryption_key = key_guard.as_ref().ok_or("Encryption key not available")?;
    
    match operations::variables::get_variables_by_environment_decrypted(
        db.connection(),
        request.environment_id,
        encryption_key,
    ) {
        Ok(variables) => Ok(GetVariablesResponse {
            success: true,
            variables,
            message: "Variables retrieved successfully".to_string(),
        }),
        Err(e) => Ok(GetVariablesResponse {
            success: false,
            variables: vec![],
            message: format!("Failed to retrieve variables: {}", e),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateVariableRequest {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateVariableResponse {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub async fn update_variable(
    state: State<'_, DatabaseState>,
    request: UpdateVariableRequest,
) -> Result<UpdateVariableResponse, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let key_guard = state.encryption_key.lock().map_err(|e| e.to_string())?;
    let encryption_key = key_guard.as_ref().ok_or("Encryption key not available")?;
    
    match operations::variables::update_variable_encrypted(
        db.connection(),
        request.id,
        request.key.clone(),
        request.value,
        request.description,
        encryption_key,
    ) {
        Ok(_) => Ok(UpdateVariableResponse {
            success: true,
            message: "Variable updated successfully".to_string(),
        }),
        Err(e) => Ok(UpdateVariableResponse {
            success: false,
            message: format!("Failed to update variable: {}", e),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteVariableRequest {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteVariableResponse {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub async fn delete_variable(
    state: State<'_, DatabaseState>,
    request: DeleteVariableRequest,
) -> Result<DeleteVariableResponse, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    match operations::variables::delete_variable(db.connection(), request.id) {
        Ok(_) => Ok(DeleteVariableResponse {
            success: true,
            message: "Variable deleted successfully".to_string(),
        }),
        Err(e) => Ok(DeleteVariableResponse {
            success: false,
            message: format!("Failed to delete variable: {}", e),
        }),
    }
}

// ============================================================================
// DASHBOARD STATS
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub project_count: usize,
    pub environment_count: usize,
    pub variable_count: usize,
}

#[tauri::command]
pub async fn get_dashboard_stats(
    state: State<'_, DatabaseState>,
) -> Result<DashboardStats, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let db = db_guard.as_ref().ok_or("Database not initialized")?;
    
    let conn = db.connection();
    
    let project_count: usize = conn
        .query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))
        .unwrap_or(0);
    
    let environment_count: usize = conn
        .query_row("SELECT COUNT(*) FROM environments", [], |row| row.get(0))
        .unwrap_or(0);
    
    let variable_count: usize = conn
        .query_row("SELECT COUNT(*) FROM variables", [], |row| row.get(0))
        .unwrap_or(0);
    
    Ok(DashboardStats {
        project_count,
        environment_count,
        variable_count,
    })
}
