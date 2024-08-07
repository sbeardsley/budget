use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{errors::GetOneBudgetError, models::NewBudget},
    repositories::GetOneBudgetRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct GetOneBudgetRepository {
    db: Arc<SqliteConnectionPool>,
}

impl GetOneBudgetRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl GetOneBudgetRepositoryContract for GetOneBudgetRepository {
    async fn get_one_budget(
        &self,
        budget_id: Uuid,
    ) -> Result<Option<NewBudget>, GetOneBudgetError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| GetOneBudgetError::Unknown)?;

        match sqlx::query!(
            r#"
            SELECT id, name, description, total, currency, user_id, created_at, updated_at
            FROM budgets
            WHERE id = $1;
            "#,
            budget_id,
        )
        .map(|row| -> Option<NewBudget> {
            let id = Uuid::from_str(&row.id).ok();
            let user_id = Uuid::from_str(&row.user_id).ok();

            if Some(id) == None || Some(user_id) == None {
                return None;
            }

            Some(NewBudget {
                id: Uuid::from_str(&row.id).expect("unable to parse id"),
                name: row.name,
                description: row.description,
                total: row.total,
                currency: row.currency,
                user_id: Uuid::from_str(&row.user_id).expect("unable to parse user_id"),
                created_at: Utc.from_utc_datetime(&row.created_at),
                updated_at: Utc.from_utc_datetime(&row.updated_at),
            })
        })
        .fetch_one(&mut *connection)
        .await
        {
            Ok(budget) => Ok(budget),
            Err(_) => Err(GetOneBudgetError::Unknown),
        }
    }
}
