use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::budget::BudgetId;
#[derive(Debug, Clone)]
pub struct CategoryId(Uuid);

impl From<Uuid> for CategoryId {
    fn from(category_id: Uuid) -> Self {
        Self(category_id)
    }
}

impl From<CategoryId> for Uuid {
    fn from(category_id: CategoryId) -> Self {
        category_id.0
    }
}

#[derive(Debug, Clone)]
pub struct CategoryName(String);

impl From<String> for CategoryName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl From<CategoryName> for String {
    fn from(name: CategoryName) -> Self {
        name.0
    }
}

#[derive(Debug, Clone)]
pub struct Category {
    pub id: CategoryId,
    pub budget_id: BudgetId,
    pub name: CategoryName,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Category {
    pub fn new(id: Uuid, budget_id: Uuid, name: String) -> Self {
        let now = Utc::now();
        Self {
            id: CategoryId::from(id),
            budget_id: BudgetId::from(budget_id),
            name: CategoryName(name),
            created_at: now,
            updated_at: now,
        }
    }
}
