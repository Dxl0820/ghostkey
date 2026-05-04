use crate::error::{Error, Result};
use crate::vault::Vault;
use crate::cli::ImportFormat;
use uuid::Uuid;

pub fn execute(format: &ImportFormat, file: &str, project: Option<&str>, env: &str) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    // Find project
    let project = match project {
        Some(name) => vault.get_project_by_name(name)?,
        None => {
            let projects = vault.list_projects()?;
            projects.into_iter().next()
                .ok_or_else(|| Error::ConfigError("No projects found".to_string()))?
        }
    };

    // Find environment and capture ID before mutable borrow
    let environments = vault.list_environments(project.id)?;
    let env_id = environments.iter().find(|e| e.name == env)
        .map(|e| e.id)
        .ok_or_else(|| Error::EnvironmentNotFound(env.to_string()))?;

    let content = std::fs::read_to_string(file)?;

    let count = match format {
        ImportFormat::Env => import_env(&mut vault, env_id, &content)?,
        ImportFormat::Json => import_json(&mut vault, env_id, &content)?,
        ImportFormat::Csv => import_csv(&mut vault, env_id, &content)?,
    };

    println!("Imported {} secret(s)", count);

    Ok(())
}

fn import_env(vault: &mut Vault, env_id: Uuid, content: &str) -> Result<usize> {
    let mut count = 0;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            if !key.is_empty() && !value.is_empty() {
                vault.import_secret_from_env(env_id, key, value)?;
                count += 1;
            }
        }
    }

    Ok(count)
}

fn import_json(vault: &mut Vault, env_id: Uuid, content: &str) -> Result<usize> {
    let data: serde_json::Value = serde_json::from_str(content)?;

    let secrets = data["secrets"]
        .as_array()
        .ok_or_else(|| Error::ConfigError("Invalid JSON format: missing secrets array".to_string()))?;

    let mut count = 0;

    for item in secrets {
        let key = item["key"]
            .as_str()
            .ok_or_else(|| Error::ConfigError("Missing secret key".to_string()))?;

        let value = item["value"]
            .as_str()
            .ok_or_else(|| Error::ConfigError("Missing secret value".to_string()))?;

        vault.import_secret_from_env(env_id, key, value)?;
        count += 1;
    }

    Ok(count)
}

fn import_csv(vault: &mut Vault, env_id: Uuid, content: &str) -> Result<usize> {
    let mut count = 0;
    let mut lines = content.lines();

    // Skip header
    lines.next();

    for line in lines {
        let fields: Vec<&str> = line.splitn(2, ',').collect();
        if fields.len() >= 2 {
            let key = fields[0].trim();
            let value = fields[1].trim();

            if !key.is_empty() && !value.is_empty() {
                vault.import_secret_from_env(env_id, key, value)?;
                count += 1;
            }
        }
    }

    Ok(count)
}
