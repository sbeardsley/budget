use crate::domain::{
    errors::GetOneUserError,
    models::{GetOneUserQuery, GetOneUserQueryResult},
};

pub trait GetOneUserQueryHandler {
    fn handle(
        &self,
        query: GetOneUserQuery,
    ) -> impl std::future::Future<Output = Result<GetOneUserQueryResult, GetOneUserError>> + Send;
}
