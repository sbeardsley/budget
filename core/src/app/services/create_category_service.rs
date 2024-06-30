use crate::{
    app::contracts::CreateCategoryCommandHandler,
    domain::{
        errors::CreateCategoryError,
        models::{CategoryDraft, CreateCategoryCommand},
        usecases::CreateCategoryUseCase,
    },
    repositories::CreateCategoryRepositoryContract,
};
use uuid::Uuid;

pub struct CreateCategoryService<T: CreateCategoryRepositoryContract> {
    create_category: crate::domain::services::CreateCategoryService<T>,
}

impl<T: CreateCategoryRepositoryContract> CreateCategoryService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            create_category: crate::domain::services::CreateCategoryService::new(repository),
        }
    }
}

impl<T: CreateCategoryRepositoryContract> CreateCategoryCommandHandler
    for CreateCategoryService<T>
{
    async fn handle(&self, command: CreateCategoryCommand) -> Result<Uuid, CreateCategoryError> {
        match self
            .create_category
            .create_category(CategoryDraft::from(command))
            .await
        {
            Ok(p) => Ok(p.id),
            Err(e) => Err(e),
        }
    }
}
