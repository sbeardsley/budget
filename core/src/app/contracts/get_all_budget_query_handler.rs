use crate::domain::{
    errors::GetAllBudgetsError,
    models::{GetAllBudgetsQuery, GetAllBudgetsQueryResult},
};

pub trait GetAllBudgetsQueryHandler {
    fn handle(
        &self,
        query: GetAllBudgetsQuery,
    ) -> impl std::future::Future<Output = Result<Vec<GetAllBudgetsQueryResult>, GetAllBudgetsError>>
           + Send;
}
