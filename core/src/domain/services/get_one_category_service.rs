use uuid::Uuid;

use crate::domain::{errors::GetOneCategoryError, models::NewCategory};

pub trait GetOneCategoryRepositoryContract {
    fn get_one_category(
        &self,
        category_id: Uuid,
    ) -> impl std::future::Future<Output = Result<NewCategory, GetOneCategoryError>> + Send;
}
