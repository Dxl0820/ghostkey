mod api;
mod cli;
mod config;
mod error;
mod models;
mod utils;
mod vault;

#[cfg(test)]
mod tests;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> error::Result<()> {
    let cli = Cli::parse();
    cli.execute().await
}
