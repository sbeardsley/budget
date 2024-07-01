use crate::domain::{
    errors::GetAllUsersError,
    models::{GetAllUsersQuery, GetAllUsersQueryResult},
};

pub trait GetAllUsersQueryHandler {
    fn handle(
        &self,
        query: GetAllUsersQuery,
    ) -> impl std::future::Future<Output = Result<Vec<GetAllUsersQueryResult>, GetAllUsersError>> + Send;
}
