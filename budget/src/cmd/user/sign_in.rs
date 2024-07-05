use anyhow::Result;
use clap::Args;

/// Authenticate as user
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    #[tracing::instrument]
    pub fn exec(&self) -> Result<()> {
        Ok(())
    }
}
