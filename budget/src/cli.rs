use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "budget", about = "A personal budgeting app", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Budget actions
    Budget,
}
