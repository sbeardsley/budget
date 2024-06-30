use crate::{
    app::contracts::GetOneCategoryQueryHandler,
    domain::{
        errors::GetOneCategoryError,
        models::{GetOneCategoryQuery, GetOneCategoryQueryResult},
        usecases::GetOneCategoryUseCase,
    },
    repositories::GetOneCategoryRepositoryContract,
};

pub struct GetOneCategoryService<T: GetOneCategoryRepositoryContract> {
    get_one_category: crate::domain::services::GetOneCategoryService<T>,
}

impl<T: GetOneCategoryRepositoryContract> GetOneCategoryService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            get_one_category: crate::domain::services::GetOneCategoryService::new(repository),
        }
    }
}

impl<T: GetOneCategoryRepositoryContract> GetOneCategoryQueryHandler for GetOneCategoryService<T> {
    async fn handle(
        &self,
        query: GetOneCategoryQuery,
    ) -> Result<GetOneCategoryQueryResult, GetOneCategoryError> {
        match self
            .get_one_category
            .get_one_category(query.category_id)
            .await
        {
            Ok(category) => match category {
                Some(category) => Ok(GetOneCategoryQueryResult::from(category)),
                None => Err(GetOneCategoryError::Unknown),
            },
            Err(e) => Err(e),
        }
    }
}
