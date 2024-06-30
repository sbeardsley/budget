use crate::domain::{
    errors::GetAllCategoriesError,
    models::{GetAllCategoriesQuery, GetAllCategoriesQueryResult},
};

pub trait GetAllCategoriesQueryHandler {
    fn handle(
        &self,
        query: GetAllCategoriesQuery,
    ) -> impl std::future::Future<
        Output = Result<Vec<GetAllCategoriesQueryResult>, GetAllCategoriesError>,
    > + Send;
}
