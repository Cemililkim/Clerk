use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params, Version,
};
use ring::rand::{SecureRandom, SystemRandom};
use zeroize::Zeroizing;

/// Derives a 32-byte encryption key from a password using Argon2id
/// 
/// # Arguments
/// * `password` - User's master password
/// * `salt` - 16-byte random salt (unique per vault)
/// 
/// # Returns
/// * 32-byte encryption key suitable for AES-256-GCM
pub fn derive_key(password: &str, salt: &[u8; 16]) -> Result<[u8; 32], argon2::Error> {
    // Argon2id parameters (OWASP recommendations for sensitive data)
    // m_cost: 64 MB memory
    // t_cost: 3 iterations
    // p_cost: 4 parallel lanes
    let params = Params::new(65536, 3, 4, Some(32))?;
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        Version::V0x13,
        params,
    );

    let mut key = Zeroizing::new([0u8; 32]);
    
    argon2.hash_password_into(
        password.as_bytes(),
        salt,
        &mut *key,
    )?;

    Ok(*key)
}

/// Hashes a password for verification purposes (not for encryption)
/// Returns a PHC string format hash
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let rng = SystemRandom::new();
    let mut salt_bytes = [0u8; 16];
    rng.fill(&mut salt_bytes)
        .map_err(|_| argon2::password_hash::Error::Password)?;

    let salt = SaltString::encode_b64(&salt_bytes)
        .map_err(|_| argon2::password_hash::Error::Password)?;

    let params = Params::new(65536, 3, 4, Some(32))
        .map_err(|_| argon2::password_hash::Error::ParamNameInvalid)?;
    
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        Version::V0x13,
        params,
    );

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

/// Verifies a password against a stored hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(e),
    }
}

/// Generates a cryptographically secure random salt
pub fn generate_salt() -> Result<[u8; 16], ring::error::Unspecified> {
    let rng = SystemRandom::new();
    let mut salt = [0u8; 16];
    rng.fill(&mut salt)?;
    Ok(salt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key() {
        let password = "MySecurePassword123!";
        let salt = [1u8; 16];

        let key1 = derive_key(password, &salt).unwrap();
        let key2 = derive_key(password, &salt).unwrap();

        // Same password + salt = same key
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_different_salt_different_key() {
        let password = "MySecurePassword123!";
        let salt1 = [1u8; 16];
        let salt2 = [2u8; 16];

        let key1 = derive_key(password, &salt1).unwrap();
        let key2 = derive_key(password, &salt2).unwrap();

        // Different salt = different key
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_hash_and_verify_password() {
        let password = "TestPassword123!";
        
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("WrongPassword", &hash).unwrap());
    }

    #[test]
    fn test_generate_salt() {
        let salt1 = generate_salt().unwrap();
        let salt2 = generate_salt().unwrap();

        // Random salts should be different
        assert_ne!(salt1, salt2);
    }
}
