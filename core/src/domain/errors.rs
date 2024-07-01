use thiserror::Error;

#[derive(Error, Debug)]
pub enum BudgetParseError {
    #[error("BudgetId parse error")]
    BudgetIdParseError,
}

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

#[derive(Error, Debug)]
pub enum CreateCategoryError {
    #[error("Category already exists")]
    CategoryAlreadyExists,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum UpdateCategoryError {
    #[error("Category not found")]
    CategoryNotFound,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum GetOneCategoryError {
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum GetAllCategoriesError {
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum CreateUserError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum UpdateUserError {
    #[error("User not found")]
    UserNotFound,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum GetOneUserError {
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum GetAllUsersError {
    #[error("Unknown error")]
    Unknown,
}
