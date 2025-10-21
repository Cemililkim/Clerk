// Keychain module - OS credential storage integration
//
// Uses platform-specific secure storage:
// - Windows: Windows Credential Manager
// - macOS: macOS Keychain
// - Linux: Secret Service API (gnome-keyring, KWallet)

use keyring::Entry;
use base64::{Engine as _, engine::general_purpose};

const SERVICE_NAME: &str = "com.clerk.app";
const USERNAME: &str = "clerk_user"; // Username for keychain entry

/// Keychain manager for storing encryption keys securely
pub struct KeychainManager;

impl KeychainManager {
    /// Create a new keychain manager
    pub fn new() -> Self {
        Self
    }

    /// Save encryption key to OS keychain
    /// 
    /// # Arguments
    /// * `key` - 32-byte encryption key to store
    /// 
    /// # Returns
    /// * `Ok(())` if key was saved successfully
    /// * `Err(String)` if saving failed
    pub fn save_key(&self, key: &[u8; 32]) -> Result<(), String> {
        // Convert key to base64 for string storage
        let key_b64 = general_purpose::STANDARD.encode(key);
        
        // Create keyring entry
        let entry = Entry::new(SERVICE_NAME, USERNAME)
            .map_err(|e| format!("Failed to create keychain entry: {}", e))?;
        
        // Save password (our encryption key)
        entry.set_password(&key_b64)
            .map_err(|e| format!("Failed to save key to keychain: {}", e))?;
        
        Ok(())
    }

    /// Retrieve encryption key from OS keychain
    /// 
    /// # Returns
    /// * `Ok(Some([u8; 32]))` if key was found and decoded
    /// * `Ok(None)` if no key is stored
    /// * `Err(String)` if retrieval or decoding failed
    pub fn get_key(&self) -> Result<Option<[u8; 32]>, String> {
        // Create keyring entry with same parameters as save
        let entry = Entry::new(SERVICE_NAME, USERNAME)
            .map_err(|e| format!("Failed to create keychain entry: {}", e))?;
        
        // Try to get password
        match entry.get_password() {
            Ok(key_b64) => {
                // Decode from base64
                let key_bytes = general_purpose::STANDARD.decode(&key_b64)
                    .map_err(|e| format!("Failed to decode key: {}", e))?;
                
                // Convert to fixed-size array
                if key_bytes.len() != 32 {
                    return Err(format!("Invalid key length: expected 32, got {}", key_bytes.len()));
                }
                
                let mut key = [0u8; 32];
                key.copy_from_slice(&key_bytes);
                
                Ok(Some(key))
            },
            Err(keyring::Error::NoEntry) => {
                // No key stored - this is OK
                Ok(None)
            },
            Err(e) => {
                // Log the actual error for debugging
                eprintln!("Keychain error: {:?}", e);
                Err(format!("Failed to get key from keychain: {}", e))
            }
        }
    }

    /// Delete encryption key from OS keychain
    /// 
    /// # Returns
    /// * `Ok(())` if key was deleted or didn't exist
    /// * `Err(String)` if deletion failed
    pub fn delete_key(&self) -> Result<(), String> {
        // Create keyring entry
        let entry = Entry::new(SERVICE_NAME, USERNAME)
            .map_err(|e| format!("Failed to create keychain entry: {}", e))?;
        
        // Delete password (ignore NotFound errors)
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
            Err(e) => Err(format!("Failed to delete key from keychain: {}", e))
        }
    }

    /// Check if a key is stored in the keychain
    /// 
    /// # Returns
    /// * `true` if a key exists
    /// * `false` if no key is stored
    pub fn has_key(&self) -> bool {
        matches!(self.get_key(), Ok(Some(_)))
    }
}

impl Default for KeychainManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests may fail on Windows due to keyring crate behavior
    // Manual testing recommended: check Windows Credential Manager after running app

    #[test]
    #[ignore] // Ignore for CI - requires OS keychain access
    fn test_save_and_get_key() {
        let manager = KeychainManager::new();
        let test_key = [42u8; 32];

        // Clean up any existing key first
        let _ = manager.delete_key();

        // Save key
        let save_result = manager.save_key(&test_key);
        println!("Save result: {:?}", save_result);
        assert!(save_result.is_ok(), "Failed to save key: {:?}", save_result.err());

        // Retrieve key
        let get_result = manager.get_key();
        println!("Get result: {:?}", get_result);
        
        match get_result {
            Ok(Some(retrieved)) => {
                assert_eq!(retrieved, test_key);
            },
            Ok(None) => {
                panic!("Key was saved but not found in keychain!");
            },
            Err(e) => {
                panic!("Failed to get key: {}", e);
            }
        }

        // Clean up
        manager.delete_key().unwrap();
    }

    #[test]
    fn test_get_nonexistent_key() {
        let manager = KeychainManager::new();
        
        // Make sure no key exists
        let _ = manager.delete_key();
        
        // Try to get key
        let result = manager.get_key().unwrap();
        assert_eq!(result, None);
    }

    #[test]
    #[ignore] // Ignore for CI
    fn test_delete_key() {
        let manager = KeychainManager::new();
        let test_key = [99u8; 32];

        // Save key
        manager.save_key(&test_key).unwrap();

        // Delete key
        manager.delete_key().unwrap();

        // Verify it's gone
        let result = manager.get_key().unwrap();
        assert_eq!(result, None);
    }

    #[test]
    #[ignore] // Ignore for CI
    fn test_has_key() {
        let manager = KeychainManager::new();
        let test_key = [77u8; 32];

        // Initially should have no key
        let _ = manager.delete_key();
        assert_eq!(manager.has_key(), false);

        // Save key
        manager.save_key(&test_key).unwrap();
        assert_eq!(manager.has_key(), true);

        // Delete key
        manager.delete_key().unwrap();
        assert_eq!(manager.has_key(), false);
    }
}
