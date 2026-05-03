use crate::error::Result;
use crate::models::CredentialType;
use crate::vault::Vault;

pub fn execute(
    name: &str,
    type_arg: Option<&crate::cli::CredentialTypeArg>,
    username: Option<&str>,
    description: Option<&str>,
    tags: Vec<String>,
    url: Option<&str>,
) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    println!("Adding credential: {}", name);
    println!();

    let credential_type = match type_arg {
        Some(crate::cli::CredentialTypeArg::Password) => CredentialType::Password,
        Some(crate::cli::CredentialTypeArg::ApiKey) => CredentialType::ApiKey,
        Some(crate::cli::CredentialTypeArg::Ssh) => CredentialType::SshKey,
        Some(crate::cli::CredentialTypeArg::Token) => CredentialType::Token,
        Some(crate::cli::CredentialTypeArg::Env) => CredentialType::EnvVar,
        Some(crate::cli::CredentialTypeArg::Custom) => CredentialType::Custom("custom".to_string()),
        None => prompt_credential_type()?,
    };

    let username = match username {
        Some(u) => Some(u.to_string()),
        None => prompt_optional("Username (optional)")?,
    };

    let description = match description {
        Some(d) => Some(d.to_string()),
        None => prompt_optional("Description (optional)")?,
    };

    let url = match url {
        Some(u) => Some(u.to_string()),
        None => prompt_optional("URL (optional)")?,
    };

    let secret = rpassword::prompt_password("Secret: ")?;
    let secret_str = secret.to_string();

    vault.add_credential(
        name,
        credential_type,
        &secret_str,
        username,
        description,
        tags,
    )?;

    println!();
    println!("✓ Credential '{}' added successfully", name);

    Ok(())
}

fn prompt_credential_type() -> Result<CredentialType> {
    println!("Select credential type:");
    println!("  1. Password");
    println!("  2. API Key");
    println!("  3. SSH Key");
    println!("  4. Token");
    println!("  5. Environment Variable");
    println!("  6. Custom");
    println!();

    let choice = prompt("Choice [1-6]")?;
    match choice.trim() {
        "1" => Ok(CredentialType::Password),
        "2" => Ok(CredentialType::ApiKey),
        "3" => Ok(CredentialType::SshKey),
        "4" => Ok(CredentialType::Token),
        "5" => Ok(CredentialType::EnvVar),
        "6" => {
            let custom = prompt("Custom type name")?;
            Ok(CredentialType::Custom(custom))
        }
        _ => {
            eprintln!("Invalid choice, defaulting to Password");
            Ok(CredentialType::Password)
        }
    }
}

fn prompt(message: &str) -> Result<String> {
    print!("{}: ", message);
    use std::io::{self, Write};
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_optional(message: &str) -> Result<Option<String>> {
    let input = prompt(message)?;
    if input.is_empty() {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}
