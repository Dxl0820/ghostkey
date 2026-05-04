use crate::error::{Error, Result};
use crate::vault::Vault;

pub fn execute() -> Result<()> {
    println!("Initializing GhostKey vault...");

    let vault = Vault::init()?;

    // Check for password in environment variable (non-interactive mode)
    let password_str = if let Ok(env_password) = std::env::var("GHOSTKEY_PASSWORD") {
        env_password
    } else {
        let password = rpassword::prompt_password("Create master password: ")?;
        let password_str = password.to_string();

        if password_str.len() < 8 {
            return Err(Error::ConfigError("Password must be at least 8 characters".to_string()));
        }

        let confirm = rpassword::prompt_password("Confirm master password: ")?;
        if password_str != confirm.to_string() {
            return Err(Error::ConfigError("Passwords do not match".to_string()));
        }

        password_str
    };

    if password_str.len() < 8 {
        return Err(Error::ConfigError("Password must be at least 8 characters".to_string()));
    }

    vault.save_with_password(&password_str)?;

    println!();
    println!("Vault initialized at {}", vault.path().display());
    println!();
    println!("Next steps:");
    println!("  ghostkey dev          Start the web UI");
    println!("  ghostkey run -- cmd   Run a command with secrets injected");
    println!();
    println!("Remember your master password - it cannot be recovered.");

    Ok(())
}
