use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{errors::GetAllBudgetsError, models::NewBudget},
    repositories::GetAllBudgetsRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct GetAllBudgetsRepository {
    db: Arc<SqliteConnectionPool>,
}

impl GetAllBudgetsRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl GetAllBudgetsRepositoryContract for GetAllBudgetsRepository {
    async fn get_all_budgets(&self, user_id: Uuid) -> Result<Vec<NewBudget>, GetAllBudgetsError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| GetAllBudgetsError::Unknown)?;

        match sqlx::query!(
            r#"
            SELECT id, name, description, total, currency, user_id, created_at, updated_at
            FROM budgets
            WHERE user_id = $1;
            "#,
            user_id,
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
        .fetch_all(&mut *connection)
        .await
        {
            Ok(budgets) => Ok(budgets.into_iter().flatten().collect()),
            Err(_) => Err(GetAllBudgetsError::Unknown),
        }
    }
}
