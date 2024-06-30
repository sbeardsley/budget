use crate::domain::{errors::GetAllCategoriesError, models::NewCategory};
use uuid::Uuid;

pub trait GetAllCategoriesRepositoryContract: Clone + Send + Sync + 'static {
    fn get_all_categories(
        &self,
        user_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<NewCategory>, GetAllCategoriesError>> + Send;
}
