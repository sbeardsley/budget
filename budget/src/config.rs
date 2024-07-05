use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "budget";
const FILE_STEM: &str = "config";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub repo: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { repo: "".into() }
    }
}

pub fn path() -> Result<PathBuf> {
    confy::get_configuration_file_path(APP_NAME, FILE_STEM)
        .with_context(|| "unable to find the config file")
}

pub fn load() -> Result<Config> {
    confy::load(APP_NAME, FILE_STEM).with_context(|| "unable to load the config file")
}

pub fn save(config: &Config) -> Result<()> {
    confy::store(APP_NAME, FILE_STEM, config).with_context(|| "unable to save the config file")
}
