use crate::{
    app::contracts::GetOneBudgetQueryHandler,
    domain::{
        errors::GetOneBudgetError,
        models::{GetOneBudgetQuery, GetOneBudgetQueryResult},
        usecases::GetOneBudgetUseCase,
    },
    repositories::GetOneBudgetRepositoryContract,
};

pub struct GetOneBudgetService<T: GetOneBudgetRepositoryContract> {
    get_one_budget: crate::domain::services::GetOneBudgetService<T>,
}

impl<T: GetOneBudgetRepositoryContract> GetOneBudgetService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            get_one_budget: crate::domain::services::GetOneBudgetService::new(repository),
        }
    }
}

impl<T: GetOneBudgetRepositoryContract> GetOneBudgetQueryHandler for GetOneBudgetService<T> {
    async fn handle(
        &self,
        query: GetOneBudgetQuery,
    ) -> Result<GetOneBudgetQueryResult, GetOneBudgetError> {
        match self.get_one_budget.get_one_budget(query.budget_id).await {
            Ok(budget) => match budget {
                Some(budget) => Ok(GetOneBudgetQueryResult::from(budget)),
                None => Err(GetOneBudgetError::Unknown),
            },
            Err(e) => Err(e),
        }
    }
}
