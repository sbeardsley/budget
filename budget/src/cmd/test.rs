use anyhow::Result;
use clap::Args;
use tracing::{debug, error, info, trace, warn};

/// Test conditional output
#[derive(Args, Debug)]
#[command(hide = true)]
pub struct Cli {
    text: String,
}

impl Cli {
    #[tracing::instrument]
    pub fn exec(&self) -> Result<()> {
        trace!("trace: {}", self.text);
        debug!("debug: {}", self.text);
        info!("info: {}", self.text);
        warn!("warn: {}", self.text);
        error!("error: {}", self.text);

        Ok(())
    }
}
