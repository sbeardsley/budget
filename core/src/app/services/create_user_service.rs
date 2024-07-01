use crate::{
    app::contracts::CreateUserCommandHandler,
    domain::{
        errors::CreateUserError,
        models::{CreateUserCommand, UserDraft},
        usecases::CreateUserUseCase,
    },
    repositories::CreateUserRepositoryContract,
};
use uuid::Uuid;

pub struct CreateUserService<T: CreateUserRepositoryContract> {
    create_user: crate::domain::services::CreateUserService<T>,
}

impl<T: CreateUserRepositoryContract> CreateUserService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            create_user: crate::domain::services::CreateUserService::new(repository),
        }
    }
}

impl<T: CreateUserRepositoryContract> CreateUserCommandHandler for CreateUserService<T> {
    async fn handle(&self, command: CreateUserCommand) -> Result<Uuid, CreateUserError> {
        match self.create_user.create_user(UserDraft::from(command)).await {
            Ok(p) => Ok(p.id),
            Err(e) => Err(e),
        }
    }
}
