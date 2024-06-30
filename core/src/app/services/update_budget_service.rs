use crate::{
    app::contracts::UpdateBudgetCommandHandler,
    domain::{
        errors::UpdateBudgetError,
        models::{BudgetPatch, UpdateBudgetCommand},
        usecases::UpdateBudgetUseCase,
    },
    repositories::UpdateBudgetRepositoryContract,
};

pub struct UpdateBudgetService<T: UpdateBudgetRepositoryContract> {
    update_budget: crate::domain::services::UpdateBudgetService<T>,
}

impl<T: UpdateBudgetRepositoryContract> UpdateBudgetService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            update_budget: crate::domain::services::UpdateBudgetService::new(repository),
        }
    }
}

impl<T: UpdateBudgetRepositoryContract> UpdateBudgetCommandHandler for UpdateBudgetService<T> {
    async fn handle(&self, command: UpdateBudgetCommand) -> Result<(), UpdateBudgetError> {
        match self
            .update_budget
            .update_budget(command.id, BudgetPatch::from(command))
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
