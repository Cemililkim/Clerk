use std::path::PathBuf;
use tauri::AppHandle;
// Manager trait is only required when calling `AppHandle::path()` in production builds
#[cfg(not(debug_assertions))]
use tauri::Manager;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

/// Get the CLI executable path from the app's resources
fn get_cli_path(_app: &AppHandle) -> Result<PathBuf, String> {
    // In dev mode, CLI is in target/debug or target/release
    // In production, CLI is in resources directory
    
    #[cfg(debug_assertions)]
    {
        // Dev mode: Use the manifest dir that was set at compile time
        let manifest_dir = env!("TAURI_MANIFEST_DIR");
        let src_tauri = PathBuf::from(manifest_dir);
        
        let debug_cli = src_tauri.join("target").join("debug").join("clerk.exe");
        let release_cli = src_tauri.join("target").join("release").join("clerk.exe");
        
        if debug_cli.exists() {
            Ok(debug_cli)
        } else if release_cli.exists() {
            Ok(release_cli)
        } else {
            Err(format!(
                "CLI executable not found in:\n- {}\n- {}\n\nPlease run: cargo build --bin clerk",
                debug_cli.display(),
                release_cli.display()
            ))
        }
    }
    
    #[cfg(not(debug_assertions))]
    {
        // Production mode: CLI is in resources directory
        let resource_dir = _app
            .path()
            .resource_dir()
            .map_err(|e| format!("Failed to get resource directory: {}", e))?;
        
        let cli_path = resource_dir.join("clerk.exe");
        
        if !cli_path.exists() {
            return Err("CLI executable not found in application directory".to_string());
        }
        
        Ok(cli_path)
    }
}

/// Check if the CLI executable is in the system PATH
#[tauri::command]
pub fn check_cli_in_path(app: AppHandle) -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        let cli_path = get_cli_path(&app)?;
        let cli_dir = cli_path
            .parent()
            .ok_or("Failed to get CLI directory")?
            .to_string_lossy()
            .to_string();

        // Check user PATH from registry
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let env_key = hkcu
            .open_subkey("Environment")
            .map_err(|e| format!("Failed to open registry key: {}", e))?;
        
        let path_value: String = env_key
            .get_value("Path")
            .map_err(|e| format!("Failed to read PATH: {}", e))?;

        // Check if our CLI directory is in the PATH
        let is_in_path = path_value
            .split(';')
            .any(|p| p.trim().eq_ignore_ascii_case(&cli_dir));

        Ok(is_in_path)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("PATH management is only supported on Windows".to_string())
    }
}

/// Add the CLI executable to the system PATH
#[tauri::command]
pub fn add_cli_to_path(app: AppHandle) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let cli_path = get_cli_path(&app)?;
        let cli_dir = cli_path
            .parent()
            .ok_or("Failed to get CLI directory")?
            .to_string_lossy()
            .to_string();

        // Open registry key with write access
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let env_key = hkcu
            .open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)
            .map_err(|e| format!("Failed to open registry key: {}", e))?;
        
        let mut path_value: String = env_key
            .get_value("Path")
            .map_err(|e| format!("Failed to read PATH: {}", e))?;

        // Check if already in PATH
        let is_in_path = path_value
            .split(';')
            .any(|p| p.trim().eq_ignore_ascii_case(&cli_dir));

        if is_in_path {
            return Ok(()); // Already in PATH
        }

        // Add to PATH
        if !path_value.ends_with(';') && !path_value.is_empty() {
            path_value.push(';');
        }
        path_value.push_str(&cli_dir);

        // Write back to registry
        env_key
            .set_value("Path", &path_value)
            .map_err(|e| format!("Failed to write PATH: {}", e))?;

        // Broadcast WM_SETTINGCHANGE message to notify system
        unsafe {
            use windows::Win32::UI::WindowsAndMessaging::*;
            use windows::Win32::Foundation::*;
            
            let environment: Vec<u16> = "Environment\0".encode_utf16().collect();
            let _ = SendMessageTimeoutW(
                HWND_BROADCAST,
                WM_SETTINGCHANGE,
                WPARAM(0),
                LPARAM(environment.as_ptr() as isize),
                SMTO_ABORTIFHUNG,
                5000,
                None,
            );
        }

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("PATH management is only supported on Windows".to_string())
    }
}

/// Remove the CLI executable from the system PATH
#[tauri::command]
pub fn remove_cli_from_path(app: AppHandle) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let cli_path = get_cli_path(&app)?;
        let cli_dir = cli_path
            .parent()
            .ok_or("Failed to get CLI directory")?
            .to_string_lossy()
            .to_string();

        // Open registry key with write access
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let env_key = hkcu
            .open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)
            .map_err(|e| format!("Failed to open registry key: {}", e))?;
        
        let path_value: String = env_key
            .get_value("Path")
            .map_err(|e| format!("Failed to read PATH: {}", e))?;

        // Remove our directory from PATH
        let new_path: Vec<&str> = path_value
            .split(';')
            .filter(|p| !p.trim().eq_ignore_ascii_case(&cli_dir))
            .collect();

        let new_path_value = new_path.join(";");

        // Write back to registry
        env_key
            .set_value("Path", &new_path_value)
            .map_err(|e| format!("Failed to write PATH: {}", e))?;

        // Broadcast WM_SETTINGCHANGE message to notify system
        unsafe {
            use windows::Win32::UI::WindowsAndMessaging::*;
            use windows::Win32::Foundation::*;
            
            let environment: Vec<u16> = "Environment\0".encode_utf16().collect();
            let _ = SendMessageTimeoutW(
                HWND_BROADCAST,
                WM_SETTINGCHANGE,
                WPARAM(0),
                LPARAM(environment.as_ptr() as isize),
                SMTO_ABORTIFHUNG,
                5000,
                None,
            );
        }

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("PATH management is only supported on Windows".to_string())
    }
}
