use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{
        errors::CreateBudgetError,
        models::{BudgetDraft, NewBudget},
    },
    repositories::CreateBudgetRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct CreateBudgetRepository {
    db: Arc<SqliteConnectionPool>,
}

impl CreateBudgetRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl CreateBudgetRepositoryContract for CreateBudgetRepository {
    async fn insert_budget(&self, budget: BudgetDraft) -> Result<NewBudget, CreateBudgetError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| CreateBudgetError::Unknown)?;

        let id = Uuid::now_v7().to_string();
        let now = Utc::now();
        let name = budget.name;
        let description = budget.description;
        let total = budget.total;
        let currency = budget.currency;
        let user_id = budget.user_id.to_string();
        let created_at = now;
        let updated_at = now;

        match sqlx::query!(
            r#"
            INSERT INTO budgets (id, name, description, total, currency, user_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, name, description, total, currency, user_id, created_at, updated_at;
            "#,
            id,
            name,
            description,
            total,
            currency,
            user_id,
            created_at,
            updated_at,
        )
        .map(|row| {
            Ok(NewBudget {
                    id: Uuid::from_str(&row.id).map_err(|_| CreateBudgetError::Unknown)?,
                    name: row.name,
                    description: row.description,
                    total: row.total,
                    currency: row.currency,
                    user_id: Uuid::from_str(&row.user_id).map_err(|_| CreateBudgetError::Unknown)?,
                    created_at: Utc.from_utc_datetime(&row.created_at),
                    updated_at: Utc.from_utc_datetime(&row.updated_at),
                })
        })
        .fetch_one(&mut *connection)
        .await
        {
            Ok(budget) => budget,
            Err(e) => {
                println!("Error inserting budget {}", e);
                Err(CreateBudgetError::Unknown)
            }
        }
    }
}
