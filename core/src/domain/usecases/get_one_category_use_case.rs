use crate::domain::{errors::GetOneCategoryError, models::NewCategory};
use uuid::Uuid;

pub trait GetOneCategoryUseCase: Send + Sync {
    fn get_one_category(
        &self,
        category_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<NewCategory>, GetOneCategoryError>> + Send;
}
