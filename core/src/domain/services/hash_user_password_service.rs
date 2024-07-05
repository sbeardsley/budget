use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use crate::domain::usecases::HashUserPasswordUseCase;

#[derive(Default)]
pub struct HashUserPasswordService {}

impl HashUserPasswordUseCase for HashUserPasswordService {
    async fn hash_user_password(&self, password: String) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        let pwd = password.as_bytes();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2
            .hash_password(pwd, &salt)
            .map_err(|err| anyhow::anyhow!("Failed to hash password: {}", err))?;

        Ok(password_hash.to_string())

        // Verify password against PHC string.
        //
        // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
        // `Argon2` instance.
        // let pwd_hash = password_hash.to_string();
        // let parsed_hash = PasswordHash::new(&pwd_hash).expect("Failed to parse hash");
        // assert!(Argon2::default().verify_password(pwd, &parsed_hash).is_ok());
    }
}
