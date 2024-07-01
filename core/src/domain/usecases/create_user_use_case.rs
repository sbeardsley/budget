use crate::domain::{
    errors::CreateUserError,
    models::{NewUser, UserDraft},
};

pub trait CreateUserUseCase: Send + Sync {
    fn create_user(
        &self,
        user: UserDraft,
    ) -> impl std::future::Future<Output = Result<NewUser, CreateUserError>> + Send;
}
