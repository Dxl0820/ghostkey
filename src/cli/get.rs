use crate::error::Result;
use crate::vault::Vault;

pub fn execute(name: &str, clipboard: bool, show: bool, json: bool) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    let credential = vault.get_credential(name)?;

    if clipboard {
        let secret = vault.get_secret(name)?;
        crate::utils::clipboard::copy_to_clipboard(&secret)?;
        println!("✓ Credential '{}' copied to clipboard", name);
        println!("  Clipboard will be cleared in 30 seconds");
        return Ok(());
    }

    if json {
        let secret = if show {
            Some(vault.get_secret(name)?)
        } else {
            None
        };

        let mut output = serde_json::json!({
            "name": credential.name,
            "type": credential.credential_type.to_string(),
            "username": credential.metadata.username,
            "description": credential.metadata.description,
            "tags": credential.metadata.tags,
            "url": credential.metadata.url,
            "created_at": credential.created_at.to_rfc3339(),
            "updated_at": credential.updated_at.to_rfc3339(),
        });

        if let Some(s) = secret {
            output["secret"] = serde_json::Value::String(s);
        }

        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    // Default output
    println!("Credential: {}", credential.name);
    println!("  Type: {}", credential.credential_type);
    if let Some(username) = &credential.metadata.username {
        println!("  Username: {}", username);
    }
    if let Some(description) = &credential.metadata.description {
        println!("  Description: {}", description);
    }
    if let Some(url) = &credential.metadata.url {
        println!("  URL: {}", url);
    }
    if !credential.metadata.tags.is_empty() {
        println!("  Tags: {}", credential.metadata.tags.join(", "));
    }
    println!("  Created: {}", credential.created_at.format("%Y-%m-%d %H:%M:%S"));
    println!("  Updated: {}", credential.updated_at.format("%Y-%m-%d %H:%M:%S"));

    if show {
        println!();
        let secret = vault.get_secret(name)?;
        println!("  Secret: {}", secret);
    } else {
        println!();
        println!("  Use --show to display the secret");
    }

    Ok(())
}
