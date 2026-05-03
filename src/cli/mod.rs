mod init;
mod add;
mod get;
mod list;
mod delete;
mod tag;
mod export;
mod import;
mod ssh;
mod env;
mod clipboard;
mod search;
mod completion;

use clap::{Parser, Subcommand, ValueHint};
use crate::error::Result;

#[derive(Parser)]
#[command(name = "ghostkey")]
#[command(version = "0.1.0")]
#[command(about = "A developer-first credential management system", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new GhostKey vault
    Init,

    /// Add a new credential
    Add {
        /// Credential name
        #[arg(required = true, value_hint = ValueHint::Other)]
        name: String,

        /// Credential type (password, apikey, ssh, token, env, custom)
        #[arg(short, long, value_enum)]
        r#type: Option<CredentialTypeArg>,

        /// Username
        #[arg(short, long)]
        username: Option<String>,

        /// Description
        #[arg(short, long)]
        description: Option<String>,

        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,

        /// URL
        #[arg(short, long)]
        url: Option<String>,
    },

    /// Get a credential
    Get {
        /// Credential name
        #[arg(required = true, value_hint = ValueHint::Other)]
        name: String,

        /// Copy to clipboard instead of displaying
        #[arg(short, long)]
        clipboard: bool,

        /// Show the secret value
        #[arg(short, long)]
        show: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// List all credentials
    List {
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,

        /// Filter by type
        #[arg(short = 'y', long, value_enum)]
        r#type: Option<CredentialTypeArg>,

        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Show only names
        #[arg(long)]
        names_only: bool,
    },

    /// Delete a credential
    Delete {
        /// Credential name
        #[arg(required = true, value_hint = ValueHint::Other)]
        name: String,

        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Search credentials
    Search {
        /// Search query
        #[arg(required = true)]
        query: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Manage tags
    Tag {
        #[command(subcommand)]
        action: TagAction,
    },

    /// Export credentials
    Export {
        /// Output format (env, json, csv)
        #[arg(short, long, value_enum, default_value = "env")]
        format: ExportFormat,

        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,

        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
    },

    /// Import credentials
    Import {
        /// Input format (env, json, csv)
        #[arg(short, long, value_enum)]
        format: ImportFormat,

        /// Input file
        #[arg(required = true)]
        file: String,
    },

    /// SSH key management
    Ssh {
        /// Action (list, config, add)
        #[arg(required = true)]
        action: String,

        /// Credential name (for add action)
        #[arg(required_unless_present = "action")]
        name: Option<String>,
    },

    /// Environment variable management
    Env {
        /// Action (export, set, unset)
        #[arg(required = true)]
        action: String,

        /// Credential name (for set/unset actions)
        name: Option<String>,

        /// Shell type (bash, fish, powershell, cmd)
        #[arg(short, long)]
        shell: Option<String>,
    },

    /// Copy credential to clipboard with auto-clear
    Clipboard {
        /// Credential name
        #[arg(required = true, value_hint = ValueHint::Other)]
        name: String,

        /// Timeout in seconds (default: 30)
        #[arg(short, long, default_value = "30")]
        timeout: u64,
    },

    /// Change master password
    Passwd,

    /// Show vault status
    Status,

    /// Generate shell completions
    Completion {
        /// Shell type (bash, zsh, fish, powershell)
        #[arg(required = true)]
        shell: String,
    },
}

#[derive(Subcommand)]
enum TagAction {
    /// List all tags
    List,

    /// Add tag to credential
    Add {
        /// Credential name
        credential: String,
        /// Tag to add
        tag: String,
    },

    /// Remove tag from credential
    Remove {
        /// Credential name
        credential: String,
        /// Tag to remove
        tag: String,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum CredentialTypeArg {
    Password,
    ApiKey,
    Ssh,
    Token,
    Env,
    Custom,
}

#[derive(clap::ValueEnum, Clone)]
enum ExportFormat {
    Env,
    Json,
    Csv,
}

#[derive(clap::ValueEnum, Clone)]
enum ImportFormat {
    Env,
    Json,
    Csv,
}

impl Cli {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Some(Commands::Init) => init::execute(),
            Some(Commands::Add { name, r#type, username, description, tags, url }) => {
                let tags_vec = tags.as_ref()
                    .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
                    .unwrap_or_default();
                add::execute(name, r#type.as_ref(), username.as_deref(), description.as_deref(), tags_vec, url.as_deref())
            }
            Some(Commands::Get { name, clipboard, show, json }) => {
                get::execute(name, *clipboard, *show, *json)
            }
            Some(Commands::List { tag, r#type, json, names_only }) => {
                list::execute(tag.as_deref(), r#type.as_ref(), *json, *names_only)
            }
            Some(Commands::Delete { name, force }) => delete::execute(name, *force),
            Some(Commands::Search { query, json }) => {
                search::execute(query, *json)
            }
            Some(Commands::Tag { action }) => match action {
                TagAction::List => tag::list_tags(),
                TagAction::Add { credential, tag } => tag::add_tag(credential, tag),
                TagAction::Remove { credential, tag } => tag::remove_tag(credential, tag),
            },
            Some(Commands::Export { format, output, tag }) => {
                export::execute(format, output.as_deref(), tag.as_deref())
            }
            Some(Commands::Import { format, file }) => {
                import::execute(format, file)
            }
            Some(Commands::Ssh { action, name }) => {
                ssh::execute(action, name.as_deref())
            }
            Some(Commands::Env { action, name, shell }) => {
                env::execute(action, name.as_deref(), shell.as_deref())
            }
            Some(Commands::Clipboard { name, timeout }) => {
                clipboard::execute(name, Some(*timeout))
            }
            Some(Commands::Passwd) => init::change_password(),
            Some(Commands::Status) => init::status(),
            Some(Commands::Completion { shell }) => {
                completion::execute(shell)
            }
            None => Ok(()),
        }
    }
}
