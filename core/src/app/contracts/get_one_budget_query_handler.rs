use crate::domain::{
    errors::GetOneBudgetError,
    models::{GetOneBudgetQuery, GetOneBudgetQueryResult},
};

pub trait GetOneBudgetQueryHandler {
    fn handle(
        &self,
        query: GetOneBudgetQuery,
    ) -> impl std::future::Future<Output = Result<GetOneBudgetQueryResult, GetOneBudgetError>> + Send;
}
