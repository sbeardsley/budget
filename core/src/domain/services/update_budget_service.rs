use crate::{
    domain::{
        errors::UpdateBudgetError,
        models::{BudgetPatch, NewBudget},
        usecases::UpdateBudgetUseCase,
    },
    repositories::UpdateBudgetRepositoryContract,
};
use uuid::Uuid;

pub struct UpdateBudgetService<R: UpdateBudgetRepositoryContract> {
    repository: R,
}

impl<R: UpdateBudgetRepositoryContract> UpdateBudgetService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: UpdateBudgetRepositoryContract> UpdateBudgetUseCase for UpdateBudgetService<R> {
    async fn update_budget(
        &self,
        budget_id: Uuid,
        budget: BudgetPatch,
    ) -> Result<NewBudget, UpdateBudgetError> {
        self.repository.update_budget(budget_id, budget).await
    }
}
