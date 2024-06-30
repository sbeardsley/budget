use crate::{
    domain::{
        errors::GetAllCategoriesError, models::NewCategory, usecases::GetAllCategoriesUseCase,
    },
    repositories::GetAllCategoriesRepositoryContract,
};
use uuid::Uuid;

pub struct GetAllCategoriesService<R: GetAllCategoriesRepositoryContract> {
    repository: R,
}

impl<R: GetAllCategoriesRepositoryContract> GetAllCategoriesService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: GetAllCategoriesRepositoryContract> GetAllCategoriesUseCase for GetAllCategoriesService<R> {
    async fn get_all_categories(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<NewCategory>, GetAllCategoriesError> {
        self.repository.get_all_categories(user_id).await
    }
}
