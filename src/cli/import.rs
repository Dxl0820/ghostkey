use crate::error::{Error, Result};
use crate::vault::Vault;
use crate::models::CredentialType;
use crate::cli::ImportFormat;

pub fn execute(format: &ImportFormat, file: &str) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    let content = std::fs::read_to_string(file)?;

    let count = match format {
        ImportFormat::Env => import_env(&mut vault, &content)?,
        ImportFormat::Json => import_json(&mut vault, &content)?,
        ImportFormat::Csv => import_csv(&mut vault, &content)?,
    };

    println!("✓ Imported {} credential(s)", count);

    Ok(())
}

fn import_env(vault: &mut Vault, content: &str) -> Result<usize> {
    let mut count = 0;

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse KEY=VALUE
        if let Some((key, value)) = line.split_once('=') {
            let name = key.trim().to_lowercase().replace('_', "-");
            let secret = value.trim();

            if !name.is_empty() && !secret.is_empty() {
                vault.add_credential(
                    &name,
                    CredentialType::EnvVar,
                    secret,
                    None,
                    Some(format!("Imported from env file")),
                    vec!["imported".to_string()],
                )?;
                count += 1;
            }
        }
    }

    Ok(count)
}

fn import_json(vault: &mut Vault, content: &str) -> Result<usize> {
    let data: serde_json::Value = serde_json::from_str(content)?;

    let credentials = data["credentials"]
        .as_array()
        .ok_or_else(|| Error::ConfigError("Invalid JSON format: missing credentials array".to_string()))?;

    let mut count = 0;

    for cred in credentials {
        let name = cred["name"]
            .as_str()
            .ok_or_else(|| Error::ConfigError("Missing credential name".to_string()))?;

        let secret = cred["secret"]
            .as_str()
            .ok_or_else(|| Error::ConfigError("Missing credential secret".to_string()))?;

        let credential_type = match cred["type"].as_str() {
            Some("Password") => CredentialType::Password,
            Some("API Key") => CredentialType::ApiKey,
            Some("SSH Key") => CredentialType::SshKey,
            Some("Token") => CredentialType::Token,
            Some("Environment Variable") => CredentialType::EnvVar,
            Some(other) => CredentialType::Custom(other.to_string()),
            None => CredentialType::Password,
        };

        let username = cred["username"].as_str().map(|s| s.to_string());
        let description = cred["description"].as_str().map(|s| s.to_string());
        let tags = cred["tags"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_else(|| vec!["imported".to_string()]);

        vault.add_credential(
            name,
            credential_type,
            secret,
            username,
            description,
            tags,
        )?;
        count += 1;
    }

    Ok(count)
}

fn import_csv(vault: &mut Vault, content: &str) -> Result<usize> {
    let mut count = 0;
    let mut lines = content.lines();

    // Skip header
    lines.next();

    for line in lines {
        let fields = parse_csv_line(line);

        if fields.len() >= 2 {
            let name = fields[0].trim().to_lowercase().replace('_', "-");
            let secret = fields[6].trim();

            let credential_type = match fields[1].trim() {
                "Password" => CredentialType::Password,
                "API Key" => CredentialType::ApiKey,
                "SSH Key" => CredentialType::SshKey,
                "Token" => CredentialType::Token,
                "Environment Variable" => CredentialType::EnvVar,
                other => CredentialType::Custom(other.to_string()),
            };

            let username = if fields.len() > 2 && !fields[2].trim().is_empty() {
                Some(fields[2].trim().to_string())
            } else {
                None
            };

            let description = if fields.len() > 3 && !fields[3].trim().is_empty() {
                Some(fields[3].trim().to_string())
            } else {
                None
            };

            let tags = if fields.len() > 4 && !fields[4].trim().is_empty() {
                fields[4].split(';').map(|s| s.trim().to_string()).collect()
            } else {
                vec!["imported".to_string()]
            };

            if !name.is_empty() && !secret.is_empty() {
                vault.add_credential(
                    &name,
                    credential_type,
                    secret,
                    username,
                    description,
                    tags,
                )?;
                count += 1;
            }
        }
    }

    Ok(count)
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;

    for ch in line.chars() {
        match ch {
            '"' => {
                if in_quotes {
                    // Check for escaped quote
                    if current_field.ends_with('"') {
                        current_field.pop();
                        current_field.push('"');
                    } else {
                        in_quotes = false;
                    }
                } else {
                    in_quotes = true;
                }
            }
            ',' if !in_quotes => {
                fields.push(current_field.clone());
                current_field.clear();
            }
            _ => {
                current_field.push(ch);
            }
        }
    }

    fields.push(current_field);
    fields
}
