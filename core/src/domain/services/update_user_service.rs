use crate::{
    domain::{
        errors::UpdateUserError,
        models::{NewUser, UserPatch},
        usecases::UpdateUserUseCase,
    },
    repositories::UpdateUserRepositoryContract,
};
use uuid::Uuid;

pub struct UpdateUserService<R: UpdateUserRepositoryContract> {
    repository: R,
}

impl<R: UpdateUserRepositoryContract> UpdateUserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: UpdateUserRepositoryContract> UpdateUserUseCase for UpdateUserService<R> {
    async fn update_user(
        &self,
        user_id: Uuid,
        user: UserPatch,
    ) -> Result<NewUser, UpdateUserError> {
        self.repository.update_user(user_id, user).await
    }
}
