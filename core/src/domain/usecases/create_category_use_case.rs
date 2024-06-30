use crate::domain::{
    errors::CreateCategoryError,
    models::{CategoryDraft, NewCategory},
};

pub trait CreateCategoryUseCase: Send + Sync {
    fn create_category(
        &self,
        category: CategoryDraft,
    ) -> impl std::future::Future<Output = Result<NewCategory, CreateCategoryError>> + Send;
}
