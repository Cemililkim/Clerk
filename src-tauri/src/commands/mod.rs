// Tauri commands - exposed to frontend

pub mod vault;
pub mod database;
pub mod export;
pub mod audit;
pub mod backup;
pub mod system;

/// Example command that will be callable from the frontend
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Clerk.", name)
}
