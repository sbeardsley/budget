use anyhow::Result;
use clap::Args;
use tracing::info;

use crate::config;

/// Locate the config file
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        info!("{}", config::path()?.display());

        Ok(())
    }
}
