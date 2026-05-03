use crate::error::{Error, Result};
use crate::vault::Vault;
use crate::config;

pub fn execute() -> Result<()> {
    println!("Initializing GhostKey vault...");
    println!();

    let vault = Vault::init()?;

    // Prompt for master password
    let password = rpassword::prompt_password("Create master password: ")?;
    let password_str = password.to_string();

    if password_str.len() < 8 {
        return Err(Error::ConfigError(
            "Password must be at least 8 characters".to_string()
        ));
    }

    let confirm = rpassword::prompt_password("Confirm master password: ")?;
    let confirm_str = confirm.to_string();

    if password_str != confirm_str {
        return Err(Error::ConfigError("Passwords do not match".to_string()));
    }

    // Save vault with password
    vault.save_with_password(&password_str)?;

    println!();
    println!("✓ Vault initialized successfully");
    println!("  Location: {}", vault.path().display());
    println!();
    println!("Next steps:");
    println!("  1. Add your first credential: ghostkey add <name>");
    println!("  2. List credentials: ghostkey list");
    println!("  3. Get help: ghostkey --help");
    println!();
    println!("⚠  Remember your master password! It cannot be recovered.");

    Ok(())
}

pub fn change_password() -> Result<()> {
    let mut vault = Vault::open()?;

    println!("Changing master password...");
    println!();

    // Unlock with current password
    vault.unlock()?;

    // Prompt for new password
    let new_password = rpassword::prompt_password("New master password: ")?;
    let new_password_str = new_password.to_string();

    if new_password_str.len() < 8 {
        return Err(Error::ConfigError(
            "Password must be at least 8 characters".to_string()
        ));
    }

    let confirm = rpassword::prompt_password("Confirm new password: ")?;
    let confirm_str = confirm.to_string();

    if new_password_str != confirm_str {
        return Err(Error::ConfigError("Passwords do not match".to_string()));
    }

    // Change password
    vault.change_password(&new_password_str)?;

    println!();
    println!("✓ Master password changed successfully");

    Ok(())
}

pub fn status() -> Result<()> {
    let ghostkey_dir = config::get_ghostkey_dir();

    println!("GhostKey Status");
    println!("  Location: {}", ghostkey_dir.display());

    if !ghostkey_dir.exists() {
        println!("  Status: Not initialized");
        println!();
        println!("Run 'ghostkey init' to create a new vault.");
        return Ok(());
    }

    let vault_path = ghostkey_dir.join("vault.enc");
    if !vault_path.exists() {
        println!("  Status: No vault found");
        println!();
        println!("Run 'ghostkey init' to create a new vault.");
        return Ok(());
    }

    println!("  Status: Vault exists");

    // Try to load vault info
    match Vault::open() {
        Ok(mut vault) => {
            match vault.unlock() {
                Ok(_) => {
                    let credentials = vault.list_credentials(None)?;
                    let tags = vault.list_tags()?;
                    println!("  Credentials: {}", credentials.len());
                    println!("  Tags: {}", tags.len());
                }
                Err(_) => {
                    println!("  Status: Locked");
                }
            }
        }
        Err(_) => {
            println!("  Status: Error reading vault");
        }
    }

    Ok(())
}
