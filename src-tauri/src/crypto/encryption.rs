use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use zeroize::Zeroizing;

/// Encrypts data using AES-256-GCM
/// 
/// # Arguments
/// * `key` - 32-byte encryption key
/// * `plaintext` - Data to encrypt
/// * `aad` - Additional Authenticated Data (optional context)
/// 
/// # Returns
/// * Encrypted data with nonce prepended (nonce || ciphertext || tag)
pub fn encrypt(
    key: &[u8; 32],
    plaintext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, Unspecified> {
    let unbound_key = UnboundKey::new(&AES_256_GCM, key)?;
    let sealing_key = LessSafeKey::new(unbound_key);

    // Generate random nonce for this encryption
    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes)?;
    let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes)?;

    // Create a copy of plaintext that we can mutate
    let mut in_out = plaintext.to_vec();
    
    // Encrypt in place
    sealing_key.seal_in_place_append_tag(nonce, Aad::from(aad), &mut in_out)?;

    // Prepend nonce to ciphertext: [nonce][ciphertext+tag]
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&in_out);

    Ok(result)
}

/// Decrypts data using AES-256-GCM
/// 
/// # Arguments
/// * `key` - 32-byte encryption key
/// * `encrypted` - Encrypted data with nonce prepended
/// * `aad` - Additional Authenticated Data (must match encryption AAD)
/// 
/// # Returns
/// * Decrypted plaintext
pub fn decrypt(
    key: &[u8; 32],
    encrypted: &[u8],
    aad: &[u8],
) -> Result<Zeroizing<Vec<u8>>, Unspecified> {
    if encrypted.len() < 12 {
        return Err(Unspecified);
    }

    // Extract nonce and ciphertext
    let (nonce_bytes, ciphertext) = encrypted.split_at(12);
    let nonce = Nonce::try_assume_unique_for_key(nonce_bytes)?;

    let unbound_key = UnboundKey::new(&AES_256_GCM, key)?;
    let opening_key = LessSafeKey::new(unbound_key);

    // Create mutable copy for in-place decryption
    let mut in_out = ciphertext.to_vec();

    // Decrypt in place
    let plaintext = opening_key.open_in_place(nonce, Aad::from(aad), &mut in_out)?;

    // Return zeroizing vector (will be securely cleared on drop)
    Ok(Zeroizing::new(plaintext.to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [42u8; 32];
        let plaintext = b"Hello, Clerk!";
        let aad = b"context";

        let encrypted = encrypt(&key, plaintext, aad).unwrap();
        let decrypted = decrypt(&key, &encrypted, aad).unwrap();

        assert_eq!(&**decrypted, plaintext);
    }

    #[test]
    fn test_wrong_key_fails() {
        let key1 = [1u8; 32];
        let key2 = [2u8; 32];
        let plaintext = b"secret data";
        let aad = b"";

        let encrypted = encrypt(&key1, plaintext, aad).unwrap();
        let result = decrypt(&key2, &encrypted, aad);

        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_aad_fails() {
        let key = [1u8; 32];
        let plaintext = b"secret data";
        let aad1 = b"context1";
        let aad2 = b"context2";

        let encrypted = encrypt(&key, plaintext, aad1).unwrap();
        let result = decrypt(&key, &encrypted, aad2);

        assert!(result.is_err());
    }
}
