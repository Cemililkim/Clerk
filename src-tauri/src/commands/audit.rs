use tauri::State;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use crate::commands::database::DatabaseState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: i64,
    pub timestamp: i64,
    pub operation_type: String,
    pub entity_type: String,
    pub entity_id: Option<i64>,
    pub entity_name: Option<String>,
    pub details: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogFilter {
    pub entity_type: Option<String>,
    pub entity_id: Option<i64>,
    pub operation_type: Option<String>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Get audit logs with optional filtering and pagination
#[tauri::command]
pub fn get_audit_logs(
    state: State<DatabaseState>,
    filter: Option<AuditLogFilter>,
) -> Result<Vec<AuditLogEntry>, String> {
    let db = state.db.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    
    if db.is_none() {
        return Err("Database not initialized".to_string());
    }
    
    let database = db.as_ref().unwrap();
    let conn = database.connection();
    
    // Build query dynamically based on filters
    let mut query = String::from(
        "SELECT id, timestamp, operation_type, entity_type, entity_id, entity_name, details, created_at 
         FROM audit_log WHERE 1=1"
    );
    
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    
    if let Some(f) = &filter {
        if let Some(ref et) = f.entity_type {
            query.push_str(" AND entity_type = ?");
            params.push(Box::new(et.clone()));
        }
        
        if let Some(eid) = f.entity_id {
            query.push_str(" AND entity_id = ?");
            params.push(Box::new(eid));
        }
        
        if let Some(ref ot) = f.operation_type {
            query.push_str(" AND operation_type = ?");
            params.push(Box::new(ot.clone()));
        }
        
        if let Some(start) = f.start_date {
            query.push_str(" AND timestamp >= ?");
            params.push(Box::new(start));
        }
        
        if let Some(end) = f.end_date {
            query.push_str(" AND timestamp <= ?");
            params.push(Box::new(end));
        }
    }
    
    // Always order by timestamp DESC (most recent first)
    query.push_str(" ORDER BY timestamp DESC");
    
    // Add pagination
    if let Some(f) = &filter {
        if let Some(limit) = f.limit {
            query.push_str(" LIMIT ?");
            params.push(Box::new(limit));
        }
        
        if let Some(offset) = f.offset {
            query.push_str(" OFFSET ?");
            params.push(Box::new(offset));
        }
    }
    
    let mut stmt = conn.prepare(&query)
        .map_err(|e| format!("Failed to prepare query: {}", e))?;
    
    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    
    let logs = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(AuditLogEntry {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            operation_type: row.get(2)?,
            entity_type: row.get(3)?,
            entity_id: row.get(4)?,
            entity_name: row.get(5)?,
            details: row.get(6)?,
            created_at: row.get(7)?,
        })
    })
    .map_err(|e| format!("Failed to query audit logs: {}", e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| format!("Failed to collect audit logs: {}", e))?;
    
    Ok(logs)
}

/// Export audit logs to CSV format
#[tauri::command]
pub fn export_audit_logs_csv(
    state: State<DatabaseState>,
    filter: Option<AuditLogFilter>,
    file_path: String,
) -> Result<String, String> {
    let db = state.db.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    
    if db.is_none() {
        return Err("Database not initialized".to_string());
    }
    
    let database = db.as_ref().unwrap();
    let conn = database.connection();
    
    // Build query without pagination for export
    let mut query = String::from(
        "SELECT id, timestamp, operation_type, entity_type, entity_id, entity_name, details, created_at 
         FROM audit_log WHERE 1=1"
    );
    
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    
    if let Some(f) = &filter {
        if let Some(ref et) = f.entity_type {
            query.push_str(" AND entity_type = ?");
            params.push(Box::new(et.clone()));
        }
        
        if let Some(eid) = f.entity_id {
            query.push_str(" AND entity_id = ?");
            params.push(Box::new(eid));
        }
        
        if let Some(ref ot) = f.operation_type {
            query.push_str(" AND operation_type = ?");
            params.push(Box::new(ot.clone()));
        }
        
        if let Some(start) = f.start_date {
            query.push_str(" AND timestamp >= ?");
            params.push(Box::new(start));
        }
        
        if let Some(end) = f.end_date {
            query.push_str(" AND timestamp <= ?");
            params.push(Box::new(end));
        }
    }
    
    query.push_str(" ORDER BY timestamp DESC");
    
    let mut stmt = conn.prepare(&query)
        .map_err(|e| format!("Failed to prepare query: {}", e))?;
    
    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    
    let logs = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(AuditLogEntry {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            operation_type: row.get(2)?,
            entity_type: row.get(3)?,
            entity_id: row.get(4)?,
            entity_name: row.get(5)?,
            details: row.get(6)?,
            created_at: row.get(7)?,
        })
    })
    .map_err(|e| format!("Failed to query audit logs: {}", e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| format!("Failed to collect audit logs: {}", e))?;
    
    // Generate CSV content
    let mut csv_content = String::from("Timestamp,Operation,Entity Type,Entity ID,Entity Name,Details\n");
    
    for log in logs.iter() {
        let timestamp = DateTime::from_timestamp(log.timestamp, 0)
            .unwrap_or_else(Utc::now)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        
        let entity_id = log.entity_id.map(|id| id.to_string()).unwrap_or_default();
        let entity_name = log.entity_name.as_deref().unwrap_or("");
        let details = log.details.as_deref().unwrap_or("");
        
        // Escape CSV fields
        let escaped_name = entity_name.replace('"', "\"\"");
        let escaped_details = details.replace('"', "\"\"");
        
        csv_content.push_str(&format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
            timestamp,
            log.operation_type,
            log.entity_type,
            entity_id,
            escaped_name,
            escaped_details
        ));
    }
    
    // Write to file
    let path = PathBuf::from(&file_path);
    fs::write(&path, csv_content)
        .map_err(|e| format!("Failed to write CSV file: {}", e))?;
    
    Ok(format!("Exported {} audit log entries to {}", logs.len(), file_path))
}

/// Export audit logs to JSON format
#[tauri::command]
pub fn export_audit_logs_json(
    state: State<DatabaseState>,
    filter: Option<AuditLogFilter>,
    file_path: String,
) -> Result<String, String> {
    // Get logs using the same filter logic
    let logs = get_audit_logs(state, filter)?;
    
    // Serialize to pretty JSON
    let json_content = serde_json::to_string_pretty(&logs)
        .map_err(|e| format!("Failed to serialize to JSON: {}", e))?;
    
    // Write to file
    let path = PathBuf::from(&file_path);
    fs::write(&path, json_content)
        .map_err(|e| format!("Failed to write JSON file: {}", e))?;
    
    Ok(format!("Exported {} audit log entries to {}", logs.len(), file_path))
}
