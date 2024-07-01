use uuid::Uuid;

use crate::domain::{errors::GetOneUserError, models::NewUser};

pub trait GetOneUserRepositoryContract: Clone + Send + Sync + 'static {
    fn get_one_user(
        &self,
        user_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<NewUser>, GetOneUserError>> + Send;
}
