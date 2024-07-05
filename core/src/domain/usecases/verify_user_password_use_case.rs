pub trait VerifyUserPasswordUseCase: Send + Sync {
    fn verify_user_password(
        &self,
        password: String,
        hashed_password: String,
    ) -> impl std::future::Future<Output = Result<bool, anyhow::Error>> + Send;
}
