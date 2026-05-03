use crate::error::Result;
use crate::vault::Vault;
use crate::models::CredentialType;

pub fn execute(
    tag: Option<&str>,
    type_filter: Option<&crate::cli::CredentialTypeArg>,
    json: bool,
    names_only: bool,
) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    let mut credentials = vault.list_credentials(tag)?;

    // Apply type filter
    if let Some(type_arg) = type_filter {
        let target_type = match type_arg {
            crate::cli::CredentialTypeArg::Password => CredentialType::Password,
            crate::cli::CredentialTypeArg::ApiKey => CredentialType::ApiKey,
            crate::cli::CredentialTypeArg::Ssh => CredentialType::SshKey,
            crate::cli::CredentialTypeArg::Token => CredentialType::Token,
            crate::cli::CredentialTypeArg::Env => CredentialType::EnvVar,
            crate::cli::CredentialTypeArg::Custom => CredentialType::Custom("custom".to_string()),
        };
        credentials.retain(|c| std::mem::discriminant(&c.credential_type) == std::mem::discriminant(&target_type));
    }

    if credentials.is_empty() {
        if names_only {
            return Ok(());
        }
        println!("No credentials found");
        if tag.is_some() {
            println!("  Try removing the tag filter");
        } else {
            println!("  Add your first credential: ghostkey add <name>");
        }
        return Ok(());
    }

    // Sort by name
    credentials.sort_by(|a, b| a.name.cmp(&b.name));

    if names_only {
        for cred in credentials {
            println!("{}", cred.name);
        }
        return Ok(());
    }

    if json {
        let items: Vec<serde_json::Value> = credentials.iter().map(|cred| {
            serde_json::json!({
                "name": cred.name,
                "type": cred.credential_type.to_string(),
                "username": cred.metadata.username,
                "tags": cred.metadata.tags,
                "updated_at": cred.updated_at.to_rfc3339(),
            })
        }).collect();

        println!("{}", serde_json::to_string_pretty(&items)?);
        return Ok(());
    }

    // Default output
    println!("Credentials:");
    println!();

    for cred in &credentials {
        println!("  {}", cred.name);
        println!("    Type: {}", cred.credential_type);
        if let Some(username) = &cred.metadata.username {
            println!("    Username: {}", username);
        }
        if !cred.metadata.tags.is_empty() {
            println!("    Tags: {}", cred.metadata.tags.join(", "));
        }
        println!("    Updated: {}", cred.updated_at.format("%Y-%m-%d %H:%M:%S"));
        println!();
    }

    println!("Total: {} credential(s)", credentials.len());

    Ok(())
}
