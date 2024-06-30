use crate::domain::{errors::CreateBudgetError, models::CreateBudgetCommand};
use uuid::Uuid;

pub trait CreateBudgetCommandHandler {
    fn handle(
        &self,
        command: CreateBudgetCommand,
    ) -> impl std::future::Future<Output = Result<Uuid, CreateBudgetError>> + Send;
}
