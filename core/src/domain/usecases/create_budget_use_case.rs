use crate::domain::{
    errors::CreateBudgetError,
    models::{BudgetDraft, NewBudget},
};

pub trait CreateBudgetUseCase: Send + Sync {
    fn create_budget(
        &self,
        budget: BudgetDraft,
    ) -> impl std::future::Future<Output = Result<NewBudget, CreateBudgetError>> + Send;
}
