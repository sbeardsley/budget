use crate::domain::{
    errors::UpdateUserError,
    models::{NewUser, UserPatch},
};
use uuid::Uuid;

pub trait UpdateUserRepositoryContract: Clone + Send + Sync + 'static {
    fn update_user(
        &self,
        user_id: Uuid,
        user: UserPatch,
    ) -> impl std::future::Future<Output = Result<NewUser, UpdateUserError>> + Send;
}
