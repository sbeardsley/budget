pub trait HashUserPasswordUseCase: Send + Sync {
    fn hash_user_password(
        &self,
        password: String,
    ) -> impl std::future::Future<Output = Result<String, anyhow::Error>> + Send;
}
