use crate::error::Result;
use crate::vault::Vault;

pub fn execute(query: &str, json: bool) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    let credentials = vault.list_credentials(None)?;
    let query_lower = query.to_lowercase();

    let results: Vec<_> = credentials.iter()
        .filter(|c| {
            c.name.to_lowercase().contains(&query_lower)
                || c.metadata.description.as_deref().unwrap_or("").to_lowercase().contains(&query_lower)
                || c.metadata.username.as_deref().unwrap_or("").to_lowercase().contains(&query_lower)
                || c.metadata.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
                || c.metadata.url.as_deref().unwrap_or("").to_lowercase().contains(&query_lower)
        })
        .collect();

    if results.is_empty() {
        println!("No credentials found matching '{}'", query);
        return Ok(());
    }

    if json {
        let items: Vec<serde_json::Value> = results.iter().map(|cred| {
            serde_json::json!({
                "name": cred.name,
                "type": cred.credential_type.to_string(),
                "username": cred.metadata.username,
                "tags": cred.metadata.tags,
                "description": cred.metadata.description,
            })
        }).collect();

        println!("{}", serde_json::to_string_pretty(&items)?);
        return Ok(());
    }

    println!("Search results for '{}':", query);
    println!();

    for cred in &results {
        println!("  {}", cred.name);
        println!("    Type: {}", cred.credential_type);
        if let Some(username) = &cred.metadata.username {
            println!("    Username: {}", username);
        }
        if let Some(description) = &cred.metadata.description {
            println!("    Description: {}", description);
        }
        if !cred.metadata.tags.is_empty() {
            println!("    Tags: {}", cred.metadata.tags.join(", "));
        }
        println!();
    }

    println!("Found: {} credential(s)", results.len());

    Ok(())
}
