use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Vault not initialized. Run 'ghostkey init' first")]
    VaultNotInitialized,

    #[error("Vault already initialized at {0}")]
    VaultAlreadyInitialized(PathBuf),

    #[error("Vault is locked. Run 'ghostkey' and enter your master password")]
    VaultLocked,

    #[error("Invalid master password")]
    InvalidPassword,

    #[error("Credential '{0}' not found")]
    CredentialNotFound(String),

    #[error("Credential '{0}' already exists")]
    CredentialAlreadyExists(String),

    #[error("Tag '{0}' not found on credential '{1}'")]
    TagNotFound(String, String),

    #[error("Invalid credential name: {0}")]
    InvalidCredentialName(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
