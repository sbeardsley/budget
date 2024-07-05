use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{
        errors::UpdateCategoryError,
        models::{CategoryPatch, NewCategory},
    },
    repositories::UpdateCategoryRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct UpdateCategoryRepository {
    db: Arc<SqliteConnectionPool>,
}

impl UpdateCategoryRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl UpdateCategoryRepositoryContract for UpdateCategoryRepository {
    async fn update_category(
        &self,
        category_id: Uuid,
        category: CategoryPatch,
    ) -> Result<NewCategory, UpdateCategoryError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| UpdateCategoryError::Unknown)?;

        let id = category_id.to_string();
        let name = category.name;
        let updated_at = category.updated_at;

        match sqlx::query!(
            r#"
            UPDATE categories
            SET name = $2
            WHERE id = $1 and updated_at = $3
            RETURNING id, name, budget_id, created_at, updated_at;
            "#,
            id,
            name,
            updated_at
        )
        .map(|row| {
            Ok(NewCategory {
                id: Uuid::from_str(&row.id).map_err(|_| UpdateCategoryError::Unknown)?,
                name: row.name,
                budget_id: Uuid::from_str(&row.budget_id)
                    .map_err(|_| UpdateCategoryError::Unknown)?,
                created_at: Utc.from_utc_datetime(&row.created_at),
                updated_at: Utc.from_utc_datetime(&row.updated_at),
            })
        })
        .fetch_one(&mut *connection)
        .await
        {
            Ok(category) => category,
            Err(e) => {
                println!("Error inserting category {}", e);
                Err(UpdateCategoryError::Unknown)
            }
        }
    }
}
