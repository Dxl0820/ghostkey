mod storage;
mod crypto;
mod key;

use std::path::PathBuf;
use crate::error::{Error, Result};
use crate::models::{Credential, CredentialType, Metadata};
use crate::config;

pub struct Vault {
    path: PathBuf,
    key: Option<[u8; 32]>,
    credentials: Vec<Credential>,
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
            credentials: Vec::new(),
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
            credentials: Vec::new(),
        })
    }

    pub fn unlock(&mut self) -> Result<()> {
        if self.key.is_some() {
            return Ok(());
        }

        let password = rpassword::prompt_password("Master password: ")?;
        let password_str = password.to_string();

        let (key, credentials) = storage::load_vault(&self.path, &password_str)?;
        self.key = Some(key);
        self.credentials = credentials;

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let key = self.key.ok_or(Error::VaultLocked)?;
        storage::save_vault(&self.path, &key, &self.credentials)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn add_credential(
        &mut self,
        name: &str,
        credential_type: CredentialType,
        secret: &str,
        username: Option<String>,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Result<()> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        if self.credentials.iter().any(|c| c.name == name) {
            return Err(Error::CredentialAlreadyExists(name.to_string()));
        }

        let metadata = Metadata {
            description,
            tags,
            username,
            url: None,
            custom_fields: std::collections::HashMap::new(),
        };

        let key = self.key.unwrap();
        let (encrypted_secret, nonce) = crypto::encrypt(secret.as_bytes(), &key)?;

        let credential = Credential::new(
            name.to_string(),
            credential_type,
            encrypted_secret,
            nonce,
            metadata,
        );

        self.credentials.push(credential);
        self.save()?;

        Ok(())
    }

    pub fn get_credential(&self, name: &str) -> Result<&Credential> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        self.credentials
            .iter()
            .find(|c| c.name == name)
            .ok_or_else(|| Error::CredentialNotFound(name.to_string()))
    }

    pub fn get_secret(&self, name: &str) -> Result<String> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let credential = self.get_credential(name)?;
        let key = self.key.unwrap();
        let decrypted = crypto::decrypt(&credential.encrypted_secret, &key, &credential.nonce)?;

        String::from_utf8(decrypted)
            .map_err(|_| Error::DecryptionError("Invalid UTF-8 in secret".to_string()))
    }

    pub fn list_credentials(&self, tag: Option<&str>) -> Result<Vec<&Credential>> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let filtered: Vec<&Credential> = match tag {
            Some(tag) => self.credentials.iter().filter(|c| c.metadata.tags.contains(&tag.to_string())).collect(),
            None => self.credentials.iter().collect(),
        };

        Ok(filtered)
    }

    pub fn delete_credential(&mut self, name: &str) -> Result<()> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let index = self.credentials.iter().position(|c| c.name == name);
        match index {
            Some(i) => {
                self.credentials.remove(i);
                self.save()?;
                Ok(())
            }
            None => Err(Error::CredentialNotFound(name.to_string())),
        }
    }

    pub fn list_tags(&self) -> Result<Vec<(String, usize)>> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let mut tag_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for cred in &self.credentials {
            for tag in &cred.metadata.tags {
                *tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }

        let mut tags: Vec<(String, usize)> = tag_counts.into_iter().collect();
        tags.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(tags)
    }

    pub fn add_tag(&mut self, credential_name: &str, tag: &str) -> Result<()> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let credential = self.credentials.iter_mut().find(|c| c.name == credential_name);
        match credential {
            Some(c) => {
                c.add_tag(tag.to_string());
                self.save()?;
                Ok(())
            }
            None => Err(Error::CredentialNotFound(credential_name.to_string())),
        }
    }

    pub fn remove_tag(&mut self, credential_name: &str, tag: &str) -> Result<()> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let credential = self.credentials.iter_mut().find(|c| c.name == credential_name);
        match credential {
            Some(c) => {
                if c.remove_tag(tag) {
                    self.save()?;
                    Ok(())
                } else {
                    Err(Error::TagNotFound(tag.to_string(), credential_name.to_string()))
                }
            }
            None => Err(Error::CredentialNotFound(credential_name.to_string())),
        }
    }

    pub fn save_with_password(&self, password: &str) -> Result<()> {
        let salt = key::generate_salt();
        let derived_key = key::derive_key(password, &salt)?;
        storage::save_vault(&self.path, &derived_key, &self.credentials)?;
        Ok(())
    }

    pub fn change_password(&self, new_password: &str) -> Result<()> {
        let _key = self.key.ok_or(Error::VaultLocked)?;
        let salt = key::generate_salt();
        let new_key = key::derive_key(new_password, &salt)?;
        storage::save_vault(&self.path, &new_key, &self.credentials)?;
        Ok(())
    }

    pub fn add_credential_with_metadata(
        &mut self,
        name: &str,
        credential_type: CredentialType,
        secret: &str,
        username: Option<String>,
        description: Option<String>,
        tags: Vec<String>,
        url: Option<String>,
        custom_fields: std::collections::HashMap<String, String>,
    ) -> Result<()> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        if self.credentials.iter().any(|c| c.name == name) {
            return Err(Error::CredentialAlreadyExists(name.to_string()));
        }

        let metadata = Metadata {
            description,
            tags,
            username,
            url,
            custom_fields,
        };

        let key = self.key.unwrap();
        let (encrypted_secret, nonce) = crypto::encrypt(secret.as_bytes(), &key)?;

        let credential = Credential::new(
            name.to_string(),
            credential_type,
            encrypted_secret,
            nonce,
            metadata,
        );

        self.credentials.push(credential);
        self.save()?;

        Ok(())
    }

    pub fn update_credential(
        &mut self,
        name: &str,
        secret: Option<&str>,
        username: Option<Option<&str>>,
        description: Option<Option<&str>>,
        tags: Option<Vec<String>>,
        url: Option<Option<&str>>,
    ) -> Result<()> {
        if self.key.is_none() {
            return Err(Error::VaultLocked);
        }

        let credential = self.credentials.iter_mut().find(|c| c.name == name)
            .ok_or_else(|| Error::CredentialNotFound(name.to_string()))?;

        let key = self.key.unwrap();
        let mut changed = false;

        if let Some(s) = secret {
            let (encrypted, nonce) = crypto::encrypt(s.as_bytes(), &key)?;
            credential.encrypted_secret = encrypted;
            credential.nonce = nonce;
            changed = true;
        }

        if let Some(u) = username {
            credential.metadata.username = u.map(|s| s.to_string());
            changed = true;
        }

        if let Some(d) = description {
            credential.metadata.description = d.map(|s| s.to_string());
            changed = true;
        }

        if let Some(t) = tags {
            credential.metadata.tags = t;
            changed = true;
        }

        if let Some(u) = url {
            credential.metadata.url = u.map(|s| s.to_string());
            changed = true;
        }

        if changed {
            credential.updated_at = chrono::Utc::now();
            self.save()?;
        }

        Ok(())
    }
}
