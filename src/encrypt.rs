use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
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

/// Decrypts encrypted bytes using AES-GCM with password and nonce slice
pub fn decrypt_bytes(data: &[u8], password: &str, nonce_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let hash = Sha256::digest(password.as_bytes());
    let key = Key::<Aes256Gcm>::from_slice(&hash);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, data)
        .map_err(|e| format!("Decryption failed: {:?}", e))
}