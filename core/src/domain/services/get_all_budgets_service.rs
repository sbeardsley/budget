use crate::{
    domain::{errors::GetAllBudgetsError, models::NewBudget, usecases::GetAllBudgetsUseCase},
    repositories::GetAllBudgetsRepositoryContract,
};
use uuid::Uuid;

pub struct GetAllBudgetsService<R: GetAllBudgetsRepositoryContract> {
    repository: R,
}

impl<R: GetAllBudgetsRepositoryContract> GetAllBudgetsService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: GetAllBudgetsRepositoryContract> GetAllBudgetsUseCase for GetAllBudgetsService<R> {
    async fn get_all_budgets(&self, user_id: Uuid) -> Result<Vec<NewBudget>, GetAllBudgetsError> {
        self.repository.get_all_budgets(user_id).await
    }
}
