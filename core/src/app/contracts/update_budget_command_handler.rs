use crate::domain::{errors::UpdateBudgetError, models::UpdateBudgetCommand};

pub trait UpdateBudgetCommandHandler {
    fn handle(
        &self,
        command: UpdateBudgetCommand,
    ) -> impl std::future::Future<Output = Result<(), UpdateBudgetError>> + Send;
}
