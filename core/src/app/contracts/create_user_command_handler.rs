use crate::domain::{errors::CreateUserError, models::CreateUserCommand};
use uuid::Uuid;

pub trait CreateUserCommandHandler {
    fn handle(
        &self,
        command: CreateUserCommand,
    ) -> impl std::future::Future<Output = Result<Uuid, CreateUserError>> + Send;
}
