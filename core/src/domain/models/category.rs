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
