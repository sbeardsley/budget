use crate::domain::{
    errors::UpdateBudgetError,
    models::{BudgetPatch, NewBudget},
};
use uuid::Uuid;

pub trait UpdateBudgetUseCase: Send + Sync {
    fn update_budget(
        &self,
        budget_id: Uuid,
        budget: BudgetPatch,
    ) -> impl std::future::Future<Output = Result<NewBudget, UpdateBudgetError>> + Send;
}
