use crate::{
    domain::{
        errors::UpdateUserError,
        models::{NewUser, UserPatch},
        usecases::{GetOneUserUseCase, UpdateUserUseCase},
    },
    repositories::{GetOneUserRepositoryContract, UpdateUserRepositoryContract},
};
use uuid::Uuid;

pub struct UpdateUserService<R: UpdateUserRepositoryContract, R2: GetOneUserRepositoryContract> {
    update_repository: R,
    get_repository: R2,
}

impl<R: UpdateUserRepositoryContract, R2: GetOneUserRepositoryContract> UpdateUserService<R, R2> {
    pub fn new(update_repository: R, get_repository: R2) -> Self {
        Self {
            update_repository,
            get_repository,
        }
    }
}

impl<R: UpdateUserRepositoryContract, R2: GetOneUserRepositoryContract> GetOneUserUseCase
    for UpdateUserService<R, R2>
{
    async fn get_one_user(
        &self,
        user_id: Uuid,
    ) -> Result<Option<NewUser>, crate::domain::errors::GetOneUserError> {
        self.get_repository.get_one_user(user_id).await
    }
}

impl<R: UpdateUserRepositoryContract, R2: GetOneUserRepositoryContract> UpdateUserUseCase
    for UpdateUserService<R, R2>
{
    async fn update_user(
        &self,
        user_id: Uuid,
        mut user: UserPatch,
    ) -> Result<NewUser, UpdateUserError> {
        let existing_user = self
            .get_one_user(user_id)
            .await
            .map_err(|_| UpdateUserError::UserNotFound)?;
        match existing_user {
            Some(existing_user) => {
                user.email = user.email.or(Some(existing_user.email));
                user.name = user.name.or(Some(existing_user.name));
                user.password = user.password.or(Some(existing_user.password));
            }
            None => return Err(UpdateUserError::UserNotFound),
        }
        self.update_repository.update_user(user_id, user).await
    }
}
