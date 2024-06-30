use crate::domain::{errors::GetOneBudgetError, models::NewBudget};
use uuid::Uuid;

pub trait GetOneBudgetRepositoryContract: Clone + Send + Sync + 'static {
    fn get_one_budget(
        &self,
        budget_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<NewBudget>, GetOneBudgetError>> + Send;
}
