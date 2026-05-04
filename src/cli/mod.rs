mod init;
mod dev;
mod run;
mod export;
mod import;

use clap::{Parser, Subcommand};
use crate::error::Result;

#[derive(Parser)]
#[command(name = "ghostkey")]
#[command(version = "0.1.0")]
#[command(about = "Local-first secrets manager for developers", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new GhostKey vault
    Init,

    /// Start the web UI server
    Dev {
        /// Port to listen on (default: random)
        #[arg(short, long)]
        port: Option<u16>,

        /// Don't open browser automatically
        #[arg(long)]
        no_open: bool,
    },

    /// Run a command with secrets injected as environment variables
    Run {
        /// Project name (default: first project)
        #[arg(short, long)]
        project: Option<String>,

        /// Environment name (default: "default")
        #[arg(short, long, default_value = "default")]
        env: String,

        /// The command to run
        #[arg(required = true, trailing_var_arg = true)]
        command: Vec<String>,
    },

    /// Export secrets to a file or stdout
    Export {
        /// Output format (env, json, csv)
        #[arg(short, long, value_enum, default_value = "env")]
        format: ExportFormat,

        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,

        /// Project name
        #[arg(short, long)]
        project: Option<String>,

        /// Environment name (default: "default")
        #[arg(short, long, default_value = "default")]
        env: String,
    },

    /// Import secrets from a file
    Import {
        /// Input format (env, json, csv)
        #[arg(short, long, value_enum)]
        format: ImportFormat,

        /// Input file
        file: String,

        /// Project name (target)
        #[arg(short, long)]
        project: Option<String>,

        /// Environment name (target)
        #[arg(short, long, default_value = "default")]
        env: String,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum ExportFormat {
    Env,
    Json,
    Csv,
}

#[derive(clap::ValueEnum, Clone)]
pub enum ImportFormat {
    Env,
    Json,
    Csv,
}

impl Cli {
    pub async fn execute(&self) -> Result<()> {
        match &self.command {
            Some(Commands::Init) => init::execute(),
            Some(Commands::Dev { port, no_open }) => dev::execute(*port, *no_open).await,
            Some(Commands::Run { project, env, command }) => {
                run::execute(project.as_deref(), env, command)
            }
            Some(Commands::Export { format, output, project, env }) => {
                export::execute(format, output.as_deref(), project.as_deref(), env)
            }
            Some(Commands::Import { format, file, project, env }) => {
                import::execute(format, file, project.as_deref(), env)
            }
            None => Ok(()),
        }
    }
}
