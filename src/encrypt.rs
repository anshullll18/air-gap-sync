use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM with 256-bit key
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use rand::RngCore;
use sha2::{Sha256, Digest};

/// Derives a 256-bit key from password using SHA256
pub fn derive_key_from_password(password: &str) -> Key<Aes256Gcm> {
    let hash = Sha256::digest(password.as_bytes());
    Key::<Aes256Gcm>::from_slice(&hash).clone()
}

/// Encrypts data using AES-GCM
pub fn encrypt(data: &[u8], password: &str) -> (Vec<u8>, [u8; 12]) {
    let key = derive_key_from_password(password);
    let cipher = Aes256Gcm::new(&key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, data).expect("encryption failed");
    (ciphertext, nonce_bytes)
}

/// Decrypts AES-GCM encrypted data
pub fn decrypt(ciphertext: &[u8], nonce_bytes: [u8; 12], password: &str) -> Vec<u8> {
    let key = derive_key_from_password(password);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    cipher.decrypt(nonce, ciphertext).expect("decryption failed")
}