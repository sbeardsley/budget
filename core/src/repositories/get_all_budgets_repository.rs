use crate::domain::{errors::GetAllBudgetsError, models::NewBudget};
use uuid::Uuid;

pub trait GetAllBudgetsRepositoryContract: Clone + Send + Sync + 'static {
    fn get_all_budgets(
        &self,
        user_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<NewBudget>, GetAllBudgetsError>> + Send;
}
