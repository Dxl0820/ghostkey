mod storage;
pub(crate) mod crypto;
pub(crate) mod key;
mod migration;

use std::path::PathBuf;
use uuid::Uuid;
use crate::error::{Error, Result};
use crate::models::{Environment, Project, Secret};
use crate::config;
use storage::VaultData;

pub struct Vault {
    path: PathBuf,
    key: Option<[u8; 32]>,
    salt: Vec<u8>,
    data: VaultData,
}

impl Vault {
    pub fn init() -> Result<Self> {
        config::ensure_ghostkey_dir()?;
        let path = config::get_ghostkey_dir().join("vault.enc");

        if path.exists() {
            return Err(Error::VaultAlreadyInitialized(path));
        }

        let vault = Self {
            path,
            key: None,
            salt: key::generate_salt(),
            data: VaultData::default(),
        };

        Ok(vault)
    }

    pub fn open() -> Result<Self> {
        let path = config::get_ghostkey_dir().join("vault.enc");

        if !path.exists() {
            return Err(Error::VaultNotInitialized);
        }

        Ok(Self {
            path,
            key: None,
            salt: Vec::new(),
            data: VaultData::default(),
        })
    }

    pub fn unlock(&mut self) -> Result<()> {
        if self.key.is_some() {
            return Ok(());
        }

        let password = rpassword::prompt_password("Master password: ")?;
        let password_str = password.to_string();

        let (key, data) = storage::load_vault(&self.path, &password_str)?;
        self.key = Some(key);
        self.salt = std::fs::read(&self.path)
            .ok()
            .and_then(|content| {
                let vault_file: serde_json::Value = serde_json::from_slice(&content).ok()?;
                let salt_arr = vault_file.get("salt")?.as_array()?;
                Some(salt_arr.iter().filter_map(|v| v.as_u64().map(|n| n as u8)).collect())
            })
            .unwrap_or_default();
        self.data = data;

        Ok(())
    }

    pub fn unlock_with_password(&mut self, password: &str) -> Result<()> {
        if self.key.is_some() {
            return Ok(());
        }

        let (key, data) = storage::load_vault(&self.path, password)?;
        self.key = Some(key);
        self.salt = std::fs::read(&self.path)
            .ok()
            .and_then(|content| {
                let vault_file: serde_json::Value = serde_json::from_slice(&content).ok()?;
                let salt_arr = vault_file.get("salt")?.as_array()?;
                Some(salt_arr.iter().filter_map(|v| v.as_u64().map(|n| n as u8)).collect())
            })
            .unwrap_or_default();
        self.data = data;

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let key = self.key.ok_or(Error::VaultLocked)?;
        storage::save_vault(&self.path, &key, &self.salt, &self.data)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn is_locked(&self) -> bool {
        self.key.is_none()
    }

    pub fn lock(&mut self) {
        self.key = None;
        self.data = VaultData::default();
    }

    // --- Project operations ---

    pub fn list_projects(&self) -> Result<Vec<Project>> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }
        Ok(self.data.projects.clone())
    }

    pub fn get_project(&self, id: Uuid) -> Result<Project> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }
        self.data.projects.iter().find(|p| p.id == id)
            .cloned()
            .ok_or_else(|| Error::ProjectNotFound(id.to_string()))
    }

    pub fn get_project_by_name(&self, name: &str) -> Result<Project> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }
        self.data.projects.iter().find(|p| p.name == name)
            .cloned()
            .ok_or_else(|| Error::ProjectNotFound(name.to_string()))
    }

    pub fn create_project(&mut self, name: String, description: Option<String>) -> Result<Project> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        if self.data.projects.iter().any(|p| p.name == name) {
            return Err(Error::ProjectNotFound(name));
        }

        let project = Project::new(name, description);
        let result = project.clone();
        self.data.projects.push(project);
        self.save()?;

        Ok(result)
    }

    pub fn delete_project(&mut self, id: Uuid) -> Result<()> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let idx = self.data.projects.iter().position(|p| p.id == id)
            .ok_or_else(|| Error::ProjectNotFound(id.to_string()))?;

        let env_ids: Vec<Uuid> = self.data.environments.iter()
            .filter(|e| e.project_id == id)
            .map(|e| e.id)
            .collect();

        self.data.environments.retain(|e| e.project_id != id);
        self.data.secrets.retain(|s| !env_ids.contains(&s.environment_id));
        self.data.projects.remove(idx);

        self.save()?;
        Ok(())
    }

    // --- Environment operations ---

    pub fn list_environments(&self, project_id: Uuid) -> Result<Vec<Environment>> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }
        Ok(self.data.environments.iter().filter(|e| e.project_id == project_id).cloned().collect())
    }

    pub fn create_environment(&mut self, project_id: Uuid, name: String) -> Result<Environment> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        if !self.data.projects.iter().any(|p| p.id == project_id) {
            return Err(Error::ProjectNotFound(project_id.to_string()));
        }

        let env = Environment::new(project_id, name);
        let result = env.clone();
        self.data.environments.push(env);
        self.save()?;

        Ok(result)
    }

    // --- Secret operations ---

    pub fn list_secrets(&self, env_id: Uuid) -> Result<Vec<Secret>> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }
        Ok(self.data.secrets.iter().filter(|s| s.environment_id == env_id).cloned().collect())
    }

    pub fn get_secret(&self, id: Uuid) -> Result<Secret> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }
        self.data.secrets.iter().find(|s| s.id == id)
            .cloned()
            .ok_or_else(|| Error::SecretNotFound(id.to_string()))
    }

    pub fn get_secret_value(&self, id: Uuid) -> Result<String> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let secret = self.data.secrets.iter().find(|s| s.id == id)
            .ok_or_else(|| Error::SecretNotFound(id.to_string()))?;

        let key = self.key.unwrap();
        let decrypted = crypto::decrypt(&secret.encrypted_value, &key, &secret.nonce)?;

        String::from_utf8(decrypted)
            .map_err(|_| Error::DecryptionError("Invalid UTF-8 in secret".to_string()))
    }

    pub fn create_secret(
        &mut self,
        env_id: Uuid,
        key: String,
        value: String,
        description: Option<String>,
    ) -> Result<()> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        if !self.data.environments.iter().any(|e| e.id == env_id) {
            return Err(Error::EnvironmentNotFound(env_id.to_string()));
        }

        if self.data.secrets.iter().any(|s| s.environment_id == env_id && s.key == key) {
            return Err(Error::SecretAlreadyExists(key));
        }

        let master_key = self.key.unwrap();
        let (encrypted_value, nonce) = crypto::encrypt(value.as_bytes(), &master_key)?;

        let secret = Secret::new(env_id, key, encrypted_value, nonce, description);
        self.data.secrets.push(secret);
        self.save()?;

        Ok(())
    }

    pub fn update_secret(
        &mut self,
        id: Uuid,
        value: Option<String>,
        description: Option<Option<String>>,
    ) -> Result<()> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let secret = self.data.secrets.iter_mut().find(|s| s.id == id)
            .ok_or_else(|| Error::SecretNotFound(id.to_string()))?;

        let master_key = self.key.unwrap();
        let mut changed = false;

        if let Some(v) = value {
            let (encrypted, nonce) = crypto::encrypt(v.as_bytes(), &master_key)?;
            secret.encrypted_value = encrypted;
            secret.nonce = nonce;
            changed = true;
        }

        if let Some(d) = description {
            secret.description = d;
            changed = true;
        }

        if changed {
            secret.updated_at = chrono::Utc::now();
            self.save()?;
        }

        Ok(())
    }

    pub fn delete_secret(&mut self, id: Uuid) -> Result<()> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let idx = self.data.secrets.iter().position(|s| s.id == id)
            .ok_or_else(|| Error::SecretNotFound(id.to_string()))?;

        self.data.secrets.remove(idx);
        self.save()?;
        Ok(())
    }

    // --- Password management ---

    pub fn save_with_password(&self, password: &str) -> Result<()> {
        let salt = key::generate_salt();
        let derived_key = key::derive_key(password, &salt)?;
        storage::save_vault(&self.path, &derived_key, &salt, &self.data)?;
        Ok(())
    }

    pub fn change_password(&self, new_password: &str) -> Result<()> {
        let _key = self.key.ok_or(Error::VaultLocked)?;
        let salt = key::generate_salt();
        let new_key = key::derive_key(new_password, &salt)?;
        storage::save_vault(&self.path, &new_key, &salt, &self.data)?;
        Ok(())
    }

    // --- Export helpers ---

    pub fn export_secrets_as_env(&self, env_id: Uuid) -> Result<String> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let master_key = self.key.unwrap();
        let mut lines = Vec::new();

        for secret in self.data.secrets.iter().filter(|s| s.environment_id == env_id) {
            let decrypted = crypto::decrypt(&secret.encrypted_value, &master_key, &secret.nonce)?;
            let value = String::from_utf8(decrypted)
                .map_err(|_| Error::DecryptionError("Invalid UTF-8".to_string()))?;
            lines.push(format!("{}={}", secret.key, value));
        }

        Ok(lines.join("\n"))
    }

    pub fn import_secret_from_env(&mut self, env_id: Uuid, key: &str, value: &str) -> Result<()> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        if !self.data.environments.iter().any(|e| e.id == env_id) {
            return Err(Error::EnvironmentNotFound(env_id.to_string()));
        }

        if let Some(existing) = self.data.secrets.iter_mut().find(|s| s.environment_id == env_id && s.key == key) {
            let master_key = self.key.unwrap();
            let (encrypted, nonce) = crypto::encrypt(value.as_bytes(), &master_key)?;
            existing.encrypted_value = encrypted;
            existing.nonce = nonce;
            existing.updated_at = chrono::Utc::now();
        } else {
            let master_key = self.key.unwrap();
            let (encrypted, nonce) = crypto::encrypt(value.as_bytes(), &master_key)?;
            let secret = Secret::new(env_id, key.to_string(), encrypted, nonce, None);
            self.data.secrets.push(secret);
        }

        self.save()?;
        Ok(())
    }

    pub fn get_all_secrets_decrypted(&self, env_id: Uuid) -> Result<Vec<(String, String)>> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let master_key = self.key.unwrap();
        let mut result = Vec::new();

        for secret in self.data.secrets.iter().filter(|s| s.environment_id == env_id) {
            let decrypted = crypto::decrypt(&secret.encrypted_value, &master_key, &secret.nonce)?;
            let value = String::from_utf8(decrypted)
                .map_err(|_| Error::DecryptionError("Invalid UTF-8".to_string()))?;
            result.push((secret.key.clone(), value));
        }

        Ok(result)
    }
}
