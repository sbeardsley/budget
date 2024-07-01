use crate::{
    app::contracts::GetOneUserQueryHandler,
    domain::{
        errors::GetOneUserError,
        models::{GetOneUserQuery, GetOneUserQueryResult},
        usecases::GetOneUserUseCase,
    },
    repositories::GetOneUserRepositoryContract,
};

pub struct GetOneUserService<T: GetOneUserRepositoryContract> {
    get_one_user: crate::domain::services::GetOneUserService<T>,
}

impl<T: GetOneUserRepositoryContract> GetOneUserService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            get_one_user: crate::domain::services::GetOneUserService::new(repository),
        }
    }
}

impl<T: GetOneUserRepositoryContract> GetOneUserQueryHandler for GetOneUserService<T> {
    async fn handle(
        &self,
        query: GetOneUserQuery,
    ) -> Result<GetOneUserQueryResult, GetOneUserError> {
        match self.get_one_user.get_one_user(query.user_id).await {
            Ok(user) => match user {
                Some(user) => Ok(GetOneUserQueryResult::from(user)),
                None => Err(GetOneUserError::Unknown),
            },
            Err(e) => Err(e),
        }
    }
}
