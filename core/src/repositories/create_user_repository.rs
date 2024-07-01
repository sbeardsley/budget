use crate::domain::{
    errors::CreateUserError,
    models::{NewUser, UserDraft},
};

pub trait CreateUserRepositoryContract: Clone + Send + Sync + 'static {
    fn insert_user(
        &self,
        user: UserDraft,
    ) -> impl std::future::Future<Output = Result<NewUser, CreateUserError>> + Send;
}
