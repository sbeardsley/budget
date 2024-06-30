use chrono::{TimeZone, Utc};
use uuid::Uuid;

use core::{
    domain::{
        errors::CreateCategoryError,
        models::{CategoryDraft, NewCategory},
    },
    repositories::CreateCategoryRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct CreateCategoryRepository {
    db: Arc<SqliteConnectionPool>,
}

impl CreateCategoryRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl CreateCategoryRepositoryContract for CreateCategoryRepository {
    async fn insert_category(
        &self,
        category: CategoryDraft,
    ) -> Result<NewCategory, CreateCategoryError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| CreateCategoryError::Unknown)?;

        let id = Uuid::now_v7().to_string();
        let now = Utc::now();
        let budget_id = category.budget_id.to_string();
        let name = category.name;
        let created_at = now;
        let updated_at = now;

        match sqlx::query!(
            r#"
            INSERT INTO categories (id, name, budget_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, budget_id, created_at, updated_at;
            "#,
            id,
            name,
            budget_id,
            created_at,
            updated_at,
        )
        .map(|row| -> Result<NewCategory, CreateCategoryError> {
            Ok(NewCategory {
                id: Uuid::from_str(&row.id).map_err(|_| CreateCategoryError::Unknown)?,
                name: row.name,
                budget_id: Uuid::from_str(&row.budget_id)
                    .map_err(|_| CreateCategoryError::Unknown)?,
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
                Err(CreateCategoryError::Unknown)
            }
        }
    }
}
