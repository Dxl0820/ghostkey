use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::models::Credential;
use super::key;
use super::crypto;

const CURRENT_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
struct VaultFile {
    version: u32,
    salt: Vec<u8>,
    nonce: Vec<u8>,
    encrypted_data: Vec<u8>,
}

pub fn load_vault(path: &Path, password: &str) -> Result<([u8; 32], Vec<Credential>)> {
    let content = std::fs::read(path)?;
    let vault_file: VaultFile = serde_json::from_slice(&content)?;

    if vault_file.version > CURRENT_VERSION {
        return Err(crate::error::Error::ConfigError(
            format!("Unsupported vault version: {}", vault_file.version)
        ));
    }

    let key = key::derive_key(password, &vault_file.salt)?;
    let decrypted = crypto::decrypt(&vault_file.encrypted_data, &key, &vault_file.nonce)?;
    let credentials: Vec<Credential> = serde_json::from_slice(&decrypted)?;

    Ok((key, credentials))
}

pub fn save_vault(path: &Path, key: &[u8; 32], credentials: &[Credential]) -> Result<()> {
    let data = serde_json::to_vec(credentials)?;
    let nonce = crypto::generate_nonce();
    let encrypted = crypto::encrypt_with_nonce(&data, key, &nonce)?;

    let salt = std::fs::read(path)
        .ok()
        .and_then(|content| {
            let vault_file: VaultFile = serde_json::from_slice(&content).ok()?;
            Some(vault_file.salt)
        })
        .unwrap_or_else(|| key::generate_salt());

    let vault_file = VaultFile {
        version: CURRENT_VERSION,
        salt,
        nonce,
        encrypted_data: encrypted,
    };

    let content = serde_json::to_vec_pretty(&vault_file)?;

    let temp_path = path.with_extension("tmp");
    std::fs::write(&temp_path, &content)?;
    std::fs::rename(&temp_path, path)?;

    Ok(())
}
