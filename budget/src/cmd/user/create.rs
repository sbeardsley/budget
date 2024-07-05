use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use clap::Args;
use dialoguer::{console::Term, theme::ColorfulTheme, FuzzySelect, Input};

/// Create new user account
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    #[tracing::instrument]
    pub fn exec(&self) -> Result<()> {
        let term = Term::stdout();
        term.set_title("Counting...");
        term.clear_screen()?;

        let password = Input::<String>::new()
            .with_prompt("Your password?")
            .interact_text()
            .unwrap();

        Ok(())
    }
}
