use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use crate::error::{Error, Result};

const NONCE_SIZE: usize = 12;

pub fn generate_nonce() -> Vec<u8> {
    use aes_gcm::aead::rand_core::RngCore;
    let mut nonce = vec![0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<(Vec<u8>, Vec<u8>)> {
    let nonce = generate_nonce();
    let encrypted = encrypt_with_nonce(data, key, &nonce)?;
    Ok((encrypted, nonce))
}

pub fn encrypt_with_nonce(data: &[u8], key: &[u8; 32], nonce: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| Error::EncryptionError(e.to_string()))?;

    let nonce = Nonce::from_slice(nonce);
    cipher.encrypt(nonce, data)
        .map_err(|e| Error::EncryptionError(e.to_string()))
}

pub fn decrypt(data: &[u8], key: &[u8; 32], nonce: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| Error::DecryptionError(e.to_string()))?;

    let nonce = Nonce::from_slice(nonce);
    cipher.decrypt(nonce, data)
        .map_err(|e| Error::DecryptionError(e.to_string()))
}
