use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{errors::GetAllUsersError, models::NewUser},
    repositories::GetAllUsersRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct GetAllUsersRepository {
    db: Arc<SqliteConnectionPool>,
}

impl GetAllUsersRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl GetAllUsersRepositoryContract for GetAllUsersRepository {
    async fn get_all_users(&self) -> Result<Vec<NewUser>, GetAllUsersError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| GetAllUsersError::Unknown)?;

        match sqlx::query!(
            r#"
            SELECT id, name, created_at, updated_at
            FROM users
            "#,
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
        .fetch_all(&mut *connection)
        .await
        {
            Ok(users) => Ok(users.into_iter().flatten().collect()),
            Err(_) => Err(GetAllUsersError::Unknown),
        }
    }
}
