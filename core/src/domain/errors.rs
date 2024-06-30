use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateBudgetError {
    #[error("Budget already exists")]
    BudgetAlreadyExists,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum UpdateBudgetError {
    #[error("Budget not found")]
    BudgetNotFound,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum GetOneBudgetError {
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum GetAllBudgetsError {
    #[error("Unknown error")]
    Unknown,
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Failed to connect to database")]
    ConnectionFailed,
    #[error("Transaction failed")]
    TransactionFailed,
    #[error("Migration failed")]
    MigrationFailed,
}
