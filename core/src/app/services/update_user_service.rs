use crate::{
    app::contracts::UpdateUserCommandHandler,
    domain::{
        errors::UpdateUserError,
        models::{UpdateUserCommand, UserPatch},
        usecases::UpdateUserUseCase,
    },
    repositories::{GetOneUserRepositoryContract, UpdateUserRepositoryContract},
};

pub struct UpdateUserService<T: UpdateUserRepositoryContract, T2: GetOneUserRepositoryContract> {
    update_user: crate::domain::services::UpdateUserService<T, T2>,
}

impl<T: UpdateUserRepositoryContract, T2: GetOneUserRepositoryContract> UpdateUserService<T, T2> {
    pub fn new(update_repository: T, get_repository: T2) -> Self {
        Self {
            update_user: crate::domain::services::UpdateUserService::new(
                update_repository,
                get_repository,
            ),
        }
    }
}

impl<T: UpdateUserRepositoryContract, T2: GetOneUserRepositoryContract> UpdateUserCommandHandler
    for UpdateUserService<T, T2>
{
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
