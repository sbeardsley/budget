use crate::{
    domain::{errors::GetOneCategoryError, models::NewCategory, usecases::GetOneCategoryUseCase},
    repositories::GetOneCategoryRepositoryContract,
};
use uuid::Uuid;

pub struct GetOneCategoryService<R: GetOneCategoryRepositoryContract> {
    repository: R,
}

impl<R: GetOneCategoryRepositoryContract> GetOneCategoryService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: GetOneCategoryRepositoryContract> GetOneCategoryUseCase for GetOneCategoryService<R> {
    async fn get_one_category(
        &self,
        category_id: Uuid,
    ) -> Result<Option<NewCategory>, GetOneCategoryError> {
        self.repository.get_one_category(category_id).await
    }
}
