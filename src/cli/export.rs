use crate::error::Result;
use crate::vault::Vault;
use crate::cli::ExportFormat;

pub fn execute(format: &ExportFormat, output: Option<&str>, project: Option<&str>, env: &str) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    // Find project
    let project = match project {
        Some(name) => vault.get_project_by_name(name)?,
        None => {
            let projects = vault.list_projects()?;
            projects.into_iter().next()
                .ok_or_else(|| crate::error::Error::ConfigError("No projects found".to_string()))?
        }
    };

    // Find environment
    let environments = vault.list_environments(project.id)?;
    let environment = environments.into_iter().find(|e| e.name == env)
        .ok_or_else(|| crate::error::Error::EnvironmentNotFound(env.to_string()))?;

    let content = match format {
        ExportFormat::Env => vault.export_secrets_as_env(environment.id)?,
        ExportFormat::Json => export_json(&vault, environment.id)?,
        ExportFormat::Csv => export_csv(&vault, environment.id)?,
    };

    match output {
        Some(path) => {
            std::fs::write(path, &content)?;
            println!("Exported to {}", path);
        }
        None => {
            println!("{}", content);
        }
    }

    Ok(())
}

fn export_json(vault: &Vault, env_id: uuid::Uuid) -> Result<String> {
    let secrets = vault.list_secrets(env_id)?;

    let items: Vec<serde_json::Value> = secrets.iter().map(|s| {
        serde_json::json!({
            "key": s.key,
            "description": s.description,
            "created_at": s.created_at.to_rfc3339(),
            "updated_at": s.updated_at.to_rfc3339(),
        })
    }).collect();

    let output = serde_json::json!({
        "exported_at": chrono::Utc::now().to_rfc3339(),
        "secrets": items,
    });

    Ok(serde_json::to_string_pretty(&output)?)
}

fn export_csv(vault: &Vault, env_id: uuid::Uuid) -> Result<String> {
    let secrets = vault.list_secrets(env_id)?;

    let mut lines = vec!["key,description,created_at,updated_at".to_string()];

    for s in &secrets {
        let desc = s.description.as_deref().unwrap_or("");
        lines.push(format!(
            "{},{},{},{}",
            escape_csv(&s.key),
            escape_csv(desc),
            s.created_at.to_rfc3339(),
            s.updated_at.to_rfc3339(),
        ));
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
