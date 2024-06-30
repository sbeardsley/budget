use crate::domain::{
    errors::UpdateCategoryError,
    models::{CategoryPatch, NewCategory},
};
use uuid::Uuid;

pub trait UpdateCategoryUseCase: Send + Sync {
    fn update_category(
        &self,
        category_id: Uuid,
        category: CategoryPatch,
    ) -> impl std::future::Future<Output = Result<NewCategory, UpdateCategoryError>> + Send;
}
