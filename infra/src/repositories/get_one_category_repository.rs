use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{errors::GetOneCategoryError, models::NewCategory},
    repositories::GetOneCategoryRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct GetOneCategoryRepository {
    db: Arc<SqliteConnectionPool>,
}

impl GetOneCategoryRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl GetOneCategoryRepositoryContract for GetOneCategoryRepository {
    async fn get_one_category(
        &self,
        category_id: Uuid,
    ) -> Result<Option<NewCategory>, GetOneCategoryError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| GetOneCategoryError::Unknown)?;

        match sqlx::query!(
            r#"
            SELECT id, name, budget_id, created_at, updated_at
            FROM categories
            WHERE id = $1
            "#,
            category_id,
        )
        .map(|row| -> Option<NewCategory> {
            let id = Uuid::from_str(&row.id).ok();
            let budget_id = Uuid::from_str(&row.budget_id).ok();

            if Some(id) == None || Some(budget_id) == None {
                return None;
            }

            Some(NewCategory {
                id: Uuid::from_str(&row.id).expect("unable to parse id"),
                name: row.name,
                budget_id: Uuid::from_str(&row.budget_id).expect("unable to parse budget_id"),
                created_at: Utc.from_utc_datetime(&row.created_at),
                updated_at: Utc.from_utc_datetime(&row.updated_at),
            })
        })
        .fetch_one(&mut *connection)
        .await
        {
            Ok(category) => Ok(category),
            Err(_) => Err(GetOneCategoryError::Unknown),
        }
    }
}
