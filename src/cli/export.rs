use crate::error::Result;
use crate::vault::Vault;
use crate::cli::ExportFormat;

pub fn execute(format: &ExportFormat, output: Option<&str>, tag: Option<&str>) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    let credentials = vault.list_credentials(tag)?;

    let content = match format {
        ExportFormat::Env => export_env(&credentials, &vault)?,
        ExportFormat::Json => export_json(&credentials, &vault)?,
        ExportFormat::Csv => export_csv(&credentials, &vault)?,
    };

    match output {
        Some(path) => {
            std::fs::write(path, &content)?;
            println!("✓ Exported to {}", path);
        }
        None => {
            println!("{}", content);
        }
    }

    Ok(())
}

fn export_env(credentials: &[&crate::models::Credential], vault: &Vault) -> Result<String> {
    let mut lines = Vec::new();
    lines.push("# GhostKey export".to_string());
    lines.push(format!("# Exported at: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")));
    lines.push(String::new());

    for cred in credentials {
        let secret = vault.get_secret(&cred.name)?;
        let name = cred.name.to_uppercase().replace('-', "_");
        lines.push(format!("{}={}", name, secret));
    }

    Ok(lines.join("\n"))
}

fn export_json(credentials: &[&crate::models::Credential], vault: &Vault) -> Result<String> {
    let mut items = Vec::new();

    for cred in credentials {
        let secret = vault.get_secret(&cred.name)?;
        let item = serde_json::json!({
            "name": cred.name,
            "type": cred.credential_type.to_string(),
            "username": cred.metadata.username,
            "description": cred.metadata.description,
            "tags": cred.metadata.tags,
            "url": cred.metadata.url,
            "secret": secret,
            "created_at": cred.created_at.to_rfc3339(),
            "updated_at": cred.updated_at.to_rfc3339(),
        });
        items.push(item);
    }

    let output = serde_json::json!({
        "version": "1.0",
        "exported_at": chrono::Utc::now().to_rfc3339(),
        "credentials": items,
    });

    Ok(serde_json::to_string_pretty(&output)?)
}

fn export_csv(credentials: &[&crate::models::Credential], vault: &Vault) -> Result<String> {
    let mut lines = Vec::new();
    lines.push("name,type,username,description,tags,url,secret".to_string());

    for cred in credentials {
        let secret = vault.get_secret(&cred.name)?;
        let tags = cred.metadata.tags.join(";");
        let line = format!(
            "{},{},{},{},{},{},{}",
            escape_csv(&cred.name),
            escape_csv(&cred.credential_type.to_string()),
            escape_csv(cred.metadata.username.as_deref().unwrap_or("")),
            escape_csv(cred.metadata.description.as_deref().unwrap_or("")),
            escape_csv(&tags),
            escape_csv(cred.metadata.url.as_deref().unwrap_or("")),
            escape_csv(&secret),
        );
        lines.push(line);
    }

    Ok(lines.join("\n"))
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
