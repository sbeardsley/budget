use chrono::{TimeZone, Utc};
use sqlx::types::Uuid;

use core::{
    domain::{
        errors::CreateUserError,
        models::{NewUser, UserDraft},
    },
    repositories::CreateUserRepositoryContract,
};
use std::{str::FromStr, sync::Arc};

use super::sqlite_connection_pool::SqliteConnectionPool;

#[derive(Debug, Clone)]
pub struct CreateUserRepository {
    db: Arc<SqliteConnectionPool>,
}

impl CreateUserRepository {
    pub fn new(db: Arc<SqliteConnectionPool>) -> Self {
        Self { db }
    }
}

impl CreateUserRepositoryContract for CreateUserRepository {
    async fn insert_user(&self, user: UserDraft) -> Result<NewUser, CreateUserError> {
        let mut connection = self
            .db
            .clone()
            .connect()
            .await
            .map_err(|_| CreateUserError::Unknown)?;

        let id = Uuid::now_v7().to_string();
        let now = Utc::now();
        let name = user.name;
        let email = user.email;
        let password = user.password;
        let created_at = now;
        let updated_at = now;

        match sqlx::query!(
            r#"
            INSERT INTO users (id, name, email, password, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, email, password, created_at, updated_at;
            "#,
            id,
            name,
            email,
            password,
            created_at,
            updated_at,
        )
        .map(|row| {
            Ok(NewUser {
                id: Uuid::from_str(&row.id).map_err(|_| CreateUserError::Unknown)?,
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
                Err(CreateUserError::Unknown)
            }
        }
    }
}
