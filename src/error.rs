use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Vault not initialized. Run 'ghostkey init' first")]
    VaultNotInitialized,

    #[error("Vault already initialized at {0}")]
    VaultAlreadyInitialized(PathBuf),

    #[error("Vault is locked")]
    VaultLocked,

    #[error("Invalid master password")]
    InvalidPassword,

    #[error("Project '{0}' not found")]
    ProjectNotFound(String),

    #[error("Environment '{0}' not found")]
    EnvironmentNotFound(String),

    #[error("Secret '{0}' not found")]
    SecretNotFound(String),

    #[error("Secret '{0}' already exists in this environment")]
    SecretAlreadyExists(String),

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
}

pub type Result<T> = std::result::Result<T, Error>;
