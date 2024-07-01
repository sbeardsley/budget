use infra::app::Service;
use inquire::{error::InquireResult, Confirm, DateSelect, MultiSelect, Password, Select, Text};
pub struct App {
    db_url: String,
    service: Service,
}

impl App {
    pub async fn new() -> Self {
        Self {
            db_url: "sqlite://budget.db".to_string(),
            service: Service::new("sqlite://budget.db").await,
        }
    }

    pub fn budget(&self) -> InquireResult<()> {
        let name = Text::new("What is your name?").prompt()?;
        let age = Text::new("What is your age?").prompt()?;
        let confirm = Confirm::new("Are you sure?").prompt()?;
        let password = Password::new("Enter your password").prompt()?;
        let date = DateSelect::new("Select a date").prompt()?;
        let select = Select::new("Select a number", vec!["One", "Two", "Three"]).prompt()?;
        let multi_select =
            MultiSelect::new("Select multiple numbers", vec!["One", "Two", "Three"]).prompt()?;
        Ok(())
    }
}
