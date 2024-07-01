use crate::domain::{errors::GetAllUsersError, models::NewUser};
use uuid::Uuid;

pub trait GetAllUsersRepositoryContract: Clone + Send + Sync + 'static {
    fn get_all_users(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<NewUser>, GetAllUsersError>> + Send;
}
