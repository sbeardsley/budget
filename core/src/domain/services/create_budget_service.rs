use crate::{
    domain::{
        errors::CreateBudgetError,
        models::{BudgetDraft, NewBudget},
        usecases::CreateBudgetUseCase,
    },
    repositories::CreateBudgetRepositoryContract,
};

pub struct CreateBudgetService<R: CreateBudgetRepositoryContract> {
    repository: R,
}

impl<R: CreateBudgetRepositoryContract> CreateBudgetService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: CreateBudgetRepositoryContract> CreateBudgetUseCase for CreateBudgetService<R> {
    async fn create_budget(&self, budget: BudgetDraft) -> Result<NewBudget, CreateBudgetError> {
        self.repository.insert_budget(budget).await
    }
}
