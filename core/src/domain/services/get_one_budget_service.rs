use crate::{
    domain::{errors::GetOneBudgetError, models::NewBudget, usecases::GetOneBudgetUseCase},
    repositories::GetOneBudgetRepositoryContract,
};
use uuid::Uuid;

pub struct GetOneBudgetService<R: GetOneBudgetRepositoryContract> {
    repository: R,
}

impl<R: GetOneBudgetRepositoryContract> GetOneBudgetService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: GetOneBudgetRepositoryContract> GetOneBudgetUseCase for GetOneBudgetService<R> {
    async fn get_one_budget(
        &self,
        budget_id: Uuid,
    ) -> Result<Option<NewBudget>, GetOneBudgetError> {
        self.repository.get_one_budget(budget_id).await
    }
}
