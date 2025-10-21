// Module declarations
mod commands;
pub mod crypto;
pub mod database;
pub mod vault;
pub mod keychain;

use commands::database::DatabaseState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .manage(DatabaseState::new())
    .invoke_handler(tauri::generate_handler![
      commands::greet,
      commands::vault::create_vault,
      commands::vault::unlock_vault,
      commands::vault::auto_unlock,
      commands::vault::lock_vault,
      commands::vault::check_vault_exists,
      commands::vault::get_lock_timeout,
      commands::vault::set_lock_timeout,
      // Project commands
      commands::database::create_project,
      commands::database::get_projects,
      commands::database::update_project,
      commands::database::delete_project,
      // Environment commands
      commands::database::create_environment,
      commands::database::get_environments,
      commands::database::update_environment,
      commands::database::delete_environment,
      // Variable commands
      commands::database::create_variable,
      commands::database::get_variables,
      commands::database::update_variable,
      commands::database::delete_variable,
      // Dashboard commands
      commands::database::get_dashboard_stats,
      // Export/Import commands
      commands::export::export_env,
      commands::export::export_env_to_file,
      commands::export::import_env,
      commands::export::read_file_content,
      commands::export::write_file_content,
      // Audit commands
      commands::audit::get_audit_logs,
      commands::audit::export_audit_logs_csv,
      commands::audit::export_audit_logs_json,
      // Backup commands
      commands::backup::create_backup,
      commands::backup::restore_backup,
      commands::backup::get_backup_info,
      commands::backup::validate_backup_file,
      // System / PATH commands
      commands::system::check_cli_in_path,
      commands::system::add_cli_to_path,
      commands::system::remove_cli_from_path,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
