use crate::{
    app::contracts::UpdateUserCommandHandler,
    domain::{
        errors::UpdateUserError,
        models::{UpdateUserCommand, UserPatch},
        usecases::UpdateUserUseCase,
    },
    repositories::UpdateUserRepositoryContract,
};

pub struct UpdateUserService<T: UpdateUserRepositoryContract> {
    update_user: crate::domain::services::UpdateUserService<T>,
}

impl<T: UpdateUserRepositoryContract> UpdateUserService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            update_user: crate::domain::services::UpdateUserService::new(repository),
        }
    }
}

impl<T: UpdateUserRepositoryContract> UpdateUserCommandHandler for UpdateUserService<T> {
    async fn handle(&self, command: UpdateUserCommand) -> Result<(), UpdateUserError> {
        match self
            .update_user
            .update_user(command.id, UserPatch::from(command))
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
