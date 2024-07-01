use crate::{
    app::contracts::GetAllUsersQueryHandler,
    domain::{
        errors::GetAllUsersError,
        models::{GetAllUsersQuery, GetAllUsersQueryResult},
        usecases::GetAllUsersUseCase,
    },
    repositories::GetAllUsersRepositoryContract,
};

pub struct GetAllUsersService<T: GetAllUsersRepositoryContract> {
    get_all_users: crate::domain::services::GetAllUsersService<T>,
}

impl<T: GetAllUsersRepositoryContract> GetAllUsersService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            get_all_users: crate::domain::services::GetAllUsersService::new(repository),
        }
    }
}

impl<T: GetAllUsersRepositoryContract> GetAllUsersQueryHandler for GetAllUsersService<T> {
    async fn handle(
        &self,
        _: GetAllUsersQuery,
    ) -> Result<Vec<GetAllUsersQueryResult>, GetAllUsersError> {
        match self.get_all_users.get_all_users().await {
            Ok(user) => Ok(user
                .into_iter()
                .map(|b| GetAllUsersQueryResult::from(b))
                .collect()),
            Err(e) => Err(e),
        }
    }
}
