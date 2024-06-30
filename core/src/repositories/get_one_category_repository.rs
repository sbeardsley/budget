use uuid::Uuid;

use crate::domain::{errors::GetOneCategoryError, models::NewCategory};

pub trait GetOneCategoryRepositoryContract: Clone + Send + Sync + 'static {
    fn get_one_category(
        &self,
        category_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<NewCategory>, GetOneCategoryError>> + Send;
}
