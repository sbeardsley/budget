use crate::domain::{errors::CreateCategoryError, models::CreateCategoryCommand};
use uuid::Uuid;

pub trait CreateCategoryCommandHandler {
    fn handle(
        &self,
        command: CreateCategoryCommand,
    ) -> impl std::future::Future<Output = Result<Uuid, CreateCategoryError>> + Send;
}
