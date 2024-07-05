use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{
        errors::UpdateUserError,
        models::{NewUser, UserPatch},
    },
    repositories::UpdateUserRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct UpdateUserRepository {
    db: Arc<SqliteConnectionPool>,
}

impl UpdateUserRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl UpdateUserRepositoryContract for UpdateUserRepository {
    async fn update_user(
        &self,
        user_id: Uuid,
        user: UserPatch,
    ) -> Result<NewUser, UpdateUserError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| UpdateUserError::Unknown)?;

        let id = user_id.to_string();
        let name = user.name;
        let email = user.email;
        let password = user.password;
        let updated_at = user.updated_at;

        match sqlx::query!(
            r#"
            UPDATE users
            SET name = $2, email = $3, password = $4
            WHERE id = $1 and  updated_at = $5
            RETURNING id, name, email, password, created_at, updated_at;
            "#,
            id,
            name,
            email,
            password,
            updated_at,
        )
        .map(|row| {
            Ok(NewUser {
                id: Uuid::from_str(&row.id).map_err(|_| UpdateUserError::Unknown)?,
                name: row.name,
                email: row.email,
                password: row.password,
                created_at: Utc.from_utc_datetime(&row.created_at),
                updated_at: Utc.from_utc_datetime(&row.updated_at),
            })
        })
        .fetch_one(&mut *connection)
        .await
        {
            Ok(user) => user,
            Err(e) => {
                println!("Error inserting user {}", e);
                Err(UpdateUserError::Unknown)
            }
        }
    }
}
