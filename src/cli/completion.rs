use crate::error::Result;
use clap::CommandFactory;
use clap_complete::{generate, shells::Bash, shells::Fish, shells::Zsh, shells::PowerShell};

pub fn execute(shell: &str) -> Result<()> {
    let mut cmd = super::Cli::command();

    match shell {
        "bash" => {
            generate(Bash, &mut cmd, "ghostkey", &mut std::io::stdout());
        }
        "zsh" => {
            generate(Zsh, &mut cmd, "ghostkey", &mut std::io::stdout());
        }
        "fish" => {
            generate(Fish, &mut cmd, "ghostkey", &mut std::io::stdout());
        }
        "powershell" | "ps" => {
            generate(PowerShell, &mut cmd, "ghostkey", &mut std::io::stdout());
        }
        _ => {
            return Err(crate::error::Error::ConfigError(
                format!("Unsupported shell: {}. Use bash, zsh, fish, or powershell", shell)
            ));
        }
    }

    Ok(())
}
