use crate::error::Result;
use crate::vault::Vault;

pub fn execute(action: &str, name: Option<&str>, shell: Option<&str>) -> Result<()> {
    match action {
        "export" => export_env(shell),
        "set" => {
            if let Some(name) = name {
                set_env(name, shell)
            } else {
                Err(crate::error::Error::ConfigError("Name required for env set".to_string()))
            }
        }
        "unset" => {
            if let Some(name) = name {
                unset_env(name, shell)
            } else {
                Err(crate::error::Error::ConfigError("Name required for env unset".to_string()))
            }
        }
        _ => Err(crate::error::Error::ConfigError(format!("Unknown env action: {}", action))),
    }
}

fn export_env(shell: Option<&str>) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    let credentials = vault.list_credentials(None)?;
    let env_vars: Vec<_> = credentials.iter()
        .filter(|c| matches!(c.credential_type, crate::models::CredentialType::EnvVar))
        .collect();

    if env_vars.is_empty() {
        println!("No environment variables found");
        println!("  Add an env var: ghostkey add <name> --type env");
        return Ok(());
    }

    let shell_type = shell.unwrap_or("bash");

    println!("# GhostKey Environment Variables");
    println!("# Generated at: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));
    println!("# Source this file: source <(ghostkey env export)");
    println!();

    for var in env_vars {
        let secret = vault.get_secret(&var.name)?;
        let name = var.name.to_uppercase().replace('-', "_");

        match shell_type {
            "bash" | "sh" | "zsh" => {
                println!("export {}=\"{}\"", name, escape_shell(&secret));
            }
            "fish" => {
                println!("set -gx {} \"{}\"", name, escape_shell(&secret));
            }
            "powershell" | "ps" => {
                println!("$env:{} = \"{}\"", name, escape_shell(&secret));
            }
            "cmd" => {
                println!("set {}={}", name, secret);
            }
            _ => {
                println!("export {}=\"{}\"", name, escape_shell(&secret));
            }
        }
    }

    Ok(())
}

fn set_env(name: &str, shell: Option<&str>) -> Result<()> {
    let mut vault = Vault::open()?;
    vault.unlock()?;

    let secret = vault.get_secret(name)?;
    let env_name = name.to_uppercase().replace('-', "_");
    let shell_type = shell.unwrap_or("bash");

    match shell_type {
        "bash" | "sh" | "zsh" => {
            println!("export {}=\"{}\"", env_name, escape_shell(&secret));
        }
        "fish" => {
            println!("set -gx {} \"{}\"", env_name, escape_shell(&secret));
        }
        "powershell" | "ps" => {
            println!("$env:{} = \"{}\"", env_name, escape_shell(&secret));
        }
        "cmd" => {
            println!("set {}={}", env_name, secret);
        }
        _ => {
            println!("export {}=\"{}\"", env_name, escape_shell(&secret));
        }
    }

    Ok(())
}

fn unset_env(name: &str, shell: Option<&str>) -> Result<()> {
    let env_name = name.to_uppercase().replace('-', "_");
    let shell_type = shell.unwrap_or("bash");

    match shell_type {
        "bash" | "sh" | "zsh" => {
            println!("unset {}", env_name);
        }
        "fish" => {
            println!("set -e {}", env_name);
        }
        "powershell" | "ps" => {
            println!("Remove-Item Env:\\{}", env_name);
        }
        "cmd" => {
            println!("set {}=", env_name);
        }
        _ => {
            println!("unset {}", env_name);
        }
    }

    Ok(())
}

fn escape_shell(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('$', "\\$")
        .replace('`', "\\`")
}
