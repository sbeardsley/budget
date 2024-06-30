use crate::domain::{
    errors::GetOneCategoryError,
    models::{GetOneCategoryQuery, GetOneCategoryQueryResult},
};

pub trait GetOneCategoryQueryHandler {
    fn handle(
        &self,
        query: GetOneCategoryQuery,
    ) -> impl std::future::Future<Output = Result<GetOneCategoryQueryResult, GetOneCategoryError>> + Send;
}
