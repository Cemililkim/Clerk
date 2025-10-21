use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;
use std::fs;

// Re-use library code from the main app
use app_lib::crypto::{self, verify_password};
use app_lib::database::{Database, operations};
use app_lib::database::operations::{Project, Environment, Variable};
use app_lib::vault;

// Session file name (stored in temp directory with process ID)
const SESSION_FILE_PREFIX: &str = ".clerk_session";

#[derive(Parser)]
#[command(name = "clerk")]
#[command(about = "Clerk - Secure Environment Variable Manager CLI", long_about = None)]
#[command(version)]
struct Cli {
    /// Skip session cache (always prompt for password)
    #[arg(short = 'S', long, global = true)]
    no_session: bool,
    
    /// Custom vault directory
    #[arg(short = 'D', long, global = true)]
    vault_dir: Option<PathBuf>,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Unlock the vault with master password
    Unlock {
        /// Custom vault directory (optional)
        #[arg(short, long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Get a variable value
    #[command(visible_alias = "g")]
    Get {
        /// Variable key name
        key: String,
        
        /// Project name
        #[arg(short, long)]
        project: String,
        
        /// Environment name
        #[arg(short, long)]
        env: String,
        
        /// Custom vault directory (optional)
        #[arg(short, long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Set a variable value
    #[command(visible_alias = "s")]
    Set {
        /// Variable key name
        key: String,
        
        /// Variable value
        value: String,
        
        /// Project name
        #[arg(short, long)]
        project: String,
        
        /// Environment name
        #[arg(short, long)]
        env: String,
        
        /// Variable description (optional)
        #[arg(short, long)]
        description: Option<String>,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// List all variables
    #[command(visible_alias = "ls")]
    List {
        /// Filter by project name (optional)
        #[arg(short, long)]
        project: Option<String>,
        
        /// Filter by environment name (optional)
        #[arg(short, long)]
        env: Option<String>,
        
        /// Show values (default: hidden)
        #[arg(short, long)]
        show_values: bool,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Export environment variables to .env format
    Export {
        /// Project name
        #[arg(short, long)]
        project: String,
        
        /// Environment name
        #[arg(short, long)]
        env: String,
        
        /// Output file (optional, defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Initialize a new project
    Init {
        /// Project name
        project: String,
        
        /// Project description (optional)
        #[arg(short, long)]
        description: Option<String>,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Run a command with environment variables injected
    Run {
        /// Project name
        #[arg(short, long)]
        project: String,
        
        /// Environment name
        #[arg(short, long)]
        env: String,
        
        /// Command to run (e.g., "npm start", "python app.py")
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Lock the vault (clear session)
    Lock,
    
    /// Check session status
    Status {
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Create a new project
    #[command(visible_alias = "pc")]
    ProjectCreate {
        /// Project name
        name: String,
        
        /// Project description (optional)
        #[arg(short, long)]
        description: Option<String>,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// List all projects
    #[command(visible_alias = "pl")]
    ProjectList {
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Delete a project
    #[command(visible_alias = "pd")]
    ProjectDelete {
        /// Project name
        name: String,
        
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Create a new environment in a project
    #[command(visible_alias = "ec")]
    EnvCreate {
        /// Environment name
        name: String,
        
        /// Project name
        #[arg(short, long)]
        project: String,
        
        /// Environment description (optional)
        #[arg(short, long)]
        description: Option<String>,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// List all environments in a project
    #[command(visible_alias = "el")]
    EnvList {
        /// Project name
        #[arg(short, long)]
        project: String,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Delete an environment
    #[command(visible_alias = "ed")]
    EnvDelete {
        /// Environment name
        name: String,
        
        /// Project name
        #[arg(short, long)]
        project: String,
        
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Delete a variable
    #[command(visible_alias = "d")]
    Delete {
        /// Variable key name
        key: String,
        
        /// Project name
        #[arg(short, long)]
        project: String,
        
        /// Environment name
        #[arg(short, long)]
        env: String,
        
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Copy a variable to another environment
    #[command(visible_alias = "cp")]
    Copy {
        /// Variable key name
        key: String,
        
        /// Source project name
        #[arg(long)]
        from_project: String,
        
        /// Source environment name
        #[arg(long)]
        from_env: String,
        
        /// Target project name
        #[arg(long)]
        to_project: String,
        
        /// Target environment name
        #[arg(long)]
        to_env: String,
        
        /// Overwrite if variable exists in target
        #[arg(long)]
        overwrite: bool,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
    
    /// Import variables from a .env file
    #[command(visible_alias = "imp")]
    Import {
        /// Path to .env file
        file: PathBuf,
        
        /// Project name
        #[arg(short, long)]
        project: String,
        
        /// Environment name
        #[arg(short, long)]
        env: String,
        
        /// Overwrite existing variables
        #[arg(long)]
        overwrite: bool,
        
        /// Custom vault directory (optional)
        #[arg(short = 'V', long)]
        vault_dir: Option<PathBuf>,
    },
}

impl Commands {
    fn vault_dir(&self) -> Option<PathBuf> {
        match self {
            Commands::Unlock { vault_dir } => vault_dir.clone(),
            Commands::Get { vault_dir, .. } => vault_dir.clone(),
            Commands::Set { vault_dir, .. } => vault_dir.clone(),
            Commands::List { vault_dir, .. } => vault_dir.clone(),
            Commands::Export { vault_dir, .. } => vault_dir.clone(),
            Commands::Init { vault_dir, .. } => vault_dir.clone(),
            Commands::Run { vault_dir, .. } => vault_dir.clone(),
            Commands::Lock => None,
            Commands::Status { vault_dir } => vault_dir.clone(),
            Commands::ProjectCreate { vault_dir, .. } => vault_dir.clone(),
            Commands::ProjectList { vault_dir } => vault_dir.clone(),
            Commands::ProjectDelete { vault_dir, .. } => vault_dir.clone(),
            Commands::EnvCreate { vault_dir, .. } => vault_dir.clone(),
            Commands::EnvList { vault_dir, .. } => vault_dir.clone(),
            Commands::EnvDelete { vault_dir, .. } => vault_dir.clone(),
            Commands::Delete { vault_dir, .. } => vault_dir.clone(),
            Commands::Copy { vault_dir, .. } => vault_dir.clone(),
            Commands::Import { vault_dir, .. } => vault_dir.clone(),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let use_session = !cli.no_session;
    let vault_dir = cli.vault_dir.or_else(|| cli.command.vault_dir());
    
    match &cli.command {
        Commands::Unlock { .. } => {
            if let Err(e) = cmd_unlock(vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::Get { key, project, env, .. } => {
            if let Err(e) = cmd_get(key, project, env, vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::Set { key, value, project, env, description, .. } => {
            if let Err(e) = cmd_set(key, value, project, env, description.as_deref(), vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::List { project, env, show_values, .. } => {
            if let Err(e) = cmd_list(project.as_deref(), env.as_deref(), *show_values, vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::Export { project, env, output, .. } => {
            if let Err(e) = cmd_export(project, env, output.clone(), vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::Init { project, description, .. } => {
            if let Err(e) = cmd_init(project, description.as_deref(), vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::Run { project, env, command, .. } => {
            if command.is_empty() {
                eprintln!("Error: No command specified");
                process::exit(1);
            }
            if let Err(e) = cmd_run(project, env, command, vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::Lock => {
            if let Err(e) = cmd_lock(vault_dir.clone()) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::Status { .. } => {
            if let Err(e) = cmd_status(vault_dir.clone()) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::ProjectCreate { name, description, .. } => {
            if let Err(e) = cmd_project_create(name, description.as_deref(), vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::ProjectList { .. } => {
            if let Err(e) = cmd_project_list(vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::ProjectDelete { name, force, .. } => {
            if let Err(e) = cmd_project_delete(name, *force, vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::EnvCreate { name, project, description, .. } => {
            if let Err(e) = cmd_env_create(name, project, description.as_deref(), vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::EnvList { project, .. } => {
            if let Err(e) = cmd_env_list(project, vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::EnvDelete { name, project, force, .. } => {
            if let Err(e) = cmd_env_delete(name, project, *force, vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::Delete { key, project, env, force, .. } => {
            if let Err(e) = cmd_delete(key, project, env, *force, vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::Copy { key, from_project, from_env, to_project, to_env, overwrite, .. } => {
            if let Err(e) = cmd_copy(key, from_project, from_env, to_project, to_env, *overwrite, vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::Import { file, project, env, overwrite, .. } => {
            if let Err(e) = cmd_import(file, project, env, *overwrite, vault_dir.clone(), use_session) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn get_vault_dir(custom_dir: Option<PathBuf>) -> Result<PathBuf, String> {
    if let Some(dir) = custom_dir {
        Ok(dir)
    } else {
        vault::get_vault_directory().map_err(|e| e.to_string())
    }
}

// ========== SESSION MANAGEMENT ==========

fn get_session_file(vault_dir: &PathBuf) -> PathBuf {
    // Use vault directory hash to create unique session file
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    vault_dir.to_string_lossy().hash(&mut hasher);
    let hash = hasher.finish();
    
    std::env::temp_dir().join(format!("{}-{:x}", SESSION_FILE_PREFIX, hash))
}

fn save_session(password: &str, vault_dir: &PathBuf) -> Result<(), String> {
    let session_data = format!("{}|{}", password, vault_dir.display());
    let session_file = get_session_file(vault_dir);
    
    fs::write(&session_file, session_data)
        .map_err(|e| format!("Failed to save session: {}", e))?;
    
    Ok(())
}

fn load_session(vault_dir: &PathBuf) -> Option<String> {
    let session_file = get_session_file(vault_dir);
    
    if !session_file.exists() {
        return None;
    }
    
    let content = fs::read_to_string(&session_file).ok()?;
    let parts: Vec<&str> = content.splitn(2, '|').collect();
    
    if parts.len() != 2 {
        return None;
    }
    
    // Verify vault directory matches
    if PathBuf::from(parts[1]) != *vault_dir {
        return None;
    }
    
    Some(parts[0].to_string())
}

fn delete_session(vault_dir: &PathBuf) {
    let session_file = get_session_file(vault_dir);
    let _ = fs::remove_file(&session_file);
}

// ========== VAULT OPERATIONS ==========

fn unlock_vault(vault_dir: Option<PathBuf>, use_session: bool) -> Result<(Database, [u8; 32]), String> {
    let vault_path = get_vault_dir(vault_dir)?;
    let metadata_path = vault_path.join("vault.clerk");
    
    if !metadata_path.exists() {
        return Err("Vault does not exist. Please create one using the GUI first.".to_string());
    }
    
    // Read vault metadata
    let metadata_content = std::fs::read_to_string(&metadata_path)
        .map_err(|e| format!("Failed to read vault metadata: {}", e))?;
    
    let metadata: vault::VaultMetadata = serde_json::from_str(&metadata_content)
        .map_err(|e| format!("Failed to parse vault metadata: {}", e))?;
    
    // Try to load password from session if enabled
    let password = if use_session {
        if let Some(cached_password) = load_session(&vault_path) {
            println!("🔓 Using cached session...");
            cached_password
        } else {
            println!("🔐 Enter master password:");
            let pwd = rpassword::read_password()
                .map_err(|e| format!("Failed to read password: {}", e))?;
            pwd
        }
    } else {
        println!("🔐 Enter master password:");
        rpassword::read_password()
            .map_err(|e| format!("Failed to read password: {}", e))?
    };
    
    // Verify password
    if !verify_password(&password, &metadata.password_hash)
        .map_err(|e| format!("Password verification failed: {}", e))? {
        // Delete invalid session if exists
        if use_session {
            delete_session(&vault_path);
        }
        return Err("Invalid password".to_string());
    }
    
    // Save session if enabled and not already cached
    if use_session && load_session(&vault_path).is_none() {
        save_session(&password, &vault_path)?;
        println!("💾 Session saved for this terminal");
    }
    
    // Derive encryption key
    let salt: [u8; 16] = metadata.salt.as_slice()
        .try_into()
        .map_err(|_| "Invalid salt length")?;
    
    let key = crypto::key_derivation::derive_key(&password, &salt)
        .map_err(|e| format!("Key derivation failed: {}", e))?;
    
    // Open database
    let db_path = vault_path.join("vault.db");
    let db = Database::new(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    println!("✅ Vault unlocked successfully!");
    Ok((db, key))
}

fn cmd_unlock(vault_dir: Option<PathBuf>, use_session: bool) -> Result<(), String> {
    unlock_vault(vault_dir, use_session)?;
    println!("✅ Vault is ready. You can now run other commands.");
    Ok(())
}

fn cmd_get(key: &str, project_name: &str, env_name: &str, vault_dir: Option<PathBuf>, use_session: bool) -> Result<(), String> {
    let (db, encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Find project
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    let project = projects.iter()
        .find(|p| p.name == project_name)
        .ok_or_else(|| format!("Project '{}' not found", project_name))?;
    
    // Find environment
    let environments = operations::environments::get_environments_by_project(db.connection(), project.id.unwrap())
        .map_err(|e| format!("Failed to get environments: {}", e))?;
    
    let environment = environments.iter()
        .find(|e| e.name == env_name)
        .ok_or_else(|| format!("Environment '{}' not found in project '{}'", env_name, project_name))?;
    
    // Get variables
    let variables = operations::variables::get_variables_by_environment_decrypted(
        db.connection(),
        environment.id.unwrap(),
        &encryption_key,
    ).map_err(|e| format!("Failed to get variables: {}", e))?;
    
    // Find the specific variable
    let variable = variables.iter()
        .find(|v| v.key == key)
        .ok_or_else(|| format!("Variable '{}' not found", key))?;
    
    // Output just the value (perfect for shell scripts)
    println!("{}", variable.value);
    Ok(())
}

fn cmd_set(
    key: &str,
    value: &str,
    project_name: &str,
    env_name: &str,
    description: Option<&str>,
    vault_dir: Option<PathBuf>,
    use_session: bool,
) -> Result<(), String> {
    let (db, encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Find project
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    let project = projects.iter()
        .find(|p| p.name == project_name)
        .ok_or_else(|| format!("Project '{}' not found", project_name))?;
    
    // Find environment
    let environments = operations::environments::get_environments_by_project(db.connection(), project.id.unwrap())
        .map_err(|e| format!("Failed to get environments: {}", e))?;
    
    let environment = environments.iter()
        .find(|e| e.name == env_name)
        .ok_or_else(|| format!("Environment '{}' not found in project '{}'", env_name, project_name))?;
    
    // Check if variable exists
    let variables = operations::variables::get_variables_by_environment_decrypted(
        db.connection(),
        environment.id.unwrap(),
        &encryption_key,
    ).map_err(|e| format!("Failed to get variables: {}", e))?;
    
    if let Some(existing) = variables.iter().find(|v| v.key == key) {
        // Update existing variable
        operations::variables::update_variable_encrypted(
            db.connection(),
            existing.id,
            key.to_string(),
            value.to_string(),
            description.map(String::from),
            &encryption_key,
        ).map_err(|e| format!("Failed to update variable: {}", e))?;
        
        println!("✅ Updated variable '{}'", key);
    } else {
        // Create new variable
        operations::variables::create_variable_encrypted(
            db.connection(),
            environment.id.unwrap(),
            key.to_string(),
            value.to_string(),
            description.map(String::from),
            &encryption_key,
        ).map_err(|e| format!("Failed to create variable: {}", e))?;
        
        println!("✅ Created variable '{}'", key);
    }
    
    Ok(())
}

fn cmd_list(
    project_filter: Option<&str>,
    env_filter: Option<&str>,
    show_values: bool,
    vault_dir: Option<PathBuf>,
    use_session: bool,
) -> Result<(), String> {
    let (db, encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Get all projects
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    let filtered_projects: Vec<_> = if let Some(filter) = project_filter {
        projects.iter().filter(|p| p.name == filter).collect()
    } else {
        projects.iter().collect()
    };
    
    if filtered_projects.is_empty() {
        if let Some(filter) = project_filter {
            println!("❌ No project found matching '{}'", filter);
        } else {
            println!("📭 No projects found. Create one using the GUI or 'clerk init'");
        }
        return Ok(());
    }
    
    for project in filtered_projects {
        println!("\n📦 Project: {}", project.name);
        if let Some(desc) = &project.description {
            println!("   Description: {}", desc);
        }
        
        // Get environments
        let environments = operations::environments::get_environments_by_project(
            db.connection(),
            project.id.unwrap(),
        ).map_err(|e| format!("Failed to get environments: {}", e))?;
        
        let filtered_envs: Vec<_> = if let Some(filter) = env_filter {
            environments.iter().filter(|e| e.name == filter).collect()
        } else {
            environments.iter().collect()
        };
        
        for env in filtered_envs {
            println!("   🌍 Environment: {}", env.name);
            
            // Get variables
            let variables = operations::variables::get_variables_by_environment_decrypted(
                db.connection(),
                env.id.unwrap(),
                &encryption_key,
            ).map_err(|e| format!("Failed to get variables: {}", e))?;
            
            if variables.is_empty() {
                println!("      (no variables)");
            } else {
                for var in variables {
                    if show_values {
                        println!("      {}={}", var.key, var.value);
                    } else {
                        println!("      {}=********", var.key);
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn cmd_export(
    project_name: &str,
    env_name: &str,
    output: Option<PathBuf>,
    vault_dir: Option<PathBuf>,
    use_session: bool,
) -> Result<(), String> {
    let (db, encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Find project
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    let project = projects.iter()
        .find(|p| p.name == project_name)
        .ok_or_else(|| format!("Project '{}' not found", project_name))?;
    
    // Find environment
    let environments = operations::environments::get_environments_by_project(db.connection(), project.id.unwrap())
        .map_err(|e| format!("Failed to get environments: {}", e))?;
    
    let environment = environments.iter()
        .find(|e| e.name == env_name)
        .ok_or_else(|| format!("Environment '{}' not found in project '{}'", env_name, project_name))?;
    
    // Get variables
    let variables = operations::variables::get_variables_by_environment_decrypted(
        db.connection(),
        environment.id.unwrap(),
        &encryption_key,
    ).map_err(|e| format!("Failed to get variables: {}", e))?;
    
    // Generate .env content
    let mut content = String::new();
    content.push_str("# Generated by Clerk CLI\n");
    content.push_str(&format!("# Project: {}\n", project_name));
    content.push_str(&format!("# Environment: {}\n", env_name));
    content.push_str(&format!("# Total variables: {}\n\n", variables.len()));
    
    for var in variables {
        let value = if var.value.contains(' ') || var.value.contains('"') {
            format!("\"{}\"", var.value.replace('"', "\\\""))
        } else {
            var.value.clone()
        };
        content.push_str(&format!("{}={}\n", var.key, value));
    }
    
    // Output to file or stdout
    if let Some(path) = output {
        std::fs::write(&path, content)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        println!("✅ Exported to {}", path.display());
    } else {
        print!("{}", content);
    }
    
    Ok(())
}

fn cmd_init(project_name: &str, description: Option<&str>, vault_dir: Option<PathBuf>, use_session: bool) -> Result<(), String> {
    let (db, _encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Check if project already exists
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    if projects.iter().any(|p| p.name == project_name) {
        return Err(format!("Project '{}' already exists", project_name));
    }
    
    // Create project
    let project = operations::Project {
        id: None,
        name: project_name.to_string(),
        description: description.map(String::from),
        created_at: chrono::Utc::now().timestamp(),
        updated_at: chrono::Utc::now().timestamp(),
    };
    
    operations::projects::create_project(db.connection(), &project)
        .map_err(|e| format!("Failed to create project: {}", e))?;
    
    println!("✅ Created project '{}'", project_name);
    println!("💡 Next steps:");
    println!("   1. Create an environment (using GUI or add to this CLI)");
    println!("   2. Add variables with: clerk set KEY VALUE -p {} -e ENV_NAME", project_name);
    
    Ok(())
}

fn cmd_run(project_name: &str, env_name: &str, command: &[String], vault_dir: Option<PathBuf>, use_session: bool) -> Result<(), String> {
    use std::process::Command;
    use std::collections::HashMap;
    
    let (db, encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Get project
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    let project = projects.iter()
        .find(|p| p.name == project_name)
        .ok_or_else(|| format!("Project '{}' not found", project_name))?;
    
    // Get environment
    let environments = operations::environments::get_environments_by_project(
        db.connection(),
        project.id.unwrap(),
    ).map_err(|e| format!("Failed to get environments: {}", e))?;
    
    let environment = environments.iter()
        .find(|e| e.name == env_name)
        .ok_or_else(|| format!("Environment '{}' not found in project '{}'", env_name, project_name))?;
    
    // Get variables (encrypted)
    let variables = operations::variables::get_variables_by_environment(
        db.connection(),
        environment.id.unwrap(),
    ).map_err(|e| format!("Failed to get variables: {}", e))?;
    
    // Build environment variable map
    let mut env_vars: HashMap<String, String> = std::env::vars().collect();
    
    println!("🔐 Injecting {} variables into process...", variables.len());
    for var in variables {
        // Create AAD (Additional Authenticated Data) matching the format used during encryption
        let aad = format!("env:{};key:{}", var.environment_id, var.key);
        
        // Decrypt the value
        let decrypted = crypto::encryption::decrypt(
            &encryption_key,
            &var.encrypted_value,
            aad.as_bytes(),
        ).map_err(|e| format!("Failed to decrypt variable '{}': {:?}", var.key, e))?;
        
        let value = String::from_utf8(decrypted.to_vec())
            .map_err(|e| format!("Invalid UTF-8 in variable '{}': {}", var.key, e))?;
        
        env_vars.insert(var.key.clone(), value);
    }
    
    // Parse command
    let program = &command[0];
    let args = &command[1..];
    
    println!("🚀 Running: {} {}", program, args.join(" "));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Run command with injected environment variables
    let mut child = Command::new(program)
        .args(args)
        .envs(&env_vars)
        .spawn()
        .map_err(|e| format!("Failed to run command: {}", e))?;
    
    // Wait for command to complete
    let status = child.wait()
        .map_err(|e| format!("Failed to wait for command: {}", e))?;
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    if status.success() {
        println!("✅ Command completed successfully");
        Ok(())
    } else {
        let code = status.code().unwrap_or(-1);
        Err(format!("Command failed with exit code {}", code))
    }
}

fn cmd_lock(vault_dir: Option<PathBuf>) -> Result<(), String> {
    let vault_path = get_vault_dir(vault_dir)?;
    delete_session(&vault_path);
    println!("🔒 Session cleared. You'll need to enter your password for the next command.");
    Ok(())
}

fn cmd_status(vault_dir: Option<PathBuf>) -> Result<(), String> {
    let vault_path = get_vault_dir(vault_dir)?;
    let session_file = get_session_file(&vault_path);
    
    if !session_file.exists() {
        println!("🔒 No active session");
        return Ok(());
    }
    
    let content = fs::read_to_string(&session_file)
        .map_err(|e| format!("Failed to read session: {}", e))?;
    
    let parts: Vec<&str> = content.splitn(2, '|').collect();
    if parts.len() != 2 {
        println!("🔒 Invalid session data");
        return Ok(());
    }
    
    let session_vault = PathBuf::from(parts[1]);
    if session_vault == vault_path {
        println!("🔓 Active session for vault: {}", vault_path.display());
        println!("   Session file: {}", session_file.display());
    } else {
        println!("🔒 Session vault mismatch");
        println!("   Current vault: {}", vault_path.display());
        println!("   Session vault: {}", session_vault.display());
    }
    
    Ok(())
}

// ========== PROJECT MANAGEMENT ==========

fn cmd_project_create(name: &str, description: Option<&str>, vault_dir: Option<PathBuf>, use_session: bool) -> Result<(), String> {
    let (db, _encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Check if project already exists
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    if projects.iter().any(|p| p.name == name) {
        return Err(format!("Project '{}' already exists", name));
    }
    
    // Create project
    let project = Project::new(name.to_string(), description.map(|s| s.to_string()));
    operations::projects::create_project(db.connection(), &project)
        .map_err(|e| format!("Failed to create project: {}", e))?;
    
    println!("✅ Project '{}' created successfully!", name);
    Ok(())
}

fn cmd_project_list(vault_dir: Option<PathBuf>, use_session: bool) -> Result<(), String> {
    let (db, _encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    if projects.is_empty() {
        println!("No projects found. Create one with: clerk project-create <name>");
        return Ok(());
    }
    
    println!("📦 Projects ({})", projects.len());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    for project in projects {
        let project_id = project.id.ok_or("Project ID is missing")?;
        let env_count = operations::environments::get_environments_by_project(db.connection(), project_id)
            .map(|envs| envs.len())
            .unwrap_or(0);
        
        println!("  • {} (ID: {})", project.name, project_id);
        if let Some(desc) = &project.description {
            if !desc.is_empty() {
                println!("    Description: {}", desc);
            }
        }
        println!("    Environments: {}", env_count);
    }
    
    Ok(())
}

fn cmd_project_delete(name: &str, force: bool, vault_dir: Option<PathBuf>, use_session: bool) -> Result<(), String> {
    let (db, _encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Find project
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    let project = projects.iter()
        .find(|p| p.name == name)
        .ok_or_else(|| format!("Project '{}' not found", name))?;
    
    let project_id = project.id.ok_or("Project ID is missing")?;
    
    // Check for environments
    let environments = operations::environments::get_environments_by_project(db.connection(), project_id)
        .map_err(|e| format!("Failed to get environments: {}", e))?;
    
    if !environments.is_empty() && !force {
        println!("⚠️  Project '{}' has {} environment(s)", name, environments.len());
        println!("   Use --force to delete anyway, or delete environments first:");
        for env in &environments {
            println!("     - {}", env.name);
        }
        return Err("Cannot delete project with environments".to_string());
    }
    
    // Delete project
    operations::projects::delete_project(db.connection(), project_id)
        .map_err(|e| format!("Failed to delete project: {}", e))?;
    
    println!("✅ Project '{}' deleted successfully!", name);
    Ok(())
}

// ========== ENVIRONMENT MANAGEMENT ==========

fn cmd_env_create(name: &str, project_name: &str, description: Option<&str>, vault_dir: Option<PathBuf>, use_session: bool) -> Result<(), String> {
    let (db, _encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Find project
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    let project = projects.iter()
        .find(|p| p.name == project_name)
        .ok_or_else(|| format!("Project '{}' not found", project_name))?;
    
    let project_id = project.id.ok_or("Project ID is missing")?;
    
    // Check if environment already exists
    let environments = operations::environments::get_environments_by_project(db.connection(), project_id)
        .map_err(|e| format!("Failed to get environments: {}", e))?;
    
    if environments.iter().any(|e| e.name == name) {
        return Err(format!("Environment '{}' already exists in project '{}'", name, project_name));
    }
    
    // Create environment
    let environment = Environment::new(project_id, name.to_string(), description.map(|s| s.to_string()));
    operations::environments::create_environment(db.connection(), &environment)
        .map_err(|e| format!("Failed to create environment: {}", e))?;
    
    println!("✅ Environment '{}' created in project '{}'!", name, project_name);
    Ok(())
}

fn cmd_env_list(project_name: &str, vault_dir: Option<PathBuf>, use_session: bool) -> Result<(), String> {
    let (db, _encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Find project
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    let project = projects.iter()
        .find(|p| p.name == project_name)
        .ok_or_else(|| format!("Project '{}' not found", project_name))?;
    
    let project_id = project.id.ok_or("Project ID is missing")?;
    
    // Get environments
    let environments = operations::environments::get_environments_by_project(db.connection(), project_id)
        .map_err(|e| format!("Failed to get environments: {}", e))?;
    
    if environments.is_empty() {
        println!("No environments found in project '{}'. Create one with: clerk env-create <name> -p {}", project_name, project_name);
        return Ok(());
    }
    
    println!("🌍 Environments in '{}' ({})", project_name, environments.len());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    for env in environments {
        let env_id = env.id.ok_or("Environment ID is missing")?;
        let var_count = operations::variables::get_variables_by_environment(db.connection(), env_id)
            .map(|vars| vars.len())
            .unwrap_or(0);
        
        println!("  • {} (ID: {})", env.name, env_id);
        if let Some(desc) = &env.description {
            if !desc.is_empty() {
                println!("    Description: {}", desc);
            }
        }
        println!("    Variables: {}", var_count);
    }
    
    Ok(())
}

fn cmd_env_delete(name: &str, project_name: &str, force: bool, vault_dir: Option<PathBuf>, use_session: bool) -> Result<(), String> {
    let (db, _encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Find project
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    let project = projects.iter()
        .find(|p| p.name == project_name)
        .ok_or_else(|| format!("Project '{}' not found", project_name))?;
    
    let project_id = project.id.ok_or("Project ID is missing")?;
    
    // Find environment
    let environments = operations::environments::get_environments_by_project(db.connection(), project_id)
        .map_err(|e| format!("Failed to get environments: {}", e))?;
    
    let environment = environments.iter()
        .find(|e| e.name == name)
        .ok_or_else(|| format!("Environment '{}' not found in project '{}'", name, project_name))?;
    
    let environment_id = environment.id.ok_or("Environment ID is missing")?;
    
    // Check for variables
    let variables = operations::variables::get_variables_by_environment(db.connection(), environment_id)
        .map_err(|e| format!("Failed to get variables: {}", e))?;
    
    if !variables.is_empty() && !force {
        println!("⚠️  Environment '{}' has {} variable(s)", name, variables.len());
        println!("   Use --force to delete anyway, or delete variables first:");
        for var in variables.iter().take(5) {
            println!("     - {}", var.key);
        }
        if variables.len() > 5 {
            println!("     ... and {} more", variables.len() - 5);
        }
        return Err("Cannot delete environment with variables".to_string());
    }
    
    // Delete environment (cascade will delete variables)
    operations::environments::delete_environment(db.connection(), environment_id)
        .map_err(|e| format!("Failed to delete environment: {}", e))?;
    
    println!("✅ Environment '{}' deleted from project '{}'!", name, project_name);
    Ok(())
}

// ========== VARIABLE OPERATIONS ==========

fn cmd_delete(key: &str, project_name: &str, env_name: &str, force: bool, vault_dir: Option<PathBuf>, use_session: bool) -> Result<(), String> {
    let (db, _encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Find project
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    let project = projects.iter()
        .find(|p| p.name == project_name)
        .ok_or_else(|| format!("Project '{}' not found", project_name))?;
    
    let project_id = project.id.ok_or("Project ID is missing")?;
    
    // Find environment
    let environments = operations::environments::get_environments_by_project(db.connection(), project_id)
        .map_err(|e| format!("Failed to get environments: {}", e))?;
    
    let environment = environments.iter()
        .find(|e| e.name == env_name)
        .ok_or_else(|| format!("Environment '{}' not found in project '{}'", env_name, project_name))?;
    
    let environment_id = environment.id.ok_or("Environment ID is missing")?;
    
    // Find variable
    let variables = operations::variables::get_variables_by_environment(db.connection(), environment_id)
        .map_err(|e| format!("Failed to get variables: {}", e))?;
    
    let variable = variables.iter()
        .find(|v| v.key == key)
        .ok_or_else(|| format!("Variable '{}' not found", key))?;
    
    let variable_id = variable.id.ok_or("Variable ID is missing")?;
    
    // Confirm deletion if not forced
    if !force {
        println!("⚠️  Are you sure you want to delete '{}'? (use --force to skip this prompt)", key);
        println!("   Project: {}", project_name);
        println!("   Environment: {}", env_name);
        
        // For CLI, we'll require --force flag instead of interactive prompt
        return Err("Deletion cancelled. Use --force to confirm".to_string());
    }
    
    // Delete variable
    operations::variables::delete_variable(db.connection(), variable_id)
        .map_err(|e| format!("Failed to delete variable: {}", e))?;
    
    println!("✅ Variable '{}' deleted from {}/{}", key, project_name, env_name);
    Ok(())
}

fn cmd_copy(
    key: &str,
    from_project: &str,
    from_env: &str,
    to_project: &str,
    to_env: &str,
    overwrite: bool,
    vault_dir: Option<PathBuf>,
    use_session: bool,
) -> Result<(), String> {
    let (db, _encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Find source project
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    let src_project = projects.iter()
        .find(|p| p.name == from_project)
        .ok_or_else(|| format!("Source project '{}' not found", from_project))?;
    
    let src_project_id = src_project.id.ok_or("Source project ID is missing")?;
    
    // Find target project
    let dest_project = projects.iter()
        .find(|p| p.name == to_project)
        .ok_or_else(|| format!("Target project '{}' not found", to_project))?;
    
    let dest_project_id = dest_project.id.ok_or("Target project ID is missing")?;
    
    // Find source environment
    let src_environments = operations::environments::get_environments_by_project(db.connection(), src_project_id)
        .map_err(|e| format!("Failed to get source environments: {}", e))?;
    
    let src_environment = src_environments.iter()
        .find(|e| e.name == from_env)
        .ok_or_else(|| format!("Source environment '{}' not found", from_env))?;
    
    let src_environment_id = src_environment.id.ok_or("Source environment ID is missing")?;
    
    // Find target environment
    let dest_environments = operations::environments::get_environments_by_project(db.connection(), dest_project_id)
        .map_err(|e| format!("Failed to get target environments: {}", e))?;
    
    let dest_environment = dest_environments.iter()
        .find(|e| e.name == to_env)
        .ok_or_else(|| format!("Target environment '{}' not found", to_env))?;
    
    let dest_environment_id = dest_environment.id.ok_or("Target environment ID is missing")?;
    
    // Find source variable
    let src_variables = operations::variables::get_variables_by_environment(db.connection(), src_environment_id)
        .map_err(|e| format!("Failed to get source variables: {}", e))?;
    
    let src_variable = src_variables.iter()
        .find(|v| v.key == key)
        .ok_or_else(|| format!("Variable '{}' not found in source environment", key))?;
    
    // Check if variable exists in target
    let dest_variables = operations::variables::get_variables_by_environment(db.connection(), dest_environment_id)
        .map_err(|e| format!("Failed to get target variables: {}", e))?;
    
    let exists_in_target = dest_variables.iter().any(|v| v.key == key);
    
    if exists_in_target && !overwrite {
        return Err(format!(
            "Variable '{}' already exists in {}/{}. Use --overwrite to replace it",
            key, to_project, to_env
        ));
    }
    
    // Create or update variable in target environment
    if exists_in_target {
        // Update existing
        let target_var = dest_variables.iter()
            .find(|v| v.key == key)
            .unwrap();
        
        let target_var_id = target_var.id.ok_or("Target variable ID is missing")?;
        
        let updated_var = Variable::new(
            dest_environment_id,
            key.to_string(),
            src_variable.encrypted_value.clone(),
            src_variable.description.clone(),
        );
        
        operations::variables::update_variable(
            db.connection(),
            target_var_id,
            &updated_var,
        )
        .map_err(|e| format!("Failed to update variable: {}", e))?;
        
        println!("✅ Variable '{}' updated in {}/{}", key, to_project, to_env);
    } else {
        // Create new
        let new_var = Variable::new(
            dest_environment_id,
            key.to_string(),
            src_variable.encrypted_value.clone(),
            src_variable.description.clone(),
        );
        
        operations::variables::create_variable(
            db.connection(),
            &new_var,
        )
        .map_err(|e| format!("Failed to create variable: {}", e))?;
        
        println!("✅ Variable '{}' copied to {}/{}", key, to_project, to_env);
    }
    
    Ok(())
}

fn cmd_import(
    file_path: &PathBuf,
    project_name: &str,
    env_name: &str,
    overwrite: bool,
    vault_dir: Option<PathBuf>,
    use_session: bool,
) -> Result<(), String> {
    let (db, encryption_key) = unlock_vault(vault_dir, use_session)?;
    
    // Check if file exists
    if !file_path.exists() {
        return Err(format!("File not found: {}", file_path.display()));
    }
    
    // Read .env file
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    // Find project
    let projects = operations::projects::get_all_projects(db.connection())
        .map_err(|e| format!("Failed to get projects: {}", e))?;
    
    let project = projects.iter()
        .find(|p| p.name == project_name)
        .ok_or_else(|| format!("Project '{}' not found", project_name))?;
    
    let project_id = project.id.ok_or("Project ID is missing")?;
    
    // Find environment
    let environments = operations::environments::get_environments_by_project(db.connection(), project_id)
        .map_err(|e| format!("Failed to get environments: {}", e))?;
    
    let environment = environments.iter()
        .find(|e| e.name == env_name)
        .ok_or_else(|| format!("Environment '{}' not found in project '{}'", env_name, project_name))?;
    
    let environment_id = environment.id.ok_or("Environment ID is missing")?;
    
    // Get existing variables
    let existing_variables = operations::variables::get_variables_by_environment(db.connection(), environment_id)
        .map_err(|e| format!("Failed to get variables: {}", e))?;
    
    // Parse .env file
    let mut imported_count = 0;
    let mut skipped_count = 0;
    let mut updated_count = 0;
    
    for line in content.lines() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // Parse KEY=VALUE
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim()
                .trim_matches('"')
                .trim_matches('\'');
            
            // Check if variable exists
            let exists = existing_variables.iter().any(|v| v.key == key);
            
            if exists && !overwrite {
                skipped_count += 1;
                continue;
            }
            
            if exists {
                // Update existing using encrypted helper
                let var = existing_variables.iter()
                    .find(|v| v.key == key)
                    .unwrap();
                
                let var_id = var.id.ok_or("Variable ID is missing")?;
                
                operations::variables::update_variable_encrypted(
                    db.connection(),
                    var_id,
                    key.to_string(),
                    value.to_string(),
                    None,
                    &encryption_key,
                )
                .map_err(|e| format!("Failed to update variable '{}': {}", key, e))?;
                
                updated_count += 1;
            } else {
                // Create new using encrypted helper
                operations::variables::create_variable_encrypted(
                    db.connection(),
                    environment_id,
                    key.to_string(),
                    value.to_string(),
                    None,
                    &encryption_key,
                )
                .map_err(|e| format!("Failed to create variable '{}': {}", key, e))?;
                
                imported_count += 1;
            }
        }
    }
    
    println!("✅ Import completed:");
    println!("   Created: {}", imported_count);
    println!("   Updated: {}", updated_count);
    if skipped_count > 0 {
        println!("   Skipped: {} (use --overwrite to update existing)", skipped_count);
    }
    
    Ok(())
}
