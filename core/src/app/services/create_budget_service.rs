use crate::{
    app::contracts::CreateBudgetCommandHandler,
    domain::{
        errors::CreateBudgetError,
        models::{BudgetDraft, CreateBudgetCommand},
        usecases::CreateBudgetUseCase,
    },
    repositories::CreateBudgetRepositoryContract,
};
use uuid::Uuid;

pub struct CreateBudgetService<T: CreateBudgetRepositoryContract> {
    create_budget: crate::domain::services::CreateBudgetService<T>,
}

impl<T: CreateBudgetRepositoryContract> CreateBudgetService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            create_budget: crate::domain::services::CreateBudgetService::new(repository),
        }
    }
}

impl<T: CreateBudgetRepositoryContract> CreateBudgetCommandHandler for CreateBudgetService<T> {
    async fn handle(&self, command: CreateBudgetCommand) -> Result<Uuid, CreateBudgetError> {
        match self
            .create_budget
            .create_budget(BudgetDraft::from(command))
            .await
        {
            Ok(p) => Ok(p.id),
            Err(e) => Err(e),
        }
    }
}
