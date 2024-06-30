use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{errors::GetAllCategoriesError, models::NewCategory},
    repositories::GetAllCategoriesRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct GetAllCategoriesRepository {
    db: Arc<SqliteConnectionPool>,
}

impl GetAllCategoriesRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl GetAllCategoriesRepositoryContract for GetAllCategoriesRepository {
    async fn get_all_categories(
        &self,
        budget_id: Uuid,
    ) -> Result<Vec<NewCategory>, GetAllCategoriesError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| GetAllCategoriesError::Unknown)?;

        match sqlx::query!(
            r#"
            SELECT id, name, budget_id, created_at, updated_at
            FROM categories
            WHERE budget_id = $1
            "#,
            budget_id,
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
        .fetch_all(&mut *connection)
        .await
        {
            Ok(category) => Ok(category.into_iter().flatten().collect()),
            Err(_) => Err(GetAllCategoriesError::Unknown),
        }
    }
}
