use crate::{
    domain::{
        errors::CreateUserError,
        models::{NewUser, UserDraft},
        usecases::CreateUserUseCase,
    },
    repositories::CreateUserRepositoryContract,
};

pub struct CreateUserService<R: CreateUserRepositoryContract> {
    repository: R,
}

impl<R: CreateUserRepositoryContract> CreateUserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: CreateUserRepositoryContract> CreateUserUseCase for CreateUserService<R> {
    async fn create_user(&self, user: UserDraft) -> Result<NewUser, CreateUserError> {
        self.repository.insert_user(user).await
    }
}
