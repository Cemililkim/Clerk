pub mod encryption;
pub mod key_derivation;

pub use encryption::{encrypt, decrypt};
pub use key_derivation::{derive_key, hash_password, verify_password, generate_salt};

use zeroize::Zeroizing;

/// Cryptography service for AES-256-GCM encryption with Argon2id key derivation
pub struct CryptoService {
    /// Cached encryption key (zeroized on drop)
    encryption_key: Option<Zeroizing<[u8; 32]>>,
}

impl CryptoService {
    pub fn new() -> Self {
        Self {
            encryption_key: None,
        }
    }

    /// Unlocks the crypto service with a master password
    /// Derives encryption key from password + salt
    pub fn unlock(&mut self, password: &str, salt: &[u8; 16]) -> Result<(), String> {
        let key = derive_key(password, salt)
            .map_err(|e| format!("Key derivation failed: {}", e))?;
        
        self.encryption_key = Some(Zeroizing::new(key));
        Ok(())
    }

    /// Locks the crypto service (clears cached key)
    pub fn lock(&mut self) {
        self.encryption_key = None;
    }

    /// Checks if the service is unlocked
    pub fn is_unlocked(&self) -> bool {
        self.encryption_key.is_some()
    }

    /// Encrypts data (requires unlocked service)
    pub fn encrypt_data(&self, plaintext: &[u8], context: &[u8]) -> Result<Vec<u8>, String> {
        let key = self.encryption_key
            .as_ref()
            .ok_or("CryptoService is locked")?;

        encrypt(key, plaintext, context)
            .map_err(|_| "Encryption failed".to_string())
    }

    /// Decrypts data (requires unlocked service)
    pub fn decrypt_data(&self, ciphertext: &[u8], context: &[u8]) -> Result<Vec<u8>, String> {
        let key = self.encryption_key
            .as_ref()
            .ok_or("CryptoService is locked")?;

        decrypt(key, ciphertext, context)
            .map(|zeroizing_vec| zeroizing_vec.to_vec())
            .map_err(|_| "Decryption failed".to_string())
    }
}

impl Default for CryptoService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_service_lifecycle() {
        let mut service = CryptoService::new();
        assert!(!service.is_unlocked());

        let password = "TestPassword123!";
        let salt = generate_salt().unwrap();

        service.unlock(password, &salt).unwrap();
        assert!(service.is_unlocked());

        let plaintext = b"Secret data";
        let context = b"test";

        let encrypted = service.encrypt_data(plaintext, context).unwrap();
        let decrypted = service.decrypt_data(&encrypted, context).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());

        service.lock();
        assert!(!service.is_unlocked());
    }
}
