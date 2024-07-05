use crate::{
    domain::{
        errors::CreateUserError,
        models::{NewUser, UserDraft},
        usecases::{CreateUserUseCase, HashUserPasswordUseCase},
    },
    repositories::CreateUserRepositoryContract,
};

use super::hash_user_password_service::HashUserPasswordService;

pub struct CreateUserService<R: CreateUserRepositoryContract> {
    repository: R,
    hash_password_service: HashUserPasswordService,
}

impl<R: CreateUserRepositoryContract> CreateUserService<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            hash_password_service: HashUserPasswordService::default(),
        }
    }
}

impl<R: CreateUserRepositoryContract> CreateUserUseCase for CreateUserService<R> {
    async fn create_user(&self, mut user: UserDraft) -> Result<NewUser, CreateUserError> {
        let user_draft = user.clone();
        let hashed_password = self
            .hash_password_service
            .hash_user_password(user_draft.password)
            .await
            .map_err(|_| CreateUserError::HashPasswordError)?;
        user.password = hashed_password;
        self.repository.insert_user(user).await
    }
}
