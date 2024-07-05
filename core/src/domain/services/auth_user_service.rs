use crate::{
    domain::{
        errors::AuthUserError,
        models::NewUser,
        usecases::{AuthUserUseCase, VerifyUserPasswordUseCase},
    },
    repositories::GetOneUserRepositoryByEmailContract,
};

use super::VerifyUserPasswordService;

pub struct AuthUserService<R: GetOneUserRepositoryByEmailContract> {
    repository: R,
    verify_user_password_service: VerifyUserPasswordService,
}

impl<R: GetOneUserRepositoryByEmailContract> AuthUserService<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            verify_user_password_service: VerifyUserPasswordService::default(),
        }
    }
}

impl<R: GetOneUserRepositoryByEmailContract> AuthUserUseCase for AuthUserService<R> {
    async fn auth_user(&self, email: String, password: String) -> Result<NewUser, AuthUserError> {
        let user = self
            .repository
            .get_one_user(email.clone())
            .await
            .map_err(|_| AuthUserError::UserNotFound)?;
        match user {
            Some(user) => {
                let hashed_password = user.password.clone();
                self.verify_user_password_service
                    .verify_user_password(password, hashed_password)
                    .await
                    .map(|_| user)
                    .map_err(|_| AuthUserError::PasswordDoesNotMatch)
            }
            None => Err(AuthUserError::UserNotFound),
        }
    }
}
