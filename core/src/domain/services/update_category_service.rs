use crate::{
    domain::{
        errors::UpdateCategoryError,
        models::{CategoryPatch, NewCategory},
        usecases::UpdateCategoryUseCase,
    },
    repositories::UpdateCategoryRepositoryContract,
};
use uuid::Uuid;

pub struct UpdateCategoryService<R: UpdateCategoryRepositoryContract> {
    repository: R,
}

impl<R: UpdateCategoryRepositoryContract> UpdateCategoryService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: UpdateCategoryRepositoryContract> UpdateCategoryUseCase for UpdateCategoryService<R> {
    async fn update_category(
        &self,
        category_id: Uuid,
        category: CategoryPatch,
    ) -> Result<NewCategory, UpdateCategoryError> {
        self.repository.update_category(category_id, category).await
    }
}
