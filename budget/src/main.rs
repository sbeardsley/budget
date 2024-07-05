#[macro_use]
mod macros;
mod app;
mod cmd;
mod config;
mod platform;

use anyhow::Result;
use clap::Parser;
use tracing_log::AsTrace;

use crate::cmd::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    app::set_global_verbosity(cli.verbose.log_level_filter().as_trace());
    app::set_global_config(config::load()?);
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        .with_max_level(cli.verbose.log_level_filter().as_trace())
        .init();

    cli.exec()
}
