use crate::error::Result;
use crate::vault::Vault;
use crate::models::CredentialType;

pub fn execute(action: &str, name: Option<&str>) -> Result<()> {
    match action {
        "list" => list_ssh_keys(),
        "config" => generate_ssh_config(),
        "add" => {
            if let Some(name) = name {
                add_ssh_key(name)
            } else {
                Err(crate::error::Error::ConfigError("Name required for ssh add".to_string()))
            }
        }
        _ => Err(crate::error::Error::ConfigError(format!("Unknown ssh action: {}", action))),
    }
}

fn list_ssh_keys() -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    let credentials = vault.list_credentials(None)?;
    let ssh_keys: Vec<_> = credentials.iter()
        .filter(|c| matches!(c.credential_type, CredentialType::SshKey))
        .collect();

    if ssh_keys.is_empty() {
        println!("No SSH keys found");
        println!("  Add an SSH key: ghostkey ssh add <name>");
        return Ok(());
    }

    println!("SSH Keys:");
    println!();

    for key in ssh_keys {
        println!("  {}", key.name);
        if let Some(username) = &key.metadata.username {
            println!("    User: {}", username);
        }
        if let Some(description) = &key.metadata.description {
            println!("    Description: {}", description);
        }
        if !key.metadata.tags.is_empty() {
            println!("    Tags: {}", key.metadata.tags.join(", "));
        }
        println!();
    }

    Ok(())
}

fn generate_ssh_config() -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    let credentials = vault.list_credentials(None)?;
    let ssh_keys: Vec<_> = credentials.iter()
        .filter(|c| matches!(c.credential_type, CredentialType::SshKey))
        .collect();

    if ssh_keys.is_empty() {
        println!("No SSH keys found");
        return Ok(());
    }

    println!("# GhostKey SSH Config");
    println!("# Generated at: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));
    println!("# Add this to ~/.ssh/config");
    println!();

    for key in ssh_keys {
        let secret = vault.get_secret(&key.name)?;
        let host = key.metadata.url.as_deref().unwrap_or(&key.name);

        println!("Host {}", host);
        println!("  IdentityFile {}", secret);

        if let Some(username) = &key.metadata.username {
            println!("  User {}", username);
        }

        // Parse additional options from custom_fields
        for (k, v) in &key.metadata.custom_fields {
            match k.as_str() {
                "port" => println!("  Port {}", v),
                "proxy" => println!("  ProxyJump {}", v),
                "forward_agent" => println!("  ForwardAgent {}", v),
                "add_keys_to_agent" => println!("  AddKeysToAgent {}", v),
                _ => println!("  {} {}", k, v),
            }
        }

        println!();
    }

    Ok(())
}

fn add_ssh_key(name: &str) -> Result<()> {
    println!("Adding SSH key: {}", name);
    println!();

    let key_path = crate::utils::input::prompt("SSH key path (e.g., ~/.ssh/id_ed25519)")?;
    let username = crate::utils::input::prompt_optional("Username (optional)")?;
    let host = crate::utils::input::prompt_optional("Host/URL (optional)")?;
    let description = crate::utils::input::prompt_optional("Description (optional)")?;
    let tags_input = crate::utils::input::prompt_optional("Tags (comma-separated, optional)")?;
    let tags: Vec<String> = tags_input
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_else(|| vec!["ssh".to_string()]);

    // Additional SSH options
    let mut custom_fields = std::collections::HashMap::new();

    let port = crate::utils::input::prompt_optional("Port (optional, default 22)")?;
    if let Some(p) = port {
        custom_fields.insert("port".to_string(), p);
    }

    let proxy = crate::utils::input::prompt_optional("ProxyJump (optional)")?;
    if let Some(p) = proxy {
        custom_fields.insert("proxy".to_string(), p);
    }

    let forward_agent = crate::utils::input::prompt_optional("ForwardAgent (yes/no, optional)")?;
    if let Some(f) = forward_agent {
        custom_fields.insert("forward_agent".to_string(), f);
    }

    let mut vault = Vault::open()?;
    vault.unlock()?;

    vault.add_credential_with_metadata(
        name,
        CredentialType::SshKey,
        &key_path,
        username,
        description,
        tags,
        host,
        custom_fields,
    )?;

    println!("✓ SSH key '{}' added successfully", name);
    println!();
    println!("Generate SSH config: ghostkey ssh config");

    Ok(())
}
