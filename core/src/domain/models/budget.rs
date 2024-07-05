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
    pub updated_at: DateTime<Utc>,
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

#[derive(Debug)]
pub struct CreateBudgetCommand {
    pub name: String,
    pub description: String,
    pub total: f64,
    pub currency: String,
    pub user_id: Uuid,
}

impl From<CreateBudgetCommand> for BudgetDraft {
    fn from(command: CreateBudgetCommand) -> Self {
        BudgetDraft {
            name: command.name,
            description: command.description,
            total: command.total,
            currency: command.currency,
            user_id: command.user_id,
        }
    }
}

#[derive(Debug)]
pub struct UpdateBudgetCommand {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub total: Option<f64>,
    pub currency: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl From<UpdateBudgetCommand> for BudgetPatch {
    fn from(command: UpdateBudgetCommand) -> Self {
        BudgetPatch {
            name: command.name,
            description: command.description,
            total: command.total,
            currency: command.currency,
            updated_at: command.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct GetAllBudgetsQuery {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct GetAllBudgetsQueryResult {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub total: f64,
    pub currency: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NewBudget> for GetAllBudgetsQueryResult {
    fn from(entity: NewBudget) -> Self {
        GetAllBudgetsQueryResult {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            total: entity.total,
            currency: entity.currency,
            user_id: entity.user_id,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct GetOneBudgetQuery {
    pub budget_id: Uuid,
}

#[derive(Debug)]
pub struct GetOneBudgetQueryResult {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub total: f64,
    pub currency: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NewBudget> for GetOneBudgetQueryResult {
    fn from(entity: NewBudget) -> Self {
        GetOneBudgetQueryResult {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            total: entity.total,
            currency: entity.currency,
            user_id: entity.user_id,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
