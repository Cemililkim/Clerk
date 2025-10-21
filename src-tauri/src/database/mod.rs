use rusqlite::Connection;
use std::path::Path;
use thiserror::Error;

pub mod schema;
pub mod migrations;
pub mod operations;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    
    #[error("Migration error: {0}")]
    MigrationError(String),
    
    #[error("Query error: {0}")]
    QueryError(String),
    
    #[error("Data not found: {0}")]
    NotFound(String),
    
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<rusqlite::Error> for DatabaseError {
    fn from(err: rusqlite::Error) -> Self {
        match err {
            rusqlite::Error::QueryReturnedNoRows => {
                DatabaseError::NotFound("Query returned no rows".to_string())
            }
            rusqlite::Error::SqliteFailure(err, Some(msg)) => {
                if err.code == rusqlite::ErrorCode::ConstraintViolation {
                    DatabaseError::ConstraintViolation(msg)
                } else {
                    DatabaseError::QueryError(format!("{}: {}", err, msg))
                }
            }
            _ => DatabaseError::QueryError(err.to_string()),
        }
    }
}

/// Database manager for the vault
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create a new database connection
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DatabaseError> {
        let conn = Connection::open(path)
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        
        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        
        Ok(Database { conn })
    }
    
    /// Create an in-memory database (for testing)
    #[cfg(test)]
    pub fn new_in_memory() -> Result<Self, DatabaseError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        
        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        
        Ok(Database { conn })
    }
    
    /// Initialize the database with schema
    pub fn initialize(&self) -> Result<(), DatabaseError> {
        migrations::run_migrations(&self.conn)?;
        Ok(())
    }
    
    /// Get a reference to the connection
    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_creation() {
        let db = Database::new_in_memory().unwrap();
        assert!(db.initialize().is_ok());
    }
    
    #[test]
    fn test_foreign_keys_enabled() {
        let db = Database::new_in_memory().unwrap();
        let foreign_keys: i32 = db.conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(foreign_keys, 1);
    }
}
