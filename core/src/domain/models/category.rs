use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct CategoryDraft {
    pub name: String,
    pub budget_id: Uuid,
}

#[derive(Debug)]
pub struct CategoryPatch {
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct NewCategory {
    pub id: Uuid,
    pub name: String,
    pub budget_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct DeleteCategory {
    pub id: Uuid,
}

#[derive(Debug)]
pub struct CreateCategoryCommand {
    pub name: String,
    pub budget_id: Uuid,
}

impl From<CreateCategoryCommand> for CategoryDraft {
    fn from(command: CreateCategoryCommand) -> Self {
        CategoryDraft {
            name: command.name,
            budget_id: command.budget_id,
        }
    }
}

#[derive(Debug)]
pub struct UpdateCategoryCommand {
    pub id: Uuid,
    pub name: Option<String>,
}

impl From<UpdateCategoryCommand> for CategoryPatch {
    fn from(command: UpdateCategoryCommand) -> Self {
        CategoryPatch { name: command.name }
    }
}

#[derive(Debug)]
pub struct GetAllCategoriesQuery {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct GetAllCategoriesQueryResult {
    pub id: Uuid,
    pub name: String,
    pub budget_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NewCategory> for GetAllCategoriesQueryResult {
    fn from(entity: NewCategory) -> Self {
        GetAllCategoriesQueryResult {
            id: entity.id,
            name: entity.name,
            budget_id: entity.budget_id,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct GetOneCategoryQuery {
    pub category_id: Uuid,
}

#[derive(Debug)]
pub struct GetOneCategoryQueryResult {
    pub id: Uuid,
    pub name: String,
    pub budget_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NewCategory> for GetOneCategoryQueryResult {
    fn from(entity: NewCategory) -> Self {
        GetOneCategoryQueryResult {
            id: entity.id,
            name: entity.name,
            budget_id: entity.budget_id,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
