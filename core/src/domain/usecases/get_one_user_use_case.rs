use crate::domain::{errors::GetOneUserError, models::NewUser};
use uuid::Uuid;

pub trait GetOneUserUseCase: Send + Sync {
    fn get_one_user(
        &self,
        user_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<NewUser>, GetOneUserError>> + Send;
}
