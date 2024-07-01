use crate::domain::{errors::UpdateUserError, models::UpdateUserCommand};

pub trait UpdateUserCommandHandler {
    fn handle(
        &self,
        command: UpdateUserCommand,
    ) -> impl std::future::Future<Output = Result<(), UpdateUserError>> + Send;
}
