use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct BudgetDraft {
    pub name: String,
    pub description: String,
    pub total: f64,
    pub currency: String,
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct BudgetPatch {
    pub name: Option<String>,
    pub description: Option<String>,
    pub total: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Debug)]
pub struct NewBudget {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub total: f64,
    pub currency: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct DeleteBudget {
    pub id: Uuid,
}
