use crate::error::{Error, Result};
use crate::vault::Vault;

pub fn execute(project: Option<&str>, env: &str, command: &[String]) -> Result<()> {
    if command.is_empty() {
        return Err(Error::ConfigError("No command specified".to_string()));
    }

    let mut vault = Vault::open()?;
    vault.unlock()?;

    // Find project
    let project = match project {
        Some(name) => vault.get_project_by_name(name)?,
        None => {
            let projects = vault.list_projects()?;
            projects.into_iter().next()
                .ok_or_else(|| Error::ConfigError("No projects found. Run 'ghostkey init' first.".to_string()))?
        }
    };

    // Find environment
    let environments = vault.list_environments(project.id)?;
    let environment = environments.into_iter().find(|e| e.name == env)
        .ok_or_else(|| Error::EnvironmentNotFound(env.to_string()))?;

    // Get all secrets for this environment
    let secrets = vault.get_all_secrets_decrypted(environment.id)?;

    if secrets.is_empty() {
        eprintln!("Warning: No secrets found for {}/{}", project.name, env);
    }

    // Build command
    let program = &command[0];
    let args = &command[1..];

    let mut cmd = std::process::Command::new(program);
    cmd.args(args);

    // Inject secrets as environment variables
    for (key, value) in &secrets {
        cmd.env(key, value);
    }

    // Run the command
    let status = cmd.status()
        .map_err(|e| Error::ConfigError(format!("Failed to run '{}': {}", program, e)))?;

    std::process::exit(status.code().unwrap_or(1));
}
