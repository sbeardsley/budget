use crate::{
    domain::{errors::GetOneUserError, models::NewUser, usecases::GetOneUserUseCase},
    repositories::GetOneUserRepositoryContract,
};
use uuid::Uuid;

pub struct GetOneUserService<R: GetOneUserRepositoryContract> {
    repository: R,
}

impl<R: GetOneUserRepositoryContract> GetOneUserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: GetOneUserRepositoryContract> GetOneUserUseCase for GetOneUserService<R> {
    async fn get_one_user(&self, user_id: Uuid) -> Result<Option<NewUser>, GetOneUserError> {
        self.repository.get_one_user(user_id).await
    }
}
