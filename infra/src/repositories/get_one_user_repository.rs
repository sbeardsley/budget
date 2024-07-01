use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{errors::GetOneUserError, models::NewUser},
    repositories::GetOneUserRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct GetOneUserRepository {
    db: Arc<SqliteConnectionPool>,
}

impl GetOneUserRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl GetOneUserRepositoryContract for GetOneUserRepository {
    async fn get_one_user(&self, user_id: Uuid) -> Result<Option<NewUser>, GetOneUserError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| GetOneUserError::Unknown)?;

        match sqlx::query!(
            r#"
            SELECT id, name, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            user_id,
        )
        .map(|row| -> Option<NewUser> {
            let id = Uuid::from_str(&row.id).ok();

            if Some(id) == None {
                return None;
            }

            Some(NewUser {
                id: Uuid::from_str(&row.id).expect("unable to parse id"),
                name: row.name,
                created_at: Utc.from_utc_datetime(&row.created_at),
                updated_at: Utc.from_utc_datetime(&row.updated_at),
            })
        })
        .fetch_one(&mut *connection)
        .await
        {
            Ok(user) => Ok(user),
            Err(_) => Err(GetOneUserError::Unknown),
        }
    }
}
