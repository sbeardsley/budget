use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage user accounts
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Create(super::create::Cli),
    SignIn(super::sign_in::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Create(cli) => cli.exec(),
            Commands::SignIn(cli) => cli.exec(),
        }
    }
}
