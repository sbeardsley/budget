use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

/// A personal budget management tool.
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    name = "budget",
    bin_name = "budget",
    disable_help_subcommand = true
)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    User(super::user::Cli),
    Complete(super::complete::Cli),
    Config(super::config::Cli),
    Exec(super::exec::Cli),
    Test(super::test::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::User(cli) => cli.exec(),
            Commands::Complete(cli) => cli.exec(),
            Commands::Config(cli) => cli.exec(),
            Commands::Exec(cli) => cli.exec(),
            Commands::Test(cli) => cli.exec(),
        }
    }
}
