use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::models::{Credential, Environment, Project, Secret};
use super::key;
use super::crypto;
use super::migration;

const CURRENT_VERSION: u32 = 2;

#[derive(Serialize, Deserialize)]
struct VaultFile {
    version: u32,
    salt: Vec<u8>,
    nonce: Vec<u8>,
    encrypted_data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultData {
    pub version: u32,
    pub projects: Vec<Project>,
    pub environments: Vec<Environment>,
    pub secrets: Vec<Secret>,
}

impl Default for VaultData {
    fn default() -> Self {
        Self {
            version: 2,
            projects: Vec::new(),
            environments: Vec::new(),
            secrets: Vec::new(),
        }
    }
}

pub fn load_vault(path: &Path, password: &str) -> Result<([u8; 32], VaultData)> {
    let content = std::fs::read(path)?;
    let vault_file: VaultFile = serde_json::from_slice(&content)?;

    if vault_file.version > CURRENT_VERSION {
        return Err(crate::error::Error::ConfigError(
            format!("Unsupported vault version: {}", vault_file.version)
        ));
    }

    let key = key::derive_key(password, &vault_file.salt)?;
    let decrypted = crypto::decrypt(&vault_file.encrypted_data, &key, &vault_file.nonce)?;

    let data = if vault_file.version <= 1 {
        let credentials: Vec<Credential> = serde_json::from_slice(&decrypted)?;
        migration::migrate_v1_to_v2(credentials)?
    } else {
        serde_json::from_slice(&decrypted)?
    };

    Ok((key, data))
}

pub fn save_vault(path: &Path, key: &[u8; 32], salt: &[u8], data: &VaultData) -> Result<()> {
    let payload = serde_json::to_vec(data)?;
    let nonce = crypto::generate_nonce();
    let encrypted = crypto::encrypt_with_nonce(&payload, key, &nonce)?;

    let vault_file = VaultFile {
        version: CURRENT_VERSION,
        salt: salt.to_vec(),
        nonce,
        encrypted_data: encrypted,
    };

    let content = serde_json::to_vec_pretty(&vault_file)?;

    let temp_path = path.with_extension("tmp");
    std::fs::write(&temp_path, &content)?;
    std::fs::rename(&temp_path, path)?;

    Ok(())
}
