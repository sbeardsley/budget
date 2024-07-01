use crate::domain::{
    errors::UpdateUserError,
    models::{NewUser, UserPatch},
};
use uuid::Uuid;

pub trait UpdateUserUseCase: Send + Sync {
    fn update_user(
        &self,
        user_id: Uuid,
        user: UserPatch,
    ) -> impl std::future::Future<Output = Result<NewUser, UpdateUserError>> + Send;
}
