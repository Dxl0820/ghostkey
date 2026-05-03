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
use error::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.execute()
}
