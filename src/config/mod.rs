use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::error::Result;

const CONFIG_FILE: &str = "config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub vault_path: PathBuf,
    pub auto_lock_timeout: u64,
    pub clipboard_timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vault_path: get_ghostkey_dir().join("vault.enc"),
            auto_lock_timeout: 300,
            clipboard_timeout: 30,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_ghostkey_dir().join(CONFIG_FILE);

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_ghostkey_dir().join(CONFIG_FILE);
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }
}

pub fn get_ghostkey_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".ghostkey")
}

pub fn ensure_ghostkey_dir() -> Result<PathBuf> {
    let dir = get_ghostkey_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}
