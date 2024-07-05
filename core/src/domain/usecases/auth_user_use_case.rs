use crate::domain::{errors::AuthUserError, models::NewUser};

pub trait AuthUserUseCase {
    async fn auth_user(&self, email: String, password: String) -> Result<NewUser, AuthUserError>;
}
