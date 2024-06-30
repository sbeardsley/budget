use crate::{
    domain::{
        errors::CreateCategoryError,
        models::{CategoryDraft, NewCategory},
        usecases::CreateCategoryUseCase,
    },
    repositories::CreateCategoryRepositoryContract,
};

pub struct CreateCategoryService<R: CreateCategoryRepositoryContract> {
    repository: R,
}

impl<R: CreateCategoryRepositoryContract> CreateCategoryService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: CreateCategoryRepositoryContract> CreateCategoryUseCase for CreateCategoryService<R> {
    async fn create_category(
        &self,
        category: CategoryDraft,
    ) -> Result<NewCategory, CreateCategoryError> {
        self.repository.insert_category(category).await
    }
}
