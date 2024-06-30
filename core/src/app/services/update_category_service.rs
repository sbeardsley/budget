use crate::{
    app::contracts::UpdateCategoryCommandHandler,
    domain::{
        errors::UpdateCategoryError,
        models::{CategoryPatch, UpdateCategoryCommand},
        usecases::UpdateCategoryUseCase,
    },
    repositories::UpdateCategoryRepositoryContract,
};

pub struct UpdateCategoryService<T: UpdateCategoryRepositoryContract> {
    update_category: crate::domain::services::UpdateCategoryService<T>,
}

impl<T: UpdateCategoryRepositoryContract> UpdateCategoryService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            update_category: crate::domain::services::UpdateCategoryService::new(repository),
        }
    }
}

impl<T: UpdateCategoryRepositoryContract> UpdateCategoryCommandHandler
    for UpdateCategoryService<T>
{
    async fn handle(&self, command: UpdateCategoryCommand) -> Result<(), UpdateCategoryError> {
        match self
            .update_category
            .update_category(command.id, CategoryPatch::from(command))
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
