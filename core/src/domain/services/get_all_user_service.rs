use crate::{
    domain::{errors::GetAllUsersError, models::NewUser, usecases::GetAllUsersUseCase},
    repositories::GetAllUsersRepositoryContract,
};

pub struct GetAllUsersService<R: GetAllUsersRepositoryContract> {
    repository: R,
}

impl<R: GetAllUsersRepositoryContract> GetAllUsersService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: GetAllUsersRepositoryContract> GetAllUsersUseCase for GetAllUsersService<R> {
    async fn get_all_users(&self) -> Result<Vec<NewUser>, GetAllUsersError> {
        self.repository.get_all_users().await
    }
}
