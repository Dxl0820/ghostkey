use std::path::PathBuf;
use crate::error::Result;

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
