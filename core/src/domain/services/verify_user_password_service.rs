use crate::domain::usecases::VerifyUserPasswordUseCase;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
pub struct VerifyUserPasswordService {}

impl Default for VerifyUserPasswordService {
    fn default() -> Self {
        Self {}
    }
}

impl VerifyUserPasswordUseCase for VerifyUserPasswordService {
    async fn verify_user_password(
        &self,
        password: String,
        hashed_password: String,
    ) -> Result<bool, anyhow::Error> {
        let parsed_hash = PasswordHash::new(&hashed_password).expect("Failed to parse hash");
        let pwd = password.as_bytes();
        Argon2::default()
            .verify_password(pwd, &parsed_hash)
            .map_err(|err| anyhow::anyhow!("Failed to verify password: {}", err))
            .map(|_| true)
    }
}
