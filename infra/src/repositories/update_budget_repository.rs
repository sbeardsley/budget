use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{
        errors::UpdateBudgetError,
        models::{BudgetPatch, NewBudget},
    },
    repositories::UpdateBudgetRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct UpdateBudgetRepository {
    db: Arc<SqliteConnectionPool>,
}

impl UpdateBudgetRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl UpdateBudgetRepositoryContract for UpdateBudgetRepository {
    async fn update_budget(
        &self,
        budget_id: Uuid,
        budget: BudgetPatch,
    ) -> Result<NewBudget, UpdateBudgetError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| UpdateBudgetError::Unknown)?;

        let id = budget_id;
        let name = budget.name;
        let description = budget.description;
        let total = budget.total;
        let currency = budget.currency;
        let updated_at = budget.updated_at;

        match sqlx::query!(
            r#"
            UPDATE budgets
            SET name = $2, description = $3, total = $4, currency = $5
            WHERE id = $1 and updated_at = $6
            RETURNING id, name, description, total, currency, user_id, created_at, updated_at;
            "#,
            id,
            name,
            description,
            total,
            currency,
            updated_at,
        )
        .map(|row| {
            Ok(NewBudget {
                id: Uuid::from_str(&row.id).map_err(|_| UpdateBudgetError::Unknown)?,
                name: row.name,
                description: row.description,
                total: row.total,
                currency: row.currency,
                user_id: Uuid::from_str(&row.user_id).map_err(|_| UpdateBudgetError::Unknown)?,
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
                Err(UpdateBudgetError::Unknown)
            }
        }
    }
}
