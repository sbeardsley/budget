use crate::{
    app::contracts::GetAllCategoriesQueryHandler,
    domain::{
        errors::GetAllCategoriesError,
        models::{GetAllCategoriesQuery, GetAllCategoriesQueryResult},
        usecases::GetAllCategoriesUseCase,
    },
    repositories::GetAllCategoriesRepositoryContract,
};

pub struct GetAllCategoriesService<T: GetAllCategoriesRepositoryContract> {
    get_all_categories: crate::domain::services::GetAllCategoriesService<T>,
}

impl<T: GetAllCategoriesRepositoryContract> GetAllCategoriesService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            get_all_categories: crate::domain::services::GetAllCategoriesService::new(repository),
        }
    }
}

impl<T: GetAllCategoriesRepositoryContract> GetAllCategoriesQueryHandler
    for GetAllCategoriesService<T>
{
    async fn handle(
        &self,
        query: GetAllCategoriesQuery,
    ) -> Result<Vec<GetAllCategoriesQueryResult>, GetAllCategoriesError> {
        match self
            .get_all_categories
            .get_all_categories(query.user_id)
            .await
        {
            Ok(categorie) => Ok(categorie
                .into_iter()
                .map(|b| GetAllCategoriesQueryResult::from(b))
                .collect()),
            Err(e) => Err(e),
        }
    }
}
