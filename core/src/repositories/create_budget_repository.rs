use crate::domain::{
    errors::CreateBudgetError,
    models::{BudgetDraft, NewBudget},
};

pub trait CreateBudgetRepositoryContract: Clone + Send + Sync + 'static {
    fn insert_budget(
        &self,
        budget: BudgetDraft,
    ) -> impl std::future::Future<Output = Result<NewBudget, CreateBudgetError>> + Send;
}
