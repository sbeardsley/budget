use crate::domain::{errors::UpdateCategoryError, models::UpdateCategoryCommand};

pub trait UpdateCategoryCommandHandler {
    fn handle(
        &self,
        command: UpdateCategoryCommand,
    ) -> impl std::future::Future<Output = Result<(), UpdateCategoryError>> + Send;
}
