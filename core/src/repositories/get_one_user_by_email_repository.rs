use crate::domain::{errors::GetOneUserError, models::NewUser};

pub trait GetOneUserRepositoryByEmailContract: Clone + Send + Sync + 'static {
    fn get_one_user(
        &self,
        user_email: String,
    ) -> impl std::future::Future<Output = Result<Option<NewUser>, GetOneUserError>> + Send;
}
