use crate::domain::{
    errors::CreateCategoryError,
    models::{CategoryDraft, NewCategory},
};

pub trait CreateCategoryRepositoryContract: Clone + Send + Sync + 'static {
    fn insert_category(
        &self,
        category: CategoryDraft,
    ) -> impl std::future::Future<Output = Result<NewCategory, CreateCategoryError>> + Send;
}
