use crate::{
    app::contracts::GetAllBudgetsQueryHandler,
    domain::{
        errors::GetAllBudgetsError,
        models::{GetAllBudgetsQuery, GetAllBudgetsQueryResult},
        usecases::GetAllBudgetsUseCase,
    },
    repositories::GetAllBudgetsRepositoryContract,
};

pub struct GetAllBudgetsService<T: GetAllBudgetsRepositoryContract> {
    get_all_budget: crate::domain::services::GetAllBudgetsService<T>,
}

impl<T: GetAllBudgetsRepositoryContract> GetAllBudgetsService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            get_all_budget: crate::domain::services::GetAllBudgetsService::new(repository),
        }
    }
}

impl<T: GetAllBudgetsRepositoryContract> GetAllBudgetsQueryHandler for GetAllBudgetsService<T> {
    async fn handle(
        &self,
        query: GetAllBudgetsQuery,
    ) -> Result<Vec<GetAllBudgetsQueryResult>, GetAllBudgetsError> {
        match self.get_all_budget.get_all_budgets(query.user_id).await {
            Ok(budget) => Ok(budget
                .into_iter()
                .map(|b| GetAllBudgetsQueryResult::from(b))
                .collect()),
            Err(e) => Err(e),
        }
    }
}
